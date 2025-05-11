use crate::datasource::DataSource;
use crate::errors::{AgrusError, AgrusResult};
use arrow::csv;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::fs::File;
use std::sync::Arc;

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
    fn scan(&self, columns: Vec<String>) -> AgrusResult<Vec<RecordBatch>> {
        // 1. Get the full schema of the CSV file. This is important for correct parsing.
        let full_file_schema: Arc<Schema> = Arc::new(self.schema()?);

        // 2. Determine the projection (which columns to select).
        let projection_indices: Option<Vec<usize>> = if columns.is_empty() {
            None
        } else {
            let mut indices = Vec::with_capacity(columns.len());
            for col_name in &columns {
                match full_file_schema.index_of(col_name) {
                    Ok(idx) => indices.push(idx),
                    Err(_) => {
                        return Err(AgrusError::SchemaMismatch(format!(
                            // Consider a more specific error variant
                            "Column '{}' not found in CSV file '{}'. Available columns: {:?}",
                            col_name,
                            self.file_path,
                            full_file_schema
                                .fields()
                                .iter()
                                .map(|f| f.name())
                                .collect::<Vec<_>>()
                        )));
                    }
                }
            }
            // If columns_to_select was not empty, but no valid columns were found (e.g. all names were wrong)
            // the `indices` vec would be empty. `with_projection([])` means select no columns.
            Some(indices)
        };

        let file = File::open(&self.file_path).map_err(AgrusError::from)?;
        let mut reader_builder = csv::ReaderBuilder::new(full_file_schema.clone())
            .with_header(self.has_header)
            .with_delimiter(self.delimiter);
        // .with_infer_schema(None) // We provide the schema, so no need to infer again. Default is None.
        // .with_batch_size(...) // Optionally configure batch size

        if let Some(indices) = projection_indices {
            reader_builder = reader_builder.with_projection(indices);
        }

        let batches_result: Result<Vec<RecordBatch>, arrow::error::ArrowError> =
            reader_builder.build(file)?.collect();

        batches_result.map_err(AgrusError::from)
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
    use std::fs;
    use arrow::util::pretty::pretty_format_batches;

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

    #[test]
    fn can_read_from_csv() {
        // Create a dummy CSV file for testing
        let csv_content = "col1,col2\n1,a\n2,b";
        fs::create_dir_all("data").unwrap();
        fs::write("data/test.csv", csv_content).unwrap();
        let csv_ds = CsvDataSource::new("data/test.csv".to_string(), true, b',');
        let batches = csv_ds
            .scan(vec!["col1".to_string(), "col2".to_string()])
            .unwrap();
        assert_eq!(batches.len(), 1);
        let batch = &batches[0];
        assert_eq!(batch.num_columns(), 2);
        print!("{}", pretty_format_batches(&batches).unwrap());
    }
}
