use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::{
    services::ratings_mod::Rating,
    utils::parser::{int_as_bool, parse_date},
};

pub use wherables::Rating as RatingWhere;

mod wherables;

fn default_date() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 42_000_000)
}

#[repr(C)]
#[derive(Debug, PostgresMapper, Serialize, Deserialize, Clone)]
#[pg_mapper(table = "ratings")]
pub struct Ratings {
    #[serde(rename = "userid")]
    pub user_id: i64,
    #[serde(rename = "cveid")]
    alert_id: String,
    #[serde(default, with = "int_as_bool")]
    like: bool,
    #[serde(default, with = "int_as_bool")]
    dislike: bool,
    #[serde(default, with = "int_as_bool")]
    critical: bool,
    #[serde(default = "default_date", with = "parse_date")]
    created_at: NaiveDateTime,
}

impl From<&Ratings> for Rating {
    fn from(rat: &Ratings) -> Self {
        Rating {
            user_id: rat.user_id as i32,
            alert_id: rat.alert_id.clone(),
            like: rat.like,
            dislike: rat.dislike,
            critical: rat.critical,
            created_at: rat.created_at.to_string(),
        }
    }
}
