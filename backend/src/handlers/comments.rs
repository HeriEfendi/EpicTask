use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::AuthUser,
    models::{Comment, CreateCommentPayload},
    ws::WsBroadcastMessage,
    AppState,
};

pub async fn list_comments(
    _auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let comments = sqlx::query_as::<_, Comment>(
        "SELECT c.id, c.issue_id, c.user_id, c.body, c.created_at, \
                u.full_name AS user_name, u.avatar_url AS user_avatar \
         FROM comments c \
         LEFT JOIN users u ON c.user_id = u.id \
         WHERE c.issue_id = ? \
         ORDER BY c.created_at ASC"
    )
    .bind(issue_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    Ok(Json(comments))
}

pub async fn create_comment(
    auth: AuthUser,
    Path(issue_id): Path<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateCommentPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let body = payload.body.trim();
    if body.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Comment body cannot be empty" })),
        ));
    }

    let res = sqlx::query("INSERT INTO comments (issue_id, user_id, body) VALUES (?, ?, ?)")
        .bind(issue_id)
        .bind(auth.user_id)
        .bind(body)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {}", e) })),
            )
        })?;

    let comment_id = res.last_insert_id() as i64;

    // Fetch issue details
    let issue_info: Result<(i64, String, Option<i64>), _> = sqlx::query_as(
        "SELECT project_id, `key`, assignee_id FROM issues WHERE id = ?"
    )
    .bind(issue_id)
    .fetch_one(&state.db)
    .await;

    if let Ok((project_id, key, assignee_id)) = issue_info {
        // Detect @mentions (FR-4.3, FR-6.3)
        // Check for words starting with @
        let words: Vec<&str> = body.split_whitespace().collect();
        for word in words {
            if let Some(mention_target) = word.strip_prefix('@') {
                let clean_mention = mention_target.trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != '_');
                if !clean_mention.is_empty() {
                    // Look up user by email prefix or full_name
                    let target_user: Result<(i64, String), _> = sqlx::query_as(
                        "SELECT id, full_name FROM users WHERE email LIKE ? OR full_name LIKE ? LIMIT 1"
                    )
                    .bind(format!("{}%", clean_mention))
                    .bind(format!("{}%", clean_mention))
                    .fetch_one(&state.db)
                    .await;

                    if let Ok((mentioned_user_id, _target_name)) = target_user {
                        if mentioned_user_id != auth.user_id {
                            let _ = sqlx::query(
                                "INSERT INTO notifications (recipient_id, sender_id, issue_id, title, message, action_type) \
                                 VALUES (?, ?, ?, 'Mentioned you in a comment', ?, 'MENTION')"
                            )
                            .bind(mentioned_user_id)
                            .bind(auth.user_id)
                            .bind(issue_id)
                            .bind(format!("You were mentioned in a comment on {}: \"{}\"", key, body.chars().take(80).collect::<String>()))
                            .execute(&state.db)
                            .await;

                            state.ws_hub.broadcast(WsBroadcastMessage {
                                event: "NOTIFICATION".to_string(),
                                project_id: Some(project_id),
                                user_id: Some(mentioned_user_id),
                                data: json!({
                                    "title": "You were mentioned",
                                    "issue_key": key,
                                    "message": body
                                }),
                            });
                        }
                    }
                }
            }
        }

        // Also if assignee is not the commenter and not already notified, notify them of new comment
        if let Some(aid) = assignee_id {
            if aid != auth.user_id {
                let _ = sqlx::query(
                    "INSERT INTO notifications (recipient_id, sender_id, issue_id, title, message, action_type) \
                     VALUES (?, ?, ?, 'New comment on your ticket', ?, 'COMMENT')"
                )
                .bind(aid)
                .bind(auth.user_id)
                .bind(issue_id)
                .bind(format!("New comment on {}: \"{}\"", key, body.chars().take(80).collect::<String>()))
                .execute(&state.db)
                .await;
            }
        }

        // Activity log
        let _ = sqlx::query(
            "INSERT INTO activity_logs (project_id, issue_id, user_id, action, details) \
             VALUES (?, ?, ?, 'ADD_COMMENT', ?)"
        )
        .bind(project_id)
        .bind(issue_id)
        .bind(auth.user_id)
        .bind(format!("Commented on {}", key))
        .execute(&state.db)
        .await;

        // Fetch inserted comment
        let comment = sqlx::query_as::<_, Comment>(
            "SELECT c.id, c.issue_id, c.user_id, c.body, c.created_at, \
                    u.full_name AS user_name, u.avatar_url AS user_avatar \
             FROM comments c \
             LEFT JOIN users u ON c.user_id = u.id \
             WHERE c.id = ?"
        )
        .bind(comment_id)
        .fetch_one(&state.db)
        .await;

        if let Ok(c) = comment {
            state.ws_hub.broadcast(WsBroadcastMessage {
                event: "COMMENT_ADDED".to_string(),
                project_id: Some(project_id),
                user_id: Some(auth.user_id),
                data: serde_json::to_value(&c).unwrap_or_default(),
            });

            return Ok((StatusCode::CREATED, Json(serde_json::to_value(c).unwrap_or_default())));
        }
    }

    Ok((StatusCode::CREATED, Json(json!({ "id": comment_id, "message": "Comment added" }))))
}
