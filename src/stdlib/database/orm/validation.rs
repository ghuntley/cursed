/// Validation framework for CURSED ORM entities
/// 
/// Provides comprehensive validation rules, custom validators,
/// and validation context management for entity constraints.

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use tracing::{instrument, debug, info, warn, error};

use super::super::{DatabaseError, DatabaseErrorKind, SqlValue};

/// fr fr Validation error with detailed context
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Field that failed validation
    /// Validation rule that failed
    /// CursedError message
    /// Expected value or constraint
    /// Actual value that failed
impl ValidationError {
    /// slay Create new validation error
    pub fn new(field: &str, rule: &str, message: &str) -> Self {
        Self {
        }
    }

    /// facts Create validation error with expected/actual values
    pub fn with_values(field: &str, rule: &str, message: &str, expected: &str, actual: &str) -> Self {
        Self {
        }
    }
// impl Display for ValidationError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Validation failed for field '{}' (rule: {}): {}", 
//                self.field, self.rule, self.message)?;
//         
//         if let (Some(expected), Some(actual)) = (&self.expected, &self.actual) {
//             write!(f, " Expected: {}, Actual: {}", expected, actual)?;
//         }
//         
//         Ok(())
//     }
// }

// impl std::error::CursedError for ValidationError {}
// 
/// fr fr Validation context for entity validation
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// Entity being validated
    /// Field values being validated
    /// Validation options
    /// Custom data for validators
impl ValidationContext {
    /// slay Create new validation context
    pub fn new(entity_type: &str, values: HashMap<String, SqlValue>) -> Self {
        Self {
        }
    }

    /// facts Get field value
    pub fn get_value(&self, field: &str) -> Option<&SqlValue> {
        self.values.get(field)
    /// periodt Add custom data
    pub fn with_custom_data(mut self, key: &str, value: &str) -> Self {
        self.custom_data.insert(key.to_string(), value.to_string());
        self
    }
}

/// fr fr Validation options
#[derive(Debug, Clone)]
pub struct ValidationOptions {
    /// Stop on first validation error
    /// Include field values in error messages
    /// Skip validation for null values
impl Default for ValidationOptions {
    fn default() -> Self {
        Self {
        }
    }
/// fr fr Main validator trait
pub trait Validator: Send + Sync + Debug {
    /// Validation rule name
    fn rule_name(&self) -> &str;
    
    /// Validate a field value
    fn validate(&self, field: &str, value: &SqlValue, context: &ValidationContext) -> crate::error::Result<()>;
    
