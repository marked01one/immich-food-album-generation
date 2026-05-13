// Custom date format docs: https://serde.rs/custom-date-format.html
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT_Z: &str = "%Y-%m-%dT%H:%M:%S%.fZ";
const FORMAT_UTC: &str = "%Y-%m-%dT%H:%M:%S%.f%:z";

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT_Z));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, FORMAT_Z)
        .or_else(|_| NaiveDateTime::parse_from_str(&s, FORMAT_UTC))
        .map_err(serde::de::Error::custom)?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
}
