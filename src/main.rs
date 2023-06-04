use console::Term;
use cw_sauron::actions::get_actions;
use cw_sauron::config::telemetry::{get_subscriber, init_subscriber};
use cw_sauron::model::error::SauronError;
use dialoguer::{theme::ColorfulTheme, Select};

#[tokio::main]
async fn main() -> Result<(), SauronError> {
    let subscriber = get_subscriber("sauron".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let actions = get_actions();

    let selected_action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the Action:")
        .items(&actions)
        .default(0)
        .interact_on(&Term::buffered_stderr())
        .unwrap();

    actions[selected_action].execute().await
}
