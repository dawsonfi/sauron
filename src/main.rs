mod args;

use clap::Parser;
use cw_sauron::LogClient;
use std::error::Error;

use args::{EntityType, LogsCommand, QueryCommand, SauronArgs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log_client = LogClient::new().await;
    let args = SauronArgs::parse();

    match args.entity_type {
        EntityType::Query(sub_command) => match sub_command.command {
            QueryCommand::List(args) => {
                println!("{}", log_client.list_queries().await?.to_string(args.full))
            }
            QueryCommand::Execute(args) => {
                println!(
                    "{}",
                    log_client
                        .execute_query(args.query_id.clone(), args.start_time()?, args.end_time()?)
                        .await?
                        .to_string(args.fields)
                );
            }
        },
        EntityType::Log(sub_command) => match sub_command.command {
            LogsCommand::Groups => {
                println!("{}", log_client.list_log_groups().await?);
            }
            LogsCommand::Streams(args) => {
                println!("{}", log_client.list_log_streams(args.log_group).await?);
            }
            LogsCommand::Fetch(args) => {
                println!("{}", log_client.list_logs(
                    args.log_group.clone(),
                    args.log_stream.clone(),
                    args.start_time()?,
                    args.end_time()?,
                ).await?.to_string(None));
            }
        },
    };

    Ok(())
}
