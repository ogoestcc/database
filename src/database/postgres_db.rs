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

        let stat = format!("SELECT * FROM users WHERE {}", r#where.clause());

        let statement = client.prepare(stat.as_str()).await.unwrap();

        log::debug!("Statment: {}", stat);

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

        let stat = format!("SELECT * FROM alerts WHERE {}", r#where.clause());

        let statement = client.prepare(stat.as_str()).await.unwrap();

        log::debug!("Statment: {}", stat);

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
            user_id: Some(format!(r#"usr.id"#)),
            ..rating_where
        };

        let stat = format!(
            "SELECT
                usr.*,
                rat.*
            FROM users AS usr
            INNER JOIN ratings AS rat
                ON {}
            {}",
            rating_where.clause(),
            user_where.clause()
        );

        log::debug!("Statment: {}", stat);
        let statement = client.prepare(stat.as_str()).await.unwrap();


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
