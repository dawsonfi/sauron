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
    /// Id of the query to be executed
    pub query_id: String,
}
