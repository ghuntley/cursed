use crate::error::CursedError;
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
// use crate::stdlib::value::Value;

/// Create a new CSV reader with default configuration
pub fn new_reader<R: io::Read>(reader: R) -> Reader<R> {
    Reader::new(reader)
/// Create a new CSV writer with default configuration
pub fn new_writer<W: io::Write>(writer: W) -> Writer<W> {
    Writer::new(writer)
/// Create a new column-based CSV reader
pub fn new_column_reader<R: io::Read>(reader: R) -> ColumnReader<R> {
    ColumnReader::new(reader)
/// Create a new CSV streamer for processing large files
pub fn new_streamer<R: io::Read>(reader: R) -> Streamer<R> {
    Streamer::new(reader)
/// Create a new schema validator
pub fn new_schema() -> Schema {
    Schema::new()
/// Create a new CSV transformer
pub fn new_transformer<R: io::Read>(reader: R) -> Transformer<R> {
    Transformer::new(reader)
/// Quick function to read all records from a CSV string
pub fn read_all_from_string(csv_data: &str) -> CsvResult<Vec<Vec<String>>> {
    use std::io::Cursor;
    let cursor = Cursor::new(csv_data);
    let mut reader = new_reader(cursor);
    reader.read_all()
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
/// Validate CSV data against a schema
pub fn validate_csv_data<R: io::Read>(reader: R, schema: &Schema) -> ValidationResult {
    schema.validate(reader)
/// Transform CSV data using a transformer
pub fn transform_csv_data<R: io::Read>(reader: R, transformer: &mut Transformer<R>) -> CsvResult<Vec<Vec<String>>> {
    transformer.transform()
