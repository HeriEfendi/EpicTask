use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use super::helpers::{fetch_single_issue, validate_transition};
use crate::{
    auth::AuthUser,
    automation::AutomationEngine,
    models::{MoveIssuePayload, UpdateIssuePayload},
    ws::WsBroadcastMessage,
    AppState,
};

pub async fn update_issue(
    auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateIssuePayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current_issue = fetch_single_issue(&state.db, issue_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Issue not found" })),
        )
    })?;

    // Check status change if provided
    let mut new_status_category = current_issue.status_category.clone();
    if let Some(target_status_id) = payload.status_id {
        if target_status_id != current_issue.status_id {
            // Validate transition rules
            validate_transition(&state.db, current_issue.project_id, current_issue.status_id, target_status_id).await?;
            let cat: (String,) = sqlx::query_as("SELECT category FROM statuses WHERE id = ?")
                .bind(target_status_id)
                .fetch_one(&state.db)
                .await
                .map_err(|_| (StatusCode::BAD_REQUEST, Json(json!({ "error": "Status not found" }))))?;
            new_status_category = Some(cat.0);
        }
    }

    let summary = payload.summary.unwrap_or(current_issue.summary);
    let description = payload.description.or(current_issue.description);
    let issue_type = payload.issue_type.unwrap_or(current_issue.issue_type);
    let status_id = payload.status_id.unwrap_or(current_issue.status_id);
    let priority = payload.priority.or(current_issue.priority);
    let assignee_id = match payload.assignee_id {
        Some(val) => val,
        None => current_issue.assignee_id,
    };
    let parent_id = match payload.parent_id {
        Some(val) => val,
        None => current_issue.parent_id,
    };
    let epic_id = match payload.epic_id {
        Some(val) => val,
        None => current_issue.epic_id,
    };
    let sprint_id = match payload.sprint_id {
        Some(val) => val,
        None => current_issue.sprint_id,
    };
    let component_id = match payload.component_id {
        Some(val) => val,
        None => current_issue.component_id,
    };
    let fix_version_id = match payload.fix_version_id {
        Some(val) => val,
        None => current_issue.fix_version_id,
    };
    let story_points = payload.story_points.or(current_issue.story_points);
    let start_date = match payload.start_date {
        Some(val) => val,
        None => current_issue.start_date,
    };
    let due_date = match payload.due_date {
        Some(val) => val,
        None => current_issue.due_date,
    };
    let position = payload.position.or(current_issue.position);

    // Resolution & resolved_at handling
    let (resolution, resolved_at) = match new_status_category.as_deref() {
        Some("DONE") => {
            if let Some(opt_res) = &payload.resolution {
                match opt_res {
                    Some(res) if !res.trim().is_empty() => {
                        (Some(res.trim().to_string()), current_issue.resolved_at.or_else(|| Some(chrono::Utc::now())))
                    }
                    _ => (None, None),
                }
            } else if current_issue.resolution.is_some() {
                (current_issue.resolution.clone(), current_issue.resolved_at.or_else(|| Some(chrono::Utc::now())))
            } else {
                let default_res = if issue_type == "BUG" { "Fixed".to_string() } else { "Done".to_string() };
                (Some(default_res), Some(chrono::Utc::now()))
            }
        }
        Some(_) => {
            // Moved to a non-DONE category -> clear resolution
            (None, None)
        }
        None => {
            // Category unknown or status unchanged
            if let Some(opt_res) = &payload.resolution {
                match opt_res {
                    Some(res) if !res.trim().is_empty() => {
                        (Some(res.trim().to_string()), current_issue.resolved_at.or_else(|| Some(chrono::Utc::now())))
                    }
                    _ => (None, None),
                }
            } else {
                (current_issue.resolution.clone(), current_issue.resolved_at)
            }
        }
    };

    sqlx::query(
        "UPDATE issues SET summary = ?, description = ?, issue_type = ?, status_id = ?, priority = ?, assignee_id = ?, parent_id = ?, epic_id = ?, story_points = ?, start_date = ?, due_date = ?, position = ?, sprint_id = ?, component_id = ?, fix_version_id = ?, resolution = ?, resolved_at = ? \
         WHERE id = ?"
    )
    .bind(&summary)
    .bind(&description)
    .bind(&issue_type)
    .bind(status_id)
    .bind(&priority)
    .bind(assignee_id)
    .bind(parent_id)
    .bind(epic_id)
    .bind(story_points)
    .bind(start_date)
    .bind(due_date)
    .bind(position)
    .bind(sprint_id)
    .bind(component_id)
    .bind(fix_version_id)
    .bind(&resolution)
    .bind(resolved_at)
    .bind(issue_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Update error: {}", e) })),
        )
    })?;

    // Update labels if provided
    if let Some(ref labels) = payload.labels {
        let _ = sqlx::query("DELETE FROM issue_labels WHERE issue_id = ?")
            .bind(issue_id)
            .execute(&state.db)
            .await;

        let default_colors = [
            "#6366f1", "#06b6d4", "#10b981", "#f59e0b", "#ec4899",
            "#8b5cf6", "#3b82f6", "#14b8a6", "#f97316", "#84cc16",
        ];
        for (idx, name_raw) in labels.iter().enumerate() {
            let clean = name_raw.trim().to_lowercase();
            if clean.is_empty() {
                continue;
            }
            let color = default_colors[idx % default_colors.len()];
            let _ = sqlx::query(
                "INSERT INTO labels (project_id, name, color) VALUES (?, ?, ?) \
                 ON DUPLICATE KEY UPDATE name=name"
            )
            .bind(current_issue.project_id)
            .bind(&clean)
            .bind(color)
            .execute(&state.db)
            .await;

            let lbl_row: Result<(i64,), _> = sqlx::query_as("SELECT id FROM labels WHERE project_id = ? AND name = ?")
                .bind(current_issue.project_id)
                .bind(&clean)
                .fetch_one(&state.db)
                .await;

            if let Ok(lbl) = lbl_row {
                let _ = sqlx::query("INSERT IGNORE INTO issue_labels (issue_id, label_id) VALUES (?, ?)")
                    .bind(issue_id)
                    .bind(lbl.0)
                    .execute(&state.db)
                    .await;
            }
        }
    }

    // Check if status changed -> trigger automation
    if let Some(cat) = new_status_category {
        if status_id != current_issue.status_id {
            AutomationEngine::on_status_changed(
                &state.db,
                &state.ws_hub,
                current_issue.project_id,
                issue_id,
                &cat,
                Some(auth.user_id),
            ).await;
        }
    }

    // Log activity
    let _ = sqlx::query(
        "INSERT INTO activity_logs (project_id, issue_id, user_id, action, details) \
         VALUES (?, ?, ?, 'UPDATE_ISSUE', ?)"
    )
    .bind(current_issue.project_id)
    .bind(issue_id)
    .bind(auth.user_id)
    .bind(format!("Updated {}", current_issue.key))
    .execute(&state.db)
    .await;

    let updated = fetch_single_issue(&state.db, issue_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Fetch updated error: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "ISSUE_UPDATED".to_string(),
        project_id: Some(current_issue.project_id),
        user_id: Some(auth.user_id),
        data: serde_json::to_value(&updated).unwrap_or_default(),
    });

    Ok(Json(updated))
}

