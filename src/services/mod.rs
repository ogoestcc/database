pub mod alerts;
pub mod users;

pub use alerts::AlertsService;
pub use users::UsersService;

mod database {
    tonic::include_proto!("database");
}

pub mod users_mod {
    pub use super::database::{
        get_users_and_ratings_response, get_users_response, GetUsersAndRatingsRequest, UsersRatings,
        GetUsersAndRatingsResponse, GetUsersRequest, GetUsersResponse,
    };

    pub use super::database::{users_server, User, UserWhereClause};
}

pub mod alerts_mod {
    pub use super::database::{get_alerts_response, GetAlertsRequest, GetAlertsResponse};

    pub use super::database::{alerts_server, Alert, AlertWhereClause};
}

pub mod ratings_mod {
    pub use super::database::Rating;
}
