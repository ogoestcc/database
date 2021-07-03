use std::sync::Arc;

use crate::services::{models::alerts::Alert, services::alerts as service, traits};

pub mod ratings;

pub async fn get<DB: traits::Alerts>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> Result<service::Response, tonic::Status> {
    log::debug!("Request {:?}", request);

    let alerts: Vec<Alert> = db_connection
        .get(request.r#where.clone().unwrap_or_default())
        .await?;

    Ok(service::Response {
        metadata: service::Metadata {
            total: alerts.len() as u64,
            r#where: request.r#where,
        },
        alerts,
    })
}
