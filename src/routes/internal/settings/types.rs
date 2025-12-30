use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct PasswordChangeRequest {
    pub current_password: String,
    pub new_password: String,
}
