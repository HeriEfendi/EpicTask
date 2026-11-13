use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

use super::issues::resolve_issue_id;
use super::projects::resolve_project_id;

#[derive(Debug, Serialize, FromRow)]
pub struct LabelCliRow {
    pub id: i64,
    pub name: String,
    pub color: Option<String>,
    pub issue_count: i64,
}

pub async fn list_labels(pool: &MySqlPool, project_id_or_key: &str, as_json: bool) -> Result<(), String> {
    let project_id = resolve_project_id(pool, project_id_or_key).await?;

    let sql = "SELECT l.id, l.name, l.color, \
                      COUNT(il.issue_id) AS issue_count \
               FROM labels l \
               LEFT JOIN issue_labels il ON il.label_id = l.id \
               WHERE l.project_id = ? \
               GROUP BY l.id \
               ORDER BY l.name ASC";

    let labels: Vec<LabelCliRow> = sqlx::query_as(sql)
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&labels).unwrap());
        return Ok(());
    }

    println!("\n🏷️  Labels in Project #{} (Total: {})", project_id, labels.len());
    println!("{:-<60}", "");
    println!("{:<6} {:<24} {:<12} {:<10}", "ID", "LABEL NAME", "COLOR", "ISSUES");
    println!("{:-<60}", "");

    for l in labels {
        println!(
            "{:<6} {:<24} {:<12} {:<10}",
            l.id,
            format!("🏷️ {}", l.name),
            l.color.unwrap_or_else(|| "#6366f1".to_string()),
            l.issue_count
        );
    }
    println!("{:-<60}\n", "");
    Ok(())
}

pub async fn add_label(
    pool: &MySqlPool,
    issue_id_or_key: &str,
    label_name: &str,
    color: Option<&str>,
    as_json: bool,
) -> Result<(), String> {
    let issue_id = resolve_issue_id(pool, issue_id_or_key).await?;

    let issue: (i64, String) = sqlx::query_as("SELECT project_id, `key` FROM issues WHERE id = ?")
        .bind(issue_id)
        .fetch_one(pool)
        .await
        .map_err(|_| format!("Issue not found"))?;

    let clean = label_name.trim().to_lowercase();
    let col = color.unwrap_or("#6366f1");

    sqlx::query("INSERT INTO labels (project_id, name, color) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE name=name")
        .bind(issue.0)
        .bind(&clean)
        .bind(col)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create label: {}", e))?;

    let lbl: (i64,) = sqlx::query_as("SELECT id FROM labels WHERE project_id = ? AND name = ?")
        .bind(issue.0)
        .bind(&clean)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to fetch label: {}", e))?;

    sqlx::query("INSERT IGNORE INTO issue_labels (issue_id, label_id) VALUES (?, ?)")
        .bind(issue_id)
        .bind(lbl.0)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to attach label: {}", e))?;

    if as_json {
        println!("{}", serde_json::json!({ "success": true, "issue_id": issue_id, "label": clean }));
    } else {
        println!("🏷️  Label '{}' attached to issue {}!", clean, issue.1);
    }
    Ok(())
}

pub async fn remove_label(pool: &MySqlPool, issue_id_or_key: &str, label_name: &str, as_json: bool) -> Result<(), String> {
    let issue_id = resolve_issue_id(pool, issue_id_or_key).await?;

    let clean = label_name.trim().to_lowercase();

    sqlx::query(
        "DELETE il FROM issue_labels il \
         JOIN labels l ON il.label_id = l.id \
         WHERE il.issue_id = ? AND l.name = ?"
    )
    .bind(issue_id)
    .bind(&clean)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to remove label: {}", e))?;

    if as_json {
        println!("{}", serde_json::json!({ "success": true, "issue_id": issue_id, "removed_label": clean }));
    } else {
        println!("🗑️  Label '{}' removed from issue #{}.", clean, issue_id);
    }
    Ok(())
}
