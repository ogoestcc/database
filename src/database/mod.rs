
pub use async_trait::async_trait;

// use super::models::{alert::Alert, rating::Rating, user::User};
use super::models::Users;

pub trait Wherable {
    fn clause(&self) -> String;
}

pub mod csv;
#[async_trait]
pub trait Database {
    type UserWhere;
    async fn users(&self, r#where: Self::UserWhere) -> Vec<Users>;
    // async fn get_ratings(&self) -> Vec<Rating>;
}
