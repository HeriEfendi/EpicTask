use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    models::{AddMemberPayload, CreateWorkspacePayload, UpdateMemberRolePayload, Workspace, WorkspaceMember},
    AppState,
};

pub async fn list_workspaces(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let workspaces = sqlx::query_as::<_, Workspace>(
        "SELECT w.id, w.name, w.created_at \
         FROM workspaces w \
         JOIN workspace_members wm ON w.id = wm.workspace_id \
         WHERE wm.user_id = ? \
         ORDER BY w.id ASC"
    )
    .bind(auth.user_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok(Json(workspaces))
}

pub async fn create_workspace(
    auth: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateWorkspacePayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if payload.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Workspace name is required" })),
        ));
    }

    let res = sqlx::query("INSERT INTO workspaces (name) VALUES (?)")
        .bind(&payload.name)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {}", e) })),
            )
        })?;

    let ws_id = res.last_insert_id() as i64;

    let _ = sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role) VALUES (?, ?, 'OWNER')"
    )
    .bind(ws_id)
    .bind(auth.user_id)
    .execute(&state.db)
    .await;

    // Create default project in new workspace
    let proj_res = sqlx::query(
        "INSERT INTO projects (workspace_id, name, `key`, project_type, current_issue_counter) \
         VALUES (?, 'General Project', 'GEN', 'KANBAN', 0)"
    )
    .bind(ws_id)
    .execute(&state.db)
    .await;

    if let Ok(p_res) = proj_res {
        let pid = p_res.last_insert_id() as i64;
        let _ = sqlx::query("INSERT INTO statuses (project_id, name, category, position, color) VALUES \
            (?, 'Backlog', 'TODO', 0, '#64748b'), \
            (?, 'To Do', 'TODO', 1, '#3b82f6'), \
            (?, 'In Progress', 'IN_PROGRESS', 2, '#f59e0b'), \
            (?, 'In Review', 'IN_PROGRESS', 3, '#8b5cf6'), \
            (?, 'Done', 'DONE', 4, '#10b981')")
        .bind(pid).bind(pid).bind(pid).bind(pid).bind(pid)
        .execute(&state.db)
        .await;
    }

    let ws = sqlx::query_as::<_, Workspace>("SELECT id, name, created_at FROM workspaces WHERE id = ?")
        .bind(ws_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {}", e) })),
            )
        })?;

    Ok((StatusCode::CREATED, Json(ws)))
}

pub async fn get_workspace(
    auth: AuthUser,
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ws = sqlx::query_as::<_, Workspace>("SELECT id, name, created_at FROM workspaces WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Workspace not found" })),
            )
        })?;

    // Check membership
    let member_opt: Result<(String,), _> = sqlx::query_as(
        "SELECT role FROM workspace_members WHERE workspace_id = ? AND user_id = ?"
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_one(&state.db)
    .await;

    let user_role = member_opt.map(|(r,)| r).unwrap_or_else(|_| "VIEWER".to_string());

    let members = sqlx::query_as::<_, WorkspaceMember>(
        "SELECT wm.workspace_id, wm.user_id, wm.role, wm.joined_at, u.email, u.full_name, u.avatar_url \
         FROM workspace_members wm \
         JOIN users u ON wm.user_id = u.id \
         WHERE wm.workspace_id = ? \
         ORDER BY wm.joined_at ASC"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "workspace": ws,
        "current_user_role": user_role,
        "members": members
    })))
}

pub async fn add_member(
    auth: AuthUser,
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<AddMemberPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Check permission (OWNER or ADMIN)
    let role_check: Result<(String,), _> = sqlx::query_as(
        "SELECT role FROM workspace_members WHERE workspace_id = ? AND user_id = ?"
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_one(&state.db)
    .await;

    match role_check {
        Ok((r,)) if r == "OWNER" || r == "ADMIN" => {},
        _ => return Err((StatusCode::FORBIDDEN, Json(json!({ "error": "Only owners or admins can invite members" })))),
    }

    // Find user by email
    let target_user: Result<(i64,), _> = sqlx::query_as("SELECT id FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_one(&state.db)
        .await;

    let target_id = target_user.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "No user found with this email" })),
        )
    })?.0;

    let role = payload.role.unwrap_or_else(|| "MEMBER".to_string());

    let _ = sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role) VALUES (?, ?, ?) \
         ON DUPLICATE KEY UPDATE role = VALUES(role)"
    )
    .bind(id)
    .bind(target_id)
    .bind(&role)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok(Json(json!({ "message": "Member added successfully" })))
}

pub async fn update_member_role(
    auth: AuthUser,
    Path((ws_id, uid)): Path<(i64, i64)>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateMemberRolePayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Only owner can change roles
    let owner_check: Result<(String,), _> = sqlx::query_as(
        "SELECT role FROM workspace_members WHERE workspace_id = ? AND user_id = ?"
    )
    .bind(ws_id)
    .bind(auth.user_id)
    .fetch_one(&state.db)
    .await;

    if !matches!(owner_check, Ok((ref r,)) if r == "OWNER") {
        return Err((StatusCode::FORBIDDEN, Json(json!({ "error": "Only workspace owner can modify roles" }))));
    }

    let _ = sqlx::query("UPDATE workspace_members SET role = ? WHERE workspace_id = ? AND user_id = ?")
        .bind(&payload.role)
        .bind(ws_id)
        .bind(uid)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {}", e) })),
            )
        })?;

    Ok(Json(json!({ "message": "Role updated successfully" })))
}
