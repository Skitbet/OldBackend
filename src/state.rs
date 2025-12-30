use std::collections::HashSet;
use dotenv::dotenv;
use std::env;
use std::sync::{Arc, Mutex};
use crate::{ database::mongo::InkvaultDB, services::{ r2::R2, watchdog::Watchdog } };
use crate::redis::InkvaultCache;
use crate::services::internal::InternalServices;
use crate::services::smtp_service::{EmailConfig, EmailService };

#[derive(Clone)]
pub struct AppState {
    pub port: u16,
    pub db: InkvaultDB,
    // pub cache: InkvaultCache,
    pub services: InternalServices,
    pub jwt_secret: String,
    pub jwt_expiration_seconds: i64,
    pub r2: R2,
    pub cdn_domain: String,
    pub frontend_domain: String,
    pub watchdog: Watchdog,
    pub smtp_service: EmailService,
    pub connected_ws_users: Arc<Mutex<HashSet<String>>>,
}

pub async fn init_app_state(watchdog: Watchdog) -> AppState {
    dotenv().ok();
    let cdn_domain = env::var("CDN_DOMAIN").expect("CDN_DOMAIN must be set");
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_expiration_seconds = env
        ::var("JWT_EXPIRATION_SECONDS")
        .unwrap_or_else(|_| "3600".to_string())
        .parse::<i64>()
        .expect("JWT_EXPIRATION_SECONDS must be an integer");

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    
    let db = InkvaultDB::new(&mongodb_uri, "inkvault").await.expect("Failed to initalize MongoDB");
    let cache = InkvaultCache::new(&redis_url).await.expect("Failed to init Redis");
    
    let services = InternalServices::new(db.clone(), cache).await.expect("Failed to init internal services");
    
    let port = env
        ::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let frontend_domain = env::var("FRONTEND_DOMAIN").expect("FRONTEND_DOMAIN is not in env vars");

    let email_service_config = EmailConfig {
        smtp_server: env::var("SMTP_SERVER").expect("Missing SMTP_SERVER"),
        smtp_port: env::var("SMTP_PORT").expect("Missing SMTP_PORT").parse().unwrap(),
        smtp_user: env::var("SMTP_USER").expect("Missing SMTP_USER"),
        smtp_pass: env::var("SMTP_PASS").expect("Missing SMTP_PASS"),
        from_address: "InkVault <noreply@inkvault.art>".into(), // wouldnt let this work in var??
        frontend_url: frontend_domain.clone(),
    };

    let smtp_service = EmailService::new(
        Arc::new(email_service_config),
        "./assets/email_templates/verify_code.html".to_string(),
        "./assets/email_templates/reset_password.html".to_string(),
    );

    AppState {
        port,
        db,
        // cache,
        services,
        jwt_secret,
        jwt_expiration_seconds,
        r2: R2::new_from_env().await,
        cdn_domain,
        frontend_domain,
        watchdog,
        smtp_service,
        connected_ws_users: Arc::new(Mutex::new(HashSet::new())),
    }
}
