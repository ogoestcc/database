use sea_query::{Order, PostgresDriver, PostgresQueryBuilder, Query};

mod contents;
mod ratings;

use super::{Database, PostgresDatabase};

use crate::{
    database::{
        tables::{Table, Users},
        Wherable,
    },
    error::{Error, Internal},
    services::models::users::{User, UserWhereClause},
};

#[async_trait::async_trait]
impl Database<User> for PostgresDatabase {
    type WhereClause = UserWhereClause;

    async fn get(&self, r#where: Self::WhereClause) -> Result<Vec<User>, Error> {
        let client = self.0.get().await.map_err(Internal::from)?;

        let select = r#where
            .conditions(Query::select().from(Users::Table))
            .columns(Users::select().to_vec())
            .order_by(Users::Id, Order::Desc)
            .to_string(PostgresQueryBuilder);

        log::debug!("USER SQL QUERY: {}", select);

        let rows = client
            .query(select.as_str(), &[])
            .await
            .map_err(Internal::from)?;

        let mut users = Vec::<User>::with_capacity(rows.len());
        for row in &rows {
            users.push(User::from_row(row).map_err(Internal::from)?)
        }

        Ok(users.into())
    }

    async fn create(&self, user: User) -> Result<User, Error> {
        let client = self.0.get().await.map_err(Internal::from)?;

        let (sql, values) = Query::insert()
            .into_table(Users::Table)
            .returning(Query::select().columns(Users::select().to_vec()).take())
            .columns(vec![Users::Email, Users::Password])
            .values(vec![user.email.into(), user.password.into()])
            .map_err(Internal::from)?
            .build(PostgresQueryBuilder);

        let inserted_row = client
            .query_one(sql.as_str(), &values.as_params())
            .await
            .map_err(Internal::from)?;

        Ok(User::from_row(&inserted_row).map_err(Internal::from)?)
    }
}
