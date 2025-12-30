use actix_web::{ get, patch, web, HttpResponse, Responder };
use log::info;
use serde::Deserialize;
use crate::middleware::admin_guard::AdminGuard;
use crate::models::report::ReportStatus;
use crate::state::AppState;
use crate::utils::error::AppError;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring /api/admin/reports scope");
    cfg.service(
        web::scope("reports").service(update_report_status).service(get_reports).service(get_report)
    );
}

#[derive(Deserialize)]
pub struct UpdateReportStatusRequest {
    pub status: String,
}

#[derive(Deserialize)]
pub struct ReportQuery {
    pub status: Option<String>, // "PENDING", "RESOLVED", or None for all
    pub limit: Option<u64>, // e.g., 20
    pub offset: Option<u64>, // e.g., 0
    pub sort_by: Option<String>, // "created_at" or "updated_at"
    pub sort_order: Option<String>, // "asc" or "desc"
}

#[get("")]
pub async fn get_reports(
    _admin_guard: AdminGuard,
    state: web::Data<AppState>,
    query: web::Query<ReportQuery>
) -> Result<impl Responder, AppError> {
    let status_filter = match query.status.as_deref() {
        Some("PENDING") => Some(ReportStatus::PENDING),
        Some("RESOLVED") => Some(ReportStatus::RESOLVED),
        Some(_) => {
            return Err(AppError::BadRequest("Invalid status filter".into()));
        }
        None => None,
    };

    let limit = query.limit.unwrap_or(20).min(100); // cap limit to 100
    let offset = query.offset.unwrap_or(0);

    let sort_field = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");

    let reports = state.db.reporting.query_reports(
        status_filter,
        limit,
        offset,
        sort_field,
        sort_order
    ).await?;

    Ok(HttpResponse::Ok().json(reports))
}

#[get("/fetch/{report_id}")]
pub async fn get_report(
    _admin_guard: AdminGuard,
    path: web::Path<String>,
    state: web::Data<AppState>
) -> Result<impl Responder, AppError> {
    let report_id = path.into_inner();

    let report = state.db.reporting.fetch_by_id(&report_id).await?;
    Ok(HttpResponse::Ok().json(report))
}

#[patch("/status/{report_id}")]
pub async fn update_report_status(
    _admin_guard: AdminGuard,
    path: web::Path<String>,
    body: web::Json<UpdateReportStatusRequest>,
    state: web::Data<AppState>
) -> Result<impl Responder, AppError> {
    let report_id = path.into_inner();
    let new_status = ReportStatus::try_from(body.status.clone())?;

    let updated = state.db.reporting.update_status(report_id, new_status).await?;

    if updated {
        Ok(HttpResponse::Ok().json(serde_json::json!({ "status": "success" })))
    } else {
        Err(AppError::ReportNotFound)
    }
}
