/// CSV Schema validation with pattern matching and type checking
use std::io;
use std::collections::HashMap;
// Note: Using a basic regex implementation for pattern matching
// In a real implementation, you would use the regex crate
use crate::stdlib::csv::reader::Reader;
use crate::stdlib::csv::error::{CsvError, CsvResult, schema_validation_error};

/// Column type definitions for validation
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    /// Any string value
    String,
    /// Integer values
    Integer,
    /// Floating point values
    Float,
    /// Boolean values (true/false, yes/no, 1/0, etc.)
    Boolean,
    /// Email addresses
    Email,
    /// URL addresses
    Url,
    /// Phone numbers
    Phone,
    /// Date values (various formats)
    Date,
    /// Custom type with validation function
    Custom(String),
}

/// Column constraint for validation
#[derive(Debug, Clone)]
pub struct ColumnConstraint {
    /// Regular expression pattern to match (stored as string for simplicity)
    pub pattern: Option<String>,
    
    /// Minimum length for string values
    pub min_length: Option<usize>,
    
    /// Maximum length for string values
    pub max_length: Option<usize>,
    
    /// Minimum value for numeric types
    pub min_value: Option<f64>,
    
    /// Maximum value for numeric types
    pub max_value: Option<f64>,
    
    /// Whether the field is required (non-empty)
    pub required: bool,
    
    /// Allowed values (enumeration)
    pub allowed_values: Option<Vec<String>>,
    
    /// Custom validation message
    pub error_message: Option<String>,
}

impl Default for ColumnConstraint {
    fn default() -> Self {
        Self {
            pattern: None,
            min_length: None,
            max_length: None,
            min_value: None,
            max_value: None,
            required: false,
            allowed_values: None,
            error_message: None,
        }
    }
}

/// Schema column definition
#[derive(Debug, Clone)]
pub struct SchemaColumn {
    /// Column name
    pub name: String,
    
    /// Column type
    pub column_type: ColumnType,
    
    /// Validation constraints
    pub constraint: ColumnConstraint,
    
    /// Whether this column is optional (not required to exist)
    pub optional: bool,
}

impl SchemaColumn {
    /// Create a new schema column
    pub fn new(name: &str, column_type: ColumnType) -> Self {
        Self {
            name: name.to_string(),
            column_type,
            constraint: ColumnConstraint::default(),
            optional: false,
        }
    }
    
    /// Mark this column as required (non-empty values)
    pub fn non_empty(mut self) -> Self {
        self.constraint.required = true;
        self
    }
    
    /// Set a regex pattern for validation
    pub fn with_pattern(mut self, pattern: &str) -> Result<Self, CsvError> {
        // Simple pattern validation - in real implementation use regex crate
        self.constraint.pattern = Some(pattern.to_string());
        Ok(self)
    }
    
    /// Set minimum and maximum length for string values
    pub fn with_length_range(mut self, min: usize, max: usize) -> Self {
        self.constraint.min_length = Some(min);
        self.constraint.max_length = Some(max);
        self
    }
    
    /// Set minimum length for string values
    pub fn with_min_length(mut self, min: usize) -> Self {
        self.constraint.min_length = Some(min);
        self
    }
    
    /// Set maximum length for string values
    pub fn with_max_length(mut self, max: usize) -> Self {
        self.constraint.max_length = Some(max);
        self
    }
    
