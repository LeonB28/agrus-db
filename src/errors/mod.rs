use std::fmt::{Display, Formatter};
use arrow::error::ArrowError;
use parquet::errors::ParquetError;

#[derive(Debug)]
pub enum AgrusError {
    ArrowError(ArrowError),
    ParquetError(ParquetError),
    SchemaMismatch(String),
    CantOpenFile(String),
}

impl AgrusError {}

impl Display for AgrusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AgrusError::ArrowError(s) => {
                let s = s.to_string();
                write!(f, "{s}")
            },
            AgrusError::ParquetError(s) => {
                let s = s.to_string();
                write!(f, "{s}")
            },
            AgrusError::SchemaMismatch(s) => {
                let s = s.to_string();
                write!(f, "{s}")
            },
            AgrusError::CantOpenFile(s) => {
                let s = s.to_string();
                write!(f, "{s}")
            }
        }
    }
}
impl From<ArrowError> for AgrusError {
    fn from(err: ArrowError) -> Self {
        Self::ArrowError(err)
    }

}

impl From<std::io::Error> for AgrusError {
    fn from(err: std::io::Error) -> Self {
        Self::CantOpenFile(err.to_string())
    }
}
impl From<ParquetError> for AgrusError {
    fn from(err: ParquetError) -> Self {
        Self::ParquetError(err)
    }
}

pub(crate) type AgrusResult<T> = Result<T, AgrusError>;