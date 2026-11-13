use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::Row;

use crate::{auth::AuthUser, AppState};

#[derive(Debug, Deserialize)]
pub struct UserPerformanceQuery {
    pub user_id: Option<i64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// GET /api/projects/:pid/reports/user-performance
pub async fn get_user_performance(
    _auth: AuthUser,
    Path(pid): Path<i64>,
    Query(query): Query<UserPerformanceQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // 1. Fetch all project users who have assigned issues or work logs, plus project team
    let available_user_rows = sqlx::query(
        "SELECT 
            u.id, u.full_name, u.email, u.avatar_url,
            CAST(COUNT(DISTINCT i.id) AS SIGNED) as assigned_count
         FROM users u
         LEFT JOIN issues i ON i.assignee_id = u.id AND i.project_id = ?
         GROUP BY u.id, u.full_name, u.email, u.avatar_url
         ORDER BY assigned_count DESC, u.full_name ASC"
    )
    .bind(pid)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut available_users = Vec::new();
    let mut default_user_id: Option<i64> = None;

    for row in &available_user_rows {
        let uid: i64 = row.try_get("id").unwrap_or(0);
        let name: String = row.try_get("full_name").unwrap_or_default();
        let email: String = row.try_get("email").unwrap_or_default();
        let avatar: Option<String> = row.try_get("avatar_url").ok();
        let cnt: i64 = row.try_get("assigned_count").unwrap_or(0);

        if default_user_id.is_none() && uid > 0 {
            default_user_id = Some(uid);
        }

        available_users.push(json!({
            "id": uid,
            "full_name": name,
            "email": email,
            "avatar_url": avatar,
            "assigned_count": cnt
        }));
    }

    // Determine target user
    let target_user_id = query.user_id.or(default_user_id).unwrap_or(1);

    // Selected user profile
    let user_row = sqlx::query(
        "SELECT id, full_name, email, avatar_url FROM users WHERE id = ?"
    )
    .bind(target_user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let selected_user = if let Some(row) = user_row {
        json!({
            "id": row.try_get::<i64, _>("id").unwrap_or(target_user_id),
            "full_name": row.try_get::<String, _>("full_name").unwrap_or_default(),
            "email": row.try_get::<String, _>("email").unwrap_or_default(),
            "avatar_url": row.try_get::<Option<String>, _>("avatar_url").ok().flatten()
        })
    } else {
        json!({
            "id": target_user_id,
            "full_name": format!("User #{}", target_user_id),
            "email": "",
            "avatar_url": null
        })
    };

    let has_date_filter = query.start_date.is_some() && query.end_date.is_some();
    let start_date = query.start_date.clone().unwrap_or_default();
    let end_date = query.end_date.clone().unwrap_or_default();

    // 2. Aggregate stats for this user in this project
    let (stats_query_str, time_query_str, by_type_query_str, by_priority_query_str, issues_query_str) = if has_date_filter {
        (
            format!(
                "SELECT 
                    CAST(COUNT(*) AS SIGNED) as total_assigned,
                    CAST(COUNT(CASE WHEN s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{start}' AND '{end}' THEN 1 END) AS SIGNED) as completed_count,
                    CAST(COUNT(CASE WHEN s.category = 'IN_PROGRESS' THEN 1 END) AS SIGNED) as in_progress_count,
                    CAST(COUNT(CASE WHEN s.category = 'TODO' THEN 1 END) AS SIGNED) as todo_count,
                    CAST(COALESCE(SUM(CASE WHEN s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{start}' AND '{end}' THEN i.story_points ELSE 0 END), 0) AS SIGNED) as completed_story_points,
                    CAST(COALESCE(SUM(i.story_points), 0) AS SIGNED) as total_story_points,
                    CAST(COALESCE(AVG(CASE WHEN s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{start}' AND '{end}' AND i.resolved_at IS NOT NULL THEN TIMESTAMPDIFF(HOUR, i.created_at, i.resolved_at) END), 0) AS DOUBLE) as avg_cycle_hours
                 FROM issues i
                 JOIN statuses s ON i.status_id = s.id
                 WHERE i.project_id = ? AND i.assignee_id = ?",
                start = start_date,
                end = end_date
            ),
            format!(
                "SELECT 
                    CAST(COALESCE(SUM(t.time_spent_seconds), 0) AS SIGNED) as total_logged_seconds,
                    CAST(COUNT(t.id) AS SIGNED) as total_worklogs
                 FROM time_logs t
                 JOIN issues i ON t.issue_id = i.id AND i.project_id = ?
                 WHERE t.user_id = ? AND DATE(t.logged_at) BETWEEN '{start}' AND '{end}'",
                start = start_date,
                end = end_date
            ),
            format!(
                "SELECT 
                    i.issue_type,
                    CAST(COUNT(*) AS SIGNED) as total,
                    CAST(COUNT(CASE WHEN s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{start}' AND '{end}' THEN 1 END) AS SIGNED) as completed,
                    CAST(COALESCE(SUM(i.story_points), 0) AS SIGNED) as total_points,
                    CAST(COALESCE(SUM(CASE WHEN s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{start}' AND '{end}' THEN i.story_points ELSE 0 END), 0) AS SIGNED) as completed_points
                 FROM issues i
                 JOIN statuses s ON i.status_id = s.id
                 WHERE i.project_id = ? AND i.assignee_id = ?
                 GROUP BY i.issue_type
                 ORDER BY total DESC",
                start = start_date,
                end = end_date
            ),
            format!(
                "SELECT 
                    COALESCE(i.priority, 'MEDIUM') as priority,
                    CAST(COUNT(*) AS SIGNED) as total,
                    CAST(COUNT(CASE WHEN s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{start}' AND '{end}' THEN 1 END) AS SIGNED) as completed
                 FROM issues i
                 JOIN statuses s ON i.status_id = s.id
                 WHERE i.project_id = ? AND i.assignee_id = ?
                 GROUP BY priority
                 ORDER BY total DESC",
                start = start_date,
                end = end_date
            ),
            format!(
                "SELECT 
                    i.id, i.project_id, i.`key`, i.summary, i.issue_type, i.priority, i.status_id,
                    i.story_points, i.resolution, i.resolved_at, i.created_at, i.updated_at,
                    s.name AS status_name, s.category AS status_category, s.color AS status_color,
                    comp.name AS component_name,
                    ver.name AS fix_version_name,
                    sp.name AS sprint_name,
                    (SELECT GROUP_CONCAT(lbl.name SEPARATOR ',') FROM issue_labels il JOIN labels lbl ON il.label_id = lbl.id WHERE il.issue_id = i.id) AS labels,
                    CAST(COALESCE((SELECT SUM(time_spent_seconds) FROM time_logs tl WHERE tl.issue_id = i.id AND tl.user_id = ? AND DATE(tl.logged_at) BETWEEN '{start}' AND '{end}'), 0) AS SIGNED) AS user_logged_seconds
                 FROM issues i
                 JOIN statuses s ON i.status_id = s.id
                 LEFT JOIN components comp ON i.component_id = comp.id
                 LEFT JOIN versions ver ON i.fix_version_id = ver.id
                 LEFT JOIN sprints sp ON i.sprint_id = sp.id
                 WHERE i.project_id = ? AND (
                     (i.assignee_id = ? AND (
                         (s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{start}' AND '{end}')
                         OR s.category != 'DONE'
                     ))
                     OR EXISTS (SELECT 1 FROM time_logs tl WHERE tl.issue_id = i.id AND tl.user_id = ? AND DATE(tl.logged_at) BETWEEN '{start}' AND '{end}')
                 )
                 ORDER BY COALESCE(i.resolved_at, i.updated_at) DESC
                 LIMIT 200",
                start = start_date,
                end = end_date
            )
        )
    } else {
        (
            "SELECT 
                CAST(COUNT(*) AS SIGNED) as total_assigned,
                CAST(COUNT(CASE WHEN s.category = 'DONE' THEN 1 END) AS SIGNED) as completed_count,
                CAST(COUNT(CASE WHEN s.category = 'IN_PROGRESS' THEN 1 END) AS SIGNED) as in_progress_count,
                CAST(COUNT(CASE WHEN s.category = 'TODO' THEN 1 END) AS SIGNED) as todo_count,
                CAST(COALESCE(SUM(CASE WHEN s.category = 'DONE' THEN i.story_points ELSE 0 END), 0) AS SIGNED) as completed_story_points,
                CAST(COALESCE(SUM(i.story_points), 0) AS SIGNED) as total_story_points,
                CAST(COALESCE(AVG(CASE WHEN s.category = 'DONE' AND i.resolved_at IS NOT NULL THEN TIMESTAMPDIFF(HOUR, i.created_at, i.resolved_at) END), 0) AS DOUBLE) as avg_cycle_hours
             FROM issues i
             JOIN statuses s ON i.status_id = s.id
             WHERE i.project_id = ? AND i.assignee_id = ?".to_string(),
            "SELECT 
                CAST(COALESCE(SUM(t.time_spent_seconds), 0) AS SIGNED) as total_logged_seconds,
                CAST(COUNT(t.id) AS SIGNED) as total_worklogs
             FROM time_logs t
             JOIN issues i ON t.issue_id = i.id AND i.project_id = ?
             WHERE t.user_id = ?".to_string(),
            "SELECT 
                i.issue_type,
                CAST(COUNT(*) AS SIGNED) as total,
                CAST(COUNT(CASE WHEN s.category = 'DONE' THEN 1 END) AS SIGNED) as completed,
                CAST(COALESCE(SUM(i.story_points), 0) AS SIGNED) as total_points,
                CAST(COALESCE(SUM(CASE WHEN s.category = 'DONE' THEN i.story_points ELSE 0 END), 0) AS SIGNED) as completed_points
             FROM issues i
             JOIN statuses s ON i.status_id = s.id
             WHERE i.project_id = ? AND i.assignee_id = ?
             GROUP BY i.issue_type
             ORDER BY total DESC".to_string(),
            "SELECT 
                COALESCE(i.priority, 'MEDIUM') as priority,
                CAST(COUNT(*) AS SIGNED) as total,
                CAST(COUNT(CASE WHEN s.category = 'DONE' THEN 1 END) AS SIGNED) as completed
             FROM issues i
             JOIN statuses s ON i.status_id = s.id
             WHERE i.project_id = ? AND i.assignee_id = ?
             GROUP BY priority
             ORDER BY total DESC".to_string(),
            "SELECT 
                i.id, i.project_id, i.`key`, i.summary, i.issue_type, i.priority, i.status_id,
                i.story_points, i.resolution, i.resolved_at, i.created_at, i.updated_at,
                s.name AS status_name, s.category AS status_category, s.color AS status_color,
                comp.name AS component_name,
                ver.name AS fix_version_name,
                sp.name AS sprint_name,
                (SELECT GROUP_CONCAT(lbl.name SEPARATOR ',') FROM issue_labels il JOIN labels lbl ON il.label_id = lbl.id WHERE il.issue_id = i.id) AS labels,
                CAST(COALESCE((SELECT SUM(time_spent_seconds) FROM time_logs tl WHERE tl.issue_id = i.id AND tl.user_id = ?), 0) AS SIGNED) AS user_logged_seconds
             FROM issues i
             JOIN statuses s ON i.status_id = s.id
             LEFT JOIN components comp ON i.component_id = comp.id
             LEFT JOIN versions ver ON i.fix_version_id = ver.id
             LEFT JOIN sprints sp ON i.sprint_id = sp.id
             WHERE i.project_id = ? AND (
                 i.assignee_id = ? 
                 OR EXISTS (SELECT 1 FROM time_logs tl WHERE tl.issue_id = i.id AND tl.user_id = ?)
             )
             ORDER BY COALESCE(i.resolved_at, i.updated_at) DESC
             LIMIT 200".to_string()
        )
    };

    // Execute stats query
    let stats_row = sqlx::query(&stats_query_str)
        .bind(pid)
        .bind(target_user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let total_assigned: i64 = stats_row.try_get("total_assigned").unwrap_or(0);
    let completed_count: i64 = stats_row.try_get("completed_count").unwrap_or(0);
    let in_progress_count: i64 = stats_row.try_get("in_progress_count").unwrap_or(0);
    let todo_count: i64 = stats_row.try_get("todo_count").unwrap_or(0);
    let completed_story_points: i64 = stats_row.try_get("completed_story_points").unwrap_or(0);
    let total_story_points: i64 = stats_row.try_get("total_story_points").unwrap_or(0);
    let avg_cycle_hours: f64 = stats_row.try_get("avg_cycle_hours").unwrap_or(0.0);

    let completion_rate = if total_assigned > 0 {
        ((completed_count as f64) / (total_assigned as f64)) * 100.0
    } else {
        0.0
    };

    let points_completion_rate = if total_story_points > 0 {
        ((completed_story_points as f64) / (total_story_points as f64)) * 100.0
    } else {
        0.0
    };

    // Execute time logs query
    let time_row = sqlx::query(&time_query_str)
        .bind(pid)
        .bind(target_user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let total_logged_seconds: i64 = time_row.try_get("total_logged_seconds").unwrap_or(0);
    let total_worklogs: i64 = time_row.try_get("total_worklogs").unwrap_or(0);
    let total_logged_hours = (total_logged_seconds as f64) / 3600.0;

    // Execute by type query
    let type_rows = sqlx::query(&by_type_query_str)
        .bind(pid)
        .bind(target_user_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut by_type = Vec::new();
    for row in type_rows {
        let itype: String = row.try_get("issue_type").unwrap_or_default();
        let total: i64 = row.try_get("total").unwrap_or(0);
        let completed: i64 = row.try_get("completed").unwrap_or(0);
        let tot_pts: i64 = row.try_get("total_points").unwrap_or(0);
        let comp_pts: i64 = row.try_get("completed_points").unwrap_or(0);
        by_type.push(json!({
            "issue_type": itype,
            "total": total,
            "completed": completed,
            "total_points": tot_pts,
            "completed_points": comp_pts
        }));
    }

    // Execute by priority query
    let prio_rows = sqlx::query(&by_priority_query_str)
        .bind(pid)
        .bind(target_user_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut by_priority = Vec::new();
    for row in prio_rows {
        let prio: String = row.try_get("priority").unwrap_or_default();
        let total: i64 = row.try_get("total").unwrap_or(0);
        let completed: i64 = row.try_get("completed").unwrap_or(0);
        by_priority.push(json!({
            "priority": prio,
            "total": total,
            "completed": completed
        }));
    }

    // Execute issues query
    let issue_rows = sqlx::query(&issues_query_str)
        .bind(target_user_id)
        .bind(pid)
        .bind(target_user_id)
        .bind(target_user_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut issues = Vec::new();
    for row in issue_rows {
        let id: i64 = row.try_get("id").unwrap_or(0);
        let key: String = row.try_get("key").unwrap_or_default();
        let summary: String = row.try_get("summary").unwrap_or_default();
        let itype: String = row.try_get("issue_type").unwrap_or_default();
        let prio: Option<String> = row.try_get("priority").ok();
        let status_id: i64 = row.try_get("status_id").unwrap_or(0);
        let status_name: Option<String> = row.try_get("status_name").ok();
        let status_cat: Option<String> = row.try_get("status_category").ok();
        let status_color: Option<String> = row.try_get("status_color").ok();
        let resolution: Option<String> = row.try_get("resolution").ok();
        let story_points: Option<i32> = row.try_get("story_points").ok();
        let comp_name: Option<String> = row.try_get("component_name").ok();
        let ver_name: Option<String> = row.try_get("fix_version_name").ok();
        let sp_name: Option<String> = row.try_get("sprint_name").ok();
        let labels_str: Option<String> = row.try_get("labels").ok();
        let user_secs: i64 = row.try_get("user_logged_seconds").unwrap_or(0);
        let resolved_at: Option<chrono::DateTime<chrono::Utc>> = row.try_get("resolved_at").ok();
        let created_at: Option<chrono::DateTime<chrono::Utc>> = row.try_get("created_at").ok();
        let updated_at: Option<chrono::DateTime<chrono::Utc>> = row.try_get("updated_at").ok();

        let labels: Vec<String> = labels_str
            .map(|s| s.split(',').filter(|x| !x.is_empty()).map(|x| x.to_string()).collect())
            .unwrap_or_default();

        issues.push(json!({
            "id": id,
            "key": key,
            "summary": summary,
            "issue_type": itype,
            "priority": prio.unwrap_or_else(|| "MEDIUM".to_string()),
            "status_id": status_id,
            "status_name": status_name.unwrap_or_else(|| "Unknown".to_string()),
            "status_category": status_cat.unwrap_or_else(|| "TODO".to_string()),
            "status_color": status_color.unwrap_or_else(|| "#64748b".to_string()),
            "resolution": resolution,
            "story_points": story_points,
            "component_name": comp_name,
            "fix_version_name": ver_name,
            "sprint_name": sp_name,
            "labels": labels,
            "user_logged_seconds": user_secs,
            "user_logged_hours": (user_secs as f64) / 3600.0,
            "resolved_at": resolved_at,
            "created_at": created_at,
            "updated_at": updated_at
        }));
    }

    Ok(Json(json!({
        "project_id": pid,
        "selected_user": selected_user,
        "available_users": available_users,
        "date_filter": {
            "start_date": if has_date_filter { Some(start_date) } else { None },
            "end_date": if has_date_filter { Some(end_date) } else { None },
            "is_filtered": has_date_filter
        },
        "stats": {
            "total_assigned": total_assigned,
            "completed_count": completed_count,
            "in_progress_count": in_progress_count,
            "todo_count": todo_count,
            "completion_rate": completion_rate,
            "completed_story_points": completed_story_points,
            "total_story_points": total_story_points,
            "points_completion_rate": points_completion_rate,
            "total_logged_seconds": total_logged_seconds,
            "total_logged_hours": total_logged_hours,
            "total_worklogs": total_worklogs,
            "avg_cycle_hours": avg_cycle_hours,
            "avg_cycle_days": avg_cycle_hours / 24.0
        },
        "by_type": by_type,
        "by_priority": by_priority,
        "issues": issues
    })))
}
