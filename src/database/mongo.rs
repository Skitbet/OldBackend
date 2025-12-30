use crate::utils::error::AppError;
use mongodb::{Client, Database};
use crate::database::repos::announcement_repository::AnnouncementRepository;
use crate::database::repos::codes_repo::CodeRepository;
use crate::database::repos::comment_replies_repo::CommentRepliesRepository;
use crate::database::repos::comment_repo::CommentRepository;
use crate::database::repos::post_repo::PostRepository;
use crate::database::repos::preuser_repo::PreRegisterUserRepository;
use crate::database::repos::profile_repo::ProfileRepository;
use crate::database::repos::reports_repo::ReportRepository;
use crate::database::repos::session_repo::SessionRepository;
use crate::database::repos::settings_repo::SettingsRepository;
use crate::database::repos::user_repo::UserRepository;

#[derive(Clone)]
pub struct InkvaultDB {
    pub _db: Database,

    pub users: UserRepository,
    pub sessions: SessionRepository,
    pub profiles: ProfileRepository,
    pub posts: PostRepository,
    pub settings: SettingsRepository,
    pub comments: CommentRepository,
    pub comment_replies: CommentRepliesRepository,
    pub codes_repo: CodeRepository,
    pub pre_user_repo: PreRegisterUserRepository,
    pub reporting: ReportRepository,
    pub announcements: AnnouncementRepository,
}

impl InkvaultDB {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self, AppError> {
        log::info!("Attempted to connect to MongoDB");
        let client = Client::with_uri_str(uri)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Mongo connection error: {}", e)))?;

        let db = client.database(db_name);
        log::info!("Successfully connected to MongoDB");
        Ok(Self {
            _db: db.clone(),
            users: UserRepository::new(&db),
            sessions: SessionRepository::new(&db),
            profiles: ProfileRepository::new(&db),
            posts: PostRepository::new(&db),
            settings: SettingsRepository::new(&db),
            comments: CommentRepository::new(&db),
            comment_replies: CommentRepliesRepository::new(&db),
            codes_repo: CodeRepository::new(&db),
            pre_user_repo: PreRegisterUserRepository::new(&db),
            reporting: ReportRepository::new(&db),
            announcements: AnnouncementRepository::new(&db),
        })
    }
}
