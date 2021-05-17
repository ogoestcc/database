use async_trait::async_trait;

use super::CSVDatabase;
use crate::{
    database::{Database, Filter},
    models,
};

#[async_trait]
impl Database<models::Ratings> for CSVDatabase {
    async fn get<W>(&self, r#where: W) -> Vec<models::Ratings>
    where
        W: Filter<models::Ratings> + Send + Sync,
    {
        self.get_data::<models::Ratings, _>(
            r"../.dataset/8Kratings100users500alerts/ratings.csv",
            |rating| r#where.filter(rating),
        )
    }
}
