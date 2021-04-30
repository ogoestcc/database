use std::sync::Arc;

use services::{alerts, users};
#[cfg(not(feature = "csv_db"))]
use tokio_postgres::NoTls;
use tonic::transport::Server;

use database::Database;

mod config;
mod database;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();
    let config = config::Config::from_env().unwrap();

    #[cfg(feature = "csv_db")]
    let db_connection = database::CSVDatabase;

    #[cfg(not(feature = "csv_db"))]
    let db_connection = database::PostgresDatabase {
        pg_pool: config.postgres.create_pool(NoTls).unwrap(),
    };
    let db_connection = Arc::new(db_connection);

    let db_1 = db_connection.clone();
    let user_service = users::UsersService { db_connection };
    let alert_service = services::AlertsService { db_connection: db_1 };

    let user_service = users::UsersServer::new(user_service);
    let alert_service = alerts::AlertsServer::new(alert_service);


    let addr = format!("[::1]:{}", config.server.port);
    log::info!("Running gRPC Server at: {}", addr);
    Server::builder()
        .add_service(user_service)
        .add_service(alert_service)
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}
