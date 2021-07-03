use std::sync::Arc;

use crate::services::{models::UsersRatings, services::users::ratings as service, traits};

pub async fn get<DB: traits::Users>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> Result<service::Response, tonic::Status> {
    log::debug!("Request {:?}", request);

    let users: Vec<UsersRatings> = db_connection
        .get(request.user_where.clone().unwrap_or_default())
        .await?;

    Ok(service::Response {
        metadata: service::Metadata {
            total: users.len() as u64,
            user_where: request.user_where,
            rating_where: request.rating_where,
        },
        users,
    })
}
