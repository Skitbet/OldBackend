use actix_web::HttpRequest;
use chrono::{Duration, Utc};
use mongodb::bson::{self, doc};

use crate::{
    models::session::Session,
    state::AppState,
    utils::error::AppError,
};

const SLIDING_EXPIRY_SECONDS: i64 = 7 * 24 * 60 * 60; // 7 days

/// Extracts the session token from the `Authorization` header.
///
/// Returns an error if the header is missing, malformed, or does not use the "Bearer" scheme.
pub fn get_session_token_from_header(req: &HttpRequest) -> Result<String, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".into()))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid authorization header".into()))?;

    if !auth_str.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("Invalid authorization scheme".into()));
    }

    Ok(auth_str.trim_start_matches("Bearer ").to_string())
}

/// Validates the session token from the request and applies sliding expiration.
///
/// If the session is expired, it is deleted and an error is returned.
/// Otherwise, its expiration time is refreshed and the session is returned.
pub async fn validate_and_refresh_session(
    token: String,
    state: &AppState,
) -> Result<Session, AppError> {
    let mut session = state.db.sessions.get_by_token(&token).await?;

    if session.expires_at < Utc::now() {
        state.db.sessions.delete_by_token(&session.token).await?;
        return Err(AppError::Unauthorized("Session expired".into()));
    }

    session.expires_at = Utc::now() + Duration::seconds(SLIDING_EXPIRY_SECONDS);

    let update_result = state.db.sessions.coll
        .update_one(
            doc! { "token": &token },
            doc! {
                "$set": {
                    "expires_at": bson::DateTime::from_millis(session.expires_at.timestamp_millis())
                }
            },
            None,
        )
        .await;

    if let Err(e) = update_result {
        eprintln!("MongoDB update error: {:?}", e);
        return Err(AppError::InternalServerError("Failed to refresh session expiry".into()));
    }

    Ok(session)
}
