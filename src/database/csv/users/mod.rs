use async_trait::async_trait;

use super::CSVDatabase;
use crate::{
    database::{Database, Filter},
    models,
};

mod contents;
mod ratings;

#[async_trait]
impl Database<models::Users> for CSVDatabase {
    async fn get<W>(&self, r#where: W) -> Vec<models::Users>
    where
        W: Filter<models::Users> + Send + Sync,
    {
        self.get_data::<models::Users, _>(
            r"../.dataset/8Kratings100users500alerts/users.csv",
            |user| r#where.filter(user),
        )
    }
}
