use std::{
    str::FromStr,
    time::{Duration, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct APIStonk {
    pub stonk_name: String,
    pub start: u64,
    pub end: u64,
}

pub struct LocalApiStonk {
    pub stonk_name: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl FromStr for APIStonk {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stonk_name = s
            .split_ascii_whitespace()
            .nth(1)
            .ok_or("no stonk name provided")?;
        let start = s
            .split_ascii_whitespace()
            .nth(2)
            .ok_or("no start date provided")?;
        let end = s
            .split_ascii_whitespace()
            .nth(3)
            .ok_or("no end date provided")?;
        Ok(APIStonk {
            stonk_name: stonk_name.to_string(),
            start: start.parse::<u64>().map_err(|e| e.to_string())?,
            end: end.parse::<u64>().map_err(|e| e.to_string())?,
        })
    }
}

impl From<&APIStonk> for LocalApiStonk {
    fn from(api_stonk: &APIStonk) -> Self {
        LocalApiStonk {
            stonk_name: api_stonk.stonk_name.clone(),
            start: DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(api_stonk.start)),
            end: DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(api_stonk.end)),
        }
    }
}
