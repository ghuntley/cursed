use crate::error::CursedError;
/// Column-based CSV reader with type conversion capabilities
use std::io;
use std::collections::HashMap;
use std::str::FromStr;
// use crate::stdlib::csv::reader::Reader;
// use crate::stdlib::csv::error::{CsvError, CsvResult, column_not_found, type_conversion_error, invalid_header};

/// Typed values that can be extracted from CSV fields
#[derive(Debug, Clone, PartialEq)]
pub enum TypedValue {
impl TypedValue {
    /// Get the string representation of the value
    pub fn as_string(&self) -> String {
        match self {
        }
    }
    
    /// Get the value as an integer if possible
    pub fn as_integer(&self) -> Option<i64> {
        match self {
        }
    }
    
    /// Get the value as a float if possible
    pub fn as_float(&self) -> Option<f64> {
        match self {
        }
    }
    
    /// Get the value as a boolean if possible
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            TypedValue::String(s) => {
                match s.to_lowercase().as_str() {
                }
        }
    }
/// Column-based CSV reader that provides access to fields by column name
pub struct ColumnReader<R: io::Read> {
    /// Underlying CSV reader
    
    /// Column name to index mapping
    
    /// Header row
    
    /// Current record
    
    /// Whether header has been read
    
    /// Current line number for error reporting
    
    /// CursedError state
impl<R: io::Read> ColumnReader<R> {
    /// Create a new column reader
    pub fn new(reader: R) -> Self {
        Self {
        }
    }
    
    /// Create a new column reader with custom CSV reader configuration
    pub fn with_reader(reader: Reader<R>) -> Self {
        Self {
        }
    }
    
