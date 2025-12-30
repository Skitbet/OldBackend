use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PasswordResetConfirm {
    pub email: String,
    pub code: String,
    pub new_password: String,
}
