use chrono::NaiveDateTime;
use tokio_postgres::{Column, Error, Row};

pub use super::types::Rating;

impl Rating {
    pub fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(Self {
            user_id: row.try_get::<&str, i64>("user_id")? as i32,
            alert_id: row.try_get("alert_id")?,
            like: row.try_get("like")?,
            dislike: row.try_get("dislike")?,
            critical: row.try_get("critical")?,
            created_at: row
                .try_get::<&str, NaiveDateTime>("created_at")?
                .to_string(),
            ..Default::default()
        })
    }

    pub fn from_columns(
        row: &Row,
        columns: &[Column],
        offset: Option<usize>,
    ) -> Result<Self, Error> {
        let mut rating: Self = Default::default();

        for (index, col) in columns.iter().enumerate() {
            let name = col.name();
            let index = offset.unwrap_or(0) + index;

            match name {
                "user_id" => rating.user_id = row.try_get::<usize, i64>(index)? as i32,
                "alert_id" => rating.alert_id = row.try_get(index)?,
                "like" => rating.like = row.try_get(index)?,
                "dislike" => rating.dislike = row.try_get(index)?,
                "critical" => rating.critical = row.try_get(index)?,
                "created_at" => {
                    rating.created_at = row.try_get::<usize, NaiveDateTime>(index)?.to_string()
                }
                _ => {}
            }
        }

        Ok(rating)
    }
}
