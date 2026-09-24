use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://remot:PasW0rd123@127.0.0.1:3307/epictask".to_string());
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "super_secret_epictask_jwt_token_key_for_dev_and_prod_32chars!".to_string());
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8088);

        Config {
            database_url,
            jwt_secret,
            host,
            port,
        }
    }
}
