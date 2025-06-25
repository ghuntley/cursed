/// CSV Schema validation with pattern matching and type checking
use std::io;
use std::collections::HashMap;
// Note: Using a basic regex implementation for pattern matching
// In a real implementation, you would use the regex crate
// use crate::stdlib::csv::reader::Reader;
// use crate::stdlib::csv::error::{CsvError, CsvResult, schema_validation_error};
use crate::error::CursedError;

/// Column type definitions for validation
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    /// Any string value
    /// Integer values
    /// Floating point values
    /// Boolean values (true/false, yes/no, 1/0, etc.)
    /// Email addresses
    /// URL addresses
    /// Phone numbers
    /// Date values (various formats)
    /// Custom type with validation function
/// Column constraint for validation
#[derive(Debug, Clone)]
pub struct ColumnConstraint {
    /// Regular expression pattern to match (stored as string for simplicity)
    
    /// Minimum length for string values
    
    /// Maximum length for string values
    
    /// Minimum value for numeric types
    
    /// Maximum value for numeric types
    
    /// Whether the field is required (non-empty)
    
    /// Allowed values (enumeration)
    
    /// Custom validation message
impl Default for ColumnConstraint {
    fn default() -> Self {
        Self {
        }
    }
/// Schema column definition
#[derive(Debug, Clone)]
pub struct SchemaColumn {
    /// Column name
    
    /// Column type
    
    /// Validation constraints
    
    /// Whether this column is optional (not required to exist)
impl SchemaColumn {
    /// Create a new schema column
    pub fn new(name: &str, column_type: ColumnType) -> Self {
        Self {
        }
    }
    