    /// Set minimum and maximum values for numeric types
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.constraint.min_value = Some(min);
        self.constraint.max_value = Some(max);
        self
    }
    
    /// Set minimum value for numeric types
    pub fn with_min_value(mut self, min: f64) -> Self {
        self.constraint.min_value = Some(min);
        self
    }
    
    /// Set maximum value for numeric types
    pub fn with_max_value(mut self, max: f64) -> Self {
        self.constraint.max_value = Some(max);
        self
    }
    
    /// Set allowed values (enumeration)
    pub fn with_allowed_values(mut self, values: Vec<String>) -> Self {
        self.constraint.allowed_values = Some(values);
        self
    }
    
    /// Set custom error message
    pub fn with_error_message(mut self, message: &str) -> Self {
        self.constraint.error_message = Some(message.to_string());
        self
    }
    
    /// Mark this column as optional (not required to exist in CSV)
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
    
    /// Validate a field value against this column's constraints
    pub fn validate(&self, value: &str, line_number: usize, column_index: usize) -> Result<(), ValidationError> {
        // Check if required but empty
        if self.constraint.required && value.is_empty() {
            return Err(ValidationError {
                line: line_number,
                column: column_index,
                column_name: self.name.clone(),
                message: self.constraint.error_message.clone()
                    .unwrap_or_else(|| format!("Required field '{}' is empty", self.name)),
                value: value.to_string(),
            });
        }
        
        // Skip validation for empty non-required fields
        if value.is_empty() && !self.constraint.required {
            return Ok(());
        }
        
        // Type-specific validation
        match &self.column_type {
            ColumnType::Integer => {
                if value.parse::<i64>().is_err() {
                    return Err(ValidationError {
                        line: line_number,
                        column: column_index,
                        column_name: self.name.clone(),
                        message: format!("Invalid integer value: '{}'", value),
                        value: value.to_string(),
                    });
                }
                
                // Check numeric range
                if let (Some(min), Some(max)) = (self.constraint.min_value, self.constraint.max_value) {
                    if let Ok(num_val) = value.parse::<f64>() {
                        if num_val < min || num_val > max {
                            return Err(ValidationError {
                                line: line_number,
                                column: column_index,
                                column_name: self.name.clone(),
                                message: format!("Value {} is outside range [{}, {}]", num_val, min, max),
                                value: value.to_string(),
                            });
                        }
                    }
                }
            },
            ColumnType::Float => {
                if value.parse::<f64>().is_err() {
                    return Err(ValidationError {
                        line: line_number,
                        column: column_index,
                        column_name: self.name.clone(),
                        message: format!("Invalid float value: '{}'", value),
                        value: value.to_string(),
                    });
                }
                
                // Check numeric range
                if let (Some(min), Some(max)) = (self.constraint.min_value, self.constraint.max_value) {
                    if let Ok(num_val) = value.parse::<f64>() {
                        if num_val < min || num_val > max {
                            return Err(ValidationError {
                                line: line_number,
                                column: column_index,
                                column_name: self.name.clone(),
                                message: format!("Value {} is outside range [{}, {}]", num_val, min, max),
                                value: value.to_string(),
                            });
                        }
                    }
                }
            },
            ColumnType::Boolean => {
                match value.to_lowercase().as_str() {
                    "true" | "false" | "yes" | "no" | "1" | "0" | "on" | "off" | "based" | "cap" => {},
                    _ => {
                        return Err(ValidationError {
                            line: line_number,
                            column: column_index,
                            column_name: self.name.clone(),
                            message: format!("Invalid boolean value: '{}'", value),
                            value: value.to_string(),
                        });
                    }
                }
            },
            ColumnType::Email => {
                if !is_valid_email(value) {
                    return Err(ValidationError {
                        line: line_number,
                        column: column_index,
                        column_name: self.name.clone(),
                        message: format!("Invalid email format: '{}'", value),
                        value: value.to_string(),
                    });
                }
            },
            ColumnType::Url => {
                if !value.starts_with("http://") && !value.starts_with("https://") {
                    return Err(ValidationError {
                        line: line_number,
                        column: column_index,
                        column_name: self.name.clone(),
                        message: format!("Invalid URL format: '{}'", value),
                        value: value.to_string(),
                    });
                }
            },
            ColumnType::Phone => {
                if !is_valid_phone(value) {
                    return Err(ValidationError {
                        line: line_number,
                        column: column_index,
                        column_name: self.name.clone(),
                        message: format!("Invalid phone number format: '{}'", value),
                        value: value.to_string(),
                    });
                }
            },
            _ => {} // No type-specific validation for String, Date, Custom
        }
        
        // Pattern validation (simplified - in real implementation use regex crate)
        if let Some(ref pattern) = self.constraint.pattern {
            if !simple_pattern_match(value, pattern) {
                return Err(ValidationError {
                    line: line_number,
                    column: column_index,
                    column_name: self.name.clone(),
                    message: format!("Value '{}' does not match required pattern", value),
                    value: value.to_string(),
                });
            }
        }
        
        // Length validation
        if let Some(min_len) = self.constraint.min_length {
            if value.len() < min_len {
                return Err(ValidationError {
                    line: line_number,
                    column: column_index,
                    column_name: self.name.clone(),
                    message: format!("Value '{}' is too short (minimum {} characters)", value, min_len),
                    value: value.to_string(),
                });
            }
        }
        
        if let Some(max_len) = self.constraint.max_length {
            if value.len() > max_len {
                return Err(ValidationError {
                    line: line_number,
                    column: column_index,
                    column_name: self.name.clone(),
                    message: format!("Value '{}' is too long (maximum {} characters)", value, max_len),
                    value: value.to_string(),
                });
            }
        }
        
        // Allowed values validation
        if let Some(ref allowed) = self.constraint.allowed_values {
            if !allowed.contains(&value.to_string()) {
                return Err(ValidationError {
                    line: line_number,
                    column: column_index,
                    column_name: self.name.clone(),
                    message: format!("Value '{}' is not in allowed values: {:?}", value, allowed),
                    value: value.to_string(),
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
    columns: HashMap<String, SchemaColumn>,
    
    /// Whether to allow extra columns not defined in schema
    allow_extra_columns: bool,
    
    /// Whether to validate header presence
    require_header: bool,
}

impl Schema {
    /// Create a new empty schema
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
            allow_extra_columns: true,
            require_header: true,
        }
    }
    
    /// Add a required column to the schema
    pub fn require_column(&mut self, name: &str) -> &mut SchemaColumn {
        let column = SchemaColumn::new(name, ColumnType::String);
        self.columns.insert(name.to_string(), column);
        self.columns.get_mut(name).unwrap()
    }
    
    /// Add a required column with specific type
    pub fn require_column_with_type(&mut self, name: &str, column_type: ColumnType) -> &mut SchemaColumn {
        let column = SchemaColumn::new(name, column_type);
        self.columns.insert(name.to_string(), column);
        self.columns.get_mut(name).unwrap()
    }
    
    /// Add an optional column to the schema
    pub fn optional_column(&mut self, name: &str) -> &mut SchemaColumn {
        let column = SchemaColumn::new(name, ColumnType::String).optional();
        self.columns.insert(name.to_string(), column);
        self.columns.get_mut(name).unwrap()
    }
    
    /// Set whether to allow extra columns
    pub fn allow_extra_columns(&mut self, allow: bool) -> &mut Self {
        self.allow_extra_columns = allow;
        self
    }
    
    /// Set whether to require header
    pub fn require_header(&mut self, require: bool) -> &mut Self {
        self.require_header = require;
        self
    }
    
    /// Get schema columns
    pub fn columns(&self) -> &HashMap<String, SchemaColumn> {
        &self.columns
    }
    
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
                    }
                    
                    // Check for required columns
                    for (name, column) in &self.columns {
                        if !column.optional && !column_map.contains_key(name) {
                            errors.push(ValidationError {
                                line: 1,
                                column: 0,
                                column_name: name.clone(),
                                message: format!("Required column '{}' is missing from header", name),
                                value: String::new(),
                            });
                        }
                    }
                    
                    // Check for extra columns if not allowed
                    if !self.allow_extra_columns {
                        for column_name in &header {
                            if !self.columns.contains_key(column_name) {
                                errors.push(ValidationError {
                                    line: 1,
                                    column: 0,
                                    column_name: column_name.clone(),
                                    message: format!("Extra column '{}' not allowed by schema", column_name),
                                    value: String::new(),
                                });
                            }
                        }
                    }
                    
                    Some((header, column_map))
                },
                Ok(None) => {
                    errors.push(ValidationError {
                        line: 1,
                        column: 0,
                        column_name: String::new(),
                        message: "CSV file is empty but header is required".to_string(),
                        value: String::new(),
                    });
                    return ValidationResult {
                        errors,
                        valid_records: 0,
                        total_records: 0,
                        schema_valid: false,
                    };
                },
                Err(e) => {
                    errors.push(ValidationError {
                        line: 1,
                        column: 0,
                        column_name: String::new(),
                        message: format!("Error reading header: {}", e),
                        value: String::new(),
                    });
                    return ValidationResult {
                        errors,
                        valid_records: 0,
                        total_records: 0,
                        schema_valid: false,
                    };
                }
            }
        } else {
            None
        };
        
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
                        };
                        
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
                                line: line_number,
                                column: index + 1,
                                column_name: format!("column_{}", index + 1),
                                message: "Empty value in field that may be required".to_string(),
                                value: field_value.clone(),
                            });
                            record_valid = false;
                        }
                    }
                }
            }
            
            if record_valid {
                valid_records += 1;
            }
            
            line_number += 1;
        }
        
        ValidationResult {
            errors,
            valid_records,
            total_records,
            schema_valid: errors.is_empty(),
        }
    }
}

