pub mod users;
pub mod alerts;

pub use users::UsersService;
pub use alerts::AlertsService;

mod database {
    // tonic::include_proto!("database");
}
