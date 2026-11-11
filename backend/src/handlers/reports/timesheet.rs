use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::Row;

use crate::{auth::AuthUser, AppState};

/// GET /api/projects/:pid/reports/timesheet
pub async fn get_timesheet(
    _auth: AuthUser,
    Path(pid): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // 1. By User
    let user_rows = sqlx::query(
        "SELECT 
            u.id as user_id,
            u.full_name as user_name,
            u.avatar_url,
            CAST(COUNT(t.id) AS SIGNED) as log_count,
            CAST(COUNT(DISTINCT t.issue_id) AS SIGNED) as issue_count,
            CAST(COALESCE(SUM(t.time_spent_seconds), 0) AS SIGNED) as total_seconds
         FROM users u
         LEFT JOIN time_logs t ON t.user_id = u.id
         JOIN issues i ON t.issue_id = i.id AND i.project_id = ?
         GROUP BY u.id, u.full_name, u.avatar_url
         ORDER BY total_seconds DESC"
    )
    .bind(pid)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut user_summary = Vec::new();
    let mut grand_total_seconds = 0i64;

    for row in user_rows {
        let uid: i64 = row.try_get("user_id").unwrap_or(0);
        let name: String = row.try_get("user_name").unwrap_or_default();
        let avatar: Option<String> = row.try_get("avatar_url").ok();
        let log_cnt: i64 = row.try_get("log_count").unwrap_or(0);
        let issue_cnt: i64 = row.try_get("issue_count").unwrap_or(0);
        let secs: i64 = row.try_get("total_seconds").unwrap_or(0);

        grand_total_seconds += secs;

        user_summary.push(json!({
            "user_id": uid,
            "user_name": name,
            "avatar_url": avatar,
            "log_count": log_cnt,
            "issue_count": issue_cnt,
            "total_seconds": secs,
            "total_hours": (secs as f64) / 3600.0
        }));
    }

    // 2. By Month
    let month_rows = sqlx::query(
        "SELECT 
            DATE_FORMAT(t.logged_at, '%Y-%m') as month,
            CAST(COUNT(t.id) AS SIGNED) as log_count,
            CAST(COALESCE(SUM(t.time_spent_seconds), 0) AS SIGNED) as total_seconds
         FROM time_logs t
         JOIN issues i ON t.issue_id = i.id AND i.project_id = ?
         GROUP BY month
         ORDER BY month ASC"
    )
    .bind(pid)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut monthly_summary = Vec::new();
    for row in month_rows {
        let m: String = row.try_get("month").unwrap_or_default();
        let log_cnt: i64 = row.try_get("log_count").unwrap_or(0);
        let secs: i64 = row.try_get("total_seconds").unwrap_or(0);
        monthly_summary.push(json!({
            "month": m,
            "log_count": log_cnt,
            "total_seconds": secs,
            "total_hours": (secs as f64) / 3600.0
        }));
    }

    // 3. Recent 50 Worklogs
    let recent_rows = sqlx::query(
        "SELECT 
            t.id, t.time_spent_seconds, t.description, t.logged_at,
            i.id as issue_id, i.`key` as issue_key, i.summary as issue_summary,
            u.id as user_id, u.full_name as user_name, u.avatar_url
         FROM time_logs t
         JOIN issues i ON t.issue_id = i.id AND i.project_id = ?
         LEFT JOIN users u ON t.user_id = u.id
         ORDER BY t.logged_at DESC
         LIMIT 60"
    )
    .bind(pid)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut recent_logs = Vec::new();
    for row in recent_rows {
        let id: i64 = row.try_get("id").unwrap_or(0);
        let secs: i64 = row.try_get("time_spent_seconds").unwrap_or(0);
        let desc: Option<String> = row.try_get("description").ok();
        let logged_at: Option<chrono::DateTime<chrono::Utc>> = row.try_get("logged_at").ok();
        let iss_id: i64 = row.try_get("issue_id").unwrap_or(0);
        let iss_key: String = row.try_get("issue_key").unwrap_or_default();
        let iss_summary: String = row.try_get("issue_summary").unwrap_or_default();
        let user_id: Option<i64> = row.try_get("user_id").ok();
        let user_name: Option<String> = row.try_get("user_name").ok();
        let avatar: Option<String> = row.try_get("avatar_url").ok();

        recent_logs.push(json!({
            "id": id,
            "time_spent_seconds": secs,
            "hours": (secs as f64) / 3600.0,
            "description": desc,
            "logged_at": logged_at,
            "issue_id": iss_id,
            "issue_key": iss_key,
            "issue_summary": iss_summary,
            "user_id": user_id,
            "user_name": user_name,
            "avatar_url": avatar
        }));
    }

    Ok(Json(json!({
        "project_id": pid,
        "grand_total_hours": (grand_total_seconds as f64) / 3600.0,
        "grand_total_seconds": grand_total_seconds,
        "users": user_summary,
        "monthly": monthly_summary,
        "recent_logs": recent_logs
    })))
}

