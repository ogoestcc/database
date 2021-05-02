use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::{models, services::types::users as user_service};

fn default_active() -> bool {
    true
}

pub fn default_date() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 42_000_000)
}

#[repr(C)]
#[derive(Debug, PostgresMapper, Serialize, Deserialize, Clone)]
#[pg_mapper(table = "users")]
pub struct Users {
    #[serde(rename = "user_id")]
    pub id: i64,
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
        self.id == id
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
            id: 0i64,
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

impl From<&Users> for user_service::User {
    fn from(usr: &Users) -> Self {
        Self {
            id: usr.id as i32,
            email: usr.email.clone(),
            password: Some(usr.password.clone()),
            active: usr.active,
            created_at: usr.created_at.to_string(),
            updated_at: usr.updated_at.to_string(),
            deleted_at: if let Some(deleted) = usr.deleted_at.clone() {
                Some(deleted.to_string())
            } else {
                None
            },
        }
    }
}
