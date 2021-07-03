use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "postgres")]
use tokio_pg_mapper_derive::PostgresMapper;

use crate::services::types::contents::Content;

#[repr(C)]
#[derive(Default, Debug, Serialize, Deserialize, Clone, PostgresMapper)]
#[pg_mapper(table = "contents")]
pub struct Contents {
    pub id: String,
    description: Option<String>,
    is_product: bool,
}

impl From<Contents> for Content {
    fn from(cnt: Contents) -> Self {
        Content {
            id: cnt.id,
            description: cnt.description,
            is_product: false,
            active: true,
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
