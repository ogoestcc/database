use lazy_static::lazy_static;

use sea_query::{Iden, Order, PostgresQueryBuilder, Query};

mod deserializer;

mod contents;
mod ratings;

use super::{Database, PostgresDatabase};

use crate::{
    database::Wherable,
    error::{Error, Internal, StdError},
    models::Users,
};

#[derive(Iden, Clone)]
#[iden = "users"]
enum UsersDef {
    Table,
    Id,
    Email,
    Password,
    Active,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

lazy_static! {
    static ref USER_COLUMNS: &'static [UsersDef] = &[
        UsersDef::Id,
        UsersDef::Email,
        UsersDef::Password,
        UsersDef::Active,
        UsersDef::CreatedAt,
        UsersDef::UpdatedAt,
        UsersDef::DeletedAt,
    ];
}

#[async_trait::async_trait]
impl Database<Users> for PostgresDatabase {
    async fn get<W>(&self, r#where: W) -> Result<Vec<Users>, Error>
    where
        W: Wherable + Send + Sync,
    {
        let client = self.0.get().await.map_err(Internal::from)?;

        let select = r#where
            .conditions(Query::select().from(UsersDef::Table))
            .columns(USER_COLUMNS.to_vec())
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

    async fn create(&self, user: Users) -> Result<Users, Error> {
        let client = self.0.get().await.map_err(Internal::from)?;

        let insert = Query::insert()
            .into_table(UsersDef::Table)
            .returning(Query::select().columns(USER_COLUMNS.to_vec()).to_owned())
            .columns(vec![UsersDef::Email, UsersDef::Password])
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
