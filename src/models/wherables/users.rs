#[cfg(feature = "postgres")]
use queler::clause::Clause;

use crate::{
    database::{Filter, Wherable},
    models,
};

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub active: Option<bool>,
}

impl Wherable for User {
    #[cfg(feature = "postgres")]
    fn clause(&self) -> Clause {
        let id = if self.id.is_some() {
            let id = self.id.unwrap();
            queler::clause! { id }
        } else {
            queler::clause! {}
        };

        let active = if self.active.is_some() {
            let active = self.active.unwrap();
            queler::clause! { active }
        } else {
            queler::clause! {}
        };

        let email = if self.email.is_some() {
            let email = self.email.clone().unwrap();
            queler::clause! { email }
        } else {
            queler::clause! {}
        };

        queler::clause! { id, active, email }
    }
}

impl Filter<models::Users> for User {
    fn filter(&self, user: &models::Users) -> bool {
        let id = if let Some(id) = self.id {
            user.same_id(id as i64)
        } else {
            true
        };

        let active = if let Some(active) = self.active {
            user.is_active() == active
        } else {
            true
        };

        let email = if let Some(email) = self.email.clone() {
            user.same_email(email)
        } else {
            true
        };

        id && active && email
    }
}

impl Filter<models::UserContents> for User {
    fn filter(&self, user_content: &models::UserContents) -> bool {
        self.filter(&user_content.user)
    }
}
