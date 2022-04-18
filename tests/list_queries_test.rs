use cw_sauron::LogClient;

#[tokio::test]
async fn should_return_queries() {
    let log_client = LogClient::new().await;

    let queries = log_client.list_queries().await;
    assert!(queries.is_ok());
}
