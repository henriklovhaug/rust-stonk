use chrono::{DateTime, Utc};
use yahoo::YQuoteItem;
use yahoo_finance_api as yahoo;

use crate::datatypes::stonk::Stonk;

pub async fn get_stonk_history(
    stonk_name: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<Stonk>, Box<dyn std::error::Error>> {
    let provider = yahoo::YahooConnector::new();
    let resp = provider
        .get_quote_history(stonk_name, start, end)
        .await?
        .quotes()?
        .iter()
        .map(|quote| Stonk::from(quote))
        .collect();
    Ok(resp)
}

pub async fn find_stonk_by_company_name(company_name: &str) -> Vec<YQuoteItem> {
    let provider = yahoo::YahooConnector::new();
    let resp = provider.search_ticker(company_name).await.unwrap();
    resp.quotes
}
