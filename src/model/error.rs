use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct SauronError {
    pub source: Box<dyn Error>,
    pub message: String,
}

impl Display for SauronError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.message, self.source.as_ref())
    }
}

impl Error for SauronError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}
