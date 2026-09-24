use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Utc};
use serde_json::json;

use crate::AppState;

pub async fn seed_database(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    run_seed(&state.db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Seed failed: {}", e) })),
        )
    })?;

    Ok(Json(json!({ "message": "Database seeded with rich demo data!" })))
}

pub async fn run_seed(pool: &sqlx::MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    // Check if users already exist
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users").fetch_one(pool).await?;
    if count.0 > 0 {
        return Ok(());
    }

    let default_pass = hash("password123", DEFAULT_COST)?;

    // 1. Insert Demo Users
    let users_data = vec![
        ("sarah@epictask.dev", "Sarah Jenkins (Tech Lead)", "https://api.dicebear.com/7.x/avataaars/svg?seed=sarah"),
        ("alex@epictask.dev", "Alex Morgan (Senior Fullstack)", "https://api.dicebear.com/7.x/avataaars/svg?seed=alex"),
        ("elena@epictask.dev", "Elena Rostova (QA Specialist)", "https://api.dicebear.com/7.x/avataaars/svg?seed=elena"),
        ("david@epictask.dev", "David Chen (Product Owner)", "https://api.dicebear.com/7.x/avataaars/svg?seed=david"),
    ];

    let mut user_ids = Vec::new();
    for (email, name, avatar) in users_data {
        let r = sqlx::query(
            "INSERT INTO users (email, password_hash, full_name, avatar_url) VALUES (?, ?, ?, ?)"
        )
        .bind(email)
        .bind(&default_pass)
        .bind(name)
        .bind(avatar)
        .execute(pool)
        .await?;
        user_ids.push(r.last_insert_id() as i64);
    }

    let sarah_id = user_ids[0];
    let alex_id = user_ids[1];
    let elena_id = user_ids[2];
    let david_id = user_ids[3];

    // 2. Insert Workspace
    let ws_res = sqlx::query("INSERT INTO workspaces (name) VALUES ('Core Engineering Squad')")
        .execute(pool)
        .await?;
    let ws_id = ws_res.last_insert_id() as i64;

    // Add members with roles
    let _ = sqlx::query("INSERT INTO workspace_members (workspace_id, user_id, role) VALUES (?, ?, 'OWNER')")
        .bind(ws_id).bind(sarah_id).execute(pool).await?;
    let _ = sqlx::query("INSERT INTO workspace_members (workspace_id, user_id, role) VALUES (?, ?, 'ADMIN')")
        .bind(ws_id).bind(david_id).execute(pool).await?;
    let _ = sqlx::query("INSERT INTO workspace_members (workspace_id, user_id, role) VALUES (?, ?, 'MEMBER')")
        .bind(ws_id).bind(alex_id).execute(pool).await?;
    let _ = sqlx::query("INSERT INTO workspace_members (workspace_id, user_id, role) VALUES (?, ?, 'MEMBER')")
        .bind(ws_id).bind(elena_id).execute(pool).await?;

    // 3. Insert Project
    let proj_res = sqlx::query(
        "INSERT INTO projects (workspace_id, name, `key`, project_type, current_issue_counter) \
         VALUES (?, 'EpicTask Platform', 'EPIC', 'SCRUM', 8)"
    )
    .bind(ws_id)
    .execute(pool)
    .await?;
    let proj_id = proj_res.last_insert_id() as i64;

    // 4. Insert Statuses
    let statuses_data = vec![
        ("Backlog", "TODO", 0, "#64748b"),
        ("To Do", "TODO", 1, "#3b82f6"),
        ("In Progress", "IN_PROGRESS", 2, "#f59e0b"),
        ("In Review", "IN_PROGRESS", 3, "#8b5cf6"),
        ("Done", "DONE", 4, "#10b981"),
    ];

    let mut status_ids = Vec::new();
    for (name, cat, pos, col) in statuses_data {
        let sr = sqlx::query(
            "INSERT INTO statuses (project_id, name, category, position, color) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(proj_id).bind(name).bind(cat).bind(pos).bind(col)
        .execute(pool)
        .await?;
        status_ids.push(sr.last_insert_id() as i64);
    }

    let s_backlog = status_ids[0];
    let s_todo = status_ids[1];
    let s_in_progress = status_ids[2];
    let s_in_review = status_ids[3];
    let s_done = status_ids[4];

    // 5. Insert Workflow Transition Rules (FR-2.3)
    let transitions = vec![
        (s_backlog, s_todo),
        (s_todo, s_in_progress),
        (s_in_progress, s_in_review),
        (s_in_review, s_in_progress),
        (s_in_review, s_done),
        (s_done, s_todo), // Reopen
    ];

    for (from_id, to_id) in transitions {
        let _ = sqlx::query(
            "INSERT INTO workflow_transitions (project_id, from_status_id, to_status_id) VALUES (?, ?, ?)"
        )
        .bind(proj_id).bind(from_id).bind(to_id)
        .execute(pool)
        .await?;
    }

    // 6. Insert Automation Rules (EPIC 5)
    let _ = sqlx::query(
        "INSERT INTO automation_rules (project_id, name, trigger_type, trigger_config, action_type, action_config, is_active) \
         VALUES (?, 'Auto Complete Subtasks', 'STATUS_CHANGED', '{\"category\":\"DONE\"}', 'CASCADE_SUBTASKS_DONE', '{}', TRUE)"
    )
    .bind(proj_id)
    .execute(pool)
    .await?;

    let _ = sqlx::query(
        "INSERT INTO automation_rules (project_id, name, trigger_type, trigger_config, action_type, action_config, is_active) \
         VALUES (?, 'Reassign to Reporter on Done', 'STATUS_CHANGED', '{\"category\":\"DONE\"}', 'ASSIGN_TO_REPORTER', '{}', TRUE)"
    )
    .bind(proj_id)
    .execute(pool)
    .await?;

    let _ = sqlx::query(
        "INSERT INTO automation_rules (project_id, name, trigger_type, trigger_config, action_type, action_config, is_active) \
         VALUES (?, '24h Due Date Alert', 'DUE_DATE_NEAR', '{\"hours\":24}', 'NOTIFY_ASSIGNEE', '{}', TRUE)"
    )
    .bind(proj_id)
    .execute(pool)
    .await?;

    // 7. Insert Issues (Epics, Stories, Tasks, Subtasks)
    let now = Utc::now().naive_utc().date();
    let d_minus_2 = now - Duration::days(2);
    let d_plus_3 = now + Duration::days(3);
    let d_plus_7 = now + Duration::days(7);
    let d_plus_14 = now + Duration::days(14);

    // Epic 1
    let r1 = sqlx::query(
        "INSERT INTO issues (project_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, start_date, due_date, position) \
         VALUES (?, 'EPIC-1', 'Real-time Multi-View Workspace Board', 'Develop high-performance Kanban, Timeline/Gantt, and List views with WebSocket synchronization.', 'EPIC', ?, 'HIGH', ?, ?, 13, ?, ?, 0)"
    )
    .bind(proj_id).bind(s_in_progress).bind(sarah_id).bind(david_id).bind(d_minus_2).bind(d_plus_14)
    .execute(pool).await?;
    let epic1_id = r1.last_insert_id() as i64;

    // Task 2 (Parent)
    let r2 = sqlx::query(
        "INSERT INTO issues (project_id, epic_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, start_date, due_date, position) \
         VALUES (?, ?, 'EPIC-2', 'Interactive Kanban Drag and Drop with Transition Guard', 'Support smooth card dragging between status columns with validation against defined workflow transition rules.', 'STORY', ?, 'HIGHEST', ?, ?, 8, ?, ?, 0)"
    )
    .bind(proj_id).bind(epic1_id).bind(s_in_progress).bind(alex_id).bind(sarah_id).bind(now).bind(d_plus_3)
    .execute(pool).await?;
    let parent_task_id = r2.last_insert_id() as i64;

    // Subtask 3 under Task 2 (Done)
    let _ = sqlx::query(
        "INSERT INTO issues (project_id, parent_id, epic_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, start_date, due_date, position) \
         VALUES (?, ?, ?, 'EPIC-3', 'Design column WIP headers and drop indicator', 'Visual feedback during dragging with subtle glow and placeholder outline.', 'SUBTASK', ?, 'MEDIUM', ?, ?, 2, ?, ?, 0)"
    )
    .bind(proj_id).bind(parent_task_id).bind(epic1_id).bind(s_done).bind(alex_id).bind(alex_id).bind(d_minus_2).bind(now)
    .execute(pool).await?;

    // Subtask 4 under Task 2 (In Progress)
    let _ = sqlx::query(
        "INSERT INTO issues (project_id, parent_id, epic_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, start_date, due_date, position) \
         VALUES (?, ?, ?, 'EPIC-4', 'Implement Axum WebSocket broadcast payload', 'Real-time state broadcast with optimistic frontend updates.', 'SUBTASK', ?, 'HIGH', ?, ?, 3, ?, ?, 1)"
    )
    .bind(proj_id).bind(parent_task_id).bind(epic1_id).bind(s_in_progress).bind(alex_id).bind(sarah_id).bind(now).bind(d_plus_3)
    .execute(pool).await?;

    // Task 5 (Timeline Gantt)
    let _ = sqlx::query(
        "INSERT INTO issues (project_id, epic_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, start_date, due_date, position) \
         VALUES (?, ?, 'EPIC-5', 'Timeline / Gantt View with Dependency Connectors', 'Horizontal interactive schedule view displaying start and due dates with milestone markers.', 'TASK', ?, 'MEDIUM', ?, ?, 5, ?, ?, 0)"
    )
    .bind(proj_id).bind(epic1_id).bind(s_todo).bind(elena_id).bind(david_id).bind(d_plus_3).bind(d_plus_7)
    .execute(pool).await?;

    // Task 6 (Atomic Issue Generator)
    let _ = sqlx::query(
        "INSERT INTO issues (project_id, epic_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, start_date, due_date, position) \
         VALUES (?, ?, 'EPIC-6', 'Atomic Concurrency Key Generator (SELECT FOR UPDATE)', 'Prevent ticket numbering race conditions across concurrent multi-user environments.', 'TASK', ?, 'HIGHEST', ?, ?, 5, ?, ?, 0)"
    )
    .bind(proj_id).bind(epic1_id).bind(s_done).bind(sarah_id).bind(sarah_id).bind(d_minus_2).bind(now)
    .execute(pool).await?;

    // Task 7 (No-code automation engine)
    let _ = sqlx::query(
        "INSERT INTO issues (project_id, epic_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, start_date, due_date, position) \
         VALUES (?, ?, 'EPIC-7', 'Visual IF-THEN Automation Rule Builder', 'Interactive modal and diagram to configure triggers (Status, Due Date) and actions (Cascade Done, Reassign).', 'STORY', ?, 'HIGH', ?, ?, 5, ?, ?, 0)"
    )
    .bind(proj_id).bind(epic1_id).bind(s_in_review).bind(alex_id).bind(david_id).bind(now).bind(d_plus_3)
    .execute(pool).await?;

    // Task 8 (List view spreadsheet)
    let _ = sqlx::query(
        "INSERT INTO issues (project_id, epic_id, `key`, summary, description, issue_type, status_id, priority, assignee_id, reporter_id, story_points, start_date, due_date, position) \
         VALUES (?, ?, 'EPIC-8', 'Tabular Spreadsheet List View with Quick Edit', 'Table view for fast multi-issue editing of summary, priority, assignee, and dates.', 'TASK', ?, 'LOW', ?, ?, 3, ?, ?, 0)"
    )
    .bind(proj_id).bind(epic1_id).bind(s_backlog).bind(elena_id).bind(sarah_id).bind(d_plus_7).bind(d_plus_14)
    .execute(pool).await?;

    // 8. Time logs
    let _ = sqlx::query(
        "INSERT INTO time_logs (issue_id, user_id, time_spent_seconds, description) VALUES (?, ?, 12600, 'Implemented DnD drop zones and card state handling')"
    )
    .bind(parent_task_id).bind(alex_id).execute(pool).await?;

    // 9. Comments & Mentions
    let _ = sqlx::query(
        "INSERT INTO comments (issue_id, user_id, body) VALUES (?, ?, 'Initial draggable mechanics are ready. @sarah could you review the transition validation endpoint?')"
    )
    .bind(parent_task_id).bind(alex_id).execute(pool).await?;

    let _ = sqlx::query(
        "INSERT INTO comments (issue_id, user_id, body) VALUES (?, ?, 'Looks great @alex! Make sure we test moving directly from Backlog to Done to confirm the rule guard rejects it.')"
    )
    .bind(parent_task_id).bind(sarah_id).execute(pool).await?;

    // 10. Notifications
    let _ = sqlx::query(
        "INSERT INTO notifications (recipient_id, sender_id, issue_id, title, message, action_type) \
         VALUES (?, ?, ?, 'Mentioned you in a comment', 'Alex mentioned you in EPIC-2', 'MENTION')"
    )
    .bind(sarah_id).bind(alex_id).bind(parent_task_id).execute(pool).await?;

    // 11. Activity logs
    let _ = sqlx::query(
        "INSERT INTO activity_logs (project_id, issue_id, user_id, action, details) VALUES (?, ?, ?, 'CREATE_ISSUE', 'Created ticket EPIC-2')"
    )
    .bind(proj_id).bind(parent_task_id).bind(sarah_id).execute(pool).await?;

    let _ = sqlx::query(
        "INSERT INTO activity_logs (project_id, issue_id, user_id, action, details) VALUES (?, ?, ?, 'MOVE_ISSUE', 'Moved EPIC-2 to In Progress')"
    )
    .bind(proj_id).bind(parent_task_id).bind(alex_id).execute(pool).await?;

    println!("Demo seed data inserted successfully!");
    Ok(())
}
