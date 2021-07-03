use std::sync::Arc;

use crate::{
    models,
    services::{services::alerts::ratings as service, traits},
};

pub async fn get<DB: traits::Alerts>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> Result<service::Response, tonic::Status> {
    log::debug!("Request {:?}", request);

    let alerts: Vec<models::AlertRatings> = db_connection
        .get(request.alert_where.clone().unwrap_or_default())
        .await?;
    let alerts: Vec<service::AlertsRatings> = alerts
        .iter()
        .map(|alert| service::AlertsRatings {
            alert: From::from(&alert.alert),
            ratings: alert.ratings.iter().map(From::from).collect(),
        })
        .collect();

    Ok(service::Response {
        metadata: service::Metadata {
            total: alerts.len() as u64,
            alert_where: request.alert_where,
            rating_where: request.rating_where,
        },
        alerts,
    })
}