    /// Read the header row and build column mapping
    pub fn read_header(&mut self) -> CsvResult<()> {
        if self.header_read {
            return Ok(());
        match self.reader.read() {
            Ok(Some(header)) => {
                self.header = header.clone();
                self.column_map.clear();
                
                for (index, column_name) in header.iter().enumerate() {
                    if self.column_map.contains_key(column_name) {
                        let err = invalid_header(&format!("duplicate column name: '{}'", column_name));
                        self.error = Some(err.clone());
                        return Err(err);
                    }
                    self.column_map.insert(column_name.clone(), index);
                self.header_read = true;
                self.line_number = 1;
                Ok(())
            Ok(None) => {
                let err = invalid_header("no header found in CSV");
                self.error = Some(err.clone());
                Err(err)
            Err(e) => {
                self.error = Some(e.clone());
                Err(e)
            }
        }
    /// Move to the next record
    pub fn next(&mut self) -> bool {
        if !self.header_read {
            if let Err(e) = self.read_header() {
                self.error = Some(e);
                return false;
            }
        }
        
        match self.reader.read() {
            Ok(Some(record)) => {
                self.current_record = Some(record);
                self.line_number += 1;
                true
            Ok(None) => {
                self.current_record = None;
                false
            Err(e) => {
                self.error = Some(e);
                self.current_record = None;
                false
            }
        }
    /// Get a field value by column name
    pub fn get(&self, column_name: &str) -> CsvResult<String> {
        if let Some(record) = &self.current_record {
            if let Some(&index) = self.column_map.get(column_name) {
                if index < record.len() {
                    Ok(record[index].clone())
                } else {
                    Ok(String::new()) // Missing field
                }
            } else {
                Err(column_not_found(column_name))
            }
        } else {
            Err(CsvError::General("no current record".to_string()))
        }
    }
    
    /// Get a field value as an integer
    pub fn get_int(&self, column_name: &str) -> CsvResult<i64> {
        let value = self.get(column_name)?;
        
        if value.is_empty() {
            return Ok(0); // Default for empty fields
        value.parse::<i64>().map_err(|_| {
            let column_index = self.column_map.get(column_name).unwrap_or(&0);
            type_conversion_error(column_name, "integer", &value, self.line_number, *column_index + 1)
        })
    /// Get a field value as a float
    pub fn get_float(&self, column_name: &str) -> CsvResult<f64> {
        let value = self.get(column_name)?;
        
        if value.is_empty() {
            return Ok(0.0); // Default for empty fields
        value.parse::<f64>().map_err(|_| {
            let column_index = self.column_map.get(column_name).unwrap_or(&0);
            type_conversion_error(column_name, "float", &value, self.line_number, *column_index + 1)
        })
    /// Get a field value as a boolean
    pub fn get_bool(&self, column_name: &str) -> CsvResult<bool> {
        let value = self.get(column_name)?;
        
        if value.is_empty() {
            return Ok(false); // Default for empty fields
        match value.to_lowercase().as_str() {
            _ => {
                let column_index = self.column_map.get(column_name).unwrap_or(&0);
                Err(type_conversion_error(column_name, "boolean", &value, self.line_number, *column_index + 1))
            }
        }
    /// Get a field value as a typed value
    pub fn get_typed(&self, column_name: &str) -> CsvResult<TypedValue> {
        let value = self.get(column_name)?;
        
        if value.is_empty() {
            return Ok(TypedValue::Null);
        // Try to parse as different types
        if let Ok(int_val) = value.parse::<i64>() {
            return Ok(TypedValue::Integer(int_val));
        if let Ok(float_val) = value.parse::<f64>() {
            return Ok(TypedValue::Float(float_val));
        match value.to_lowercase().as_str() {
        }
    }
    
    /// Get all fields in the current record as a map
    pub fn get_all(&self) -> CsvResult<HashMap<String, String>> {
        if let Some(record) = &self.current_record {
            let mut result = HashMap::new();
            
            for (column_name, &index) in &self.column_map {
                let value = if index < record.len() {
                    record[index].clone()
                } else {
                    String::new()
                result.insert(column_name.clone(), value);
            Ok(result)
        } else {
            Err(CsvError::General("no current record".to_string()))
        }
    }
    
    /// Get all fields in the current record as typed values
    pub fn get_all_typed(&self) -> CsvResult<HashMap<String, TypedValue>> {
        if let Some(record) = &self.current_record {
            let mut result = HashMap::new();
            
            for (column_name, &index) in &self.column_map {
                let typed_value = if index < record.len() {
                    let value = &record[index];
                    if value.is_empty() {
                        TypedValue::Null
                    } else {
                        // Try to parse as different types
                        if let Ok(int_val) = value.parse::<i64>() {
                            TypedValue::Integer(int_val)
                        } else if let Ok(float_val) = value.parse::<f64>() {
                            TypedValue::Float(float_val)
                        } else {
                            match value.to_lowercase().as_str() {
                            }
                        }
                    }
                } else {
                    TypedValue::Null
                
                result.insert(column_name.clone(), typed_value);
            Ok(result)
        } else {
            Err(CsvError::General("no current record".to_string()))
        }
    }
    
    /// Get the header columns
    pub fn columns(&self) -> &[String] {
        &self.header
    /// Check if a column exists
    pub fn has_column(&self, column_name: &str) -> bool {
        self.column_map.contains_key(column_name)
    /// Get the current record as a vector
    pub fn current_record(&self) -> Option<&Vec<String>> {
        self.current_record.as_ref()
    /// Get any error that occurred
    pub fn err(&self) -> Option<&CsvError> {
        self.error.as_ref()
    /// Get the current line number
    pub fn line_number(&self) -> usize {
        self.line_number
    /// Check if header has been read
    pub fn has_header(&self) -> bool {
        self.header_read
    /// Get access to the underlying reader
    pub fn reader(&mut self) -> &mut Reader<R> {
        &mut self.reader
    /// Configure the underlying reader
    pub fn comma(mut self, c: char) -> Self {
        self.reader = self.reader.comma(c);
        self
    pub fn comment(mut self, c: char) -> Self {
        self.reader = self.reader.comment(c);
        self
    pub fn trim_leading_space(mut self, enable: bool) -> Self {
        self.reader = self.reader.trim_leading_space(enable);
        self
    }
}

