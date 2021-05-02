use queler::clause::Clause;
use crate::{database, models};

#[derive(Debug, Clone, Default)]
pub struct Rating {
    pub user_id: Option<String>,
    pub alert_id: Option<String>,
    pub like: Option<bool>,
    pub dislike: Option<bool>,
    pub critical: Option<bool>,
}


impl database::Wherable for Rating {
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

impl database::Filter<models::Ratings> for Rating {
    fn filter(&self, _: &models::Ratings) -> bool {
        false
    }
}
