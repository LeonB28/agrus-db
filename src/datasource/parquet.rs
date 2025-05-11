use std::fs::File;
use std::path::PathBuf;
use arrow::{
    record_batch::RecordBatch,
};
use arrow::datatypes::SchemaRef;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::ProjectionMask;
use crate::datasource::DataSource;
use crate::errors::{AgrusError, AgrusResult};

pub struct ParquetDataSource {
    file_path: PathBuf,
}
impl ParquetDataSource {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }
}
impl DataSource for ParquetDataSource {
    fn scan(&self, columns: &[&str]) -> AgrusResult<Vec<RecordBatch>> {
        let file = File::open(&self.file_path)?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let parquet_schema_descriptor = builder.parquet_schema();

        let projection = ProjectionMask::columns(
            parquet_schema_descriptor,
            columns.iter().copied() // ???????? what is going on here
        );

        let reader = builder
            .with_projection(projection)
            .build()?;

        let batches_result: Result<Vec<RecordBatch>, arrow::error::ArrowError> = reader.collect();
        batches_result.map_err(AgrusError::from)
    }

    fn schema(&self) -> AgrusResult<SchemaRef> {
        let file = File::open(&self.file_path)?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        Ok(builder.schema().clone())
    }
}

#[cfg(test)]
mod tests {
    use arrow::util::pretty::pretty_format_batches;
    use super::*;

    #[test]
    fn just_run() {
        let reader = ParquetDataSource::new("/Users/leon.bam/data-sets/f1/datalake/car_data/year=2023/month=10/part-00001-17afb535-8116-40af-a102-413f477453db-c000.snappy.parquet".parse().unwrap());
        let res_batches = reader
            .scan(&[
                "driver_number",
                "speed"
            ]).unwrap();
        print!("{}", pretty_format_batches(&[res_batches[0].clone()]).unwrap());
    }
}