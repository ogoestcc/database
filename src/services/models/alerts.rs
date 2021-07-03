use chrono::NaiveDateTime;
use sea_query::Expr;
use tokio_postgres::{row::Row, Column, Error};

use crate::database::{tables, Wherable};

pub use super::types::{alert_where_clause::View, Alert, AlertWhereClause};

impl Alert {
    pub async fn from_columns(
        row: &Row,
        columns: &[Column],
        offset: Option<usize>,
    ) -> Result<Self, Error> {
        let mut alert: Self = Default::default();

        for (index, col) in columns.iter().enumerate() {
            let name = col.name();
            let index = offset.unwrap_or(0) + index;

            match name {
                "id" => alert.id = row.try_get(index)?,
                "cvss_score" => alert.cvss_score = row.try_get(index)?,
                "product" => alert.product = row.try_get(index)?,
                "provider" => alert.provider = row.try_get(index)?,
                "description" => alert.description = row.try_get(index)?,
                "published_at" => {
                    alert.published_at = row.try_get::<usize, NaiveDateTime>(index)?.to_string()
                }
                "updated_at" => {
                    alert.updated_at = row.try_get::<usize, NaiveDateTime>(index)?.to_string()
                }
                "favorited" => alert.starred = row.try_get(index)?,
                "starred" => alert.starred = row.try_get(index)?,
                _ => {}
            }
        }

        Ok(alert)
    }
}

impl Wherable for AlertWhereClause {
    fn clause(&self) -> queler::clause::Clause {
        todo!()
    }

    fn conditions<'q, Q: sea_query::QueryStatementBuilder + sea_query::ConditionalStatement>(
        &self,
        query_builder: &'q mut Q,
    ) -> &'q mut Q {
        let query = match self.id.clone() {
            Some(id) => query_builder
                .and_where(Expr::col((tables::Alerts::Table, tables::Alerts::Id)).eq(id)),
            None => query_builder,
        };

        let query = match self.content.clone() {
            Some(content) => query.and_where(
                Expr::col((tables::Alerts::Table, tables::Alerts::Product))
                    .eq(content.clone())
                    .or(Expr::col((tables::Alerts::Table, tables::Alerts::Product)).eq(content)),
            ),
            None => query,
        };

        query
    }
}
