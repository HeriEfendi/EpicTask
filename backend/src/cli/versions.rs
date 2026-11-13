use chrono::NaiveDate;
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

use super::projects::resolve_project_id;

#[derive(Debug, Serialize, FromRow)]
pub struct VersionCliRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub release_date: Option<NaiveDate>,
    pub total_issues: i64,
    pub done_issues: i64,
    pub in_progress_issues: i64,
    pub todo_issues: i64,
}

pub async fn list_versions(pool: &MySqlPool, project_id_or_key: &str, as_json: bool) -> Result<(), String> {
    let project_id = resolve_project_id(pool, project_id_or_key).await?;

    let sql = "SELECT v.id, v.name, v.description, v.status, v.release_date, \
                      COUNT(i.id) AS total_issues, \
                      COUNT(CASE WHEN st.category = 'DONE' THEN 1 END) AS done_issues, \
                      COUNT(CASE WHEN st.category = 'IN_PROGRESS' THEN 1 END) AS in_progress_issues, \
                      COUNT(CASE WHEN st.category = 'TODO' THEN 1 END) AS todo_issues \
               FROM versions v \
               LEFT JOIN issues i ON i.fix_version_id = v.id \
               LEFT JOIN statuses st ON i.status_id = st.id \
               WHERE v.project_id = ? \
               GROUP BY v.id \
               ORDER BY v.release_date ASC, v.id DESC";

    let versions: Vec<VersionCliRow> = sqlx::query_as(sql)
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&versions).unwrap());
        return Ok(());
    }

    println!("\n🚀 Release Hub / Versions for #{} (Total: {})", project_id, versions.len());
    println!("{:-<100}", "");
    println!("{:<6} {:<20} {:<14} {:<15} {:<12} {:<25}", "ID", "VERSION NAME", "STATUS", "RELEASE DATE", "TOTAL ISSUES", "RATIO (DONE/PROG/TODO)");
    println!("{:-<100}", "");

    for v in versions {
        let status_badge = match v.status.as_str() {
            "RELEASED" => "🟢 RELEASED",
            "ARCHIVED" => "⚪ ARCHIVED",
            _ => "🔵 UNRELEASED",
        };
        let ratio = format!("{} / {} / {}", v.done_issues, v.in_progress_issues, v.todo_issues);
        println!(
            "{:<6} {:<20} {:<14} {:<15} {:<12} {:<25}",
            v.id,
            v.name,
            status_badge,
            v.release_date.map(|d| d.to_string()).unwrap_or_else(|| "-".to_string()),
            v.total_issues,
            ratio
        );
    }
    println!("{:-<100}\n", "");
    Ok(())
}

pub async fn create_version(
    pool: &MySqlPool,
    project_id_or_key: &str,
    name: &str,
    description: Option<&str>,
    release_date: Option<&str>,
    as_json: bool,
) -> Result<(), String> {
    let project_id = resolve_project_id(pool, project_id_or_key).await?;

    let parsed_date = release_date.and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let res = sqlx::query(
        "INSERT INTO versions (project_id, name, description, release_date, status) \
         VALUES (?, ?, ?, ?, 'UNRELEASED')"
    )
    .bind(project_id)
    .bind(name)
    .bind(description)
    .bind(parsed_date)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create version: {}", e))?;

    let new_id = res.last_insert_id() as i64;

    if as_json {
        println!("{}", serde_json::json!({ "success": true, "id": new_id, "name": name }));
    } else {
        println!("✅ Version '🚀 {}' created successfully (ID: {})", name, new_id);
    }
    Ok(())
}

pub async fn release_version(
    pool: &MySqlPool,
    version_id: i64,
    move_to_version_id: Option<i64>,
    as_json: bool,
) -> Result<(), String> {
    let ver: (i64, String) = sqlx::query_as("SELECT project_id, name FROM versions WHERE id = ?")
        .bind(version_id)
        .fetch_one(pool)
        .await
        .map_err(|_| format!("Version #{} not found", version_id))?;

    sqlx::query("UPDATE versions SET status = 'RELEASED', release_date = CURRENT_DATE() WHERE id = ?")
        .bind(version_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to release version: {}", e))?;

    let moved_count = if let Some(target_ver_id) = move_to_version_id {
        sqlx::query(
            "UPDATE issues i JOIN statuses s ON i.status_id = s.id \
             SET i.fix_version_id = ? \
             WHERE i.fix_version_id = ? AND s.category != 'DONE'"
        )
        .bind(target_ver_id)
        .bind(version_id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected())
        .unwrap_or(0)
    } else {
        sqlx::query(
            "UPDATE issues i JOIN statuses s ON i.status_id = s.id \
             SET i.fix_version_id = NULL \
             WHERE i.fix_version_id = ? AND s.category != 'DONE'"
        )
        .bind(version_id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected())
        .unwrap_or(0)
    };

    if as_json {
        println!("{}", serde_json::json!({
            "success": true,
            "id": version_id,
            "status": "RELEASED",
            "unresolved_issues_migrated": moved_count,
        }));
    } else {
        println!("🚀 Version '{}' released! {} unresolved issues migrated.", ver.1, moved_count);
    }
    Ok(())
}

pub async fn archive_version(pool: &MySqlPool, version_id: i64, as_json: bool) -> Result<(), String> {
    let ver: (String, String) = sqlx::query_as("SELECT name, status FROM versions WHERE id = ?")
        .bind(version_id)
        .fetch_one(pool)
        .await
        .map_err(|_| format!("Version #{} not found", version_id))?;

    let next_status = if ver.1 == "ARCHIVED" { "UNRELEASED" } else { "ARCHIVED" };

    sqlx::query("UPDATE versions SET status = ? WHERE id = ?")
        .bind(next_status)
        .bind(version_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update archive status: {}", e))?;

    if as_json {
        println!("{}", serde_json::json!({ "success": true, "id": version_id, "status": next_status }));
    } else {
        println!("📦 Version '{}' status updated to: {}", ver.0, next_status);
    }
    Ok(())
}
