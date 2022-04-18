use clap::{command, Command};
use cw_sauron::LogClient;

#[tokio::main]
async fn main() {
    let log_client = LogClient::new().await;

    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("list-queries").about("List Cloudwatch Queries"))
        .get_matches();

    match matches.subcommand() {
        Some(("list-queries", _)) => {
            println!("{}", log_client.list_queries().await.unwrap());
        }
        _ => unreachable!("Invalid Command"),
    }
}
