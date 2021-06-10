use std::sync::Arc;

use crate::{
    models::{self, wherables},
    services::{services::users::contents as service, traits},
};

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

    let users: Vec<models::UserContents> = db_connection.get(r#where).await?;
    let users: Vec<service::UsersContents> = users
        .iter()
        .map(|user| service::UsersContents {
            user: From::from(&user.user),
            preferences: user.preferences.iter().map(From::from).collect(),
        })
        .collect();

    Ok(service::Response {
        metadata: service::Metadata {
            total: users.len() as u64,
            user_where: request.r#where,
        },
        users,
    })
}
