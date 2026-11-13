use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use super::helpers::fetch_single_issue;
use crate::{
    auth::AuthUser,
    models::{Issue, IssueLink},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct IssueFilterParams {
    pub search: Option<String>,
    pub status_id: Option<i64>,
    pub assignee_id: Option<i64>,
    pub priority: Option<String>,
    pub issue_type: Option<String>,
    pub epic_id: Option<i64>,
    pub sprint_id: Option<i64>,
    pub is_backlog: Option<bool>,
    pub component_id: Option<i64>,
    pub fix_version_id: Option<i64>,
    pub label: Option<String>,
    pub resolution: Option<String>,
}

pub async fn list_project_issues(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    Query(params): Query<IssueFilterParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut sql = String::from(
        "SELECT i.id, i.project_id, i.parent_id, i.epic_id, i.`key`, i.summary, i.description, \
                i.issue_type, i.status_id, i.priority, i.assignee_id, i.reporter_id, i.story_points, \
                i.start_date, i.due_date, i.position, i.sprint_id, i.component_id, i.fix_version_id, \
                i.resolution, i.resolved_at, i.created_at, i.updated_at, \
                s.name AS status_name, s.category AS status_category, s.color AS status_color, \
                ua.full_name AS assignee_name, ua.avatar_url AS assignee_avatar, \
                ur.full_name AS reporter_name, \
                ep.summary AS epic_summary, \
                p.`key` AS parent_key, p.summary AS parent_summary, \
                sp.name AS sprint_name, \
                comp.name AS component_name, \
                ver.name AS fix_version_name, \
                (SELECT GROUP_CONCAT(lbl.name SEPARATOR ',') FROM issue_labels il JOIN labels lbl ON il.label_id = lbl.id WHERE il.issue_id = i.id) AS labels, \
                (SELECT COUNT(*) FROM issues sub WHERE sub.parent_id = i.id) AS subtask_count, \
                (SELECT COUNT(*) FROM issues sub JOIN statuses subs ON sub.status_id = subs.id WHERE sub.parent_id = i.id AND subs.category = 'DONE') AS subtask_done_count \
         FROM issues i \
         JOIN statuses s ON i.status_id = s.id \
         LEFT JOIN users ua ON i.assignee_id = ua.id \
         LEFT JOIN users ur ON i.reporter_id = ur.id \
         LEFT JOIN issues ep ON i.epic_id = ep.id \
         LEFT JOIN issues p ON i.parent_id = p.id \
         LEFT JOIN sprints sp ON i.sprint_id = sp.id \
         LEFT JOIN components comp ON i.component_id = comp.id \
         LEFT JOIN versions ver ON i.fix_version_id = ver.id \
         WHERE i.project_id = ?"
    );

    if let Some(ref q) = params.search {
        if !q.trim().is_empty() {
            sql.push_str(" AND (i.summary LIKE '%");
            sql.push_str(&q.replace('\'', "''"));
            sql.push_str("%' OR i.`key` LIKE '%");
            sql.push_str(&q.replace('\'', "''"));
            sql.push_str("%')");
        }
    }

    if let Some(status_id) = params.status_id {
        sql.push_str(&format!(" AND i.status_id = {}", status_id));
    }
    if let Some(assignee_id) = params.assignee_id {
        sql.push_str(&format!(" AND i.assignee_id = {}", assignee_id));
    }
    if let Some(ref priority) = params.priority {
        sql.push_str(&format!(" AND i.priority = '{}'", priority));
    }
    if let Some(ref itype) = params.issue_type {
        sql.push_str(&format!(" AND i.issue_type = '{}'", itype));
    }
    if let Some(epic_id) = params.epic_id {
        sql.push_str(&format!(" AND i.epic_id = {}", epic_id));
    }
    if let Some(sprint_id) = params.sprint_id {
        sql.push_str(&format!(" AND i.sprint_id = {}", sprint_id));
    }
    if let Some(is_backlog) = params.is_backlog {
        if is_backlog {
            sql.push_str(" AND i.sprint_id IS NULL");
        }
    }
    if let Some(component_id) = params.component_id {
        sql.push_str(&format!(" AND i.component_id = {}", component_id));
    }
    if let Some(fix_version_id) = params.fix_version_id {
        sql.push_str(&format!(" AND i.fix_version_id = {}", fix_version_id));
    }
    if let Some(ref lbl) = params.label {
        if !lbl.trim().is_empty() {
            sql.push_str(&format!(
                " AND EXISTS (SELECT 1 FROM issue_labels il JOIN labels lbl ON il.label_id = lbl.id WHERE il.issue_id = i.id AND lbl.name = '{}')",
                lbl.trim().replace('\'', "''")
            ));
        }
    }
    if let Some(ref res) = params.resolution {
        if res.eq_ignore_ascii_case("unresolved") {
            sql.push_str(" AND i.resolution IS NULL");
        } else if !res.trim().is_empty() {
            sql.push_str(&format!(" AND i.resolution = '{}'", res.trim().replace('\'', "''")));
        }
    }

    sql.push_str(" ORDER BY s.position ASC, i.position ASC, i.id DESC");

    let issues = sqlx::query_as::<_, Issue>(&sql)
        .bind(project_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {}", e) })),
            )
        })?;

    Ok(Json(issues))
}

pub async fn get_issue(
    _auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let issue = fetch_single_issue(&state.db, issue_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Issue not found" })),
        )
    })?;

    // Fetch subtasks
    let subtasks = sqlx::query_as::<_, Issue>(
        "SELECT i.id, i.project_id, i.parent_id, i.epic_id, i.`key`, i.summary, i.description, \
                i.issue_type, i.status_id, i.priority, i.assignee_id, i.reporter_id, i.story_points, \
                i.start_date, i.due_date, i.position, i.sprint_id, i.component_id, i.fix_version_id, \
                i.resolution, i.resolved_at, i.created_at, i.updated_at, \
                s.name AS status_name, s.category AS status_category, s.color AS status_color, \
                ua.full_name AS assignee_name, ua.avatar_url AS assignee_avatar, \
                ur.full_name AS reporter_name, \
                NULL AS epic_summary, NULL AS parent_key, NULL AS parent_summary, \
                NULL AS sprint_name, NULL AS component_name, NULL AS fix_version_name, \
                NULL AS labels, \
                0 AS subtask_count, 0 AS subtask_done_count \
         FROM issues i \
         JOIN statuses s ON i.status_id = s.id \
         LEFT JOIN users ua ON i.assignee_id = ua.id \
         LEFT JOIN users ur ON i.reporter_id = ur.id \
         WHERE i.parent_id = ? \
         ORDER BY s.position ASC, i.id ASC"
    )
    .bind(issue_id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    // Fetch links
    let links = sqlx::query_as::<_, IssueLink>(
        "SELECT il.id, il.source_issue_id, il.target_issue_id, il.link_type, il.created_at, \
                target.key AS target_key, target.summary AS target_summary, s.name AS target_status_name \
         FROM issue_links il \
         JOIN issues target ON il.target_issue_id = target.id \
         JOIN statuses s ON target.status_id = s.id \
         WHERE il.source_issue_id = ?"
    )
    .bind(issue_id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "issue": issue,
        "subtasks": subtasks,
        "links": links
    })))
}

