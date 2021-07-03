use std::collections::HashMap;

use super::{Database, PostgresDatabase};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    database::Wherable,
    error::{Error, Internal},
    models::{alerts::AlertRatings, wherables, Alerts, Ratings},
    services::models::alerts::AlertWhereClause,
};

#[async_trait::async_trait]
impl Database<AlertRatings> for PostgresDatabase {
    type WhereClause = AlertWhereClause;
    async fn get(&self, r#where: Self::WhereClause) -> Result<Vec<AlertRatings>, Error> {
        let client = self.0.get().await.map_err(Internal::from)?;

        let rating_where = wherables::Rating {
            alert_id: Some(r#":alert.id"#.to_string()),
            ..Default::default()
        };

        let select = queler::select::SelectBuilder::new()
            .from((Alerts::sql_table(), "alert"))
            .inner_join((Ratings::sql_table(), "rat"), rating_where.clause())
            .r#where(r#where.clause())
            .build();

        log::debug!("{}", select);

        let statement = client
            .prepare(select.to_string().as_str())
            .await
            .map_err(Internal::from)?;

        let mut hash = HashMap::<String, AlertRatings>::new();

        for row in &client.query(&statement, &[]).await.unwrap() {
            let rating = Ratings::from_row_ref_prefixed(row, "").unwrap();

            let alert = Alerts::from_row_ref_prefixed(row, "").unwrap();
            let alert_id = &alert.id;

            if let Some(user_rating) = hash.get_mut(alert_id) {
                user_rating.ratings.push(rating);
            } else {
                hash.insert(
                    alert.id.clone(),
                    AlertRatings {
                        alert,
                        ratings: vec![rating],
                    },
                );
            }
        }
        Ok(hash.values().map(|v| v.to_owned()).collect::<Vec<_>>())
    }
}
