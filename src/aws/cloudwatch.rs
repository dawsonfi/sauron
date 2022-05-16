use crate::aws::model::{
    LogField, LogLine, LogQueryInfo, LogQueryInfoBuilder, LogQueryInfoList, LogResults,
    TerminalError,
};
use aws_sdk_cloudwatchlogs::model::{QueryDefinition, QueryStatus};
use chrono::{DateTime, Utc};
#[double]
use internal::CloudWatchClient;
use mockall_double::double;
use std::error::Error;

#[allow(dead_code)]
mod internal {

    use aws_config::from_env;
    use aws_sdk_cloudwatchlogs::{
        error::DescribeLogGroupsError, error::DescribeQueryDefinitionsError,
        error::GetLogEventsError, error::GetQueryResultsError, error::StartQueryError,
        output::DescribeLogGroupsOutput, output::DescribeQueryDefinitionsOutput,
        output::GetLogEventsOutput, output::GetQueryResultsOutput, output::StartQueryOutput,
        Client,
    };
    use aws_smithy_http::result::SdkError;
    use chrono::{DateTime, Utc};

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
            &self,
            next_token: Option<String>,
        ) -> Result<DescribeQueryDefinitionsOutput, SdkError<DescribeQueryDefinitionsError>>
        {
            let mut req = self.client.describe_query_definitions();

            if next_token.is_some() {
                req = req.next_token(next_token.unwrap());
            }

            req.send().await
        }

        pub async fn start_query(
            &self,
            log_group_names: Vec<String>,
            query_string: String,
            start_time: DateTime<Utc>,
            end_time: DateTime<Utc>,
        ) -> Result<StartQueryOutput, SdkError<StartQueryError>> {
            let mut req = self
                .client
                .start_query()
                .query_string(query_string)
                .start_time(start_time.timestamp())
                .end_time(end_time.timestamp());

            for log_group_name in log_group_names {
                req = req.log_group_name(log_group_name);
            }

            req.send().await
        }

        pub async fn get_query_results(
            &self,
            query_id: String,
        ) -> Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>> {
            self.client
                .get_query_results()
                .query_id(query_id)
                .send()
                .await
        }

        pub async fn describe_log_groups(
            &self,
            next_token: Option<String>,
        ) -> Result<DescribeLogGroupsOutput, SdkError<DescribeLogGroupsError>> {
            let mut req = self.client.describe_log_groups();

            if next_token.is_some() {
                req = req.next_token(next_token.unwrap());
            }

            req.send().await
        }

        pub async fn get_log_events(
            &self,
            log_group_name: String,
            start_time: DateTime<Utc>,
            end_time: DateTime<Utc>,
            next_token: Option<String>,
        ) -> Result<GetLogEventsOutput, SdkError<GetLogEventsError>> {
            let mut req = self
                .client
                .get_log_events()
                .log_group_name(log_group_name)
                .start_time(start_time.timestamp())
                .end_time(end_time.timestamp());

            if next_token.is_some() {
                req = req.next_token(next_token.unwrap());
            }

            req.send().await
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

    pub async fn list_queries(&self) -> Result<LogQueryInfoList, Box<dyn Error>> {
        let mut queries: Vec<LogQueryInfo> = vec![];
        let mut next_token: Option<String> = None;

        loop {
            let query_results = self.client.describe_query_definitions(next_token).await?;

            next_token = query_results.next_token.clone();

            let fetched_queries = query_results
                .query_definitions
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(LogClient::build_query_info)
                .collect::<Vec<LogQueryInfo>>();

            queries.extend(fetched_queries);

            if next_token.is_none() {
                break;
            }
        }

        Ok(LogQueryInfoList { queries: queries })
    }

    pub async fn execute_query(
        &self,
        query_id: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<LogResults, Box<dyn Error>> {
        let query = self.list_queries().await?.find(query_id.clone());

        let query = match query {
            Some(result) => Ok(result),
            None => Err(TerminalError::new(&format!(
                "Query with id={} not found",
                query_id.clone()
            ))),
        }?;

        let query_execution = self
            .client
            .start_query(query.log_group_names, query.query, start_time, end_time)
            .await?;

        let mut log_lines: Vec<LogLine> = vec![];

        loop {
            let results = self
                .client
                .get_query_results(query_execution.query_id().unwrap().to_string())
                .await?;

            let status = results.status().unwrap();

            if status == &QueryStatus::Failed
                || status == &QueryStatus::Timeout
                || status == &QueryStatus::Cancelled
            {
                return Err(Box::new(TerminalError::new("Query failed to run")));
            }

            if status == &QueryStatus::Complete {
                for result in results.results.unwrap() {
                    let mut line_fields: Vec<LogField> = vec![];
                    for result_field in result {
                        line_fields.push(LogField {
                            field: result_field.field().unwrap().to_string(),
                            value: result_field.value().unwrap().to_string(),
                        });
                    }
                    log_lines.push(LogLine {
                        fields: line_fields,
                    });
                }

                return Ok(LogResults { lines: log_lines });
            }
        }
    }

    fn build_query_info(query: QueryDefinition) -> LogQueryInfo {
        LogQueryInfoBuilder::default()
            .id(query.query_definition_id().unwrap().to_string())
            .name(query.name().unwrap().to_string())
            .query(query.query_string().unwrap().to_string())
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
    }
}

#[cfg(test)]
mod list_queries_tests {
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
        let mut cw_client = CloudWatchClient::default();
        mock_describe_queries(&mut cw_client);

        let client = LogClient { client: cw_client };

        let queries = client.list_queries().await.unwrap();

        assert_eq!(
            queries,
            LogQueryInfoList {
                queries: vec![
                    LogQueryInfoBuilder::default()
                        .id("dinosaur".to_string())
                        .name("DinoQuery".to_string())
                        .query("fields dinosaur".to_string())
                        .log_group_names(vec!["dinosaur::logs".to_string()])
                        .build()
                        .unwrap(),
                    LogQueryInfoBuilder::default()
                        .id("dinosaur".to_string())
                        .name("DinoQuery2".to_string())
                        .query("fields dinosaur".to_string())
                        .log_group_names(vec![])
                        .build()
                        .unwrap()
                ]
            }
        );
    }

    #[tokio::test]
    async fn should_return_queries_with_token() {
        let mut result_with_token = Some(Ok(DescribeQueryDefinitionOutputBuilder::default()
            .query_definitions(
                QueryDefinitionsBuilder::default()
                    .query_definition_id("dinosaur")
                    .name("DinoQuery")
                    .query_string("fields dinosaur")
                    .log_group_names("dinosaur::logs")
                    .build(),
            )
            .next_token("batata")
            .build()));

        let mut result_without_token = Some(Ok(DescribeQueryDefinitionOutputBuilder::default()
            .query_definitions(
                QueryDefinitionsBuilder::default()
                    .query_definition_id("dinosaur")
                    .name("DinoQuery2")
                    .query_string("fields dinosaur")
                    .build(),
            )
            .build()));

        let mut cw_client = CloudWatchClient::default();
        cw_client
            .expect_describe_query_definitions()
            .times(2)
            .returning(move |token| {
                if token.is_some() && token.unwrap() == "batata" {
                    return result_without_token.take().unwrap();
                }
                return result_with_token.take().unwrap();
            });

        let client = LogClient { client: cw_client };

        let queries = client.list_queries().await.unwrap();

        assert_eq!(
            queries,
            LogQueryInfoList {
                queries: vec![
                    LogQueryInfoBuilder::default()
                        .id("dinosaur".to_string())
                        .name("DinoQuery".to_string())
                        .query("fields dinosaur".to_string())
                        .log_group_names(vec!["dinosaur::logs".to_string()])
                        .build()
                        .unwrap(),
                    LogQueryInfoBuilder::default()
                        .id("dinosaur".to_string())
                        .name("DinoQuery2".to_string())
                        .query("fields dinosaur".to_string())
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
            .returning(move |_| result.take().unwrap());

        let client = LogClient { client: cw_client };

        let queries = client.list_queries().await.unwrap();

        assert_eq!(queries, LogQueryInfoList { queries: vec![] });
    }

    #[tokio::test]
    async fn should_return_error_when_describe_query_fails() {
        let mut result: Option<
            Result<DescribeQueryDefinitionsOutput, SdkError<DescribeQueryDefinitionsError>>,
        > = Some(Err(SdkError::TimeoutError(Box::new(
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
            .returning(move |_| result.take().unwrap());

        let client = LogClient { client: cw_client };

        let queries = client.list_queries().await;

        assert!(queries.is_err());
    }

    fn mock_describe_queries(cw_client: &mut CloudWatchClient) {
        let mut result = Some(Ok(DescribeQueryDefinitionOutputBuilder::default()
            .query_definitions(
                QueryDefinitionsBuilder::default()
                    .query_definition_id("dinosaur")
                    .name("DinoQuery")
                    .query_string("fields dinosaur")
                    .log_group_names("dinosaur::logs")
                    .build(),
            )
            .query_definitions(
                QueryDefinitionsBuilder::default()
                    .query_definition_id("dinosaur")
                    .name("DinoQuery2")
                    .query_string("fields dinosaur")
                    .build(),
            )
            .build()));

        cw_client
            .expect_describe_query_definitions()
            .times(1)
            .returning(move |_| result.take().unwrap());
    }
}

#[cfg(test)]
mod start_query_tests {

    use super::*;
    use aws_sdk_cloudwatchlogs::error::{
        invalid_parameter_exception::Builder as InvalidParameterExceptionBuilder,
        GetQueryResultsError, GetQueryResultsErrorKind, StartQueryError, StartQueryErrorKind,
    };
    use aws_sdk_cloudwatchlogs::model::query_definition::Builder as QueryDefinitionsBuilder;
    use aws_sdk_cloudwatchlogs::model::result_field::Builder as GetQueryResultsFieldBuilder;
    use aws_sdk_cloudwatchlogs::output::describe_query_definitions_output::Builder as DescribeQueryDefinitionOutputBuilder;
    use aws_sdk_cloudwatchlogs::output::get_query_results_output::Builder as GetQueryResultsOutputBuilder;
    use aws_sdk_cloudwatchlogs::output::start_query_output::Builder as StartQueryOutputBuilder;
    use aws_sdk_cloudwatchlogs::output::{GetQueryResultsOutput, StartQueryOutput};
    use aws_smithy_http::result::SdkError;
    use aws_smithy_types::error::Builder as ErrorBuilder;

    #[tokio::test]
    async fn should_return_query_results_when_available() {
        let mut cw_client = CloudWatchClient::default();
        mock_describe_queries(&mut cw_client);
        mock_start_query(&mut cw_client);
        mock_get_query_results(&mut cw_client);

        let client = LogClient { client: cw_client };

        let result = client
            .execute_query("dinosaur".to_string(), Utc::now(), Utc::now())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_return_query_results_when_complete() {
        let mut cw_client = CloudWatchClient::default();
        mock_describe_queries(&mut cw_client);
        mock_start_query(&mut cw_client);

        let mut result_running: Option<
            Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>,
        > = Some(Ok(GetQueryResultsOutputBuilder::default()
            .status(QueryStatus::Running)
            .build()));

        let mut result_complete: Option<
            Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>,
        > = Some(Ok(GetQueryResultsOutputBuilder::default()
            .status(QueryStatus::Complete)
            .results(vec![GetQueryResultsFieldBuilder::default()
                .field("@message")
                .value("Dinosaur Logs")
                .build()])
            .build()));

        let mut called = false;
        cw_client
            .expect_get_query_results()
            .times(2)
            .returning(move |_| {
                if called {
                    return result_complete.take().unwrap();
                }

                called = true;
                result_running.take().unwrap()
            });

        let client = LogClient { client: cw_client };

        let result = client
            .execute_query("dinosaur".to_string(), Utc::now(), Utc::now())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_return_query_results_error_when_failed() {
        let mut cw_client = CloudWatchClient::default();
        mock_describe_queries(&mut cw_client);
        mock_start_query(&mut cw_client);

        let mut result_error: Option<
            Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>,
        > = Some(Ok(GetQueryResultsOutputBuilder::default()
            .status(QueryStatus::Failed)
            .build()));

        cw_client
            .expect_get_query_results()
            .times(1)
            .returning(move |_| result_error.take().unwrap());

        let client = LogClient { client: cw_client };

        let result = client
            .execute_query("dinosaur".to_string(), Utc::now(), Utc::now())
            .await;

        assert!(result.is_err());
        assert_eq!("Query failed to run", format!("{}", result.err().unwrap()));
    }

    #[tokio::test]
    async fn should_return_query_results_error_when_timeout() {
        let mut cw_client = CloudWatchClient::default();
        mock_describe_queries(&mut cw_client);
        mock_start_query(&mut cw_client);

        let mut result_error: Option<
            Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>,
        > = Some(Ok(GetQueryResultsOutputBuilder::default()
            .status(QueryStatus::Timeout)
            .build()));

        cw_client
            .expect_get_query_results()
            .times(1)
            .returning(move |_| result_error.take().unwrap());

        let client = LogClient { client: cw_client };

        let result = client
            .execute_query("dinosaur".to_string(), Utc::now(), Utc::now())
            .await;

        assert!(result.is_err());
        assert_eq!("Query failed to run", format!("{}", result.err().unwrap()));
    }

    #[tokio::test]
    async fn should_return_query_results_error_when_cancelled() {
        let mut cw_client = CloudWatchClient::default();
        mock_describe_queries(&mut cw_client);
        mock_start_query(&mut cw_client);

        let mut result_error: Option<
            Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>,
        > = Some(Ok(GetQueryResultsOutputBuilder::default()
            .status(QueryStatus::Cancelled)
            .build()));

        cw_client
            .expect_get_query_results()
            .times(1)
            .returning(move |_| result_error.take().unwrap());

        let client = LogClient { client: cw_client };

        let result = client
            .execute_query("dinosaur".to_string(), Utc::now(), Utc::now())
            .await;

        assert!(result.is_err());
        assert_eq!("Query failed to run", format!("{}", result.err().unwrap()));
    }

    #[tokio::test]
    async fn should_return_error_when_query_results_fail() {
        let mut cw_client = CloudWatchClient::default();
        mock_describe_queries(&mut cw_client);
        mock_start_query(&mut cw_client);

        let mut result: Option<Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>> =
            Some(Err(SdkError::TimeoutError(Box::new(
                GetQueryResultsError::new(
                    GetQueryResultsErrorKind::InvalidParameterException(
                        InvalidParameterExceptionBuilder::default()
                            .message("Error")
                            .build(),
                    ),
                    ErrorBuilder::default().build(),
                ),
            ))));

        cw_client
            .expect_get_query_results()
            .times(1)
            .returning(move |_| result.take().unwrap());

        let client = LogClient { client: cw_client };

        let result = client
            .execute_query("dinosaur".to_string(), Utc::now(), Utc::now())
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_return_error_when_start_query_fail() {
        let mut cw_client = CloudWatchClient::default();
        mock_describe_queries(&mut cw_client);

        let mut result: Option<Result<StartQueryOutput, SdkError<StartQueryError>>> =
            Some(Err(SdkError::TimeoutError(Box::new(StartQueryError::new(
                StartQueryErrorKind::InvalidParameterException(
                    InvalidParameterExceptionBuilder::default()
                        .message("Error")
                        .build(),
                ),
                ErrorBuilder::default().build(),
            )))));

        cw_client
            .expect_start_query()
            .times(1)
            .returning(move |_, _, _, _| result.take().unwrap());

        let client = LogClient { client: cw_client };

        let result = client
            .execute_query("dinosaur".to_string(), Utc::now(), Utc::now())
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_return_error_when_query_id_not_found() {
        let mut cw_client = CloudWatchClient::default();
        mock_describe_queries(&mut cw_client);

        let client = LogClient { client: cw_client };

        let result = client
            .execute_query("batata".to_string(), Utc::now(), Utc::now())
            .await;
        assert!(result.is_err());
    }

    fn mock_describe_queries(cw_client: &mut CloudWatchClient) {
        let mut result = Some(Ok(DescribeQueryDefinitionOutputBuilder::default()
            .query_definitions(
                QueryDefinitionsBuilder::default()
                    .query_definition_id("dinosaur")
                    .name("DinoQuery")
                    .query_string("fields dinosaur")
                    .log_group_names("dinosaur::logs")
                    .build(),
            )
            .query_definitions(
                QueryDefinitionsBuilder::default()
                    .query_definition_id("dinosaur")
                    .name("DinoQuery2")
                    .query_string("fields dinosaur")
                    .build(),
            )
            .build()));

        cw_client
            .expect_describe_query_definitions()
            .times(1)
            .returning(move |_| result.take().unwrap());
    }

    fn mock_start_query(cw_client: &mut CloudWatchClient) {
        let mut result: Option<Result<StartQueryOutput, SdkError<StartQueryError>>> =
            Some(Ok(StartQueryOutputBuilder::default()
                .query_id("dinosaur_id")
                .build()));

        cw_client
            .expect_start_query()
            .times(1)
            .returning(move |_, _, _, _| result.take().unwrap());
    }

    fn mock_get_query_results(cw_client: &mut CloudWatchClient) {
        let mut result: Option<Result<GetQueryResultsOutput, SdkError<GetQueryResultsError>>> =
            Some(Ok(GetQueryResultsOutputBuilder::default()
                .status(QueryStatus::Complete)
                .results(vec![GetQueryResultsFieldBuilder::default()
                    .field("@message")
                    .value("Dinosaur Logs")
                    .build()])
                .build()));

        cw_client
            .expect_get_query_results()
            .times(1)
            .returning(move |_| result.take().unwrap());
    }
}
