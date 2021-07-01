use chrono::NaiveDateTime;
use tokio_postgres::{row::Row, Column, Error};

pub use super::types::{Alert, AlertWhereClause};

impl Alert {
    pub fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(Self {
            id: row.try_get("id")?,
            cvss_score: row.try_get("cvss_score")?,
            provider: row.try_get("provider")?,
            product: row.try_get("product")?,
            description: row.try_get("description")?,
            published_at: row
                .try_get::<&str, NaiveDateTime>("published_at")?
                .to_string(),
            updated_at: row
                .try_get::<&str, NaiveDateTime>("updated_at")?
                .to_string(),
        })
    }

    pub fn from_columns(
        row: &Row,
        columns: &[Column],
        offset: Option<usize>,
    ) -> Result<Self, Error> {
        let mut user: Self = Default::default();

        for (index, col) in columns.iter().enumerate() {
            let name = col.name();
            let index = offset.unwrap_or(0) + index;

            match name {
                "id" => user.id = row.try_get(index)?,
                "cvss_score" => user.cvss_score = row.try_get(index)?,
                "product" => user.product = row.try_get(index)?,
                "provider" => user.provider = row.try_get(index)?,
                "description" => user.description = row.try_get(index)?,
                "published_at" => {
                    user.published_at = row.try_get::<usize, NaiveDateTime>(index)?.to_string()
                }
                "updated_at" => {
                    user.updated_at = row.try_get::<usize, NaiveDateTime>(index)?.to_string()
                }
                _ => {}
            }
        }

        Ok(user)
    }
}