    /// Check if validator applies to field type
    fn applies_to(&self, _field_type: &str) -> bool {
        true
    }
}

/// fr fr Validation rule enum for common validations
#[derive(Debug, Clone)]
pub enum ValidationRule {
    /// Field is required (not null/empty)
    /// Minimum length for strings
    /// Maximum length for strings
    /// Exact length for strings
    /// Minimum value for numbers
    /// Maximum value for numbers
    /// Value must be in range
    /// Email format validation
    /// URL format validation
    /// Phone number format validation
    /// Custom regex pattern
    /// Value must be in list
    /// Value must not be in list
    /// Custom validation function
impl ValidationRule {
    /// bestie Convert to validator instance
    pub fn to_validator(&self) -> Box<dyn Validator> {
        match self {
            ValidationRule::Pattern { regex, message } => Box::new(Pattern { 
                message: message.clone() 
            ValidationRule::Custom { validator } => Box::new(CustomValidator { 
                validator: validator.clone() 
        }
    }
/// fr fr Entity validator that manages all validation rules
#[derive(Debug)]
pub struct EntityValidator {
    /// Field validation rules
    /// Entity-level validators
    /// Validation options
impl EntityValidator {
    /// slay Create new entity validator
    #[instrument]
    pub fn new() -> Self {
        info!("Creating new entity validator");
        Self {
        }
    }

    /// facts Add validation rule for field
    #[instrument(skip(self, validator))]
    pub fn add_field_rule(&mut self, field: &str, validator: Box<dyn Validator>) {
        debug!(field = field, rule = validator.rule_name(), "Adding field validation rule");
        
        self.field_rules
            .entry(field.to_string())
            .or_insert_with(Vec::new)
            .push(validator);
    /// periodt Add entity-level validator
    #[instrument(skip(self, validator))]
    pub fn add_entity_validator(&mut self, validator: Box<dyn Validator>) {
        debug!(rule = validator.rule_name(), "Adding entity-level validator");
        self.entity_validators.push(validator);
    /// bestie Set validation options
    pub fn with_options(mut self, options: ValidationOptions) -> Self {
        self.options = options;
        self
    /// yolo Validate entity with all rules
    #[instrument(skip(self, context))]
    pub fn validate(&self, context: &ValidationContext) -> crate::error::Result<()> {
        debug!(entity = %context.entity_type, "Validating entity");
        
        let mut errors = Vec::new();

        // Validate individual fields
        for (field, validators) in &self.field_rules {
            if let Some(value) = context.get_value(field) {
                // Skip null values if configured
                if self.options.skip_null_values && matches!(value, SqlValue::Null) {
                    continue;
                for validator in validators {
                    match validator.validate(field, value, context) {
                        Err(error) => {
                            errors.push(error);
                            if self.options.fail_fast {
                                return Err(errors);
                            }
                        }
                    }
                }
            }
        }

        // Validate entity-level rules
        for validator in &self.entity_validators {
            // For entity-level validation, we validate against the entire context
            let dummy_value = SqlValue::Null;
            match validator.validate("_entity", &dummy_value, context) {
                Err(error) => {
                    errors.push(error);
                    if self.options.fail_fast {
                        return Err(errors);
                    }
                }
            }
        }

        if errors.is_empty() {
            info!(entity = %context.entity_type, "Entity validation passed");
            Ok(())
        } else {
            warn!(entity = %context.entity_type, error_count = errors.len(), "Entity validation failed");
            Err(errors)
        }
    }
// Built-in validator implementations

/// fr fr Required field validator
#[derive(Debug)]
pub struct Required;

impl Validator for Required {
    fn rule_name(&self) -> &str {
        "required"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        match value {
        }
    }
/// fr fr Minimum length validator
#[derive(Debug)]
pub struct MinLength {
impl Validator for MinLength {
    fn rule_name(&self) -> &str {
        "min_length"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        match value {
            SqlValue::String(s) => {
                if s.len() < self.min {
                    Err(ValidationError::with_values(
                    ))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()), // Skip non-string values
        }
    }

    fn applies_to(&self, field_type: &str) -> bool {
        field_type == "string" || field_type == "text"
    }
}

/// fr fr Maximum length validator
#[derive(Debug)]
pub struct MaxLength {
impl Validator for MaxLength {
    fn rule_name(&self) -> &str {
        "max_length"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        match value {
            SqlValue::String(s) => {
                if s.len() > self.max {
                    Err(ValidationError::with_values(
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }

    fn applies_to(&self, field_type: &str) -> bool {
        field_type == "string" || field_type == "text"
    }
}

/// fr fr Exact length validator
#[derive(Debug)]
pub struct ExactLength {
impl Validator for ExactLength {
    fn rule_name(&self) -> &str {
        "exact_length"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        match value {
            SqlValue::String(s) => {
                if s.len() != self.length {
                    Err(ValidationError::with_values(
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }
/// fr fr Minimum value validator
#[derive(Debug)]
pub struct MinValue {
impl Validator for MinValue {
    fn rule_name(&self) -> &str {
        "min_value"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        let numeric_value = match value {
            _ => return Ok(()), // Skip non-numeric values

        if numeric_value < self.min {
            Err(ValidationError::with_values(
            ))
        } else {
            Ok(())
        }
    }

    fn applies_to(&self, field_type: &str) -> bool {
        field_type == "number" || field_type == "integer" || field_type == "float"
    }
}

/// fr fr Maximum value validator
#[derive(Debug)]
pub struct MaxValue {
impl Validator for MaxValue {
    fn rule_name(&self) -> &str {
        "max_value"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        let numeric_value = match value {

        if numeric_value > self.max {
            Err(ValidationError::with_values(
            ))
        } else {
            Ok(())
        }
    }
/// fr fr Range validator
#[derive(Debug)]
pub struct Range {
impl Validator for Range {
    fn rule_name(&self) -> &str {
        "range"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        let numeric_value = match value {

        if numeric_value < self.min || numeric_value > self.max {
            Err(ValidationError::with_values(
            ))
        } else {
            Ok(())
        }
    }
/// fr fr Email format validator
#[derive(Debug)]
pub struct EmailFormat;

impl Validator for EmailFormat {
    fn rule_name(&self) -> &str {
        "email_format"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        match value {
            SqlValue::String(s) => {
                // Simple email validation (would use proper regex in production)
                if s.contains('@') && s.contains('.') && s.len() > 5 {
                    Ok(())
                } else {
                    Err(ValidationError::new(field, "email_format", "Invalid email format"))
                }
            }
        }
    }

    fn applies_to(&self, field_type: &str) -> bool {
        field_type == "email" || field_type == "string"
    }
}

/// fr fr URL format validator
#[derive(Debug)]
pub struct UrlFormat;

impl Validator for UrlFormat {
    fn rule_name(&self) -> &str {
        "url_format"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        match value {
            SqlValue::String(s) => {
                // Simple URL validation
                if s.starts_with("http://") || s.starts_with("https://") {
                    Ok(())
                } else {
                    Err(ValidationError::new(field, "url_format", "Invalid URL format"))
                }
            }
        }
    }
/// fr fr Phone format validator
#[derive(Debug)]
pub struct PhoneFormat;

impl Validator for PhoneFormat {
    fn rule_name(&self) -> &str {
        "phone_format"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        match value {
            SqlValue::String(s) => {
                // Simple phone validation (digits, spaces, dashes, parentheses)
                let cleaned = s.chars().filter(|c| c.is_ascii_digit()).count();
                if cleaned >= 10 && cleaned <= 15 {
                    Ok(())
                } else {
                    Err(ValidationError::new(field, "phone_format", "Invalid phone number format"))
                }
            }
        }
    }
/// fr fr Pattern validator using regex
#[derive(Debug)]
pub struct Pattern {
impl Validator for Pattern {
    fn rule_name(&self) -> &str {
        "pattern"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        match value {
            SqlValue::String(s) => {
                // Simplified pattern matching (would use proper regex crate)
                // For now, just check if pattern is contained in string
                if s.contains(&self.regex) {
                    Ok(())
                } else {
                    Err(ValidationError::new(field, "pattern", &self.message))
                }
            }
        }
    }
/// fr fr In validator (value must be in list)
#[derive(Debug)]
pub struct In {
impl Validator for In {
    fn rule_name(&self) -> &str {
        "in"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        let string_value = match value {

        if self.values.contains(&string_value) {
            Ok(())
        } else {
            Err(ValidationError::with_values(
            ))
        }
    }
/// fr fr NotIn validator (value must not be in list)
#[derive(Debug)]
pub struct NotIn {
impl Validator for NotIn {
    fn rule_name(&self) -> &str {
        "not_in"
    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> crate::error::Result<()> {
        let string_value = match value {

        if !self.values.contains(&string_value) {
            Ok(())
        } else {
            Err(ValidationError::with_values(
            ))
        }
    }
/// fr fr Custom validator wrapper
#[derive(Debug)]
pub struct CustomValidator {
impl Validator for CustomValidator {
    fn rule_name(&self) -> &str {
        self.validator.rule_name()
    fn validate(&self, field: &str, value: &SqlValue, context: &ValidationContext) -> crate::error::Result<()> {
        self.validator.validate(field, value, context)
    fn applies_to(&self, field_type: &str) -> bool {
        self.validator.applies_to(field_type)
    }
}

