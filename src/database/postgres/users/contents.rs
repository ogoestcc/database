use sea_query::{Expr, Func, Order, PostgresQueryBuilder, Query};

use crate::{
    database::{
        tables::{Contents, Table, Users, UsersContents},
        Database, PostgresDatabase,
    },
    error::{Error, Internal},
    services::models::{
        contents::Content,
        users::{User, UserWhereClause},
        UsersContents as Model,
    },
};

lazy_static::lazy_static! {
    static ref USERS_CONTENTS_COUNT: String = Query::select()
            .from(Users::Table)
            .expr(Func::count(Expr::cust(r#"distinct "users"."id""#)))
            .inner_join(
                UsersContents::Table,
                Expr::tbl(UsersContents::Table, UsersContents::UserId)
                    .equals(Users::Table, Users::Id),
            )
            .inner_join(
                Contents::Table,
                Expr::tbl(UsersContents::Table, UsersContents::ContentId)
                    .equals(Contents::Table, Contents::Id),
            )
            .to_string(PostgresQueryBuilder);

    static ref USERS_CONTENTS_SELECT: String = Query::select()
            .from(Users::Table)
            .columns(Users::select_table().to_vec())
            .columns(Contents::select_table().to_vec())
            .inner_join(
                UsersContents::Table,
                Expr::tbl(UsersContents::Table, UsersContents::UserId)
                    .equals(Users::Table, Users::Id),
            )
            .inner_join(
                Contents::Table,
                Expr::tbl(UsersContents::Table, UsersContents::ContentId)
                    .equals(Contents::Table, Contents::Id),
            )
            .order_by((Users::Table, Users::Id), Order::Desc)
            .to_string(PostgresQueryBuilder);
}

#[async_trait::async_trait]
impl<'a> Database<Model> for PostgresDatabase {
    type WhereClause = UserWhereClause;
    async fn get(&self, _: Self::WhereClause) -> Result<Vec<Model>, Error> {
        let client = self.0.get().await.map_err(Internal::from)?;

        let users_columns = Users::select_table();

        let (count, query) = futures::join!(
            client.query_one(USERS_CONTENTS_COUNT.as_str(), &[]),
            client.query(USERS_CONTENTS_SELECT.as_str(), &[])
        );

        let count = match count {
            Err(err) => return Err(Internal::from(err).into()),
            Ok(row) => match row.try_get::<usize, i64>(0) {
                Ok(count) if count == 0 => return Ok(vec![]),
                Ok(count) => count,
                Err(err) => return Err(Internal::from(err).into()),
            },
        };

        let rows = &query.map_err(Internal::from)?;

        let mut users: Vec<Model> = Vec::with_capacity(count as usize);
        for row in rows {
            let (users_columns, content_columns) = row.columns().split_at(users_columns.len());

            let (user, content) = futures::join!(
                User::from_columns(row, users_columns, None),
                Content::from_columns(row, content_columns, Some(users_columns.len()))
            );

            let user = user.map_err(Internal::from)?;
            let content = content.map_err(Internal::from)?;

            match users.last_mut() {
                Some(u) if u.user.id == user.id => u.preferences.push(content),
                _ => users.push(Model {
                    user,
                    preferences: vec![content],
                }),
            }
        }

        Ok(users)
    }
}
