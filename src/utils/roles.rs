use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Owner,
    Moderator,
    #[serde(other)]
    User, // fallback for unrecognized roles
}

impl Role {
    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Moderator | Role::Owner)
    }

    pub fn can_edit_others(&self) -> bool {
        matches!(self, Role::Moderator | Role::Owner)
    }

    pub fn can_manage_roles(&self) -> bool {
        matches!(self, Role::Owner)
    }
}

impl Default for Role {
    fn default() -> Self {
        Role::User
    }
}