    /// Mark this column as required (non-empty values)
    pub fn non_empty(mut self) -> Self {
        self.constraint.required = true;
        self
    /// Set a regex pattern for validation
    pub fn with_pattern(mut self, pattern: &str) -> crate::error::Result<()> {
        // Simple pattern validation - in real implementation use regex crate
        self.constraint.pattern = Some(pattern.to_string());
        Ok(self)
    /// Set minimum and maximum length for string values
    pub fn with_length_range(mut self, min: usize, max: usize) -> Self {
        self.constraint.min_length = Some(min);
        self.constraint.max_length = Some(max);
        self
    /// Set minimum length for string values
    pub fn with_min_length(mut self, min: usize) -> Self {
        self.constraint.min_length = Some(min);
        self
    /// Set maximum length for string values
    pub fn with_max_length(mut self, max: usize) -> Self {
        self.constraint.max_length = Some(max);
        self
    /// Set minimum and maximum values for numeric types
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.constraint.min_value = Some(min);
        self.constraint.max_value = Some(max);
        self
    /// Set minimum value for numeric types
    pub fn with_min_value(mut self, min: f64) -> Self {
        self.constraint.min_value = Some(min);
        self
    /// Set maximum value for numeric types
    pub fn with_max_value(mut self, max: f64) -> Self {
        self.constraint.max_value = Some(max);
        self
    /// Set allowed values (enumeration)
    pub fn with_allowed_values(mut self, values: Vec<String>) -> Self {
        self.constraint.allowed_values = Some(values);
        self
    /// Set custom error message
    pub fn with_error_message(mut self, message: &str) -> Self {
        self.constraint.error_message = Some(message.to_string());
        self
    /// Mark this column as optional (not required to exist in CSV)
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    /// Validate a field value against this column's constraints
    pub fn validate(&self, value: &str, line_number: usize, column_index: usize) -> crate::error::Result<()> {
        // Check if required but empty
        if self.constraint.required && value.is_empty() {
            return Err(ValidationError {
                message: self.constraint.error_message.clone()
            });
        // Skip validation for empty non-required fields
        if value.is_empty() && !self.constraint.required {
            return Ok(());
        // Type-specific validation
        match &self.column_type {
            ColumnType::Integer => {
                if value.parse::<i64>().is_err() {
                    return Err(ValidationError {
                    });
                // Check numeric range
                if let (Some(min), Some(max)) = (self.constraint.min_value, self.constraint.max_value) {
                    if let Ok(num_val) = value.parse::<f64>() {
                        if num_val < min || num_val > max {
                            return Err(ValidationError {
                            });
                        }
                    }
                }
            ColumnType::Float => {
                if value.parse::<f64>().is_err() {
                    return Err(ValidationError {
                    });
                // Check numeric range
                if let (Some(min), Some(max)) = (self.constraint.min_value, self.constraint.max_value) {
                    if let Ok(num_val) = value.parse::<f64>() {
                        if num_val < min || num_val > max {
                            return Err(ValidationError {
                            });
                        }
                    }
                }
            ColumnType::Boolean => {
                match value.to_lowercase().as_str() {
                    _ => {
                        return Err(ValidationError {
                        });
                    }
                }
            ColumnType::Email => {
                if !is_valid_email(value) {
                    return Err(ValidationError {
                    });
                }
            ColumnType::Url => {
                if !value.starts_with("http://") && !value.starts_with("https://") {
                    return Err(ValidationError {
                    });
                }
            ColumnType::Phone => {
                if !is_valid_phone(value) {
                    return Err(ValidationError {
                    });
                }
            _ => {} // No type-specific validation for String, Date, Custom
        // Pattern validation (simplified - in real implementation use regex crate)
        if let Some(ref pattern) = self.constraint.pattern {
            if !simple_pattern_match(value, pattern) {
                return Err(ValidationError {
                });
            }
        }
        
        // Length validation
        if let Some(min_len) = self.constraint.min_length {
            if value.len() < min_len {
                return Err(ValidationError {
                });
            }
        }
        
        if let Some(max_len) = self.constraint.max_length {
            if value.len() > max_len {
                return Err(ValidationError {
                });
            }
        }
        
        // Allowed values validation
        if let Some(ref allowed) = self.constraint.allowed_values {
            if !allowed.contains(&value.to_string()) {
                return Err(ValidationError {
                });
            }
        }
        
        Ok(())
    }
}

/// CSV Schema for validation
#[derive(Debug, Clone)]
pub struct Schema {
    /// Schema columns
    
    /// Whether to allow extra columns not defined in schema
    
    /// Whether to validate header presence
impl Schema {
    /// Create a new empty schema
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Add a required column to the schema
    pub fn require_column(&mut self, name: &str) -> &mut SchemaColumn {
        let column = SchemaColumn::new(name, ColumnType::String);
        self.columns.insert(name.to_string(), column);
        self.columns.get_mut(name).unwrap()
    /// Add a required column with specific type
    pub fn require_column_with_type(&mut self, name: &str, column_type: ColumnType) -> &mut SchemaColumn {
        let column = SchemaColumn::new(name, column_type);
        self.columns.insert(name.to_string(), column);
        self.columns.get_mut(name).unwrap()
    /// Add an optional column to the schema
    pub fn optional_column(&mut self, name: &str) -> &mut SchemaColumn {
        let column = SchemaColumn::new(name, ColumnType::String).optional();
        self.columns.insert(name.to_string(), column);
        self.columns.get_mut(name).unwrap()
    /// Set whether to allow extra columns
    pub fn allow_extra_columns(&mut self, allow: bool) -> &mut Self {
        self.allow_extra_columns = allow;
        self
    /// Set whether to require header
    pub fn require_header(&mut self, require: bool) -> &mut Self {
        self.require_header = require;
        self
    /// Get schema columns
    pub fn columns(&self) -> &HashMap<String, SchemaColumn> {
        &self.columns
    /// Validate CSV data against this schema
    pub fn validate<R: io::Read>(&self, reader: R) -> ValidationResult {
        let mut csv_reader = Reader::new(reader);
        let mut errors = Vec::new();
        let mut valid_records = 0;
        let mut total_records = 0;
        
        // Read header if required
        let header = if self.require_header {
            match csv_reader.read() {
                Ok(Some(header)) => {
                    // Validate header columns
                    let mut column_map = HashMap::new();
                    for (index, column_name) in header.iter().enumerate() {
                        column_map.insert(column_name.clone(), index);
                    // Check for required columns
                    for (name, column) in &self.columns {
                        if !column.optional && !column_map.contains_key(name) {
                            errors.push(ValidationError {
                            });
                        }
                    }
                    
                    // Check for extra columns if not allowed
                    if !self.allow_extra_columns {
                        for column_name in &header {
                            if !self.columns.contains_key(column_name) {
                                errors.push(ValidationError {
                                });
                            }
                        }
                    Some((header, column_map))
                Ok(None) => {
                    errors.push(ValidationError {
                    });
                    return ValidationResult {
                Err(e) => {
                    errors.push(ValidationError {
                    });
                    return ValidationResult {
                }
            }
        } else {
            None
        
        // Validate data records
        let mut line_number = if header.is_some() { 2 } else { 1 };
        
        while let Ok(Some(record)) = csv_reader.read() {
            total_records += 1;
            let mut record_valid = true;
            
            if let Some((ref header_row, ref column_map)) = header {
                // Validate each field according to schema
                for (column_name, column_def) in &self.columns {
                    if let Some(&column_index) = column_map.get(column_name) {
                        let field_value = if column_index < record.len() {
                            &record[column_index]
                        } else {
                            ""
                        
                        if let Err(mut validation_error) = column_def.validate(field_value, line_number, column_index + 1) {
                            validation_error.line = line_number;
                            errors.push(validation_error);
                            record_valid = false;
                        }
                    }
                }
            } else {
                // Without header, validate by position if possible
                for (index, field_value) in record.iter().enumerate() {
                    // This is a simplified validation without column names
                    if field_value.is_empty() {
                        // Check if any schema columns require non-empty values
                        if self.columns.values().any(|col| col.constraint.required) {
                            errors.push(ValidationError {
                            });
                            record_valid = false;
                        }
                    }
                }
            }
            
            if record_valid {
                valid_records += 1;
            line_number += 1;
        ValidationResult {
        }
    }
/// Validation error details
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Line number where error occurred
    
    /// Column number where error occurred
    
    /// Column name
    
    /// CursedError message
    
    /// The problematic value
// impl std::fmt::Display for ValidationError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Line {}, Column {} ({}): {} (value: '{}')", 
//                self.line, self.column, self.column_name, self.message, self.value)
//     }
// }

/// Result of schema validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// List of validation errors
    
    /// Number of valid records
    
    /// Total number of records processed
    
    /// Whether the overall schema is valid
impl ValidationResult {
    /// Check if validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    /// Get the number of errors
    pub fn error_count(&self) -> usize {
        self.errors.len()
    /// Get validation success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_records == 0 {
            1.0
        } else {
            self.valid_records as f64 / self.total_records as f64
        }
    }
    
    /// Get a summary of the validation
    pub fn summary(&self) -> String {
        format!(
            "Validation: {} valid / {} total records ({:.1}% success), {} errors",
            self.error_count()
        )
    }
}

/// Helper function for simple pattern matching (simplified for demo)
/// In a real implementation, you would use the regex crate
fn simple_pattern_match(value: &str, pattern: &str) -> bool {
    // Very basic pattern matching - in real implementation use regex
    match pattern {
        r"^\d{3}-\d{3}-\d{4}$" => {
            // Phone number pattern like 555-123-4567
            value.len() == 12 && 
            value.chars().nth(3) == Some('-') && 
            value.chars().nth(7) == Some('-') &&
            value.chars().enumerate().all(|(i, c)| {
                if i == 3 || i == 7 { c == '-' } else { c.is_ascii_digit() }
            })
        _ => true, // Default to true for unknown patterns
    }
}

/// Check if a string is a valid email address
fn is_valid_email(value: &str) -> bool {
    value.contains('@') && 
    value.contains('.') && 
    !value.starts_with('@') && 
    !value.ends_with('@') &&
    !value.starts_with('.') && 
    !value.ends_with('.') &&
    value.matches('@').count() == 1
/// Check if a string is a valid phone number
fn is_valid_phone(value: &str) -> bool {
    if value.is_empty() {
        return false;
    let mut has_digit = false;
    for ch in value.chars() {
        match ch {
        }
    }
    
    has_digit
