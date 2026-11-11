use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::Row;

use crate::{auth::AuthUser, AppState};

/// GET /api/projects/:pid/reports/cumulative-flow
pub async fn get_cumulative_flow(
    _auth: AuthUser,
    Path(pid): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows = sqlx::query(
        "SELECT 
            DATE_FORMAT(created_at, '%Y-%m') as month,
            CAST(SUM(CASE WHEN s.name = 'Backlog' THEN 1 ELSE 0 END) AS SIGNED) as backlog,
            CAST(SUM(CASE WHEN s.name = 'To Do' THEN 1 ELSE 0 END) AS SIGNED) as todo,
            CAST(SUM(CASE WHEN s.name = 'In Progress' THEN 1 ELSE 0 END) AS SIGNED) as in_progress,
            CAST(SUM(CASE WHEN s.name = 'In Review' THEN 1 ELSE 0 END) AS SIGNED) as in_review,
            CAST(SUM(CASE WHEN s.name = 'Done' THEN 1 ELSE 0 END) AS SIGNED) as done,
            CAST(COUNT(*) AS SIGNED) as total
         FROM issues i
         JOIN statuses s ON i.status_id = s.id
         WHERE i.project_id = ?
         GROUP BY month
         ORDER BY month ASC"
    )
    .bind(pid)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut points = Vec::new();
    let mut cum_done = 0i64;
    let mut cum_review = 0i64;
    let mut cum_progress = 0i64;
    let mut cum_todo = 0i64;
    let mut cum_backlog = 0i64;

    for row in rows {
        let month: String = row.try_get("month").unwrap_or_default();
        let backlog: i64 = row.try_get("backlog").unwrap_or(0);
        let todo: i64 = row.try_get("todo").unwrap_or(0);
        let in_progress: i64 = row.try_get("in_progress").unwrap_or(0);
        let in_review: i64 = row.try_get("in_review").unwrap_or(0);
        let done: i64 = row.try_get("done").unwrap_or(0);

        cum_backlog += backlog;
        cum_todo += todo;
        cum_progress += in_progress;
        cum_review += in_review;
        cum_done += done;

        points.push(json!({
            "month": month,
            "backlog": backlog,
            "todo": todo,
            "in_progress": in_progress,
            "in_review": in_review,
            "done": done,
            "cumulative_done": cum_done,
            "cumulative_total": cum_done + cum_review + cum_progress + cum_todo + cum_backlog
        }));
    }

    Ok(Json(json!({
        "project_id": pid,
        "timeline": points
    })))
}

