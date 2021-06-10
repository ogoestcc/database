use sea_query::{Iden, Order, PostgresQueryBuilder, Query};

mod deserializer;

mod contents;
mod ratings;

use super::{Database, PostgresDatabase};

use crate::{
    database::Wherable,
    error::{Error, Internal},
    models::Users,
};

#[async_trait::async_trait]
impl Database<Users> for PostgresDatabase {
    async fn get<W>(&self, r#where: W) -> Result<Vec<Users>, Error>
    where
        W: Wherable + Send + Sync,
    {
        let client = self.0.get().await.map_err(Internal::from)?;

        let columns = vec![
            UsersDef::Id,
            UsersDef::Email,
            UsersDef::Password,
            UsersDef::Active,
            UsersDef::CreatedAt,
            UsersDef::UpdatedAt,
        ];

        let select = r#where
            .conditions(Query::select().from(UsersDef::Table))
            .columns(columns)
            .order_by(UsersDef::Id, Order::Desc)
            .to_string(PostgresQueryBuilder);

        log::debug!("USER SQL QUERY: {}", select);

        let statement = client
            .prepare(select.as_str())
            .await
            .map_err(Internal::from)?;

        Ok(client
            .query(&statement, &[])
            .await
            .map_err(Internal::from)?
            .iter()
            .map(|row| deserializer::user(row, None))
            .collect())
    }
}

#[derive(Iden)]
#[iden = "users"]
enum UsersDef {
    Table,
    Id,
    Email,
    Password,
    Active,
    CreatedAt,
    UpdatedAt,
}
