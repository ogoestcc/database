use sea_query::{Expr, PostgresQueryBuilder, Query};

use super::{Database, PostgresDatabase};

use crate::{
    database::{
        tables::{self, Table},
        Wherable,
    },
    error::{Error, Internal},
    services::models::{
        alerts,
        ratings::{self, RatingWhereClause},
        users,
    },
};

#[async_trait::async_trait]
impl Database<ratings::Rating> for PostgresDatabase {
    type WhereClause = RatingWhereClause;
    async fn get(&self, r#where: Self::WhereClause) -> Result<Vec<ratings::Rating>, Error> {
        let client = self.0.get().await.map_err(Internal::from)?;

        let select = r#where
            .conditions(Query::select().from(tables::Ratings::Table))
            .columns(tables::Users::select_without_password().to_vec())
            .columns(tables::Ratings::select_table().to_vec())
            .columns(tables::Alerts::select_table().to_vec())
            .inner_join(
                tables::Users::Table,
                Expr::tbl(tables::Ratings::Table, tables::Ratings::UserId)
                    .equals(tables::Users::Table, tables::Users::Id),
            )
            .inner_join(
                tables::Alerts::Table,
                Expr::tbl(tables::Ratings::Table, tables::Ratings::AlertId)
                    .equals(tables::Alerts::Table, tables::Alerts::Id),
            )
            .to_string(PostgresQueryBuilder);

        log::debug!("RATINGS SQL QUERY: {}", select);

        let rows = client
            .query(select.as_str(), &[])
            .await
            .map_err(Internal::from)?;

        let mut result: Vec<ratings::Rating> = Vec::with_capacity(rows.len());

        for row in &rows {
            let (users_columns, remaining) =
                row.columns().split_at(tables::Users::select_table().len());
            let (rating_columns, alert_columns) =
                remaining.split_at(tables::Alerts::select_table().len());

            let (user, rating, alert) = futures::join!(
                users::User::from_columns(row, users_columns, None),
                ratings::Rating::from_columns(row, rating_columns, Some(users_columns.len())),
                alerts::Alert::from_columns(
                    row,
                    alert_columns,
                    Some(users_columns.len() + rating_columns.len()),
                )
            );

            let mut rating = rating.map_err(Internal::from)?;
            rating.user = Some(user.map_err(Internal::from)?);
            rating.alert = Some(alert.map_err(Internal::from)?);

            result.push(rating);
        }

        Ok(result)
    }
}
