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

impl Contents {
    pub fn from_columns(
        row: &tokio_postgres::row::Row,
        cols: &[tokio_postgres::Column],
        offset: Option<usize>,
    ) -> Result<Self, tokio_postgres::Error> {
        let mut content: Self = Default::default();

        for (index, col) in cols.iter().enumerate() {
            let name = col.name();
            let index = offset.unwrap_or(0) + index;

            match name {
                "id" => content.id = row.try_get(index)?,
                "description" => content.description = row.try_get(index)?,
                "is_product" => content.is_product = row.try_get(index)?,
                _ => {}
            }
        }

        Ok(content)
    }
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
