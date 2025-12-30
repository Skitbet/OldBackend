use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use crate::utils::error::AppError;
use crate::utils::uuid_as_string;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Report {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub report_id: Uuid, // Unique identifier for the report
    #[serde(with = "uuid_as_string")]
    pub creator_id: Uuid, // Represents the UUID of the user who created the report
    #[serde(with = "uuid_as_string")]
    pub target_id: Uuid, // Represents the UUID of the object reported, post, or user
    pub report_type: ReportType, // Type of report (user, post, etc.)
    pub reason: Option<String>, // Reason for the report
    pub status: ReportStatus, // Status of the report (pending, resolved, etc)
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ReportType {
    POST,
    USER,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum ReportStatus {
    PENDING,
    RESOLVED,
}

impl TryFrom<String> for ReportStatus {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "PENDING" => Ok(ReportStatus::PENDING),
            "RESOLVED" => Ok(ReportStatus::RESOLVED),
            _ => Err(AppError::BadRequest("Invalid status value".into())),
        }
    }
}

impl ToString for ReportStatus {
    fn to_string(&self) -> String {
        (
            match self {
                ReportStatus::PENDING => "PENDING",
                ReportStatus::RESOLVED => "RESOLVED",
            }
        ).to_string()
    }
}
