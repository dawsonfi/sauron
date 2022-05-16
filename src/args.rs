use chrono::format::ParseError;
use chrono::{DateTime, Utc};
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct SauronArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// List/Execute saved Queries on Cloudwatch.
    Query(QuerySubCommand),
    Log(LogsSubCommand),
}

#[derive(Debug, Args)]
pub struct LogsSubCommand {
    #[clap(subcommand)]
    pub command: LogsCommand,
}

#[derive(Debug, Subcommand)]
pub enum LogsCommand {
    Groups,
    Fetch(FetchLogsArgs),
}

#[derive(Debug, Args)]
pub struct FetchLogsArgs {
    #[clap(short)]
    /// Log Group for the logs to be fetched
    pub log_group: String,

    #[clap(short)]
    /// Start time to execute the query (format 01-12-2022 18:10:11 +0300)
    pub start_time: String,

    #[clap(short)]
    /// (Optional) End time to execute the query (format 01-12-2022 18:10:11 +0300)
    pub end_time: Option<String>,
}

#[derive(Debug, Args)]
pub struct QuerySubCommand {
    #[clap(subcommand)]
    pub command: QueryCommand,
}

#[derive(Debug, Subcommand)]
pub enum QueryCommand {
    /// List saved Queries on Cloudwatch.
    List(ListQueryArgs),

    /// Execute Query on Cloudwatch
    Execute(ExecuteQueryArgs),
}

#[derive(Debug, Args)]
pub struct ListQueryArgs {
    #[clap(short, parse(from_flag))]
    /// Show full query definition
    pub full: bool,
}

#[derive(Debug, Args)]
pub struct ExecuteQueryArgs {
    #[clap(short)]
    /// Id of the query to be executed
    pub query_id: String,

    #[clap(short)]
    /// Start time to execute the query (format 01-12-2022 18:10:11 +0300)
    pub start_time: String,

    #[clap(short)]
    /// (Optional) End time to execute the query (format 01-12-2022 18:10:11 +0300)
    pub end_time: Option<String>,

    #[clap(short)]
    /// (Optional) print only provided fields.
    /// All fields will be print if not provided
    pub fields: Option<Vec<String>>,
}

impl ExecuteQueryArgs {
    pub fn start_time(&self) -> Result<DateTime<Utc>, ParseError> {
        ExecuteQueryArgs::convert_date(&self.start_time)
    }

    pub fn end_time(&self) -> Result<DateTime<Utc>, ParseError> {
        match &self.end_time {
            Some(end_time) => ExecuteQueryArgs::convert_date(end_time),
            None => Ok(Utc::now()),
        }
    }

    fn convert_date(date_str: &String) -> Result<DateTime<Utc>, ParseError> {
        let date = DateTime::parse_from_str(date_str.as_str(), "%d-%m-%Y %H:%M:%S %z")?;

        Ok(date.with_timezone(&Utc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_start_time_to_utc() {
        let args = ExecuteQueryArgs {
            query_id: "batata".to_string(),
            start_time: "16-05-2022 08:00:00 +0300".to_string(),
            end_time: None,
            fields: None,
        };

        let start_time = args.start_time();
        assert!(start_time.is_ok());
        assert_eq!(
            format!("{}", start_time.unwrap()),
            "2022-05-16 05:00:00 UTC"
        );
    }

    #[test]
    fn convert_end_time_to_utc() {
        let args = ExecuteQueryArgs {
            query_id: "batata".to_string(),
            start_time: "17-05-2022 08:00:00 +0300".to_string(),
            end_time: Some("18-05-2022 16:00:00 +0300".to_string()),
            fields: None,
        };

        let start_time = args.end_time();
        assert!(start_time.is_ok());
        assert_eq!(
            format!("{}", start_time.unwrap()),
            "2022-05-18 13:00:00 UTC"
        );
    }

    #[test]
    fn should_return_now_when_end_time_is_none() {
        let args = ExecuteQueryArgs {
            query_id: "batata".to_string(),
            start_time: "17-05-2022 08:00:00 +0300".to_string(),
            end_time: None,
            fields: None,
        };

        let start_time = args.end_time();
        assert!(start_time.is_ok());
        assert_eq!(
            format!("{}", start_time.unwrap().to_rfc2822()),
            format!("{}", Utc::now().to_rfc2822())
        );
    }
}
