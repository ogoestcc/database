use crate::{database::Wherable, models::AlertRatings};

use super::protos::types;
pub use super::services::{
    alerts::ratings::AlertsRatings,
    users::{contents::UsersContents, ratings::UsersRatings},
};

struct UsersRatingsWhere(
    Option<users::UserWhereClause>,
    Option<ratings::RatingWhereClause>,
);

impl Wherable for UsersRatingsWhere {
    fn clause(&self) -> queler::clause::Clause {
        todo!()
    }

    fn conditions<'q, Q: sea_query::QueryStatementBuilder + sea_query::ConditionalStatement>(
        &self,
        query_builder: &'q mut Q,
    ) -> &'q mut Q {
        let query = match &self.1 {
            Some(rating) => rating.conditions(query_builder),
            None => query_builder,
        };

        match &self.0 {
            Some(user) => user.conditions(query),
            None => query,
        }
    }
}

impl Wherable for UsersRatings {
    fn clause(&self) -> queler::clause::Clause {
        todo!()
    }

    fn conditions<'q, Q: sea_query::QueryStatementBuilder + sea_query::ConditionalStatement>(
        &self,
        query_builder: &'q mut Q,
    ) -> &'q mut Q {
        query_builder
    }
}

impl Wherable for AlertRatings {
    fn clause(&self) -> queler::clause::Clause {
        todo!()
    }

    fn conditions<'q, Q: sea_query::QueryStatementBuilder + sea_query::ConditionalStatement>(
        &self,
        query_builder: &'q mut Q,
    ) -> &'q mut Q {
        query_builder
    }
}

pub mod alerts;
pub mod contents;
pub mod ratings;
pub mod users;
