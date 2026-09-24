use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    auth::AuthUser,
    automation::AutomationEngine,
    models::{
        CreateIssueLinkPayload, CreateIssuePayload, Issue, IssueLink, MoveIssuePayload,
        UpdateIssuePayload,
    },
    ws::WsBroadcastMessage,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct IssueFilterParams {
    pub search: Option<String>,
    pub status_id: Option<i64>,
    pub assignee_id: Option<i64>,
    pub priority: Option<String>,
    pub issue_type: Option<String>,
    pub epic_id: Option<i64>,
}

pub async fn list_project_issues(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    Query(params): Query<IssueFilterParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut sql = String::from(
        "SELECT i.id, i.project_id, i.parent_id, i.epic_id, i.`key`, i.summary, i.description, \
                i.issue_type, i.status_id, i.priority, i.assignee_id, i.reporter_id, i.story_points, \
                i.start_date, i.due_date, i.position, i.created_at, i.updated_at, \
                s.name AS status_name, s.category AS status_category, s.color AS status_color, \
                ua.full_name AS assignee_name, ua.avatar_url AS assignee_avatar, \
                ur.full_name AS reporter_name, \
                ep.summary AS epic_summary, \
                p.`key` AS parent_key, p.summary AS parent_summary, \
                (SELECT COUNT(*) FROM issues sub WHERE sub.parent_id = i.id) AS subtask_count, \
                (SELECT COUNT(*) FROM issues sub JOIN statuses subs ON sub.status_id = subs.id WHERE sub.parent_id = i.id AND subs.category = 'DONE') AS subtask_done_count \
         FROM issues i \
         JOIN statuses s ON i.status_id = s.id \
         LEFT JOIN users ua ON i.assignee_id = ua.id \
         LEFT JOIN users ur ON i.reporter_id = ur.id \
         LEFT JOIN issues ep ON i.epic_id = ep.id \
         LEFT JOIN issues p ON i.parent_id = p.id \
         WHERE i.project_id = ?"
    );

    if let Some(ref q) = params.search {
        if !q.trim().is_empty() {
            sql.push_str(" AND (i.summary LIKE '%");
            sql.push_str(&q.replace('\'', "''"));
            sql.push_str("%' OR i.`key` LIKE '%");
            sql.push_str(&q.replace('\'', "''"));
            sql.push_str("%')");
        }
    }

    if let Some(status_id) = params.status_id {
        sql.push_str(&format!(" AND i.status_id = {}", status_id));
    }
    if let Some(assignee_id) = params.assignee_id {
        sql.push_str(&format!(" AND i.assignee_id = {}", assignee_id));
    }
    if let Some(ref priority) = params.priority {
        sql.push_str(&format!(" AND i.priority = '{}'", priority));
    }
    if let Some(ref itype) = params.issue_type {
        sql.push_str(&format!(" AND i.issue_type = '{}'", itype));
    }
    if let Some(epic_id) = params.epic_id {
        sql.push_str(&format!(" AND i.epic_id = {}", epic_id));
    }

    sql.push_str(" ORDER BY s.position ASC, i.position ASC, i.id DESC");

    let issues = sqlx::query_as::<_, Issue>(&sql)
        .bind(project_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {}", e) })),
            )
        })?;

    Ok(Json(issues))
}

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
        "INSERT INTO issues (project_id, parent_id, epic_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, start_date, due_date, position) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
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

pub async fn get_issue(
    _auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let issue = fetch_single_issue(&state.db, issue_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Issue not found" })),
        )
    })?;

    // Fetch subtasks
    let subtasks = sqlx::query_as::<_, Issue>(
        "SELECT i.id, i.project_id, i.parent_id, i.epic_id, i.`key`, i.summary, i.description, \
                i.issue_type, i.status_id, i.priority, i.assignee_id, i.reporter_id, i.story_points, \
                i.start_date, i.due_date, i.position, i.created_at, i.updated_at, \
                s.name AS status_name, s.category AS status_category, s.color AS status_color, \
                ua.full_name AS assignee_name, ua.avatar_url AS assignee_avatar, \
                ur.full_name AS reporter_name, \
                NULL AS epic_summary, NULL AS parent_key, NULL AS parent_summary, \
                0 AS subtask_count, 0 AS subtask_done_count \
         FROM issues i \
         JOIN statuses s ON i.status_id = s.id \
         LEFT JOIN users ua ON i.assignee_id = ua.id \
         LEFT JOIN users ur ON i.reporter_id = ur.id \
         WHERE i.parent_id = ? \
         ORDER BY s.position ASC, i.id ASC"
    )
    .bind(issue_id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    // Fetch links
    let links = sqlx::query_as::<_, IssueLink>(
        "SELECT il.id, il.source_issue_id, il.target_issue_id, il.link_type, il.created_at, \
                target.key AS target_key, target.summary AS target_summary, s.name AS target_status_name \
         FROM issue_links il \
         JOIN issues target ON il.target_issue_id = target.id \
         JOIN statuses s ON target.status_id = s.id \
         WHERE il.source_issue_id = ?"
    )
    .bind(issue_id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "issue": issue,
        "subtasks": subtasks,
        "links": links
    })))
}

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
    let assignee_id = payload.assignee_id.or(current_issue.assignee_id);
    let parent_id = payload.parent_id.or(current_issue.parent_id);
    let epic_id = payload.epic_id.or(current_issue.epic_id);
    let story_points = payload.story_points.or(current_issue.story_points);
    let start_date = payload.start_date.or(current_issue.start_date);
    let due_date = payload.due_date.or(current_issue.due_date);
    let position = payload.position.or(current_issue.position);

    sqlx::query(
        "UPDATE issues SET summary = ?, description = ?, issue_type = ?, status_id = ?, priority = ?, assignee_id = ?, parent_id = ?, epic_id = ?, story_points = ?, start_date = ?, due_date = ?, position = ? \
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
    .bind(issue_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Update error: {}", e) })),
        )
    })?;

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

    sqlx::query("UPDATE issues SET status_id = ?, position = ? WHERE id = ?")
        .bind(payload.target_status_id)
        .bind(target_pos)
        .bind(issue_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Move failed: {}", e) })),
            )
        })?;

    // Get target category
    let target_cat: Result<(String, String), _> = sqlx::query_as(
        "SELECT name, category FROM statuses WHERE id = ?"
    )
    .bind(payload.target_status_id)
    .fetch_one(&state.db)
    .await;

    if let Ok((status_name, category)) = target_cat {
        // Log movement in activity stream
        let _ = sqlx::query(
            "INSERT INTO activity_logs (project_id, issue_id, user_id, action, details) \
             VALUES (?, ?, ?, 'MOVE_ISSUE', ?)"
        )
        .bind(current_issue.project_id)
        .bind(issue_id)
        .bind(auth.user_id)
        .bind(format!("Moved {} to {}", current_issue.key, status_name))
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
    }

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

