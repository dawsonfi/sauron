use chrono::Utc;
use cw_sauron::LogClient;

#[tokio::test]
async fn should_execute_query_and_return_valid_result() {
    let log_client = LogClient::new().await;

    let execution = log_client
        .execute_query(
            "878917c1-b1a7-47ae-9177-2e11226b9323".to_string(),
            Utc::now(),
            Utc::now(),
        )
        .await;

    assert!(execution.is_ok());
}
