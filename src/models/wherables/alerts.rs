#[cfg(feature = "postgres")]
use queler::clause::Clause;

use crate::{
    database::{Filter, Wherable},
    models,
};

#[derive(Debug, Clone, Default)]
pub struct Alert {
    pub id: Option<String>,
    pub content: Option<String>,
}

impl Wherable for Alert {
    #[cfg(feature = "postgres")]
    fn clause(&self) -> Clause {
        let id = if self.id.is_some() {
            let id = self.id.clone().unwrap();
            queler::clause! { id }
        } else {
            queler::clause! {}
        };

        let content = if self.content.is_some() {
            let content = self.content.clone().unwrap();
            queler::or_clause! { "provider" => &content, "product" => &content }
        } else {
            queler::clause! {}
        };

        if self.id.is_some() || self.content.is_some() {
            queler::clause! { id, content } // WHERE (id = 'asdasd') AND (provider = 'asda' OR product = 'asda')
        } else {
            queler::clause! {}
        }
    }
}

impl Filter<models::Alerts> for Alert {
    fn filter(&self, alert: &models::Alerts) -> bool {
        let id = if let Some(id) = self.id.clone() {
            alert.same_id(id)
        } else {
            true
        };

        let content = if let Some(content) = self.content.clone() {
            alert.has_content(content)
        } else {
            true
        };

        id && content
    }
}
