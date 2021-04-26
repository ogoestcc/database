
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::{services::alerts_mod::Alert, utils::parser::parse_date};

fn default_date() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 42_000_000)
}

#[repr(C)]
#[derive(Debug, PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "alerts")]
pub struct Alerts {
    #[serde(rename = "cveid")]
    id: String,
    #[serde(default)]
    cvss_score: f32,
    #[serde(default)]
    description: String,
    #[serde(default)]
    provider: String,
    #[serde(default)]
    product: String,
    #[serde(
        default = "default_date",
        rename = "date_published",
        with = "parse_date"
    )]
    published_at: NaiveDateTime,
    #[serde(
        default = "default_date",
        rename = "date_modified",
        with = "parse_date"
    )]
    updated_at: NaiveDateTime,
}


impl Alerts {
    pub fn same_id(&self, id: String) -> bool {
        return self.id == id
    }

    pub fn has_content(&self, content: String) -> bool {
        self.product == content || self.provider == content
    }
}


#[derive(Debug, Clone, Default)]
pub struct AlertWhere {
    pub id: Option<String>,
    pub content: Option<String>,
}

impl super::super::database::Wherable for AlertWhere {
    fn clause(&self) -> String {
        if self.id.is_some() || self.content.is_some() {
            let mut _where = format!("WHERE");
            if self.id.is_some() {
                let id = self.id.clone().unwrap();
                _where = format!("{} id = '{}'", _where, id);
            }

            if self.content.is_some() {
                let content = self.content.clone().unwrap();
                _where = format!(
                    "{}{} (`provider` = '{}' OR `product` = '{}')",
                    _where,
                    if self.id.is_some() { " AND" } else { " " },
                    content,
                    content,
                );
            }

            _where
        } else {
            format!("")
        }
    }
}


impl From<&Alerts> for Alert {
    fn from(alert: &Alerts) -> Self {
        Alert {
            id: alert.id.clone(),
            cvss_score: Some(alert.cvss_score),
            provider: alert.provider.clone(),
            product: alert.product.clone(),
            published_at: alert.published_at.to_string(),
            updated_at: alert.updated_at.to_string(),
            description: alert.description.clone(),
        }
    }
}