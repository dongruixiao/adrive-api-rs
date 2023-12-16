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

fn main() {
    #[derive(Deserialize, Debug)]
    struct Data {
        #[serde(deserialize_with = "rfc3339_string_as_datetime")]
        time: DateTime<Utc>,
    }

    let s = r#"{"time": "2022-04-09T13:34:50.023Z"}"#;
    let v: Data = serde_json::from_str(s).unwrap();
    // let v = Utc::now();
    println!("{:#?}", v);
}
