mod users {
    include!(concat!(std::env!("OUT_DIR"), "/users.rs"));
    // tonic::include_proto!("users");
}

use tonic::{Request, Response, Status};
pub use users::users_server::{Users, UsersServer};
use users::{Empty, User};

pub struct UsersService {
    pub pg_pool: deadpool_postgres::Pool,
}

#[tonic::async_trait]
impl Users for UsersService {
    async fn test_user(&self, _request: Request<Empty>) -> Result<Response<User>, Status> {
        log::info!("GET_ALL_USERS");
        // println!("GetFeature = {:?}", request);

        // for feature in &self.features[..] {
        //     if feature.location.as_ref() == Some(request.get_ref()) {
        //         return Ok(Response::new(feature.clone()));
        //     }
        // }

        Ok(Response::new(User { name: "Otávio".into() }))
    }
}
