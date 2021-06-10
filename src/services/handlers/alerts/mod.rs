use std::sync::Arc;

use crate::{
    models::{self, wherables},
    services::{services::alerts as service, traits, types::alerts as types},
};

pub mod ratings;

pub async fn get<DB: traits::Alerts>(
    db_connection: Arc<DB>,
    request: service::Request,
) -> Result<service::Response, tonic::Status> {
    log::debug!("Request {:?}", request);

    let r#where = if let Some(filters) = &request.r#where {
        wherables::Alert {
            id: filters.id.clone(),
            content: filters.content.clone(),
        }
    } else {
        Default::default()
    };

    let alerts: Vec<models::Alerts> = db_connection.get(r#where).await?;
    let alerts: Vec<types::Alert> = alerts.iter().map(From::from).collect();

    Ok(service::Response {
        metadata: service::Metadata {
            total: alerts.len() as u64,
            r#where: request.r#where,
        },
        alerts,
    })
}
