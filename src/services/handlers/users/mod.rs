use std::sync::Arc;

use service::operations::create;

use crate::services::{models::users, services::users as service, traits};

pub mod contents;
pub mod ratings;

pub async fn get<DB: traits::Users>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> Result<service::Response, tonic::Status> {
    log::debug!("Request {:?}", request);

    let users: Vec<users::User> = db_connection
        .get(request.r#where.clone().unwrap_or_default())
        .await?;

    Ok(service::Response {
        metadata: service::Metadata {
            total: users.len() as u64,
            r#where: request.r#where,
        },
        users,
    })
}

pub async fn create<DB: traits::Users>(
    db_connection: Arc<DB>,
    request: create::Request,
) -> Result<create::Response, tonic::Status> {
    log::debug!("Request {:?}", request);

    let create::Request { user } = request;

    let new_user = users::User {
        email: user.email.unwrap(),
        password: user.password,
        ..Default::default()
    };
    Ok(db_connection.create(new_user).await?)
}
