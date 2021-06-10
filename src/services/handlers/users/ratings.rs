use std::sync::Arc;

use crate::{
    models::{self, wherables},
    services::{services::users::ratings as service, traits},
};

pub async fn get<DB: traits::Users>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> Result<service::Response, tonic::Status> {
    log::debug!("Request {:?}", request);

    let user_where = request
        .user_where
        .clone()
        .map_or(Default::default(), |w| wherables::User {
            id: w.id,
            active: w.active,
            email: w.email,
        });

    let rating_where = request
        .rating_where
        .clone()
        .map_or(Default::default(), |w| wherables::Rating {
            user_id: None,
            alert_id: w.alert_id,
            like: w.like,
            dislike: w.dislike,
            critical: w.critical,
        });

    let users: Vec<models::UserRatings> = db_connection
        .get(wherables::UserRatings::from((user_where, rating_where)))
        .await?;

    let users: Vec<service::UsersRatings> = users
        .iter()
        .map(|user| service::UsersRatings {
            user: From::from(&user.user),
            ratings: user.ratings.iter().map(From::from).collect(),
        })
        .collect();

    Ok(service::Response {
        metadata: service::Metadata {
            total: users.len() as u64,
            user_where: request.user_where,
            rating_where: request.rating_where,
        },
        users,
    })
}
