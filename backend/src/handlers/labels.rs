use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    handlers::issues::helpers::fetch_single_issue,
    models::{CreateLabelPayload, Label, SetIssueLabelsPayload},
    ws::WsBroadcastMessage,
    AppState,
};

pub async fn list_project_labels(
    _auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sql = "SELECT l.id, l.project_id, l.name, l.color, l.created_at, \
                      COUNT(il.issue_id) AS issue_count \
               FROM labels l \
               LEFT JOIN issue_labels il ON il.label_id = l.id \
               WHERE l.project_id = ? \
               GROUP BY l.id \
               ORDER BY l.name ASC";

    let labels = sqlx::query_as::<_, Label>(sql)
        .bind(project_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to list labels: {}", e) })),
            )
        })?;

    Ok(Json(labels))
}

pub async fn create_label(
    auth: AuthUser,
    Path(project_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateLabelPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let clean_name = payload.name.trim().to_lowercase();
    if clean_name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Label name cannot be empty" })),
        ));
    }

    let color = payload.color.unwrap_or_else(|| "#6366f1".to_string());

    // Insert or fetch existing
    let res = sqlx::query(
        "INSERT INTO labels (project_id, name, color) \
         VALUES (?, ?, ?) \
         ON DUPLICATE KEY UPDATE color = VALUES(color)"
    )
    .bind(project_id)
    .bind(&clean_name)
    .bind(&color)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to save label: {}", e) })),
        )
    })?;

    let label_id = if res.last_insert_id() > 0 {
        res.last_insert_id() as i64
    } else {
        let row: (i64,) = sqlx::query_as("SELECT id FROM labels WHERE project_id = ? AND name = ?")
            .bind(project_id)
            .bind(&clean_name)
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Failed to query label: {}", e) })),
                )
            })?;
        row.0
    };

    let label = sqlx::query_as::<_, Label>(
        "SELECT id, project_id, name, color, created_at, 0 AS issue_count \
         FROM labels WHERE id = ?"
    )
    .bind(label_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to fetch label: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "LABEL_CREATED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: serde_json::to_value(&label).unwrap_or_default(),
    });

    Ok((StatusCode::CREATED, Json(label)))
}

pub async fn delete_label(
    auth: AuthUser,
    Path(label_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let row: Result<(i64,), _> = sqlx::query_as("SELECT project_id FROM labels WHERE id = ?")
        .bind(label_id)
        .fetch_one(&state.db)
        .await;

    let project_id = match row {
        Ok(r) => r.0,
        Err(_) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Label not found" })),
            ))
        }
    };

    sqlx::query("DELETE FROM labels WHERE id = ?")
        .bind(label_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to delete label: {}", e) })),
            )
        })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "LABEL_DELETED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: json!({ "id": label_id, "project_id": project_id }),
    });

    Ok(Json(json!({ "status": "success", "id": label_id })))
}

pub async fn get_issue_labels(
    _auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sql = "SELECT l.id, l.project_id, l.name, l.color, l.created_at, 1 AS issue_count \
               FROM labels l \
               JOIN issue_labels il ON il.label_id = l.id \
               WHERE il.issue_id = ? \
               ORDER BY l.name ASC";

    let labels = sqlx::query_as::<_, Label>(sql)
        .bind(issue_id)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to fetch issue labels: {}", e) })),
            )
        })?;

    Ok(Json(labels))
}

pub async fn set_issue_labels(
    auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<SetIssueLabelsPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let issue_row: Result<(i64, String), _> = sqlx::query_as("SELECT project_id, `key` FROM issues WHERE id = ?")
        .bind(issue_id)
        .fetch_one(&state.db)
        .await;

    let (project_id, _issue_key) = match issue_row {
        Ok(r) => r,
        Err(_) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Issue not found" })),
            ))
        }
    };

    let mut tx = state.db.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Transaction error: {}", e) })),
        )
    })?;

    // Clear existing associations
    sqlx::query("DELETE FROM issue_labels WHERE issue_id = ?")
        .bind(issue_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to clear old labels: {}", e) })),
            )
        })?;

    // Predefined vibrant label colors for auto-assignment
    let default_colors = [
        "#6366f1", "#06b6d4", "#10b981", "#f59e0b", "#ec4899",
        "#8b5cf6", "#3b82f6", "#14b8a6", "#f97316", "#84cc16",
    ];

    for (idx, name_raw) in payload.labels.iter().enumerate() {
        let clean = name_raw.trim().to_lowercase();
        if clean.is_empty() {
            continue;
        }

        let assigned_color = default_colors[idx % default_colors.len()];

        // Insert or ignore label
        sqlx::query(
            "INSERT INTO labels (project_id, name, color) \
             VALUES (?, ?, ?) \
             ON DUPLICATE KEY UPDATE name=name"
        )
        .bind(project_id)
        .bind(&clean)
        .bind(assigned_color)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to ensure label: {}", e) })),
            )
        })?;

        let label_row: (i64,) = sqlx::query_as("SELECT id FROM labels WHERE project_id = ? AND name = ?")
            .bind(project_id)
            .bind(&clean)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Failed to query label ID: {}", e) })),
                )
            })?;

        // Link issue to label
        sqlx::query(
            "INSERT IGNORE INTO issue_labels (issue_id, label_id) VALUES (?, ?)"
        )
        .bind(issue_id)
        .bind(label_row.0)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to link label: {}", e) })),
            )
        })?;
    }

    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Commit error: {}", e) })),
        )
    })?;

    // Fetch updated issue
    let updated_issue = fetch_single_issue(&state.db, issue_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Fetch error: {}", e) })),
        )
    })?;

    state.ws_hub.broadcast(WsBroadcastMessage {
        event: "ISSUE_UPDATED".to_string(),
        project_id: Some(project_id),
        user_id: Some(auth.user_id),
        data: serde_json::to_value(&updated_issue).unwrap_or_default(),
    });

    Ok(Json(updated_issue))
}
