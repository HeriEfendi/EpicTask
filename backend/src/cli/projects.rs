use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Serialize, FromRow)]
pub struct ProjectRow {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub project_type: String,
    pub current_issue_counter: i32,
    pub issue_count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct StatusRow {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub position: i32,
    pub color: Option<String>,
}

pub async fn resolve_project_id(pool: &MySqlPool, id_or_key: &str) -> Result<i64, String> {
    if let Ok(id) = id_or_key.parse::<i64>() {
        return Ok(id);
    }
    let res: Result<(i64,), _> = sqlx::query_as("SELECT id FROM projects WHERE `key` = ?")
        .bind(id_or_key.to_uppercase())
        .fetch_one(pool)
        .await;
    match res {
        Ok((id,)) => Ok(id),
        Err(_) => Err(format!("Project '{}' not found", id_or_key)),
    }
}

pub async fn list_projects(pool: &MySqlPool, as_json: bool) -> Result<(), String> {
    let sql = "SELECT p.id, p.`key`, p.name, p.project_type, p.current_issue_counter, \
                      (SELECT COUNT(*) FROM issues i WHERE i.project_id = p.id) AS issue_count \
               FROM projects p \
               ORDER BY p.id ASC";

    let projects: Vec<ProjectRow> = sqlx::query_as(sql)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&projects).unwrap());
        return Ok(());
    }

    println!("\n📁 EpicTask Projects (Total: {})", projects.len());
    println!("{:-<95}", "");
    println!("{:<6} {:<10} {:<32} {:<12} {:<10} {:<15}", "ID", "KEY", "NAME", "TYPE", "ISSUES", "COUNTER");
    println!("{:-<95}", "");

    for p in projects {
        println!(
            "{:<6} {:<10} {:<32} {:<12} {:<10} {:<15}",
            p.id,
            p.key,
            if p.name.len() > 30 { format!("{}...", &p.name[..27]) } else { p.name },
            p.project_type,
            p.issue_count,
            format!("#{}-{}", p.key, p.current_issue_counter)
        );
    }
    println!("{:-<95}\n", "");
    Ok(())
}

pub async fn list_statuses(pool: &MySqlPool, id_or_key: &str, as_json: bool) -> Result<(), String> {
    let project_id = resolve_project_id(pool, id_or_key).await?;

    let sql = "SELECT id, name, category, position, color FROM statuses \
               WHERE project_id = ? ORDER BY position ASC";

    let statuses: Vec<StatusRow> = sqlx::query_as(sql)
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&statuses).unwrap());
        return Ok(());
    }

    println!("\n🔄 Workflow Statuses for Project #{} (Total: {})", project_id, statuses.len());
    println!("{:-<70}", "");
    println!("{:<6} {:<24} {:<16} {:<10} {:<10}", "ID", "STATUS NAME", "CATEGORY", "POSITION", "COLOR");
    println!("{:-<70}", "");

    for s in statuses {
        let cat_badge = match s.category.as_str() {
            "TODO" => "⚪ TODO",
            "IN_PROGRESS" => "🔵 IN_PROGRESS",
            "DONE" => "🟢 DONE",
            _ => "⚫ UNKNOWN",
        };
        println!(
            "{:<6} {:<24} {:<16} {:<10} {:<10}",
            s.id,
            s.name,
            cat_badge,
            s.position,
            s.color.unwrap_or_else(|| "-".to_string())
        );
    }
    println!("{:-<70}\n", "");
    Ok(())
}
