use serde::{Deserialize, Serialize};
use yahoo_finance_api::Quote;

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
    #[allow(dead_code)]
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
