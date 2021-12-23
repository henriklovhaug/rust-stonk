use chrono::{DateTime, Utc};
use yahoo_finance_api as yahoo;

use crate::datatypes::stonk::Stonk;

pub async fn get_stonk_history(stonk_name: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<Stonk> {
    let provider = yahoo::YahooConnector::new();
    let resp = provider
        .get_quote_history(stonk_name, start, end)
        .await
        .unwrap();
    let quotes = resp.quotes().unwrap();
    quotes.iter().map(|quote| Stonk::from(quote)).collect()
}