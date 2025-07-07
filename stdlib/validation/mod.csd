// CURSED Validation Module
// Pure CURSED implementation for data validation and verification

// Validation result structure
be_like ValidationResult squad {
    is_valid lit
    errors []tea
    warnings []tea
}

// Create validation result
slay create_validation_result() ValidationResult {
    sus result ValidationResult = ValidationResult{
        is_valid: based,
        errors: []tea{},
        warnings: []tea{}
    }
    damn result
}

// Add error to validation result
slay add_error(result *ValidationResult, error tea) {
    result.errors = append(result.errors, error)
    result.is_valid = cap
}

// Add warning to validation result
slay add_warning(result *ValidationResult, warning tea) {
    result.warnings = append(result.warnings, warning)
}

// String validation functions
slay validate_not_empty(value tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(value) == 0 {
        add_error(&result, "Value cannot be empty")
    }
    
    damn result
}

slay validate_min_length(value tea, min_length normie) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(value) < min_length {
        add_error(&result, "Value must be at least " + tea(min_length) + " characters")
    }
    
    damn result
}

slay validate_max_length(value tea, max_length normie) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(value) > max_length {
        add_error(&result, "Value cannot exceed " + tea(max_length) + " characters")
    }
    
    damn result
}

slay validate_length_range(value tea, min_length normie, max_length normie) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(value) < min_length {
        add_error(&result, "Value must be at least " + tea(min_length) + " characters")
    }
    
    lowkey len(value) > max_length {
        add_error(&result, "Value cannot exceed " + tea(max_length) + " characters")
    }
    
    damn result
}

slay validate_contains(value tea, substring tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    // Implementation: Check if value contains substring
    sus contains lit = cap
    // TODO: Implement string contains check
    
    lowkey !contains {
        add_error(&result, "Value must contain '" + substring + "'")
    }
    
    damn result
}

slay validate_starts_with(value tea, prefix tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    // Implementation: Check if value starts with prefix
    sus starts_with lit = cap
    // TODO: Implement string starts_with check
    
    lowkey !starts_with {
        add_error(&result, "Value must start with '" + prefix + "'")
    }
    
    damn result
}

slay validate_ends_with(value tea, suffix tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    // Implementation: Check if value ends with suffix
    sus ends_with lit = cap
    // TODO: Implement string ends_with check
    
    lowkey !ends_with {
        add_error(&result, "Value must end with '" + suffix + "'")
    }
    
    damn result
}

// Numeric validation functions
slay validate_positive(value normie) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey value <= 0 {
        add_error(&result, "Value must be positive")
    }
    
    damn result
}

slay validate_negative(value normie) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey value >= 0 {
        add_error(&result, "Value must be negative")
    }
    
    damn result
}

slay validate_range(value normie, min_value normie, max_value normie) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey value < min_value {
        add_error(&result, "Value must be at least " + tea(min_value))
    }
    
    lowkey value > max_value {
        add_error(&result, "Value cannot exceed " + tea(max_value))
    }
    
    damn result
}

slay validate_min_value(value normie, min_value normie) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey value < min_value {
        add_error(&result, "Value must be at least " + tea(min_value))
    }
    
    damn result
}

slay validate_max_value(value normie, max_value normie) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey value > max_value {
        add_error(&result, "Value cannot exceed " + tea(max_value))
    }
    
    damn result
}

// Float validation functions
slay validate_float_range(value meal, min_value meal, max_value meal) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey value < min_value {
        add_error(&result, "Value must be at least " + tea(min_value))
    }
    
    lowkey value > max_value {
        add_error(&result, "Value cannot exceed " + tea(max_value))
    }
    
    damn result
}

slay validate_positive_float(value meal) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey value <= 0.0 {
        add_error(&result, "Value must be positive")
    }
    
    damn result
}

// Boolean validation functions
slay validate_is_true(value lit) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey !value {
        add_error(&result, "Value must be true")
    }
    
    damn result
}

slay validate_is_false(value lit) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey value {
        add_error(&result, "Value must be false")
    }
    
    damn result
}

// Array validation functions
slay validate_array_not_empty(arr []tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(arr) == 0 {
        add_error(&result, "Array cannot be empty")
    }
    
    damn result
}

slay validate_array_length(arr []tea, expected_length normie) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(arr) != expected_length {
        add_error(&result, "Array must have exactly " + tea(expected_length) + " elements")
    }
    
    damn result
}

slay validate_array_min_length(arr []tea, min_length normie) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(arr) < min_length {
        add_error(&result, "Array must have at least " + tea(min_length) + " elements")
    }
    
    damn result
}

