pub use async_trait::async_trait;

#[cfg(feature = "csv")]
pub use csv::CSVDatabase;

#[cfg(feature = "postgres")]
pub use postgres::PostgresDatabase;
use sea_query::{ConditionalStatement, query::QueryStatementBuilder};

use crate::error::{Error, Internal, StdError};

#[cfg(feature = "csv")]
mod csv;

#[cfg(feature = "postgres")]
mod postgres;

pub trait Wherable {
    #[cfg(feature = "postgres")]
    fn clause(&self) -> queler::clause::Clause;
    fn conditions<'q, Q: QueryStatementBuilder + ConditionalStatement>(&self, query_builder: &'q mut Q) -> &'q mut Q {
        query_builder
    }
}

pub trait Filter<M> {
    fn filter(&self, value: &M) -> bool;
}

#[async_trait]
pub trait Database<Model>
where
    Model: Send + 'static,
{
    async fn get<W>(&self, r#where: W) -> Result<Vec<Model>, Error>
    where
        W: Wherable + Filter<Model> + Send + Sync;

    async fn create(&self, _: Model) -> Result<Model, Error> {
        Err(StdError("Unimplemented".to_owned())).map_err(Internal::from)?
    }
}
