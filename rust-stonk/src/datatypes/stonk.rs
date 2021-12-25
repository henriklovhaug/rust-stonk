use serde::{Deserialize, Serialize};
use yahoo_finance_api::{Quote, YQuoteItem};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchStonk {
    pub stonk_name: String,
}

impl From<&YQuoteItem> for SearchStonk {
    fn from(quote: &YQuoteItem) -> SearchStonk {
        SearchStonk {
            stonk_name: quote.symbol.clone(),
        }
    }
}
