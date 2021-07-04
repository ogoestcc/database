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
        query_builder
            .and_where_option(self.user_id.map(|id| Expr::col(UserId).eq(id)))
            .and_where_option(
                self.alert_id
                    .as_ref()
                    .map(|id| Expr::col(AlertId).eq(id.to_owned())),
            )
            .and_where_option(self.like.map(|like| Expr::col(Like).eq(like)))
            .and_where_option(self.dislike.map(|dislike| Expr::col(Dislike).eq(dislike)))
            .and_where_option(self.critical.map(|c| Expr::col(Critical).eq(c)))
    }

    fn clause(&self) -> queler::clause::Clause {
        todo!()
    }
}
