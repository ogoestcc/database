use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[cfg(not(feature = "csv"))]
use tokio_postgres::Row;

use crate::services::types::contents::Content;

#[repr(C)]
#[derive(Default, Debug, PostgresMapper, Serialize, Deserialize, Clone)]
#[pg_mapper(table = "contents")]
pub struct Contents {
    pub id: String,
    description: Option<String>,
    #[serde(default)]
    is_product: bool,
    #[serde(default)]
    active: bool,
}

impl Into<Content> for Contents {
    fn into(self) -> Content {
        Content {
            id: self.id,
            description: self.description,
            is_product: self.is_product,
            active: self.active,
        }
    }
}

#[cfg(not(feature = "csv"))]
impl Contents {
    pub fn from_row_ref_with_prefix(row: &Row, prefix: &str) -> Self {
        Self {
            id: row.get(format!("{}id", prefix).as_str()),
            description: row.get(format!("{}description", prefix).as_str()),
            is_product: row.get(format!("{}is_product", prefix).as_str()),
            active: row.get(format!("{}active", prefix).as_str()),
        }
    }
}

impl From<&str> for Contents {
    fn from(base: &str) -> Self {
        Contents {
            id: base.to_string(),
            ..Default::default()
        }
    }
}
