/// CSV (Comma Separated Values) processing module for CURSED
/// 
/// This module provides comprehensive CSV reading, writing, and processing functionality
/// including column-based access, streaming for large files, schema validation, and
/// data transformation capabilities.

pub mod error;
pub mod reader;
pub mod writer;
pub mod column_reader;
pub mod streamer;
pub mod schema;
pub mod transformer;

// Re-export main types and functions
pub use error::{CsvError, CsvResult, ParseError};
pub use reader::{Reader, ReaderConfig};
pub use writer::{Writer, WriterConfig};
pub use column_reader::{ColumnReader, TypedValue};
pub use streamer::{Streamer, StreamProcessor};
pub use schema::{Schema, SchemaColumn, ValidationResult, ValidationError, ColumnType};
pub use transformer::{Transformer, ColumnTransform, TransformResult};

use std::io;
use std::sync::Arc;
use crate::stdlib::value::Value;

/// Create a new CSV reader with default configuration
pub fn new_reader<R: io::Read>(reader: R) -> Reader<R> {
    Reader::new(reader)
}

/// Create a new CSV writer with default configuration
pub fn new_writer<W: io::Write>(writer: W) -> Writer<W> {
    Writer::new(writer)
}

/// Create a new column-based CSV reader
pub fn new_column_reader<R: io::Read>(reader: R) -> ColumnReader<R> {
    ColumnReader::new(reader)
}

/// Create a new CSV streamer for processing large files
pub fn new_streamer<R: io::Read>(reader: R) -> Streamer<R> {
    Streamer::new(reader)
}

/// Create a new schema validator
pub fn new_schema() -> Schema {
    Schema::new()
}

/// Create a new CSV transformer
pub fn new_transformer<R: io::Read>(reader: R) -> Transformer<R> {
    Transformer::new(reader)
}

/// Quick function to read all records from a CSV string
pub fn read_all_from_string(csv_data: &str) -> CsvResult<Vec<Vec<String>>> {
    use std::io::Cursor;
    let cursor = Cursor::new(csv_data);
    let mut reader = new_reader(cursor);
    reader.read_all()
}

/// Quick function to write records to a CSV string
pub fn write_all_to_string(records: &[Vec<String>]) -> CsvResult<String> {
    use std::io::Cursor;
    let mut buf = Vec::new();
    {
        let cursor = Cursor::new(&mut buf);
        let mut writer = new_writer(cursor);
        writer.write_all(records)?;
        writer.flush()?;
    }
    String::from_utf8(buf).map_err(|e| CsvError::InvalidUtf8(e))
}

/// Validate CSV data against a schema
pub fn validate_csv_data<R: io::Read>(reader: R, schema: &Schema) -> ValidationResult {
    schema.validate(reader)
}

/// Transform CSV data using a transformer
pub fn transform_csv_data<R: io::Read>(reader: R, transformer: &mut Transformer<R>) -> CsvResult<Vec<Vec<String>>> {
    transformer.transform()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_initialization() {
        // Test that we can create all the main components
        let csv_data = "name,age\nAlice,30\nBob,25";
        
        // Test reader creation
        let reader = new_reader(std::io::Cursor::new(csv_data));
        assert!(reader.comma() == ',');
        
        // Test writer creation
        let mut buf = Vec::new();
        let writer = new_writer(std::io::Cursor::new(&mut buf));
        assert!(writer.comma() == ',');
        
        // Test column reader creation
        let column_reader = new_column_reader(std::io::Cursor::new(csv_data));
        assert!(column_reader.has_header());
        
        // Test streamer creation
        let streamer = new_streamer(std::io::Cursor::new(csv_data));
        assert!(streamer.batch_size() > 0);
        
        // Test schema creation
        let schema = new_schema();
        assert!(schema.columns().is_empty());
        
        // Test transformer creation
        let transformer = new_transformer(std::io::Cursor::new(csv_data));
        assert!(transformer.transforms().is_empty());
    }

    #[test]
    fn test_quick_functions() {
        let csv_data = "name,age\nAlice,30\nBob,25";
        
        // Test reading all from string
        let records = read_all_from_string(csv_data).unwrap();
        assert_eq!(records.len(), 3); // Header + 2 data rows
        assert_eq!(records[0], vec!["name", "age"]);
        assert_eq!(records[1], vec!["Alice", "30"]);
        assert_eq!(records[2], vec!["Bob", "25"]);
        
        // Test writing all to string
        let output = write_all_to_string(&records).unwrap();
        assert!(output.contains("name,age"));
        assert!(output.contains("Alice,30"));
        assert!(output.contains("Bob,25"));
    }
}
