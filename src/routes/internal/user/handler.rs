use std::fmt::format;
use std::hint::assert_unchecked;
use actix_web::{get, patch, post, web::{self, Data, Path}, HttpResponse, Responder, ResponseError};
use uuid::Uuid;
use std::str::FromStr;
use actix_web::web::Query;
use bson::doc;
use chrono::Utc;
use serde_json::json;
use crate::auth::password::hash_password;
use crate::middleware::auther::Auther;
use crate::models::codes::{Code, CodeType};
use crate::models::post::PostResponse;
use crate::models::profile::DBProfile;
use crate::models::settings::UserSettings;
use crate::models::user::User;
use crate::routes::internal::posts::types::QueryPostsParams;
use crate::routes::internal::session::{create_session, AuthResponse};
use crate::routes::internal::user::types::{PasswordResetConfirm, PasswordResetRequest};
use crate::state::AppState;
use crate::utils::error::AppError;
use crate::utils::error::AppError::Unauthorized;
use crate::utils::json::json_merge;

#[get("/{username}/posts")]
pub async fn get_user_posts(
    state: Data<AppState>,
    path: Path<String>,
    query: Query<QueryPostsParams>,
) -> impl Responder {
    let username = path.into_inner();

    let limit = query.limit.unwrap_or(20);
    let page = query
        .page
        .as_ref()
        .and_then(|p| p.parse::<u64>().ok())
        .unwrap_or(0);
    let skip = page * limit;

    let tags: Option<Vec<String>> = query.tags.as_ref().map(|tags_str| {
        tags_str
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect()
    });

    let result = state.db.posts
        .get_all_by_user(&*username, limit, skip, tags)
        .await
        .map(|posts| {
            let responses: Vec<_> = posts.iter().map(PostResponse::from).collect();
            HttpResponse::Ok().json(responses)
        })
        .unwrap_or_else(|e| e.error_response());

    result
}

/**
 * Settings
 */
#[patch("/settings")]
pub async fn patch_user_settings(
    auther: Auther,
    state: Data<AppState>,
    patch: web::Json<serde_json::Value>,
) -> Result<impl Responder, AppError> {
    let session = auther.session;
    let user_id = session.user_uuid;

    let current_settings = state.db.settings.get_by_uuid(&user_id).await?;
    let mut current_settings_json = json!(current_settings);

    json_merge(&mut current_settings_json, &patch.into_inner());
    let updated_settings: UserSettings = serde_json::from_value(current_settings_json.clone())
        .map_err(|e| AppError::BadRequest(format!("Invalid settings: {}", e)))?;

    // check settings
    if updated_settings.page_length > 50 {
        return Err(AppError::BadRequest("Page Length cannot exceed 100!".into()))
    }

    state.db.settings.save(&user_id, &updated_settings).await?;
    Ok(HttpResponse::Ok().json(updated_settings))
}

#[get("/settings")]
pub async fn get_user_settings(
    auther: Auther,
    state: Data<AppState>,
) -> Result<impl Responder, AppError> {
    let session = auther.session;
    let user_id = session.user_uuid;

    let current_settings = state.db.settings.get_by_uuid(&user_id).await?;
    Ok(HttpResponse::Ok().json(current_settings))
}

#[get("/verify/{code}")]
pub async fn verify_user_email(
    path: Path<String>,
    state: Data<AppState>
) -> Result<impl Responder, AppError> {
    let code = path.into_inner();

    let db_code = state.db.codes_repo.get_code_by_code(&code).await?;
    if db_code.is_expired() {
        return Err(AppError::BadRequest("Code has expired".into()));
    }
    state.db.codes_repo.delete_code_by_code(&*db_code.code).await?;

    let pre_user = state.db.pre_user_repo.get_by_email(&db_code.email).await?;
    state.db.pre_user_repo.delete(&pre_user).await?;

    let new_user = User::new(&pre_user.id, &pre_user.email, &pre_user.username, &pre_user.password_hash);
    state.db.users.create(&new_user).await?;

    let new_profile = DBProfile::new(&new_user, &new_user.username);
    state.db.profiles.create(&new_profile).await?;

    let new_settings = UserSettings::new_from_user(&new_user);
    state.db.settings.create(&new_settings).await?;

    let (_session, token) = create_session(new_user.clone(), state).await?;

    Ok(HttpResponse::Ok().json(AuthResponse {
        username: new_user.username.clone(),
        token
    }))
}

#[post("/request_password_reset")]
async fn request_password_reset(
    state: Data<AppState>,
    body: web::Json<PasswordResetRequest>,
) -> Result<impl Responder, AppError> {
    let email = body.email.clone();
    
    // check if user exists
    let user = state.db.users.get_by_email(&email).await?;
    
    // generate code
    let code = Code::new(email.clone(), user.username.clone(), CodeType::PasswordReset);
    state.db.codes_repo.create(&code).await?;
    
    // send email
    state.smtp_service.send_password_reset_code(&email, &code.code).await
        .map_err(|e| AppError::InternalServerError(format!("Failed to send email: {e}")))?;
    
    Ok(HttpResponse::Ok().json(json!({ 
        "message": "Password reset email send."
    })))
}

#[post("/reset_password")]
pub async fn reset_password(
    state: Data<AppState>,
    body: web::Json<PasswordResetConfirm>,
) -> Result<impl Responder, AppError> {
    let email = body.email.clone();
    let code_str = body.code.clone();
    let new_password = body.new_password.clone();
    
    // get the code and handle checks
    let db_code = state.db.codes_repo.get_code_by_code(&code_str).await?;
    if db_code.is_expired() || db_code.code_type != CodeType::PasswordReset {
        return Err(AppError::BadRequest("Invalid or expired code".into()));
    }
    if db_code.email != email {
        return Err(AppError::BadRequest("Invalid email".into()));
    }
    
    // update user
    let mut user = state.db.users.get_by_email(&email).await?;
    let hashed_pass = hash_password(&new_password)
        .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {e}")))?;
    
    let mut fields = doc! {};
    fields.insert("password_hash", hashed_pass);
    fields.insert("updated_at", Utc::now());

    state.db.users.update_fields(&user.id, fields).await?;
    state.db.codes_repo.delete_code_by_code(&code_str).await?;
    state.db.sessions.delete_all_for_user(&user.id).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "message": "Password reset successful!"
    })))
}