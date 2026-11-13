use chrono::NaiveDate;
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

use crate::models::{Issue, IssueWatcher};
use super::projects::resolve_project_id;

#[derive(Debug, Serialize, FromRow)]
pub struct IssueSummaryRow {
    pub id: i64,
    pub key: String,
    pub summary: String,
    pub issue_type: String,
    pub status_name: String,
    pub status_category: String,
    pub priority: Option<String>,
    pub resolution: Option<String>,
    pub assignee_name: Option<String>,
    pub story_points: Option<i32>,
    pub sprint_name: Option<String>,
    pub component_name: Option<String>,
    pub fix_version_name: Option<String>,
    pub labels: Option<String>,
    pub watcher_count: i64,
}

pub async fn resolve_issue_id(pool: &MySqlPool, id_or_key: &str) -> Result<i64, String> {
    if let Ok(id) = id_or_key.parse::<i64>() {
        return Ok(id);
    }
    let res: Result<(i64,), _> = sqlx::query_as("SELECT id FROM issues WHERE `key` = ?")
        .bind(id_or_key.to_uppercase())
        .fetch_one(pool)
        .await;
    match res {
        Ok((id,)) => Ok(id),
        Err(_) => Err(format!("Issue '{}' not found", id_or_key)),
    }
}

pub struct ListIssuesFilter<'a> {
    pub status: Option<&'a str>,
    pub assignee_id: Option<i64>,
    pub issue_type: Option<&'a str>,
    pub priority: Option<&'a str>,
    pub resolution: Option<&'a str>,
    pub sprint_id: Option<i64>,
    pub component_id: Option<i64>,
    pub label: Option<&'a str>,
    pub search: Option<&'a str>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn list_issues(
    pool: &MySqlPool,
    project_id_or_key: &str,
    filters: ListIssuesFilter<'_>,
    as_json: bool,
) -> Result<(), String> {
    let project_id = resolve_project_id(pool, project_id_or_key).await?;

    let mut sql = String::from(
        "SELECT i.id, i.`key`, i.summary, i.issue_type, \
                s.name AS status_name, s.category AS status_category, \
                i.priority, i.resolution, \
                ua.full_name AS assignee_name, i.story_points, \
                sp.name AS sprint_name, comp.name AS component_name, ver.name AS fix_version_name, \
                (SELECT GROUP_CONCAT(lbl.name SEPARATOR ', ') FROM issue_labels il JOIN labels lbl ON il.label_id = lbl.id WHERE il.issue_id = i.id) AS labels, \
                (SELECT COUNT(*) FROM issue_watchers iw WHERE iw.issue_id = i.id) AS watcher_count \
         FROM issues i \
         JOIN statuses s ON i.status_id = s.id \
         LEFT JOIN users ua ON i.assignee_id = ua.id \
         LEFT JOIN sprints sp ON i.sprint_id = sp.id \
         LEFT JOIN components comp ON i.component_id = comp.id \
         LEFT JOIN versions ver ON i.fix_version_id = ver.id \
         WHERE i.project_id = ?"
    );

    if let Some(status_str) = filters.status {
        if let Ok(sid) = status_str.parse::<i64>() {
            sql.push_str(&format!(" AND i.status_id = {}", sid));
        } else {
            sql.push_str(&format!(" AND (s.name = '{}' OR s.category = '{}')", status_str.replace('\'', "''"), status_str.replace('\'', "''")));
        }
    }
    if let Some(aid) = filters.assignee_id {
        sql.push_str(&format!(" AND i.assignee_id = {}", aid));
    }
    if let Some(itype) = filters.issue_type {
        sql.push_str(&format!(" AND i.issue_type = '{}'", itype.to_uppercase()));
    }
    if let Some(prio) = filters.priority {
        sql.push_str(&format!(" AND i.priority = '{}'", prio.to_uppercase()));
    }
    if let Some(res) = filters.resolution {
        if res.eq_ignore_ascii_case("unresolved") {
            sql.push_str(" AND i.resolution IS NULL");
        } else {
            sql.push_str(&format!(" AND i.resolution = '{}'", res.replace('\'', "''")));
        }
    }
    if let Some(sp_id) = filters.sprint_id {
        sql.push_str(&format!(" AND i.sprint_id = {}", sp_id));
    }
    if let Some(c_id) = filters.component_id {
        sql.push_str(&format!(" AND i.component_id = {}", c_id));
    }
    if let Some(lbl) = filters.label {
        sql.push_str(&format!(
            " AND EXISTS (SELECT 1 FROM issue_labels il JOIN labels l ON il.label_id = l.id WHERE il.issue_id = i.id AND l.name = '{}')",
            lbl.trim().to_lowercase().replace('\'', "''")
        ));
    }
    if let Some(q) = filters.search {
        if !q.trim().is_empty() {
            sql.push_str(&format!(
                " AND (i.summary LIKE '%{}%' OR i.`key` LIKE '%{}%')",
                q.replace('\'', "''"),
                q.replace('\'', "''")
            ));
        }
    }

    sql.push_str(" ORDER BY s.position ASC, i.position ASC, i.id DESC");

    let limit = filters.limit.unwrap_or(50);
    sql.push_str(&format!(" LIMIT {}", limit));
    if let Some(offset) = filters.offset {
        sql.push_str(&format!(" OFFSET {}", offset));
    }

    let issues: Vec<IssueSummaryRow> = sqlx::query_as(&sql)
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&issues).unwrap());
        return Ok(());
    }

    println!("\n📋 EpicTask Issues for Project #{} (Total: {})", project_id, issues.len());
    println!("{:-<120}", "");
    println!("{:<10} {:<8} {:<36} {:<15} {:<12} {:<16} {:<5} {:<10}", "KEY", "TYPE", "SUMMARY", "STATUS", "RESOLUTION", "ASSIGNEE", "PTS", "WATCHERS");
    println!("{:-<120}", "");

    for i in issues {
        let type_icon = match i.issue_type.as_str() {
            "BUG" => "🐞 BUG",
            "STORY" => "📖 STORY",
            "EPIC" => "🟣 EPIC",
            _ => "📋 TASK",
        };
        let res_text = i.resolution.unwrap_or_else(|| "-".to_string());
        println!(
            "{:<10} {:<8} {:<36} {:<15} {:<12} {:<16} {:<5} {:<10}",
            i.key,
            type_icon,
            if i.summary.len() > 34 { format!("{}...", &i.summary[..31]) } else { i.summary },
            i.status_name,
            res_text,
            i.assignee_name.unwrap_or_else(|| "Unassigned".to_string()),
            i.story_points.unwrap_or(0),
            format!("👁️ {}", i.watcher_count)
        );
    }
    println!("{:-<120}\n", "");
    Ok(())
}

