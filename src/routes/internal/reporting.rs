use crate::middleware::auther::Auther;
use crate::models::report::{ Report, ReportStatus, ReportType };
use crate::state::AppState;
use actix_web::{ post, web, HttpResponse, Responder };
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::utils::error::AppError;

pub fn config(web: &mut web::ServiceConfig) {
    web.service(web::scope("/reporting").service(create_report));
}

#[derive(Deserialize, ToSchema)]
pub struct CreateReportRequest {
    pub target_id: String,
    pub report_type: String,
    pub reason: String,
}

#[post("/new")]
pub async fn create_report(
    auther: Auther,
    state: web::Data<AppState>,
    body: web::Json<CreateReportRequest>
) -> Result<impl Responder, AppError> {
    let session = auther.session;

    let target_id = Uuid::parse_str(&body.target_id).map_err(|_|
        AppError::BadRequest("invalid target id".to_string())
    )?;

    let report_type = match body.report_type.to_uppercase().as_str() {
        "POST" => ReportType::POST,
        "USER" => ReportType::USER,
        _ => {
            return Err(AppError::BadRequest("invalid report type".to_string()));
        }
    };

    let time = chrono::Utc::now();

    let report = Report {
        report_id: Uuid::new_v4(),
        creator_id: session.user_uuid,
        target_id,
        report_type,
        reason: Option::from(body.reason.clone()),
        status: ReportStatus::PENDING,
        created_at: time,
        updated_at: time,
    };

    state.db.reporting.create(&report).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "status": "success", "report": report })))
}
