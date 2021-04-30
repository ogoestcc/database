use async_trait::async_trait;
use std::collections::HashMap;
use tokio_pg_mapper::FromTokioPostgresRow;

use super::{
    super::models::{
        alerts::AlertWhere,
        ratings::RatingWhere,
        users::{UserRatings, UserWhere},
        Alerts, Ratings, Users,
    },
    Database, Wherable,
};

pub struct PostgresDatabase {
    pub pg_pool: deadpool_postgres::Pool,
}

#[async_trait]
impl Database for PostgresDatabase {
    type U = UserWhere;
    async fn users(&self, r#where: Self::U) -> Vec<Users> {
        let client = self.pg_pool.get().await.unwrap();

        let select = queler::select::SelectBuilder::new()
            .from("users")
            .r#where(r#where.clause())
            .build();

        log::debug!("{}", select);

        let statement = client.prepare(select.to_string().as_str()).await.unwrap();

        client
            .query(&statement, &[])
            .await
            .unwrap()
            .iter()
            .map(|row| Users::from_row_ref(row).unwrap())
            .collect()
    }

    type A = AlertWhere;

    async fn alerts(&self, r#where: Self::A) -> Vec<Alerts> {
        let client = self.pg_pool.get().await.unwrap();

        let select = queler::select::SelectBuilder::new()
            .from("alerts")
            .r#where(r#where.clause())
            .build();

        log::debug!("{}", select);

        let statement = client.prepare(select.to_string().as_str()).await.unwrap();

        client
            .query(&statement, &[])
            .await
            .unwrap()
            .iter()
            .map(|row| Alerts::from_row_ref(row).unwrap())
            .collect()
    }

    type R = RatingWhere;

    async fn users_ratings(
        &self,
        user_where: Self::U,
        rating_where: Self::R,
    ) -> Vec<crate::models::users::UserRatings> {
        let client = self.pg_pool.get().await.unwrap();

        let rating_where = RatingWhere {
            user_id: Some(format!(r#":usr.id"#)),
            ..rating_where
        };

        let select = queler::select::SelectBuilder::new()
            .from(("users", "usr"))
            .inner_join(("ratings", "rat"), rating_where.clause())
            .r#where(user_where.clause())
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
