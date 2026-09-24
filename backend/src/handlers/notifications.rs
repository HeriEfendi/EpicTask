use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    models::Notification,
    AppState,
};

pub async fn list_notifications(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let notifications = sqlx::query_as::<_, Notification>(
        "SELECT n.id, n.recipient_id, n.sender_id, n.issue_id, n.title, n.message, \
                n.action_type, n.is_read, n.created_at, \
                u.full_name AS sender_name, i.`key` AS issue_key \
         FROM notifications n \
         LEFT JOIN users u ON n.sender_id = u.id \
         LEFT JOIN issues i ON n.issue_id = i.id \
         WHERE n.recipient_id = ? \
         ORDER BY n.created_at DESC \
         LIMIT 50"
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

    let unread_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM notifications WHERE recipient_id = ? AND is_read = FALSE"
    )
    .bind(auth.user_id)
    .fetch_one(&state.db)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "notifications": notifications,
        "unread_count": unread_count.0
    })))
}

pub async fn mark_as_read(
    auth: AuthUser,
    Path(notification_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = sqlx::query(
        "UPDATE notifications SET is_read = TRUE WHERE id = ? AND recipient_id = ?"
    )
    .bind(notification_id)
    .bind(auth.user_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok(Json(json!({ "message": "Notification marked as read" })))
}

pub async fn mark_all_as_read(
    auth: AuthUser,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = sqlx::query(
        "UPDATE notifications SET is_read = TRUE WHERE recipient_id = ?"
    )
    .bind(auth.user_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok(Json(json!({ "message": "All notifications marked as read" })))
}
