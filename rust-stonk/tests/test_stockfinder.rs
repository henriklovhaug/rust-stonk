extern crate rust_stonk;

#[cfg(test)]
mod test_finder {
    use rust_stonk::stonk_finder::stonk_finder::get_latest_stonk;

    #[tokio::test]
    async fn test_find_stonk() {
        let stonks = get_latest_stonk("AAPL").await.unwrap();
        assert_ne!(stonks.len(), 0);
    }
}
