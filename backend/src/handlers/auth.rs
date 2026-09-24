use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::json;

use crate::{
    auth::{create_token, AuthUser},
    models::{AuthResponse, LoginPayload, RegisterPayload, User, UserPublic, Workspace},
    AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if payload.email.trim().is_empty() || payload.password.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Email and password are required" })),
        ));
    }

    let password_hash = hash(&payload.password, DEFAULT_COST).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Hashing error: {}", e) })),
        )
    })?;

    // Check if user exists
    let existing: Result<(i64,), _> = sqlx::query_as("SELECT id FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_one(&state.db)
        .await;

    if existing.is_ok() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({ "error": "User with this email already exists" })),
        ));
    }

    // Insert user
    let res = sqlx::query(
        "INSERT INTO users (email, password_hash, full_name, avatar_url) VALUES (?, ?, ?, ?)"
    )
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.full_name)
    .bind(format!("https://api.dicebear.com/7.x/avataaars/svg?seed={}", &payload.email))
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    let user_id = res.last_insert_id() as i64;

    // Create default workspace for user
    let ws_name = format!("{}'s Workspace", payload.full_name);
    let ws_res = sqlx::query("INSERT INTO workspaces (name) VALUES (?)")
        .bind(&ws_name)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Workspace error: {}", e) })),
            )
        })?;

    let ws_id = ws_res.last_insert_id() as i64;

    // Add user as OWNER
    let _ = sqlx::query(
        "INSERT INTO workspace_members (workspace_id, user_id, role) VALUES (?, ?, 'OWNER')"
    )
    .bind(ws_id)
    .bind(user_id)
    .execute(&state.db)
    .await;

    // Create default Project: "Software Development" with key "DEV"
    let proj_res = sqlx::query(
        "INSERT INTO projects (workspace_id, name, `key`, project_type, current_issue_counter) \
         VALUES (?, 'Software Development', 'DEV', 'KANBAN', 0)"
    )
    .bind(ws_id)
    .execute(&state.db)
    .await;

    if let Ok(p_res) = proj_res {
        let proj_id = p_res.last_insert_id() as i64;

        // Create default statuses
        let _ = sqlx::query("INSERT INTO statuses (project_id, name, category, position, color) VALUES \
            (?, 'Backlog', 'TODO', 0, '#64748b'), \
            (?, 'To Do', 'TODO', 1, '#3b82f6'), \
            (?, 'In Progress', 'IN_PROGRESS', 2, '#f59e0b'), \
            (?, 'In Review', 'IN_PROGRESS', 3, '#8b5cf6'), \
            (?, 'Done', 'DONE', 4, '#10b981')")
        .bind(proj_id).bind(proj_id).bind(proj_id).bind(proj_id).bind(proj_id)
        .execute(&state.db)
        .await;

        // Create default automation: Cascade Done to Subtasks
        let _ = sqlx::query(
            "INSERT INTO automation_rules (project_id, name, trigger_type, trigger_config, action_type, action_config, is_active) \
             VALUES (?, 'Cascade Done to Subtasks', 'STATUS_CHANGED', '{\"category\":\"DONE\"}', 'CASCADE_SUBTASKS_DONE', '{}', TRUE)"
        )
        .bind(proj_id)
        .execute(&state.db)
        .await;
    }

    let token = create_token(user_id, &payload.email, &state.config.jwt_secret).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Token error: {}", e) })),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user: UserPublic {
                id: user_id,
                email: payload.email,
                full_name: payload.full_name,
                avatar_url: Some(format!("https://api.dicebear.com/7.x/avataaars/svg?seed={}", user_id)),
            },
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, full_name, avatar_url, created_at FROM users WHERE email = ?"
    )
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid email or password" })),
        )
    })?;

    let is_valid = verify(&payload.password, &user.password_hash).unwrap_or(false);
    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid email or password" })),
        ));
    }

    let token = create_token(user.id, &user.email, &state.config.jwt_secret).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Token error: {}", e) })),
        )
    })?;

    Ok(Json(AuthResponse {
        token,
        user: UserPublic {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            avatar_url: user.avatar_url,
        },
    }))
}

pub async fn me(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as::<_, UserPublic>(
        "SELECT id, email, full_name, avatar_url FROM users WHERE id = ?"
    )
    .bind(auth.user_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" })),
        )
    })?;

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
    .unwrap_or_default();

    Ok(Json(json!({
        "user": user,
        "workspaces": workspaces
    })))
}

pub async fn list_users(
    _auth: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let users = sqlx::query_as::<_, UserPublic>(
        "SELECT id, email, full_name, avatar_url FROM users ORDER BY full_name ASC"
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Ok(Json(users))
}
