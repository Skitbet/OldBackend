use utoipa::ToSchema;

pub mod comment;
pub mod post;
pub mod profile;
pub mod session;
pub mod settings;
pub mod user;
pub mod character;
pub mod codes;
pub mod report;
pub mod media;
pub mod announcement;

#[derive(serde::Serialize, ToSchema)]
pub struct OkResponse {
    pub message: String,
}
