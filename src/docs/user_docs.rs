#![allow(dead_code)]

use crate::routes::internal::user::types::{ PasswordResetRequest, PasswordResetConfirm };
use crate::routes::internal::posts::types::QueryPostsParams;

#[utoipa::path(
    get,
    path = "/api/user/{username}/posts",
    params(
        ("username" = String, Path, description = "Username to get posts for"),
        QueryPostsParams
    ),
    responses((status = 200, description = "Posts fetched successfully")),
    tag = "Users"
)]
pub async fn get_user_posts() {}

#[utoipa::path(
    patch,
    path = "/api/user/settings",
    request_body = serde_json::Value,
    responses((status = 200, description = "Settings updated successfully")),
    tag = "Users",
    params(("Authorization" = String, Header, description = "Bearer token for user."))
)]
pub async fn patch_user_settings() {}

#[utoipa::path(
    get,
    path = "/api/user/settings",
    responses((status = 200, description = "User settings fetched successfully")),
    tag = "Users",
    params(("Authorization" = String, Header, description = "Bearer token for user."))
)]
pub async fn get_user_settings() {}

#[utoipa::path(
    get,
    path = "/api/user/verify/{code}",
    params(("code" = String, Path, description = "Verification code")),
    responses((status = 200, description = "User verified successfully")),
    tag = "Users"
)]
pub async fn verify_user_email() {}

#[utoipa::path(
    post,
    path = "/api/user/request_password_reset",
    request_body = PasswordResetRequest,
    responses((status = 200, description = "Password reset request sent")),
    tag = "Users",
    params(("Authorization" = String, Header, description = "Bearer token for user."))
)]
pub async fn request_password_reset() {}

#[utoipa::path(
    post,
    path = "/api/user/reset_password",
    request_body = PasswordResetConfirm,
    responses((status = 200, description = "Password reset successful")),
    tag = "Users",
    params(("Authorization" = String, Header, description = "Bearer token for user."))
)]
pub async fn reset_password() {}
