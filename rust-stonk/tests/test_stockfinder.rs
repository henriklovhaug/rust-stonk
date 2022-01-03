use rust_stonk::stonk_finder::stonk_finder::get_latest_stonks;

#[tokio::test]
async fn test_find_stonk() {
    let stonks = get_latest_stonks("AAPL").await.unwrap();
    assert_ne!(stonks.len(), 0);
}
