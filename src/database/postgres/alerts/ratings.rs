use std::collections::HashMap;

use super::{Database, PostgresDatabase};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    database::Wherable,
    models::{
        wherables,
        alerts::AlertRatings,
        Ratings, Alerts,
    },
};


#[async_trait::async_trait]
impl Database<AlertRatings> for PostgresDatabase {
    async fn get<W>(&self, r#where: W) -> Vec<AlertRatings>
    where
        W: Wherable + Send + Sync,
    {
        let client = self.0.get().await.unwrap();

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

        let statement = client.prepare(select.to_string().as_str()).await.unwrap();

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

        let mut ratings = vec![];
        for (_, rating) in hash {
            ratings.push(rating);
        }
        ratings
    }
}