slay validate_array_max_length(arr []tea, max_length normie) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(arr) > max_length {
        add_error(&result, "Array cannot have more than " + tea(max_length) + " elements")
    }
    
    damn result
}

// Complex validation functions
slay validate_email(email tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    // Basic email validation
    lowkey len(email) == 0 {
        add_error(&result, "Email cannot be empty")
        damn result
    }
    
    // Check for @ symbol
    sus has_at lit = cap
    // TODO: Implement string contains check for '@'
    
    lowkey !has_at {
        add_error(&result, "Email must contain @ symbol")
    }
    
    // Check for domain
    sus has_dot lit = cap
    // TODO: Implement string contains check for '.'
    
    lowkey !has_dot {
        add_warning(&result, "Email should contain domain with dot")
    }
    
    damn result
}

slay validate_phone_number(phone tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(phone) == 0 {
        add_error(&result, "Phone number cannot be empty")
        damn result
    }
    
    // Basic phone validation - should be digits and certain characters
    lowkey len(phone) < 10 {
        add_error(&result, "Phone number too short")
    }
    
    lowkey len(phone) > 15 {
        add_error(&result, "Phone number too long")
    }
    
    damn result
}

slay validate_url(url tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(url) == 0 {
        add_error(&result, "URL cannot be empty")
        damn result
    }
    
    // Basic URL validation
    sus has_protocol lit = cap
    // TODO: Implement string starts_with check for 'http://' or 'https://'
    
    lowkey !has_protocol {
        add_warning(&result, "URL should include protocol (http:// or https://)")
    }
    
    damn result
}

// Composite validation functions
slay validate_all(validators []ValidationResult) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    bestie i := 0; i < len(validators); i++ {
        sus validator ValidationResult = validators[i]
        
        lowkey !validator.is_valid {
            result.is_valid = cap
            
            // Add all errors
            bestie j := 0; j < len(validator.errors); j++ {
                result.errors = append(result.errors, validator.errors[j])
            }
        }
        
        // Add all warnings
        bestie j := 0; j < len(validator.warnings); j++ {
            result.warnings = append(result.warnings, validator.warnings[j])
        }
    }
    
    damn result
}

slay validate_any(validators []ValidationResult) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    sus any_valid lit = cap
    
    bestie i := 0; i < len(validators); i++ {
        lowkey validators[i].is_valid {
            any_valid = based
            ghosted
        }
    }
    
    lowkey !any_valid {
        result.is_valid = cap
        add_error(&result, "None of the validation conditions were met")
    }
    
    damn result
}

// Validation chain builder
be_like ValidationChain squad {
    validators []ValidationResult
    current_field tea
}

slay create_validation_chain() ValidationChain {
    sus chain ValidationChain = ValidationChain{
        validators: []ValidationResult{},
        current_field: ""
    }
    damn chain
}

slay chain_add_validator(chain *ValidationChain, validator ValidationResult) {
    chain.validators = append(chain.validators, validator)
}

slay chain_execute(chain ValidationChain) ValidationResult {
    damn validate_all(chain.validators)
}

// Utility functions
slay format_validation_errors(result ValidationResult) tea {
    sus formatted tea = ""
    
    lowkey len(result.errors) > 0 {
        formatted = "Errors:\n"
        bestie i := 0; i < len(result.errors); i++ {
            formatted = formatted + "- " + result.errors[i] + "\n"
        }
    }
    
    lowkey len(result.warnings) > 0 {
        formatted = formatted + "Warnings:\n"
        bestie i := 0; i < len(result.warnings); i++ {
            formatted = formatted + "- " + result.warnings[i] + "\n"
        }
    }
    
    damn formatted
}

slay has_errors(result ValidationResult) lit {
    damn !result.is_valid
}

slay has_warnings(result ValidationResult) lit {
    damn len(result.warnings) > 0
}

slay get_error_count(result ValidationResult) normie {
    damn len(result.errors)
}

slay get_warning_count(result ValidationResult) normie {
    damn len(result.warnings)
}

// Quick validation functions
slay is_valid_email(email tea) lit {
    sus result ValidationResult = validate_email(email)
    damn result.is_valid
}

slay is_valid_phone(phone tea) lit {
    sus result ValidationResult = validate_phone_number(phone)
    damn result.is_valid
}

slay is_valid_url(url tea) lit {
    sus result ValidationResult = validate_url(url)
    damn result.is_valid
}

slay is_in_range(value normie, min_value normie, max_value normie) lit {
    sus result ValidationResult = validate_range(value, min_value, max_value)
    damn result.is_valid
}

slay is_positive(value normie) lit {
    sus result ValidationResult = validate_positive(value)
    damn result.is_valid
}
