use super::{Database, PostgresDatabase};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    error::{Error, Internal},
    models::Alerts,
};

mod ratings;

#[async_trait::async_trait]
impl Database<Alerts> for PostgresDatabase {
    async fn get<W>(&self, r#where: W) -> Result<Vec<Alerts>, Error>
    where
        W: crate::database::Wherable + Send + Sync,
    {
        let client = self.0.get().await.map_err(Internal::from)?;

        let select = queler::select::SelectBuilder::new()
            .from("alerts")
            .r#where(r#where.clause())
            .build();

        log::debug!("ALERTS SQL QUERY{}", select);

        let statement = client.prepare(select.to_string().as_str()).await.unwrap();

        Ok(client
            .query(&statement, &[])
            .await
            .map_err(Internal::from)?
            .iter()
            .map(|row| Alerts::from_row_ref(row).unwrap())
            .collect())
    }
}
