use clap::Parser;
use cw_sauron::LogClient;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct LogCommand {
    #[clap(short, long)]
    command: String,
}

#[tokio::main]
async fn main() {
    let log_client = LogClient::new().await;
    let log_command = LogCommand::parse();

    match log_command.command.as_str() {
        "list_queries" => {
            println!("{}", log_client.list_queries().await.unwrap());
        }
        _ => println!("Command Not Found"),
    };
}
