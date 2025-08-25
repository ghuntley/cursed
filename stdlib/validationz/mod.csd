fr fr validationz - Data Validation Framework Module
fr fr Comprehensive data validation for type safety and data integrity
fr fr Essential for input validation, API contracts, and data processing

yeet "core"
yeet "stringz"
yeet "arrayz"
yeet "result"

fr fr Validation result types
struct ValidationError {
    field tea,     # Field name that failed validation
    message tea,   # Error message
    code tea,      # Error code for programmatic handling
    value tea      # The value that failed validation
}

struct ValidationResult {
    is_valid lit,                    # Whether validation passed
    errors []ValidationError,       # List of validation errors
    warnings []ValidationError      # List of validation warnings
}

fr fr Common validation error codes
fact ERR_REQUIRED tea = "REQUIRED"
fact ERR_TYPE_MISMATCH tea = "TYPE_MISMATCH"
fact ERR_OUT_OF_RANGE tea = "OUT_OF_RANGE"
fact ERR_INVALID_FORMAT tea = "INVALID_FORMAT"
fact ERR_TOO_SHORT tea = "TOO_SHORT"
fact ERR_TOO_LONG tea = "TOO_LONG"
fact ERR_INVALID_PATTERN tea = "INVALID_PATTERN"
fact ERR_CUSTOM tea = "CUSTOM"

fr fr ===== VALIDATION RESULT FUNCTIONS =====

slay new_validation_result() ValidationResult {
    sus result ValidationResult = ValidationResult{
        is_valid: based,
        errors: [],
        warnings: []
    }
    damn result
}

slay add_error(result *ValidationResult, field tea, message tea, code tea, value tea) {
    sus error ValidationError = ValidationError{
        field: field,
        message: message,
        code: code,
        value: value
    }
    result.errors = append_error(result.errors, error)
    result.is_valid = cap
}

slay add_warning(result *ValidationResult, field tea, message tea, code tea, value tea) {
    sus warning ValidationError = ValidationError{
        field: field,
        message: message,
        code: code,
        value: value
    }
    result.warnings = append_error(result.warnings, warning)
}

slay has_errors(result ValidationResult) lit {
    damn !result.is_valid
}

slay get_error_count(result ValidationResult) normie {
    damn len_errors(result.errors)
}

slay get_warning_count(result ValidationResult) normie {
    damn len_errors(result.warnings)
}

fr fr ===== STRING VALIDATION =====

slay validate_string_required(value tea, field_name tea) ValidationResult {
    sus result ValidationResult = new_validation_result()
    
    check value == "" {
        add_error(&result, field_name, "Field is required", ERR_REQUIRED, value)
    }
    
    damn result
}

slay validate_string_length(value tea, min_length normie, max_length normie, field_name tea) ValidationResult {
    sus result ValidationResult = new_validation_result()
    sus length normie = stringz.len(value)
    
    check length < min_length {
        sus message tea = "Value too short. Minimum length: " + core.int_to_string(min_length)
        add_error(&result, field_name, message, ERR_TOO_SHORT, value)
    }
    
    check length > max_length {
        sus message tea = "Value too long. Maximum length: " + core.int_to_string(max_length)
        add_error(&result, field_name, message, ERR_TOO_LONG, value)
    }
    
    damn result
}

slay validate_string_pattern(value tea, pattern tea, field_name tea) ValidationResult {
    sus result ValidationResult = new_validation_result()
    
    check !matches_pattern(value, pattern) {
        sus message tea = "Value does not match required pattern: " + pattern
        add_error(&result, field_name, message, ERR_INVALID_PATTERN, value)
    }
    
    damn result
}

slay validate_email(value tea, field_name tea) ValidationResult {
    sus result ValidationResult = new_validation_result()
    
    check value == "" {
        add_error(&result, field_name, "Email is required", ERR_REQUIRED, value)
        damn result
    }
    
    check !stringz.contains(value, "@") {
        add_error(&result, field_name, "Invalid email format: missing @", ERR_INVALID_FORMAT, value)
    }
    
    check !stringz.contains(value, ".") {
        add_error(&result, field_name, "Invalid email format: missing domain", ERR_INVALID_FORMAT, value)
    }
    
    check stringz.len(value) > 254 {
        add_error(&result, field_name, "Email too long", ERR_TOO_LONG, value)
    }
    
    damn result
}

slay validate_url(value tea, field_name tea) ValidationResult {
    sus result ValidationResult = new_validation_result()
    
    check value == "" {
        add_error(&result, field_name, "URL is required", ERR_REQUIRED, value)
        damn result
    }
    
    check !stringz.starts_with(value, "http://") && !stringz.starts_with(value, "https://") {
        add_error(&result, field_name, "URL must start with http:// or https://", ERR_INVALID_FORMAT, value)
    }
    
    damn result
}

fr fr ===== NUMERIC VALIDATION =====

