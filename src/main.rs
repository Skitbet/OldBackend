use std::sync::Arc;
use std::time::Duration;

use actix_web::{ App, HttpServer, web };
use env_logger::Env;
use log::info;
use state::init_app_state;
use tracing_actix_web::TracingLogger;
use utils::default_cors;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::docs::ApiDoc;
use crate::scheduler::Scheduler;
use crate::services::watchdog::Watchdog;
use crate::socket::registry::WsRegistry;

mod auth;
mod database;
mod models;
mod routes;
mod services;
mod state;
mod utils;
mod task;
mod scheduler;
mod redis;
mod middleware;
mod socket;
mod docs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setup loggin
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Initializing app state...");
    let watchdog = Watchdog::new(
        Duration::from_secs(5), // tick interval
        Duration::from_secs(10), // check interval
        Duration::from_secs(10) // max allowed elapsed
    );
    watchdog.start();

    // load app state // managers // db // redis // env
    let state = init_app_state(watchdog).await;

    // get port from app state
    let port = state.port;
    info!("🚀 Server starting on 0.0.0.0:{}", port);

    // load our scheduler for tasks
    let mut scheduler = Scheduler::new(Arc::new(state.clone()));
    scheduler.start_all();

    // start the http server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(WsRegistry::default()))
            .wrap(TracingLogger::default())
            // .wrap(Compress::default())
            .wrap(default_cors())
            .service(
                SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
            )
            .configure(|cfg| get_config(cfg))
            .route("/ws", web::get().to(socket::handler::ws_route))
    })
        .bind(("0.0.0.0", port))?
        .run().await
}

pub fn get_config(cfg: &mut web::ServiceConfig) {
    cfg.configure(routes::config);

    cfg.default_service(
        web::to(|req: actix_web::HttpRequest| async move {
            log::warn!("404 Not Found: {} {}", req.method(), req.path());
            actix_web::HttpResponse::NotFound().body("Route not found")
        })
    );
}
