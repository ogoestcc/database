use std::sync::Arc;

use crate::services::{models, services::users::contents as service, traits};

pub async fn get<DB: traits::Users>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> Result<service::Response, tonic::Status> {
    log::debug!("Request {:?}", request);

    let r#where = request.r#where.clone();

    let users: Vec<models::UsersContents> = db_connection.get(r#where.unwrap_or_default()).await?;

    Ok(service::Response {
        metadata: service::Metadata {
            total: users.len() as u64,
            user_where: request.r#where,
        },
        users,
    })
}
