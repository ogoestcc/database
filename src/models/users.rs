use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "postgres")]
use tokio_pg_mapper_derive::PostgresMapper;

use crate::{models, services::types::users as user_service};

fn default_active() -> bool {
    true
}

pub fn default_date() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 42_000_000)
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct Users {
    #[serde(rename = "user_id")]
    pub id: i32,
    #[serde(default)]
    email: String,
    #[serde(default)]
    password: String,
    #[serde(default = "default_active")]
    active: bool,
    #[serde(default = "default_date")]
    created_at: NaiveDateTime,
    #[serde(default = "default_date")]
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}

impl Users {
    pub fn same_id(&self, id: i64) -> bool {
        self.id == id as i32
    }

    pub fn same_email(&self, email: String) -> bool {
        self.email == email
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl Default for Users {
    fn default() -> Self {
        Users {
            id: 0,
            email: "".into(),
            password: "".into(),
            active: true,
            created_at: default_date(),
            updated_at: default_date(),
            deleted_at: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserRatings {
    pub user: Users,
    pub ratings: Vec<models::Ratings>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserContents {
    #[serde(flatten)]
    pub user: Users,
    #[serde(with = "preferences")]
    pub preferences: Vec<models::Contents>,
}

pub mod preferences {
    use serde::{Deserialize, Deserializer};

    use crate::models::Contents;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Contents>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?
            .replace(&['[', ']', '\'', ' '][..], "")
            .split(',')
            .map(|s| s.into())
            .collect())
    }
}

impl From<Users> for user_service::User {
    fn from(user: Users) -> Self {
        user_service::User {
            id: user.id as i32,
            email: user.email.clone(),
            password: Some(user.password.clone()),
            active: user.active,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
            deleted_at: user.deleted_at.map(|d| d.to_string()),
        }
    }
}
