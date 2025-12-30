#![allow(dead_code)]

use crate::models::{ OkResponse };
use crate::routes::internal::session::{ AuthResponse, LoginInput, RegisterInput, RegisterResponse };
use crate::utils::error::AppError;

#[utoipa::path(
    get,
    path = "/api/session/validate",
    tag = "Session",
    responses(
        (status = 200, description = "Session valid.", body = OkResponse),
        (status = 401, description = "Invalid or expired token.", body = AppError)
    ),
    params((
        "Authorization" = String,
        Header,
        description = "Bearer token for user session validation.",
    ))
)]
pub fn validate_docs() {}

#[utoipa::path(
    post,
    path = "/api/session/register",
    tag = "Session",
    request_body = RegisterInput,
    responses(
        (status = 200, description = "User pre-registered successfully.", body = RegisterResponse),
        (status = 400, description = "Username or email already taken.", body = AppError),
        (status = 500, description = "Internal server error.", body = AppError)
    )
)]
pub fn register_docs() {}

#[utoipa::path(
    post,
    path = "/api/session/login",
    tag = "Session",
    request_body = LoginInput,
    responses(
        (status = 200, description = "Login successful, returns JWT token.", body = AuthResponse),
        (
            status = 401,
            description = "Invalid credentials or pending email verification.",
            body = AppError,
        ),
        (status = 404, description = "User not found.", body = AppError)
    )
)]
pub fn login_docs() {}

#[utoipa::path(
    post,
    path = "/api/session/logout",
    tag = "Session",
    responses(
        (status = 200, description = "Successfully logged out.", body = OkResponse),
        (status = 401, description = "Invalid or expired session.", body = AppError)
    ),
    params(("Authorization" = String, Header, description = "Bearer token for user logout."))
)]
pub fn logout_docs() {}
