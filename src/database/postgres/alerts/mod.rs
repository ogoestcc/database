use super::{Database, PostgresDatabase};
use sea_query::Expr;
use sea_query::{Order, PostgresQueryBuilder, Query, SelectStatement};

use crate::database::tables;
use crate::{
    database::{
        tables::{Alerts, AlertsViews, Table},
        Wherable,
    },
    error::{Error, Internal},
    services::{
        models::alerts::{Alert, AlertWhereClause},
        types::alerts::alert_where_clause::View,
    },
};

mod ratings;

fn get_alerts_query(view: Option<View>) -> SelectStatement {
    println!("{:?}", view);
    match view {
        Some(View { user_id, favorited }) => {
            let conditions = Expr::tbl(AlertsViews::Table, AlertsViews::AlertId)
                .equals(Alerts::Table, Alerts::Id)
                .and(Expr::tbl(AlertsViews::Table, AlertsViews::UserId).eq(user_id));

            let conditions = if let Some(fav) = favorited {
                conditions.and(Expr::col(AlertsViews::Favorited).eq(fav))
            } else {
                conditions
            };

            Query::select()
                .from(Alerts::Table)
                .column(tables::AlertsViews::Favorited)
                .inner_join(AlertsViews::Table, conditions)
                .clone()
        }
        None => Query::select().from(Alerts::Table).clone(),
    }
}

#[async_trait::async_trait]
impl Database<Alert> for PostgresDatabase {
    type WhereClause = AlertWhereClause;
    async fn get(&self, r#where: Self::WhereClause) -> Result<Vec<Alert>, Error> {
        let client = self.0.get().await.map_err(Internal::from)?;

        let select = r#where
            .conditions(&mut get_alerts_query(r#where.viewer.clone()))
            .columns(Alerts::select().to_vec())
            .order_by(Alerts::Id, Order::Desc)
            .to_string(PostgresQueryBuilder);

        log::debug!("ALERTS SQL QUERY: {}", select);

        let rows = client
            .query(select.as_str(), &[])
            .await
            .map_err(Internal::from)?;

        let mut alerts: Vec<Alert> = Vec::with_capacity(rows.len());

        for row in &rows {
            alerts.push(
                Alert::from_columns(row, row.columns(), None)
                    .await
                    .map_err(Internal::from)?,
            );
        }

        Ok(alerts)
    }
}
