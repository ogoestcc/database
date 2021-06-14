use std::collections::HashMap;

use super::{Database, PostgresDatabase};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    database::Wherable,
    error::{Error, Internal},
    models::{users::UserRatings, wherables, Ratings, Users},
};

#[async_trait::async_trait]
impl Database<UserRatings> for PostgresDatabase {
    async fn get<W>(&self, r#where: W) -> Result<Vec<UserRatings>, Error>
    where
        W: Wherable + Send + Sync,
    {
        let client = self.0.get().await.map_err(Internal::from)?;

        let rating_where = wherables::Rating {
            user_id: Some(r#":usr.id"#.to_string()),
            ..Default::default()
        };

        let select = queler::select::SelectBuilder::new()
            .from((Users::sql_table(), "usr"))
            .inner_join((Ratings::sql_table(), "rat"), rating_where.clause())
            .r#where(r#where.clause())
            .build();

        log::debug!("{}", select);

        let statement = client.prepare(select.to_string().as_str()).await.unwrap();

        let mut hash = HashMap::<i32, UserRatings>::new();

        let result = client.query(&statement, &[]).await;

        for row in result.unwrap() {
            let rating = Ratings::from_row_ref_prefixed(&row, "").unwrap();
            let user = Users::from_row_ref_prefixed(&row, "").unwrap();

            if let Some(user_rating) = hash.get_mut(&user.id) {
                user_rating.ratings.push(rating);
            } else {
                hash.insert(
                    user.id,
                    UserRatings {
                        user,
                        ratings: vec![rating],
                    },
                );
            }
        }

        Ok(hash.values().map(|v| v.to_owned()).collect())
    }
}
