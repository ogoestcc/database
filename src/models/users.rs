use chrono::NaiveDateTime;
use queler::clause::Clause;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::services::users::User;

fn default_active() -> bool {
    true
}

pub fn default_date() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 42_000_000)
}

#[repr(C)]
#[derive(Debug, PostgresMapper, Serialize, Deserialize)]
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

pub struct UserRatings {
    pub user: Users,
    pub ratings: Vec<super::ratings::Ratings>,
}

#[derive(Debug, Clone, Default)]
pub struct UserWhere {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub active: Option<bool>,
}

impl super::super::database::Wherable for UserWhere {
    fn clause(&self) -> Clause {
        // let conditions = vec![
        //     self.id.is_some(),
        //     self.active.is_some(),
        //     self.email.is_some(),
        // ];

        let id = if self.id.is_some() {
            let id = self.id.unwrap();
            queler::clause! { id }
        } else {
            queler::clause! {}
        };

        let active = if self.active.is_some() {
            let active = self.active.unwrap();
            queler::clause! { active }
        } else {
            queler::clause! {}
        };

        let email = if self.email.is_some() {
            let email = self.email.clone().unwrap();
            queler::clause! { email }
        } else {
            queler::clause! {}
        };

        queler::clause!{ id, active, email }
        // let conditions = conditions
        //     .iter()
        //     .fold(0u16, |count, has| if *has { count + 1 } else { count });

        // if conditions > 0 {
        //     let mut _where = format!("WHERE");
        //     let mut apply_and = false;

        //     if self.id.is_some() {
        //         let id = self.id.unwrap();

        //         if conditions > 1 {
        //             apply_and = true;
        //         }

        //         _where = format!("{} `id` = {}", _where, id);
        //     }

        //     if self.email.is_some() {
        //         let email = self.email.clone().unwrap();
        //         _where = format!(
        //             "{} {} `email` '{}'",
        //             _where,
        //             if apply_and { "AND" } else { "" },
        //             email
        //         );

        //         if !apply_and && conditions > 1 {
        //             apply_and = true;
        //         }
        //     }

        //     if self.active.is_some() {
        //         let active = self.active.unwrap();

        //         _where = format!(
        //             "{} {} `active` = {}",
        //             _where,
        //             if apply_and { "AND" } else { "" },
        //             active
        //         );
        //     }

        //     _where
        // } else {
        //     format!("")
        // }
    }
}

impl From<&Users> for User {
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
