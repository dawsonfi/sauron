mod args;

use clap::Parser;
use cw_sauron::LogClient;
use std::error::Error;

use args::{EntityType, SauronArgs, QueryCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log_client = LogClient::new().await;
    let args = SauronArgs::parse();

    match args.entity_type {
        EntityType::Query(sub_command) => {
            match sub_command.command {
                QueryCommand::List(_) => println!("{}", log_client.list_queries().await?),
                QueryCommand::Execute(_) => println!("Not Implemented Yet")
            }            
        },
    };

    Ok(())
}
