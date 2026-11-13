use chrono::{Duration, NaiveDate, Utc};
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

use super::projects::resolve_project_id;

#[derive(Debug, Serialize, FromRow)]
pub struct SprintCliRow {
    pub id: i64,
    pub name: String,
    pub goal: Option<String>,
    pub status: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub issue_count: i64,
    pub total_points: i64,
    pub todo_points: i64,
    pub in_progress_points: i64,
    pub done_points: i64,
}

pub async fn list_sprints(pool: &MySqlPool, project_id_or_key: &str, as_json: bool) -> Result<(), String> {
    let project_id = resolve_project_id(pool, project_id_or_key).await?;

    let sql = "SELECT sp.id, sp.name, sp.goal, sp.status, sp.start_date, sp.end_date, \
                      COUNT(i.id) AS issue_count, \
                      CAST(COALESCE(SUM(i.story_points), 0) AS SIGNED) AS total_points, \
                      CAST(COALESCE(SUM(CASE WHEN st.category = 'TODO' THEN COALESCE(i.story_points, 0) ELSE 0 END), 0) AS SIGNED) AS todo_points, \
                      CAST(COALESCE(SUM(CASE WHEN st.category = 'IN_PROGRESS' THEN COALESCE(i.story_points, 0) ELSE 0 END), 0) AS SIGNED) AS in_progress_points, \
                      CAST(COALESCE(SUM(CASE WHEN st.category = 'DONE' THEN COALESCE(i.story_points, 0) ELSE 0 END), 0) AS SIGNED) AS done_points \
               FROM sprints sp \
               LEFT JOIN issues i ON i.sprint_id = sp.id \
               LEFT JOIN statuses st ON i.status_id = st.id \
               WHERE sp.project_id = ? \
               GROUP BY sp.id \
               ORDER BY CASE sp.status WHEN 'ACTIVE' THEN 1 WHEN 'PLANNED' THEN 2 ELSE 3 END, sp.id DESC";

    let sprints: Vec<SprintCliRow> = sqlx::query_as(sql)
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&sprints).unwrap());
        return Ok(());
    }

    println!("\n🏃 Scrum Sprints for Project #{} (Total: {})", project_id, sprints.len());
    println!("{:-<110}", "");
    println!("{:<6} {:<24} {:<12} {:<22} {:<8} {:<25}", "ID", "SPRINT NAME", "STATUS", "DATES", "ISSUES", "POINTS (TODO/PROG/DONE)");
    println!("{:-<110}", "");

    for s in sprints {
        let status_badge = match s.status.as_str() {
            "ACTIVE" => "🟢 ACTIVE",
            "COMPLETED" => "⚪ COMPLETED",
            _ => "🟡 PLANNED",
        };
        let dates = match (s.start_date, s.end_date) {
            (Some(st), Some(en)) => format!("{} -> {}", st, en),
            _ => "-".to_string(),
        };
        let points_pill = format!(
            "{} pts [{} / {} / {}]",
            s.total_points, s.todo_points, s.in_progress_points, s.done_points
        );
        println!(
            "{:<6} {:<24} {:<12} {:<22} {:<8} {:<25}",
            s.id,
            if s.name.len() > 22 { format!("{}...", &s.name[..19]) } else { s.name },
            status_badge,
            dates,
            s.issue_count,
            points_pill
        );
    }
    println!("{:-<110}\n", "");
    Ok(())
}

pub async fn create_sprint(
    pool: &MySqlPool,
    project_id_or_key: &str,
    name: &str,
    goal: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
    as_json: bool,
) -> Result<(), String> {
    let project_id = resolve_project_id(pool, project_id_or_key).await?;

    let parsed_start = start_date.and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
    let parsed_end = end_date.and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let res = sqlx::query(
        "INSERT INTO sprints (project_id, name, goal, status, start_date, end_date) \
         VALUES (?, ?, ?, 'PLANNED', ?, ?)"
    )
    .bind(project_id)
    .bind(name)
    .bind(goal)
    .bind(parsed_start)
    .bind(parsed_end)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create sprint: {}", e))?;

    let new_id = res.last_insert_id() as i64;

    if as_json {
        println!("{}", serde_json::json!({ "success": true, "id": new_id, "name": name }));
    } else {
        println!("✅ Sprint '{}' created successfully (ID: {})", name, new_id);
    }
    Ok(())
}

