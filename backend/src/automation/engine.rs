use sqlx::MySqlPool;
use tracing::{error, info};

use crate::models::automation::AutomationRule;
use crate::ws::{WsBroadcastMessage, WsHub};

pub struct AutomationEngine;

impl AutomationEngine {
    pub async fn on_status_changed(
        pool: &MySqlPool,
        ws_hub: &WsHub,
        project_id: i64,
        issue_id: i64,
        new_status_category: &str,
        user_id: Option<i64>,
    ) {
        // Find active rules for project
        let rules = match sqlx::query_as::<_, AutomationRule>(
            "SELECT id, project_id, name, trigger_type, trigger_config, action_type, action_config, is_active, created_at \
             FROM automation_rules \
             WHERE project_id = ? AND is_active = TRUE AND trigger_type = 'STATUS_CHANGED'"
        )
        .bind(project_id)
        .fetch_all(pool)
        .await {
            Ok(r) => r,
            Err(e) => {
                error!("Error fetching automation rules: {}", e);
                return;
            }
        };

        for rule in rules {
            let target_category = rule.trigger_config.get("category")
                .and_then(|v| v.as_str())
                .unwrap_or("DONE");

            if target_category.eq_ignore_ascii_case(new_status_category) {
                info!("Executing automation rule '{}' ({}) for issue {}", rule.name, rule.action_type, issue_id);

                match rule.action_type.as_str() {
                    "CASCADE_SUBTASKS_DONE" => {
                        // Find a status in project with category 'DONE'
                        let done_status: Result<(i64,), _> = sqlx::query_as(
                            "SELECT id FROM statuses WHERE project_id = ? AND category = 'DONE' ORDER BY position ASC LIMIT 1"
                        )
                        .bind(project_id)
                        .fetch_one(pool)
                        .await;

                        if let Ok((done_status_id,)) = done_status {
                            let res = sqlx::query(
                                "UPDATE issues SET status_id = ? WHERE parent_id = ? AND status_id != ?"
                            )
                            .bind(done_status_id)
                            .bind(issue_id)
                            .bind(done_status_id)
                            .execute(pool)
                            .await;

                            if let Ok(affected) = res {
                                info!("Cascade subtasks: {} subtasks updated to Done", affected.rows_affected());
                                // Log activity
                                let _ = sqlx::query(
                                    "INSERT INTO activity_logs (project_id, issue_id, user_id, action, details) \
                                     VALUES (?, ?, ?, 'AUTOMATION', ?)"
                                )
                                .bind(project_id)
                                .bind(issue_id)
                                .bind(user_id)
                                .bind(format!("Automation rule '{}' marked {} subtasks as Done", rule.name, affected.rows_affected()))
                                .execute(pool)
                                .await;

                                // Broadcast board update
                                ws_hub.broadcast(WsBroadcastMessage {
                                    event: "AUTOMATION_TRIGGERED".to_string(),
                                    project_id: Some(project_id),
                                    user_id,
                                    data: serde_json::json!({
                                        "rule_name": rule.name,
                                        "parent_issue_id": issue_id,
                                        "action": "CASCADE_SUBTASKS_DONE",
                                        "rows_affected": affected.rows_affected(),
                                    }),
                                });
                            }
                        }
                    }
                    "ASSIGN_TO_REPORTER" => {
                        // Get reporter of this issue
                        let reporter_res: Result<(Option<i64>, String), _> = sqlx::query_as(
                            "SELECT reporter_id, `key` FROM issues WHERE id = ?"
                        )
                        .bind(issue_id)
                        .fetch_one(pool)
                        .await;

                        if let Ok((Some(reporter_id), issue_key)) = reporter_res {
                            let _ = sqlx::query("UPDATE issues SET assignee_id = ? WHERE id = ?")
                                .bind(reporter_id)
                                .bind(issue_id)
                                .execute(pool)
                                .await;

                            // Create notification
                            let _ = sqlx::query(
                                "INSERT INTO notifications (recipient_id, sender_id, issue_id, title, message, action_type) \
                                 VALUES (?, ?, ?, 'Reassigned by Automation', ?, 'AUTOMATION')"
                            )
                            .bind(reporter_id)
                            .bind(user_id)
                            .bind(issue_id)
                            .bind(format!("Ticket {} was marked Done and reassigned to you as reporter by rule '{}'", issue_key, rule.name))
                            .execute(pool)
                            .await;

                            ws_hub.broadcast(WsBroadcastMessage {
                                event: "AUTOMATION_TRIGGERED".to_string(),
                                project_id: Some(project_id),
                                user_id,
                                data: serde_json::json!({
                                    "rule_name": rule.name,
                                    "issue_id": issue_id,
                                    "action": "ASSIGN_TO_REPORTER",
                                    "assignee_id": reporter_id,
                                }),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub async fn check_due_date_alerts(pool: &MySqlPool, ws_hub: &WsHub) {
        // Trigger 2: When Due Date is within 24 hours
        let rules: Vec<AutomationRule> = match sqlx::query_as(
            "SELECT id, project_id, name, trigger_type, trigger_config, action_type, action_config, is_active, created_at \
             FROM automation_rules \
             WHERE is_active = TRUE AND trigger_type = 'DUE_DATE_NEAR'"
        )
        .fetch_all(pool)
        .await {
            Ok(r) => r,
            Err(_) => return,
        };

        for rule in rules {
            // Find open issues due within 24 hours
            let issues: Vec<(i64, String, String, Option<i64>)> = match sqlx::query_as(
                "SELECT i.id, i.key, i.summary, i.assignee_id \
                 FROM issues i \
                 JOIN statuses s ON i.status_id = s.id \
                 WHERE i.project_id = ? AND s.category != 'DONE' \
                   AND i.due_date IS NOT NULL AND i.due_date <= DATE_ADD(CURRENT_DATE(), INTERVAL 1 DAY)"
            )
            .bind(rule.project_id)
            .fetch_all(pool)
            .await {
                Ok(iss) => iss,
                Err(_) => continue,
            };

            for (issue_id, key, summary, assignee_id) in issues {
                if let Some(uid) = assignee_id {
                    // Check if already notified in last 24h
                    let exists: Result<(i64,), _> = sqlx::query_as(
                        "SELECT id FROM notifications \
                         WHERE recipient_id = ? AND issue_id = ? AND action_type = 'DUE_DATE_NEAR' \
                           AND created_at >= DATE_SUB(NOW(), INTERVAL 1 DAY)"
                    )
                    .bind(uid)
                    .bind(issue_id)
                    .fetch_one(pool)
                    .await;

                    if exists.is_err() {
                        let _ = sqlx::query(
                            "INSERT INTO notifications (recipient_id, issue_id, title, message, action_type) \
                             VALUES (?, ?, 'Due Date Approaching', ?, 'DUE_DATE_NEAR')"
                        )
                        .bind(uid)
                        .bind(issue_id)
                        .bind(format!("Ticket {} ({}) is due within 24 hours!", key, summary))
                        .execute(pool)
                        .await;

                        ws_hub.broadcast(WsBroadcastMessage {
                            event: "NOTIFICATION".to_string(),
                            project_id: Some(rule.project_id),
                            user_id: Some(uid),
                            data: serde_json::json!({
                                "issue_id": issue_id,
                                "key": key,
                                "message": format!("Ticket {} is due within 24 hours", key)
                            }),
                        });
                    }
                }
            }
        }
    }
}
