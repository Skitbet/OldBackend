use std::sync::Arc;
use tokio::sync::Mutex;
use crate::database::repos::post_repo::PostRepository;
use crate::models::post::{AdminPatchPost, Post};
use crate::redis::cache::post_cache::PostCache;
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct PostService {
    repo: PostRepository,
    cache: PostCache,
    latest_head: Arc<Mutex<Vec<Post>>>, // i love rust ;3
}

impl PostService {
    pub fn new(repo: PostRepository, cache: PostCache) -> Self {
        Self { repo, cache, latest_head: Arc::new(Mutex::new(Vec::new())) }
    }

    /// Gets a post by ID, checking Redis cache first.
    pub async fn get_by_id(&self, id: &str) -> Result<Post, AppError> {
        if let Some(cached) = self.cache.get(id).await? {
            log::debug!("Cache hit for post {id}");
            return Ok(cached);
        }

        log::debug!("Cache miss for post {id}, loading from Mongo");
        let post = self.repo.get_by_id(id).await?;
        self.cache.set(&post).await.ok(); // cache set failure is non-fatal
        Ok(post)
    }

    /// creates a post (no need to cache right now)
    pub async fn create(&self, post: &Post) -> Result<(), AppError> {
        self.repo.create(post).await?;
        self.cache.set(&post).await.ok(); // cache it immediately
        
        // update latest post cache
        {
            let mut latest = self.latest_head.lock().await;
            latest.insert(0, post.clone());
            if latest.len() > 100 {
                latest.truncate(100);
            }
        }
        
        Ok(())
    }

    /// Saves a post and updates the cache
    pub async fn save(&self, post: &Post) -> Result<(), AppError> {
        self.repo.save(post).await?;
        self.cache.set(&post).await.ok(); // update cache
        Ok(())
    }

    pub async fn patch(&self, id: &str, patch: AdminPatchPost) -> Result<Post, AppError> {
        let mut post = self.get_by_id(id).await?;

        post.apply_patch(patch);

        self.save(&post).await?;

        Ok(post)
    }

    /// invalidates a post by ID
    pub async fn invalidate(&self, id: &str) -> Result<(), AppError> {
        self.cache.invalidate(id).await
    }
    
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        self.repo.delete(id).await?;
        self.cache.invalidate(id).await?;
        
        // also remove from latest head cache
        let mut latest = self.latest_head.lock().await;
        if let Some(pos) = latest.iter().position(|p| p.id.to_string() == id) {
            latest.remove(pos);
        }
        
        Ok(())
    }

    /// Fetches latest posts
    pub async fn get_latest(&self, limit: u64, skip: u64) -> Result<Vec<Post>, AppError> {
        const CACHE_LIMIT: usize = 100;

        if skip + limit <= CACHE_LIMIT as u64 {
            let head = self.latest_head.lock().await;
            let start = skip as usize;
            let end = (skip + limit) as usize;
            if head.len() >= end {
                log::debug!("Serving from in-memory cache");
                return Ok(head[start..end].to_vec());
            }
        }

        // fallback to DB
        let posts = self.repo.get_latest(limit, skip).await?;

        // set last 100 cache
        if skip == 0 {
            let mut head = self.latest_head.lock().await;
            *head = self.repo.get_latest(CACHE_LIMIT as u64, 0).await?;
        }

        self.cache.set_many(&posts).await.ok();
        Ok(posts)
    }

    /// gets posts by author, with optional tags, no caching
    pub async fn get_all_by_user(&self, username: &str, limit: u64, skip: u64, tags: Option<Vec<String>>) -> Result<Vec<Post>, AppError> {
        let posts = self.repo.get_all_by_user(username, limit, skip, tags).await?;
        self.cache.set_many(&posts).await.ok();
        Ok(posts)
    }
    
    // Get all posts (no caching, used for admin panel)
    pub async fn get_all(&self) -> Result<Vec<Post>, AppError> {
        let posts = self.repo.get_all().await?;
        Ok(posts)
    }

    /// Gets posts by filter (for global tag-based feed)
    pub async fn get_filtered(&self, limit: u64, skip: u64, tags: Option<Vec<String>>) -> Result<Vec<Post>, AppError> {
        let posts = self.repo.get_filtered_post(limit, skip, tags).await?;
        self.cache.set_many(&posts).await.ok();
        Ok(posts)
    }

    /// Find a post using author + short ID (no caching... used for routing)
    pub async fn find_by_author_and_short_id(&self, author: &str, short_id: &str) -> Result<Post, AppError> {
        let post = self.repo.find_by_author_and_short_id(author, short_id).await?;
        self.cache.set(&post).await.ok();
        Ok(post)
    }
    
    pub async fn get_popular(&self, limit: u64, offset: u64, tags: Option<Vec<String>>) -> Result<Vec<Post>, AppError> {
        let posts = self.repo.get_popular_posts(limit, offset, tags).await?;
        self.cache.set_many(&posts).await.ok();
        Ok(posts)
    }

    pub async fn get_random(&self, limit: u64, tags: Option<Vec<String>>) -> Result<Vec<Post>, AppError> {
        let posts = self.repo.get_random_posts(limit, tags).await?;
        self.cache.set_many(&posts).await.ok();
        Ok(posts)
    }

    pub async fn get_premium(&self, limit: u64, offset: u64, tags: Option<Vec<String>>) -> Result<Vec<Post>, AppError> {
        let posts = self.repo.get_premium_posts(limit, offset, tags).await?;
        self.cache.set_many(&posts).await.ok();
        Ok(posts)
    }
}
