use std::time::{UNIX_EPOCH, Duration};

use yahoo_finance_api as yahoo;
use chrono::{Utc,TimeZone, DateTime};
use tokio;

#[tokio::main]
async fn main() {
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = provider.get_quote_history("AAPL", start, end).await.unwrap();
    let quotes = resp.quotes().unwrap();
    // println!("Apple's quotes in January: {:?}", quotes);
    // let max_price_1 = quotes[0].high;
    // println!("Max price in January: {}", max_price_1);
    for  quote in quotes {
        let time = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
        println!("{:?} {}", quote, time);
    }
}