use bson::{doc, to_document};
use futures::TryStreamExt;
use mongodb::{Collection, Database, options::FindOptions};

use crate::{models::comment::Comment, utils::error::AppError};

#[derive(Clone)]
pub struct CommentRepository {
    coll: Collection<Comment>,
}

impl CommentRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            coll: db.collection("comments"),
        }
    }

    pub async fn create(&self, comment: &Comment) -> Result<(), AppError> {
        self.coll
            .insert_one(comment, None)
            .await
            .map_err(|_| AppError::DBError)?;
        Ok(())
    }
    
    pub async fn get_by_id(&self, comment_id: &str) -> Result<Comment, AppError> {
        let filter = doc! { "_id": comment_id };
        self.coll.find_one(filter, None)
            .await
            .map_err(|_| AppError::DBError)?
            .ok_or(AppError::CommentNotFound)
    }

    pub async fn get_for_post(&self, post_id: &str) -> Result<Vec<Comment>, AppError> {
        let filter = doc! { "post_id": post_id };
        let options = FindOptions::builder()
            .sort(doc! { "created_at": -1 }) // newest first
            .build();

        let cursor = self
            .coll
            .find(filter, options)
            .await
            .map_err(|_| AppError::DBError)?;

        let comments: Vec<Comment> = cursor
            .try_collect()
            .await
            .map_err(|_| AppError::InternalServerError("Failed to collect comments.".into()))?;

        Ok(comments)
    }

    pub async fn save(&self, comment: &Comment) -> Result<(), AppError> {
        let filter = doc! { "_id": comment.id.to_string() };
        let update = doc! { "$set": to_document(comment).map_err(|e| AppError::InternalServerError(e.to_string()))? };

        self.coll
            .update_one(filter, update, None)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn delete_by_id(&self, id: &str) -> Result<bool, AppError> {
        let filter = doc! { "_id": id };
        let result = self
            .coll
            .delete_one(filter, None)
            .await
            .map_err(|_| AppError::DBError)?;

        Ok(result.deleted_count > 0)
    }
}
