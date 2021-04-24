use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::services::users::User;

fn default_active() -> bool {
    true
}

#[repr(C)]
#[derive(Debug, PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "users")]
pub struct Users {
    #[serde(rename = "user_id")]
    id: i32,
    #[serde(default)]
    email: String,
    #[serde(default)]
    password: String,
    #[serde(default = "default_active")]
    active: bool,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    updated_at: String,
    #[serde(default)]
    deleted_at: Option<String>,
}

impl Users {
    pub fn same_id(&self, id: i32) -> bool {
        self.id == id
    }

    pub fn same_email(&self, email: String) -> bool {
        self.email == email
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
#[derive(Debug, Clone, Default)]
pub struct UserWhere {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub active: Option<bool>,
}

impl super::super::database::Wherable for UserWhere {
    fn clause(&self) -> String {
        todo!()
    }
}

impl From<&Users> for User {
    fn from(usr: &Users) -> Self {
        Self {
            id: usr.id,
            email: usr.email.clone(),
            password: Some(usr.password.clone()),
            active: usr.active,
            created_at: usr.created_at.clone(),
            updated_at: usr.updated_at.clone(),
            deleted_at: usr.deleted_at.clone(),
        }
    }
}
