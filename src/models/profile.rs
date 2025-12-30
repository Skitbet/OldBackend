use std::collections::HashSet;

use bson::Document;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{ models::user::User, utils::uuid_as_string };
use crate::utils::roles::Role;

#[derive(Debug, Deserialize, ToSchema)]
pub struct PatchProfile {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub pronouns: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub links: Option<Vec<String>>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicProfile {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
    pub verified: bool,
    pub views: u64,
    pub role: Vec<Role>,
    pub pronouns: Vec<String>,
    pub languages: Vec<String>,
    pub links: Vec<String>,
    pub status: Option<String>,
    pub following: HashSet<String>,
    pub followers: HashSet<String>,
    pub profile_picture: Option<String>,
    pub banner_picture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DBProfile {
    #[serde(rename = "_id", with = "uuid_as_string")]
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
    pub verified: bool,
    pub views: u64,
    #[serde(default)]
    pub role: Vec<Role>,
    pub pronouns: Vec<String>,
    pub languages: Vec<String>,
    pub links: Vec<String>,
    pub status: Option<String>,
    pub following: HashSet<String>,
    pub followers: HashSet<String>,
    pub profile_picture: Option<String>,
    pub banner_picture: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PatchDBProfile {
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<Option<String>>,
    pub verified: Option<bool>,
    pub last_login: Option<DateTime<Utc>>,
    pub views: Option<u64>,
    pub role: Option<Vec<Role>>,
    pub pronouns: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub links: Option<Vec<String>>,
    pub status: Option<Option<String>>,
    pub following: Option<HashSet<String>>,
    pub followers: Option<HashSet<String>>,
    pub profile_picture: Option<Option<String>>,
    pub banner_picture: Option<Option<String>>,
}

impl DBProfile {
    pub fn new(user: &User, display_name: &String) -> Self {
        let now = Utc::now();
        DBProfile {
            id: user.id,
            username: user.username.to_string(),
            display_name: display_name.to_string(),
            bio: None,
            created_at: now,
            last_login: now,
            verified: false,
            views: 0,
            role: vec![Role::User],
            pronouns: vec![],
            languages: vec![],
            links: vec![],
            status: None,
            following: HashSet::new(),
            followers: HashSet::new(),
            profile_picture: None,
            banner_picture: None,
        }
    }

    pub fn to_public(&self) -> PublicProfile {
        PublicProfile {
            id: self.id,
            username: self.username.clone(),
            display_name: self.display_name.clone(),
            bio: self.bio.clone(),
            created_at: self.created_at,
            verified: self.verified,
            views: self.views,
            role: self.role.clone(),
            pronouns: self.pronouns.clone(),
            languages: self.languages.clone(),
            links: self.links.clone(),
            status: self.status.clone(),
            following: self.following.clone(),
            followers: self.followers.clone(),
            profile_picture: self.profile_picture.clone(),
            banner_picture: self.banner_picture.clone(),
        }
    }

    pub fn apply_patch(&mut self, patch: PatchDBProfile) {
        if let Some(username) = patch.username {
            self.username = username;
        }
        if let Some(display_name) = patch.display_name {
            self.display_name = display_name;
        }
        if let Some(bio) = patch.bio {
            self.bio = bio;
        }
        if let Some(last_login) = patch.last_login {
            self.last_login = last_login;
        }
        if let Some(verified) = patch.verified {
            self.verified = verified;
        }
        if let Some(views) = patch.views {
            self.views = views;
        }
        if let Some(role) = patch.role {
            self.role = role;
        }
        if let Some(pronouns) = patch.pronouns {
            self.pronouns = pronouns;
        }
        if let Some(languages) = patch.languages {
            self.languages = languages;
        }
        if let Some(links) = patch.links {
            self.links = links;
        }
        if let Some(status) = patch.status {
            self.status = status;
        }
        if let Some(following) = patch.following {
            self.following = following;
        }
        if let Some(followers) = patch.followers {
            self.followers = followers;
        }
        if let Some(profile_picture) = patch.profile_picture {
            self.profile_picture = profile_picture;
        }
        if let Some(banner_picture) = patch.banner_picture {
            self.banner_picture = banner_picture;
        }
    }
}

impl PatchDBProfile {
    pub fn to_mongo_doc(self) -> Document {
        let mut doc = Document::new();

        if let Some(username) = self.username {
            doc.insert("username", username);
        }
        if let Some(display_name) = self.display_name {
            doc.insert("display_name", display_name);
        }
        if let Some(bio) = self.bio {
            doc.insert("bio", bio); // Nullable string
        }
        if let Some(last_login) = self.last_login {
            doc.insert("last_login", last_login);
        }
        if let Some(verified) = self.verified {
            doc.insert("verified", verified);
        }
        if let Some(views) = self.views {
            doc.insert("views", views as i64); // BSON uses i64
        }
        if let Some(role) = self.role {
            doc.insert("role", bson::to_bson(&role).unwrap());
        }
        if let Some(pronouns) = self.pronouns {
            doc.insert("pronouns", bson::to_bson(&pronouns).unwrap());
        }
        if let Some(languages) = self.languages {
            doc.insert("languages", bson::to_bson(&languages).unwrap());
        }
        if let Some(links) = self.links {
            doc.insert("links", bson::to_bson(&links).unwrap());
        }
        if let Some(status) = self.status {
            doc.insert("status", status);
        }
        if let Some(following) = self.following {
            doc.insert("following", bson::to_bson(&following).unwrap());
        }
        if let Some(followers) = self.followers {
            doc.insert("followers", bson::to_bson(&followers).unwrap());
        }
        if let Some(profile_picture) = self.profile_picture {
            doc.insert("profile_picture", profile_picture);
        }
        if let Some(banner_picture) = self.banner_picture {
            doc.insert("banner_picture", banner_picture);
        }

        doc
    }
}
