use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    models::ActivityLog,
    AppState,
};

pub async fn list_activities(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let activities = sqlx::query_as::<_, ActivityLog>(
        "SELECT a.id, a.project_id, a.issue_id, a.user_id, a.action, a.details, a.created_at, \
                u.full_name AS user_name, i.`key` AS issue_key \
         FROM activity_logs a \
         LEFT JOIN users u ON a.user_id = u.id \
         LEFT JOIN issues i ON a.issue_id = i.id \
         WHERE a.project_id = ? \
         ORDER BY a.created_at DESC \
         LIMIT 50"
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

    Ok(Json(activities))
}
