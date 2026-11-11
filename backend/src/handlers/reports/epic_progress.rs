use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::Row;

use crate::{auth::AuthUser, AppState};

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

