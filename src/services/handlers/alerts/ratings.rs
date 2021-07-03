use std::sync::Arc;

use crate::services::{services::alerts::ratings as service, traits};

pub async fn get<DB: traits::Alerts>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> Result<service::Response, tonic::Status> {
    log::debug!("Request {:?}", request);

    let alerts: Vec<service::AlertsRatings> = db_connection
        .get(request.alert_where.clone().unwrap_or_default())
        .await?;

    Ok(service::Response {
        metadata: service::Metadata {
            total: alerts.len() as u64,
            alert_where: request.alert_where,
            rating_where: request.rating_where,
        },
        alerts,
    })
}
