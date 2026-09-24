use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    models::{CreateTimeLogPayload, TimeLog},
    AppState,
};

pub async fn list_time_logs(
    _auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let logs = sqlx::query_as::<_, TimeLog>(
        "SELECT tl.id, tl.issue_id, tl.user_id, tl.time_spent_seconds, tl.description, tl.logged_at, \
                u.full_name AS user_name, u.avatar_url AS user_avatar \
         FROM time_logs tl \
         LEFT JOIN users u ON tl.user_id = u.id \
         WHERE tl.issue_id = ? \
         ORDER BY tl.logged_at DESC"
    )
    .bind(issue_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    let total_seconds: i64 = logs.iter().map(|l| l.time_spent_seconds).sum();

    Ok(Json(json!({
        "time_logs": logs,
        "total_seconds": total_seconds,
        "total_hours": (total_seconds as f64) / 3600.0
    })))
}

pub async fn create_time_log(
    auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateTimeLogPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if payload.time_spent_seconds <= 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Time spent must be greater than zero" })),
        ));
    }

    let res = sqlx::query(
        "INSERT INTO time_logs (issue_id, user_id, time_spent_seconds, description) \
         VALUES (?, ?, ?, ?)"
    )
    .bind(issue_id)
    .bind(auth.user_id)
    .bind(payload.time_spent_seconds)
    .bind(&payload.description)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    let log_id = res.last_insert_id() as i64;

    // Log in activity
    let issue_meta: Result<(i64, String), _> = sqlx::query_as("SELECT project_id, `key` FROM issues WHERE id = ?")
        .bind(issue_id)
        .fetch_one(&state.db)
        .await;

    if let Ok((project_id, key)) = issue_meta {
        let hrs = (payload.time_spent_seconds as f64) / 3600.0;
        let _ = sqlx::query(
            "INSERT INTO activity_logs (project_id, issue_id, user_id, action, details) \
             VALUES (?, ?, ?, 'LOG_TIME', ?)"
        )
        .bind(project_id)
        .bind(issue_id)
        .bind(auth.user_id)
        .bind(format!("Logged {:.1}h on {}", hrs, key))
        .execute(&state.db)
        .await;
    }

    Ok((StatusCode::CREATED, Json(json!({ "id": log_id, "message": "Time logged successfully" }))))
}
