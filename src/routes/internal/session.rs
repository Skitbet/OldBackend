use actix_web::{ HttpResponse, Responder, get, post, web::{ self, Data } };
use chrono::Utc;
use log::info;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    auth::{ create_jwt, password::{ hash_password, verify_password } },
    models::{ OkResponse, session::Session, user::User },
    state::AppState,
    utils::{ error::AppError, is_email },
};
use mongodb::bson::{ doc, oid::ObjectId };
use crate::middleware::auther::Auther;
use crate::models::codes::{ Code, CodeType };
use crate::models::user::PreRegisteredUser;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring /api/session scope");
    cfg.service(
        web::scope("session").service(validate).service(login).service(register).service(logout)
    );
}

#[derive(Deserialize, Clone, ToSchema)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginInput {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Deserialize, serde::Serialize, ToSchema)]
pub struct AuthResponse {
    pub username: String,
    pub token: String,
}

#[derive(Deserialize, serde::Serialize, ToSchema)]
pub struct RegisterResponse {
    pub username: String,
}

#[get("/validate")]
pub async fn validate(_auther: Auther) -> Result<impl Responder, AppError> {
    Ok(
        HttpResponse::Ok().json(OkResponse {
            message: "Session valid.".into(),
        })
    )
}

#[post("/register")]
pub async fn register(
    state: Data<AppState>,
    data: web::Json<RegisterInput>
) -> Result<impl Responder, AppError> {
    // check if email or username exists
    if state.db.users.exists(&data.username).await? {
        return Err(AppError::BadRequest("Username has been taken".into()));
    }

    if state.db.users.email_exists(&data.email).await? {
        return Err(AppError::BadRequest("Account has been taken".into()));
    }

    if state.db.pre_user_repo.email_exists(&data.email).await? {
        return Err(AppError::BadRequest("Account has been taken".into()));
    }

    // hash password
    let password_hash = match hash_password(&data.password) {
        Ok(hash) => hash,
        Err(_) => {
            return Err(AppError::InternalServerError("Failed to hash password".into()));
        }
    };

    let username = data.username.to_lowercase();

    let unregistered_user = PreRegisteredUser::new(&data.email, &username, &password_hash);
    state.db.pre_user_repo.create(&unregistered_user).await?;

    let code = Code::new(data.email.clone(), data.username.clone(), CodeType::EmailVerify);
    state.db.codes_repo.create(&code).await?;
    state.smtp_service
        .send_verification_code(&*code.email, &*code.code).await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Ok(HttpResponse::Ok().json(RegisterResponse { username }))
}

#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    data: web::Json<LoginInput>
) -> Result<impl Responder, AppError> {
    // Try to get the user from the primary users collection
    let user_result = if is_email(&data.username_or_email) {
        state.db.users.get_by_email(&data.username_or_email).await
    } else {
        state.db.users.get_by_username(&data.username_or_email).await
    };

    let user = match user_result {
        Ok(user) => user,
        Err(AppError::UserNotFound) => {
            let preuser_result = if is_email(&data.username_or_email) {
                state.db.pre_user_repo.get_by_email(&data.username_or_email).await
            } else {
                state.db.pre_user_repo.get_by_username(&data.username_or_email).await
            };

            return match preuser_result {
                Ok(_) => { Err(AppError::Unauthorized("Pending email verification.".into())) }
                Err(AppError::UserNotFound) => {
                    Err(AppError::Unauthorized("User not found".into()))
                }
                Err(e) => Err(e),
            };
        }
        Err(e) => {
            return Err(e);
        }
    };

    // verify password
    if !verify_password(&data.password, &user.password_hash) {
        return Err(AppError::Unauthorized("Invalid password".into()));
    }

    // create session
    let (_session, token) = create_session(user.clone(), state).await?;

    Ok(
        HttpResponse::Ok().json(AuthResponse {
            username: user.username.clone(),
            token,
        })
    )
}

#[post("/logout")]
pub async fn logout(auther: Auther, state: Data<AppState>) -> Result<impl Responder, AppError> {
    let session = auther.session;

    if state.db.sessions.delete_by_token(&session.token).await? {
        return Err(AppError::Unauthorized("Invalid session".into()));
    }

    Ok(
        HttpResponse::Ok().json(OkResponse {
            message: "Successfully logged out.".into(),
        })
    )
}

pub async fn create_session(
    user: User,
    state: Data<AppState>
) -> Result<(Session, String), AppError> {
    let session_id = Uuid::new_v4();
    let now = Utc::now();
    let expires_at = now + chrono::Duration::seconds(state.jwt_expiration_seconds);
    let token: String = create_jwt(
        &user.id,
        &session_id,
        &state.jwt_secret,
        state.jwt_expiration_seconds
    );

    let session = Session {
        id: ObjectId::new(),
        session_id,
        token: token.clone(),
        user_uuid: user.id,
        created_at: now,
        expires_at,
    };

    state.db.sessions.create(&session).await?;
    Ok((session, token))
}
