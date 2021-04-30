use std::collections::HashMap;

use async_trait::async_trait;
use csv::Reader;
use models::users::UserRatings;
use serde::Deserialize;

use crate::models::Users;

use super::{
    super::models::{self, alerts::AlertWhere, ratings::RatingWhere, users::UserWhere},
    Database,
};

pub struct CSVDatabase;

impl CSVDatabase {
    pub fn get_data<D: for<'de> Deserialize<'de>, F: FnMut(&D) -> bool>(
        &self,
        file: &str,
        filter: &mut F,
    ) -> Vec<D> {
        log::info!("Get users infos from {}", file);

        let mut rdr = Reader::from_path(file).unwrap();
        let iter = rdr.deserialize();
        let mut vec = vec![];
        iter.fold(&mut vec, |acc, u| {
            if let Ok(d) = u {
                if filter(&d) {
                    acc.push(d)
                }
            } else {
                println!("{:?}", u.err());
            }

            acc
        });

        vec
    }
}

#[async_trait]
impl Database for CSVDatabase {
    type U = UserWhere;
    async fn users(&self, r#where: Self::U) -> Vec<models::Users> {
        let filters = r#where;

        let mut filter = |user: &models::Users| {
            let id = if let Some(id) = filters.id {
                user.same_id(id as i64)
            } else {
                true
            };

            let active = if let Some(active) = filters.active {
                user.is_active() == active
            } else {
                true
            };

            let email = if let Some(email) = filters.email.clone() {
                user.same_email(email)
            } else {
                true
            };

            id && active && email
        };

        self.get_data::<models::Users, _>(
            r"../.dataset/8Kratings100users500alerts/users.csv",
            &mut filter,
        )
    }

    type A = AlertWhere;

    async fn alerts(&self, r#where: Self::A) -> Vec<models::Alerts> {
        let filters = r#where;

        let mut filter = |alert: &models::Alerts| {
            let id = if let Some(id) = filters.id.clone() {
                alert.same_id(id)
            } else {
                true
            };

            let content = if let Some(content) = filters.content.clone() {
                alert.has_content(content)
            } else {
                true
            };

            id && content
        };

        self.get_data::<models::Alerts, _>(r"../.dataset/alerts.csv", &mut filter)
    }

    type R = RatingWhere;

    async fn users_ratings(&self, _user_where: Self::U, _rating_where: Self::R) -> Vec<UserRatings> {
        let data = self.get_data::<models::Ratings, _>(
            r"../.dataset/8Kratings100users500alerts/ratings.csv",
            &mut |_| true,
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

        let mut ratings = vec![];
        for (_, rating) in hash {
            ratings.push(rating);
        }
        ratings
    }
}
