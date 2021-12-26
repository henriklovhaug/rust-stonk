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

pub async fn get_latest_stonk(stonk_name: &str) -> Result<Vec<Stonk>, Box<dyn std::error::Error>> {
    let provider = yahoo::YahooConnector::new();
    let resp = provider
        .get_latest_quotes(stonk_name, "1m")
        .await?
        .quotes()?
        .iter()
        .map(|quote| Stonk::from(quote))
        .collect();
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[tokio::test]
    async fn test_get_stonk_history() {
        let start = Utc.ymd(2020, 1, 1).and_hms(0, 0, 0);
        let end = Utc.ymd(2020, 1, 3).and_hms(0, 0, 0);
        let stonk_name = "AAPL";
        let stonk_history = get_stonk_history(stonk_name, start, end).await.unwrap();
        assert_eq!(stonk_history.len(), 1);
    }

    #[tokio::test]
    async fn test_find_stonk_by_company_name() {
        let company_name = "apple";
        let stonk_history = find_stonk_by_company_name(company_name).await;
        assert_ne!(stonk_history.len(), 0);
    }
}
