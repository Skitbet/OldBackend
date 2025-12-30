use crate::database::repos::announcement_repository::AnnouncementRepository;
use crate::models::announcement::Announcement;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct AnnouncementService {
    repo: AnnouncementRepository,
    cache: Arc<RwLock<Vec<Announcement>>>,
}

impl AnnouncementService {
    pub fn new(repo: AnnouncementRepository) -> Self {
        Self {
            repo,
            cache: Arc::new(RwLock::new(Vec::new())),
        }
    }

    // load all announcements into memory
    pub async fn load_cache(&self) -> anyhow::Result<()> {
        let anns = self.repo.get_all().await?;
        let mut cache = self.cache.write().await;
        cache.clear();
        cache.extend(anns);
        Ok(())
    }

    // get announcements from memory
    pub async fn get_announcements(&self) -> Vec<Announcement> {
        self.cache.read().await.clone()
    }

    // add a new announcement
    pub async fn new_announcement(&self, title: &str, body: &str) -> anyhow::Result<Announcement> {
        let ann = self.repo.create(title, body).await?;
        let mut cache = self.cache.write().await;
        cache.push(ann.clone());
        Ok(ann)
    }
}
