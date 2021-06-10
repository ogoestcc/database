mod alerts;
mod ratings;
mod users;

pub use super::Database;

pub struct PostgresDatabase(pub deadpool_postgres::Pool);
