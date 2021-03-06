use super::{Database, PostgresDatabase};
use sea_query::{Expr, Func, Order, PostgresQueryBuilder, Query};

use crate::{
    database::tables::{self, Table},
    error::{Error, Internal},
    services::models::{
        alerts::Alert,
        ratings::Rating,
        users::{User, UserWhereClause},
        UsersRatings,
    },
};

lazy_static::lazy_static! {
    static ref USER_RATINGS_COUNT: String = Query::select()
            .from(tables::Users::Table)
            .expr(Func::count(Expr::cust(r#"distinct "users"."id""#)))
            .inner_join(
                tables::Ratings::Table,
                Expr::tbl(tables::Ratings::Table, tables::Ratings::UserId)
                    .equals(tables::Users::Table, tables::Users::Id),
            )
            .to_string(PostgresQueryBuilder);

    static ref USER_RATING_SELECT: String = Query::select()
            .from(tables::Users::Table)
            .columns(tables::Users::select_table().to_vec())
            .columns(tables::Ratings::select_table().to_vec())
            .columns(tables::Alerts::select_table().to_vec())
            .inner_join(
                tables::Ratings::Table,
                Expr::tbl(tables::Ratings::Table, tables::Ratings::UserId)
                    .equals(tables::Users::Table, tables::Users::Id),
            )
            .inner_join(
                tables::Alerts::Table,
                Expr::tbl(tables::Ratings::Table, tables::Ratings::AlertId)
                    .equals(tables::Alerts::Table, tables::Alerts::Id),
            )
            .order_by((tables::Users::Table, tables::Users::Id), Order::Desc)
            .to_string(PostgresQueryBuilder);
}

#[async_trait::async_trait]
impl Database<UsersRatings> for PostgresDatabase {
    type WhereClause = UserWhereClause;
    async fn get(&self, _: Self::WhereClause) -> Result<Vec<UsersRatings>, Error> {
        let client = self.0.get().await.map_err(Internal::from)?;

        let (count, query) = futures::join!(
            client.query_one(USER_RATINGS_COUNT.as_str(), &[]),
            client.query(USER_RATING_SELECT.as_str(), &[])
        );

        let count = match count {
            Err(err) => return Err(Internal::from(err).into()),
            Ok(row) => match row.try_get::<usize, i64>(0) {
                Ok(count) if count == 0 => return Ok(vec![]),
                Ok(count) => count,
                Err(err) => return Err(Internal::from(err).into()),
            },
        };

        let rows = &query.map_err(Internal::from)?;

        let mut users: Vec<UsersRatings> = Vec::with_capacity(count as usize);

        for row in rows {
            let (users_columns, remaining) =
                row.columns().split_at(tables::Users::select_table().len());
            let (rating_columns, alert_columns) =
                remaining.split_at(tables::Alerts::select_table().len());

            let (user, rating, alert) = futures::join!(
                User::from_columns(row, users_columns, None),
                Rating::from_columns(row, rating_columns, Some(users_columns.len())),
                Alert::from_columns(
                    row,
                    alert_columns,
                    Some(users_columns.len() + rating_columns.len()),
                )
            );

            let user = user.map_err(Internal::from)?;
            let mut rating = rating.map_err(Internal::from)?;
            rating.alert = Some(alert.map_err(Internal::from)?);

            match users.last_mut() {
                Some(u) if u.user.id == user.id => u.ratings.push(rating),
                _ => users.push(UsersRatings {
                    user,
                    ratings: vec![rating],
                }),
            }
        }

        Ok(users)
    }
}
