use super::{Database, PostgresDatabase};
use sea_query::{PostgresQueryBuilder, Query};

use crate::{
    database::{
        tables::{Alerts, Table},
        Wherable,
    },
    error::{Error, Internal},
    services::models::alerts::{Alert, AlertWhereClause},
};

mod ratings;

#[async_trait::async_trait]
impl Database<Alert> for PostgresDatabase {
    type WhereClause = AlertWhereClause;
    async fn get(&self, r#where: Self::WhereClause) -> Result<Vec<Alert>, Error> {
        let client = self.0.get().await.map_err(Internal::from)?;

        let select = r#where
            .view_join(r#where.conditions(&mut Query::select()))
            .from(Alerts::Table)
            .columns(Alerts::select().to_vec())
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
