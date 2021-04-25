pub use async_trait::async_trait;

// use super::models::{alert::Alert, rating::Rating, user::User};
use super::models::Users;

pub trait Wherable {
    fn clause(&self) -> String;
}

mod csv_db;
mod postgres_db;

pub use csv_db::CSVDatabase;
pub use postgres_db::PostgresDatabase;

#[async_trait]
pub trait Database {
    type U;
    async fn users(&self, r#where: Self::U) -> Vec<Users>;
    // async fn get_ratings(&self) -> Vec<Rating>;
}
