use chrono::NaiveDateTime;
use sea_query::Expr;
use tables::Ratings::{AlertId, Critical, Dislike, Like, UserId};
use tokio_postgres::{Column, Error, Row};

use crate::database::{tables, Wherable};

pub use super::types::{Rating, RatingWhereClause};

impl Rating {
    pub async fn from_columns(
        row: &Row,
        columns: &[Column],
        offset: Option<usize>,
    ) -> Result<Self, Error> {
        let mut rating: Self = Default::default();

        for (index, col) in columns.iter().enumerate() {
            let name = col.name();
            let index = offset.unwrap_or(0) + index;

            match name {
                "user_id" => rating.user_id = row.try_get::<usize, i64>(index)? as i32,
                "alert_id" => rating.alert_id = row.try_get(index)?,
                "like" => rating.like = row.try_get(index)?,
                "dislike" => rating.dislike = row.try_get(index)?,
                "critical" => rating.critical = row.try_get(index)?,
                "created_at" => {
                    rating.created_at = row.try_get::<usize, NaiveDateTime>(index)?.to_string()
                }
                _ => {}
            }
        }

        Ok(rating)
    }
}

impl Wherable for RatingWhereClause {
    fn conditions<'q, Q: sea_query::QueryStatementBuilder + sea_query::ConditionalStatement>(
        &self,
        query_builder: &'q mut Q,
    ) -> &'q mut Q {
        let query = match self.user_id {
            Some(id) => query_builder.and_where(Expr::col(UserId).eq(id)),
            None => query_builder,
        };

        let query = match self.alert_id.clone() {
            Some(id) => query.and_where(Expr::col(AlertId).eq(id)),
            None => query,
        };

        let query = match self.like {
            Some(like) => query.and_where(Expr::col(Like).eq(like)),
            None => query,
        };

        let query = match self.dislike {
            Some(dislike) => query.and_where(Expr::col(Dislike).eq(dislike)),
            None => query,
        };

        let query = match self.critical {
            Some(critical) => query.and_where(Expr::col(Critical).eq(critical)),
            None => query,
        };

        query
    }

    fn clause(&self) -> queler::clause::Clause {
        todo!()
    }
}
