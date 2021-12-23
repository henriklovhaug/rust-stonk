use chrono::{DateTime, TimeZone, Utc};
use files::stonksaver;
use std::time::{Duration, UNIX_EPOCH};
use tokio;
use yahoo::Quote;
use yahoo_finance_api as yahoo;
mod files;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    let resp = provider
        .get_quote_history("TSLA", start, end)
        .await
        .unwrap();
    let quotes = resp.quotes().unwrap();
    let stonk: Vec<Stonk> = quotes.iter().map(|quote| Stonk::from(quote)).collect();
    stonk_printer(&stonk, "TSLA");
    stonksaver::save_stonk(stonk);
}

fn stonk_printer(stonks: &Vec<Stonk>, stonk_name: &str) {
    for stonk in stonks {
        //let time = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(stonk.timestamp));
        println!("{}:", stonk_name);
        println!("\t Opening price: {} \n
                  \t Dayly high: {} \n
                  ", stonk.open, stonk.high);
    }
}