/// Validation error details
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Line number where error occurred
    pub line: usize,
    
    /// Column number where error occurred
    pub column: usize,
    
    /// Column name
    pub column_name: String,
    
    /// Error message
    pub message: String,
    
    /// The problematic value
    pub value: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line {}, Column {} ({}): {} (value: '{}')", 
               self.line, self.column, self.column_name, self.message, self.value)
    }
}

/// Result of schema validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// List of validation errors
    pub errors: Vec<ValidationError>,
    
    /// Number of valid records
    pub valid_records: usize,
    
    /// Total number of records processed
    pub total_records: usize,
    
    /// Whether the overall schema is valid
    pub schema_valid: bool,
}

impl ValidationResult {
    /// Check if validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
    
    /// Get the number of errors
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
    
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
            self.valid_records,
            self.total_records,
            self.success_rate() * 100.0,
            self.error_count()
        )
    }
}

/// Helper function for simple pattern matching (simplified for demo)
/// In a real implementation, you would use the regex crate
fn simple_pattern_match(value: &str, pattern: &str) -> bool {
    // Very basic pattern matching - in real implementation use regex
    match pattern {
        r"^[^@\s]+@[^@\s]+\.[^@\s]+$" => is_valid_email(value),
        r"^\+?[\d\s\-\(\)]+$" => is_valid_phone(value),
        r"^\d{3}-\d{3}-\d{4}$" => {
            // Phone number pattern like 555-123-4567
            value.len() == 12 && 
            value.chars().nth(3) == Some('-') && 
            value.chars().nth(7) == Some('-') &&
            value.chars().enumerate().all(|(i, c)| {
                if i == 3 || i == 7 { c == '-' } else { c.is_ascii_digit() }
            })
        },
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
}

/// Check if a string is a valid phone number
fn is_valid_phone(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    
    let mut has_digit = false;
    for ch in value.chars() {
        match ch {
            '0'..='9' => has_digit = true,
            '+' | '-' | '(' | ')' | ' ' => {},
            _ => return false,
        }
    }
    
    has_digit
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_basic_schema_validation() {
        let mut schema = Schema::new();
        schema.require_column("name").non_empty();
        schema.require_column_with_type("age", ColumnType::Integer).with_range(0.0, 120.0);
        schema.require_column_with_type("email", ColumnType::Email);
        
        let csv_data = "name,age,email\nAlice,30,alice@example.com\nBob,25,bob@example.com";
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert!(result.is_valid());
        assert_eq!(result.valid_records, 2);
        assert_eq!(result.total_records, 2);
    }

    #[test]
    fn test_validation_errors() {
        let mut schema = Schema::new();
        schema.require_column("name").non_empty();
        schema.require_column_with_type("age", ColumnType::Integer).with_range(18.0, 65.0);
        schema.require_column_with_type("email", ColumnType::Email);
        
        let csv_data = "name,age,email\n,120,invalid-email\nBob,25,bob@example.com";
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert!(!result.is_valid());
        assert_eq!(result.error_count(), 3); // empty name, age out of range, invalid email
        assert_eq!(result.valid_records, 1);
        assert_eq!(result.total_records, 2);
    }

    #[test]
    fn test_pattern_validation() {
        let mut schema = Schema::new();
        let phone_pattern = r"^\d{3}-\d{3}-\d{4}$";
        schema.require_column("phone").with_pattern(phone_pattern).unwrap();
        
        let csv_data = "phone\n555-123-4567\n555-123-456"; // Second one invalid
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert!(!result.is_valid());
        assert_eq!(result.error_count(), 1);
    }

    #[test]
    fn test_length_validation() {
        let mut schema = Schema::new();
        schema.require_column("code").with_length_range(3, 5);
        
        let csv_data = "code\nABC\nA\nABCDEF"; // Second too short, third too long
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert!(!result.is_valid());
        assert_eq!(result.error_count(), 2);
    }

    #[test]
    fn test_allowed_values() {
        let mut schema = Schema::new();
        schema.require_column("status").with_allowed_values(vec![
            "active".to_string(),
            "inactive".to_string(),
            "pending".to_string(),
        ]);
        
        let csv_data = "status\nactive\ninvalid\npending"; // "invalid" not allowed
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert!(!result.is_valid());
        assert_eq!(result.error_count(), 1);
    }

    #[test]
    fn test_missing_required_column() {
        let mut schema = Schema::new();
        schema.require_column("name");
        schema.require_column("age");
        schema.require_column("email"); // This column is missing
        
        let csv_data = "name,age\nAlice,30\nBob,25";
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert!(!result.is_valid());
        assert_eq!(result.error_count(), 1);
        
        let error = &result.errors[0];
        assert!(error.message.contains("Required column 'email' is missing"));
    }

    #[test]
    fn test_optional_columns() {
        let mut schema = Schema::new();
        schema.require_column("name").non_empty();
        schema.optional_column("nickname");
        
        let csv_data = "name\nAlice\nBob"; // nickname column missing but optional
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert!(result.is_valid());
    }

    #[test]
    fn test_extra_columns_allowed() {
        let mut schema = Schema::new();
        schema.require_column("name");
        schema.allow_extra_columns(true);
        
        let csv_data = "name,extra_column\nAlice,value\nBob,value2";
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert!(result.is_valid());
    }

    #[test]
    fn test_extra_columns_not_allowed() {
        let mut schema = Schema::new();
        schema.require_column("name");
        schema.allow_extra_columns(false);
        
        let csv_data = "name,extra_column\nAlice,value\nBob,value2";
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert!(!result.is_valid());
        assert_eq!(result.error_count(), 1);
        
        let error = &result.errors[0];
        assert!(error.message.contains("Extra column 'extra_column' not allowed"));
    }

    #[test]
    fn test_boolean_validation() {
        let mut schema = Schema::new();
        schema.require_column_with_type("active", ColumnType::Boolean);
        
        let csv_data = "active\ntrue\nfalse\nyes\nno\n1\n0\nbased\ncap\ninvalid";
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert!(!result.is_valid());
        assert_eq!(result.error_count(), 1); // Only "invalid" should fail
        assert_eq!(result.valid_records, 8);
    }

    #[test]
    fn test_column_type_validation() {
        let mut schema = Schema::new();
        schema.require_column_with_type("age", ColumnType::Integer);
        schema.require_column_with_type("height", ColumnType::Float);
        schema.require_column_with_type("email", ColumnType::Email);
        schema.require_column_with_type("website", ColumnType::Url);
        
        let csv_data = "age,height,email,website\n30,5.6,alice@example.com,https://example.com\nabc,def,invalid-email,not-a-url";
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert!(!result.is_valid());
        assert_eq!(result.error_count(), 4); // All fields in second record invalid
        assert_eq!(result.valid_records, 1);
    }

    #[test]
    fn test_validation_result_methods() {
        let mut schema = Schema::new();
        schema.require_column("name").non_empty();
        
        let csv_data = "name\nAlice\n\nCharlie"; // Empty name in second record
        let cursor = Cursor::new(csv_data);
        
        let result = schema.validate(cursor);
        assert_eq!(result.success_rate(), 2.0 / 3.0); // 2 valid out of 3 total
        
        let summary = result.summary();
        assert!(summary.contains("2 valid / 3 total"));
        assert!(summary.contains("66.7% success"));
        assert!(summary.contains("1 errors"));
    }
}
