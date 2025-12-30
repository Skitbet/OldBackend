use actix_web::{ http::StatusCode, HttpResponse, ResponseError };
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

/// Standard error response payload returned by the API.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

/// Centralized application error enum for consistent error handling.
#[derive(Debug, Error, ToSchema)]
pub enum AppError {
    // Client errors
    #[error("{0}")] BadRequest(String),

    #[error("{0}")] Unauthorized(String),

    #[error("Media file exceeds {0} MB limit")] FileToBig(String),

    // Resource errors
    #[error("Session not found")]
    SessionNotFound,

    #[error("User not found")]
    UserNotFound,

    #[error("Profile not found")]
    ProfileNotFound,

    #[error("Comment was not found")]
    CommentNotFound,

    #[error("Post was not found")]
    PostNotFound,

    #[error("Report was not found")]
    ReportNotFound,

    // Internal errors
    #[error("Internal server error: {0}")] InternalServerError(String),

    #[error("DB Error")]
    DBError,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            // 400 - Bad Request
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,

            // 401 - Unauthorized
            AppError::Unauthorized(_) | AppError::SessionNotFound => StatusCode::UNAUTHORIZED,

            // 404 - Not Found
            | AppError::UserNotFound
            | AppError::ProfileNotFound
            | AppError::CommentNotFound
            | AppError::PostNotFound
            | AppError::ReportNotFound => StatusCode::NOT_FOUND,

            // 413 - Payload Too Large
            AppError::FileToBig(_) => StatusCode::PAYLOAD_TOO_LARGE,

            // 500 - Internal Server Error
            AppError::InternalServerError(_) | AppError::DBError =>
                StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let response = ErrorResponse {
            code: self.status_code().as_u16(),
            message: self.to_string(),
        };
        HttpResponse::build(self.status_code()).json(response)
    }
}
