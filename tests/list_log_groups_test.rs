use cw_sauron::LogClient;

#[tokio::test]
async fn should_list_log_groups() {
    let log_client = LogClient::new().await;

    let execution = log_client.list_log_groups().await;

    assert!(execution.is_ok());
}
