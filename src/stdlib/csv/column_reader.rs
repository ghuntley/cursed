/// Column-based CSV reader with type conversion capabilities
use std::io;
use std::collections::HashMap;
use std::str::FromStr;
use crate::stdlib::csv::reader::Reader;
use crate::stdlib::csv::error::{CsvError, CsvResult, column_not_found, type_conversion_error, invalid_header};

/// Typed values that can be extracted from CSV fields
#[derive(Debug, Clone, PartialEq)]
pub enum TypedValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl TypedValue {
    /// Get the string representation of the value
    pub fn as_string(&self) -> String {
        match self {
            TypedValue::String(s) => s.clone(),
            TypedValue::Integer(i) => i.to_string(),
            TypedValue::Float(f) => f.to_string(),
            TypedValue::Boolean(b) => b.to_string(),
            TypedValue::Null => "".to_string(),
        }
    }
    
    /// Get the value as an integer if possible
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            TypedValue::Integer(i) => Some(*i),
            TypedValue::Float(f) => Some(*f as i64),
            TypedValue::String(s) => s.parse().ok(),
            TypedValue::Boolean(b) => Some(if *b { 1 } else { 0 }),
            TypedValue::Null => None,
        }
    }
    
    /// Get the value as a float if possible
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TypedValue::Float(f) => Some(*f),
            TypedValue::Integer(i) => Some(*i as f64),
            TypedValue::String(s) => s.parse().ok(),
            TypedValue::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            TypedValue::Null => None,
        }
    }
    
    /// Get the value as a boolean if possible
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            TypedValue::Boolean(b) => Some(*b),
            TypedValue::Integer(i) => Some(*i != 0),
            TypedValue::Float(f) => Some(*f != 0.0),
            TypedValue::String(s) => {
                match s.to_lowercase().as_str() {
                    "true" | "yes" | "1" | "on" | "based" => Some(true),
                    "false" | "no" | "0" | "off" | "cap" => Some(false),
                    _ => None,
                }
            },
            TypedValue::Null => Some(false),
        }
    }
}

/// Column-based CSV reader that provides access to fields by column name
pub struct ColumnReader<R: io::Read> {
    /// Underlying CSV reader
    reader: Reader<R>,
    
    /// Column name to index mapping
    column_map: HashMap<String, usize>,
    
    /// Header row
    header: Vec<String>,
    
    /// Current record
    current_record: Option<Vec<String>>,
    
    /// Whether header has been read
    header_read: bool,
    
    /// Current line number for error reporting
    line_number: usize,
    
    /// Error state
    error: Option<CsvError>,
}

impl<R: io::Read> ColumnReader<R> {
    /// Create a new column reader
    pub fn new(reader: R) -> Self {
        Self {
            reader: Reader::new(reader),
            column_map: HashMap::new(),
            header: Vec::new(),
            current_record: None,
            header_read: false,
            line_number: 0,
            error: None,
        }
    }
    
    /// Create a new column reader with custom CSV reader configuration
    pub fn with_reader(reader: Reader<R>) -> Self {
        Self {
            reader,
            column_map: HashMap::new(),
            header: Vec::new(),
            current_record: None,
            header_read: false,
            line_number: 0,
            error: None,
        }
    }
    
    /// Read the header row and build column mapping
    pub fn read_header(&mut self) -> CsvResult<()> {
        if self.header_read {
            return Ok(());
        }
        
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
                }
                
