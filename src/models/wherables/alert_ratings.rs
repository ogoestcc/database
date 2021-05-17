use crate::{
    database::{Filter, Wherable},
    models::{
        self,
        wherables::{Alert, Rating},
    },
};

#[cfg(feature = "postgres")]
use queler::clause::Clause;

#[derive(Debug, Clone, Default)]
pub struct AlertRatings(Alert, Rating);

impl Wherable for AlertRatings {
    #[cfg(feature = "postgres")]
    fn clause(&self) -> Clause {
        let alert = self.0.clause();
        let rating = self.1.clause();

        queler::clause! { alert, rating }
    }
}

impl From<Rating> for AlertRatings {
    fn from(w: Rating) -> Self {
        (Alert::default(), w).into()
    }
}

impl From<Alert> for AlertRatings {
    fn from(w: Alert) -> Self {
        (w, Rating::default()).into()
    }
}

impl<U: Into<Alert>, R: Into<Rating>> From<(U, R)> for AlertRatings {
    fn from((uw, rw): (U, R)) -> Self {
        Self(uw.into(), rw.into())
    }
}

impl<B: Into<AlertRatings> + Clone> From<&B> for AlertRatings {
    fn from(item: &B) -> Self {
        item.clone().into()
    }
}

impl Filter<models::Alerts> for AlertRatings {
    fn filter(&self, alert: &models::Alerts) -> bool {
        self.0.filter(alert)
    }
}

impl Filter<models::Ratings> for AlertRatings {
    fn filter(&self, rating: &models::Ratings) -> bool {
        self.1.filter(rating)
    }
}

impl Filter<models::AlertRatings> for AlertRatings {
    fn filter(&self, alert_rating: &models::AlertRatings) -> bool {
        self.0.filter(&alert_rating.alert)
    }
}
