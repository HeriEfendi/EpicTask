use clap::Parser;
use epictask_backend::cli::{run_cli, Cli};
use epictask_backend::db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Attempt loading .env from current directory or backend/ directory
    dotenvy::dotenv().ok();
    dotenvy::from_path("backend/.env").ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://remot:PasW0rd123@127.0.0.1:3307/epictask".to_string());
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "epictask_super_secret_jwt_key_2026".to_string());

    let cli = Cli::parse();

    let pool = match db::create_pool(&database_url).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Failed to connect to database ({}): {}", database_url, e);
            std::process::exit(1);
        }
    };

    if let Err(err) = run_cli(cli, &pool, &jwt_secret).await {
        eprintln!("❌ Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}
