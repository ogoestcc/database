use std::sync::Arc;

use crate::{
    models::{self, wherables},
    services::{traits, services::users::contents as service},
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

    let users: Vec<models::UserContents> = db_connection.get(r#where).await;
    let users: Vec<service::UsersContents> = users
        .iter()
        .map(|user| service::UsersContents {
            user: From::from(&user.user),
            preferences: user.preferences.iter().map(From::from).collect(),
        })
        .collect();

     service::Response {
        metadata: service::Metadata {
            total: users.len() as u64,
            user_where: request.r#where,
        },
        users,
    }
}
