use async_trait::async_trait;
use std::sync::Arc;

use crate::services::{
    services::users::{self, contents, operations, ratings, server},
    traits,
};

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
    async fn get_users(&self, request: users::GetInput) -> users::GetOutput {
        log::info!("Got request from {:?}", request.remote_addr());
        let response = users::handler::get(self.db_connection.clone(), request.into_inner());
        Ok(tonic::Response::new(response.await?))
    }

    async fn get_users_and_ratings(&self, request: ratings::GetInput) -> ratings::GetOutput {
        let response = ratings::handler::get(self.db_connection.clone(), request.into_inner());
        Ok(tonic::Response::new(response.await?))
    }

    async fn get_users_and_contents(&self, request: contents::GetInput) -> contents::GetOutput {
        let response = contents::handler::get(self.db_connection.clone(), request.into_inner());
        Ok(tonic::Response::new(response.await?))
    }

    //* CRUD

    async fn create(&self, request: operations::create::Input) -> operations::create::Output {
        let response = users::handler::create(self.db_connection.clone(), request.into_inner());
        Ok(tonic::Response::new(response.await?))
    }

    async fn retrieve(&self, _: operations::retrieve::Input) -> operations::retrieve::Output {
        todo!()
    }

    async fn update(&self, _: operations::update::Input) -> operations::update::Output {
        todo!()
    }

    async fn delete(&self, _: operations::delete::Input) -> operations::delete::Output {
        todo!()
    }
}
