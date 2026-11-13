use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    handlers::issues::helpers::fetch_single_issue,
    models::{AddWatcherPayload, IssueWatcher},
    ws::WsBroadcastMessage,
    AppState,
};

pub async fn list_issue_watchers(
    _auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sql = "SELECT u.id AS user_id, u.full_name, u.email, u.avatar_url, iw.created_at \
               FROM issue_watchers iw \
               JOIN users u ON iw.user_id = u.id \
               WHERE iw.issue_id = ? \
               ORDER BY iw.created_at ASC";

    let watchers = sqlx::query_as::<_, IssueWatcher>(sql)
        .bind(issue_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to list watchers: {}", e) })),
            )
        })?;

    Ok(Json(watchers))
}

pub async fn add_issue_watcher(
    auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<AddWatcherPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let issue = fetch_single_issue(&state.db, issue_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Issue not found" })),
        )
    })?;

    let target_user_id = payload.user_id.unwrap_or(auth.user_id);

    // Verify target user exists
    let user_exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE id = ?")
        .bind(target_user_id)
        .fetch_one(&state.db)
        .await
        .unwrap_or((0,));

    if user_exists.0 == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" })),
        ));
    }

    sqlx::query(
        "INSERT INTO issue_watchers (issue_id, user_id) VALUES (?, ?) \
         ON DUPLICATE KEY UPDATE issue_id = issue_id"
    )
    .bind(issue_id)
    .bind(target_user_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to add watcher: {}", e) })),
        )
    })?;

    // Log activity
    let _ = sqlx::query(
        "INSERT INTO activity_logs (project_id, issue_id, user_id, action, details) \
         VALUES (?, ?, ?, 'WATCH_ISSUE', ?)"
    )
    .bind(issue.project_id)
    .bind(issue_id)
    .bind(auth.user_id)
    .bind(format!("User {} started watching {}", target_user_id, issue.key))
    .execute(&state.db)
    .await;

    // Fetch updated watchers list
    let sql = "SELECT u.id AS user_id, u.full_name, u.email, u.avatar_url, iw.created_at \
               FROM issue_watchers iw \
               JOIN users u ON iw.user_id = u.id \
               WHERE iw.issue_id = ? \
               ORDER BY iw.created_at ASC";

    let watchers = sqlx::query_as::<_, IssueWatcher>(sql)
        .bind(issue_id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "WATCHER_ADDED".to_string(),
        project_id: Some(issue.project_id),
        user_id: Some(auth.user_id),
        data: json!({
            "issue_id": issue_id,
            "user_id": target_user_id,
            "watchers": &watchers,
        }),
    });

    Ok(Json(watchers))
}

pub async fn remove_issue_watcher(
    auth: AuthUser,
    Path((issue_id, user_id)): Path<(i64, i64)>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let issue = fetch_single_issue(&state.db, issue_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Issue not found" })),
        )
    })?;

    sqlx::query("DELETE FROM issue_watchers WHERE issue_id = ? AND user_id = ?")
        .bind(issue_id)
        .bind(user_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to remove watcher: {}", e) })),
            )
        })?;

    // Log activity
    let _ = sqlx::query(
        "INSERT INTO activity_logs (project_id, issue_id, user_id, action, details) \
         VALUES (?, ?, ?, 'UNWATCH_ISSUE', ?)"
    )
    .bind(issue.project_id)
    .bind(issue_id)
    .bind(auth.user_id)
    .bind(format!("User {} stopped watching {}", user_id, issue.key))
    .execute(&state.db)
    .await;

    // Fetch updated watchers list
    let sql = "SELECT u.id AS user_id, u.full_name, u.email, u.avatar_url, iw.created_at \
               FROM issue_watchers iw \
               JOIN users u ON iw.user_id = u.id \
               WHERE iw.issue_id = ? \
               ORDER BY iw.created_at ASC";

    let watchers = sqlx::query_as::<_, IssueWatcher>(sql)
        .bind(issue_id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "WATCHER_REMOVED".to_string(),
        project_id: Some(issue.project_id),
        user_id: Some(auth.user_id),
        data: json!({
            "issue_id": issue_id,
            "user_id": user_id,
            "watchers": &watchers,
        }),
    });

    Ok(Json(watchers))
}

