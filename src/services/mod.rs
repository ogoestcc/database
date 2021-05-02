pub mod alerts;
pub mod users;

mod handlers;

pub use alerts::AlertsService;
pub use users::UsersService;

mod database {
    tonic::include_proto!("database");
}

pub mod alerts_mod {
    pub use super::database::{get_alerts_response, GetAlertsRequest, GetAlertsResponse};

    pub use super::database::{alerts_server, Alert, AlertWhereClause};
}

pub mod ratings_mod {
    pub use super::database::Rating;
}

pub mod types {

    use super::{database, handlers};

    pub mod users {
        use super::{database, handlers};

        pub use database::{
            get_users_response::Metadata, users_server as server, GetUsersRequest as Request,
            GetUsersResponse as Response, User, UserWhereClause as WhereClause,
        };

        pub use handlers::users as handler;

        pub type GetInput = tonic::Request<Request>;
        pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;

        pub mod ratings {
            use super::{database, handlers::users};

            pub use database::{
                get_users_and_ratings_response::Metadata, GetUsersAndRatingsRequest as Request,
                GetUsersAndRatingsResponse as Response, UsersRatings,
            };

            pub use users::ratings as handler;

            pub use database::RatingWhereClause as WhereClause;

            pub type GetInput = tonic::Request<Request>;
            pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;
        }

        pub mod contents {
            use super::{database, handlers::users};

            pub use database::{
                get_users_and_contents_response::Metadata, GetUsersRequest as Request,
                GetUsersAndContentsResponse as Response, UsersContents,
            };

            pub use users::contents as handler;

            pub use database::UserWhereClause as WhereClause;

            pub type GetInput = tonic::Request<super::Request>;
            pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;
        }
    }

    pub mod alerts {
        use super::{database, handlers};

        pub use database::{
            alerts_server as server, get_alerts_response::Metadata, Alert,
            AlertWhereClause as WhereClause, GetAlertsRequest as Request,
            GetAlertsResponse as Response,
        };

        pub use handlers::alerts as handler;

        pub type GetInput = tonic::Request<Request>;
        pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;
    }

    pub mod ratings {}
}

mod traits {
    use crate::{database::Database, models};

    pub trait Users: Database<models::Users> + Database<models::UserRatings> + Send + Sync
    where
        Self: std::marker::Sized,
    {
    }
    impl<T> Users for T where T: Database<models::Users> + Database<models::UserRatings> + Send + Sync {}

    pub trait Alerts: Database<models::Users> + Database<models::Alerts> + Send + Sync
    where
        Self: std::marker::Sized,
    {
    }
    impl<T> Alerts for T where T: Database<models::Users> + Database<models::Alerts> + Send + Sync {}
}
