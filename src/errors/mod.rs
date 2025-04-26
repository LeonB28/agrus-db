use std::fmt::{Display, Formatter};
use arrow::error::ArrowError;

#[derive(Debug)]
pub enum AgrusError {
    ArrowError(ArrowError),
}

impl Display for AgrusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AgrusError::ArrowError(s) => {
                let s = s.to_string();
                write!(f, "{s}")
            },
        }
    }
}
impl From<ArrowError> for AgrusError {
    fn from(err: ArrowError) -> Self {
        Self::ArrowError(err)
    }
}

pub(crate) type AgrusResult<T> = Result<T, AgrusError>;