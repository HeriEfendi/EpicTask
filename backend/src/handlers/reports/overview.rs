use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::Row;

use crate::{auth::AuthUser, AppState};

/// GET /api/projects/:pid/reports/overview
pub async fn get_overview(
    _auth: AuthUser,
    Path(pid): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // 1. KPI Counts
    let issue_stats = sqlx::query(
        "SELECT 
            CAST(COUNT(*) AS SIGNED) as total_issues,
            CAST(COALESCE(SUM(story_points), 0) AS SIGNED) as total_points,
            CAST(SUM(CASE WHEN s.category = 'DONE' THEN 1 ELSE 0 END) AS SIGNED) as done_issues,
            CAST(COALESCE(SUM(CASE WHEN s.category = 'DONE' THEN story_points ELSE 0 END), 0) AS SIGNED) as done_points,
            CAST(SUM(CASE WHEN s.category = 'IN_PROGRESS' THEN 1 ELSE 0 END) AS SIGNED) as in_progress_issues,
            CAST(SUM(CASE WHEN s.category = 'TODO' THEN 1 ELSE 0 END) AS SIGNED) as todo_issues
         FROM issues i
         JOIN statuses s ON i.status_id = s.id
         WHERE i.project_id = ?"
    )
    .bind(pid)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let total_issues: i64 = issue_stats.try_get("total_issues").unwrap_or(0);
    let total_points: i64 = issue_stats.try_get("total_points").unwrap_or(0);
    let done_issues: i64 = issue_stats.try_get("done_issues").unwrap_or(0);
    let done_points: i64 = issue_stats.try_get("done_points").unwrap_or(0);
    let in_progress_issues: i64 = issue_stats.try_get("in_progress_issues").unwrap_or(0);
    let todo_issues: i64 = issue_stats.try_get("todo_issues").unwrap_or(0);

    // 2. Total Time Logged
    let time_stats = sqlx::query(
        "SELECT 
            CAST(COALESCE(SUM(t.time_spent_seconds), 0) AS SIGNED) as total_seconds,
            CAST(COUNT(t.id) AS SIGNED) as log_count
         FROM time_logs t
         JOIN issues i ON t.issue_id = i.id
         WHERE i.project_id = ?"
    )
    .bind(pid)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let total_seconds: i64 = time_stats.try_get("total_seconds").unwrap_or(0);
    let log_count: i64 = time_stats.try_get("log_count").unwrap_or(0);

    // 3. Status breakdown
    let status_rows = sqlx::query(
        "SELECT s.id, s.name, s.category, s.color, CAST(COUNT(i.id) AS SIGNED) as count
         FROM statuses s
         LEFT JOIN issues i ON i.status_id = s.id AND i.project_id = s.project_id
         WHERE s.project_id = ?
         GROUP BY s.id, s.name, s.category, s.color, s.position
         ORDER BY s.position ASC"
    )
    .bind(pid)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut status_breakdown = Vec::new();
    for row in status_rows {
        let name: String = row.try_get("name").unwrap_or_default();
        let cat: String = row.try_get("category").unwrap_or_default();
        let color: Option<String> = row.try_get("color").ok();
        let count: i64 = row.try_get("count").unwrap_or(0);
        status_breakdown.push(json!({
            "name": name,
            "category": cat,
            "color": color.unwrap_or_else(|| "#64748b".into()),
            "count": count
        }));
    }

    // 4. Issue Type breakdown
    let type_rows = sqlx::query(
        "SELECT issue_type, CAST(COUNT(*) AS SIGNED) as count, CAST(COALESCE(SUM(story_points), 0) AS SIGNED) as points
         FROM issues
         WHERE project_id = ?
         GROUP BY issue_type
         ORDER BY count DESC"
    )
    .bind(pid)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut type_breakdown = Vec::new();
    for row in type_rows {
        let itype: String = row.try_get("issue_type").unwrap_or_default();
        let count: i64 = row.try_get("count").unwrap_or(0);
        let points: i64 = row.try_get("points").unwrap_or(0);
        type_breakdown.push(json!({
            "issue_type": itype,
            "count": count,
            "points": points
        }));
    }

    // 5. Priority breakdown
    let priority_rows = sqlx::query(
        "SELECT priority, CAST(COUNT(*) AS SIGNED) as count
         FROM issues
         WHERE project_id = ?
         GROUP BY priority"
    )
    .bind(pid)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut priority_breakdown = Vec::new();
    for row in priority_rows {
        let prio: String = row.try_get("priority").unwrap_or_default();
        let count: i64 = row.try_get("count").unwrap_or(0);
        priority_breakdown.push(json!({
            "priority": prio,
            "count": count
        }));
    }

    // 6. Team Capacity / Workload summary
    let member_rows = sqlx::query(
        "SELECT 
            u.id, u.full_name, u.avatar_url,
            CAST(COUNT(DISTINCT i.id) AS SIGNED) as assigned_issues,
            CAST(SUM(CASE WHEN s.category = 'DONE' THEN 1 ELSE 0 END) AS SIGNED) as done_issues,
            CAST(COALESCE(SUM(i.story_points), 0) AS SIGNED) as assigned_points,
            CAST(COALESCE(tl_agg.total_seconds, 0) AS SIGNED) as logged_seconds
         FROM users u
         JOIN workspace_members wm ON wm.user_id = u.id
         JOIN projects p ON p.workspace_id = wm.workspace_id AND p.id = ?
         LEFT JOIN issues i ON i.assignee_id = u.id AND i.project_id = p.id
         LEFT JOIN statuses s ON i.status_id = s.id
         LEFT JOIN (
             SELECT tl.user_id, CAST(SUM(tl.time_spent_seconds) AS SIGNED) as total_seconds
             FROM time_logs tl
             JOIN issues iss ON tl.issue_id = iss.id
             WHERE iss.project_id = ?
             GROUP BY tl.user_id
         ) tl_agg ON tl_agg.user_id = u.id
         GROUP BY u.id, u.full_name, u.avatar_url, tl_agg.total_seconds
         ORDER BY logged_seconds DESC"
    )
    .bind(pid)
    .bind(pid)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut team_workload = Vec::new();
    for row in member_rows {
        let uid: i64 = row.try_get("id").unwrap_or(0);
        let name: String = row.try_get("full_name").unwrap_or_default();
        let avatar: Option<String> = row.try_get("avatar_url").ok();
        let assigned_issues: i64 = row.try_get("assigned_issues").unwrap_or(0);
        let done_issues_cnt: i64 = row.try_get("done_issues").unwrap_or(0);
        let assigned_points: i64 = row.try_get("assigned_points").unwrap_or(0);
        let logged_seconds: i64 = row.try_get("logged_seconds").unwrap_or(0);

        team_workload.push(json!({
            "user_id": uid,
            "full_name": name,
            "avatar_url": avatar,
            "assigned_issues": assigned_issues,
            "done_issues": done_issues_cnt,
            "assigned_points": assigned_points,
            "logged_hours": (logged_seconds as f64) / 3600.0,
            "logged_seconds": logged_seconds
        }));
    }

    Ok(Json(json!({
        "project_id": pid,
        "kpi": {
            "total_issues": total_issues,
            "total_points": total_points,
            "done_issues": done_issues,
            "done_points": done_points,
            "in_progress_issues": in_progress_issues,
            "todo_issues": todo_issues,
            "total_logged_hours": (total_seconds as f64) / 3600.0,
            "total_logged_seconds": total_seconds,
            "total_time_logs": log_count,
            "completion_rate": if total_issues > 0 { ((done_issues as f64) / (total_issues as f64) * 100.0).round() } else { 0.0 }
        },
        "status_breakdown": status_breakdown,
        "type_breakdown": type_breakdown,
        "priority_breakdown": priority_breakdown,
        "team_workload": team_workload
    })))
}

