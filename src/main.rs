use tokio_postgres::NoTls;
use tonic::transport::Server;

mod config;
mod services;
mod database;
mod models;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();
    let config = config::Config::from_env().unwrap();

    let user_service = services::users::UsersService {
        pg_pool: config.postgres.create_pool(NoTls).unwrap(),
    };

    let user_service = services::users::UsersServer::new(user_service);

    let addr = format!("[::1]:{}", config.server.port);
    log::info!("Running gRPC Server at: {}", addr);
    Server::builder()
        .add_service(user_service)
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}
