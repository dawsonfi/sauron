use crate::actions::CloudWatchAction;
use crate::model::error::SauronError;
use async_trait::async_trait;
use tracing::log;

pub struct FetchLogGroupsAction {}

impl FetchLogGroupsAction {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

#[async_trait]
impl CloudWatchAction for FetchLogGroupsAction {
    async fn options(&self) -> Result<Vec<String>, SauronError> {
        log::info!("Fetching Log Groups");
        Ok(vec!["Log Group 1".to_string()])
    }

    async fn execute(
        &self,
        _selected_option: Option<String>,
    ) -> Result<Option<Box<dyn CloudWatchAction>>, SauronError> {
        Ok(None)
    }

    fn prompt(&self) -> Option<String> {
        None
    }
}
