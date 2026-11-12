use axum::{http::StatusCode, Json};
use serde_json::json;

use crate::models::Issue;

pub async fn fetch_single_issue(
    pool: &sqlx::MySqlPool,
    issue_id: i64,
) -> Result<Issue, sqlx::Error> {
    sqlx::query_as::<_, Issue>(
        "SELECT i.id, i.project_id, i.parent_id, i.epic_id, i.`key`, i.summary, i.description, \
                i.issue_type, i.status_id, i.priority, i.assignee_id, i.reporter_id, i.story_points, \
                i.start_date, i.due_date, i.position, i.sprint_id, i.component_id, i.created_at, i.updated_at, \
                s.name AS status_name, s.category AS status_category, s.color AS status_color, \
                ua.full_name AS assignee_name, ua.avatar_url AS assignee_avatar, \
                ur.full_name AS reporter_name, \
                ep.summary AS epic_summary, \
                p.`key` AS parent_key, p.summary AS parent_summary, \
                sp.name AS sprint_name, \
                comp.name AS component_name, \
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
         WHERE i.id = ?"
    )
    .bind(issue_id)
    .fetch_one(pool)
    .await
}

pub async fn validate_transition(
    pool: &sqlx::MySqlPool,
    project_id: i64,
    from_status_id: i64,
    to_status_id: i64,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let rules_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM workflow_transitions WHERE project_id = ? AND from_status_id = ?"
    )
    .bind(project_id)
    .bind(from_status_id)
    .fetch_one(pool)
    .await
    .unwrap_or((0,));

    if rules_count.0 > 0 {
        let allowed: Result<(i64,), _> = sqlx::query_as(
            "SELECT id FROM workflow_transitions WHERE project_id = ? AND from_status_id = ? AND to_status_id = ?"
        )
        .bind(project_id)
        .bind(from_status_id)
        .bind(to_status_id)
        .fetch_one(pool)
        .await;

        if allowed.is_err() {
            let from_name: (String,) = sqlx::query_as("SELECT name FROM statuses WHERE id = ?")
                .bind(from_status_id)
                .fetch_one(pool)
                .await
                .unwrap_or(("Origin Status".to_string(),));
            let to_name: (String,) = sqlx::query_as("SELECT name FROM statuses WHERE id = ?")
                .bind(to_status_id)
                .fetch_one(pool)
                .await
                .unwrap_or(("Destination Status".to_string(),));

            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": format!("Workflow rule violation: Transition directly from '{}' to '{}' is not allowed.", from_name.0, to_name.0)
                })),
            ));
        }
    }

    Ok(())
}

