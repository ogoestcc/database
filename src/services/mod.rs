pub mod alerts;
pub mod ratings;
pub mod types;
pub mod users;

mod handlers;

pub use alerts::AlertsService;
pub use ratings::RatingsService;
pub use users::UsersService;

mod protos {
    pub mod database {
        tonic::include_proto!("proto.database");
    }

    pub mod types {
        tonic::include_proto!("proto.types");
    }
}

#[allow(clippy::module_inception)]
pub mod services {
    use super::{handlers, protos, types};

    pub mod users {
        use super::protos::database;

        pub use database::{get_users::*, users_server as server};

        pub use super::handlers::users as handler;

        pub type GetInput = tonic::Request<Request>;
        pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;

        pub mod operations {
            use super::super::types::users;
            use super::database::operations;

            pub mod create {
                pub use super::{operations::Create as Request, users::User as Response};

                pub type Input = tonic::Request<Request>;
                pub type Output = Result<tonic::Response<Response>, tonic::Status>;
            }

            pub mod retrieve {
                pub use super::{operations::Retrieve as Request, users::User as Response};

                pub type Input = tonic::Request<Request>;
                pub type Output = Result<tonic::Response<Response>, tonic::Status>;
            }

            pub mod update {
                pub use super::{operations::Update as Request, users::User as Response};

                pub type Input = tonic::Request<Request>;
                pub type Output = Result<tonic::Response<Response>, tonic::Status>;
            }

            pub mod delete {
                pub use super::{operations::Delete as Request, users::User as Response};

                pub type Input = tonic::Request<Request>;
                pub type Output = Result<tonic::Response<Response>, tonic::Status>;
            }
        }

        pub mod ratings {

            pub use super::{
                super::handlers::users::ratings as handler, database::get_users_and_ratings::*,
            };

            pub type GetInput = tonic::Request<Request>;
            pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;
        }

        pub mod contents {
            pub use super::{
                super::handlers::users::contents as handler, database::get_users_and_contents::*,
                Request,
            };

            pub type GetInput = tonic::Request<Request>;
            pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;

            impl<T: Into<UsersContents> + Clone> From<&T> for UsersContents {
                fn from(base: &T) -> Self {
                    base.clone().into()
                }
            }
        }
    }

    pub mod alerts {
        use super::protos::database;

        pub use database::{alerts_server as server, get_alerts::*};

        pub use super::handlers::alerts as handler;

        pub type GetInput = tonic::Request<Request>;
        pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;

        pub mod ratings {

            pub use super::{
                super::handlers::alerts::ratings as handler, database::get_alerts_and_ratings::*,
            };

            pub type GetInput = tonic::Request<Request>;
            pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;
        }
    }

    pub mod ratings {
        use super::protos::database;

        pub use database::{get_ratings::*, ratings_server as server};

        pub use super::handlers::ratings as handler;

        pub type GetInput = tonic::Request<Request>;
        pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;
    }
}

mod traits {
    use crate::{database::Database, models};

    pub trait Users:
        Database<models::Users>
        + Database<models::UserRatings>
        + Database<models::UserContents>
        + Send
        + Sync
    where
        Self: std::marker::Sized,
    {
    }
    impl<T> Users for T where
        T: Database<models::Users>
            + Database<models::UserRatings>
            + Database<models::UserContents>
            + Send
            + Sync
    {
    }

    pub trait Alerts:
        Database<models::Alerts> + Database<models::AlertRatings> + Send + Sync
    where
        Self: std::marker::Sized,
    {
    }
    impl<T> Alerts for T where T: Database<models::Alerts> + Database<models::AlertRatings> + Send + Sync
    {}

    pub trait Ratings: Database<models::Ratings> + Send + Sync
    where
        Self: std::marker::Sized,
    {
    }
    impl<T> Ratings for T where T: Database<models::Ratings> + Send + Sync {}
}
