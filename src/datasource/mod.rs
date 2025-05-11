pub mod csv;
mod parquet;

use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use crate::errors::AgrusResult;

pub trait DataSource {
    fn scan(&self, columns: &[&str]) -> AgrusResult<Vec<RecordBatch>>;
    fn schema(&self) -> AgrusResult<SchemaRef>;
}