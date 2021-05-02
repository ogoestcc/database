
mod ratings;

use super::{Database, PostgresDatabase};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    database::Wherable,
    models::Users,
};

#[async_trait::async_trait]
impl Database<Users> for PostgresDatabase {
    async fn get<W>(&self, r#where: W) -> Vec<Users>
    where
        W: Wherable + Send + Sync,
    {
        let client = self.0.get().await.unwrap();

        let select = queler::select::SelectBuilder::new()
            .select(&Users::sql_fields().split(r#", "#).collect::<Vec<&str>>())
            .from(Users::sql_table())
            .r#where(r#where.clause())
            .build();

        log::debug!("USER SQL QUERY: {}", select);

        let statement = client.prepare(select.to_string().as_str()).await.unwrap();

        client
            .query(&statement, &[])
            .await
            .unwrap()
            .iter()
            .map(|row| Users::from_row_ref(row).unwrap())
            .collect()
    }
}



