use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest};
use futures_util::future::BoxFuture;

use crate::state::AppState;
use crate::utils::auth::get_session_token_from_header;

#[derive(Debug)]
pub struct AdminGuard;

impl FromRequest for AdminGuard {
    type Error = Error;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let data = req
            .app_data::<web::Data<AppState>>()
            .cloned()
            .expect("AppState missing");

        let token = match get_session_token_from_header(&req) {
            Ok(t) => t,
            Err(_) => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Invalid or missing authorization token"))
                });
            }
        };

        Box::pin(async move {
            let session = data
                .db
                .sessions
                .get_by_token(&token)
                .await
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid session"))?;

            let profile = data
                .db
                .profiles
                .get_by_uuid(&session.user_uuid)
                .await
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid user data"))?;

            if profile.role.iter().any(|r| r.is_admin()) {
                Ok(AdminGuard)
            } else {
                Err(actix_web::error::ErrorForbidden("User is not an admin"))
            }
        })
    }
}