                self.header_read = true;
                self.line_number = 1;
                Ok(())
            },
            Ok(None) => {
                let err = invalid_header("no header found in CSV");
                self.error = Some(err.clone());
                Err(err)
            },
            Err(e) => {
                self.error = Some(e.clone());
                Err(e)
            }
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
            },
            Ok(None) => {
                self.current_record = None;
                false
            },
            Err(e) => {
                self.error = Some(e);
                self.current_record = None;
                false
            }
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
        }
        
        value.parse::<i64>().map_err(|_| {
            let column_index = self.column_map.get(column_name).unwrap_or(&0);
            type_conversion_error(column_name, "integer", &value, self.line_number, *column_index + 1)
        })
    }
    
    /// Get a field value as a float
    pub fn get_float(&self, column_name: &str) -> CsvResult<f64> {
        let value = self.get(column_name)?;
        
        if value.is_empty() {
            return Ok(0.0); // Default for empty fields
        }
        
        value.parse::<f64>().map_err(|_| {
            let column_index = self.column_map.get(column_name).unwrap_or(&0);
            type_conversion_error(column_name, "float", &value, self.line_number, *column_index + 1)
        })
    }
    
    /// Get a field value as a boolean
    pub fn get_bool(&self, column_name: &str) -> CsvResult<bool> {
        let value = self.get(column_name)?;
        
        if value.is_empty() {
            return Ok(false); // Default for empty fields
        }
        
        match value.to_lowercase().as_str() {
            "true" | "yes" | "1" | "on" | "based" => Ok(true),
            "false" | "no" | "0" | "off" | "cap" => Ok(false),
            _ => {
                let column_index = self.column_map.get(column_name).unwrap_or(&0);
                Err(type_conversion_error(column_name, "boolean", &value, self.line_number, *column_index + 1))
            }
        }
    }
    
    /// Get a field value as a typed value
    pub fn get_typed(&self, column_name: &str) -> CsvResult<TypedValue> {
        let value = self.get(column_name)?;
        
        if value.is_empty() {
            return Ok(TypedValue::Null);
        }
        
        // Try to parse as different types
        if let Ok(int_val) = value.parse::<i64>() {
            return Ok(TypedValue::Integer(int_val));
        }
        
        if let Ok(float_val) = value.parse::<f64>() {
            return Ok(TypedValue::Float(float_val));
        }
        
        match value.to_lowercase().as_str() {
            "true" | "yes" | "1" | "on" | "based" => Ok(TypedValue::Boolean(true)),
            "false" | "no" | "0" | "off" | "cap" => Ok(TypedValue::Boolean(false)),
            _ => Ok(TypedValue::String(value)),
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
                };
                result.insert(column_name.clone(), value);
            }
            
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
                                "true" | "yes" | "1" | "on" | "based" => TypedValue::Boolean(true),
                                "false" | "no" | "0" | "off" | "cap" => TypedValue::Boolean(false),
                                _ => TypedValue::String(value.clone()),
                            }
                        }
                    }
                } else {
                    TypedValue::Null
                };
                
                result.insert(column_name.clone(), typed_value);
            }
            
            Ok(result)
        } else {
            Err(CsvError::General("no current record".to_string()))
        }
    }
    
    /// Get the header columns
    pub fn columns(&self) -> &[String] {
        &self.header
    }
    
    /// Check if a column exists
    pub fn has_column(&self, column_name: &str) -> bool {
        self.column_map.contains_key(column_name)
    }
    
    /// Get the current record as a vector
    pub fn current_record(&self) -> Option<&Vec<String>> {
        self.current_record.as_ref()
    }
    
    /// Get any error that occurred
    pub fn err(&self) -> Option<&CsvError> {
        self.error.as_ref()
    }
    
    /// Get the current line number
    pub fn line_number(&self) -> usize {
        self.line_number
    }
    
    /// Check if header has been read
    pub fn has_header(&self) -> bool {
        self.header_read
    }
    
    /// Get access to the underlying reader
    pub fn reader(&mut self) -> &mut Reader<R> {
        &mut self.reader
    }
    
    /// Configure the underlying reader
    pub fn comma(mut self, c: char) -> Self {
        self.reader = self.reader.comma(c);
        self
    }
    
    pub fn comment(mut self, c: char) -> Self {
        self.reader = self.reader.comment(c);
        self
    }
    
    pub fn trim_leading_space(mut self, enable: bool) -> Self {
        self.reader = self.reader.trim_leading_space(enable);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_basic_column_access() {
        let csv_data = "name,age,registered\nAlice,30,true\nBob,25,false";
        let cursor = Cursor::new(csv_data);
        let mut reader = ColumnReader::new(cursor);
        
        reader.read_header().unwrap();
        assert_eq!(reader.columns(), &["name", "age", "registered"]);
        
        // Read first record
        assert!(reader.next());
        assert_eq!(reader.get("name").unwrap(), "Alice");
        assert_eq!(reader.get("age").unwrap(), "30");
        assert_eq!(reader.get("registered").unwrap(), "true");
        
        // Read second record
        assert!(reader.next());
        assert_eq!(reader.get("name").unwrap(), "Bob");
        assert_eq!(reader.get("age").unwrap(), "25");
        assert_eq!(reader.get("registered").unwrap(), "false");
        
        // No more records
        assert!(!reader.next());
    }

    #[test]
    fn test_type_conversion() {
        let csv_data = "name,age,height,registered\nAlice,30,5.6,true\nBob,25,6.0,false";
        let cursor = Cursor::new(csv_data);
        let mut reader = ColumnReader::new(cursor);
        
        reader.read_header().unwrap();
        
        // Read first record and test type conversions
        assert!(reader.next());
        assert_eq!(reader.get_int("age").unwrap(), 30);
        assert_eq!(reader.get_float("height").unwrap(), 5.6);
        assert_eq!(reader.get_bool("registered").unwrap(), true);
        
        // Read second record
        assert!(reader.next());
        assert_eq!(reader.get_int("age").unwrap(), 25);
        assert_eq!(reader.get_float("height").unwrap(), 6.0);
        assert_eq!(reader.get_bool("registered").unwrap(), false);
    }

    #[test]
    fn test_typed_values() {
        let csv_data = "name,age,height,registered,notes\nAlice,30,5.6,true,";
        let cursor = Cursor::new(csv_data);
        let mut reader = ColumnReader::new(cursor);
        
        reader.read_header().unwrap();
        reader.next();
        
        let name = reader.get_typed("name").unwrap();
        assert_eq!(name, TypedValue::String("Alice".to_string()));
        
        let age = reader.get_typed("age").unwrap();
        assert_eq!(age, TypedValue::Integer(30));
        
        let height = reader.get_typed("height").unwrap();
        assert_eq!(height, TypedValue::Float(5.6));
        
        let registered = reader.get_typed("registered").unwrap();
        assert_eq!(registered, TypedValue::Boolean(true));
        
        let notes = reader.get_typed("notes").unwrap();
        assert_eq!(notes, TypedValue::Null);
    }

    #[test]
    fn test_get_all() {
        let csv_data = "name,age,city\nAlice,30,New York";
        let cursor = Cursor::new(csv_data);
        let mut reader = ColumnReader::new(cursor);
        
        reader.read_header().unwrap();
        reader.next();
        
        let all_fields = reader.get_all().unwrap();
        assert_eq!(all_fields.get("name"), Some(&"Alice".to_string()));
        assert_eq!(all_fields.get("age"), Some(&"30".to_string()));
        assert_eq!(all_fields.get("city"), Some(&"New York".to_string()));
        assert_eq!(all_fields.len(), 3);
    }

    #[test]
    fn test_get_all_typed() {
        let csv_data = "name,age,registered\nAlice,30,true";
        let cursor = Cursor::new(csv_data);
        let mut reader = ColumnReader::new(cursor);
        
        reader.read_header().unwrap();
        reader.next();
        
        let all_typed = reader.get_all_typed().unwrap();
        assert_eq!(all_typed.get("name"), Some(&TypedValue::String("Alice".to_string())));
        assert_eq!(all_typed.get("age"), Some(&TypedValue::Integer(30)));
        assert_eq!(all_typed.get("registered"), Some(&TypedValue::Boolean(true)));
    }

    #[test]
    fn test_column_not_found() {
        let csv_data = "name,age\nAlice,30";
        let cursor = Cursor::new(csv_data);
        let mut reader = ColumnReader::new(cursor);
        
        reader.read_header().unwrap();
        reader.next();
        
        let result = reader.get("nonexistent");
        assert!(result.is_err());
        
        if let Err(CsvError::ColumnNotFound(name)) = result {
            assert_eq!(name, "nonexistent");
        } else {
            panic!("Expected ColumnNotFound error");
        }
    }

    #[test]
    fn test_type_conversion_errors() {
        let csv_data = "name,age\nAlice,abc";
        let cursor = Cursor::new(csv_data);
        let mut reader = ColumnReader::new(cursor);
        
        reader.read_header().unwrap();
        reader.next();
        
        let result = reader.get_int("age");
        assert!(result.is_err());
        
        if let Err(CsvError::TypeConversion { field, expected_type, value, .. }) = result {
            assert_eq!(field, "age");
            assert_eq!(expected_type, "integer");
            assert_eq!(value, "abc");
        } else {
            panic!("Expected TypeConversion error");
        }
    }

    #[test]
    fn test_boolean_parsing() {
        let csv_data = "flag1,flag2,flag3,flag4,flag5,flag6\ntrue,yes,1,false,no,0";
        let cursor = Cursor::new(csv_data);
        let mut reader = ColumnReader::new(cursor);
        
        reader.read_header().unwrap();
        reader.next();
        
        assert_eq!(reader.get_bool("flag1").unwrap(), true);
        assert_eq!(reader.get_bool("flag2").unwrap(), true);
        assert_eq!(reader.get_bool("flag3").unwrap(), true);
        assert_eq!(reader.get_bool("flag4").unwrap(), false);
        assert_eq!(reader.get_bool("flag5").unwrap(), false);
        assert_eq!(reader.get_bool("flag6").unwrap(), false);
    }

    #[test]
    fn test_empty_fields() {
        let csv_data = "name,age,notes\nAlice,,";
        let cursor = Cursor::new(csv_data);
        let mut reader = ColumnReader::new(cursor);
        
        reader.read_header().unwrap();
        reader.next();
        
        assert_eq!(reader.get("name").unwrap(), "Alice");
        assert_eq!(reader.get("age").unwrap(), "");
        assert_eq!(reader.get_int("age").unwrap(), 0); // Default for empty
        assert_eq!(reader.get_bool("notes").unwrap(), false); // Default for empty
    }

    #[test]
    fn test_duplicate_column_names() {
        let csv_data = "name,age,name\nAlice,30,Bob";
        let cursor = Cursor::new(csv_data);
        let mut reader = ColumnReader::new(cursor);
        
        let result = reader.read_header();
        assert!(result.is_err());
        
        if let Err(CsvError::InvalidHeader(msg)) = result {
            assert!(msg.contains("duplicate column name"));
        } else {
            panic!("Expected InvalidHeader error");
        }
    }

    #[test]
    fn test_configuration_methods() {
        let csv_data = "name;age\nAlice;30";
        let cursor = Cursor::new(csv_data);
        let mut reader = ColumnReader::new(cursor).comma(';');
        
        reader.read_header().unwrap();
        reader.next();
        
        assert_eq!(reader.get("name").unwrap(), "Alice");
        assert_eq!(reader.get("age").unwrap(), "30");
    }
}
