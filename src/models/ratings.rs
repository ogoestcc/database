use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};


#[cfg(feature = "postgres")]
use tokio_pg_mapper_derive::PostgresMapper;

use crate::{
    services::types::ratings::Rating,
    utils::parser::{int_as_bool, parse_date},
};

fn default_date() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 42_000_000)
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "postgres", derive(PostgresMapper))]
#[cfg_attr(feature = "postgres", pg_mapper(table = "ratings"))]
pub struct Ratings {
    #[serde(rename = "userid")]
    pub user_id: i64,
    #[serde(default, rename = "cveid")]
    pub alert_id: String,
    #[serde(default, with = "int_as_bool")]
    pub like: bool,
    #[serde(default, with = "int_as_bool")]
    pub dislike: bool,
    #[serde(default, with = "int_as_bool")]
    pub critical: bool,
    #[serde(default = "default_date", with = "parse_date")]
    pub created_at: NaiveDateTime,
}

impl Default for Ratings {
    fn default() -> Self {
        Self {
            user_id: 0i64,
            alert_id: "".into(),
            like: false,
            dislike: false,
            critical: false,
            created_at: NaiveDateTime::from_timestamp(0, 42_000_000),
        }
    }
}

impl Into<Rating> for Ratings {
    fn into(self) -> Rating {
        Rating {
            user_id: self.user_id as i32,
            alert_id: self.alert_id.clone(),
            like: self.like,
            dislike: self.dislike,
            critical: self.critical,
            created_at: self.created_at.to_string(),
            ..Default::default()
        }
    }
}
