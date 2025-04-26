use crate::errors::{AgrusError, AgrusResult};
use arrow::csv;
use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;
use std::fs;
use crate::datasource::DataSource;

pub struct CsvDataSource {
    file_path: String,
    has_header: bool,
    delimiter: u8,
}

impl CsvDataSource {
    pub fn new(file_path: String, has_header: bool, delimiter: u8) -> Self {
        Self {
            file_path,
            has_header,
            delimiter,
        }
    }
}

impl DataSource for CsvDataSource {
    #[warn(unused_variables)]
    fn scan(columns: Vec<String>) -> AgrusResult<Vec<RecordBatch>> {
        unimplemented!()
    }

    fn schema(&self) -> AgrusResult<Schema> {
        csv::infer_schema_from_files(
            &[self.file_path.clone()],
            self.delimiter,
            Some(200),
            self.has_header,
        )
        .map_err(AgrusError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::datatypes::{DataType, Field};

    #[test]
    fn can_read_schema_from_csv() {
        // Create a dummy CSV file for testing
        let csv_content = "col1,col2\n1,a\n2,b";
        fs::create_dir_all("data").unwrap();
        fs::write("data/test.csv", csv_content).unwrap();
        let csv_ds = CsvDataSource::new("data/test.csv".to_string(), true, b',');
        let schema = csv_ds.schema().unwrap();
        assert_eq!(schema.fields().len(), 2);

        let schema_to_compare = Schema::new(vec![
            Field::new("col1", DataType::Int64, true),
            Field::new("col2", DataType::Utf8, true),
        ]);
        assert_eq!(schema, schema_to_compare);
        // Cleanup
        fs::remove_file("data/test.csv").unwrap();
        fs::remove_dir_all("data").unwrap();
    }
}
