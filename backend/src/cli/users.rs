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

pub async fn show_user_performance(
    pool: &MySqlPool,
    email_or_id: &str,
    project_id: i64,
    start_date: Option<String>,
    end_date: Option<String>,
    as_json: bool,
) -> Result<(), String> {
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

    let has_date_filter = start_date.is_some() && end_date.is_some();
    let s_date = start_date.clone().unwrap_or_default();
    let e_date = end_date.clone().unwrap_or_default();

    let (stats_sql, time_sql) = if has_date_filter {
        (
            format!(
                "SELECT 
                    CAST(COUNT(*) AS SIGNED) as total_assigned,
                    CAST(COUNT(CASE WHEN s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{s}' AND '{e}' THEN 1 END) AS SIGNED) as completed_count,
                    CAST(COUNT(CASE WHEN s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{s}' AND '{e}' AND (i.due_date IS NULL OR DATE(COALESCE(i.resolved_at, i.updated_at)) <= i.due_date) THEN 1 END) AS SIGNED) as on_time_count,
                    CAST(COUNT(CASE WHEN s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{s}' AND '{e}' AND i.due_date IS NOT NULL AND DATE(COALESCE(i.resolved_at, i.updated_at)) > i.due_date THEN 1 END) AS SIGNED) as overdue_count,
                    CAST(COUNT(CASE WHEN s.category = 'IN_PROGRESS' THEN 1 END) AS SIGNED) as in_progress_count,
                    CAST(COUNT(CASE WHEN s.category = 'TODO' THEN 1 END) AS SIGNED) as todo_count,
                    CAST(COALESCE(SUM(CASE WHEN s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{s}' AND '{e}' THEN i.story_points ELSE 0 END), 0) AS SIGNED) as completed_story_points,
                    CAST(COALESCE(SUM(i.story_points), 0) AS SIGNED) as total_story_points,
                    CAST(COALESCE(AVG(CASE WHEN s.category = 'DONE' AND DATE(COALESCE(i.resolved_at, i.updated_at)) BETWEEN '{s}' AND '{e}' AND i.resolved_at IS NOT NULL THEN TIMESTAMPDIFF(HOUR, i.created_at, i.resolved_at) END), 0) AS DOUBLE) as avg_cycle_hours
                 FROM issues i
                 JOIN statuses s ON i.status_id = s.id
                 WHERE i.project_id = ? AND i.assignee_id = ?",
                s = s_date,
                e = e_date
            ),
            format!(
                "SELECT 
                    CAST(COALESCE(SUM(t.time_spent_seconds), 0) AS SIGNED) as total_logged_seconds,
                    CAST(COUNT(t.id) AS SIGNED) as total_worklogs
                 FROM time_logs t
                 JOIN issues i ON t.issue_id = i.id AND i.project_id = ?
                 WHERE t.user_id = ? AND DATE(t.logged_at) BETWEEN '{s}' AND '{e}'",
                s = s_date,
                e = e_date
            )
        )
    } else {
        (
            "SELECT 
                CAST(COUNT(*) AS SIGNED) as total_assigned,
                CAST(COUNT(CASE WHEN s.category = 'DONE' THEN 1 END) AS SIGNED) as completed_count,
                CAST(COUNT(CASE WHEN s.category = 'DONE' AND (i.due_date IS NULL OR DATE(COALESCE(i.resolved_at, i.updated_at)) <= i.due_date) THEN 1 END) AS SIGNED) as on_time_count,
                CAST(COUNT(CASE WHEN s.category = 'DONE' AND i.due_date IS NOT NULL AND DATE(COALESCE(i.resolved_at, i.updated_at)) > i.due_date THEN 1 END) AS SIGNED) as overdue_count,
                CAST(COUNT(CASE WHEN s.category = 'IN_PROGRESS' THEN 1 END) AS SIGNED) as in_progress_count,
                CAST(COUNT(CASE WHEN s.category = 'TODO' THEN 1 END) AS SIGNED) as todo_count,
                CAST(COALESCE(SUM(CASE WHEN s.category = 'DONE' THEN i.story_points ELSE 0 END), 0) AS SIGNED) as completed_story_points,
                CAST(COALESCE(SUM(i.story_points), 0) AS SIGNED) as total_story_points,
                CAST(COALESCE(AVG(CASE WHEN s.category = 'DONE' AND i.resolved_at IS NOT NULL THEN TIMESTAMPDIFF(HOUR, i.created_at, i.resolved_at) END), 0) AS DOUBLE) as avg_cycle_hours
             FROM issues i
             JOIN statuses s ON i.status_id = s.id
             WHERE i.project_id = ? AND i.assignee_id = ?".to_string(),
            "SELECT 
                CAST(COALESCE(SUM(t.time_spent_seconds), 0) AS SIGNED) as total_logged_seconds,
                CAST(COUNT(t.id) AS SIGNED) as total_worklogs
             FROM time_logs t
             JOIN issues i ON t.issue_id = i.id AND i.project_id = ?
             WHERE t.user_id = ?".to_string()
        )
    };

    use sqlx::Row;
    let s_row = sqlx::query(&stats_sql)
        .bind(project_id)
        .bind(u.id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Stats query failed: {}", e))?;

    let total_assigned: i64 = s_row.try_get("total_assigned").unwrap_or(0);
    let completed_count: i64 = s_row.try_get("completed_count").unwrap_or(0);
    let on_time_count: i64 = s_row.try_get("on_time_count").unwrap_or(0);
    let overdue_count: i64 = s_row.try_get("overdue_count").unwrap_or(0);
    let in_progress_count: i64 = s_row.try_get("in_progress_count").unwrap_or(0);
    let todo_count: i64 = s_row.try_get("todo_count").unwrap_or(0);
    let completed_story_points: i64 = s_row.try_get("completed_story_points").unwrap_or(0);
    let total_story_points: i64 = s_row.try_get("total_story_points").unwrap_or(0);
    let avg_cycle_hours: f64 = s_row.try_get("avg_cycle_hours").unwrap_or(0.0);

    let t_row = sqlx::query(&time_sql)
        .bind(project_id)
        .bind(u.id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Time query failed: {}", e))?;

    let total_logged_seconds: i64 = t_row.try_get("total_logged_seconds").unwrap_or(0);
    let total_worklogs: i64 = t_row.try_get("total_worklogs").unwrap_or(0);
    let total_logged_hours = (total_logged_seconds as f64) / 3600.0;

    let completion_rate = if total_assigned > 0 {
        ((completed_count as f64) / (total_assigned as f64)) * 100.0
    } else {
        0.0
    };

    let on_time_rate = if completed_count > 0 {
        ((on_time_count as f64) / (completed_count as f64)) * 100.0
    } else {
        0.0
    };

    if as_json {
        println!("{}", serde_json::json!({
            "user": {
                "id": u.id,
                "full_name": u.full_name,
                "email": u.email,
            },
            "project_id": project_id,
            "date_filter": {
                "start": start_date,
                "end": end_date,
            },
            "performance_summary": {
                "done_count": completed_count,
                "on_time_count": on_time_count,
                "overdue_count": overdue_count,
                "on_time_rate": on_time_rate,
            },
            "stats": {
                "total_assigned": total_assigned,
                "completed_count": completed_count,
                "on_time_count": on_time_count,
                "overdue_count": overdue_count,
                "on_time_rate": on_time_rate,
                "in_progress_count": in_progress_count,
                "todo_count": todo_count,
                "completion_rate": completion_rate,
                "completed_story_points": completed_story_points,
                "total_story_points": total_story_points,
                "total_logged_hours": total_logged_hours,
                "total_worklogs": total_worklogs,
                "avg_cycle_hours": avg_cycle_hours,
                "avg_cycle_days": avg_cycle_hours / 24.0,
            }
        }));
        return Ok(());
    }

    println!("\n📊 User Performance: {} ({}) [Project #{}]", u.full_name, u.email, project_id);
    if has_date_filter {
        println!("📅 Period: {} to {}", s_date, e_date);
    } else {
        println!("📅 Period: All-time");
    }
    println!("{:-<75}", "");
    println!("  🎯 Performance Summary :");
    println!("     - Total Tiket Done   : {} tiket", completed_count);
    println!("     - Selesai Sesuai Plan: {} tiket", on_time_count);
    println!("     - Terlambat (Overdue): {} tiket", overdue_count);
    println!("     - Ketepatan Waktu    : {:.1}%", on_time_rate);
    println!("{:-<75}", "");
    println!("  📋 Work Status Breakdown: {} assigned, {} active, {} todo ({:.1}% completed)", total_assigned, in_progress_count, todo_count, completion_rate);
    println!("  ⚡ Story Points Done    : {} / {} pts", completed_story_points, total_story_points);
    println!("  ⏱️  Total Time Logged    : {:.1} hours across {} worklogs", total_logged_hours, total_worklogs);
    println!("  🚀 Avg Cycle Velocity   : {:.1} hours ({:.1} days)", avg_cycle_hours, avg_cycle_hours / 24.0);
    println!("{:-<75}\n", "");

    Ok(())
}
