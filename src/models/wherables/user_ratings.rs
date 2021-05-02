use crate::{
    database::{Filter, Wherable},
    models::{
        self,
        wherables::{Rating, User},
    },
    services::types::ratings::WhereClause,
};
use queler::clause::Clause;

#[derive(Debug, Clone, Default)]
pub struct UserRatings(User, Rating);

impl Wherable for UserRatings {
    fn clause(&self) -> Clause {
        let user = self.0.clause();
        let rating = self.1.clause();

        queler::clause! { user, rating }
    }
}

impl From<Rating> for UserRatings {
    fn from(w: Rating) -> Self {
        (User::default(), w).into()
    }
}

impl From<User> for UserRatings {
    fn from(w: User) -> Self {
        (w, Rating::default()).into()
    }
}


impl From<WhereClause> for UserRatings {
    fn from(w: WhereClause) -> Self {
        (User::default(), Rating::from(w)).into()
    }
}

impl<U: Into<User>, R: Into<Rating>> From<(U, R)> for UserRatings {
    fn from((uw, rw): (U, R)) -> Self {
        Self(uw.into(), rw.into())
    }
}

impl<B: Into<UserRatings> + Clone> From<&B> for UserRatings {
    fn from(item: &B) -> Self {
        item.clone().into()
    }
}

impl Filter<models::Users> for UserRatings {
    fn filter(&self, user: &models::Users) -> bool {
        self.0.filter(user)
    }
}

impl Filter<models::Ratings> for UserRatings {
    fn filter(&self, rating: &models::Ratings) -> bool {
        self.1.filter(rating)
    }
}

impl Filter<models::UserRatings> for UserRatings {
    fn filter(&self, user_rating: &models::UserRatings) -> bool {
        let valid_user = self.0.filter(&user_rating.user);

        valid_user
    }
}
