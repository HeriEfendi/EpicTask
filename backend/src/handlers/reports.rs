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

/// GET /api/projects/:pid/reports/epic-progress
pub async fn get_epic_progress(
    _auth: AuthUser,
    Path(pid): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows = sqlx::query(
        "SELECT 
            e.id,
            e.`key`,
            e.summary,
            e.priority,
            e.start_date,
            e.due_date,
            s.name as status_name,
            s.color as status_color,
            s.category as status_category,
            e.story_points as epic_points,
            CAST(COUNT(child.id) AS SIGNED) as total_children,
            CAST(SUM(CASE WHEN child_status.category = 'DONE' THEN 1 ELSE 0 END) AS SIGNED) as done_children,
            CAST(COALESCE(SUM(child.story_points), 0) AS SIGNED) as child_points_total,
            CAST(COALESCE(SUM(CASE WHEN child_status.category = 'DONE' THEN child.story_points ELSE 0 END), 0) AS SIGNED) as child_points_done
         FROM issues e
         JOIN statuses s ON e.status_id = s.id
         LEFT JOIN issues child ON child.epic_id = e.id AND child.project_id = e.project_id
         LEFT JOIN statuses child_status ON child.status_id = child_status.id
         WHERE e.project_id = ? AND e.issue_type = 'EPIC'
         GROUP BY e.id, e.`key`, e.summary, e.priority, e.start_date, e.due_date, s.name, s.color, s.category, e.story_points
         ORDER BY e.id ASC"
    )
    .bind(pid)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let mut epics = Vec::new();
    for row in rows {
        let id: i64 = row.try_get("id").unwrap_or(0);
        let key: String = row.try_get("key").unwrap_or_default();
        let summary: String = row.try_get("summary").unwrap_or_default();
        let prio: String = row.try_get("priority").unwrap_or_default();
        let start_date: Option<chrono::NaiveDate> = row.try_get("start_date").ok();
        let due_date: Option<chrono::NaiveDate> = row.try_get("due_date").ok();
        let status_name: String = row.try_get("status_name").unwrap_or_default();
        let status_color: Option<String> = row.try_get("status_color").ok();
        let status_category: String = row.try_get("status_category").unwrap_or_default();
        let epic_pts: i64 = row.try_get("epic_points").unwrap_or(0);
        let total_children: i64 = row.try_get("total_children").unwrap_or(0);
        let done_children: i64 = row.try_get("done_children").unwrap_or(0);
        let child_pts_total: i64 = row.try_get("child_points_total").unwrap_or(0);
        let child_pts_done: i64 = row.try_get("child_points_done").unwrap_or(0);

        let progress_pct = if total_children > 0 {
            ((done_children as f64) / (total_children as f64) * 100.0).round()
        } else if status_category == "DONE" {
            100.0
        } else {
            0.0
        };

        epics.push(json!({
            "id": id,
            "key": key,
            "summary": summary,
            "priority": prio,
            "start_date": start_date,
            "due_date": due_date,
            "status_name": status_name,
            "status_color": status_color.unwrap_or_else(|| "#8b5cf6".into()),
            "status_category": status_category,
            "epic_points": epic_pts,
            "total_children": total_children,
            "done_children": done_children,
            "child_points_total": child_pts_total,
            "child_points_done": child_pts_done,
            "progress_pct": progress_pct
        }));
    }

    Ok(Json(json!({
        "project_id": pid,
        "epics": epics
    })))
}
