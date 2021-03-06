pub use async_trait::async_trait;
pub mod tables;

#[cfg(feature = "csv")]
pub use csv::CSVDatabase;

#[cfg(feature = "postgres")]
pub use postgres::PostgresDatabase;
use sea_query::{query::QueryStatementBuilder, ConditionalStatement};

use crate::error::{Error, Internal, StdError};

#[cfg(feature = "csv")]
mod csv;

#[cfg(feature = "postgres")]
mod postgres;

pub trait Wherable {
    #[cfg(feature = "postgres")]
    fn clause(&self) -> queler::clause::Clause;
    fn conditions<'q, Q: QueryStatementBuilder + ConditionalStatement>(
        &self,
        query_builder: &'q mut Q,
    ) -> &'q mut Q {
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
    Self::WhereClause: Wherable + Send + Sync,
{
    type WhereClause;

    async fn get(&self, r#where: Self::WhereClause) -> Result<Vec<Model>, Error>;

    async fn create(&self, _: Model) -> Result<Model, Error> {
        Err(StdError("Unimplemented".to_owned())).map_err(Internal::from)?
    }
}
