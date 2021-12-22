use yahoo_finance_api as yahoo;
use std::time::{Duration, UNIX_EPOCH};
use chrono::prelude::*;
use tokio_test;

fn main() {
    let provider = yahoo::YahooConnector::new();
    // get the latest quotes in 1 minute intervals
    let response = tokio_test::block_on(provider.get_latest_quotes("AAPL", "1m")).unwrap();
    // extract just the latest valid quote summery
    // including timestamp,open,close,high,low,volume
    let quote = response.last_quote().unwrap();
    let time: DateTime<Utc> =
        DateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
    println!("At {} quote price of Apple was {}", time.to_rfc3339(), quote.close);
}