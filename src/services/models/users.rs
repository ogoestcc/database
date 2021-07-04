use chrono::NaiveDateTime;
use sea_query::Expr;
use tokio_postgres::{row::Row, Column, Error};

pub use super::types::{User, UserWhereClause};
use crate::database::{tables, Filter, Wherable};

impl User {
    pub fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            email: row.try_get("email")?,
            password: row.try_get("password")?,
            active: row.try_get("active")?,
            created_at: row
                .try_get::<&str, NaiveDateTime>("created_at")?
                .to_string(),
            updated_at: row
                .try_get::<&str, NaiveDateTime>("updated_at")?
                .to_string(),
            deleted_at: row
                .try_get::<&str, Option<NaiveDateTime>>("deleted_at")?
                .map(|timestamp| timestamp.to_string()),
        })
    }

    pub async fn from_columns(
        row: &Row,
        columns: &[Column],
        offset: Option<usize>,
    ) -> Result<Self, Error> {
        let mut user: Self = Default::default();

        for (index, col) in columns.iter().enumerate() {
            let name = col.name();
            let index = offset.unwrap_or(0) + index;

            match name {
                "id" => user.id = row.try_get(index)?,
                "email" => user.email = row.try_get(index)?,
                "password" => user.password = row.try_get(index)?,
                "active" => user.active = row.try_get(index)?,
                "created_at" => {
                    user.created_at = row.try_get::<usize, NaiveDateTime>(index)?.to_string()
                }
                "updated_at" => {
                    user.updated_at = row.try_get::<usize, NaiveDateTime>(index)?.to_string()
                }
                "deleted_at" => {
                    user.deleted_at = row
                        .try_get::<usize, Option<NaiveDateTime>>(index)?
                        .map(|timestamp| timestamp.to_string())
                }
                _ => {}
            }
        }

        Ok(user)
    }
}

impl Wherable for UserWhereClause {
    fn clause(&self) -> queler::clause::Clause {
        todo!()
    }

    fn conditions<'q, Q: sea_query::QueryStatementBuilder + sea_query::ConditionalStatement>(
        &self,
        query_builder: &'q mut Q,
    ) -> &'q mut Q {
        query_builder
            .and_where_option(self.id.map(|id| Expr::col(tables::Users::Id).eq(id)))
            .and_where_option(
                self.active
                    .map(|active| Expr::col(tables::Users::Active).eq(active)),
            )
            .and_where_option(
                self.email
                    .as_ref()
                    .map(|email| Expr::col(tables::Users::Email).eq(email.to_owned())),
            )
    }
}

impl Filter<User> for UserWhereClause {
    fn filter(&self, _: &User) -> bool {
        todo!()
    }
}
