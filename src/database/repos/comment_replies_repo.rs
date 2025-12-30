use std::collections::HashSet;
use bson::{doc, to_document};
use futures::TryStreamExt;
use uuid::Uuid;
use mongodb::{Collection, Database};

use crate::utils::error::AppError;
use crate::models::comment::{CommentReplies, Reply};

#[derive(Clone)]
pub struct CommentRepliesRepository {
    coll: Collection<CommentReplies>,
}

impl CommentRepliesRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            coll: db.collection("comment_replies"),
        }
    }

    pub async fn create(&self, comment_id: &Uuid) -> Result<(), AppError> {
        let comment_replies = CommentReplies {
            id: *comment_id,
            has_replies: false,
            sub_ids: HashSet::new(),
            replies: Vec::new(),
        };
        
        self.coll
            .insert_one(comment_replies, None)
            .await
            .map_err(|_| AppError::DBError)?;
        Ok(()) 
    }

    // Recursively add a reply to the correct parent reply by id
    fn add_reply_recursive(replies: &mut Vec<Reply>, parent_id: &str, reply: &Reply) -> bool {
        for r in replies.iter_mut() {
            if r.id.to_string() == parent_id {
                r.replies.push(reply.clone());
                return true;
            }
            if Self::add_reply_recursive(&mut r.replies, parent_id, &reply) {
                return true;
            }
        }
        false
    }

    pub async fn add_reply(&self, comment_id: &str, parent_id: &str, reply: &Reply) -> Result<(), AppError> {
        // Fetch the comment
        let filter = doc! { "_id": comment_id };
        let mut comment = self.coll
            .find_one(filter.clone(), None)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .ok_or(AppError::CommentNotFound)?;

        // If parent_id is the comment itself, add as top-level reply
        if comment.id.to_string() == parent_id {
            comment.replies.push(reply.clone());
        } else {
            if !Self::add_reply_recursive(&mut comment.replies, parent_id, reply) {
                return Err(AppError::CommentNotFound);
            }
        }

        comment.sub_ids.insert(reply.id.to_string());

        // Save updated comment
        let update = doc! { "$set": to_document(&comment).unwrap() };
        self.coll
            .update_one(filter, update, None)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn add_reply_by_parent_id(&self, parent_id: &str, reply: &Reply) -> Result<(), AppError> {
        // Find the CommentReplies document containing the parent_id
        let mut cursor = self.coll.find(None, None)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        while let Some(mut comment) = cursor.try_next().await
            .map_err(|e| AppError::InternalServerError(e.to_string()))? {
            // Check if parent_id matches the comment itself
            if comment.id.to_string() == parent_id {
                comment.replies.push(reply.clone());
            } else if Self::add_reply_recursive(&mut comment.replies, parent_id, reply) {
                // reply added in nested replies
            } else {
                continue;
            }
            
            comment.has_replies = true;

            comment.sub_ids.insert(reply.id.to_string());
            let filter = doc! { "_id": comment.id.to_string() };
            let update = doc! { "$set": to_document(&comment).unwrap() };
            self.coll.update_one(filter, update, None)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;
            return Ok(());
        }

        Err(AppError::CommentNotFound)
    }

    pub async fn get_replies(&self, comment_id: &str) -> Result<CommentReplies, AppError> {
        let filter = doc! { "_id": comment_id };
        let comment = self.coll
            .find_one(filter, None)
            .await
            .map_err(|_| AppError::DBError)?
            .ok_or(AppError::CommentNotFound)?;
        Ok(comment)
    }

    // pub async fn save(&self, comment: &CommentReplies) -> Result<(), AppError> {
    //     let filter = doc! { "_id": comment.id.to_string() };
    //     let update = doc! { "$set": to_document(comment).map_err(|e| AppError::InternalServerError(e.to_string()))? };
    // 
    //     self.coll
    //         .update_one(filter, update, None)
    //         .await
    //         .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    // 
    //     Ok(())
    // }

    pub async fn delete_by_id(&self, id: &str) -> Result<bool, AppError> {
        let filter = doc! { "_id": id };
        let result = self
            .coll
            .delete_one(filter, None)
            .await
            .map_err(|_| AppError::DBError)?;

        Ok(result.deleted_count > 0)
    }

    fn like_reply_recursive(replies: &mut Vec<Reply>, reply_id: &str, user_id: &str) -> Option<bool> {
        for reply in replies.iter_mut() {
            if reply.id.to_string() == reply_id {
                let inserted = reply.likes.insert(user_id.to_string());
                if inserted {
                    reply.dislikes.remove(user_id);
                } else {
                    reply.likes.remove(user_id); // toggle off
                }
                return Some(inserted);
            }
            if let Some(r) = Self::like_reply_recursive(&mut reply.replies, reply_id, user_id) {
                return Some(r);
            }
        }
        None
    }

    pub async fn like_reply(&self, reply_id: &str, user_id: &str) -> Result<bool, AppError> {
        let mut cursor = self.coll.find(None, None)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        while let Some(mut comment) = cursor.try_next().await
            .map_err(|e| AppError::InternalServerError(e.to_string()))? {
            if let Some(liked) = Self::like_reply_recursive(&mut comment.replies, reply_id, user_id) {
                let filter = doc! { "_id": comment.id.to_string() };
                let update = doc! { "$set": to_document(&comment).unwrap() };
                self.coll.update_one(filter, update, None)
                    .await
                    .map_err(|e| AppError::InternalServerError(e.to_string()))?;
                return Ok(liked);
            }
        }

        Err(AppError::CommentNotFound)
    }

    fn dislike_reply_recursive(replies: &mut Vec<Reply>, reply_id: &str, user_id: &str) -> Option<bool> {
        for reply in replies.iter_mut() {
            if reply.id.to_string() == reply_id {
                let inserted = reply.dislikes.insert(user_id.to_string());
                if inserted {
                    reply.likes.remove(user_id);
                } else {
                    reply.dislikes.remove(user_id); // toggle off
                }
                return Some(inserted);
            }
            if let Some(r) = Self::dislike_reply_recursive(&mut reply.replies, reply_id, user_id) {
                return Some(r);
            }
        }
        None
    }

    pub async fn dislike_reply(&self, reply_id: &str, user_id: &str) -> Result<bool, AppError> {
        let mut cursor = self.coll.find(None, None)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        while let Some(mut comment) = cursor.try_next().await
            .map_err(|e| AppError::InternalServerError(e.to_string()))? {
            if let Some(disliked) = Self::dislike_reply_recursive(&mut comment.replies, reply_id, user_id) {
                let filter = doc! { "_id": comment.id.to_string() };
                let update = doc! { "$set": to_document(&comment).unwrap() };
                self.coll.update_one(filter, update, None)
                    .await
                    .map_err(|e| AppError::InternalServerError(e.to_string()))?;
                return Ok(disliked);
            }
        }

        Err(AppError::CommentNotFound)
    }
}
