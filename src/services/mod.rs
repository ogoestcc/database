pub mod alerts;
pub mod users;

mod handlers;

pub use alerts::AlertsService;
pub use users::UsersService;

mod database {
    tonic::include_proto!("database");
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
                get_users_and_contents_response::Metadata, GetUsersAndContentsResponse as Response,
                GetUsersRequest as Request, UsersContents,
            };

            pub use users::contents as handler;

            pub use database::UserWhereClause as WhereClause;

            pub type GetInput = tonic::Request<super::Request>;
            pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;

            impl<T: Into<UsersContents> + Clone> From<&T> for UsersContents {
                fn from(base: &T) -> Self {
                    base.clone().into()
                }
            }
        }

        impl<T: Into<User> + Clone> From<&T> for User {
            fn from(base: &T) -> Self {
                base.clone().into()
            }
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

        pub mod ratings {
            use super::{database, handlers::alerts};

            pub use database::{
                get_alerts_and_ratings_response::Metadata, AlertsRatings,
                GetAlertsAndRatingsRequest as Request, GetAlertsAndRatingsResponse as Response,
            };

            pub use alerts::ratings as handler;

            pub use database::RatingWhereClause as WhereClause;

            pub type GetInput = tonic::Request<Request>;
            pub type GetOutput = Result<tonic::Response<Response>, tonic::Status>;
        }
    }

    pub mod ratings {
        use super::database;

        pub use database::{Rating, RatingWhereClause as WhereClause};

        impl<T: Into<Rating> + Clone> From<&T> for Rating {
            fn from(base: &T) -> Self {
                base.clone().into()
            }
        }
    }

    pub mod contents {
        use super::database;

        pub use database::Content;

        impl<T: Into<Content> + Clone> From<&T> for Content {
            fn from(base: &T) -> Self {
                base.clone().into()
            }
        }
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
}
