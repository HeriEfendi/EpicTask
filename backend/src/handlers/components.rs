use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    models::{Component, CreateComponentPayload, UpdateComponentPayload},
    ws::WsBroadcastMessage,
    AppState,
};

pub async fn list_project_components(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sql = "SELECT c.id, c.project_id, c.name, c.description, c.lead_user_id, c.created_at, \
                      u.full_name AS lead_name, \
                      COUNT(i.id) AS issue_count \
               FROM components c \
               LEFT JOIN users u ON c.lead_user_id = u.id \
               LEFT JOIN issues i ON i.component_id = c.id \
               WHERE c.project_id = ? \
               GROUP BY c.id \
               ORDER BY c.name ASC";

    let components = sqlx::query_as::<_, Component>(sql)
        .bind(project_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to list components: {}", e) })),
            )
        })?;

    Ok(Json(components))
}

pub async fn create_component(
    auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateComponentPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if payload.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Component name is required" })),
        ));
    }

    let res = sqlx::query(
        "INSERT INTO components (project_id, name, description, lead_user_id) \
         VALUES (?, ?, ?, ?)"
    )
    .bind(project_id)
    .bind(payload.name.trim())
    .bind(payload.description)
    .bind(payload.lead_user_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to create component: {}", e) })),
        )
    })?;

    let component_id = res.last_insert_id() as i64;

    let comp: Component = sqlx::query_as(
        "SELECT c.id, c.project_id, c.name, c.description, c.lead_user_id, c.created_at, \
                u.full_name AS lead_name, 0 AS issue_count \
         FROM components c \
         LEFT JOIN users u ON c.lead_user_id = u.id \
         WHERE c.id = ?"
    )
    .bind(component_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to fetch created component: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "COMPONENT_CREATED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: serde_json::to_value(&comp).unwrap_or_default(),
    });

    Ok((StatusCode::CREATED, Json(comp)))
}

pub async fn update_component(
    auth: AuthUser,
    Path(component_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateComponentPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current: (i64, String, Option<String>, Option<i64>) = sqlx::query_as(
        "SELECT project_id, name, description, lead_user_id FROM components WHERE id = ?"
    )
    .bind(component_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Component not found" })),
        )
    })?;

    let project_id = current.0;
    let name = payload.name.unwrap_or(current.1);
    let description = payload.description.or(current.2);
    let lead_user_id = payload.lead_user_id.or(current.3);

    sqlx::query(
        "UPDATE components SET name = ?, description = ?, lead_user_id = ? WHERE id = ?"
    )
    .bind(name)
    .bind(description)
    .bind(lead_user_id)
    .bind(component_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to update component: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "COMPONENT_UPDATED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({ "id": component_id, "project_id": project_id }),
    });

    Ok(Json(json!({ "status": "success", "id": component_id })))
}

pub async fn delete_component(
    auth: AuthUser,
    Path(component_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current: (i64,) = sqlx::query_as("SELECT project_id FROM components WHERE id = ?")
        .bind(component_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Component not found" })),
            )
        })?;

    let project_id = current.0;

    // Reset component_id on any issues that use it
    let _ = sqlx::query("UPDATE issues SET component_id = NULL WHERE component_id = ?")
        .bind(component_id)
        .execute(&state.db)
        .await;

    sqlx::query("DELETE FROM components WHERE id = ?")
        .bind(component_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to delete component: {}", e) })),
            )
        })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "COMPONENT_DELETED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({ "id": component_id, "project_id": project_id }),
    });

    Ok(Json(json!({ "status": "success", "id": component_id })))
}

