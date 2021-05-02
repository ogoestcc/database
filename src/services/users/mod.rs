use async_trait::async_trait;
use std::sync::Arc;

use crate::services::{types::users::{self as service, ratings, server}, traits};

mod users;
mod users_ratings;

pub struct UsersService<DB>
where
    DB: traits::Users,
{
    pub db_connection: Arc<DB>,
}

#[async_trait]
impl<DB> server::Users for UsersService<DB>
where
    DB: traits::Users + 'static,
{
    async fn get_users(&self, request: service::GetInput) -> service::GetOutput {
        log::info!("Got request from {:?}", request.remote_addr());
        let response = users::get(self.db_connection.clone(), request.into_inner());
        Ok(tonic::Response::new(response.await))
    }

    async fn get_users_and_ratings(
        &self,
        request: ratings::GetInput,
    ) -> ratings::GetOutput {
        let response = users_ratings::get(self.db_connection.clone(), request.into_inner());
        Ok(tonic::Response::new(response.await))}
}
