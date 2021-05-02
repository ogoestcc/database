use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::{
    services::types::ratings::Rating,
    utils::parser::{int_as_bool, parse_date},
};

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

impl Into<Rating> for Ratings {
    fn into(self) -> Rating {
        Rating {
            user_id: self.user_id as i32,
            alert_id: self.alert_id.clone(),
            like: self.like,
            dislike: self.dislike,
            critical: self.critical,
            created_at: self.created_at.to_string(),
        }
    }
}
