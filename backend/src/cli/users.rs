use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

use crate::auth::create_token;

#[derive(Debug, Serialize, FromRow)]
pub struct UserCliRow {
    pub id: i64,
    pub email: String,
    pub full_name: String,
    pub avatar_url: Option<String>,
}

pub async fn list_users(pool: &MySqlPool, as_json: bool) -> Result<(), String> {
    let sql = "SELECT id, email, full_name, avatar_url FROM users ORDER BY id ASC";

    let users: Vec<UserCliRow> = sqlx::query_as(sql)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&users).unwrap());
        return Ok(());
    }

    println!("\n👥 EpicTask Users (Total: {})", users.len());
    println!("{:-<75}", "");
    println!("{:<6} {:<32} {:<32}", "ID", "FULL NAME", "EMAIL");
    println!("{:-<75}", "");

    for u in users {
        println!("{:<6} {:<32} {:<32}", u.id, u.full_name, u.email);
    }
    println!("{:-<75}\n", "");
    Ok(())
}

pub async fn generate_token(pool: &MySqlPool, email_or_id: &str, jwt_secret: &str, as_json: bool) -> Result<(), String> {
    let user: Result<UserCliRow, _> = if let Ok(id) = email_or_id.parse::<i64>() {
        sqlx::query_as("SELECT id, email, full_name, avatar_url FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
    } else {
        sqlx::query_as("SELECT id, email, full_name, avatar_url FROM users WHERE email = ?")
            .bind(email_or_id)
            .fetch_one(pool)
            .await
    };

    let u = user.map_err(|_| format!("User '{}' not found", email_or_id))?;

    let expires_at = chrono::Utc::now() + chrono::Duration::days(30);
    let token = create_token(u.id, &u.email, jwt_secret)
        .map_err(|e| format!("Token creation failed: {}", e))?;

    if as_json {
        println!("{}", serde_json::json!({
            "token": token,
            "user_id": u.id,
            "email": u.email,
            "name": u.full_name,
            "expires_at": expires_at.to_rfc3339(),
        }));
    } else {
        println!("\n🔑 Development JWT Token for {} ({})", u.full_name, u.email);
        println!("{:-<80}", "");
        println!("Authorization: Bearer {}", token);
        println!("{:-<80}\n", "");
    }
    Ok(())
}
