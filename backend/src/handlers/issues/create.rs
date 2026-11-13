use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use super::helpers::fetch_single_issue;
use crate::{
    auth::AuthUser,
    models::CreateIssuePayload,
    ws::WsBroadcastMessage,
    AppState,
};

pub async fn create_issue(
    auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateIssuePayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if payload.summary.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Issue summary (title) is required" })),
        ));
    }

    // Begin isolated transaction for atomic issue counter & key generation (FR-3.2, Non-Functional Requirement 5)
    let mut tx = state.db.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Transaction start failed: {}", e) })),
        )
    })?;

    // Lock project row FOR UPDATE
    let proj: (String, i32) = sqlx::query_as(
        "SELECT `key`, current_issue_counter FROM projects WHERE id = ? FOR UPDATE"
    )
    .bind(project_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Project not found" })),
        )
    })?;

    let proj_key = proj.0;
    let next_counter = proj.1 + 1;

    // Update project counter
    sqlx::query("UPDATE projects SET current_issue_counter = ? WHERE id = ?")
        .bind(next_counter)
        .bind(project_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to update counter: {}", e) })),
            )
        })?;

    let issue_key = format!("{}-{}", proj_key, next_counter);

    // Resolve status_id if not given
    let status_id = if let Some(sid) = payload.status_id {
        sid
    } else {
        let first_status: (i64,) = sqlx::query_as(
            "SELECT id FROM statuses WHERE project_id = ? ORDER BY position ASC LIMIT 1"
        )
        .bind(project_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "No statuses found for this project" })),
            )
        })?;
        first_status.0
    };

    let issue_type = payload.issue_type.unwrap_or_else(|| "TASK".to_string());
    let priority = payload.priority.unwrap_or_else(|| "MEDIUM".to_string());
    let points = payload.story_points.unwrap_or(0);

    // Position in column
    let max_pos: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(position) FROM issues WHERE project_id = ? AND status_id = ?"
    )
    .bind(project_id)
    .bind(status_id)
    .fetch_one(&mut *tx)
    .await
    .unwrap_or((Some(0),));

    let position = max_pos.0.unwrap_or(0) + 1;

    let res = sqlx::query(
        "INSERT INTO issues (project_id, parent_id, epic_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, start_date, due_date, position, sprint_id, component_id, fix_version_id) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(project_id)
    .bind(payload.parent_id)
    .bind(payload.epic_id)
    .bind(&issue_key)
    .bind(&payload.summary)
    .bind(&payload.description)
    .bind(&issue_type)
    .bind(status_id)
    .bind(&priority)
    .bind(payload.assignee_id)
    .bind(auth.user_id)
    .bind(points)
    .bind(payload.start_date)
    .bind(payload.due_date)
    .bind(position)
    .bind(payload.sprint_id)
    .bind(payload.component_id)
    .bind(payload.fix_version_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to insert issue: {}", e) })),
        )
    })?;

    let issue_id = res.last_insert_id() as i64;

    // Log activity
    let _ = sqlx::query(
        "INSERT INTO activity_logs (project_id, issue_id, user_id, action, details) \
         VALUES (?, ?, ?, 'CREATE_ISSUE', ?)"
    )
    .bind(project_id)
    .bind(issue_id)
    .bind(auth.user_id)
    .bind(format!("Created {} - {}", issue_key, payload.summary))
    .execute(&mut *tx)
    .await;

    // If assignee was set, notify them
    if let Some(assignee_id) = payload.assignee_id {
        if assignee_id != auth.user_id {
            let _ = sqlx::query(
                "INSERT INTO notifications (recipient_id, sender_id, issue_id, title, message, action_type) \
                 VALUES (?, ?, ?, 'Assigned to you', ?, 'ASSIGNED')"
            )
            .bind(assignee_id)
            .bind(auth.user_id)
            .bind(issue_id)
            .bind(format!("You were assigned to {} ({})", issue_key, payload.summary))
            .execute(&mut *tx)
            .await;
        }
    }

    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Commit failed: {}", e) })),
        )
    })?;

    // Fetch complete issue object
    let created_issue = fetch_single_issue(&state.db, issue_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Fetch failed: {}", e) })),
        )
    })?;

    // Broadcast over WebSocket
    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "ISSUE_CREATED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: serde_json::to_value(&created_issue).unwrap_or_default(),
    });

    Ok((StatusCode::CREATED, Json(created_issue)))
}

