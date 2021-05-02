use std::collections::HashMap;

use super::{Database, PostgresDatabase};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    database::Wherable,
    models::{
        ratings::RatingWhere,
        users::UserRatings,
        Ratings, Users,
    },
};


#[async_trait::async_trait]
impl Database<UserRatings> for PostgresDatabase {
    async fn get<W>(&self, r#where: W) -> Vec<UserRatings>
    where
        W: Wherable + Send + Sync,
    {
        let client = self.pg_pool.get().await.unwrap();

        let rating_where = RatingWhere {
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

        let mut hash = HashMap::<i64, UserRatings>::new();

        for row in &client.query(&statement, &[]).await.unwrap() {
            let rating = Ratings::from_row_ref(row).unwrap();

            let user = Users::from_row_ref(row).unwrap();

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

        let mut ratings = vec![];
        for (_, rating) in hash {
            ratings.push(rating);
        }
        ratings
    }
}
