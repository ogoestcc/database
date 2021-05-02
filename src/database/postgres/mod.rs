
mod users;
mod alerts;

pub use super::Database;

pub struct PostgresDatabase(pub deadpool_postgres::Pool);
