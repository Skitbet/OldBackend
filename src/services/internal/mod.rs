use crate::database::mongo::InkvaultDB;
use crate::database::repos::announcement_repository::AnnouncementRepository;
use crate::redis::InkvaultCache;
use crate::services::internal::announcement_service::AnnouncementService;
use crate::services::internal::post_service::PostService;
use crate::services::internal::profile_service::ProfileService;
use crate::utils::error::AppError;

mod profile_service;
mod post_service;
mod announcement_service;

#[derive(Clone)]
pub struct InternalServices {
    pub profile_service: ProfileService,
    pub post_service: PostService,
    pub announcement_service: AnnouncementService,
}

impl InternalServices {
    pub async fn new(db: InkvaultDB, cache: InkvaultCache) -> Result<Self, AppError> {
        let announcement_service = AnnouncementService::new(db.announcements);
        announcement_service
            .load_cache()
            .await
            .expect("Failed to load announcement cache");
        Ok(Self {
            profile_service: ProfileService::new(db.profiles, cache.profile_cache),
            post_service: PostService::new(db.posts, cache.post_cache),
            announcement_service
        })
    }
}