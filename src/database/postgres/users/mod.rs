use sea_query::{Order, PostgresQueryBuilder, Query};

mod deserializer;

mod contents;
mod ratings;

use super::{Database, PostgresDatabase};

use crate::{
    database::{
        tables::{Table, Users},
        Wherable,
    },
    error::{Error, Internal, StdError},
    models::Users as Model,
};

#[async_trait::async_trait]
impl Database<Model> for PostgresDatabase {
    async fn get<W>(&self, r#where: W) -> Result<Vec<Model>, Error>
    where
        W: Wherable + Send + Sync,
    {
        let client = self.0.get().await.map_err(Internal::from)?;

        let select = r#where
            .conditions(Query::select().from(Users::Table))
            .columns(Users::select().to_vec())
            .order_by(Users::Id, Order::Desc)
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

    async fn create(&self, user: Model) -> Result<Model, Error> {
        let client = self.0.get().await.map_err(Internal::from)?;

        let insert = Query::insert()
            .into_table(Users::Table)
            .returning(Query::select().columns(Users::select().to_vec()).take())
            .columns(vec![Users::Email, Users::Password])
            .values(vec![user.email().into(), user.password().into()])
            .map_err(Internal::from)?
            .to_string(PostgresQueryBuilder);

        client
            .query(insert.as_str(), &[])
            .await
            .map_err(Internal::from)?
            .first()
            .map_or_else(
                || Err(Internal::from(StdError("User not created".to_owned())).into()),
                |row| Ok(deserializer::user(row, None)),
            )
    }
}
