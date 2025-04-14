use crate::actions::CloudWatchAction;
use crate::model::error::SauronError;
use async_trait::async_trait;
use tracing::log;

pub struct CustomCommandAction {}

impl CustomCommandAction {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

#[async_trait]
impl CloudWatchAction for CustomCommandAction {
    async fn options(&self) -> Result<Vec<String>, SauronError> {
        Ok(vec!["Command 1".to_string()])
    }

    async fn execute(&self, _selected_option: Option<String>) -> Result<Option<Box<dyn CloudWatchAction>>, SauronError> {
        log::info!("batata");
        Ok(None)
    }

    fn prompt(&self) -> Option<String> {
        Some("Select Command: ".to_string())
    }
}