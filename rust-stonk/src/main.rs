use std::time::{UNIX_EPOCH, Duration};
use yahoo::Quote;
use yahoo_finance_api as yahoo;
use chrono::{Utc,TimeZone, DateTime};
use tokio;
use files::stonksaver;
mod files;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Stonk {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u64,
    pub close: f64,
    pub adjclose: f64,
}
impl Stonk {
    pub fn new(quote: Quote) -> Stonk {
        Stonk {
            timestamp: quote.timestamp,
            open: quote.open,
            high: quote.high,
            low: quote.low,
            volume: quote.volume,
            close: quote.close,
            adjclose: quote.adjclose,
        }
    }
}
impl From<&Quote> for Stonk {
    fn from(quote: &Quote) -> Stonk {
        Stonk {
            timestamp: quote.timestamp,
            open: quote.open,
            high: quote.high,
            low: quote.low,
            volume: quote.volume,
            close: quote.close,
            adjclose: quote.adjclose,
        }
    }
}
#[tokio::main]
async fn main() {
    let provider = yahoo::YahooConnector::new();
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = provider.get_quote_history("TSLA", start, end).await.unwrap();
    let quotes = resp.quotes().unwrap();
    // println!("Apple's quotes in January: {:?}", quotes);
    // let max_price_1 = quotes[0].high;
    // println!("Max price in January: {}", max_price_1);
    for quote in &quotes {
        let time = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
        println!("Tesla stonk: {:?} {}", quote, time);
    }
    let stonk = Stonk::from(&quotes[0]);
    stonksaver::create_file();
    stonksaver::save_stonk(quotes, "Tesla");
}