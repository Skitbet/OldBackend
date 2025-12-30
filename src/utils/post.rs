use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

/// Represents the type of a post.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PostType {
    Generic,
}
