# CURSED Validation Module

Production-ready data validation and verification system for CURSED programs with comprehensive validation rules, error handling, and validation chains.

## Features

- **String Validation**: Length checks, content validation, format verification
- **Numeric Validation**: Range checks, positive/negative validation, boundary testing
- **Float Validation**: Decimal number validation with precision controls
- **Boolean Validation**: True/false state verification
- **Array Validation**: Length checks, content validation, structure verification
- **Complex Validation**: Email, phone, URL validation with pattern matching
- **Composite Validation**: Combine multiple validators with AND/OR logic
- **Validation Chains**: Build complex validation pipelines
- **Error Management**: Detailed error and warning collection
- **Quick Validation**: Simple boolean validation functions

## Usage

### Import the Module

```cursed
yeet "validation"
```

### Basic String Validation

```cursed
// Check if string is not empty
sus result validation.ValidationResult = validation.validate_not_empty("test")
lowkey result.is_valid {
    vibez.spill("String is valid")
} highkey {
    vibez.spill("Validation failed")
}

// Check string length
sus min_length_result validation.ValidationResult = validation.validate_min_length("hello", 3)
sus max_length_result validation.ValidationResult = validation.validate_max_length("hello", 10)
sus range_result validation.ValidationResult = validation.validate_length_range("hello", 3, 10)
```

### Numeric Validation

```cursed
// Validate positive numbers
sus positive_result validation.ValidationResult = validation.validate_positive(42)
assert_true(positive_result.is_valid)

// Validate negative numbers
sus negative_result validation.ValidationResult = validation.validate_negative(-10)
assert_true(negative_result.is_valid)

// Validate ranges
sus range_result validation.ValidationResult = validation.validate_range(5, 1, 10)
assert_true(range_result.is_valid)

// Validate minimum and maximum values
sus min_result validation.ValidationResult = validation.validate_min_value(5, 3)
sus max_result validation.ValidationResult = validation.validate_max_value(5, 10)
```

### Float Validation

```cursed
// Validate positive floats
sus float_result validation.ValidationResult = validation.validate_positive_float(3.14)
assert_true(float_result.is_valid)

// Validate float ranges
sus float_range_result validation.ValidationResult = validation.validate_float_range(5.5, 1.0, 10.0)
assert_true(float_range_result.is_valid)
```

### Boolean Validation

```cursed
// Validate true/false states
sus true_result validation.ValidationResult = validation.validate_is_true(based)
sus false_result validation.ValidationResult = validation.validate_is_false(cap)
```

### Array Validation

```cursed
// Validate array length
sus array []tea = []tea{"item1", "item2", "item3"}
sus not_empty_result validation.ValidationResult = validation.validate_array_not_empty(array)
sus length_result validation.ValidationResult = validation.validate_array_length(array, 3)
sus min_length_result validation.ValidationResult = validation.validate_array_min_length(array, 2)
sus max_length_result validation.ValidationResult = validation.validate_array_max_length(array, 5)
```

### Complex Validation

```cursed
// Email validation
sus email_result validation.ValidationResult = validation.validate_email("user@example.com")
lowkey email_result.is_valid {
    vibez.spill("Valid email")
} highkey {
    vibez.spill("Invalid email: " + validation.format_validation_errors(email_result))
}

// Phone number validation
sus phone_result validation.ValidationResult = validation.validate_phone_number("1234567890")

// URL validation
sus url_result validation.ValidationResult = validation.validate_url("https://example.com")
```

### Composite Validation

```cursed
// Validate all conditions must pass
sus validators []validation.ValidationResult = []validation.ValidationResult{
    validation.validate_not_empty("test"),
    validation.validate_positive(5),
    validation.validate_is_true(based)
}

sus all_result validation.ValidationResult = validation.validate_all(validators)
assert_true(all_result.is_valid)

// Validate any condition must pass
sus any_result validation.ValidationResult = validation.validate_any(validators)
assert_true(any_result.is_valid)
```

### Validation Chains

