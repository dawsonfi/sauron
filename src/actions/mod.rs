use crate::actions::tail_logs_action::TailLogsWrapperAction;
use crate::model::error::SauronError;
use async_trait::async_trait;

mod tail_logs_action;

#[async_trait]
pub trait CloudWatchAction {
    async fn options(&self) -> Result<Vec<String>, SauronError>;

    async fn execute(
        &self,
        selected_option: Option<String>,
    ) -> Result<Option<Box<dyn CloudWatchAction>>, SauronError>;

    fn prompt(&self) -> Option<String>;
}

pub struct MainMenuAction {}

impl MainMenuAction {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

#[async_trait]
impl CloudWatchAction for MainMenuAction {
    async fn options(&self) -> Result<Vec<String>, SauronError> {
        Ok(vec!["Tail Logs".to_string()])
    }

    async fn execute(
        &self,
        selected_option: Option<String>,
    ) -> Result<Option<Box<dyn CloudWatchAction>>, SauronError> {
        Ok(match selected_option.unwrap().as_str() {
            "Tail Logs" => Some(TailLogsWrapperAction::new()),
            _ => None,
        })
    }

    fn prompt(&self) -> Option<String> {
        Some("Select the Action:".to_string())
    }
}
