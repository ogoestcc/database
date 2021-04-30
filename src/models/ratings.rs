use chrono::NaiveDateTime;
use queler::clause::Clause;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::{
    services::ratings_mod::Rating,
    utils::parser::{int_as_bool, parse_date},
};

use super::super::database;

fn default_date() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 42_000_000)
}

#[repr(C)]
#[derive(Debug, PostgresMapper, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default)]
pub struct RatingWhere {
    pub user_id: Option<String>,
    pub alert_id: Option<String>,
    pub like: Option<bool>,
    pub dislike: Option<bool>,
    pub critical: Option<bool>,
}

impl database::Wherable for RatingWhere {
    fn clause(&self) -> Clause {
        // let filters = &[
        //     self.user_id.is_some(),
        //     self.alert_id.is_some(),
        //     self.like.is_some(),
        //     self.dislike.is_some(),
        //     self.critical.is_some(),
        // ];

        // let filter_count = filters
        //     .iter()
        //     .fold(0u16, |has, f| if *f { has + 1 } else { has });

        let user_id = if self.user_id.is_some() {
            let user_id = self.user_id.clone().unwrap();
            queler::clause! { user_id }
        } else {
            queler::clause! {}
        };

        let alert_id = if self.alert_id.is_some() {
            let alert_id = self.alert_id.clone().unwrap();
            queler::clause! { alert_id }
        } else {
            queler::clause! {}
        };

        let like = if self.like.is_some() {
            let like = self.like.unwrap();
            queler::clause! { "\"like\"" => like }
        } else {
            queler::clause! {}
        };

        let dislike = if self.dislike.is_some() {
            let dislike = self.dislike.unwrap();
            queler::clause! { dislike }
        } else {
            queler::clause! {}
        };

        let critical = if self.critical.is_some() {
            let critical = self.critical.unwrap();
            queler::clause! { critical }
        } else {
            queler::clause! {}
        };

        queler::clause! { user_id, alert_id, like, dislike, critical }

        // if filter_count > 0 {
        //     let mut _where = format!("");
        //     let mut apply_and = false;

        //     if self.user_id.is_some() {
        //         _where = format!("{} user_id = {}", _where, self.user_id.clone().unwrap());

        //         if filter_count > 1 {
        //             apply_and = true;
        //         }
        //     }

        //     if self.alert_id.is_some() {
        //         _where = format!(
        //             "{}{} alert_id = '{}'",
        //             _where,
        //             if apply_and { "AND" } else { "" },
        //             self.alert_id.clone().unwrap()
        //         );

        //         if !apply_and && filter_count > 1 {
        //             apply_and = true;
        //         }
        //     }

        //     if self.like.is_some() {
        //         _where = format!(
        //             "{} {} rat.like = {}",
        //             _where,
        //             if apply_and { "AND" } else { "" },
        //             self.like.unwrap()
        //         );

        //         if !apply_and && filter_count > 1 {
        //             apply_and = true;
        //         }
        //     }

        //     if self.dislike.is_some() {
        //         _where = format!(
        //             "{} {} dislike = {}",
        //             _where,
        //             if apply_and { "AND" } else { "" },
        //             self.dislike.unwrap()
        //         );

        //         if !apply_and && filter_count > 1 {
        //             apply_and = true;
        //         }
        //     }

        //     if self.critical.is_some() {
        //         _where = format!(
        //             "{} {} critical = {}",
        //             _where,
        //             if apply_and { "AND" } else { "" },
        //             self.critical.unwrap()
        //         );
        //     }

        //     _where
        // } else {
        //     format!("")
        // }
    }
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
