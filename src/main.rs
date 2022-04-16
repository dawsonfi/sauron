use aws_config::from_env;
use aws_sdk_cloudwatchlogs::{model::QueryStatus, Client};
use chrono::{Duration, Utc};
use cw_sauron::aws::cloudwatch::LogClient;

#[tokio::main]
async fn main() {
    let config = from_env().load().await;
    let client = Client::new(&config);

    let log_client = LogClient::new().await;

    let query = log_client
        .list_queries()
        .await
        .unwrap()
        .into_iter()
        .nth(0)
        .unwrap();

    let start_time = Utc::now() - Duration::days(2);
    let query_str = query.query_string;
    let mut query_str = query_str.split("|");
    query_str.next();
    let query_str = query_str.collect::<Vec<&str>>().join("|");

    let query_execution = client
        .start_query()
        .log_group_name(query.log_group_name.unwrap())
        .query_string(query_str)
        .start_time(start_time.timestamp())
        .end_time(Utc::now().timestamp())
        .send()
        .await
        .unwrap();

    loop {
        // get_query_results
        let results = client
            .get_query_results()
            .query_id(query_execution.query_id().unwrap())
            .send()
            .await
            .unwrap();

        if results.status().unwrap() == &QueryStatus::Complete {
            for result in results.results.unwrap() {
                for result_field in result {
                    if result_field.field().unwrap() == "@message" {
                        println!(
                            "{} -> {}",
                            result_field.field().unwrap(),
                            result_field.value().unwrap()
                        );
                    }
                }
            }
            break;
        }
    }
}
