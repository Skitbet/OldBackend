use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest};
use futures_util::future::BoxFuture;
use crate::models::session::Session;
use crate::state::AppState;
use crate::utils::auth::{get_session_token_from_header, validate_and_refresh_session};

#[derive(Debug)]
pub struct Auther {
    pub session: Session,
}

impl FromRequest for Auther {
    type Error = Error;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let data = match req.app_data::<web::Data<AppState>>().cloned() {
            Some(d) => d,
            None => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorInternalServerError("AppState missing"))
                });
            }
        };

        // Try to get the token, return 401 if missing or invalid
        let token = match get_session_token_from_header(&req) {
            Ok(t) => t,
            Err(_) => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Invalid or missing authorization token"))
                });
            }
        };

        Box::pin(async move {
            let session = validate_and_refresh_session(token, &data)
                .await
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid authorization"))?;

            Ok(Auther { session })
        })
    }
}