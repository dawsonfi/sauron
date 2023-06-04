use crate::actions::CloudWatchFunctionsAction;
use crate::model::error::SauronError;
use async_trait::async_trait;
use std::fmt::{Display, Formatter, Result as FmtResult};
use tracing::log;

pub struct DummyAction {}

impl DummyAction {
    pub fn new() -> Self {
        DummyAction {}
    }
}

impl Display for DummyAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.name())
    }
}

#[async_trait]
impl CloudWatchFunctionsAction for DummyAction {
    async fn execute(&self) -> Result<(), SauronError> {
        log::info!("Executing Dummy Logic");

        Ok(())
    }

    fn name(&self) -> String {
        "Dummy Action".to_string()
    }
}
