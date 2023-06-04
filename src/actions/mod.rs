use crate::actions::dummy_action::DummyAction;
use crate::model::error::SauronError;
use async_trait::async_trait;
use std::fmt::Display;

mod dummy_action;

#[async_trait]
pub trait CloudWatchFunctionsAction: Display {
    async fn execute(&self) -> Result<(), SauronError>;

    fn name(&self) -> String {
        "Invalid Action".to_string()
    }
}

pub fn get_actions() -> Vec<Box<dyn CloudWatchFunctionsAction>> {
    vec![Box::new(DummyAction::new())]
}
