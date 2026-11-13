use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Debug, Serialize)]
pub struct FeatureDiagnostic {
    pub name: String,
    pub status: String,
    pub details: String,
}

pub async fn run_doctor(pool: &MySqlPool, as_json: bool) -> Result<(), String> {
    let mut results: Vec<FeatureDiagnostic> = Vec::new();

    // 1. Database Connection
    let db_ok = sqlx::query("SELECT 1").execute(pool).await.is_ok();
    results.push(FeatureDiagnostic {
        name: "Database Connection (MariaDB)".to_string(),
        status: if db_ok { "PASS".into() } else { "FAIL".into() },
        details: if db_ok { "Active connection established successfully".into() } else { "Cannot query DB".into() },
    });

    // 2. Schema Verification (Tables check)
    let required_tables = [
        "users", "workspaces", "projects", "statuses", "issues",
        "sprints", "components", "versions", "labels", "issue_labels",
        "issue_watchers", "comments", "activity_logs", "notifications",
    ];

    let mut missing_tables = Vec::new();
    for tbl in required_tables {
        let exists: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = DATABASE() AND table_name = ?"
        )
        .bind(tbl)
        .fetch_one(pool)
        .await
        .unwrap_or((0,));

        if exists.0 == 0 {
            missing_tables.push(tbl);
        }
    }

    results.push(FeatureDiagnostic {
        name: "Database Schema Integrity".to_string(),
        status: if missing_tables.is_empty() { "PASS".into() } else { "FAIL".into() },
        details: if missing_tables.is_empty() {
            format!("All {} required core and Jira standardization tables present", required_tables.len())
        } else {
            format!("Missing tables: {:?}", missing_tables)
        },
    });

    // 3. Point 1: Sprints & Backlog
    let sprint_stats: Result<(i64, i64), _> = sqlx::query_as(
        "SELECT COUNT(*), COUNT(CASE WHEN status = 'ACTIVE' THEN 1 END) FROM sprints"
    )
    .fetch_one(pool)
    .await;

    results.push(FeatureDiagnostic {
        name: "Point 1: Scrum Backlog & Sprint Engine".to_string(),
        status: if sprint_stats.is_ok() { "PASS".into() } else { "FAIL".into() },
        details: match sprint_stats {
            Ok((total, active)) => format!("Total sprints: {}, Active sprint(s): {}", total, active),
            Err(e) => format!("Sprint check failed: {}", e),
        },
    });

    // 4. Point 2: Components
    let comp_count: Result<(i64,), _> = sqlx::query_as("SELECT COUNT(*) FROM components").fetch_one(pool).await;
    results.push(FeatureDiagnostic {
        name: "Point 2: Components (Subsystems)".to_string(),
        status: if comp_count.is_ok() { "PASS".into() } else { "FAIL".into() },
        details: match comp_count {
            Ok((c,)) => format!("{} subsystem component(s) defined", c),
            Err(e) => format!("Component check failed: {}", e),
        },
    });

    // 5. Point 3: Versions / Release Hub
    let ver_count: Result<(i64, i64), _> = sqlx::query_as(
        "SELECT COUNT(*), COUNT(CASE WHEN status = 'RELEASED' THEN 1 END) FROM versions"
    )
    .fetch_one(pool)
    .await;

    results.push(FeatureDiagnostic {
        name: "Point 3: Versions / Release Hub".to_string(),
        status: if ver_count.is_ok() { "PASS".into() } else { "FAIL".into() },
        details: match ver_count {
            Ok((total, released)) => format!("Total versions: {}, Released: {}", total, released),
            Err(e) => format!("Version check failed: {}", e),
        },
    });

    // 6. Point 4: Board Swimlanes Dimensions
    let swimlane_dimensions: Result<(i64, i64, i64), _> = sqlx::query_as(
        "SELECT \
            COALESCE(COUNT(DISTINCT assignee_id), 0), \
            COALESCE(COUNT(DISTINCT epic_id), 0), \
            COALESCE(COUNT(DISTINCT priority), 0) \
         FROM issues"
    )
    .fetch_one(pool)
    .await;

    results.push(FeatureDiagnostic {
        name: "Point 4: Board Swimlanes Data Ready".to_string(),
        status: if swimlane_dimensions.is_ok() { "PASS".into() } else { "FAIL".into() },
        details: match swimlane_dimensions {
            Ok((users, epics, priorities)) => {
                format!("Active dimensions - Assignees: {}, Epics: {}, Priorities: {}", users, epics, priorities)
            }
            Err(e) => format!("Swimlanes check failed: {}", e),
        },
    });

    // 7. Point 5: Issue Labels
    let label_count: Result<(i64, i64), _> = sqlx::query_as(
        "SELECT COUNT(*), (SELECT COUNT(*) FROM issue_labels) FROM labels"
    )
    .fetch_one(pool)
    .await;

    results.push(FeatureDiagnostic {
        name: "Point 5: Issue Labels & Ad-hoc Tagging".to_string(),
        status: if label_count.is_ok() { "PASS".into() } else { "FAIL".into() },
        details: match label_count {
            Ok((total_labels, attached)) => format!("Total unique labels: {}, Attached to issues: {}", total_labels, attached),
            Err(e) => format!("Label check failed: {}", e),
        },
    });

    // 8. Point 6: Issue Resolution & Quality Audit
    let res_count: Result<(i64, i64), _> = sqlx::query_as(
        "SELECT \
            COUNT(CASE WHEN resolution IS NOT NULL THEN 1 END), \
            COUNT(CASE WHEN s.category = 'DONE' THEN 1 END) \
         FROM issues i JOIN statuses s ON i.status_id = s.id"
    )
    .fetch_one(pool)
    .await;

    results.push(FeatureDiagnostic {
        name: "Point 6: Issue Resolution (Audit Readiness)".to_string(),
        status: if res_count.is_ok() { "PASS".into() } else { "FAIL".into() },
        details: match res_count {
            Ok((resolved, done_issues)) => format!("Issues with resolution: {}, Issues in DONE status: {}", resolved, done_issues),
            Err(e) => format!("Resolution check failed: {}", e),
        },
    });

    // 9. Point 7: Quick Filters JQL Performance
    let qf_test: Result<(i64, i64, i64), _> = sqlx::query_as(
        "SELECT \
            COUNT(CASE WHEN issue_type = 'BUG' THEN 1 END), \
            COUNT(CASE WHEN priority IN ('HIGH', 'HIGHEST') THEN 1 END), \
            COUNT(CASE WHEN assignee_id IS NULL THEN 1 END) \
         FROM issues"
    )
    .fetch_one(pool)
    .await;

    results.push(FeatureDiagnostic {
        name: "Point 7: Quick Filter Dimensions Ready".to_string(),
        status: if qf_test.is_ok() { "PASS".into() } else { "FAIL".into() },
        details: match qf_test {
            Ok((bugs, high_prio, unassigned)) => {
                format!("Bugs: {}, High/Highest: {}, Unassigned: {}", bugs, high_prio, unassigned)
            }
            Err(e) => format!("Quick filter check failed: {}", e),
        },
    });

    // 10. Point 8: Issue Watchers
    let watcher_stats: Result<(i64, i64), _> = sqlx::query_as(
        "SELECT COUNT(*), COUNT(DISTINCT issue_id) FROM issue_watchers"
    )
    .fetch_one(pool)
    .await;

    results.push(FeatureDiagnostic {
        name: "Point 8: Issue Watchers (Collaborator Observers)".to_string(),
        status: if watcher_stats.is_ok() { "PASS".into() } else { "FAIL".into() },
        details: match watcher_stats {
            Ok((total_watching, issues_watched)) => {
                format!("Active watch subscriptions: {}, Issues currently watched: {}", total_watching, issues_watched)
            }
            Err(e) => format!("Watchers check failed: {}", e),
        },
    });

    if as_json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
        return Ok(());
    }

    println!("\n🩺 EpicTask System & Jira Enterprise Standards Diagnostic");
    println!("{}", "═".repeat(88));
    println!("{:<48} {:<10} {}", "FEATURE / SUBSYSTEM", "STATUS", "DETAILS");
    println!("{}", "─".repeat(88));

    let all_passed = results.iter().all(|r| r.status == "PASS");

    for r in results {
        let badge = if r.status == "PASS" { "✅ PASS" } else { "❌ FAIL" };
        println!("{:<48} {:<10} {}", r.name, badge, r.details);
    }

    println!("{}", "═".repeat(88));
    if all_passed {
        println!("🎉 All features and database schemas are HEALTHY and verified!");
    } else {
        println!("⚠️  Some diagnostics reported failures. Please check the details above.");
    }
    println!();
    Ok(())
}
