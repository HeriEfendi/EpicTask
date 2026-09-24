use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    models::{AutomationRule, CreateAutomationRulePayload, UpdateAutomationRulePayload},
    AppState,
};

pub async fn list_automations(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rules = sqlx::query_as::<_, AutomationRule>(
        "SELECT id, project_id, name, trigger_type, trigger_config, action_type, action_config, is_active, created_at \
         FROM automation_rules \
         WHERE project_id = ? \
         ORDER BY id DESC"
    )
    .bind(project_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok(Json(rules))
}

pub async fn create_automation(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateAutomationRulePayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if payload.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Rule name is required" })),
        ));
    }

    let is_active = payload.is_active.unwrap_or(true);

    let res = sqlx::query(
        "INSERT INTO automation_rules (project_id, name, trigger_type, trigger_config, action_type, action_config, is_active) \
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(project_id)
    .bind(&payload.name)
    .bind(&payload.trigger_type)
    .bind(&payload.trigger_config)
    .bind(&payload.action_type)
    .bind(&payload.action_config)
    .bind(is_active)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    let rule_id = res.last_insert_id() as i64;

    let rule = sqlx::query_as::<_, AutomationRule>(
        "SELECT id, project_id, name, trigger_type, trigger_config, action_type, action_config, is_active, created_at \
         FROM automation_rules WHERE id = ?"
    )
    .bind(rule_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok((StatusCode::CREATED, Json(rule)))
}

pub async fn update_automation(
    _auth: AuthUser,
    Path((_project_id, rule_id)): Path<(i64, i64)>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateAutomationRulePayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let current = sqlx::query_as::<_, AutomationRule>(
        "SELECT id, project_id, name, trigger_type, trigger_config, action_type, action_config, is_active, created_at \
         FROM automation_rules WHERE id = ?"
    )
    .bind(rule_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Automation rule not found" })),
        )
    })?;

    let name = payload.name.unwrap_or(current.name);
    let trigger_type = payload.trigger_type.unwrap_or(current.trigger_type);
    let trigger_config = payload.trigger_config.unwrap_or(current.trigger_config);
    let action_type = payload.action_type.unwrap_or(current.action_type);
    let action_config = payload.action_config.unwrap_or(current.action_config);
    let is_active = payload.is_active.or(current.is_active).unwrap_or(true);

    sqlx::query(
        "UPDATE automation_rules SET name = ?, trigger_type = ?, trigger_config = ?, action_type = ?, action_config = ?, is_active = ? \
         WHERE id = ?"
    )
    .bind(&name)
    .bind(&trigger_type)
    .bind(&trigger_config)
    .bind(&action_type)
    .bind(&action_config)
    .bind(is_active)
    .bind(rule_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Update error: {}", e) })),
        )
    })?;

    let updated = sqlx::query_as::<_, AutomationRule>(
        "SELECT id, project_id, name, trigger_type, trigger_config, action_type, action_config, is_active, created_at \
         FROM automation_rules WHERE id = ?"
    )
    .bind(rule_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Fetch error: {}", e) })),
        )
    })?;

    Ok(Json(updated))
}

pub async fn delete_automation(
    _auth: AuthUser,
    Path((_project_id, rule_id)): Path<(i64, i64)>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM automation_rules WHERE id = ?")
        .bind(rule_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Delete error: {}", e) })),
            )
        })?;

    Ok(Json(json!({ "message": "Automation rule deleted" })))
}
