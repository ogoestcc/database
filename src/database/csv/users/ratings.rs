use std::collections::HashMap;

use async_trait::async_trait;

use super::super::CSVDatabase;
use crate::{
    database::{Database, Filter, Wherable},
    models::{self, UserRatings, Users},
};

#[async_trait]
impl Database<models::UserRatings> for CSVDatabase {
    async fn get<W>(&self, r#where: W) -> Vec<models::UserRatings>
    where
        W: Wherable + Filter<models::UserRatings> + Send + Sync,
    {
        let data = self.get_data::<models::Ratings, _>(
            r"../.dataset/8Kratings100users500alerts/ratings.csv",
            |_| true,
        );

        let mut hash = HashMap::<i64, UserRatings>::new();

        for rating in data {
            let mut user: Users = Default::default();

            user.id = rating.user_id;

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
        hash.iter_mut()
            .filter_map(|(_, user_rating)| {
                if r#where.filter(user_rating) {
                    Some(user_rating.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
}
