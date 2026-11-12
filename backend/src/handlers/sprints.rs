use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    models::{
        CompleteSprintPayload, CreateSprintPayload, MoveSprintIssuesPayload,
        Sprint, StartSprintPayload, UpdateSprintPayload,
    },
    ws::WsBroadcastMessage,
    AppState,
};

pub async fn list_project_sprints(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sql = "SELECT sp.id, sp.project_id, sp.name, sp.goal, sp.status, \
                      sp.start_date, sp.end_date, sp.created_at, sp.updated_at, \
                      COUNT(i.id) AS total_issues, \
                      COALESCE(SUM(CASE WHEN st.category = 'TODO' THEN COALESCE(i.story_points, 0) ELSE 0 END), 0) AS todo_points, \
                      COALESCE(SUM(CASE WHEN st.category = 'IN_PROGRESS' THEN COALESCE(i.story_points, 0) ELSE 0 END), 0) AS in_progress_points, \
                      COALESCE(SUM(CASE WHEN st.category = 'DONE' THEN COALESCE(i.story_points, 0) ELSE 0 END), 0) AS done_points \
               FROM sprints sp \
               LEFT JOIN issues i ON i.sprint_id = sp.id \
               LEFT JOIN statuses st ON i.status_id = st.id \
               WHERE sp.project_id = ? \
               GROUP BY sp.id \
               ORDER BY \
                 CASE sp.status \
                   WHEN 'ACTIVE' THEN 1 \
                   WHEN 'PLANNED' THEN 2 \
                   WHEN 'COMPLETED' THEN 3 \
                   ELSE 4 \
                 END, \
                 sp.id DESC";

    let sprints = sqlx::query_as::<_, Sprint>(sql)
        .bind(project_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to list sprints: {}", e) })),
            )
        })?;

    Ok(Json(sprints))
}

pub async fn create_sprint(
    auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateSprintPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if payload.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Sprint name is required" })),
        ));
    }

    let res = sqlx::query(
        "INSERT INTO sprints (project_id, name, goal, status, start_date, end_date) \
         VALUES (?, ?, ?, 'PLANNED', ?, ?)"
    )
    .bind(project_id)
    .bind(payload.name.trim())
    .bind(payload.goal)
    .bind(payload.start_date)
    .bind(payload.end_date)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to create sprint: {}", e) })),
        )
    })?;

    let sprint_id = res.last_insert_id() as i64;

    let sprint: Sprint = sqlx::query_as(
        "SELECT id, project_id, name, goal, status, start_date, end_date, created_at, updated_at, \
                0 AS total_issues, 0 AS todo_points, 0 AS in_progress_points, 0 AS done_points \
         FROM sprints WHERE id = ?"
    )
    .bind(sprint_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to fetch created sprint: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "SPRINT_CREATED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: serde_json::to_value(&sprint).unwrap_or_default(),
    });

    Ok((StatusCode::CREATED, Json(sprint)))
}

pub async fn update_sprint(
    auth: AuthUser,
    Path(sprint_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateSprintPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current: (i64, String, Option<String>, Option<chrono::NaiveDate>, Option<chrono::NaiveDate>) = sqlx::query_as(
        "SELECT project_id, name, goal, start_date, end_date FROM sprints WHERE id = ?"
    )
    .bind(sprint_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Sprint not found" })),
        )
    })?;

    let project_id = current.0;
    let name = payload.name.unwrap_or(current.1);
    let goal = payload.goal.or(current.2);
    let start_date = payload.start_date.or(current.3);
    let end_date = payload.end_date.or(current.4);

    sqlx::query(
        "UPDATE sprints SET name = ?, goal = ?, start_date = ?, end_date = ? WHERE id = ?"
    )
    .bind(name)
    .bind(goal)
    .bind(start_date)
    .bind(end_date)
    .bind(sprint_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to update sprint: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "SPRINT_UPDATED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({ "id": sprint_id, "project_id": project_id }),
    });

    Ok(Json(json!({ "status": "success", "id": sprint_id })))
}

