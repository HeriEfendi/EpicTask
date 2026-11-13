use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

use super::issues::resolve_issue_id;

#[derive(Debug, Serialize, FromRow)]
pub struct WatcherCliRow {
    pub user_id: i64,
    pub full_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

pub async fn list_watchers(pool: &MySqlPool, issue_id_or_key: &str, as_json: bool) -> Result<(), String> {
    let issue_id = resolve_issue_id(pool, issue_id_or_key).await?;

    let sql = "SELECT u.id AS user_id, u.full_name, u.email, u.avatar_url \
               FROM issue_watchers iw \
               JOIN users u ON iw.user_id = u.id \
               WHERE iw.issue_id = ? \
               ORDER BY iw.created_at ASC";

    let watchers: Vec<WatcherCliRow> = sqlx::query_as(sql)
        .bind(issue_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&watchers).unwrap());
        return Ok(());
    }

    println!("\n👁️  Watchers for Issue #{} (Total: {})", issue_id, watchers.len());
    println!("{:-<60}", "");
    println!("{:<8} {:<24} {:<28}", "USER ID", "FULL NAME", "EMAIL");
    println!("{:-<60}", "");

    for w in watchers {
        println!("{:<8} {:<24} {:<28}", w.user_id, w.full_name, w.email);
    }
    println!("{:-<60}\n", "");
    Ok(())
}

pub async fn add_watcher(pool: &MySqlPool, issue_id_or_key: &str, user_id: i64, as_json: bool) -> Result<(), String> {
    let issue_id = resolve_issue_id(pool, issue_id_or_key).await?;

    let user: (String,) = sqlx::query_as("SELECT full_name FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|_| format!("User #{} not found", user_id))?;

    sqlx::query("INSERT INTO issue_watchers (issue_id, user_id) VALUES (?, ?) ON DUPLICATE KEY UPDATE issue_id=issue_id")
        .bind(issue_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to add watcher: {}", e))?;

    if as_json {
        println!("{}", serde_json::json!({ "success": true, "issue_id": issue_id, "user_id": user_id, "name": user.0 }));
    } else {
        println!("👁️  User '{}' (ID: {}) is now watching issue #{}.", user.0, user_id, issue_id);
    }
    Ok(())
}

pub async fn remove_watcher(pool: &MySqlPool, issue_id_or_key: &str, user_id: i64, as_json: bool) -> Result<(), String> {
    let issue_id = resolve_issue_id(pool, issue_id_or_key).await?;

    let res = sqlx::query("DELETE FROM issue_watchers WHERE issue_id = ? AND user_id = ?")
        .bind(issue_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to remove watcher: {}", e))?;

    if res.rows_affected() == 0 {
        return Err(format!("User #{} was not watching issue #{}", user_id, issue_id));
    }

    if as_json {
        println!("{}", serde_json::json!({ "success": true, "issue_id": issue_id, "user_id": user_id }));
    } else {
        println!("🗑️  User #{} stopped watching issue #{}.", user_id, issue_id);
    }
    Ok(())
}