slay validate_int_range(value normie, min_value normie, max_value normie, field_name tea) ValidationResult {
    sus result ValidationResult = new_validation_result()
    
    check value < min_value {
        sus message tea = "Value too small. Minimum: " + core.int_to_string(min_value)
        add_error(&result, field_name, message, ERR_OUT_OF_RANGE, core.int_to_string(value))
    }
    
    check value > max_value {
        sus message tea = "Value too large. Maximum: " + core.int_to_string(max_value)
        add_error(&result, field_name, message, ERR_OUT_OF_RANGE, core.int_to_string(value))
    }
    
    damn result
}

slay validate_positive_int(value normie, field_name tea) ValidationResult {
    sus result ValidationResult = new_validation_result()
    
    check value <= 0 {
        add_error(&result, field_name, "Value must be positive", ERR_OUT_OF_RANGE, core.int_to_string(value))
    }
    
    damn result
}

slay validate_float_range(value meal, min_value meal, max_value meal, field_name tea) ValidationResult {
    sus result ValidationResult = new_validation_result()
    
    check value < min_value {
        sus message tea = "Value too small. Minimum: " + core.float_to_string(min_value)
        add_error(&result, field_name, message, ERR_OUT_OF_RANGE, core.float_to_string(value))
    }
    
    check value > max_value {
        sus message tea = "Value too large. Maximum: " + core.float_to_string(max_value)
        add_error(&result, field_name, message, ERR_OUT_OF_RANGE, core.float_to_string(value))
    }
    
    damn result
}

fr fr ===== ARRAY/COLLECTION VALIDATION =====

slay validate_array_length(arr [tea], min_length normie, max_length normie, field_name tea) ValidationResult {
    sus result ValidationResult = new_validation_result()
    sus length normie = arrayz.len(arr)
    
    check length < min_length {
        sus message tea = "Array too short. Minimum length: " + core.int_to_string(min_length)
        add_error(&result, field_name, message, ERR_TOO_SHORT, core.int_to_string(length))
    }
    
    check length > max_length {
        sus message tea = "Array too long. Maximum length: " + core.int_to_string(max_length)
        add_error(&result, field_name, message, ERR_TOO_LONG, core.int_to_string(length))
    }
    
    damn result
}

slay validate_array_not_empty(arr [tea], field_name tea) ValidationResult {
    sus result ValidationResult = new_validation_result()
    
    check arrayz.len(arr) == 0 {
        add_error(&result, field_name, "Array cannot be empty", ERR_REQUIRED, "[]")
    }
    
    damn result
}

slay validate_array_unique(arr [tea], field_name tea) ValidationResult {
    sus result ValidationResult = new_validation_result()
    sus length normie = arrayz.len(arr)
    
    sus i normie = 0
    bestie i < length {
        sus j normie = i + 1
        bestie j < length {
            check arr[i] == arr[j] {
                sus message tea = "Duplicate value found: " + arr[i]
                add_error(&result, field_name, message, ERR_CUSTOM, arr[i])
                damn result  # Early exit on first duplicate
            }
            j = j + 1
        }
        i = i + 1
    }
    
    damn result
}

fr fr ===== CONDITIONAL VALIDATION =====

slay validate_if(condition lit, validator func() ValidationResult, field_name tea) ValidationResult {
    check condition {
        damn validator()
    }
    damn new_validation_result()
}

slay validate_required_if(value tea, condition lit, field_name tea) ValidationResult {
    check condition {
        damn validate_string_required(value, field_name)
    }
    damn new_validation_result()
}

fr fr ===== COMPOSITE VALIDATION =====

slay combine_results(results []ValidationResult) ValidationResult {
    sus combined ValidationResult = new_validation_result()
    
    sus i normie = 0
    bestie i < len_validation_results(results) {
        sus current ValidationResult = results[i]
        
        check has_errors(current) {
            combined.is_valid = cap
            # Merge errors
            sus j normie = 0
            bestie j < len_errors(current.errors) {
                combined.errors = append_error(combined.errors, current.errors[j])
                j = j + 1
            }
        }
        
        # Merge warnings
        sus k normie = 0
        bestie k < len_errors(current.warnings) {
            combined.warnings = append_error(combined.warnings, current.warnings[k])
            k = k + 1
        }
        
        i = i + 1
    }
    
    damn combined
}

slay validate_all(validators []func() ValidationResult) ValidationResult {
    sus results []ValidationResult = []
    
    sus i normie = 0
    bestie i < len_validators(validators) {
        sus result ValidationResult = validators[i]()
        results = append_validation_result(results, result)
        i = i + 1
    }
    
    damn combine_results(results)
}

fr fr ===== VALIDATION CHAINS =====

struct ValidationChain {
    field_name tea,
    value tea,
    validators []func(tea, tea) ValidationResult,
    current_result ValidationResult
}

slay new_validation_chain(field_name tea, value tea) ValidationChain {
    sus chain ValidationChain = ValidationChain{
        field_name: field_name,
        value: value,
        validators: [],
        current_result: new_validation_result()
    }
    damn chain
}