pub async fn move_issue(
    auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<MoveIssuePayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current_issue = fetch_single_issue(&state.db, issue_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Issue not found" })),
        )
    })?;

    // If changing status, validate workflow transition rule (FR-2.3, FR-4.1)
    if current_issue.status_id != payload.target_status_id {
        validate_transition(
            &state.db,
            current_issue.project_id,
            current_issue.status_id,
            payload.target_status_id,
        ).await?;
    }

    let target_pos = payload.target_position.unwrap_or(0);

    // Get target category
    let target_cat: Result<(String, String), _> = sqlx::query_as(
        "SELECT name, category FROM statuses WHERE id = ?"
    )
    .bind(payload.target_status_id)
    .fetch_one(&state.db)
    .await;

    let (status_name, category) = target_cat.map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Target status not found" })),
        )
    })?;

    // Determine resolution & resolved_at based on category
    let (resolution, resolved_at) = if category == "DONE" {
        let res = payload.resolution.or_else(|| current_issue.resolution.clone()).unwrap_or_else(|| {
            if current_issue.issue_type == "BUG" {
                "Fixed".to_string()
            } else {
                "Done".to_string()
            }
        });
        (Some(res), current_issue.resolved_at.or_else(|| Some(chrono::Utc::now())))
    } else {
        (None, None)
    };

    sqlx::query("UPDATE issues SET status_id = ?, position = ?, resolution = ?, resolved_at = ? WHERE id = ?")
        .bind(payload.target_status_id)
        .bind(target_pos)
        .bind(&resolution)
        .bind(resolved_at)
        .bind(issue_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Move failed: {}", e) })),
            )
        })?;

    let log_detail = if let Some(ref r) = resolution {
        format!("Moved {} to {} (Resolution: {})", current_issue.key, status_name, r)
    } else {
        format!("Moved {} to {}", current_issue.key, status_name)
    };

    // Log movement in activity stream
    let _ = sqlx::query(
        "INSERT INTO activity_logs (project_id, issue_id, user_id, action, details) \
         VALUES (?, ?, ?, 'MOVE_ISSUE', ?)"
    )
    .bind(current_issue.project_id)
    .bind(issue_id)
    .bind(auth.user_id)
    .bind(log_detail)
    .execute(&state.db)
    .await;

    // Run automation engine
    AutomationEngine::on_status_changed(
        &state.db,
        &state.ws_hub,
        current_issue.project_id,
        issue_id,
        &category,
        Some(auth.user_id),
    ).await;

    let updated = fetch_single_issue(&state.db, issue_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Fetch error: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "ISSUE_MOVED".to_string(),
        project_id: Some(current_issue.project_id),
        user_id: Some(auth.user_id),
        data: serde_json::to_value(&updated).unwrap_or_default(),
    });

    Ok(Json(updated))
}

