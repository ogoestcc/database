use std::sync::Arc;

use tonic::transport::Server;

use services::services::{
    alerts::server as alerts_server, ratings::server as rating_server, users::server as user_server,
};

mod config;
mod database;
mod error;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();
    let config = config::Config::from_env().unwrap();

    #[cfg(feature = "csv")]
    let db_connection = database::CSVDatabase;

    #[cfg(feature = "postgres")]
    let db_connection =
        database::PostgresDatabase(config.postgres.create_pool(tokio_postgres::NoTls).unwrap());

    let db_connection = Arc::new(db_connection);

    let db_clone = db_connection.clone();
    let user_service = services::UsersService { db_connection };
    let alert_service = services::AlertsService {
        db_connection: db_clone.clone(),
    };
    let ratings_service = services::RatingsService {
        db_connection: db_clone.clone(),
    };

    let user_service = user_server::UsersServer::new(user_service);
    let alert_service = alerts_server::AlertsServer::new(alert_service);
    let rating_service = rating_server::RatingsServer::new(ratings_service);

    let addr = format!("[::1]:{}", config.server.port);
    log::info!("Running gRPC Server at: {}", addr);
    Server::builder()
        .add_service(user_service)
        .add_service(alert_service)
        .add_service(rating_service)
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}