pub async fn start_sprint(
    auth: AuthUser,
    Path(sprint_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<StartSprintPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current: (i64, String, String) = sqlx::query_as(
        "SELECT project_id, name, status FROM sprints WHERE id = ?"
    )
    .bind(sprint_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Sprint not found" })),
        )
    })?;

    let project_id = current.0;
    let sprint_name = current.1;
    let status = current.2;

    if status == "COMPLETED" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Cannot restart a completed sprint" })),
        ));
    }

    // Check if another active sprint exists in this project
    let active_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sprints WHERE project_id = ? AND status = 'ACTIVE' AND id != ?"
    )
    .bind(project_id)
    .bind(sprint_id)
    .fetch_one(&state.db)
    .await
    .unwrap_or((0,));

    if active_count.0 > 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Another sprint is currently active. Please complete it before starting a new one." })),
        ));
    }

    let name = payload.name.unwrap_or(sprint_name);
    let goal = payload.goal;
    let start_date = payload.start_date.unwrap_or_else(|| chrono::Utc::now().date_naive());
    let end_date = payload.end_date.unwrap_or_else(|| chrono::Utc::now().date_naive() + chrono::Duration::days(14));

    sqlx::query(
        "UPDATE sprints SET name = ?, goal = COALESCE(?, goal), status = 'ACTIVE', start_date = ?, end_date = ? WHERE id = ?"
    )
    .bind(name)
    .bind(goal)
    .bind(start_date)
    .bind(end_date)
    .bind(sprint_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to start sprint: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "SPRINT_STARTED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({ "id": sprint_id, "project_id": project_id, "status": "ACTIVE" }),
    });

    Ok(Json(json!({ "status": "success", "id": sprint_id, "sprint_status": "ACTIVE" })))
}

pub async fn complete_sprint(
    auth: AuthUser,
    Path(sprint_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CompleteSprintPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current: (i64, String) = sqlx::query_as(
        "SELECT project_id, status FROM sprints WHERE id = ?"
    )
    .bind(sprint_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Sprint not found" })),
        )
    })?;

    let project_id = current.0;

    // Begin transaction
    let mut tx = state.db.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Transaction start failed: {}", e) })),
        )
    })?;

    // Mark current sprint as COMPLETED
    sqlx::query("UPDATE sprints SET status = 'COMPLETED' WHERE id = ?")
        .bind(sprint_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to complete sprint: {}", e) })),
            )
        })?;

    // Move incomplete issues (issues whose status category != 'DONE')
    // Either to the designated sprint or to Backlog (sprint_id = NULL)
    if let Some(target_sprint_id) = payload.move_incomplete_to_sprint_id {
        sqlx::query(
            "UPDATE issues i \
             JOIN statuses s ON i.status_id = s.id \
             SET i.sprint_id = ? \
             WHERE i.sprint_id = ? AND s.category != 'DONE'"
        )
        .bind(target_sprint_id)
        .bind(sprint_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to move incomplete issues: {}", e) })),
            )
        })?;
    } else {
        // Move to product backlog (NULL)
        sqlx::query(
            "UPDATE issues i \
             JOIN statuses s ON i.status_id = s.id \
             SET i.sprint_id = NULL \
             WHERE i.sprint_id = ? AND s.category != 'DONE'"
        )
        .bind(sprint_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to move incomplete issues to backlog: {}", e) })),
            )
        })?;
    }

    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Commit failed: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "SPRINT_COMPLETED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({
            "id": sprint_id,
            "project_id": project_id,
            "move_incomplete_to": payload.move_incomplete_to_sprint_id
        }),
    });

    Ok(Json(json!({ "status": "success", "id": sprint_id, "sprint_status": "COMPLETED" })))
}

pub async fn delete_sprint(
    auth: AuthUser,
    Path(sprint_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current: (i64,) = sqlx::query_as("SELECT project_id FROM sprints WHERE id = ?")
        .bind(sprint_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Sprint not found" })),
            )
        })?;

    let project_id = current.0;

    // Reset sprint_id to NULL on any issues belonging to this sprint so they return to backlog safely
    let _ = sqlx::query("UPDATE issues SET sprint_id = NULL WHERE sprint_id = ?")
        .bind(sprint_id)
        .execute(&state.db)
        .await;

    sqlx::query("DELETE FROM sprints WHERE id = ?")
        .bind(sprint_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to delete sprint: {}", e) })),
            )
        })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "SPRINT_DELETED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({ "id": sprint_id, "project_id": project_id }),
    });

    Ok(Json(json!({ "status": "success", "id": sprint_id })))
}

pub async fn move_issues_to_sprint(
    auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<MoveSprintIssuesPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if payload.issue_ids.is_empty() {
        return Ok(Json(json!({ "status": "success", "updated_count": 0 })));
    }

    let mut query = String::from("UPDATE issues SET sprint_id = ? WHERE project_id = ? AND id IN (");
    for (i, id) in payload.issue_ids.iter().enumerate() {
        if i > 0 {
            query.push_str(", ");
        }
        query.push_str(&id.to_string());
    }
    query.push(')');

    sqlx::query(&query)
        .bind(payload.target_sprint_id)
        .bind(project_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to move issues: {}", e) })),
            )
        })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "SPRINT_ISSUES_MOVED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({
            "project_id": project_id,
            "target_sprint_id": payload.target_sprint_id,
            "issue_ids": payload.issue_ids
        }),
    });

    Ok(Json(json!({
        "status": "success",
        "target_sprint_id": payload.target_sprint_id,
        "count": payload.issue_ids.len()
    })))
}

