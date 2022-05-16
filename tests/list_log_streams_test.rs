use cw_sauron::LogClient;

#[tokio::test]
async fn should_list_log_streams() {
    let log_client = LogClient::new().await;

    let log_groups = log_client.list_log_groups().await.unwrap();
    let execution = log_client
        .list_log_streams(log_groups.log_groups[0].name.clone())
        .await;

    assert!(execution.is_ok());
}
