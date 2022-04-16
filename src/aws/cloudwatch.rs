use crate::aws::model::{LogQueryInfo, LogQueryInfoBuilder};
#[double]
use internal::CloudWatchClient;
use mockall_double::double;
use std::error::Error;

#[allow(dead_code)]
mod internal {

    use aws_config::from_env;
    use aws_sdk_cloudwatchlogs::{
        error::DescribeQueriesError, output::DescribeQueriesOutput, Client,
    };
    use aws_smithy_http::result::SdkError;

    pub struct CloudWatchClient {
        client: Client,
    }

    #[cfg_attr(test, mockall::automock)]
    impl CloudWatchClient {
        pub async fn new() -> Self {
            let config = from_env().load().await;
            CloudWatchClient {
                client: Client::new(&config),
            }
        }

        pub async fn describe_queries(
            self,
        ) -> Result<DescribeQueriesOutput, SdkError<DescribeQueriesError>> {
            self.client.describe_queries().send().await
        }
    }
}

pub struct LogClient {
    client: CloudWatchClient,
}

impl LogClient {
    pub async fn new() -> Self {
        LogClient {
            client: CloudWatchClient::new().await,
        }
    }

    pub async fn list_queries(self) -> Result<Vec<LogQueryInfo>, Box<dyn Error>> {
        let queries = self
            .client
            .describe_queries()
            .await?
            .queries
            .unwrap_or_default()
            .into_iter()
            .map(|query| {
                LogQueryInfoBuilder::default()
                    .query_id(query.query_id().unwrap().to_string())
                    .query_string(query.query_string().unwrap().to_string())
                    .log_group_name(
                        query
                            .log_group_name()
                            .map(|log_group_name| log_group_name.to_string()),
                    )
                    .build()
                    .unwrap()
            })
            .collect();

        Ok(queries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_sdk_cloudwatchlogs::error::{
        invalid_parameter_exception::Builder as InvalidParameterExceptionBuilder,
        DescribeQueriesError, DescribeQueriesErrorKind,
    };
    use aws_sdk_cloudwatchlogs::model::query_info::Builder as QueryInfoBuilder;
    use aws_sdk_cloudwatchlogs::output::describe_queries_output::Builder as DescribeQueriesOutputBuilder;
    use aws_sdk_cloudwatchlogs::output::DescribeQueriesOutput;
    use aws_smithy_http::result::SdkError;
    use aws_smithy_types::error::Builder as ErrorBuilder;

    #[tokio::test]
    async fn should_return_queries() {
        let mut result = Some(Ok(DescribeQueriesOutputBuilder::default()
            .queries(
                QueryInfoBuilder::default()
                    .query_id("dinosaur")
                    .query_string("fields dinosaur")
                    .log_group_name("dinosaur::logs")
                    .build(),
            )
            .queries(
                QueryInfoBuilder::default()
                    .query_id("dinosaur")
                    .query_string("fields dinosaur")
                    .build(),
            )
            .build()));

        let mut cw_client = CloudWatchClient::default();
        cw_client
            .expect_describe_queries()
            .times(1)
            .returning(move || result.take().unwrap());

        let client = LogClient { client: cw_client };

        let queries = client.list_queries().await.unwrap();

        assert_eq!(
            queries,
            vec![
                LogQueryInfoBuilder::default()
                    .query_id("dinosaur".to_string())
                    .query_string("fields dinosaur".to_string())
                    .log_group_name(Some("dinosaur::logs".to_string()))
                    .build()
                    .unwrap(),
                LogQueryInfoBuilder::default()
                    .query_id("dinosaur".to_string())
                    .query_string("fields dinosaur".to_string())
                    .log_group_name(None)
                    .build()
                    .unwrap()
            ]
        );
    }

    #[tokio::test]
    async fn should_return_empty_query_list() {
        let mut result = Some(Ok(DescribeQueriesOutputBuilder::default().build()));

        let mut cw_client = CloudWatchClient::default();
        cw_client
            .expect_describe_queries()
            .times(1)
            .returning(move || result.take().unwrap());

        let client = LogClient { client: cw_client };

        let queries = client.list_queries().await.unwrap();

        assert_eq!(queries, vec![]);
    }

    #[tokio::test]
    async fn should_return_error_when_describe_query_fails() {
        let mut result: Option<Result<DescribeQueriesOutput, SdkError<DescribeQueriesError>>> =
            Some(Err(SdkError::TimeoutError(Box::new(
                DescribeQueriesError::new(
                    DescribeQueriesErrorKind::InvalidParameterException(
                        InvalidParameterExceptionBuilder::default()
                            .message("Error")
                            .build(),
                    ),
                    ErrorBuilder::default().build(),
                ),
            ))));

        let mut cw_client = CloudWatchClient::default();
        cw_client
            .expect_describe_queries()
            .times(1)
            .returning(move || result.take().unwrap());

        let client = LogClient { client: cw_client };

        let queries = client.list_queries().await;

        assert!(queries.is_err());
    }
}
