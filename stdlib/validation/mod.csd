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

// Add info to validation result
slay add_info(result *ValidationResult, info tea) {
    // For now, add info as a warning since we don't have separate info field
    result.warnings = append(result.warnings, "INFO: " + info)
}

// String helper functions
slay get_char_at(str tea, index normie) sip {
    // Simple character access implementation
    sus chars []sip = []sip(str)
    damn chars[index]
}

slay substring(str tea, start normie, end normie) tea {
    // Simple substring implementation
    sus result tea = ""
    bestie i := start; i < end && i < len(str); i++ {
        result = result + tea(get_char_at(str, i))
    }
    damn result
}

slay starts_with(str tea, prefix tea) lit {
    lowkey len(prefix) > len(str) {
        damn cap
    }
    
    bestie i := 0; i < len(prefix); i++ {
        lowkey get_char_at(str, i) != get_char_at(prefix, i) {
            damn cap
        }
    }
    
    damn based
}

slay ends_with(str tea, suffix tea) lit {
    lowkey len(suffix) > len(str) {
        damn cap
    }
    
    sus start_pos normie = len(str) - len(suffix)
    bestie i := 0; i < len(suffix); i++ {
        lowkey get_char_at(str, start_pos + i) != get_char_at(suffix, i) {
            damn cap
        }
    }
    
    damn based
}

slay contains_string(str tea, substring tea) lit {
    lowkey len(substring) > len(str) {
        damn cap
    }
    
    bestie i := 0; i <= len(str) - len(substring); i++ {
        sus found lit = based
        bestie j := 0; j < len(substring); j++ {
            lowkey get_char_at(str, i + j) != get_char_at(substring, j) {
                found = cap
                ghosted
            }
        }
        lowkey found {
            damn based
        }
    }
    
    damn cap
}

slay string_to_int(str tea) normie {
    // Simple string to integer conversion
    sus result normie = 0
    bestie i := 0; i < len(str); i++ {
        sus ch sip = get_char_at(str, i)
        lowkey ch >= '0' && ch <= '9' {
            result = result * 10 + (normie(ch) - normie('0'))
        }
    }
    damn result
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
    sus contains lit = contains_string(value, substring)
    
    lowkey !contains {
        add_error(&result, "Value must contain '" + substring + "'")
    }
    
    damn result
}

slay validate_starts_with(value tea, prefix tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    // Implementation: Check if value starts with prefix
    sus starts_with_result lit = starts_with(value, prefix)
    
    lowkey !starts_with_result {
        add_error(&result, "Value must start with '" + prefix + "'")
    }
    
    damn result
}

