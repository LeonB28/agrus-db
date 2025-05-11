pub mod csv;

use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;
use crate::errors::AgrusResult;

pub trait DataSource {
    fn scan(&self, columns: Vec<String>) -> AgrusResult<Vec<RecordBatch>>;
    fn schema(&self) -> AgrusResult<Schema>;
}