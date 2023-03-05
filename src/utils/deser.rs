use chrono::{DateTime, Utc};

use serde::{Deserialize, Deserializer};
use std::result::Result;

pub fn rfc3339_string_as_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let dt = s.parse::<DateTime<Utc>>().unwrap();
    Ok(dt)
}
