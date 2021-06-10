use std::sync::Arc;

use service::operations::create;

use crate::{
    models::{self, wherables},
    services::{services::users as service, traits, types::users as types},
};

pub mod contents;
pub mod ratings;

pub async fn get<DB: traits::Users>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> Result<service::Response, tonic::Status> {
    log::debug!("Request {:?}", request);

    let r#where = request
        .r#where
        .clone()
        .map_or(Default::default(), |w| wherables::User {
            id: w.id,
            active: w.active,
            email: w.email,
        });

    let users: Vec<models::Users> = db_connection.get(r#where).await?;
    let users: Vec<types::User> = users.iter().map(From::from).collect();

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

    let mut new_user = models::Users::default();
    new_user.set_email(user.email.unwrap());
    new_user.set_password(user.password.unwrap());

    Ok(db_connection.create(new_user).await?.into())
}
