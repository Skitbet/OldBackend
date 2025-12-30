use actix_web::{HttpResponse, get};

#[get("/apiTest")]
pub async fn api_test() -> impl actix_web::Responder {
    HttpResponse::Ok().finish()
}
