pub use async_trait::async_trait;

#[cfg(feature = "csv")]
pub use csv::CSVDatabase;

#[cfg(feature = "postgres")]
pub use postgres::PostgresDatabase;
use sea_query::query::QueryStatementBuilder;

use crate::error::Error;

#[cfg(feature = "csv")]
mod csv;

#[cfg(feature = "postgres")]
mod postgres;

pub trait Wherable {
    #[cfg(feature = "postgres")]
    fn clause(&self) -> queler::clause::Clause;
    fn conditions<'q, Q: QueryStatementBuilder>(&self, query_builder: &'q mut Q) -> &'q mut Q {
        query_builder
    }
}

pub trait Filter<M> {
    fn filter(&self, value: &M) -> bool;
}

#[async_trait]
pub trait Database<Model> {
    async fn get<W>(&self, r#where: W) -> Result<Vec<Model>, Error>
    where
        W: Wherable + Filter<Model> + Send + Sync;
}