pub async fn delete_issue(
    auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let issue = fetch_single_issue(&state.db, issue_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Issue not found" })),
        )
    })?;

    sqlx::query("DELETE FROM issues WHERE id = ?")
        .bind(issue_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Delete failed: {}", e) })),
            )
        })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "ISSUE_DELETED".to_string(),
        project_id: Some(issue.project_id),
        user_id: Some(auth.user_id),
        data: json!({ "issue_id": issue_id, "key": issue.key }),
    });

    Ok(Json(json!({ "message": "Issue deleted successfully" })))
}

pub async fn create_issue_link(
    _auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateIssueLinkPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = sqlx::query(
        "INSERT INTO issue_links (source_issue_id, target_issue_id, link_type) VALUES (?, ?, ?)"
    )
    .bind(issue_id)
    .bind(payload.target_issue_id)
    .bind(&payload.link_type)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to create link: {}", e) })),
        )
    })?;

    Ok((StatusCode::CREATED, Json(json!({ "message": "Issue linked successfully" }))))
}

// Helper: validate custom workflow transition rules (FR-2.3)
async fn validate_transition(
    pool: &sqlx::MySqlPool,
    project_id: i64,
    from_status_id: i64,
    to_status_id: i64,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    // Check if there are ANY transition rules defined for from_status_id in this project
    let rules_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM workflow_transitions WHERE project_id = ? AND from_status_id = ?"
    )
    .bind(project_id)
    .bind(from_status_id)
    .fetch_one(pool)
    .await
    .unwrap_or((0,));

    if rules_count.0 > 0 {
        // If rules exist for this status, transition MUST be in the table
        let allowed: Result<(i64,), _> = sqlx::query_as(
            "SELECT id FROM workflow_transitions WHERE project_id = ? AND from_status_id = ? AND to_status_id = ?"
        )
        .bind(project_id)
        .bind(from_status_id)
        .bind(to_status_id)
        .fetch_one(pool)
        .await;

        if allowed.is_err() {
            let from_name: (String,) = sqlx::query_as("SELECT name FROM statuses WHERE id = ?")
                .bind(from_status_id)
                .fetch_one(pool)
                .await
                .unwrap_or(("Origin Status".to_string(),));
            let to_name: (String,) = sqlx::query_as("SELECT name FROM statuses WHERE id = ?")
                .bind(to_status_id)
                .fetch_one(pool)
                .await
                .unwrap_or(("Destination Status".to_string(),));

            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": format!("Workflow rule violation: Transition directly from '{}' to '{}' is not allowed.", from_name.0, to_name.0)
                })),
            ));
        }
    }

    Ok(())
}

async fn fetch_single_issue(
    pool: &sqlx::MySqlPool,
    issue_id: i64,
) -> Result<Issue, sqlx::Error> {
    sqlx::query_as::<_, Issue>(
        "SELECT i.id, i.project_id, i.parent_id, i.epic_id, i.`key`, i.summary, i.description, \
                i.issue_type, i.status_id, i.priority, i.assignee_id, i.reporter_id, i.story_points, \
                i.start_date, i.due_date, i.position, i.created_at, i.updated_at, \
                s.name AS status_name, s.category AS status_category, s.color AS status_color, \
                ua.full_name AS assignee_name, ua.avatar_url AS assignee_avatar, \
                ur.full_name AS reporter_name, \
                ep.summary AS epic_summary, \
                p.`key` AS parent_key, p.summary AS parent_summary, \
                (SELECT COUNT(*) FROM issues sub WHERE sub.parent_id = i.id) AS subtask_count, \
                (SELECT COUNT(*) FROM issues sub JOIN statuses subs ON sub.status_id = subs.id WHERE sub.parent_id = i.id AND subs.category = 'DONE') AS subtask_done_count \
         FROM issues i \
         JOIN statuses s ON i.status_id = s.id \
         LEFT JOIN users ua ON i.assignee_id = ua.id \
         LEFT JOIN users ur ON i.reporter_id = ur.id \
         LEFT JOIN issues ep ON i.epic_id = ep.id \
         LEFT JOIN issues p ON i.parent_id = p.id \
         WHERE i.id = ?"
    )
    .bind(issue_id)
    .fetch_one(pool)
    .await
}
