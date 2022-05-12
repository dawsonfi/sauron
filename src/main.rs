mod args;

use clap::Parser;
use cw_sauron::LogClient;
use std::error::Error;

use args::{EntityType, SauronArgs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log_client = LogClient::new().await;
    let args = SauronArgs::parse();

    match args.entity_type {
        EntityType::ListQueries => println!("{}", log_client.list_queries().await?),
    };

    Ok(())
}
