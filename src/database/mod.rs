pub use async_trait::async_trait;

#[cfg(feature = "csv")]
mod csv;

#[cfg(not(feature = "csv"))]
mod postgres;

pub trait Wherable {
    fn clause(&self) -> queler::clause::Clause;
}

pub trait Filter<M> {
    fn filter(&self, value: &M) -> bool;
}

#[cfg(feature = "csv")]
pub use csv::CSVDatabase;

#[cfg(not(feature = "csv"))]
pub use postgres::PostgresDatabase;

#[async_trait]
pub trait Database<Model> {
    async fn get<W>(&self, r#where: W) -> Vec<Model>
    where
        W: Wherable + Filter<Model> + Send  + Sync;
}