pub async fn start_sprint(
    pool: &MySqlPool,
    sprint_id: i64,
    duration_weeks: Option<i64>,
    goal: Option<&str>,
    as_json: bool,
) -> Result<(), String> {
    let sprint: (i64, String, String) = sqlx::query_as("SELECT project_id, name, status FROM sprints WHERE id = ?")
        .bind(sprint_id)
        .fetch_one(pool)
        .await
        .map_err(|_| format!("Sprint #{} not found", sprint_id))?;

    if sprint.2 == "ACTIVE" {
        return Err(format!("Sprint '{}' is already active", sprint.1));
    }

    let today = Utc::now().date_naive();
    let weeks = duration_weeks.unwrap_or(2);
    let end_date = today + Duration::weeks(weeks);

    sqlx::query("UPDATE sprints SET status = 'ACTIVE', start_date = ?, end_date = ?, goal = COALESCE(?, goal) WHERE id = ?")
        .bind(today)
        .bind(end_date)
        .bind(goal)
        .bind(sprint_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to start sprint: {}", e))?;

    if as_json {
        println!("{}", serde_json::json!({
            "success": true,
            "id": sprint_id,
            "status": "ACTIVE",
            "start_date": today.to_string(),
            "end_date": end_date.to_string(),
        }));
    } else {
        println!("🚀 Sprint '{}' (ID: {}) started! Duration: {} weeks (until {})", sprint.1, sprint_id, weeks, end_date);
    }
    Ok(())
}

pub async fn complete_sprint(
    pool: &MySqlPool,
    sprint_id: i64,
    target_sprint_id: Option<i64>,
    as_json: bool,
) -> Result<(), String> {
    let sprint: (i64, String) = sqlx::query_as("SELECT project_id, name FROM sprints WHERE id = ?")
        .bind(sprint_id)
        .fetch_one(pool)
        .await
        .map_err(|_| format!("Sprint #{} not found", sprint_id))?;

    // Mark sprint COMPLETED
    sqlx::query("UPDATE sprints SET status = 'COMPLETED', end_date = ? WHERE id = ?")
        .bind(Utc::now().date_naive())
        .bind(sprint_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to complete sprint: {}", e))?;

    // Move incomplete issues
    let moved_issues = if let Some(target_id) = target_sprint_id {
        sqlx::query(
            "UPDATE issues i JOIN statuses s ON i.status_id = s.id \
             SET i.sprint_id = ? \
             WHERE i.sprint_id = ? AND s.category != 'DONE'"
        )
        .bind(target_id)
        .bind(sprint_id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected())
        .unwrap_or(0)
    } else {
        sqlx::query(
            "UPDATE issues i JOIN statuses s ON i.status_id = s.id \
             SET i.sprint_id = NULL \
             WHERE i.sprint_id = ? AND s.category != 'DONE'"
        )
        .bind(sprint_id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected())
        .unwrap_or(0)
    };

    if as_json {
        println!("{}", serde_json::json!({
            "success": true,
            "id": sprint_id,
            "status": "COMPLETED",
            "incomplete_issues_moved": moved_issues,
            "target_sprint_id": target_sprint_id,
        }));
    } else {
        let dest = target_sprint_id.map(|id| format!("Sprint #{}", id)).unwrap_or_else(|| "Backlog".to_string());
        println!("🏁 Sprint '{}' completed! {} incomplete issues moved to {}.", sprint.1, moved_issues, dest);
    }
    Ok(())
}
