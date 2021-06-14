use std::collections::HashMap;

use queler::{clause, select};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    database::{Database, PostgresDatabase, Wherable},
    error::{Error, Internal},
    models::{users::UserContents, Contents, Users},
};

#[async_trait::async_trait]
impl<'a> Database<UserContents> for PostgresDatabase {
    async fn get<W>(&self, _: W) -> Result<Vec<UserContents>, Error>
    where
        W: Wherable + Send + Sync,
    {
        let client = self.0.get().await.map_err(Internal::from)?;

        let usr_fields = Users::sql_fields();
        let con_fields = Contents::sql_fields();

        let mut users_columns: Vec<_> = usr_fields
            .split(',')
            .map(|column| format!("usr.{}", column.trim()))
            .collect();
        let mut content_columns: Vec<_> = con_fields
            .split(", ")
            .map(|column| format!("con.{} AS con_{}", column.trim(), column.trim()))
            .collect();

        users_columns.append(&mut content_columns);

        let select = select::SelectBuilder::new()
            .select(&users_columns)
            .from((Users::sql_table(), "usr"))
            .inner_join(
                ("users_contents", "usco"),
                clause! { "usco.user_id" => ":usr.id"},
            )
            .inner_join(
                (Contents::sql_table(), "con"),
                clause! { "con.id" => ":usco.content_id"},
            )
            // .r#where(r#where.clause())
            .build();

        log::debug!("{}", select);

        let statement = client.prepare(select.to_string().as_str()).await.unwrap();

        let mut hash = HashMap::<i32, UserContents>::new();

        for row in &client.query(&statement, &[]).await.unwrap() {
            let (user, content) =
                super::deserializer::user_and_preferences(row, None, Some("con_"));

            if let Some(user_content) = hash.get_mut(&user.id) {
                user_content.preferences.push(content);
            } else {
                hash.insert(
                    user.id,
                    UserContents {
                        user,
                        preferences: vec![content],
                    },
                );
            }
        }

        Ok(hash.values().map(|v| v.to_owned()).collect())
    }
    // async fn get<W>(&self, r#where: W) -> Vec<UserContents>
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
