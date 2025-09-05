# Validation Module

Comprehensive data validation and verification system for CURSED.

## Overview

The `validation` module provides a complete validation framework for data verification, including string validation, numeric validation, array validation, and complex validation scenarios. It supports validation chaining and detailed error reporting.

## Features

### Core Validation System
- **Validation Results**: `ValidationResult` structure with success/failure tracking
- **Error Management**: Detailed error and warning collection
- **Validation Chaining**: Chain multiple validations together
- **Composite Validation**: Validate multiple conditions with AND/OR logic

### String Validation
- **Emptiness**: `validate_not_empty()` - Ensure string is not empty
- **Length**: `validate_min_length()`, `validate_max_length()`, `validate_length_range()`
- **Content**: `validate_contains()`, `validate_starts_with()`, `validate_ends_with()`

### Numeric Validation
- **Sign**: `validate_positive()`, `validate_negative()`
- **Range**: `validate_range()`, `validate_min_value()`, `validate_max_value()`
- **Float Range**: `validate_float_range()`, `validate_positive_float()`

### Boolean Validation
- **Value**: `validate_is_true()`, `validate_is_false()`

### Array Validation
- **Emptiness**: `validate_array_not_empty()`
- **Length**: `validate_array_length()`, `validate_array_min_length()`, `validate_array_max_length()`

### Complex Validation
- **Email**: `validate_email()` - RFC 5322 compliant email validation
- **Phone**: `validate_phone_number()` - International phone number validation (E.164)
- **URL**: `validate_url()` - Comprehensive URL validation with scheme checking
- **IP Address**: `validate_ip_address()` - IPv4 and IPv6 address validation
- **Credit Card**: `validate_credit_card()` - Credit card validation with Luhn algorithm
- **Date Format**: `validate_date_format()` - Multiple date format validation

### Composite Validation
- **All Valid**: `validate_all()` - All validations must pass
- **Any Valid**: `validate_any()` - At least one validation must pass
- **Validation Chain**: `ValidationChain` for sequential validation

## Data Structures

### ValidationResult
```cursed
be_like ValidationResult squad {
    is_valid lit        // Whether validation passed
    errors []tea        // Error messages
    warnings []tea      // Warning messages
}
```

### ValidationChain
```cursed
be_like ValidationChain squad {
    validators []ValidationResult    // List of validations
    current_field tea               // Current field being validated
}
```

## Usage Examples

```cursed
yeet "validation"

// Basic string validation
sus result ValidationResult = validate_not_empty("hello")
sus is_valid lit = result.is_valid

// Numeric validation
sus positive_result ValidationResult = validate_positive(5)
sus range_result ValidationResult = validate_range(7, 1, 10)

// Array validation
sus array []tea = []tea{"item1", "item2", "item3"}
sus array_result ValidationResult = validate_array_min_length(array, 2)

// Complex validation
sus email_result ValidationResult = validate_email("user@example.com")
sus phone_result ValidationResult = validate_phone_number("123-456-7890")
sus url_result ValidationResult = validate_url("https://example.com")
sus ip_result ValidationResult = validate_ip_address("192.168.1.1")
sus card_result ValidationResult = validate_credit_card("4111111111111111")
sus date_result ValidationResult = validate_date_format("2023-12-31", "YYYY-MM-DD")

// Composite validation - all must pass
sus all_validators []ValidationResult = []ValidationResult{
    validate_not_empty("test"),
    validate_positive(5),
    validate_range(7, 1, 10)
}
sus all_result ValidationResult = validate_all(all_validators)

// Composite validation - any can pass
sus any_result ValidationResult = validate_any(all_validators)

// Validation chain
sus chain ValidationChain = create_validation_chain()
chain_add_validator(&chain, validate_not_empty("test"))
chain_add_validator(&chain, validate_positive(5))
sus chain_result ValidationResult = chain_execute(chain)

// Error handling
sus error_result ValidationResult = validate_positive(-5)
sus has_errors lit = has_errors(error_result)
sus error_count normie = get_error_count(error_result)
sus formatted_errors tea = format_validation_errors(error_result)
```

## Quick Validation Functions

For simple validation scenarios, use quick validation functions:

```cursed
// Quick boolean checks
sus is_positive lit = is_positive(5)
sus is_in_range lit = is_in_range(7, 1, 10)
sus is_email_valid lit = is_valid_email("user@example.com")
sus is_phone_valid lit = is_valid_phone("123-456-7890")
sus is_url_valid lit = is_valid_url("https://example.com")
sus is_ip_valid lit = is_valid_ip("192.168.1.1")
```

## Error Management

The validation system provides comprehensive error management:

```cursed
// Create validation result
sus result ValidationResult = create_validation_result()

// Add errors and warnings
add_error(&result, "Custom error message")
add_warning(&result, "Custom warning message")

// Check for errors and warnings
sus has_errors lit = has_errors(result)
sus has_warnings lit = has_warnings(result)
sus error_count normie = get_error_count(result)
sus warning_count normie = get_warning_count(result)

// Format errors for display
sus formatted tea = format_validation_errors(result)
```

## Validation Patterns

### Form Validation
```cursed
slay validate_user_form(name tea, email tea, age normie) ValidationResult {
    sus validators []ValidationResult = []ValidationResult{
        validate_not_empty(name),
        validate_length_range(name, 2, 50),
        validate_email(email),
        validate_range(age, 18, 120)
    }
    
    damn validate_all(validators)
}
```

### API Input Validation
```cursed
slay validate_api_request(data map[tea]tea) ValidationResult {
    sus chain ValidationChain = create_validation_chain()
    
    // Validate required fields
    chain_add_validator(&chain, validate_not_empty(data["username"]))
    chain_add_validator(&chain, validate_email(data["email"]))
    chain_add_validator(&chain, validate_positive(string_to_int(data["age"])))
    
    damn chain_execute(chain)
}
```

## Performance

The validation module is optimized for performance:
- Lazy evaluation of validation chains
- Efficient error collection and formatting
- Minimal memory overhead for validation results
- Fast validation for common patterns

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/validation/test_validation.💀
```

## Status

✅ **Production Ready**: All validation functions implemented and tested
✅ **Pure CURSED**: No external validation library dependencies
✅ **Cross-Platform**: Consistent validation behavior across platforms
✅ **Extensible**: Easy to add new validation rules and patterns
✅ **Fully Tested**: Comprehensive test coverage for all validation scenarios
