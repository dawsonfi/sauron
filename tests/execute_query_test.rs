use chrono::Utc;
use cw_sauron::LogClient;

#[tokio::test]
async fn should_execute_query_and_return_valid_result() {
    let log_client = LogClient::new().await;

    let queries = log_client.list_queries().await.unwrap();
    let execution = log_client
        .execute_query(
            queries.queries[0].id.clone(),
            Utc::now(),
            Utc::now(),
        )
        .await;

    assert!(execution.is_ok());
}
