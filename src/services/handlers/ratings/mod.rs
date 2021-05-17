use std::sync::Arc;

use crate::{
    models::{self, wherables},
    services::{traits, services::ratings as service, types::ratings as types},
};


pub async fn get<DB: traits::Ratings>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> service::Response {
    log::debug!("Request {:?}", request);

    let r#where: wherables::Rating = request
        .r#where
        .clone()
        .map_or(Default::default(), From::from);


    let ratings: Vec<models::Ratings> = db_connection.get(r#where).await;
    let ratings: Vec<types::Rating> = ratings.iter().map(From::from).collect();

    service::Response {
        metadata: service::Metadata {
            total: ratings.len() as u64,
            r#where: request.r#where,
        },
        ratings,
    }
}