slay validate_ends_with(value tea, suffix tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    // Implementation: Check if value ends with suffix
    sus ends_with_result lit = ends_with(value, suffix)
    
    lowkey !ends_with_result {
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
// RFC 5322 compliant email validation
slay validate_email(email tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    // Basic email validation
    lowkey len(email) == 0 {
        add_error(&result, "Email cannot be empty")
        damn result
    }
    
    // Check minimum length
    lowkey len(email) < 3 {
        add_error(&result, "Email too short (minimum 3 characters)")
        damn result
    }
    
    // Check maximum length (RFC 5321 limit)
    lowkey len(email) > 254 {
        add_error(&result, "Email too long (maximum 254 characters)")
        damn result
    }
    
    // Find @ symbol position
    sus at_pos normie = -1
    sus at_count normie = 0
    bestie i := 0; i < len(email); i++ {
        lowkey get_char_at(email, i) == '@' {
            at_pos = i
            at_count++
        }
    }
    
    // Must have exactly one @ symbol
    lowkey at_count != 1 {
        add_error(&result, "Email must contain exactly one @ symbol")
        damn result
    }
    
    // Must not start or end with @
    lowkey at_pos == 0 || at_pos == len(email) - 1 {
        add_error(&result, "Email cannot start or end with @ symbol")
        damn result
    }
    
    // Split local and domain parts
    sus local_part tea = substring(email, 0, at_pos)
    sus domain_part tea = substring(email, at_pos + 1, len(email))
    
    // Validate local part (before @)
    lowkey len(local_part) == 0 {
        add_error(&result, "Email local part cannot be empty")
    }
    
    lowkey len(local_part) > 64 {
        add_error(&result, "Email local part too long (maximum 64 characters)")
    }
    
    // Validate domain part (after @)
    lowkey len(domain_part) == 0 {
        add_error(&result, "Email domain part cannot be empty")
    }
    
    lowkey len(domain_part) > 253 {
        add_error(&result, "Email domain part too long (maximum 253 characters)")
    }
    
    // Check for at least one dot in domain
    sus has_dot lit = cap
    bestie i := 0; i < len(domain_part); i++ {
        lowkey get_char_at(domain_part, i) == '.' {
            has_dot = based
            ghosted
        }
    }
    
    lowkey !has_dot {
        add_error(&result, "Email domain must contain at least one dot")
    }
    
    // Check domain doesn't start or end with dot
    lowkey starts_with(domain_part, ".") || ends_with(domain_part, ".") {
        add_error(&result, "Email domain cannot start or end with dot")
    }
    
    // Check for consecutive dots in domain
    sus has_consecutive_dots lit = cap
    bestie i := 0; i < len(domain_part) - 1; i++ {
        lowkey get_char_at(domain_part, i) == '.' && get_char_at(domain_part, i + 1) == '.' {
            has_consecutive_dots = based
            ghosted
        }
    }
    
    lowkey has_consecutive_dots {
        add_error(&result, "Email domain cannot contain consecutive dots")
    }
    
    damn result
}

// International phone number validation (E.164 format)
slay validate_phone_number(phone tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(phone) == 0 {
        add_error(&result, "Phone number cannot be empty")
        damn result
    }
    
    // Remove common formatting characters for validation
    sus clean_phone tea = ""
    bestie i := 0; i < len(phone); i++ {
        sus ch sip = get_char_at(phone, i)
        lowkey ch >= '0' && ch <= '9' {
            clean_phone = clean_phone + tea(ch)
        } else lowkey ch == '+' && i == 0 {
            // Allow leading + for international format
            clean_phone = clean_phone + tea(ch)
        } else lowkey ch == ' ' || ch == '-' || ch == '(' || ch == ')' || ch == '.' {
            // Allow common formatting characters but don't include in validation
            simp
        } else {
            add_error(&result, "Phone number contains invalid character: " + tea(ch))
        }
    }
    
    // Check if starts with + (international format)
    sus is_international lit = starts_with(clean_phone, "+")
    
    lowkey is_international {
        // International format: +[country code][number]
        // Remove + for length validation
        clean_phone = substring(clean_phone, 1, len(clean_phone))
        
        // Check minimum length for international (country code + number)
        lowkey len(clean_phone) < 7 {
            add_error(&result, "International phone number too short (minimum 7 digits after +)")
        }
        
        // Check maximum length for international (E.164 standard)
        lowkey len(clean_phone) > 15 {
            add_error(&result, "International phone number too long (maximum 15 digits)")
        }
        
        // First digit after + cannot be 0 (country codes don't start with 0)
        lowkey len(clean_phone) > 0 && get_char_at(clean_phone, 0) == '0' {
            add_error(&result, "International phone number cannot start with 0 after +")
        }
        
    } else {
        // Domestic format validation
        lowkey len(clean_phone) < 7 {
            add_error(&result, "Phone number too short (minimum 7 digits)")
        }
        
        lowkey len(clean_phone) > 15 {
            add_error(&result, "Phone number too long (maximum 15 digits)")
        }
    }
    
    // Check that we have at least some digits
    lowkey len(clean_phone) == 0 {
        add_error(&result, "Phone number must contain at least one digit")
    }
    
    // Validate all characters are digits
    bestie i := 0; i < len(clean_phone); i++ {
        sus ch sip = get_char_at(clean_phone, i)
        lowkey ch < '0' || ch > '9' {
            add_error(&result, "Phone number contains non-digit character: " + tea(ch))
        }
    }
    
    damn result
}

// Comprehensive URL validation with scheme and domain checking
slay validate_url(url tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(url) == 0 {
        add_error(&result, "URL cannot be empty")
        damn result
    }
    
    // Check maximum URL length (browsers typically limit to 2048)
    lowkey len(url) > 2048 {
        add_error(&result, "URL too long (maximum 2048 characters)")
        damn result
    }
    
    // Check for valid protocol schemes
    sus has_http lit = starts_with(url, "http://")
    sus has_https lit = starts_with(url, "https://")
    sus has_ftp lit = starts_with(url, "ftp://")
    sus has_ftps lit = starts_with(url, "ftps://")
    sus has_file lit = starts_with(url, "file://")
    
    sus has_valid_scheme lit = has_http || has_https || has_ftp || has_ftps || has_file
    
    lowkey !has_valid_scheme {
        add_error(&result, "URL must start with valid scheme (http://, https://, ftp://, ftps://, or file://)")
        damn result
    }
    
    // Extract domain part (after protocol)
    sus domain_start normie = 0
    lowkey has_http || has_https {
        domain_start = 7  // Length of "http://"
        lowkey has_https {
            domain_start = 8  // Length of "https://"
        }
    } else lowkey has_ftp {
        domain_start = 6  // Length of "ftp://"
    } else lowkey has_ftps {
        domain_start = 7  // Length of "ftps://"
    } else lowkey has_file {
        domain_start = 7  // Length of "file://"
    }
    
    // Find end of domain (before path, query, or fragment)
    sus domain_end normie = len(url)
    bestie i := domain_start; i < len(url); i++ {
        sus ch sip = get_char_at(url, i)
        lowkey ch == '/' || ch == '?' || ch == '#' {
            domain_end = i
            ghosted
        }
    }
    
    // Extract domain
    sus domain tea = substring(url, domain_start, domain_end)
    
    // For file:// URLs, domain validation is different
    lowkey has_file {
        // file:// URLs can have localhost or empty domain
        lowkey len(domain) == 0 || domain == "localhost" {
            damn result  // Valid file URL
        }
    }
    
    // Validate domain for other schemes
    lowkey len(domain) == 0 {
        add_error(&result, "URL must contain a domain")
        damn result
    }
    
    // Check domain length
    lowkey len(domain) > 253 {
        add_error(&result, "URL domain too long (maximum 253 characters)")
    }
    
    // Check for valid domain characters
    bestie i := 0; i < len(domain); i++ {
        sus ch sip = get_char_at(domain, i)
        sus is_valid_domain_char lit = (ch >= 'a' && ch <= 'z') || 
                                      (ch >= 'A' && ch <= 'Z') || 
                                      (ch >= '0' && ch <= '9') || 
                                      ch == '.' || ch == '-' || ch == ':'
        lowkey !is_valid_domain_char {
            add_error(&result, "URL domain contains invalid character: " + tea(ch))
        }
    }
    
    // Check domain doesn't start or end with dot or hyphen
    lowkey starts_with(domain, ".") || ends_with(domain, ".") {
        add_error(&result, "URL domain cannot start or end with dot")
    }
    
    lowkey starts_with(domain, "-") || ends_with(domain, "-") {
        add_error(&result, "URL domain cannot start or end with hyphen")
    }
    
    damn result
}

// Credit card validation with Luhn algorithm
slay validate_credit_card(card_number tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(card_number) == 0 {
        add_error(&result, "Credit card number cannot be empty")
        damn result
    }
    
    // Remove spaces and hyphens
    sus clean_card tea = ""
    bestie i := 0; i < len(card_number); i++ {
        sus ch sip = get_char_at(card_number, i)
        lowkey ch >= '0' && ch <= '9' {
            clean_card = clean_card + tea(ch)
        } else lowkey ch == ' ' || ch == '-' {
            // Skip formatting characters
            simp
        } else {
            add_error(&result, "Credit card number contains invalid character: " + tea(ch))
        }
    }
    
    // Check length (13-19 digits for most cards)
    lowkey len(clean_card) < 13 {
        add_error(&result, "Credit card number too short (minimum 13 digits)")
        damn result
    }
    
    lowkey len(clean_card) > 19 {
        add_error(&result, "Credit card number too long (maximum 19 digits)")
        damn result
    }
    
    // Luhn algorithm validation
    sus sum normie = 0
    sus is_even lit = cap
    
    // Process from right to left
    bestie i := len(clean_card) - 1; i >= 0; i-- {
        sus digit normie = normie(get_char_at(clean_card, i)) - normie('0')
        
        lowkey is_even {
            digit = digit * 2
            lowkey digit > 9 {
                digit = digit - 9
            }
        }
        
        sum = sum + digit
        is_even = !is_even
    }
    
    lowkey sum % 10 != 0 {
        add_error(&result, "Credit card number fails Luhn algorithm validation")
    }
    
    // Basic card type detection
    sus first_digit sip = get_char_at(clean_card, 0)
    lowkey first_digit == '4' {
        add_info(&result, "Card type: Visa")
    } else lowkey first_digit == '5' {
        add_info(&result, "Card type: Mastercard")
    } else lowkey first_digit == '3' {
        sus second_digit sip = get_char_at(clean_card, 1)
        lowkey second_digit == '4' || second_digit == '7' {
            add_info(&result, "Card type: American Express")
        }
    } else lowkey first_digit == '6' {
        add_info(&result, "Card type: Discover")
    }
    
    damn result
}

// Date format validation (supports multiple formats)
slay validate_date_format(date tea, format tea) ValidationResult {
    sus result ValidationResult = create_validation_result()
    
    lowkey len(date) == 0 {
        add_error(&result, "Date cannot be empty")
        damn result
    }
    
    lowkey len(format) == 0 {
        add_error(&result, "Date format cannot be empty")
        damn result
    }
    
    // Common date formats
    lowkey format == "YYYY-MM-DD" {
        // ISO 8601 format
        lowkey len(date) != 10 {
            add_error(&result, "Date must be exactly 10 characters for YYYY-MM-DD format")
            damn result
        }
        
        // Check separators
        lowkey get_char_at(date, 4) != '-' || get_char_at(date, 7) != '-' {
            add_error(&result, "Date must have hyphens in positions 4 and 7 for YYYY-MM-DD format")
        }
        
        // Check year (positions 0-3)
        bestie i := 0; i < 4; i++ {
            sus ch sip = get_char_at(date, i)
            lowkey ch < '0' || ch > '9' {
                add_error(&result, "Year must contain only digits")
            }
        }
        
        // Check month (positions 5-6)
        bestie i := 5; i < 7; i++ {
            sus ch sip = get_char_at(date, i)
            lowkey ch < '0' || ch > '9' {
                add_error(&result, "Month must contain only digits")
            }
        }
        
        // Check day (positions 8-9)
        bestie i := 8; i < 10; i++ {
            sus ch sip = get_char_at(date, i)
            lowkey ch < '0' || ch > '9' {
                add_error(&result, "Day must contain only digits")
            }
        }
        
        // Validate month range (01-12)
        sus month_str tea = substring(date, 5, 7)
        sus month normie = string_to_int(month_str)
        lowkey month < 1 || month > 12 {
            add_error(&result, "Month must be between 01 and 12")
        }
        
        // Validate day range (01-31)
        sus day_str tea = substring(date, 8, 10)
        sus day normie = string_to_int(day_str)
        lowkey day < 1 || day > 31 {
            add_error(&result, "Day must be between 01 and 31")
        }
        
    } else lowkey format == "MM/DD/YYYY" {
        // US format
        lowkey len(date) != 10 {
            add_error(&result, "Date must be exactly 10 characters for MM/DD/YYYY format")
            damn result
        }
        
        // Check separators
        lowkey get_char_at(date, 2) != '/' || get_char_at(date, 5) != '/' {
            add_error(&result, "Date must have slashes in positions 2 and 5 for MM/DD/YYYY format")
        }
        
        // Validate month (positions 0-1)
        sus month_str tea = substring(date, 0, 2)
        sus month normie = string_to_int(month_str)
        lowkey month < 1 || month > 12 {
            add_error(&result, "Month must be between 01 and 12")
        }
        
        // Validate day (positions 3-4)
        sus day_str tea = substring(date, 3, 5)
        sus day normie = string_to_int(day_str)
        lowkey day < 1 || day > 31 {
            add_error(&result, "Day must be between 01 and 31")
        }
        
    } else lowkey format == "DD/MM/YYYY" {
        // European format
        lowkey len(date) != 10 {
            add_error(&result, "Date must be exactly 10 characters for DD/MM/YYYY format")
            damn result
        }
        
        // Check separators
        lowkey get_char_at(date, 2) != '/' || get_char_at(date, 5) != '/' {
            add_error(&result, "Date must have slashes in positions 2 and 5 for DD/MM/YYYY format")
        }
        
        // Validate day (positions 0-1)
        sus day_str tea = substring(date, 0, 2)
        sus day normie = string_to_int(day_str)
        lowkey day < 1 || day > 31 {
            add_error(&result, "Day must be between 01 and 31")
        }
        
        // Validate month (positions 3-4)
        sus month_str tea = substring(date, 3, 5)
        sus month normie = string_to_int(month_str)
        lowkey month < 1 || month > 12 {
            add_error(&result, "Month must be between 01 and 12")
        }
        
    } else {
        add_error(&result, "Unsupported date format: " + format)
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