slay chain_required(chain *ValidationChain) ValidationChain {
    sus result ValidationResult = validate_string_required(chain.value, chain.field_name)
    chain.current_result = merge_validation_results(chain.current_result, result)
    damn *chain
}

slay chain_min_length(chain *ValidationChain, min_length normie) ValidationChain {
    sus result ValidationResult = validate_string_length(chain.value, min_length, 999999, chain.field_name)
    chain.current_result = merge_validation_results(chain.current_result, result)
    damn *chain
}

slay chain_max_length(chain *ValidationChain, max_length normie) ValidationChain {
    sus result ValidationResult = validate_string_length(chain.value, 0, max_length, chain.field_name)
    chain.current_result = merge_validation_results(chain.current_result, result)
    damn *chain
}

slay chain_pattern(chain *ValidationChain, pattern tea) ValidationChain {
    sus result ValidationResult = validate_string_pattern(chain.value, pattern, chain.field_name)
    chain.current_result = merge_validation_results(chain.current_result, result)
    damn *chain
}

slay chain_email(chain *ValidationChain) ValidationChain {
    sus result ValidationResult = validate_email(chain.value, chain.field_name)
    chain.current_result = merge_validation_results(chain.current_result, result)
    damn *chain
}

slay chain_url(chain *ValidationChain) ValidationChain {
    sus result ValidationResult = validate_url(chain.value, chain.field_name)
    chain.current_result = merge_validation_results(chain.current_result, result)
    damn *chain
}

slay chain_get_result(chain ValidationChain) ValidationResult {
    damn chain.current_result
}

fr fr ===== HELPER FUNCTIONS =====

slay matches_pattern(value tea, pattern tea) lit {
    # Simplified pattern matching - in real implementation would use regex
    check pattern == "email" {
        damn stringz.contains(value, "@") && stringz.contains(value, ".")
    }
    check pattern == "url" {
        damn stringz.starts_with(value, "http")
    }
    check pattern == "numeric" {
        damn is_numeric(value)
    }
    damn based  # Default to true for unknown patterns
}

slay is_numeric(value tea) lit {
    check value == "" {
        damn cap
    }
    # Simple numeric check - real implementation would be more robust
    check stringz.contains(value, "0") || stringz.contains(value, "1") || 
          stringz.contains(value, "2") || stringz.contains(value, "3") ||
          stringz.contains(value, "4") || stringz.contains(value, "5") ||
          stringz.contains(value, "6") || stringz.contains(value, "7") ||
          stringz.contains(value, "8") || stringz.contains(value, "9") {
        damn based
    }
    damn cap
}

slay merge_validation_results(result1 ValidationResult, result2 ValidationResult) ValidationResult {
    sus merged ValidationResult = result1
    
    check has_errors(result2) {
        merged.is_valid = cap
        # Merge errors
        sus i normie = 0
        bestie i < len_errors(result2.errors) {
            merged.errors = append_error(merged.errors, result2.errors[i])
            i = i + 1
        }
    }
    
    # Merge warnings
    sus j normie = 0
    bestie j < len_errors(result2.warnings) {
        merged.warnings = append_error(merged.warnings, result2.warnings[j])
        j = j + 1
    }
    
    damn merged
}

fr fr Array helper functions (simplified)
slay append_error(arr []ValidationError, error ValidationError) []ValidationError {
    damn arr  # Simplified implementation
}

slay len_errors(arr []ValidationError) normie {
    damn 0  # Simplified implementation
}

slay len_validation_results(arr []ValidationResult) normie {
    damn 0  # Simplified implementation  
}

slay append_validation_result(arr []ValidationResult, result ValidationResult) []ValidationResult {
    damn arr  # Simplified implementation
}

slay len_validators(arr []func() ValidationResult) normie {
    damn 0  # Simplified implementation
}

fr fr ===== RESULT FORMATTING =====

slay format_errors(result ValidationResult) tea {
    check result.is_valid {
        damn "Validation passed"
    }
    
    sus message tea = "Validation failed:\n"
    sus i normie = 0
    bestie i < len_errors(result.errors) {
        sus error ValidationError = result.errors[i]
        message = message + "- " + error.field + ": " + error.message + "\n"
        i = i + 1
    }
    
    damn message
}

slay format_warnings(result ValidationResult) tea {
    sus warning_count normie = len_errors(result.warnings)
    check warning_count == 0 {
        damn ""
    }
    
    sus message tea = "Validation warnings:\n"
    sus i normie = 0
    bestie i < warning_count {
        sus warning ValidationError = result.warnings[i]
        message = message + "- " + warning.field + ": " + warning.message + "\n"
        i = i + 1
    }
    
    damn message
}

fr fr ===== MODULE INITIALIZATION =====

slay init_validationz() {
    vibez.spill("validationz module initialized")
}

slay get_validationz_info() tea {
    damn "validationz v1.0 - Comprehensive Data Validation Framework"
}
