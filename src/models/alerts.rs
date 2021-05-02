use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::{services::types::alerts as alert_service, utils::parser::parse_date};

fn default_date() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(0, 42_000_000)
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
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
        return self.id == id;
    }

    pub fn has_content(&self, content: String) -> bool {
        self.product == content || self.provider == content
    }
}

impl From<&Alerts> for alert_service::Alert {
    fn from(alert: &Alerts) -> Self {
        alert_service::Alert {
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
