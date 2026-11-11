use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use super::helpers::fetch_single_issue;
use crate::{
    auth::AuthUser,
    models::CreateIssueLinkPayload,
    ws::WsBroadcastMessage,
    AppState,
};

pub async fn delete_issue(
    auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let issue = fetch_single_issue(&state.db, issue_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Issue not found" })),
        )
    })?;

    sqlx::query("DELETE FROM issues WHERE id = ?")
        .bind(issue_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Delete failed: {}", e) })),
            )
        })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "ISSUE_DELETED".to_string(),
        project_id: Some(issue.project_id),
        user_id: Some(auth.user_id),
        data: json!({ "issue_id": issue_id, "key": issue.key }),
    });

    Ok(Json(json!({ "message": "Issue deleted successfully" })))
}

pub async fn create_issue_link(
    _auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateIssueLinkPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = sqlx::query(
        "INSERT INTO issue_links (source_issue_id, target_issue_id, link_type) VALUES (?, ?, ?)"
    )
    .bind(issue_id)
    .bind(payload.target_issue_id)
    .bind(&payload.link_type)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to create link: {}", e) })),
        )
    })?;

    Ok((StatusCode::CREATED, Json(json!({ "message": "Issue linked successfully" }))))
}

