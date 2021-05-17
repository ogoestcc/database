use crate::{database, models, services::types::ratings::WhereClause};

#[cfg(feature = "postgres")]
use queler::clause::Clause;

#[derive(Debug, Clone, Default)]
pub struct Rating {
    pub user_id: Option<String>,
    pub alert_id: Option<String>,
    pub like: Option<bool>,
    pub dislike: Option<bool>,
    pub critical: Option<bool>,
}

impl database::Wherable for Rating {
    #[cfg(feature = "postgres")]
    fn clause(&self) -> Clause {
        let user_id = if self.user_id.is_some() {
            let user_id = self.user_id.clone().unwrap();
            queler::clause! { user_id }
        } else {
            queler::clause! {}
        };

        let alert_id = if self.alert_id.is_some() {
            let alert_id = self.alert_id.clone().unwrap();
            queler::clause! { alert_id }
        } else {
            queler::clause! {}
        };

        let like = if self.like.is_some() {
            let like = self.like.unwrap();
            queler::clause! { "\"like\"" => like }
        } else {
            queler::clause! {}
        };

        let dislike = if self.dislike.is_some() {
            let dislike = self.dislike.unwrap();
            queler::clause! { dislike }
        } else {
            queler::clause! {}
        };

        let critical = if self.critical.is_some() {
            let critical = self.critical.unwrap();
            queler::clause! { critical }
        } else {
            queler::clause! {}
        };

        queler::clause! { user_id, alert_id, like, dislike, critical }
    }
}

impl From<WhereClause> for Rating {
    fn from(w: WhereClause) -> Self {
        Rating {
            user_id: if let Some(id) = w.user_id {
                Some(id.to_string())
            } else {
                None
            },
            alert_id: w.alert_id,
            like: w.like,
            dislike: w.dislike,
            critical: w.critical,
        }
    }
}

impl database::Filter<models::Ratings> for Rating {
    fn filter(&self, rating: &models::Ratings) -> bool {
        if let Some(user_id) = &self.user_id {
            if user_id != &rating.user_id.to_string() {
                return false;
            }
        }

        if let Some(alert_id) = &self.alert_id {
            if alert_id != &rating.alert_id {
                return false;
            }
        }

        if let Some(like) = &self.like {
            if like != &rating.like {
                return false;
            }
        }

        if let Some(dislike) = &self.dislike {
            if dislike != &rating.dislike {
                return false;
            }
        }

        if let Some(critical) = &self.critical {
            if critical != &rating.critical {
                return false;
            }
        }

        true
    }
}
