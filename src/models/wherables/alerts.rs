#[cfg(feature = "postgres")]
use queler::clause::Clause;
use sea_query::Expr;

use crate::{
    database::{Filter, Wherable},
    models,
};

#[derive(Debug, Clone, Default)]
pub struct Alert {
    pub id: Option<String>,
    pub content: Option<String>,
    pub viewed: Option<bool>,
    pub favorited: Option<bool>,
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

    #[cfg(feature = "postgres")]
    fn conditions<'q, Q>(&self, query_builder: &'q mut Q) -> &'q mut Q
    where
        Q: sea_query::QueryStatementBuilder + sea_query::ConditionalStatement,
    {
        if self.viewed.is_some() || self.favorited.is_some() {
            if let Some(viewed) = self.viewed {}
            if let Some(favorited) = self.favorited {}
        }

        query_builder
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
