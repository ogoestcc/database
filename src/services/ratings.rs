use async_trait::async_trait;
use std::sync::Arc;

use crate::services::{
    traits,
    services::ratings::{self, server},
};

pub struct RatingsService<DB>
where
    DB: traits::Ratings,
{
    pub db_connection: Arc<DB>,
}

#[async_trait]
impl<DB> server::Ratings for RatingsService<DB>
where
    DB: traits::Ratings + 'static,
{
    async fn get_ratings(&self, request: ratings::GetInput) -> ratings::GetOutput {
        log::info!("Got request from {:?}", request.remote_addr());
        let response = ratings::handler::get(self.db_connection.clone(), request.into_inner());
        Ok(tonic::Response::new(response.await))
    }
}
