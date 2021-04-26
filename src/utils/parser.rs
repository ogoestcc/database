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