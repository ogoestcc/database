use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "postgres")]
use tokio_pg_mapper_derive::PostgresMapper;

use crate::services::types::contents::Content;

#[repr(C)]
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "postgres", derive(PostgresMapper))]
#[cfg_attr(feature = "postgres", pg_mapper(table = "alerts"))]
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

impl ToString for Contents {
    fn to_string(&self) -> String {
        self.id.clone()
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
