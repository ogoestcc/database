use std::sync::Arc;
use crate::database::{self, Database};
use async_trait::async_trait;

mod users {
    // include!(concat!(std::env!("OUT_DIR"), "/users.rs"));
    tonic::include_proto!("users");
}

pub use users::{
    get_users_response::Metadata, users_server, users_server::UsersServer, GetUsersRequest,
    GetUsersResponse, User,
};

pub struct UsersService{
    #[cfg(feature="csv_db")]
    pub db_connection: Arc<database::CSVDatabase>,
    #[cfg(not(feature="csv_db"))]
    pub db_connection: Arc<database::PostgresDatabase>,
}


#[async_trait]
impl users_server::Users for UsersService {
    async fn get_users(
        &self,
        request: tonic::Request<GetUsersRequest>,
    ) -> Result<tonic::Response<GetUsersResponse>, tonic::Status> {
        log::info!("Got request from {:?}", request.remote_addr());

        let request = request.into_inner();
        log::debug!("Request {:?}", request);

        let r#where = if let Some(filters) = &request.r#where {
            crate::models::users::UserWhere {
                id: filters.id,
                active: filters.active,
                email: filters.email.clone(),
            }
        } else {
            Default::default()
        };

        let users = self.db_connection.users(r#where).await;

        let users: Vec<User> = users.iter().map(From::from).collect();

        let reply = GetUsersResponse {
            metadata: Metadata {
                total: users.len() as u64,
                r#where: request.r#where,
            },
            users: users,
        };

        Ok(tonic::Response::new(reply))
    }
}
