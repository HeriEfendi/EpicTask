use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

use super::projects::resolve_project_id;

#[derive(Debug, Serialize, FromRow)]
pub struct ComponentCliRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub lead_name: Option<String>,
    pub issue_count: i64,
}

pub async fn list_components(pool: &MySqlPool, project_id_or_key: &str, as_json: bool) -> Result<(), String> {
    let project_id = resolve_project_id(pool, project_id_or_key).await?;

    let sql = "SELECT c.id, c.name, c.description, u.full_name AS lead_name, \
                      COUNT(i.id) AS issue_count \
               FROM components c \
               LEFT JOIN users u ON c.lead_user_id = u.id \
               LEFT JOIN issues i ON i.component_id = c.id \
               WHERE c.project_id = ? \
               GROUP BY c.id \
               ORDER BY c.name ASC";

    let components: Vec<ComponentCliRow> = sqlx::query_as(sql)
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&components).unwrap());
        return Ok(());
    }

    println!("\n🧩 Project Components for #{} (Total: {})", project_id, components.len());
    println!("{:-<80}", "");
    println!("{:<6} {:<24} {:<20} {:<10} {:<20}", "ID", "COMPONENT NAME", "LEAD", "ISSUES", "DESCRIPTION");
    println!("{:-<80}", "");

    for c in components {
        println!(
            "{:<6} {:<24} {:<20} {:<10} {:<20}",
            c.id,
            c.name,
            c.lead_name.unwrap_or_else(|| "-".to_string()),
            c.issue_count,
            c.description.unwrap_or_else(|| "-".to_string())
        );
    }
    println!("{:-<80}\n", "");
    Ok(())
}

pub async fn create_component(
    pool: &MySqlPool,
    project_id_or_key: &str,
    name: &str,
    description: Option<&str>,
    lead_user_id: Option<i64>,
    as_json: bool,
) -> Result<(), String> {
    let project_id = resolve_project_id(pool, project_id_or_key).await?;

    let res = sqlx::query(
        "INSERT INTO components (project_id, name, description, lead_user_id) VALUES (?, ?, ?, ?)"
    )
    .bind(project_id)
    .bind(name)
    .bind(description)
    .bind(lead_user_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create component: {}", e))?;

    let new_id = res.last_insert_id() as i64;

    if as_json {
        println!("{}", serde_json::json!({ "success": true, "id": new_id, "name": name }));
    } else {
        println!("✅ Component '🧩 {}' created successfully (ID: {})", name, new_id);
    }
    Ok(())
}

pub async fn delete_component(pool: &MySqlPool, component_id: i64, as_json: bool) -> Result<(), String> {
    let res = sqlx::query("DELETE FROM components WHERE id = ?")
        .bind(component_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete component: {}", e))?;

    if res.rows_affected() == 0 {
        return Err(format!("Component #{} not found", component_id));
    }

    if as_json {
        println!("{}", serde_json::json!({ "success": true, "deleted_id": component_id }));
    } else {
        println!("🗑️  Component #{} deleted successfully.", component_id);
    }
    Ok(())
}
