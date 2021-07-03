use std::sync::Arc;

use crate::services::{services::ratings as service, traits, types::ratings as types};

pub async fn get<DB: traits::Ratings>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> Result<service::Response, tonic::Status> {
    log::debug!("Request {:?}", request);
    let ratings: Vec<types::Rating> = db_connection
        .get(request.r#where.clone().unwrap_or_default())
        .await?;

    Ok(service::Response {
        metadata: service::Metadata {
            total: ratings.len() as u64,
            r#where: request.r#where,
        },
        ratings,
    })
}
