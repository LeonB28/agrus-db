mod datasource;
mod errors;

use datasource::csv::CsvDataSource;
use crate::datasource::DataSource;

fn main() {
    let path = "/Users/leon.bam/data-sets/f1/drivers_sessions.csv";
    let csv_ds: CsvDataSource = CsvDataSource::new(path.to_string(), true, b',');
    let schema = csv_ds.schema().unwrap();
    println!("{:?}", schema);
}
