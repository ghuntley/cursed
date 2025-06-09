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
    pub field: String,
    /// Validation rule that failed
    pub rule: String,
    /// Error message
    pub message: String,
    /// Expected value or constraint
    pub expected: Option<String>,
    /// Actual value that failed
    pub actual: Option<String>,
}

impl ValidationError {
    /// slay Create new validation error
    pub fn new(field: &str, rule: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            rule: rule.to_string(),
            message: message.to_string(),
            expected: None,
            actual: None,
        }
    }

    /// facts Create validation error with expected/actual values
    pub fn with_values(field: &str, rule: &str, message: &str, expected: &str, actual: &str) -> Self {
        Self {
            field: field.to_string(),
            rule: rule.to_string(),
            message: message.to_string(),
            expected: Some(expected.to_string()),
            actual: Some(actual.to_string()),
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation failed for field '{}' (rule: {}): {}", 
               self.field, self.rule, self.message)?;
        
        if let (Some(expected), Some(actual)) = (&self.expected, &self.actual) {
            write!(f, " Expected: {}, Actual: {}", expected, actual)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for ValidationError {}

/// fr fr Validation context for entity validation
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// Entity being validated
    pub entity_type: String,
    /// Field values being validated
    pub values: HashMap<String, SqlValue>,
    /// Validation options
    pub options: ValidationOptions,
    /// Custom data for validators
    pub custom_data: HashMap<String, String>,
}

impl ValidationContext {
    /// slay Create new validation context
    pub fn new(entity_type: &str, values: HashMap<String, SqlValue>) -> Self {
        Self {
            entity_type: entity_type.to_string(),
            values,
            options: ValidationOptions::default(),
            custom_data: HashMap::new(),
        }
    }

    /// facts Get field value
    pub fn get_value(&self, field: &str) -> Option<&SqlValue> {
        self.values.get(field)
    }

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
    pub fail_fast: bool,
    /// Include field values in error messages
    pub include_values: bool,
    /// Skip validation for null values
    pub skip_null_values: bool,
}

impl Default for ValidationOptions {
    fn default() -> Self {
        Self {
            fail_fast: false,
            include_values: true,
            skip_null_values: true,
        }
    }
}

/// fr fr Main validator trait
pub trait Validator: Send + Sync + Debug {
    /// Validation rule name
    fn rule_name(&self) -> &str;
    
    /// Validate a field value
    fn validate(&self, field: &str, value: &SqlValue, context: &ValidationContext) -> Result<(), ValidationError>;
    
    /// Check if validator applies to field type
    fn applies_to(&self, _field_type: &str) -> bool {
        true
    }
}

/// fr fr Validation rule enum for common validations
#[derive(Debug, Clone)]
pub enum ValidationRule {
    /// Field is required (not null/empty)
    Required,
    /// Minimum length for strings
    MinLength { min: usize },
    /// Maximum length for strings
    MaxLength { max: usize },
    /// Exact length for strings
    ExactLength { length: usize },
    /// Minimum value for numbers
    MinValue { min: f64 },
    /// Maximum value for numbers
    MaxValue { max: f64 },
    /// Value must be in range
    Range { min: f64, max: f64 },
    /// Email format validation
    EmailFormat,
    /// URL format validation
    UrlFormat,
    /// Phone number format validation
    PhoneFormat,
    /// Custom regex pattern
    Pattern { regex: String, message: String },
    /// Value must be in list
    In { values: Vec<String> },
    /// Value must not be in list
    NotIn { values: Vec<String> },
    /// Custom validation function
    Custom { validator: Arc<dyn Validator> },
}

impl ValidationRule {
    /// bestie Convert to validator instance
    pub fn to_validator(&self) -> Box<dyn Validator> {
        match self {
            ValidationRule::Required => Box::new(Required),
            ValidationRule::MinLength { min } => Box::new(MinLength { min: *min }),
            ValidationRule::MaxLength { max } => Box::new(MaxLength { max: *max }),
            ValidationRule::ExactLength { length } => Box::new(ExactLength { length: *length }),
            ValidationRule::MinValue { min } => Box::new(MinValue { min: *min }),
            ValidationRule::MaxValue { max } => Box::new(MaxValue { max: *max }),
            ValidationRule::Range { min, max } => Box::new(Range { min: *min, max: *max }),
            ValidationRule::EmailFormat => Box::new(EmailFormat),
            ValidationRule::UrlFormat => Box::new(UrlFormat),
            ValidationRule::PhoneFormat => Box::new(PhoneFormat),
            ValidationRule::Pattern { regex, message } => Box::new(Pattern { 
                regex: regex.clone(), 
                message: message.clone() 
            }),
            ValidationRule::In { values } => Box::new(In { values: values.clone() }),
            ValidationRule::NotIn { values } => Box::new(NotIn { values: values.clone() }),
            ValidationRule::Custom { validator } => Box::new(CustomValidator { 
                validator: validator.clone() 
            }),
        }
    }
}

/// fr fr Entity validator that manages all validation rules
#[derive(Debug)]
pub struct EntityValidator {
    /// Field validation rules
    field_rules: HashMap<String, Vec<Box<dyn Validator>>>,
    /// Entity-level validators
    entity_validators: Vec<Box<dyn Validator>>,
    /// Validation options
    options: ValidationOptions,
}

impl EntityValidator {
    /// slay Create new entity validator
    #[instrument]
    pub fn new() -> Self {
        info!("Creating new entity validator");
        Self {
            field_rules: HashMap::new(),
            entity_validators: Vec::new(),
            options: ValidationOptions::default(),
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
    }

    /// periodt Add entity-level validator
    #[instrument(skip(self, validator))]
    pub fn add_entity_validator(&mut self, validator: Box<dyn Validator>) {
        debug!(rule = validator.rule_name(), "Adding entity-level validator");
        self.entity_validators.push(validator);
    }

    /// bestie Set validation options
    pub fn with_options(mut self, options: ValidationOptions) -> Self {
        self.options = options;
        self
    }

    /// yolo Validate entity with all rules
    #[instrument(skip(self, context))]
    pub fn validate(&self, context: &ValidationContext) -> Result<(), Vec<ValidationError>> {
        debug!(entity = %context.entity_type, "Validating entity");
        
        let mut errors = Vec::new();

        // Validate individual fields
        for (field, validators) in &self.field_rules {
            if let Some(value) = context.get_value(field) {
                // Skip null values if configured
                if self.options.skip_null_values && matches!(value, SqlValue::Null) {
                    continue;
                }

                for validator in validators {
                    match validator.validate(field, value, context) {
                        Ok(_) => continue,
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
                Ok(_) => continue,
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
}

// Built-in validator implementations

/// fr fr Required field validator
#[derive(Debug)]
pub struct Required;

impl Validator for Required {
    fn rule_name(&self) -> &str {
        "required"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
        match value {
            SqlValue::Null => Err(ValidationError::new(field, "required", "Field is required")),
            SqlValue::String(s) if s.is_empty() => Err(ValidationError::new(field, "required", "Field cannot be empty")),
            _ => Ok(()),
        }
    }
}

/// fr fr Minimum length validator
#[derive(Debug)]
pub struct MinLength {
    pub min: usize,
}

impl Validator for MinLength {
    fn rule_name(&self) -> &str {
        "min_length"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
        match value {
            SqlValue::String(s) => {
                if s.len() < self.min {
                    Err(ValidationError::with_values(
                        field,
                        "min_length",
                        &format!("Field must be at least {} characters long", self.min),
                        &self.min.to_string(),
                        &s.len().to_string(),
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
    pub max: usize,
}

impl Validator for MaxLength {
    fn rule_name(&self) -> &str {
        "max_length"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
        match value {
            SqlValue::String(s) => {
                if s.len() > self.max {
                    Err(ValidationError::with_values(
                        field,
                        "max_length",
                        &format!("Field must be no more than {} characters long", self.max),
                        &self.max.to_string(),
                        &s.len().to_string(),
                    ))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    fn applies_to(&self, field_type: &str) -> bool {
        field_type == "string" || field_type == "text"
    }
}

/// fr fr Exact length validator
#[derive(Debug)]
pub struct ExactLength {
    pub length: usize,
}

impl Validator for ExactLength {
    fn rule_name(&self) -> &str {
        "exact_length"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
        match value {
            SqlValue::String(s) => {
                if s.len() != self.length {
                    Err(ValidationError::with_values(
                        field,
                        "exact_length",
                        &format!("Field must be exactly {} characters long", self.length),
                        &self.length.to_string(),
                        &s.len().to_string(),
                    ))
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

/// fr fr Minimum value validator
#[derive(Debug)]
pub struct MinValue {
    pub min: f64,
}

impl Validator for MinValue {
    fn rule_name(&self) -> &str {
        "min_value"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
        let numeric_value = match value {
            SqlValue::Integer(i) => *i as f64,
            SqlValue::Float(f) => *f,
            _ => return Ok(()), // Skip non-numeric values
        };

        if numeric_value < self.min {
            Err(ValidationError::with_values(
                field,
                "min_value",
                &format!("Value must be at least {}", self.min),
                &self.min.to_string(),
                &numeric_value.to_string(),
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
    pub max: f64,
}

impl Validator for MaxValue {
    fn rule_name(&self) -> &str {
        "max_value"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
        let numeric_value = match value {
            SqlValue::Integer(i) => *i as f64,
            SqlValue::Float(f) => *f,
            _ => return Ok(()),
        };

        if numeric_value > self.max {
            Err(ValidationError::with_values(
                field,
                "max_value",
                &format!("Value must be no more than {}", self.max),
                &self.max.to_string(),
                &numeric_value.to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

/// fr fr Range validator
#[derive(Debug)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

impl Validator for Range {
    fn rule_name(&self) -> &str {
        "range"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
        let numeric_value = match value {
            SqlValue::Integer(i) => *i as f64,
            SqlValue::Float(f) => *f,
            _ => return Ok(()),
        };

        if numeric_value < self.min || numeric_value > self.max {
            Err(ValidationError::with_values(
                field,
                "range",
                &format!("Value must be between {} and {}", self.min, self.max),
                &format!("{}-{}", self.min, self.max),
                &numeric_value.to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

/// fr fr Email format validator
#[derive(Debug)]
pub struct EmailFormat;

impl Validator for EmailFormat {
    fn rule_name(&self) -> &str {
        "email_format"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
        match value {
            SqlValue::String(s) => {
                // Simple email validation (would use proper regex in production)
                if s.contains('@') && s.contains('.') && s.len() > 5 {
                    Ok(())
                } else {
                    Err(ValidationError::new(field, "email_format", "Invalid email format"))
                }
            }
            _ => Ok(()),
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
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
        match value {
            SqlValue::String(s) => {
                // Simple URL validation
                if s.starts_with("http://") || s.starts_with("https://") {
                    Ok(())
                } else {
                    Err(ValidationError::new(field, "url_format", "Invalid URL format"))
                }
            }
            _ => Ok(()),
        }
    }
}

/// fr fr Phone format validator
#[derive(Debug)]
pub struct PhoneFormat;

impl Validator for PhoneFormat {
    fn rule_name(&self) -> &str {
        "phone_format"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
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
            _ => Ok(()),
        }
    }
}

/// fr fr Pattern validator using regex
#[derive(Debug)]
pub struct Pattern {
    pub regex: String,
    pub message: String,
}

impl Validator for Pattern {
    fn rule_name(&self) -> &str {
        "pattern"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
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
            _ => Ok(()),
        }
    }
}

/// fr fr In validator (value must be in list)
#[derive(Debug)]
pub struct In {
    pub values: Vec<String>,
}

impl Validator for In {
    fn rule_name(&self) -> &str {
        "in"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
        let string_value = match value {
            SqlValue::String(s) => s.clone(),
            SqlValue::Integer(i) => i.to_string(),
            SqlValue::Float(f) => f.to_string(),
            SqlValue::Boolean(b) => b.to_string(),
            _ => return Ok(()),
        };

        if self.values.contains(&string_value) {
            Ok(())
        } else {
            Err(ValidationError::with_values(
                field,
                "in",
                "Value must be one of the allowed values",
                &self.values.join(", "),
                &string_value,
            ))
        }
    }
}

/// fr fr NotIn validator (value must not be in list)
#[derive(Debug)]
pub struct NotIn {
    pub values: Vec<String>,
}

impl Validator for NotIn {
    fn rule_name(&self) -> &str {
        "not_in"
    }

    fn validate(&self, field: &str, value: &SqlValue, _context: &ValidationContext) -> Result<(), ValidationError> {
        let string_value = match value {
            SqlValue::String(s) => s.clone(),
            SqlValue::Integer(i) => i.to_string(),
            SqlValue::Float(f) => f.to_string(),
            SqlValue::Boolean(b) => b.to_string(),
            _ => return Ok(()),
        };

        if !self.values.contains(&string_value) {
            Ok(())
        } else {
            Err(ValidationError::with_values(
                field,
                "not_in",
                "Value must not be one of the forbidden values",
                &format!("not in [{}]", self.values.join(", ")),
                &string_value,
            ))
        }
    }
}

/// fr fr Custom validator wrapper
#[derive(Debug)]
pub struct CustomValidator {
    pub validator: Arc<dyn Validator>,
}

impl Validator for CustomValidator {
    fn rule_name(&self) -> &str {
        self.validator.rule_name()
    }

    fn validate(&self, field: &str, value: &SqlValue, context: &ValidationContext) -> Result<(), ValidationError> {
        self.validator.validate(field, value, context)
    }

    fn applies_to(&self, field_type: &str) -> bool {
        self.validator.applies_to(field_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_required_validator() {
        let validator = Required;
        let context = ValidationContext::new("test", HashMap::new());

        // Test null value (should fail)
        let result = validator.validate("field", &SqlValue::Null, &context);
        assert!(result.is_err());

        // Test empty string (should fail)
        let result = validator.validate("field", &SqlValue::String("".to_string()), &context);
        assert!(result.is_err());

        // Test valid value (should pass)
        let result = validator.validate("field", &SqlValue::String("value".to_string()), &context);
        assert!(result.is_ok());
    }

    #[traced_test]
    #[test]
    fn test_min_length_validator() {
        let validator = MinLength { min: 5 };
        let context = ValidationContext::new("test", HashMap::new());

        // Test short string (should fail)
        let result = validator.validate("field", &SqlValue::String("abc".to_string()), &context);
        assert!(result.is_err());

        // Test exact length (should pass)
        let result = validator.validate("field", &SqlValue::String("abcde".to_string()), &context);
        assert!(result.is_ok());

        // Test longer string (should pass)
        let result = validator.validate("field", &SqlValue::String("abcdef".to_string()), &context);
        assert!(result.is_ok());
    }

    #[traced_test]
    #[test]
    fn test_max_length_validator() {
        let validator = MaxLength { max: 5 };
        let context = ValidationContext::new("test", HashMap::new());

        // Test long string (should fail)
        let result = validator.validate("field", &SqlValue::String("abcdef".to_string()), &context);
        assert!(result.is_err());

        // Test exact length (should pass)
        let result = validator.validate("field", &SqlValue::String("abcde".to_string()), &context);
        assert!(result.is_ok());

        // Test shorter string (should pass)
        let result = validator.validate("field", &SqlValue::String("abc".to_string()), &context);
        assert!(result.is_ok());
    }

    #[traced_test]
    #[test]
    fn test_min_value_validator() {
        let validator = MinValue { min: 10.0 };
        let context = ValidationContext::new("test", HashMap::new());

        // Test small value (should fail)
        let result = validator.validate("field", &SqlValue::Integer(5), &context);
        assert!(result.is_err());

        // Test exact value (should pass)
        let result = validator.validate("field", &SqlValue::Float(10.0), &context);
        assert!(result.is_ok());

        // Test larger value (should pass)
        let result = validator.validate("field", &SqlValue::Integer(15), &context);
        assert!(result.is_ok());
    }

    #[traced_test]
    #[test]
    fn test_email_format_validator() {
        let validator = EmailFormat;
        let context = ValidationContext::new("test", HashMap::new());

        // Test invalid email (should fail)
        let result = validator.validate("field", &SqlValue::String("invalid".to_string()), &context);
        assert!(result.is_err());

        // Test valid email (should pass)
        let result = validator.validate("field", &SqlValue::String("test@example.com".to_string()), &context);
        assert!(result.is_ok());
    }

    #[traced_test]
    #[test]
    fn test_in_validator() {
        let validator = In {
            values: Vec::from(["red".to_string(), "green".to_string(), "blue".to_string()]),
        };
        let context = ValidationContext::new("test", HashMap::new());

        // Test invalid value (should fail)
        let result = validator.validate("field", &SqlValue::String("yellow".to_string()), &context);
        assert!(result.is_err());

        // Test valid value (should pass)
        let result = validator.validate("field", &SqlValue::String("red".to_string()), &context);
        assert!(result.is_ok());
    }

    #[traced_test]
    #[test]
    fn test_entity_validator() {
        let mut entity_validator = EntityValidator::new();
        
        entity_validator.add_field_rule("name", Box::new(Required));
        entity_validator.add_field_rule("name", Box::new(MinLength { min: 2 }));
        entity_validator.add_field_rule("email", Box::new(EmailFormat));

        let mut values = HashMap::new();
        values.insert("name".to_string(), SqlValue::String("John".to_string()));
        values.insert("email".to_string(), SqlValue::String("john@example.com".to_string()));
        
        let context = ValidationContext::new("user", values);
        
        let result = entity_validator.validate(&context);
        assert!(result.is_ok());
    }

    #[traced_test]
    #[test]
    fn test_entity_validator_with_errors() {
        let mut entity_validator = EntityValidator::new();
        
        entity_validator.add_field_rule("name", Box::new(Required));
        entity_validator.add_field_rule("name", Box::new(MinLength { min: 10 }));

        let mut values = HashMap::new();
        values.insert("name".to_string(), SqlValue::String("Bob".to_string()));
        
        let context = ValidationContext::new("user", values);
        
        let result = entity_validator.validate(&context);
        assert!(result.is_err());
        
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].rule, "min_length");
        }
    }

    #[traced_test]
    #[test]
    fn test_validation_rule_to_validator() {
        let rule = ValidationRule::MinLength { min: 5 };
        let validator = rule.to_validator();
        
        assert_eq!(validator.rule_name(), "min_length");
    }

    #[traced_test]
    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::with_values(
            "name",
            "min_length",
            "Field too short",
            "5",
            "3"
        );
        
        let error_string = error.to_string();
        assert!(error_string.contains("min_length"));
        assert!(error_string.contains("Expected: 5"));
        assert!(error_string.contains("Actual: 3"));
    }
}
