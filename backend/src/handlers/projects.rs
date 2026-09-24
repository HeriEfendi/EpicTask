use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    models::{
        CreateProjectPayload, CreateStatusPayload, CreateTransitionPayload, Project, Status,
        WorkflowTransition,
    },
    AppState,
};

pub async fn list_projects(
    _auth: AuthUser,
    Path(workspace_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let projects = sqlx::query_as::<_, Project>(
        "SELECT id, workspace_id, name, `key`, project_type, current_issue_counter, created_at \
         FROM projects \
         WHERE workspace_id = ? \
         ORDER BY id ASC"
    )
    .bind(workspace_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok(Json(projects))
}

pub async fn create_project(
    _auth: AuthUser,
    Path(workspace_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateProjectPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let clean_key = payload.key.trim().to_uppercase();
    if clean_key.is_empty() || clean_key.len() > 10 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Project key must be between 1 and 10 characters" })),
        ));
    }

    let ptype = payload.project_type.unwrap_or_else(|| "KANBAN".to_string());

    let res = sqlx::query(
        "INSERT INTO projects (workspace_id, name, `key`, project_type, current_issue_counter) \
         VALUES (?, ?, ?, ?, 0)"
    )
    .bind(workspace_id)
    .bind(&payload.name)
    .bind(&clean_key)
    .bind(&ptype)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    let pid = res.last_insert_id() as i64;

    // Create default statuses
    let _ = sqlx::query("INSERT INTO statuses (project_id, name, category, position, color) VALUES \
        (?, 'Backlog', 'TODO', 0, '#64748b'), \
        (?, 'To Do', 'TODO', 1, '#3b82f6'), \
        (?, 'In Progress', 'IN_PROGRESS', 2, '#f59e0b'), \
        (?, 'In Review', 'IN_PROGRESS', 3, '#8b5cf6'), \
        (?, 'Done', 'DONE', 4, '#10b981')")
    .bind(pid).bind(pid).bind(pid).bind(pid).bind(pid)
    .execute(&state.db)
    .await;

    // Create default automation
    let _ = sqlx::query(
        "INSERT INTO automation_rules (project_id, name, trigger_type, trigger_config, action_type, action_config, is_active) \
         VALUES (?, 'Cascade Done to Subtasks', 'STATUS_CHANGED', '{\"category\":\"DONE\"}', 'CASCADE_SUBTASKS_DONE', '{}', TRUE)"
    )
    .bind(pid)
    .execute(&state.db)
    .await;

    let project = sqlx::query_as::<_, Project>(
        "SELECT id, workspace_id, name, `key`, project_type, current_issue_counter, created_at \
         FROM projects WHERE id = ?"
    )
    .bind(pid)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok((StatusCode::CREATED, Json(project)))
}

pub async fn get_project(
    _auth: AuthUser,
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let project = sqlx::query_as::<_, Project>(
        "SELECT id, workspace_id, name, `key`, project_type, current_issue_counter, created_at \
         FROM projects WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Project not found" })),
        )
    })?;

    let statuses = sqlx::query_as::<_, Status>(
        "SELECT id, project_id, name, category, position, color \
         FROM statuses WHERE project_id = ? ORDER BY position ASC, id ASC"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let transitions = sqlx::query_as::<_, WorkflowTransition>(
        "SELECT wt.id, wt.project_id, wt.from_status_id, wt.to_status_id, \
                sf.name AS from_status_name, st.name AS to_status_name \
         FROM workflow_transitions wt \
         JOIN statuses sf ON wt.from_status_id = sf.id \
         JOIN statuses st ON wt.to_status_id = st.id \
         WHERE wt.project_id = ?"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "project": project,
        "statuses": statuses,
        "transitions": transitions
    })))
}

pub async fn list_statuses(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let statuses = sqlx::query_as::<_, Status>(
        "SELECT id, project_id, name, category, position, color \
         FROM statuses WHERE project_id = ? ORDER BY position ASC, id ASC"
    )
    .bind(project_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok(Json(statuses))
}

pub async fn create_status(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateStatusPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Determine next position
    let max_pos: (Option<i32>,) = sqlx::query_as(
        "SELECT MAX(position) FROM statuses WHERE project_id = ?"
    )
    .bind(project_id)
    .fetch_one(&state.db)
    .await
    .unwrap_or((Some(0),));

    let next_pos = max_pos.0.unwrap_or(0) + 1;
    let color = payload.color.unwrap_or_else(|| "#6366f1".to_string());

    let res = sqlx::query(
        "INSERT INTO statuses (project_id, name, category, position, color) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(project_id)
    .bind(&payload.name)
    .bind(&payload.category)
    .bind(next_pos)
    .bind(&color)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    let sid = res.last_insert_id() as i64;

    let status = sqlx::query_as::<_, Status>(
        "SELECT id, project_id, name, category, position, color FROM statuses WHERE id = ?"
    )
    .bind(sid)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok((StatusCode::CREATED, Json(status)))
}

pub async fn list_transitions(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let transitions = sqlx::query_as::<_, WorkflowTransition>(
        "SELECT wt.id, wt.project_id, wt.from_status_id, wt.to_status_id, \
                sf.name AS from_status_name, st.name AS to_status_name \
         FROM workflow_transitions wt \
         JOIN statuses sf ON wt.from_status_id = sf.id \
         JOIN statuses st ON wt.to_status_id = st.id \
         WHERE wt.project_id = ?"
    )
    .bind(project_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok(Json(transitions))
}

pub async fn create_transition(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateTransitionPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let res = sqlx::query(
        "INSERT INTO workflow_transitions (project_id, from_status_id, to_status_id) \
         VALUES (?, ?, ?) \
         ON DUPLICATE KEY UPDATE id = id"
    )
    .bind(project_id)
    .bind(payload.from_status_id)
    .bind(payload.to_status_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    let tid = res.last_insert_id() as i64;
    Ok((StatusCode::CREATED, Json(json!({ "id": tid, "message": "Workflow transition rule saved" }))))
}

pub async fn delete_transition(
    _auth: AuthUser,
    Path((_project_id, transition_id)): Path<(i64, i64)>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = sqlx::query("DELETE FROM workflow_transitions WHERE id = ?")
        .bind(transition_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {}", e) })),
            )
        })?;

    Ok(Json(json!({ "message": "Transition rule removed" })))
}
