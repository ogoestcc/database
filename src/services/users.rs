use crate::{
    database::{self, Database},
    models::{ratings::RatingWhere, users::UserWhere},
};
use async_trait::async_trait;
use std::sync::Arc;

pub use super::users_mod::{
    UsersRatings,
    get_users_and_ratings_response, GetUsersAndRatingsResponse,
    get_users_response, users_server, GetUsersRequest, GetUsersResponse, User,
};

pub use super::users_mod::users_server::UsersServer;

pub struct UsersService {
    #[cfg(feature = "csv_db")]
    pub db_connection: Arc<database::CSVDatabase>,
    #[cfg(not(feature = "csv_db"))]
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

        let r#where = request
            .r#where
            .clone()
            .map_or(Default::default(), |w| UserWhere {
                id: w.id,
                active: w.active,
                email: w.email,
            });

        let users = self.db_connection.users(r#where).await;

        let users: Vec<User> = users.iter().map(From::from).collect();

        let reply = GetUsersResponse {
            metadata: get_users_response::Metadata {
                total: users.len() as u64,
                r#where: request.r#where,
            },
            users,
        };

        Ok(tonic::Response::new(reply))
    }

    async fn get_users_and_preferences(
        &self,
        request: tonic::Request<super::database::GetUsersAndRatingsRequest>,
    ) -> Result<tonic::Response<super::database::GetUsersAndRatingsResponse>, tonic::Status> {
        log::info!("Got request from {:?}", request.remote_addr());

        let request = request.into_inner();
        log::debug!("Request {:?}", request);

        let user_where = request
            .user_where
            .clone()
            .map_or(Default::default(), |w| UserWhere {
                id: w.id,
                active: w.active,
                email: w.email,
            });

        let rating_where = request
            .rating_where
            .clone()
            .map_or(Default::default(), |w| RatingWhere {
                user_id: w.user_id.map(|a| a.to_string()),
                alert_id: w.alert_id,
                like: w.like,
                dislike: w.dislike,
                critical: w.critical,
            });

        let users = self
            .db_connection
            .users_ratings(user_where, rating_where)
            .await;

        let users: Vec<UsersRatings> = users.iter().map(|user| {
            UsersRatings {
                user: From::from(&user.user),
                ratings: user.ratings.iter().map(From::from).collect(),
            }
        }).collect();

        let reply = GetUsersAndRatingsResponse {
            metadata: get_users_and_ratings_response::Metadata {
                total: users.len() as u64,
                user_where: request.user_where,
                rating_where: request.rating_where,
            },
            users,
        };

        Ok(tonic::Response::new(reply))
    }
}
