use async_trait::async_trait;

mod users {
    // include!(concat!(std::env!("OUT_DIR"), "/users.rs"));
    tonic::include_proto!("users");
}

pub use users::{
    get_users_response::Metadata,
    users_server::{Users, UsersServer},
    GetUsersRequest, GetUsersResponse,
};

pub struct UsersService {
    pub pg_pool: deadpool_postgres::Pool,
}

#[async_trait]
impl Users for UsersService {
    async fn get_users(
        &self,
        request: tonic::Request<GetUsersRequest>,
    ) -> Result<tonic::Response<GetUsersResponse>, tonic::Status> {
        log::info!("Got request from {:?}", request.remote_addr());

        let request = request.into_inner();
        log::debug!("Request {:?}", request);

        let reply = GetUsersResponse {
            metadata: Metadata {
                total: 0,
                r#where: request.r#where,
            },
            users: vec![],
        };

        Ok(tonic::Response::new(reply))
    }
}
