use crate::models::report::{ Report, ReportType, ReportStatus };
use crate::routes::internal::reporting::CreateReportRequest;

#[utoipa::path(
    post,
    path = "/api/reporting/new",
    params((
        "Authorization" = String,
        Header,
        description = "Bearer token for authenticated user.",
    )),
    request_body(
        content = CreateReportRequest,
        description = "Data required to create a new report."
    ),
    responses(
        (status = 200, description = "Report created successfully", body = Report),
        (status = 400, description = "Invalid target id or report type"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Reporting"
)]
pub fn create_report() {}
