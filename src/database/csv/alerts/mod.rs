use async_trait::async_trait;

use super::CSVDatabase;
use crate::{
    database::{Database, Filter},
    models,
};


#[async_trait]
impl Database<models::Alerts> for CSVDatabase {
    async fn get<W>(&self, r#where: W) -> Vec<models::Alerts>
    where
        W: Filter<models::Alerts> + Send + Sync,
    {
        self.get_data::<models::Alerts, _>(
            r"../.dataset/alerts.csv",
            |alert| r#where.filter(alert),
        )
    }
}