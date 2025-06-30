//! I/O functionality for validation

use crate::error::CursedError;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use regex::Regex;

/// Result type for I/O operations
pub type IOResult<T> = Result<T, CursedError>;

/// Validation result
pub type ValidationResult = Result<(), Vec<String>>;

/// Required field validator
#[derive(Debug, Clone)]
pub struct Required {
    pub message: String,
}

/// Minimum length validator
#[derive(Debug, Clone)]
pub struct MinLength {
    pub min_length: usize,
    pub message: String,
}

/// Maximum length validator
#[derive(Debug, Clone)]
pub struct MaxLength {
    pub max_length: usize,
    pub message: String,
}

/// Email format validator
#[derive(Debug, Clone)]
pub struct EmailFormat {
    pub message: String,
}

/// Custom validation function
pub type ValidationFunction = fn(&str) -> ValidationResult;

/// Custom validator
#[derive(Debug)]
pub struct CustomValidator {
    pub name: String,
    pub validator: ValidationFunction,
    pub message: String,
}

impl Required {
    pub fn new() -> Self {
        Self {
            message: "This field is required".to_string(),
        }
    }
    
    pub fn with_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
    
    pub fn validate(&self, value: Option<&str>) -> ValidationResult {
        match value {
            Some(v) if !v.trim().is_empty() => Ok(()),
            _ => Err(vec![self.message.clone()]),
        }
    }
}

impl MinLength {
    pub fn new(min_length: usize) -> Self {
        Self {
            min_length,
            message: format!("Must be at least {} characters long", min_length),
        }
    }
    
    pub fn with_message(min_length: usize, message: &str) -> Self {
        Self {
            min_length,
            message: message.to_string(),
        }
    }
    
    pub fn validate(&self, value: &str) -> ValidationResult {
        if value.len() >= self.min_length {
            Ok(())
        } else {
            Err(vec![self.message.clone()])
        }
    }
}

impl MaxLength {
    pub fn new(max_length: usize) -> Self {
        Self {
            max_length,
            message: format!("Must be no more than {} characters long", max_length),
        }
    }
    
    pub fn with_message(max_length: usize, message: &str) -> Self {
        Self {
            max_length,
            message: message.to_string(),
        }
    }
    
    pub fn validate(&self, value: &str) -> ValidationResult {
        if value.len() <= self.max_length {
            Ok(())
        } else {
            Err(vec![self.message.clone()])
        }
    }
}

impl EmailFormat {
    pub fn new() -> Self {
        Self {
            message: "Must be a valid email address".to_string(),
        }
    }
    
    pub fn with_message(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
    
    pub fn validate(&self, value: &str) -> ValidationResult {
        // Simple email regex pattern
        let email_pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
        
        match Regex::new(email_pattern) {
            Ok(regex) => {
                if regex.is_match(value) {
                    Ok(())
                } else {
                    Err(vec![self.message.clone()])
                }
            }
            Err(_) => Err(vec!["Invalid email validation pattern".to_string()]),
        }
    }
}

impl CustomValidator {
    pub fn new(name: &str, validator: ValidationFunction) -> Self {
        Self {
            name: name.to_string(),
            validator,
            message: format!("Custom validation failed for {}", name),
        }
    }
    
    pub fn with_message(name: &str, validator: ValidationFunction, message: &str) -> Self {
        Self {
            name: name.to_string(),
            validator,
            message: message.to_string(),
        }
    }
    
    pub fn validate(&self, value: &str) -> ValidationResult {
        (self.validator)(value).map_err(|mut errors| {
            if errors.is_empty() {
                errors.push(self.message.clone());
            }
            errors
        })
    }
}

/// Validation rule set for a field
#[derive(Debug)]
pub struct FieldValidator {
    pub field_name: String,
    pub required: Option<Required>,
    pub min_length: Option<MinLength>,
    pub max_length: Option<MaxLength>,
    pub email_format: Option<EmailFormat>,
    pub custom_validators: Vec<CustomValidator>,
}

impl FieldValidator {
    pub fn new(field_name: &str) -> Self {
        Self {
            field_name: field_name.to_string(),
            required: None,
            min_length: None,
            max_length: None,
            email_format: None,
            custom_validators: Vec::new(),
        }
    }
    
