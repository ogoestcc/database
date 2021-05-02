use std::sync::Arc;

use crate::{
    models::{self, wherables},
    services::{traits, types::users as service},
};

pub async fn get<DB: traits::Users>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> service::Response {
    log::debug!("Request {:?}", request);

    let r#where = request
        .r#where
        .clone()
        .map_or(Default::default(), |w| wherables::User {
            id: w.id,
            active: w.active,
            email: w.email,
        });

    let users: Vec<models::Users> = db_connection.get(r#where).await;
    let users: Vec<service::User> = users.iter().map(From::from).collect();

    service::Response {
        metadata: service::Metadata {
            total: users.len() as u64,
            r#where: request.r#where,
        },
        users,
    }
}
