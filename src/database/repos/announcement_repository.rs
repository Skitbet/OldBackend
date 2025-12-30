use crate::models::announcement::Announcement;
use mongodb::{ Collection, Database };
use uuid::Uuid;
use chrono::Utc;
use futures_util::TryStreamExt;

#[derive(Clone)]
pub struct AnnouncementRepository {
    collection: Collection<Announcement>,
}

impl AnnouncementRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("announcements"),
        }
    }
    pub async fn create(&self, title: &str, body: &str) -> mongodb::error::Result<Announcement> {
        let ann = Announcement {
            id: Uuid::new_v4(),
            title: title.to_string(),
            body: body.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.collection.insert_one(&ann, None).await?;
        Ok(ann)
    }

    pub async fn get_all(&self) -> mongodb::error::Result<Vec<Announcement>> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut anns = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            anns.push(doc);
        }
        Ok(anns)
    }
}
