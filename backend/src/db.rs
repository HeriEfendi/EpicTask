use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::time::Duration;
use tracing::info;

pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
}

pub async fn init_db(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    info!("Verifying database schema...");

    // Create tables if not already created
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS workspaces (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            full_name VARCHAR(100) NOT NULL,
            avatar_url VARCHAR(500) DEFAULT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS workspace_members (
            workspace_id BIGINT NOT NULL,
            user_id BIGINT NOT NULL,
            role VARCHAR(20) NOT NULL DEFAULT 'MEMBER',
            joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (workspace_id, user_id),
            FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS projects (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            workspace_id BIGINT NOT NULL,
            name VARCHAR(100) NOT NULL,
            `key` VARCHAR(10) UNIQUE NOT NULL,
            project_type VARCHAR(20) NOT NULL DEFAULT 'KANBAN',
            current_issue_counter INT DEFAULT 0 NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS statuses (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            project_id BIGINT NOT NULL,
            name VARCHAR(50) NOT NULL,
            category VARCHAR(20) NOT NULL DEFAULT 'TODO',
            position INT NOT NULL DEFAULT 0,
            color VARCHAR(20) DEFAULT '#64748b',
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS workflow_transitions (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            project_id BIGINT NOT NULL,
            from_status_id BIGINT NOT NULL,
            to_status_id BIGINT NOT NULL,
            CONSTRAINT unique_transition UNIQUE(project_id, from_status_id, to_status_id),
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (from_status_id) REFERENCES statuses(id) ON DELETE CASCADE,
            FOREIGN KEY (to_status_id) REFERENCES statuses(id) ON DELETE CASCADE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS issues (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            project_id BIGINT NOT NULL,
            parent_id BIGINT DEFAULT NULL,
            epic_id BIGINT DEFAULT NULL,
            `key` VARCHAR(20) UNIQUE NOT NULL,
            summary VARCHAR(255) NOT NULL,
            description TEXT,
            issue_type VARCHAR(20) NOT NULL DEFAULT 'TASK',
            status_id BIGINT NOT NULL,
            priority VARCHAR(20) DEFAULT 'MEDIUM',
            assignee_id BIGINT DEFAULT NULL,
            reporter_id BIGINT DEFAULT NULL,
            story_points INT DEFAULT 0,
            start_date DATE DEFAULT NULL,
            due_date DATE DEFAULT NULL,
            position INT DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (parent_id) REFERENCES issues(id) ON DELETE SET NULL,
            FOREIGN KEY (epic_id) REFERENCES issues(id) ON DELETE SET NULL,
            FOREIGN KEY (status_id) REFERENCES statuses(id) ON DELETE RESTRICT,
            FOREIGN KEY (assignee_id) REFERENCES users(id) ON DELETE SET NULL,
            FOREIGN KEY (reporter_id) REFERENCES users(id) ON DELETE SET NULL
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS issue_links (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            source_issue_id BIGINT NOT NULL,
            target_issue_id BIGINT NOT NULL,
            link_type VARCHAR(50) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (source_issue_id) REFERENCES issues(id) ON DELETE CASCADE,
            FOREIGN KEY (target_issue_id) REFERENCES issues(id) ON DELETE CASCADE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS time_logs (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            issue_id BIGINT NOT NULL,
            user_id BIGINT DEFAULT NULL,
            time_spent_seconds BIGINT NOT NULL,
            description VARCHAR(255) DEFAULT NULL,
            logged_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS comments (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            issue_id BIGINT NOT NULL,
            user_id BIGINT DEFAULT NULL,
            body TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS notifications (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            recipient_id BIGINT NOT NULL,
            sender_id BIGINT DEFAULT NULL,
            issue_id BIGINT DEFAULT NULL,
            title VARCHAR(255) NOT NULL,
            message TEXT NOT NULL,
            action_type VARCHAR(50) NOT NULL,
            is_read BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (recipient_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (sender_id) REFERENCES users(id) ON DELETE SET NULL,
            FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS activity_logs (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            project_id BIGINT NOT NULL,
            issue_id BIGINT DEFAULT NULL,
            user_id BIGINT DEFAULT NULL,
            action VARCHAR(50) NOT NULL,
            details TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (issue_id) REFERENCES issues(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS automation_rules (
            id BIGINT AUTO_INCREMENT PRIMARY KEY,
            project_id BIGINT NOT NULL,
            name VARCHAR(100) NOT NULL,
            trigger_type VARCHAR(50) NOT NULL,
            trigger_config JSON NOT NULL,
            action_type VARCHAR(50) NOT NULL,
            action_config JSON NOT NULL,
            is_active BOOLEAN DEFAULT TRUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    )
    .execute(pool).await?;

    info!("Database schema verified successfully.");
    Ok(())
}