pub async fn get_issue(pool: &MySqlPool, id_or_key: &str, as_json: bool) -> Result<(), String> {
    let issue_id = resolve_issue_id(pool, id_or_key).await?;

    let issue: Issue = sqlx::query_as(
        "SELECT i.id, i.project_id, i.parent_id, i.epic_id, i.`key`, i.summary, i.description, \
                i.issue_type, i.status_id, i.priority, i.assignee_id, i.reporter_id, i.story_points, \
                i.start_date, i.due_date, i.position, i.sprint_id, i.component_id, i.fix_version_id, \
                i.resolution, i.resolved_at, i.created_at, i.updated_at, \
                s.name AS status_name, s.category AS status_category, s.color AS status_color, \
                ua.full_name AS assignee_name, ua.avatar_url AS assignee_avatar, \
                ur.full_name AS reporter_name, \
                ep.summary AS epic_summary, \
                p.`key` AS parent_key, p.summary AS parent_summary, \
                sp.name AS sprint_name, \
                comp.name AS component_name, \
                ver.name AS fix_version_name, \
                (SELECT GROUP_CONCAT(lbl.name SEPARATOR ', ') FROM issue_labels il JOIN labels lbl ON il.label_id = lbl.id WHERE il.issue_id = i.id) AS labels, \
                (SELECT COUNT(*) FROM issues sub WHERE sub.parent_id = i.id) AS subtask_count, \
                (SELECT COUNT(*) FROM issues sub JOIN statuses subs ON sub.status_id = subs.id WHERE sub.parent_id = i.id AND subs.category = 'DONE') AS subtask_done_count, \
                (SELECT COUNT(*) FROM issue_watchers iw WHERE iw.issue_id = i.id) AS watcher_count \
         FROM issues i \
         JOIN statuses s ON i.status_id = s.id \
         LEFT JOIN users ua ON i.assignee_id = ua.id \
         LEFT JOIN users ur ON i.reporter_id = ur.id \
         LEFT JOIN issues ep ON i.epic_id = ep.id \
         LEFT JOIN issues p ON i.parent_id = p.id \
         LEFT JOIN sprints sp ON i.sprint_id = sp.id \
         LEFT JOIN components comp ON i.component_id = comp.id \
         LEFT JOIN versions ver ON i.fix_version_id = ver.id \
         WHERE i.id = ?"
    )
    .bind(issue_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Issue not found: {}", e))?;

    let watchers: Vec<IssueWatcher> = sqlx::query_as(
        "SELECT u.id AS user_id, u.full_name, u.email, u.avatar_url, iw.created_at \
         FROM issue_watchers iw \
         JOIN users u ON iw.user_id = u.id \
         WHERE iw.issue_id = ? \
         ORDER BY iw.created_at ASC"
    )
    .bind(issue_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    if as_json {
        let result = serde_json::json!({
            "issue": issue,
            "watchers": watchers,
        });
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
        return Ok(());
    }

    println!("\n╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("  Ticket: {} — {}", issue.key, issue.summary);
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝");
    println!("  Type:          {:<15} Priority:    {}", issue.issue_type, issue.priority.unwrap_or_else(|| "MEDIUM".into()));
    println!("  Status:        {:<15} Category:    {}", issue.status_name.unwrap_or_default(), issue.status_category.unwrap_or_default());
    println!("  Resolution:    {:<15} Resolved At: {}", issue.resolution.unwrap_or_else(|| "Unresolved".into()), issue.resolved_at.map(|d| d.to_string()).unwrap_or_else(|| "-".into()));
    println!("  Assignee:      {:<15} Reporter:    {}", issue.assignee_name.unwrap_or_else(|| "Unassigned".into()), issue.reporter_name.unwrap_or_else(|| "System".into()));
    println!("  Sprint:        {:<15} Points:      {}", issue.sprint_name.unwrap_or_else(|| "-".into()), issue.story_points.unwrap_or(0));
    println!("  Component:     {:<15} Fix Version: {}", issue.component_name.unwrap_or_else(|| "-".into()), issue.fix_version_name.unwrap_or_else(|| "-".into()));
    println!("  Labels:        {}", issue.labels.unwrap_or_else(|| "-".into()));
    println!("  Due Date:      {}", issue.due_date.map(|d| d.to_string()).unwrap_or_else(|| "-".into()));

    if let Some(desc) = &issue.description {
        println!("\n  [Description]\n  {}", desc.lines().collect::<Vec<_>>().join("\n  "));
    }

    println!("\n  [Watchers (Total: {})]", watchers.len());
    if watchers.is_empty() {
        println!("  (No watchers yet)");
    } else {
        for w in watchers {
            println!("  👁️  {} <{}>", w.full_name, w.email);
        }
    }
    println!();
    Ok(())
}

pub struct CreateIssueInput<'a> {
    pub summary: &'a str,
    pub description: Option<&'a str>,
    pub issue_type: Option<&'a str>,
    pub priority: Option<&'a str>,
    pub assignee_id: Option<i64>,
    pub sprint_id: Option<i64>,
    pub component_id: Option<i64>,
    pub fix_version_id: Option<i64>,
    pub story_points: Option<i32>,
    pub due_date: Option<&'a str>,
    pub labels: Option<Vec<String>>,
}

pub async fn create_issue(
    pool: &MySqlPool,
    project_id_or_key: &str,
    input: CreateIssueInput<'_>,
    as_json: bool,
) -> Result<(), String> {
    let project_id = resolve_project_id(pool, project_id_or_key).await?;

    let mut tx = pool.begin().await.map_err(|e| format!("Transaction error: {}", e))?;

    let proj: (String, i32) = sqlx::query_as(
        "SELECT `key`, current_issue_counter FROM projects WHERE id = ? FOR UPDATE"
    )
    .bind(project_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| format!("Failed to lock project: {}", e))?;

    let next_counter = proj.1 + 1;
    let issue_key = format!("{}-{}", proj.0, next_counter);

    sqlx::query("UPDATE projects SET current_issue_counter = ? WHERE id = ?")
        .bind(next_counter)
        .bind(project_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to update counter: {}", e))?;

    let first_status: (i64,) = sqlx::query_as(
        "SELECT id FROM statuses WHERE project_id = ? ORDER BY position ASC LIMIT 1"
    )
    .bind(project_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| format!("No statuses found: {}", e))?;

    let parsed_due: Option<NaiveDate> = input.due_date.and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
    let issue_type = input.issue_type.unwrap_or("TASK").to_uppercase();
    let priority = input.priority.unwrap_or("MEDIUM").to_uppercase();
    let points = input.story_points.unwrap_or(0);

    let res = sqlx::query(
        "INSERT INTO issues (project_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, due_date, sprint_id, component_id, fix_version_id) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?, ?, ?, ?)"
    )
    .bind(project_id)
    .bind(&issue_key)
    .bind(input.summary)
    .bind(input.description)
    .bind(&issue_type)
    .bind(first_status.0)
    .bind(&priority)
    .bind(input.assignee_id)
    .bind(points)
    .bind(parsed_due)
    .bind(input.sprint_id)
    .bind(input.component_id)
    .bind(input.fix_version_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Failed to insert issue: {}", e))?;

    let new_id = res.last_insert_id() as i64;

    // Attach initial labels if any
    if let Some(labels) = input.labels {
        for lbl_name in labels {
            let clean = lbl_name.trim().to_lowercase();
            if clean.is_empty() {
                continue;
            }
            let _ = sqlx::query("INSERT INTO labels (project_id, name) VALUES (?, ?) ON DUPLICATE KEY UPDATE name=name")
                .bind(project_id)
                .bind(&clean)
                .execute(&mut *tx)
                .await;

            let lbl_id: Result<(i64,), _> = sqlx::query_as("SELECT id FROM labels WHERE project_id = ? AND name = ?")
                .bind(project_id)
                .bind(&clean)
                .fetch_one(&mut *tx)
                .await;

            if let Ok(l) = lbl_id {
                let _ = sqlx::query("INSERT IGNORE INTO issue_labels (issue_id, label_id) VALUES (?, ?)")
                    .bind(new_id)
                    .bind(l.0)
                    .execute(&mut *tx)
                    .await;
            }
        }
    }

    tx.commit().await.map_err(|e| format!("Commit error: {}", e))?;

    if as_json {
        let json_res = serde_json::json!({
            "success": true,
            "id": new_id,
            "key": issue_key,
            "summary": input.summary,
        });
        println!("{}", serde_json::to_string_pretty(&json_res).unwrap());
    } else {
        println!("✅ Issue created successfully: {} (ID: {})", issue_key, new_id);
    }
    Ok(())
}

pub async fn move_issue(
    pool: &MySqlPool,
    id_or_key: &str,
    status_id_or_name: &str,
    resolution: Option<&str>,
    as_json: bool,
) -> Result<(), String> {
    let issue_id = resolve_issue_id(pool, id_or_key).await?;

    // Get current issue
    let cur: (i64, String, i64, String, Option<String>) = sqlx::query_as(
        "SELECT i.project_id, i.`key`, i.status_id, i.issue_type, i.resolution FROM issues i WHERE i.id = ?"
    )
    .bind(issue_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Issue not found: {}", e))?;

    let (project_id, key, cur_status_id, issue_type, cur_res) = cur;

    // Resolve target status
    let target_status: (i64, String, String) = if let Ok(sid) = status_id_or_name.parse::<i64>() {
        sqlx::query_as("SELECT id, name, category FROM statuses WHERE id = ? AND project_id = ?")
            .bind(sid)
            .bind(project_id)
            .fetch_one(pool)
            .await
            .map_err(|_| format!("Status ID {} not found in project", sid))?
    } else {
        sqlx::query_as("SELECT id, name, category FROM statuses WHERE (name = ? OR category = ?) AND project_id = ? LIMIT 1")
            .bind(status_id_or_name)
            .bind(status_id_or_name.to_uppercase())
            .bind(project_id)
            .fetch_one(pool)
            .await
            .map_err(|_| format!("Status '{}' not found in project", status_id_or_name))?
    };

    let (new_status_id, new_status_name, new_category) = target_status;

    // Handle resolution
    let (final_res, final_resolved_at) = if new_category == "DONE" {
        let res = resolution
            .map(|s| s.to_string())
            .or(cur_res)
            .unwrap_or_else(|| if issue_type == "BUG" { "Fixed".into() } else { "Done".into() });
        (Some(res), Some(chrono::Utc::now()))
    } else {
        (None, None)
    };

    sqlx::query("UPDATE issues SET status_id = ?, resolution = ?, resolved_at = ? WHERE id = ?")
        .bind(new_status_id)
        .bind(&final_res)
        .bind(final_resolved_at)
        .bind(issue_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Move failed: {}", e))?;

    if as_json {
        let res_json = serde_json::json!({
            "success": true,
            "id": issue_id,
            "key": key,
            "previous_status_id": cur_status_id,
            "new_status_id": new_status_id,
            "new_status_name": new_status_name,
            "resolution": final_res,
        });
        println!("{}", serde_json::to_string_pretty(&res_json).unwrap());
    } else {
        println!(
            "🚀 Issue {} moved to '{}' ({})! Resolution: {}",
            key,
            new_status_name,
            new_category,
            final_res.as_deref().unwrap_or("None")
        );
    }
    Ok(())
}

pub async fn delete_issue(pool: &MySqlPool, id_or_key: &str, as_json: bool) -> Result<(), String> {
    let issue_id = resolve_issue_id(pool, id_or_key).await?;

    let res = sqlx::query("DELETE FROM issues WHERE id = ?")
        .bind(issue_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Delete failed: {}", e))?;

    if res.rows_affected() == 0 {
        return Err(format!("Issue #{} not found or already deleted", issue_id));
    }

    if as_json {
        println!("{}", serde_json::json!({ "success": true, "deleted_id": issue_id }));
    } else {
        println!("🗑️  Issue #{} deleted successfully.", issue_id);
    }
    Ok(())
}
