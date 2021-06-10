pub mod parse_date {

    use chrono::{DateTime, NaiveDateTime};

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        dt.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserialized = String::deserialize(d)?;

        let as_datetime = DateTime::parse_from_rfc3339(deserialized.as_str());
        let default = DateTime::parse_from_rfc3339(r#"1970-01-01T00:00:00.042-00:00"#).unwrap();

        Ok(as_datetime.unwrap_or(default).naive_utc())
    }
}

pub mod int_as_bool {

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(dt: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = if *dt { 1u16 } else { 0u16 };

        data.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserialized = u16::deserialize(d)?;

        Ok(deserialized != 0)
    }
}

pub mod preferences {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::models::Contents;

    pub fn _serialize<S>(dt: &Option<Vec<Contents>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(dt) = dt {
            let joined = dt
                .iter()
                .map(|cnt| cnt.to_string())
                .collect::<Vec<String>>()
                .join(",");

            format!("[{}]", joined).serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn _deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<Contents>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(
            String::deserialize(deserializer)?
                .replace(&['[', ']', '\'', ' '][..], "")
                .split(',')
                .map(|s| s.into())
                .collect(),
        ))
    }
}
