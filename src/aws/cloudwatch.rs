use crate::aws::model::{LogQueryInfoBuilder, LogQueryInfoList};
#[double]
use internal::CloudWatchClient;
use mockall_double::double;
use std::error::Error;

#[allow(dead_code)]
mod internal {

    use aws_config::from_env;
    use aws_sdk_cloudwatchlogs::{
        error::DescribeQueryDefinitionsError, output::DescribeQueryDefinitionsOutput, Client,
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

        pub async fn describe_query_definitions(
            self,
        ) -> Result<DescribeQueryDefinitionsOutput, SdkError<DescribeQueryDefinitionsError>> {
            self.client.describe_query_definitions().send().await
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

    pub async fn list_queries(self) -> Result<LogQueryInfoList, Box<dyn Error>> {
        let queries = self
            .client
            .describe_query_definitions()
            .await?
            .query_definitions
            .unwrap_or_default()
            .into_iter()
            .map(|query| {
                LogQueryInfoBuilder::default()
                    .query_id(query.query_definition_id().unwrap().to_string())
                    .query_string(query.query_string().unwrap().to_string())
                    .log_group_names(
                        query
                            .log_group_names()
                            .unwrap_or_default()
                            .into_iter()
                            .map(|log_group_name| log_group_name.clone())
                            .collect(),
                    )
                    .build()
                    .unwrap()
            })
            .collect();

        Ok(LogQueryInfoList { queries: queries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_sdk_cloudwatchlogs::error::{
        invalid_parameter_exception::Builder as InvalidParameterExceptionBuilder,
        DescribeQueryDefinitionsError, DescribeQueryDefinitionsErrorKind,
    };
    use aws_sdk_cloudwatchlogs::model::query_definition::Builder as QueryDefinitionsBuilder;
    use aws_sdk_cloudwatchlogs::output::describe_query_definitions_output::Builder as DescribeQueryDefinitionOutputBuilder;
    use aws_sdk_cloudwatchlogs::output::DescribeQueryDefinitionsOutput;
    use aws_smithy_http::result::SdkError;
    use aws_smithy_types::error::Builder as ErrorBuilder;

    #[tokio::test]
    async fn should_return_queries() {
        let mut result = Some(Ok(DescribeQueryDefinitionOutputBuilder::default()
            .query_definitions(
                QueryDefinitionsBuilder::default()
                    .query_definition_id("dinosaur")
                    .query_string("fields dinosaur")
                    .log_group_names("dinosaur::logs")
                    .build(),
            )
            .query_definitions(
                QueryDefinitionsBuilder::default()
                    .query_definition_id("dinosaur")
                    .query_string("fields dinosaur")
                    .build(),
            )
            .build()));

        let mut cw_client = CloudWatchClient::default();
        cw_client
            .expect_describe_query_definitions()
            .times(1)
            .returning(move || result.take().unwrap());

        let client = LogClient { client: cw_client };

        let queries = client.list_queries().await.unwrap();

        assert_eq!(
            queries,
            LogQueryInfoList {
                queries: vec![
                    LogQueryInfoBuilder::default()
                        .query_id("dinosaur".to_string())
                        .query_string("fields dinosaur".to_string())
                        .log_group_names(vec!["dinosaur::logs".to_string()])
                        .build()
                        .unwrap(),
                    LogQueryInfoBuilder::default()
                        .query_id("dinosaur".to_string())
                        .query_string("fields dinosaur".to_string())
                        .log_group_names(vec![])
                        .build()
                        .unwrap()
                ]
            }
        );
    }

    #[tokio::test]
    async fn should_return_empty_query_list() {
        let mut result = Some(Ok(DescribeQueryDefinitionOutputBuilder::default().build()));

        let mut cw_client = CloudWatchClient::default();
        cw_client
            .expect_describe_query_definitions()
            .times(1)
            .returning(move || result.take().unwrap());

        let client = LogClient { client: cw_client };

        let queries = client.list_queries().await.unwrap();

        assert_eq!(queries, LogQueryInfoList { queries: vec![] });
    }

    #[tokio::test]
    async fn should_return_error_when_describe_query_fails() {
        let mut result: Option<Result<DescribeQueryDefinitionsOutput, SdkError<DescribeQueryDefinitionsError>>> =
            Some(Err(SdkError::TimeoutError(Box::new(
                DescribeQueryDefinitionsError::new(
                    DescribeQueryDefinitionsErrorKind::InvalidParameterException(
                        InvalidParameterExceptionBuilder::default()
                            .message("Error")
                            .build(),
                    ),
                    ErrorBuilder::default().build(),
                ),
            ))));

        let mut cw_client = CloudWatchClient::default();
        cw_client
            .expect_describe_query_definitions()
            .times(1)
            .returning(move || result.take().unwrap());

        let client = LogClient { client: cw_client };

        let queries = client.list_queries().await;

        assert!(queries.is_err());
    }
}
