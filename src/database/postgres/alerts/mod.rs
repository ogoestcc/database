use super::{Database, PostgresDatabase};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::models::Alerts;

mod ratings;

#[async_trait::async_trait]
impl Database<Alerts> for PostgresDatabase {
    async fn get<W>(&self, r#where: W) -> Vec<Alerts> where W: crate::database::Wherable + Send + Sync {
        let client = self.0.get().await.unwrap();

        let select = queler::select::SelectBuilder::new()
            .from("alerts")
            .r#where(r#where.clause())
            .build();

        log::debug!("{}", select);

        let statement = client.prepare(select.to_string().as_str()).await.unwrap();

        client
            .query(&statement, &[])
            .await
            .unwrap()
            .iter()
            .map(|row| Alerts::from_row_ref(row).unwrap())
            .collect()
    }
}
