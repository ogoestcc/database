use serde::Deserialize;
use std::collections::HashMap;


use async_trait::async_trait;

use super::super::CSVDatabase;
use crate::{
    database::{Database, Filter, Wherable},
    models::{self, AlertRatings, Alerts, Ratings},
};


#[derive(Debug, Deserialize)]
struct AlertRatingCSV {
    #[serde(flatten)]
    alert: Alerts,
    #[serde(flatten)]
    rating: Ratings
}


#[async_trait]
impl Database<models::AlertRatings> for CSVDatabase {
    async fn get<W>(&self, r#where: W) -> Vec<models::AlertRatings>
    where
        W: Wherable + Filter<models::AlertRatings> + Send + Sync,
    {
        let data = self.get_data::<AlertRatingCSV, _>(
            r"../.dataset/8Kratings100users500alerts/ratings.csv",
            |_| true,
        );

        let mut hash = HashMap::<String, AlertRatings>::new();

        for rating in data {
            let alert_id = &rating.alert.id;
            if let Some(user_rating) = hash.get_mut(alert_id) {
                user_rating.ratings.push(rating.rating);
            } else {
                hash.insert(
                    alert_id.to_owned(),
                    AlertRatings {
                        alert: rating.alert,
                        ratings: vec![rating.rating],
                    },
                );
            }
        }
        hash.iter_mut()
            .filter_map(|(_, rating)| {
                if r#where.filter(rating) {
                    Some(rating.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
}
