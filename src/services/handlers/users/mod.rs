use std::sync::Arc;

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
