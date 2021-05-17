use super::protos::types;

pub mod users {
    pub use super::types::{User, UserWhereClause as WhereClause};

    impl<T: Into<User> + Clone> From<&T> for User {
        fn from(base: &T) -> Self {
            base.to_owned().into()
        }
    }
}

pub mod alerts {
    pub use super::types::{Alert, AlertWhereClause as WhereClause};

    impl<T: Into<Alert> + Clone> From<&T> for Alert {
        fn from(base: &T) -> Self {
            base.to_owned().into()
        }
    }
}

pub mod ratings {
    pub use super::types::{Rating, RatingWhereClause as WhereClause};

    impl<T: Into<Rating> + Clone> From<&T> for Rating {
        fn from(base: &T) -> Self {
            base.to_owned().into()
        }
    }
}

pub mod contents {
    pub use super::types::{Content, UserWhereClause as WhereClause};

    impl<T: Into<Content> + Clone> From<&T> for Content {
        fn from(base: &T) -> Self {
            base.to_owned().into()
        }
    }
}
