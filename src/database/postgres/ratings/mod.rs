use sea_query::{Iden, PostgresQueryBuilder, Query};
use tokio_pg_mapper::FromTokioPostgresRow;

use super::{Database, PostgresDatabase};

use crate::{
    database::Wherable,
    error::{Error, Internal},
    models::Ratings,
};

#[derive(Iden)]
#[iden = "ratings"]
enum RatingsTable {
    Table,
    UserId,
    AlertId,
    Like,
    Dislike,
    Critical,
    CreatedAt,
}

#[async_trait::async_trait]
impl Database<Ratings> for PostgresDatabase {
    async fn get<W>(&self, r#where: W) -> Result<Vec<Ratings>, Error>
    where
        W: Wherable + Send + Sync,
    {
        let client = self.0.get().await.map_err(Internal::from)?;

        let select = r#where
            .conditions(Query::select().from(RatingsTable::Table))
            .columns(vec![
                RatingsTable::Table,
                RatingsTable::UserId,
                RatingsTable::AlertId,
                RatingsTable::Like,
                RatingsTable::Dislike,
                RatingsTable::Critical,
                RatingsTable::CreatedAt,
            ])
            .to_string(PostgresQueryBuilder);

        log::debug!("RATINGS SQL QUERY: {}", select);

        let statement = client
            .prepare(select.as_str())
            .await
            .map_err(Internal::from)?;

        Ok(client
            .query(&statement, &[])
            .await
            .map_err(Internal::from)?
            .iter()
            .map(|r| Ratings::from_row_ref(r).unwrap())
            .collect())
    }
}
