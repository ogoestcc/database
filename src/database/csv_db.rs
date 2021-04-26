use async_trait::async_trait;
use csv::Reader;
use serde::Deserialize;

use super::{
    super::models::{self, alerts::AlertWhere, users::UserWhere},
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

    type R = UserWhere;

    async fn users_ratings(&self, user_where: Self::U, rating_where: Self::R) -> Vec<models::users::UserRatings> {
        todo!()
    }
}