```cursed
// Build validation chain
sus chain validation.ValidationChain = validation.create_validation_chain()

// Add validators to chain
validation.chain_add_validator(&chain, validation.validate_not_empty("test"))
validation.chain_add_validator(&chain, validation.validate_min_length("test", 3))
validation.chain_add_validator(&chain, validation.validate_max_length("test", 10))

// Execute chain
sus chain_result validation.ValidationResult = validation.chain_execute(chain)
lowkey chain_result.is_valid {
    vibez.spill("All validations passed")
} highkey {
    vibez.spill("Validation chain failed: " + validation.format_validation_errors(chain_result))
}
```

### Error and Warning Management

```cursed
// Create validation result
sus result validation.ValidationResult = validation.create_validation_result()

// Add errors and warnings
validation.add_error(&result, "This is an error")
validation.add_warning(&result, "This is a warning")

// Check for errors and warnings
lowkey validation.has_errors(result) {
    vibez.spill("Errors found: " + tea(validation.get_error_count(result)))
}

lowkey validation.has_warnings(result) {
    vibez.spill("Warnings found: " + tea(validation.get_warning_count(result)))
}

// Format all errors and warnings
vibez.spill(validation.format_validation_errors(result))
```

### Quick Validation Functions

```cursed
// Quick boolean validation functions
lowkey validation.is_positive(42) {
    vibez.spill("Number is positive")
}

lowkey validation.is_in_range(5, 1, 10) {
    vibez.spill("Number is in range")
}

lowkey validation.is_valid_email("user@example.com") {
    vibez.spill("Email is valid")
}

lowkey validation.is_valid_phone("1234567890") {
    vibez.spill("Phone is valid")
}

lowkey validation.is_valid_url("https://example.com") {
    vibez.spill("URL is valid")
}
```

## API Reference

### Core Types

```cursed
be_like ValidationResult squad {
    is_valid lit        // Whether validation passed
    errors []tea        // List of error messages
    warnings []tea      // List of warning messages
}

be_like ValidationChain squad {
    validators []ValidationResult  // List of validators
    current_field tea             // Current field being validated
}
```

### Validation Result Management
- `create_validation_result() -> ValidationResult` - Create new validation result
- `add_error(result: *ValidationResult, error: tea)` - Add error to result
- `add_warning(result: *ValidationResult, warning: tea)` - Add warning to result

### String Validation
- `validate_not_empty(value: tea) -> ValidationResult` - Check if string is not empty
- `validate_min_length(value: tea, min_length: normie) -> ValidationResult` - Check minimum length
- `validate_max_length(value: tea, max_length: normie) -> ValidationResult` - Check maximum length
- `validate_length_range(value: tea, min_length: normie, max_length: normie) -> ValidationResult` - Check length range
- `validate_contains(value: tea, substring: tea) -> ValidationResult` - Check if contains substring
- `validate_starts_with(value: tea, prefix: tea) -> ValidationResult` - Check if starts with prefix
- `validate_ends_with(value: tea, suffix: tea) -> ValidationResult` - Check if ends with suffix

### Numeric Validation
- `validate_positive(value: normie) -> ValidationResult` - Check if positive
- `validate_negative(value: normie) -> ValidationResult` - Check if negative
- `validate_range(value: normie, min_value: normie, max_value: normie) -> ValidationResult` - Check range
- `validate_min_value(value: normie, min_value: normie) -> ValidationResult` - Check minimum value
- `validate_max_value(value: normie, max_value: normie) -> ValidationResult` - Check maximum value

### Float Validation
- `validate_float_range(value: meal, min_value: meal, max_value: meal) -> ValidationResult` - Check float range
- `validate_positive_float(value: meal) -> ValidationResult` - Check if positive float

### Boolean Validation
- `validate_is_true(value: lit) -> ValidationResult` - Check if true
- `validate_is_false(value: lit) -> ValidationResult` - Check if false

### Array Validation
- `validate_array_not_empty(arr: []tea) -> ValidationResult` - Check if array not empty
- `validate_array_length(arr: []tea, expected_length: normie) -> ValidationResult` - Check exact length
- `validate_array_min_length(arr: []tea, min_length: normie) -> ValidationResult` - Check minimum length
- `validate_array_max_length(arr: []tea, max_length: normie) -> ValidationResult` - Check maximum length

