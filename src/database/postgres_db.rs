use async_trait::async_trait;
use tokio_pg_mapper::FromTokioPostgresRow;

use super::{
    super::models::{alerts::AlertWhere, users::UserWhere, Alerts, Users},
    Database, Wherable,
};

pub struct PostgresDatabase {
    pub pg_pool: deadpool_postgres::Pool,
}

#[async_trait]
impl Database for PostgresDatabase {
    type U = UserWhere;
    async fn users(&self, r#where: Self::U) -> Vec<Users> {
        let client = self.pg_pool.get().await.unwrap();

        let stat = format!("SELECT * FROM users {}", r#where.clause());

        let statement = client.prepare(stat.as_str()).await.unwrap();

        log::debug!("Statment: {}", stat);

        client
            .query(&statement, &[])
            .await
            .unwrap()
            .iter()
            .map(|row| Users::from_row_ref(row).unwrap())
            .collect()
    }

    type A = AlertWhere;

    async fn alerts(&self, r#where: Self::A) -> Vec<Alerts> {
        let client = self.pg_pool.get().await.unwrap();

        let stat = format!("SELECT * FROM alerts {}", r#where.clause());

        let statement = client.prepare(stat.as_str()).await.unwrap();

        log::debug!("Statment: {}", stat);

        client
            .query(&statement, &[])
            .await
            .unwrap()
            .iter()
            .map(|row| Alerts::from_row_ref(row).unwrap())
            .collect()
    }
}
