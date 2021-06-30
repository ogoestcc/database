use sea_query::{Expr, Order, PostgresQueryBuilder, Query};

use queler::{clause, select};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    database::{
        tables::{Contents, Table, Users, UsersContents},
        Database, PostgresDatabase, Wherable,
    },
    error::{Error, Internal},
    models::{self, users::UserContents as Model},
};

#[async_trait::async_trait]
impl<'a> Database<Model> for PostgresDatabase {
    async fn get<W>(&self, _: W) -> Result<Vec<Model>, Error>
    where
        W: Wherable + Send + Sync,
    {
        let client = self.0.get().await.map_err(Internal::from)?;

        let users_columns = Users::select_table();

        let select = Query::select()
            .from(Users::Table)
            .columns(users_columns.to_vec())
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

        log::debug!("UserContents Query: {}", select);

        let statement = client
            .prepare(select.as_str())
            .await
            .map_err(Internal::from)?;

        let rows = &client
            .query(&statement, &[])
            .await
            .map_err(Internal::from)?;

        let mut users: Vec<Model> = vec![];
        for row in rows {
            let (users_columns, content_columns) = row.columns().split_at(users_columns.len());

            let user =
                models::Users::from_columns(row, users_columns, None).map_err(Internal::from)?;
            let content =
                models::Contents::from_columns(row, content_columns, Some(users_columns.len()))
                    .map_err(Internal::from)?;

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
    // async fn get<W>(&self, r#where: W) -> Vec<Model>
    // where
    //     W: Wherable + Filter<Users> + Send + Sync,
    // {
    //     // let client = self.0.get().await.unwrap();

    //     // let rating_where = wherables::Rating {
    //     //     user_id: Some(r#":usr.id"#.to_string()),
    //     //     ..Default::default()
    //     // };

    //     // let select = queler::select::SelectBuilder::new()
    //     //     .from((Users::sql_table(), "usr"))
    //     //     .inner_join((Ratings::sql_table(), "rat"), rating_where.clause())
    //     //     .r#where(r#where.clause())
    //     //     .build();

    //     // log::debug!("{}", select);

    //     // let statement = client.prepare(select.to_string().as_str()).await.unwrap();

    //     // let mut hash = HashMap::<i64, UsersContents>::new();

    //     // for row in &client.query(&statement, &[]).await.unwrap() {
    //     //     let rating = Ratings::from_row_ref(row).unwrap();

    //     //     let user = Users::from_row_ref(row).unwrap();

    //     //     if let Some(user_rating) = hash.get_mut(&user.id) {
    //     //         user_rating.ratings.push(rating);
    //     //     } else {
    //     //         hash.insert(
    //     //             user.id,
    //     //             UsersContents {
    //     //                 user,
    //     //                 ratings: vec![rating],
    //     //             },
    //     //         );
    //     //     }
    //     // }

    //     // let mut ratings = vec![];
    //     // for (_, rating) in hash {
    //     //     ratings.push(rating);
    //     // }
    //     // ratings
    //     todo!()
    // }
}
