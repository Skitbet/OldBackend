use actix_web::web;
use log::info;

use crate::routes::internal::settings::handler::change_password;

mod handler;
pub mod types;

pub fn config(cfg: &mut web::ServiceConfig) {
    info!("Configuring /api/settings scope");
    cfg.service(web::scope("/settings").service(change_password));
}
