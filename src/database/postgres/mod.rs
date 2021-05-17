
mod users;
mod alerts;
mod ratings;

pub use super::Database;

pub struct PostgresDatabase(pub deadpool_postgres::Pool);
