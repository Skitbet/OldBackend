#![allow(dead_code)]

use crate::{ routes::internal::userassets::UpdateUserAssetResponse, utils::error::AppError };
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct FileUploadSchema {
    /// File to upload (binary data)
    #[schema(value_type = String, format = Binary)]
    pub file: String,
}

#[utoipa::path(
    post,
    path = "/api/media/me/assets/profile_picture",
    tag = "User Assets",
    request_body(
        content = FileUploadSchema,
        content_type = "multipart/form-data",
        description = "Multipart form data containing the profile picture file."
    ),
    responses(
        (
            status = 200,
            description = "Profile picture uploaded successfully.",
            body = UpdateUserAssetResponse,
        ),
        (
            status = 400,
            description = "No file data provided or invalid multipart data.",
            body = AppError,
        ),
        (
            status = 401,
            description = "Unauthorized. Missing or invalid Bearer token.",
            body = AppError,
        ),
        (
            status = 500,
            description = "Internal server error while uploading to R2.",
            body = AppError,
        )
    ),
    params(("Authorization" = String, Header, description = "Bearer token for authenticated user."))
)]
pub fn upload_profile_picture() {}

#[utoipa::path(
    post,
    path = "/api/media/me/assets/banner",
    tag = "User Assets",
    request_body(
        content = FileUploadSchema,
        content_type = "multipart/form-data",
        description = "Multipart form data containing the banner picture file."
    ),
    responses(
        (
            status = 200,
            description = "Banner image uploaded successfully.",
            body = UpdateUserAssetResponse,
        ),
        (
            status = 400,
            description = "No file data provided or invalid multipart data.",
            body = AppError,
        ),
        (
            status = 401,
            description = "Unauthorized. Missing or invalid Bearer token.",
            body = AppError,
        ),
        (
            status = 500,
            description = "Internal server error while uploading to R2.",
            body = AppError,
        )
    ),
    params(("Authorization" = String, Header, description = "Bearer token for authenticated user."))
)]
pub fn upload_banner() {}
