
use tokio_pg_mapper::FromTokioPostgresRow;

use super::{Database, PostgresDatabase};

use crate::{
    database::Wherable,
    models::Ratings,
};

#[async_trait::async_trait]
impl Database<Ratings> for PostgresDatabase {
    async fn get<W>(&self, r#where: W) -> Vec<Ratings>
    where
        W: Wherable + Send + Sync,
    {
        let client = self.0.get().await.unwrap();

        let select = queler::select::SelectBuilder::new()
            .select(&Ratings::sql_fields().split(r#","#).collect::<Vec<&str>>())
            .from(Ratings::sql_table())
            .r#where(r#where.clause())
            .build();

        log::debug!("RATINGS SQL QUERY: {}", select);

        let statement = client.prepare(select.to_string().as_str()).await.unwrap();

        client
            .query(&statement, &[])
            .await
            .unwrap()
            .iter()
            .map(|r| Ratings::from_row_ref(r).unwrap())
            .collect()
    }
}



