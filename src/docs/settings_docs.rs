#![allow(dead_code)]

use crate::routes::internal::settings::types::PasswordChangeRequest;

#[utoipa::path(
    post,
    path = "/settings/change_password",
    tag = "Settings",
    summary = "Change user password",
    request_body = PasswordChangeRequest,
    responses(
        (status = 200, description = "Password changed successfully"),
        (status = 400, description = "Invalid input data"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("Authorization" = String, Header, description = "Bearer token for user.")
    ),
)]
pub fn change_password() {}
