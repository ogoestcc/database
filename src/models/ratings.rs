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

impl From<Ratings> for Rating {
    fn from(rating: Ratings) -> Self {
        Rating {
            user_id: rating.user_id as i32,
            alert_id: rating.alert_id.clone(),
            like: rating.like,
            dislike: rating.dislike,
            critical: rating.critical,
            created_at: rating.created_at.to_string(),
            ..Default::default()
        }
    }
}