### Complex Validation
- `validate_email(email: tea) -> ValidationResult` - Validate email format
- `validate_phone_number(phone: tea) -> ValidationResult` - Validate phone number format
- `validate_url(url: tea) -> ValidationResult` - Validate URL format

### Composite Validation
- `validate_all(validators: []ValidationResult) -> ValidationResult` - All must pass
- `validate_any(validators: []ValidationResult) -> ValidationResult` - Any must pass

### Validation Chains
- `create_validation_chain() -> ValidationChain` - Create validation chain
- `chain_add_validator(chain: *ValidationChain, validator: ValidationResult)` - Add validator to chain
- `chain_execute(chain: ValidationChain) -> ValidationResult` - Execute validation chain

### Utility Functions
- `format_validation_errors(result: ValidationResult) -> tea` - Format errors and warnings
- `has_errors(result: ValidationResult) -> lit` - Check if has errors
- `has_warnings(result: ValidationResult) -> lit` - Check if has warnings
- `get_error_count(result: ValidationResult) -> normie` - Get error count
- `get_warning_count(result: ValidationResult) -> normie` - Get warning count

### Quick Validation Functions
- `is_valid_email(email: tea) -> lit` - Quick email validation
- `is_valid_phone(phone: tea) -> lit` - Quick phone validation
- `is_valid_url(url: tea) -> lit` - Quick URL validation
- `is_in_range(value: normie, min_value: normie, max_value: normie) -> lit` - Quick range check
- `is_positive(value: normie) -> lit` - Quick positive check

## Common Validation Patterns

### Form Validation
```cursed
// Validate user registration form
sus name_result validation.ValidationResult = validation.validate_not_empty(name)
sus email_result validation.ValidationResult = validation.validate_email(email)
sus phone_result validation.ValidationResult = validation.validate_phone_number(phone)
sus password_result validation.ValidationResult = validation.validate_min_length(password, 8)

sus form_validators []validation.ValidationResult = []validation.ValidationResult{
    name_result, email_result, phone_result, password_result
}

sus form_result validation.ValidationResult = validation.validate_all(form_validators)
```

### API Input Validation
```cursed
// Validate API request parameters
sus user_id_result validation.ValidationResult = validation.validate_positive(user_id)
sus limit_result validation.ValidationResult = validation.validate_range(limit, 1, 100)
sus offset_result validation.ValidationResult = validation.validate_min_value(offset, 0)

sus api_chain validation.ValidationChain = validation.create_validation_chain()
validation.chain_add_validator(&api_chain, user_id_result)
validation.chain_add_validator(&api_chain, limit_result)
validation.chain_add_validator(&api_chain, offset_result)

sus api_result validation.ValidationResult = validation.chain_execute(api_chain)
```

### Configuration Validation
```cursed
// Validate application configuration
sus port_result validation.ValidationResult = validation.validate_range(port, 1, 65535)
sus host_result validation.ValidationResult = validation.validate_not_empty(host)
sus timeout_result validation.ValidationResult = validation.validate_positive(timeout)

sus config_validators []validation.ValidationResult = []validation.ValidationResult{
    port_result, host_result, timeout_result
}

sus config_result validation.ValidationResult = validation.validate_all(config_validators)
```

## Error Handling

All validation functions return `ValidationResult` structures that contain:
- `is_valid`: Boolean indicating if validation passed
- `errors`: Array of error messages for validation failures
- `warnings`: Array of warning messages for potential issues

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/validation/test_validation.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/validation/test_validation.csd
./test_validation
```

## Implementation Notes

- Uses pure CURSED language features
- Minimal FFI dependencies
- Comprehensive error collection
- Extensible validation framework
- Performance-optimized validation chains
- Memory-safe string handling
- Production-ready error reporting

## Self-Hosting Ready

This module is essential for self-hosting and provides all validation capabilities needed for:

- Compiler input validation
- Configuration file validation
- Build parameter validation
- Runtime argument validation
- Data integrity checking
- API input validation
- Form validation
- File format validation
