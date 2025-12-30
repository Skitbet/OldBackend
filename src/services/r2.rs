use std::env;

use aws_config::Region;
use aws_sdk_s3::{
    Client, Config,
    config::Credentials,
    primitives::ByteStream,
    types::ObjectCannedAcl,
};
use log::info;
use mime_guess::from_path;
use uuid::Uuid;

use crate::{
    utils::{post::PostType, r2endpoint::CustomEndpointResolver},
    utils::hash::hash_bytes_sha256_slimmed,
};

// max sizes
pub const MAX_IMAGE_SIZE_BYTES: usize = 15 * 1024 * 1024; // 15 MB
pub const MAX_VIDEO_SIZE_BYTES: usize = 20 * 1024 * 1024; // 20 MB

// allowed MIME types
pub const ALLOWED_IMAGE_TYPES: &[&str] = &[
    "image/png",
    "image/jpeg",
    "image/webp",
    "image/gif",
];
pub const ALLOWED_VIDEO_TYPES: &[&str] = &[
    "video/mp4",
    "video/webm",
];

/// Types of user assets for organized storage.
pub enum UserAssetType {
    ProfilePicture,
    Banner,
}

/// Alias for S3 upload errors to simplify signatures.
type S3Error = aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::put_object::PutObjectError>;

#[derive(Clone, Debug)]
pub struct R2 {
    client: Client,
    bucket: String,
    cdn_domain: String,
}

impl R2 {
    /// Creates a new `R2` client configured from environment variables.
    pub async fn new_from_env() -> Self {
        info!("Attempting to initialize R2");
        let account_id = env::var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID must be set");
        let access_key = env::var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID must be set");
        let secret_key = env::var("R2_SECRET_KEY").expect("R2_SECRET_KEY must be set");
        let bucket = env::var("R2_BUCKET").expect("R2_BUCKET must be set");
        let cdn_domain = env::var("CDN_DOMAIN").expect("CDN_DOMAIN must be set");

        let endpoint_url = format!("https://{}.r2.cloudflarestorage.com/{}", account_id, bucket);
        let endpoint = CustomEndpointResolver { endpoint: endpoint_url };

        let config = Config::builder()
            .credentials_provider(Credentials::new(
                access_key, secret_key, None, None, "static",
            ))
            .region(Region::new("auto"))
            .endpoint_resolver(endpoint)
            .behavior_version_latest()
            .build();

        let client = Client::from_conf(config);
        info!("Successfully to initialize R2");
        Self { client, bucket, cdn_domain }
    }

    /// Uploads a user asset (profile picture or banner) and returns its public URL.
    pub async fn upload_user_asset(
        &self,
        username: &str,
        asset_type: UserAssetType,
        bytes: &[u8],
    ) -> Result<String, S3Error> {
        let hash = hash_bytes_sha256_slimmed(bytes);
        let dir = match asset_type {
            UserAssetType::ProfilePicture => "profile",
            UserAssetType::Banner => "banner",
        };

        let key = format!("userassets/{}/{}/{}.png", username, dir, hash);
        self.upload_bytes(&key, bytes, Some("image/png".to_string())).await?;
        Ok(format!("https://{}/{}", self.cdn_domain, key))
    }

    /// Uploads a post asset with the given filename and bytes.
    pub async fn upload_post_asset(
        &self,
        post_id: Uuid,
        _post_type: PostType,
        filename: &str,
        bytes: &[u8],
    ) -> Result<(), S3Error> {
        let key = format!("postassets/{}/{}", post_id, filename);
        let content_type = from_path(filename).first_or_octet_stream().to_string();
        self.upload_bytes(&key, bytes, Some(content_type)).await
    }

    /// Core method to upload bytes to the given key in the R2 bucket.
    ///
    /// If `content_type` is `None`, it will be guessed from the key's file extension.
    pub async fn upload_bytes(
        &self,
        key: &str,
        bytes: &[u8],
        content_type: Option<String>,
    ) -> Result<(), S3Error> {
        let content_type = content_type.unwrap_or_else(|| from_path(key).first_or_octet_stream().to_string());
        let stream = ByteStream::from(bytes.to_vec());

        info!("Uploading {} to R2 bucket {}", key, self.bucket);
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(stream)
            .content_type(content_type)
            .metadata("postedon", "InkVault")
            .metadata("posteddate", chrono::Utc::now().to_rfc3339())
            .acl(ObjectCannedAcl::PublicRead)
            .send()
            .await?;

        info!("Uploaded {} to R2 bucket {}", key, self.bucket);
        Ok(())
    }
}
