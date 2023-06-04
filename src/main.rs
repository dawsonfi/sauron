use console::Term;
use cw_sauron::actions::{CloudWatchAction, MainMenuAction};
use cw_sauron::config::telemetry::{get_subscriber, init_subscriber};
use cw_sauron::model::error::SauronError;
use dialoguer::{theme::ColorfulTheme, Select};

#[tokio::main]
async fn main() -> Result<(), SauronError> {
    let subscriber = get_subscriber("sauron".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let mut current_action_option: Option<Box<dyn CloudWatchAction>> = Some(MainMenuAction::new());

    while current_action_option.is_some() {
        let current_action = current_action_option.unwrap();
        let options = current_action.options().await?;
        let prompt = current_action.prompt();

        let selected_option = match prompt {
            Some(value) => {
                let selected_action = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(value)
                    .items(&options)
                    .default(0)
                    .interact_on(&Term::buffered_stderr())
                    .unwrap();
                Some(options[selected_action].clone())
            }
            None => None,
        };

        current_action_option = current_action.execute(selected_option.clone()).await?
    }

    Ok(())
}
