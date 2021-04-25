use std::sync::Arc;
use async_trait::async_trait;

use super::super::{database, Database};

mod alerts {
    // include!(concat!(std::env!("OUT_DIR"), "/users.rs"));
    tonic::include_proto!("alerts");
}

pub use alerts::{
    alerts_server::{self, AlertsServer},
    get_alerts_response::Metadata,
    Alert, GetAlertsRequest, GetAlertsResponse,
};

pub struct AlertsService {
    #[cfg(feature = "csv_db")]
    pub db_connection: Arc<database::CSVDatabase>,
    #[cfg(not(feature = "csv_db"))]
    pub db_connection: Arc<database::PostgresDatabase>,
}

#[async_trait]
impl alerts_server::Alerts for AlertsService {
    async fn get_alerts(
        &self,
        request: tonic::Request<GetAlertsRequest>,
    ) -> Result<tonic::Response<GetAlertsResponse>, tonic::Status> {
        log::info!("Got request from {:?}", request.remote_addr());

        let request = request.into_inner();
        log::debug!("Request {:?}", request);

        let r#where = if let Some(filters) = &request.r#where {
            crate::models::alerts::AlertWhere {
                id: filters.id.clone(),
                content: filters.content.clone(), 
            }
        } else {
            Default::default()
        };

        let alerts = self.db_connection.alerts(r#where).await;

        let alerts: Vec<Alert> = alerts.iter().map(From::from).collect();

        let reply = GetAlertsResponse {
            metadata: Metadata {
                total: alerts.len() as u64,
                r#where: request.r#where,
            },
            alerts,
        };

        log::info!("Alerrts filtered");

        Ok(tonic::Response::new(reply))
    }
}
