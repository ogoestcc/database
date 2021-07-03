use tokio_postgres::{Column, Error, Row};

pub use super::types::Content;

impl Content {
    pub async fn from_columns(
        row: &Row,
        columns: &[Column],
        offset: Option<usize>,
    ) -> Result<Self, Error> {
        let mut content: Self = Default::default();

        for (index, col) in columns.iter().enumerate() {
            let name = col.name();
            let index = offset.unwrap_or(0) + index;

            match name {
                "id" => content.id = row.try_get(index)?,
                "description" => content.description = row.try_get(index)?,
                "is_product" => content.is_product = row.try_get(index)?,
                "active" => content.active = row.try_get(index)?,
                _ => {}
            }
        }

        Ok(content)
    }
}
