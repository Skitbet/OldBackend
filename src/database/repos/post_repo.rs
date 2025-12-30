use bson::{ doc, from_document, to_document, Document };
use futures::{ StreamExt, TryStreamExt };
use mongodb::{ Collection, Database, options::FindOptions };
use mongodb::options::AggregateOptions;
use crate::{ models::post::Post, utils::error::AppError };

#[derive(Clone)]
pub struct PostRepository {
    pub coll: Collection<Post>,
}

impl PostRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            coll: db.collection("posts"),
        }
    }

    pub async fn create(&self, post: &Post) -> Result<(), AppError> {
        self.coll.insert_one(post, None).await.map_err(|_| AppError::DBError)?;
        Ok(())
    }
    
    pub async fn get_all(&self) -> Result<Vec<Post>, AppError> {
        let options  = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();
        
        let cursor = self.coll.find(None, options).await.map_err(|_| AppError::DBError)?;
        let posts: Vec<Post> = cursor
            .try_collect().await
            .map_err(|_| AppError::InternalServerError("Failed to collect posts".into()))?;
        Ok(posts)
    }

    pub async fn save(&self, post: &Post) -> Result<(), AppError> {
        let filter = doc! { "_id": post.id.to_string() };
        let update =
            doc! { "$set": to_document(post).map_err(|e| AppError::InternalServerError(e.to_string()))? };

        self.coll
            .update_one(filter, update, None).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        let filter = doc! { "_id": id };

        self.coll.delete_one(filter, None).await.map_err(|_| AppError::DBError)?;
        Ok(())
    }

    pub async fn get_filtered_post(
        &self,
        limit: u64,
        skip: u64,
        tags: Option<Vec<String>>
    ) -> Result<Vec<Post>, AppError> {
        let mut filter = doc! {};

        if let Some(tags) = tags {
            if !tags.is_empty() {
                filter.insert("tags", doc! { "$in": tags });
            }
        }

        let options = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .limit(Some(limit as i64))
            .skip(Some(skip))
            .build();

        let cursor = self.coll
            .find(filter, options).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let posts: Vec<Post> = cursor
            .try_collect().await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(posts)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Post, AppError> {
        let filter = doc! { "_id": id };

        self.coll
            .find_one(filter, None).await
            .map_err(|_| AppError::DBError)?
            .ok_or(AppError::PostNotFound)
    }

    pub async fn find_by_author_and_short_id(
        &self,
        username: &str,
        short_id: &str
    ) -> Result<Post, AppError> {
        let filter =
            doc! {
            "author": username,
            "_id": { "$regex": format!("^{}", short_id) }
        };

        self.coll
            .find_one(filter, None).await
            .map_err(|_| AppError::DBError)?
            .ok_or(AppError::PostNotFound)
    }

    pub async fn get_all_by_user(
        &self,
        username: &str,
        limit: u64,
        skip: u64,
        tags: Option<Vec<String>>
    ) -> Result<Vec<Post>, AppError> {
        let mut filter = doc! { "author": username };

        if let Some(tags) = tags {
            if !tags.is_empty() {
                filter.insert("tags", doc! { "$in": tags });
            }
        }

        // log::debug!("filter {:?}", filter);
        // log::debug!("limit {}, skip {}", limit, skip);

        let options = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .limit(Some(limit as i64))
            .skip(Some(skip))
            .build();

        let cursor = self.coll
            .find(filter, options).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let posts: Vec<Post> = cursor
            .try_collect().await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(posts)
    }

    pub async fn get_latest(&self, limit: u64, skip: u64) -> Result<Vec<Post>, AppError> {
        let options = FindOptions::builder()
            .sort(doc! { "created_at": -1 })
            .skip(Some(skip))
            .limit(Some(limit as i64))
            .build();

        let cursor = self.coll.find(None, options).await.map_err(|_| AppError::DBError)?;

        let posts: Vec<Post> = cursor
            .try_collect().await
            .map_err(|_| AppError::InternalServerError("Failed to collect posts".into()))?;

        Ok(posts)
    }

    pub async fn get_popular_posts(
        &self,
        limit: u64,
        skip: u64,
        tags: Option<Vec<String>>
    ) -> Result<Vec<Post>, AppError> {
        let mut filter = doc! {};

        if let Some(tags) = tags {
            if !tags.is_empty() {
                filter.insert("tags", doc! { "$in": tags });
            }
        }

        let options = FindOptions::builder()
            .sort(doc! { "likes": -1 })
            .limit(Some(limit as i64))
            .skip(Some(skip))
            .build();

        let cursor = self.coll
            .find(filter, options).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let posts: Vec<Post> = cursor
            .try_collect().await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(posts)
    }

    pub async fn get_random_posts(
        &self,
        limit: u64,
        tags: Option<Vec<String>>
    ) -> Result<Vec<Post>, AppError> {
        let mut pipeline: Vec<Document> = vec![];

        if let Some(tags) = tags {
            if !tags.is_empty() {
                pipeline.push(
                    doc! {
                "$match": { "tags": { "$in": tags } }
            }
                );
            }
        }

        pipeline.push(doc! {
        "$sample": { "size": limit as i32 }
    });

        let mut cursor = self.coll
            .aggregate(pipeline, None).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let mut posts = Vec::new();

        while let Some(doc) = cursor.next().await {
            let doc = doc.map_err(|e| AppError::InternalServerError(e.to_string()))?;
            let post: Post = from_document(doc).map_err(|e|
                AppError::InternalServerError(format!("Deserialization error: {}", e))
            )?;
            posts.push(post);
        }

        Ok(posts)
    }

    pub async fn get_premium_posts(
        &self,
        limit: u64,
        skip: u64,
        tags: Option<Vec<String>>
    ) -> Result<Vec<Post>, AppError> {
        let mut pipeline: Vec<Document> = vec![
            doc! {
            "$lookup": {
                "from": "users",
                "localField": "author_id",   // from posts
                "foreignField": "_id",       // from users
                "as": "author_info"
            }
        },
            doc! {
            "$unwind": "$author_info"
        },
            doc! {
            "$match": {
                "author_info.premium": true
            }
        },
            doc! {
            "$sort": { "created_at": -1 }
        },
            doc! {
            "$skip": skip as i64
        },
            doc! {
            "$limit": limit as i64
        }
        ];

        if let Some(tags) = tags {
            if !tags.is_empty() {
                pipeline.insert(
                    3,
                    doc! {
                "$match": { "tags": { "$in": tags } }
            }
                );
            }
        }

        let mut cursor = self.coll
            .aggregate(pipeline, AggregateOptions::default()).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let mut posts = Vec::new();

        while
            let Some(doc) = cursor
                .try_next().await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?
        {
            let post: Post = from_document(doc).map_err(|e|
                AppError::InternalServerError(format!("Deserialization error: {}", e))
            )?;
            posts.push(post);
        }

        Ok(posts)
    }
}
