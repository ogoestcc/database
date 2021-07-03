use super::{Database, PostgresDatabase};
use sea_query::{Expr, Func, Order, PostgresQueryBuilder, Query};

use crate::{
    database::{
        tables::{self, Table},
        Wherable,
    },
    error::{Error, Internal},
    services::models::{
        alerts::{Alert, AlertWhereClause},
        ratings::Rating,
        AlertsRatings,
    },
};

lazy_static::lazy_static! {
    static ref ALERT_RATINGS_COUNT: String = Query::select()
            .from(tables::Alerts::Table)
            .expr(Func::count(Expr::cust(r#"distinct "alerts"."id""#)))
            .inner_join(
                tables::Ratings::Table,
                Expr::tbl(tables::Ratings::Table, tables::Ratings::AlertId)
                    .equals(tables::Alerts::Table, tables::Alerts::Id),
            )
            .to_string(PostgresQueryBuilder);

    static ref ALERT_RATINGS_SELECT: String = Query::select()
            .from(tables::Alerts::Table)
            .columns(tables::Alerts::select_table().to_vec())
            .columns(tables::Ratings::select_table().to_vec())
            .inner_join(
                tables::Ratings::Table,
                Expr::tbl(tables::Ratings::Table, tables::Ratings::AlertId)
                    .equals(tables::Alerts::Table, tables::Alerts::Id),
            )
            .order_by((tables::Alerts::Table, tables::Alerts::Id), Order::Desc)
            .to_string(PostgresQueryBuilder);
}

#[async_trait::async_trait]
impl Database<AlertsRatings> for PostgresDatabase {
    type WhereClause = AlertWhereClause;
    async fn get(&self, r#where: Self::WhereClause) -> Result<Vec<AlertsRatings>, Error> {
        let client = self.0.get().await.map_err(Internal::from)?;

        let select = r#where
            .conditions(Query::select().from(tables::Alerts::Table))
            .columns(tables::Alerts::select().to_vec())
            .columns(tables::Ratings::select().to_vec())
            .inner_join(
                tables::Ratings::Table,
                Expr::tbl(tables::Ratings::Table, tables::Ratings::AlertId)
                    .equals(tables::Alerts::Table, tables::Alerts::Id),
            )
            .to_string(PostgresQueryBuilder);

        log::debug!("{}", select);

        let (count, rows) = futures::join!(
            client.query_one(ALERT_RATINGS_COUNT.as_str(), &[]),
            client.query(ALERT_RATINGS_SELECT.as_str(), &[]),
        );

        let count = match count {
            Err(err) => return Err(Internal::from(err).into()),
            Ok(row) => match row.try_get::<usize, i64>(0) {
                Ok(count) if count == 0 => return Ok(vec![]),
                Ok(count) => count,
                Err(err) => return Err(Internal::from(err).into()),
            },
        };

        let mut alerts: Vec<AlertsRatings> = Vec::with_capacity(count as usize);

        for row in &rows.map_err(Internal::from)? {
            let (alerts_columns, remaining) =
                row.columns().split_at(tables::Alerts::select_table().len());

            let (rating, alert) = futures::join!(
                Rating::from_columns(row, remaining, Some(alerts_columns.len())),
                Alert::from_columns(row, alerts_columns, None),
            );
            let alert = alert.map_err(Internal::from)?;
            let rating = rating.map_err(Internal::from)?;

            match alerts.last_mut() {
                Some(a) if a.alert.id == alert.id => a.ratings.push(rating),
                _ => alerts.push(AlertsRatings {
                    alert,
                    ratings: vec![rating],
                }),
            }
        }

        Ok(alerts)
    }
}
