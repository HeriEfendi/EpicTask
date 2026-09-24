use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use super::jwt::verify_token;
use crate::config::Config;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AuthUser {
    pub user_id: i64,
    pub email: String,
}

pub struct AuthError(pub StatusCode, pub String);

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.1,
            "status": self.0.as_u16(),
        }));
        (self.0, body).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // First check Authorization header
        let token_opt = if let Some(auth_header) = parts.headers.get(header::AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    Some(token.trim().to_string())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            // Also check query param "token" (for WebSockets)
            parts.uri.query().and_then(|q| {
                q.split('&')
                    .find(|pair| pair.starts_with("token="))
                    .map(|pair| pair.trim_start_matches("token=").to_string())
            })
        };

        let token = token_opt.ok_or_else(|| {
            AuthError(
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization token".to_string(),
            )
        })?;

        let config = Config::from_env();
        let claims = verify_token(&token, &config.jwt_secret).map_err(|_| {
            AuthError(
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            )
        })?;

        let user_id = claims.sub.parse::<i64>().map_err(|_| {
            AuthError(
                StatusCode::UNAUTHORIZED,
                "Malformed user identifier in token".to_string(),
            )
        })?;

        Ok(AuthUser {
            user_id,
            email: claims.email,
        })
    }
}
