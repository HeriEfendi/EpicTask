use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::Row;

use crate::{auth::AuthUser, AppState};

/// GET /api/projects/:pid/reports/velocity
pub async fn get_velocity(
    _auth: AuthUser,
    Path(pid): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows = sqlx::query(
        "SELECT 
            DATE_FORMAT(created_at, '%Y-%m') as month,
            CAST(COUNT(*) AS SIGNED) as created_count,
            CAST(SUM(CASE WHEN s.category = 'DONE' THEN 1 ELSE 0 END) AS SIGNED) as resolved_count,
            CAST(COALESCE(SUM(story_points), 0) AS SIGNED) as committed_points,
            CAST(COALESCE(SUM(CASE WHEN s.category = 'DONE' THEN story_points ELSE 0 END), 0) AS SIGNED) as completed_points
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

    let mut velocity = Vec::new();
    let mut total_completed_pts = 0i64;
    let count = rows.len();

    for row in &rows {
        let month: String = row.try_get("month").unwrap_or_default();
        let created_cnt: i64 = row.try_get("created_count").unwrap_or(0);
        let resolved_cnt: i64 = row.try_get("resolved_count").unwrap_or(0);
        let committed_pts: i64 = row.try_get("committed_points").unwrap_or(0);
        let completed_pts: i64 = row.try_get("completed_points").unwrap_or(0);

        total_completed_pts += completed_pts;

        velocity.push(json!({
            "month": month,
            "created_count": created_cnt,
            "resolved_count": resolved_cnt,
            "committed_points": committed_pts,
            "completed_points": completed_pts,
            "predictability_pct": if committed_pts > 0 { ((completed_pts as f64) / (committed_pts as f64) * 100.0).round() } else { 100.0 }
        }));
    }

    let avg_velocity = if count > 0 { (total_completed_pts as f64) / (count as f64) } else { 0.0 };

    Ok(Json(json!({
        "project_id": pid,
        "average_velocity": avg_velocity.round(),
        "total_completed_points": total_completed_pts,
        "sprints": velocity
    })))
}

