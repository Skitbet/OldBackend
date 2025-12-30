use actix_multipart::Multipart;
use actix_web::web::BytesMut;
use futures::StreamExt;
use mime_guess::from_path;
use std::collections::HashMap;

use crate::utils::error::AppError;
use crate::services::r2::{ALLOWED_IMAGE_TYPES, ALLOWED_VIDEO_TYPES, MAX_IMAGE_SIZE_BYTES, MAX_VIDEO_SIZE_BYTES};

// lil struct to hold file info when someone uploads something
pub struct UploadedFile {
    pub filename: String,       // name of the file
    pub content_type: String,   // like "image/png" or "text/plain"
    pub data: BytesMut,         // the actual file data in memory
}

pub async fn parse_multipart(
    mut payload: Multipart,
) -> Result<(HashMap<String, String>, Vec<UploadedFile>), AppError> {
    let mut fields = HashMap::new();
    let mut media_files = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|_| AppError::BadRequest("Invalid multipart".into()))?;
        let cd = field.content_disposition().cloned();

        match cd.as_ref().and_then(|cd| cd.get_name()) {
            Some("media") => {
                if let Some(filename) = cd.and_then(|cd| cd.get_filename().map(str::to_owned)) {
                    let content_type = field
                        .content_type()
                        .map(|ct| ct.to_string())
                        .unwrap_or_else(|| from_path(&filename).first_or_octet_stream().to_string());

                    let is_image = ALLOWED_IMAGE_TYPES.contains(&content_type.as_str());
                    let is_video = ALLOWED_VIDEO_TYPES.contains(&content_type.as_str());

                    if !is_image && !is_video {
                        return Err(AppError::BadRequest(format!(
                            "Unsupported file type: {}",
                            content_type
                        )));
                    }

                    let mut file_data = BytesMut::new();
                    while let Some(chunk) = field.next().await {
                        let data = chunk.map_err(|_| AppError::BadRequest("Read chunk failure".into()))?;

                        let limit = if is_video { MAX_VIDEO_SIZE_BYTES } else { MAX_IMAGE_SIZE_BYTES };

                        if file_data.len() + data.len() > limit {
                            return Err(AppError::FileToBig(limit.to_string()));
                        }
                        file_data.extend_from_slice(&data);
                    }

                    media_files.push(UploadedFile {
                        filename,
                        content_type,
                        data: file_data,
                    });
                }
            }
            Some(name) => {
                let mut value = Vec::new();
                while let Some(chunk) = field.next().await {
                    let data = chunk.map_err(|_| AppError::BadRequest("Read form field failed".into()))?;
                    value.extend_from_slice(&data);
                }
                fields.insert(name.to_string(), String::from_utf8(value).unwrap_or_default());
            }
            _ => {}
        }
    }

    Ok((fields, media_files))
}