    pub fn required(mut self, validator: Required) -> Self {
        self.required = Some(validator);
        self
    }
    
    pub fn min_length(mut self, validator: MinLength) -> Self {
        self.min_length = Some(validator);
        self
    }
    
    pub fn max_length(mut self, validator: MaxLength) -> Self {
        self.max_length = Some(validator);
        self
    }
    
    pub fn email_format(mut self, validator: EmailFormat) -> Self {
        self.email_format = Some(validator);
        self
    }
    
    pub fn custom(mut self, validator: CustomValidator) -> Self {
        self.custom_validators.push(validator);
        self
    }
    
    pub fn validate(&self, value: Option<&str>) -> ValidationResult {
        let mut errors = Vec::new();
        
        // Check required validation first
        if let Some(ref required) = self.required {
            if let Err(mut req_errors) = required.validate(value) {
                errors.append(&mut req_errors);
                return Err(errors); // Stop validation if required fails
            }
        }
        
        // If we have a value, continue with other validations
        if let Some(val) = value {
            if let Some(ref min_len) = self.min_length {
                if let Err(mut min_errors) = min_len.validate(val) {
                    errors.append(&mut min_errors);
                }
            }
            
            if let Some(ref max_len) = self.max_length {
                if let Err(mut max_errors) = max_len.validate(val) {
                    errors.append(&mut max_errors);
                }
            }
            
            if let Some(ref email) = self.email_format {
                if let Err(mut email_errors) = email.validate(val) {
                    errors.append(&mut email_errors);
                }
            }
            
            for custom in &self.custom_validators {
                if let Err(mut custom_errors) = custom.validate(val) {
                    errors.append(&mut custom_errors);
                }
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// I/O operations handler
pub struct IOHandler {
    buffer_size: usize,
}

impl IOHandler {
    /// Create a new I/O handler
    pub fn new() -> Self {
        Self {
            buffer_size: 8192,
        }
    }
    
    /// Set buffer size
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }
    
    /// Read from a reader
    pub fn read_all<R: Read>(&self, mut reader: R) -> IOResult<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)
            .map_err(|e| CursedError::runtime_error(&format!("Read error: {}", e)))?;
        Ok(buffer)
    }
    
    /// Write to a writer
    pub fn write_all<W: Write>(&self, mut writer: W, data: &[u8]) -> IOResult<()> {
        writer.write_all(data)
            .map_err(|e| CursedError::runtime_error(&format!("Write error: {}", e)))?;
        Ok(())
    }
    
    /// Read string from reader
    pub fn read_string<R: Read>(&self, reader: R) -> IOResult<String> {
        let bytes = self.read_all(reader)?;
        String::from_utf8(bytes)
            .map_err(|e| CursedError::runtime_error(&format!("UTF-8 decode error: {}", e)))
    }
    
    /// Write string to writer
    pub fn write_string<W: Write>(&self, writer: W, text: &str) -> IOResult<()> {
        self.write_all(writer, text.as_bytes())
    }
}

impl Default for IOHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize I/O processing
pub fn init_validation() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_data = b"test data";
    let mut cursor = std::io::Cursor::new(test_data);
    let result = handler.read_all(&mut cursor)?;
    if result != test_data {
        return Err(CursedError::runtime_error("I/O test failed"));
    }
    println!("📁 I/O processing (validation) initialized");
    Ok(())
}

/// Test I/O functionality
pub fn test_validation() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_string = "Hello, CURSED I/O!";
    let mut buffer = Vec::new();
    handler.write_string(&mut buffer, test_string)?;
    let result = handler.read_string(std::io::Cursor::new(&buffer))?;
    if result != test_string {
        return Err(CursedError::runtime_error("I/O string test failed"));
    }
    Ok(())
}
