use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    models::{CreateVersionPayload, ReleaseVersionPayload, UpdateVersionPayload, Version},
    ws::WsBroadcastMessage,
    AppState,
};

pub async fn list_project_versions(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sql = "SELECT v.id, v.project_id, v.name, v.description, v.start_date, v.release_date, \
                      v.status, v.created_at, v.updated_at, \
                      COUNT(i.id) AS total_issues, \
                      COALESCE(SUM(CASE WHEN st.category = 'TODO' THEN 1 ELSE 0 END), 0) AS todo_issues, \
                      COALESCE(SUM(CASE WHEN st.category = 'IN_PROGRESS' THEN 1 ELSE 0 END), 0) AS in_progress_issues, \
                      COALESCE(SUM(CASE WHEN st.category = 'DONE' THEN 1 ELSE 0 END), 0) AS done_issues \
               FROM versions v \
               LEFT JOIN issues i ON i.fix_version_id = v.id \
               LEFT JOIN statuses st ON i.status_id = st.id \
               WHERE v.project_id = ? \
               GROUP BY v.id \
               ORDER BY \
                 CASE v.status \
                   WHEN 'UNRELEASED' THEN 1 \
                   WHEN 'RELEASED' THEN 2 \
                   WHEN 'ARCHIVED' THEN 3 \
                   ELSE 4 \
                 END, \
                 v.release_date ASC, \
                 v.id DESC";

    let versions = sqlx::query_as::<_, Version>(sql)
        .bind(project_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to list versions: {}", e) })),
            )
        })?;

    Ok(Json(versions))
}

pub async fn create_version(
    auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateVersionPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if payload.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Version name is required" })),
        ));
    }

    let res = sqlx::query(
        "INSERT INTO versions (project_id, name, description, start_date, release_date, status) \
         VALUES (?, ?, ?, ?, ?, 'UNRELEASED')"
    )
    .bind(project_id)
    .bind(payload.name.trim())
    .bind(payload.description)
    .bind(payload.start_date)
    .bind(payload.release_date)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to create version: {}", e) })),
        )
    })?;

    let version_id = res.last_insert_id() as i64;

    let ver: Version = sqlx::query_as(
        "SELECT id, project_id, name, description, start_date, release_date, status, created_at, updated_at, \
                0 AS total_issues, 0 AS todo_issues, 0 AS in_progress_issues, 0 AS done_issues \
         FROM versions WHERE id = ?"
    )
    .bind(version_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to fetch created version: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "VERSION_CREATED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: serde_json::to_value(&ver).unwrap_or_default(),
    });

    Ok((StatusCode::CREATED, Json(ver)))
}

pub async fn update_version(
    auth: AuthUser,
    Path(version_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateVersionPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current: (i64, String, Option<String>, Option<chrono::NaiveDate>, Option<chrono::NaiveDate>, String) = sqlx::query_as(
        "SELECT project_id, name, description, start_date, release_date, status FROM versions WHERE id = ?"
    )
    .bind(version_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Version not found" })),
        )
    })?;

    let project_id = current.0;
    let name = payload.name.unwrap_or(current.1);
    let description = payload.description.or(current.2);
    let start_date = payload.start_date.or(current.3);
    let release_date = payload.release_date.or(current.4);
    let status = payload.status.unwrap_or(current.5);

    sqlx::query(
        "UPDATE versions SET name = ?, description = ?, start_date = ?, release_date = ?, status = ? WHERE id = ?"
    )
    .bind(name)
    .bind(description)
    .bind(start_date)
    .bind(release_date)
    .bind(status)
    .bind(version_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to update version: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "VERSION_UPDATED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({ "id": version_id, "project_id": project_id }),
    });

    Ok(Json(json!({ "status": "success", "id": version_id })))
}

pub async fn release_version(
    auth: AuthUser,
    Path(version_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<ReleaseVersionPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current: (i64,) = sqlx::query_as(
        "SELECT project_id FROM versions WHERE id = ?"
    )
    .bind(version_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Version not found" })),
        )
    })?;

    let project_id = current.0;
    let release_date = payload.release_date.unwrap_or_else(|| chrono::Utc::now().date_naive());

    let mut tx = state.db.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Transaction start failed: {}", e) })),
        )
    })?;

    // Update version status to RELEASED
    sqlx::query("UPDATE versions SET status = 'RELEASED', release_date = ? WHERE id = ?")
        .bind(release_date)
        .bind(version_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to release version: {}", e) })),
            )
        })?;

    // If target version supplied for unreleased issues, move them
    if let Some(target_ver_id) = payload.move_unreleased_to_version_id {
        sqlx::query(
            "UPDATE issues i \
             JOIN statuses s ON i.status_id = s.id \
             SET i.fix_version_id = ? \
             WHERE i.fix_version_id = ? AND s.category != 'DONE'"
        )
        .bind(target_ver_id)
        .bind(version_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to move unreleased issues: {}", e) })),
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
        event: "VERSION_RELEASED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({ "id": version_id, "project_id": project_id, "status": "RELEASED" }),
    });

    Ok(Json(json!({ "status": "success", "id": version_id, "version_status": "RELEASED" })))
}

pub async fn archive_version(
    auth: AuthUser,
    Path(version_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current: (i64, String) = sqlx::query_as(
        "SELECT project_id, status FROM versions WHERE id = ?"
    )
    .bind(version_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Version not found" })),
        )
    })?;

    let project_id = current.0;
    let new_status = if current.1 == "ARCHIVED" { "UNRELEASED" } else { "ARCHIVED" };

    sqlx::query("UPDATE versions SET status = ? WHERE id = ?")
        .bind(new_status)
        .bind(version_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to toggle archive version: {}", e) })),
            )
        })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "VERSION_UPDATED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({ "id": version_id, "project_id": project_id, "status": new_status }),
    });

    Ok(Json(json!({ "status": "success", "id": version_id, "version_status": new_status })))
}

pub async fn delete_version(
    auth: AuthUser,
    Path(version_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current: (i64,) = sqlx::query_as("SELECT project_id FROM versions WHERE id = ?")
        .bind(version_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Version not found" })),
            )
        })?;

    let project_id = current.0;

    // Reset fix_version_id to NULL on any issues using this version
    let _ = sqlx::query("UPDATE issues SET fix_version_id = NULL WHERE fix_version_id = ?")
        .bind(version_id)
        .execute(&state.db)
        .await;

    sqlx::query("DELETE FROM versions WHERE id = ?")
        .bind(version_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to delete version: {}", e) })),
            )
        })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "VERSION_DELETED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({ "id": version_id, "project_id": project_id }),
    });

    Ok(Json(json!({ "status": "success", "id": version_id })))
}
