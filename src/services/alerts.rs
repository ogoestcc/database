use async_trait::async_trait;
use std::sync::Arc;

use crate::services::{
    traits,
    types::alerts::{self, server},
};

pub use super::alerts_mod::alerts_server::AlertsServer;

pub struct AlertsService<DB>
where
    DB: traits::Alerts,
{
    pub db_connection: Arc<DB>,
}

#[async_trait]
impl<DB> server::Alerts for AlertsService<DB>
where
    DB: traits::Alerts + 'static,
{
    async fn get_alerts(&self, request: alerts::GetInput) -> alerts::GetOutput {
        log::info!("Got request from {:?}", request.remote_addr());
        let response = alerts::handler::get(self.db_connection.clone(), request.into_inner());
        Ok(tonic::Response::new(response.await))
    }
}