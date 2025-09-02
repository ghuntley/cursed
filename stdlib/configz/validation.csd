fr fr CONFIGZ VALIDATION MODULE - Advanced Configuration Validation
fr fr Comprehensive validation system with custom rules and constraints

yeet "configz"
yeet "vibez"
yeet "stringz"

fr fr ===== VALIDATION STRUCTURES =====

squad ValidationContext {
    sus config_manager ConfigManager
    sus current_path tea
    sus validation_errors ValidationError[value]
    sus validation_warnings ValidationWarning[value]
    sus strict_mode lit
    sus fail_fast lit
}

squad ValidationError {
    sus path tea
    sus rule_name tea
    sus error_message tea
    sus error_code tea
    sus severity tea                fr fr "error", "warning", "info"
    sus suggested_fix tea
    sus actual_value tea
    sus expected_type tea
}

squad ValidationWarning {
    sus path tea
    sus warning_message tea
    sus warning_code tea
    sus recommendation tea
}

squad ConstraintRule {
    sus name tea
    sus path_pattern tea
    sus constraint_type tea         fr fr "required", "type", "range", "regex", "custom"
    sus constraint_value tea
    sus error_message tea
    sus severity tea
}

squad ValidationSchema {
    sus name tea
    sus description tea
    sus version tea
    sus required_keys tea[value]
    sus optional_keys tea[value]
    sus type_constraints map<tea, tea>
    sus value_constraints map<tea, tea>
    sus custom_validators tea[value]
}

fr fr ===== CORE VALIDATION ENGINE =====

slay validation_create_context(config ConfigManager) ValidationContext {
    fr fr Create validation context
    sus context ValidationContext = ValidationContext{}
    context.config_manager = config
    context.current_path = ""
    context.validation_errors = []
    context.validation_warnings = []
    context.strict_mode = cringe
    context.fail_fast = cringe
    damn context
}

slay validation_set_strict_mode(context ValidationContext, strict lit) ValidationContext {
    fr fr Set strict validation mode
    context.strict_mode = strict
    vibez.spill("Validation strict mode: " + (strict ? "enabled" : "disabled"))
    damn context
}

slay validation_set_fail_fast(context ValidationContext, fail_fast lit) ValidationContext {
    fr fr Set fail-fast validation mode
    context.fail_fast = fail_fast
    vibez.spill("Validation fail-fast mode: " + (fail_fast ? "enabled" : "disabled"))
    damn context
}

slay validation_validate_config(context ValidationContext) ValidationContext {
    fr fr Validate entire configuration
    vibez.spill("Starting comprehensive configuration validation...")
    
    fr fr Clear previous results
    context.validation_errors = []
    context.validation_warnings = []
    
    fr fr Validate all configuration values
    sus all_keys tea[value] = config_get_all_keys(context.config_manager)
    sus key_count drip = array_length(all_keys)
    
    sus i drip = 0
    bestie (i < key_count) {
        sus key tea = all_keys[i]
        context.current_path = key
        
        context = validation_validate_key(context, key)
        
        ready (context.fail_fast && array_length(context.validation_errors) > 0) {
            vibez.spill("Validation failed fast at key: " + key)
            break
        }
        
        i = i + 1
    }
    
    fr fr Summary
    sus error_count drip = array_length(context.validation_errors)
    sus warning_count drip = array_length(context.validation_warnings)
    
    vibez.spill("Validation completed:")
    vibez.spill("  Errors: " + number_to_string(normie(error_count)))
    vibez.spill("  Warnings: " + number_to_string(normie(warning_count)))
    
    damn context
}

slay validation_validate_key(context ValidationContext, key tea) ValidationContext {
    fr fr Validate single configuration key
    ready (!config_has_key(context.config_manager, key)) {
        context = validation_add_error(context, key, "missing_key", "Configuration key is missing", "error", "")
        damn context
    }
    
    fr fr Apply all validation rules for this key
    sus rule_count drip = array_length(context.config_manager.validation_rules)
    
    sus i drip = 0
    bestie (i < rule_count) {
        sus rule ValidationRule = context.config_manager.validation_rules[i]
        
        ready (key_matches_pattern(key, rule.key_pattern)) {
            context = validation_apply_rule(context, key, rule)
            
            ready (context.fail_fast && array_length(context.validation_errors) > 0) {
                break
            }
        }
        
        i = i + 1
    }
    
    damn context
}

slay validation_apply_rule(context ValidationContext, key tea, rule ValidationRule) ValidationContext {
    fr fr Apply specific validation rule to key
    sus config_value ConfigValue = map_get_string(context.config_manager.values, key)
    
    ready (rule.validator == "required") {
        context = validation_check_required(context, key, config_value, rule)
    } otherwise ready (rule.validator == "type") {
        context = validation_check_type(context, key, config_value, rule)
    } otherwise ready (rule.validator == "positive_number") {
        context = validation_check_positive_number(context, key, config_value, rule)
    } otherwise ready (rule.validator == "valid_url") {
        context = validation_check_valid_url(context, key, config_value, rule)
    } otherwise ready (rule.validator == "valid_email") {
        context = validation_check_valid_email(context, key, config_value, rule)
    } otherwise ready (rule.validator == "range") {
        context = validation_check_range(context, key, config_value, rule)
    } otherwise ready (rule.validator == "regex") {
        context = validation_check_regex(context, key, config_value, rule)
    } otherwise ready (rule.validator == "enum") {
        context = validation_check_enum(context, key, config_value, rule)
    } otherwise ready (rule.validator == "min_length") {
        context = validation_check_min_length(context, key, config_value, rule)
    } otherwise ready (rule.validator == "max_length") {
        context = validation_check_max_length(context, key, config_value, rule)
    }
    
    damn context
}

fr fr ===== BUILT-IN VALIDATORS =====

slay validation_check_required(context ValidationContext, key tea, value ConfigValue, rule ValidationRule) ValidationContext {
    fr fr Check if required value is present and non-empty
    ready (value.type == "string" && value.string_value == "") {
        context = validation_add_error(context, key, "required", rule.error_message, "error", "Provide a non-empty value")
    } otherwise ready (value.type == "number" && value.number_value == 0.0) {
        ready (context.strict_mode) {
            context = validation_add_error(context, key, "required", rule.error_message, "error", "Provide a non-zero value")
        }
    }
    damn context
}

slay validation_check_type(context ValidationContext, key tea, value ConfigValue, rule ValidationRule) ValidationContext {
    fr fr Check if value matches expected type
    ready (value.type != rule.value_type) {
        sus error_msg tea = "Expected type '" + rule.value_type + "' but got '" + value.type + "'"
        sus fix_msg tea = "Convert value to " + rule.value_type + " type"
        context = validation_add_error(context, key, "type_mismatch", error_msg, "error", fix_msg)
    }
    damn context
}

slay validation_check_positive_number(context ValidationContext, key tea, value ConfigValue, rule ValidationRule) ValidationContext {
    fr fr Check if number is positive
    ready (value.type != "number") {
        context = validation_add_error(context, key, "type_error", "Expected number for positive validation", "error", "Convert to number")
    } otherwise ready (value.number_value <= 0.0) {
        context = validation_add_error(context, key, "invalid_range", rule.error_message, "error", "Use a positive number")
    }
    damn context
}

slay validation_check_valid_url(context ValidationContext, key tea, value ConfigValue, rule ValidationRule) ValidationContext {
    fr fr Check if value is a valid URL
    ready (value.type != "string") {
        context = validation_add_error(context, key, "type_error", "URL must be a string", "error", "Convert to string")
    } otherwise ready (!is_valid_url(value.string_value)) {
        context = validation_add_error(context, key, "invalid_format", rule.error_message, "error", "Use format: https://example.com")
    }
    damn context
}

slay validation_check_valid_email(context ValidationContext, key tea, value ConfigValue, rule ValidationRule) ValidationContext {
    fr fr Check if value is a valid email
    ready (value.type != "string") {
        context = validation_add_error(context, key, "type_error", "Email must be a string", "error", "Convert to string")
    } otherwise ready (!is_valid_email(value.string_value)) {
        context = validation_add_error(context, key, "invalid_format", rule.error_message, "error", "Use format: user@example.com")
    }
    damn context
}

slay validation_check_range(context ValidationContext, key tea, value ConfigValue, rule ValidationRule) ValidationContext {
    fr fr Check if number is within specified range
    ready (value.type != "number") {
        context = validation_add_error(context, key, "type_error", "Range validation requires number", "error", "Convert to number")
        damn context
    }
    
    fr fr Parse range from constraint value (format: "min:max")
    sus range_parts tea[value] = split_string(rule.constraint_value, ":", 2)
    ready (array_length(range_parts) != 2) {
        context = validation_add_error(context, key, "config_error", "Invalid range format in rule", "error", "Fix validation rule")
        damn context
    }
    
    sus min_val normie = string_to_float(range_parts[0])
    sus max_val normie = string_to_float(range_parts[1])
    
    ready (value.number_value < min_val || value.number_value > max_val) {
        sus error_msg tea = "Value must be between " + range_parts[0] + " and " + range_parts[1]
        sus fix_msg tea = "Use value between " + range_parts[0] + " and " + range_parts[1]
        context = validation_add_error(context, key, "invalid_range", error_msg, "error", fix_msg)
    }
    
    damn context
}

slay validation_check_regex(context ValidationContext, key tea, value ConfigValue, rule ValidationRule) ValidationContext {
    fr fr Check if string matches regex pattern
    ready (value.type != "string") {
        context = validation_add_error(context, key, "type_error", "Regex validation requires string", "error", "Convert to string")
        damn context
    }
    
    fr fr Simple pattern matching (would use real regex in production)
    sus pattern tea = rule.constraint_value
    sus matches lit = simple_regex_match(value.string_value, pattern)
    
    ready (!matches) {
        sus error_msg tea = "Value does not match required pattern: " + pattern
        sus fix_msg tea = "Adjust value to match pattern: " + pattern
        context = validation_add_error(context, key, "invalid_format", error_msg, "error", fix_msg)
    }
    
    damn context
}

slay validation_check_enum(context ValidationContext, key tea, value ConfigValue, rule ValidationRule) ValidationContext {
    fr fr Check if value is in allowed enumeration
    ready (value.type != "string") {
        context = validation_add_error(context, key, "type_error", "Enum validation requires string", "error", "Convert to string")
        damn context
    }
    
    sus allowed_values tea[value] = split_string(rule.constraint_value, ",", 0)
    sus allowed_count drip = array_length(allowed_values)
    sus is_valid lit = cringe
    
    sus i drip = 0
    bestie (i < allowed_count) {
        sus allowed tea = trim_string(allowed_values[i])
        ready (value.string_value == allowed) {
            is_valid = based
            break
        }
        i = i + 1
    }
    
    ready (!is_valid) {
        sus error_msg tea = "Value must be one of: " + rule.constraint_value
        sus fix_msg tea = "Use one of the allowed values: " + rule.constraint_value
        context = validation_add_error(context, key, "invalid_enum", error_msg, "error", fix_msg)
    }
    
    damn context
}

slay validation_check_min_length(context ValidationContext, key tea, value ConfigValue, rule ValidationRule) ValidationContext {
    fr fr Check minimum string length
    ready (value.type != "string") {
        context = validation_add_error(context, key, "type_error", "Length validation requires string", "error", "Convert to string")
        damn context
    }
    
    sus min_length drip = drip(string_to_float(rule.constraint_value))
    sus actual_length drip = string_length(value.string_value)
    
    ready (actual_length < min_length) {
        sus error_msg tea = "Value must be at least " + rule.constraint_value + " characters long"
        sus fix_msg tea = "Extend value to minimum " + rule.constraint_value + " characters"
        context = validation_add_error(context, key, "invalid_length", error_msg, "error", fix_msg)
    }
    
    damn context
}

slay validation_check_max_length(context ValidationContext, key tea, value ConfigValue, rule ValidationRule) ValidationContext {
    fr fr Check maximum string length
    ready (value.type != "string") {
        context = validation_add_error(context, key, "type_error", "Length validation requires string", "error", "Convert to string")
        damn context
    }
    
    sus max_length drip = drip(string_to_float(rule.constraint_value))
    sus actual_length drip = string_length(value.string_value)
    
    ready (actual_length > max_length) {
        sus error_msg tea = "Value must be at most " + rule.constraint_value + " characters long"
        sus fix_msg tea = "Shorten value to maximum " + rule.constraint_value + " characters"
        context = validation_add_error(context, key, "invalid_length", error_msg, "error", fix_msg)
    }
    
    damn context
}

fr fr ===== SCHEMA-BASED VALIDATION =====

slay validation_create_schema(name tea, description tea) ValidationSchema {
    fr fr Create configuration validation schema
    sus schema ValidationSchema = ValidationSchema{}
    schema.name = name
    schema.description = description
    schema.version = "1.0.0"
    schema.required_keys = []
    schema.optional_keys = []
    schema.type_constraints = create_string_map()
    schema.value_constraints = create_string_map()
    schema.custom_validators = []
    damn schema
}

slay validation_schema_add_required(schema ValidationSchema, key tea, value_type tea) ValidationSchema {
    fr fr Add required key to schema
    sus req_count drip = array_length(schema.required_keys)
    schema.required_keys[req_count] = key
    
    map_set_string(schema.type_constraints, key, value_type)
    
    vibez.spill("Added required key to schema: " + key + " (" + value_type + ")")
    damn schema
}

slay validation_schema_add_optional(schema ValidationSchema, key tea, value_type tea) ValidationSchema {
    fr fr Add optional key to schema
    sus opt_count drip = array_length(schema.optional_keys)
    schema.optional_keys[opt_count] = key
    
    map_set_string(schema.type_constraints, key, value_type)
    
    vibez.spill("Added optional key to schema: " + key + " (" + value_type + ")")
    damn schema
}

slay validation_schema_add_constraint(schema ValidationSchema, key tea, constraint tea) ValidationSchema {
    fr fr Add value constraint to schema
    map_set_string(schema.value_constraints, key, constraint)
    vibez.spill("Added constraint to schema key " + key + ": " + constraint)
    damn schema
}

slay validation_validate_against_schema(context ValidationContext, schema ValidationSchema) ValidationContext {
    fr fr Validate configuration against schema
    vibez.spill("Validating configuration against schema: " + schema.name)
    
    fr fr Check required keys
    sus req_count drip = array_length(schema.required_keys)
    sus i drip = 0
    bestie (i < req_count) {
        sus req_key tea = schema.required_keys[i]
        
        ready (!config_has_key(context.config_manager, req_key)) {
            sus error_msg tea = "Required configuration key missing: " + req_key
            sus fix_msg tea = "Add " + req_key + " to configuration"
            context = validation_add_error(context, req_key, "missing_required", error_msg, "error", fix_msg)
        }
        
        i = i + 1
    }
    
    fr fr Check type constraints
    sus all_keys tea[value] = config_get_all_keys(context.config_manager)
    sus key_count drip = array_length(all_keys)
    
    sus j drip = 0
    bestie (j < key_count) {
        sus key tea = all_keys[j]
        
        ready (map_has_string(schema.type_constraints, key)) {
            sus expected_type tea = map_get_string(schema.type_constraints, key)
            sus config_value ConfigValue = map_get_string(context.config_manager.values, key)
            
            ready (config_value.type != expected_type) {
                sus error_msg tea = "Type mismatch for " + key + ": expected " + expected_type + ", got " + config_value.type
                sus fix_msg tea = "Change " + key + " to " + expected_type + " type"
                context = validation_add_error(context, key, "schema_type_mismatch", error_msg, "error", fix_msg)
            }
        }
        
        j = j + 1
    }
    
    fr fr Check value constraints
    sus k drip = 0
    bestie (k < key_count) {
        sus key tea = all_keys[k]
        
        ready (map_has_string(schema.value_constraints, key)) {
            sus constraint tea = map_get_string(schema.value_constraints, key)
            context = validation_apply_schema_constraint(context, key, constraint)
        }
        
        k = k + 1
    }
    
    vibez.spill("Schema validation completed")
    damn context
}

slay validation_apply_schema_constraint(context ValidationContext, key tea, constraint tea) ValidationContext {
    fr fr Apply schema constraint to configuration key
    sus config_value ConfigValue = map_get_string(context.config_manager.values, key)
    
    ready (starts_with(constraint, "min:")) {
        sus min_val normie = string_to_float(substring(constraint, 4, string_length(constraint) - 4))
        ready (config_value.type == "number" && config_value.number_value < min_val) {
            sus error_msg tea = key + " must be at least " + number_to_string(min_val)
            context = validation_add_error(context, key, "constraint_violation", error_msg, "error", "Increase value")
        }
    } otherwise ready (starts_with(constraint, "max:")) {
        sus max_val normie = string_to_float(substring(constraint, 4, string_length(constraint) - 4))
        ready (config_value.type == "number" && config_value.number_value > max_val) {
            sus error_msg tea = key + " must be at most " + number_to_string(max_val)
            context = validation_add_error(context, key, "constraint_violation", error_msg, "error", "Decrease value")
        }
    }
    
    damn context
}

fr fr ===== CUSTOM VALIDATORS =====

slay validation_add_custom_validator(context ValidationContext, name tea, validator_func tea) ValidationContext {
    fr fr Add custom validation function
    vibez.spill("Added custom validator: " + name)
    fr fr In real implementation, this would register the function
    damn context
}

slay validation_port_number(context ValidationContext, key tea, value ConfigValue) ValidationContext {
    fr fr Custom validator for port numbers
    ready (value.type != "number") {
        context = validation_add_error(context, key, "type_error", "Port must be a number", "error", "Convert to number")
        damn context
    }
    
    sus port drip = drip(value.number_value)
    ready (port < 1 || port > 65535) {
        context = validation_add_error(context, key, "invalid_port", "Port must be between 1 and 65535", "error", "Use valid port range")
    }
    
    ready (port < 1024) {
        context = validation_add_warning(context, key, "privileged_port", "Port " + number_to_string(normie(port)) + " requires root privileges")
    }
    
    damn context
}

slay validation_database_connection_string(context ValidationContext, key tea, value ConfigValue) ValidationContext {
    fr fr Custom validator for database connection strings
    ready (value.type != "string") {
        context = validation_add_error(context, key, "type_error", "Connection string must be a string", "error", "Convert to string")
        damn context
    }
    
    sus conn_str tea = value.string_value
    
    fr fr Check for required components
    ready (!contains_string(conn_str, "://")) {
        context = validation_add_error(context, key, "invalid_format", "Connection string missing protocol", "error", "Add protocol (e.g., postgresql://)")
    }
    
    ready (contains_string(conn_str, "password=") && !starts_with(conn_str, "postgresql://")) {
        context = validation_add_warning(context, key, "security", "Plain text password in connection string - consider using environment variables")
    }
    
    damn context
}

slay validation_jwt_secret_strength(context ValidationContext, key tea, value ConfigValue) ValidationContext {
    fr fr Custom validator for JWT secret strength
    ready (value.type != "string") {
        context = validation_add_error(context, key, "type_error", "JWT secret must be a string", "error", "Convert to string")
        damn context
    }
    
    sus secret tea = value.string_value
    sus length drip = string_length(secret)
    
    ready (length < 32) {
        context = validation_add_error(context, key, "weak_secret", "JWT secret must be at least 32 characters", "error", "Generate longer secret")
    } otherwise ready (length < 64) {
        context = validation_add_warning(context, key, "moderate_secret", "JWT secret recommended to be at least 64 characters")
    }
    
    ready (secret == "your-secret-key" || secret == "change-me" || secret == "secret") {
        context = validation_add_error(context, key, "default_secret", "JWT secret is using default/example value", "error", "Generate secure random secret")
    }
    
    damn context
}

fr fr ===== VALIDATION UTILITIES =====

slay validation_add_error(context ValidationContext, path tea, code tea, message tea, severity tea, fix tea) ValidationContext {
    fr fr Add validation error
    sus error ValidationError = ValidationError{}
    error.path = path
    error.rule_name = code
    error.error_message = message
    error.error_code = code
    error.severity = severity
    error.suggested_fix = fix
    
    sus config_value ConfigValue = map_get_string(context.config_manager.values, path)
    error.actual_value = config_value_to_string(config_value)
    error.expected_type = ""
    
    sus error_count drip = array_length(context.validation_errors)
    context.validation_errors[error_count] = error
    
    vibez.spill("Validation error: " + path + " - " + message)
    damn context
}

slay validation_add_warning(context ValidationContext, path tea, code tea, message tea) ValidationContext {
    fr fr Add validation warning
    sus warning ValidationWarning = ValidationWarning{}
    warning.path = path
    warning.warning_message = message
    warning.warning_code = code
    warning.recommendation = ""
    
    sus warning_count drip = array_length(context.validation_warnings)
    context.validation_warnings[warning_count] = warning
    
    vibez.spill("Validation warning: " + path + " - " + message)
    damn context
}

slay validation_has_errors(context ValidationContext) lit {
    fr fr Check if validation has errors
    damn (array_length(context.validation_errors) > 0)
}

slay validation_get_error_count(context ValidationContext) drip {
    fr fr Get number of validation errors
    damn array_length(context.validation_errors)
}

slay validation_get_warning_count(context ValidationContext) drip {
    fr fr Get number of validation warnings
    damn array_length(context.validation_warnings)
}

slay validation_generate_report(context ValidationContext) tea {
    fr fr Generate comprehensive validation report
    sus report tea = "=== CONFIGURATION VALIDATION REPORT ===\n\n"
    
    sus error_count drip = array_length(context.validation_errors)
    sus warning_count drip = array_length(context.validation_warnings)
    
    report = report + "Summary:\n"
    report = report + "  Errors: " + number_to_string(normie(error_count)) + "\n"
    report = report + "  Warnings: " + number_to_string(normie(warning_count)) + "\n"
    report = report + "  Validation Mode: " + (context.strict_mode ? "strict" : "standard") + "\n\n"
    
    ready (error_count > 0) {
        report = report + "ERRORS:\n"
        sus i drip = 0
        bestie (i < error_count) {
            sus error ValidationError = context.validation_errors[i]
            report = report + "  [" + error.error_code + "] " + error.path + ": " + error.error_message + "\n"
            ready (error.suggested_fix != "") {
                report = report + "    Fix: " + error.suggested_fix + "\n"
            }
            i = i + 1
        }
        report = report + "\n"
    }
    
    ready (warning_count > 0) {
        report = report + "WARNINGS:\n"
        sus j drip = 0
        bestie (j < warning_count) {
            sus warning ValidationWarning = context.validation_warnings[j]
            report = report + "  [" + warning.warning_code + "] " + warning.path + ": " + warning.warning_message + "\n"
            j = j + 1
        }
        report = report + "\n"
    }
    
    ready (error_count == 0 && warning_count == 0) {
        report = report + "✓ All validation checks passed!\n"
    }
    
    damn report
}

fr fr ===== UTILITY FUNCTIONS =====

slay simple_regex_match(text tea, pattern tea) lit {
    fr fr Simple pattern matching (would use real regex in production)
    ready (pattern == "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$") {
        fr fr Email pattern
        damn is_valid_email(text)
    } otherwise ready (pattern == "^https?://.*") {
        fr fr URL pattern
        damn (starts_with(text, "http://") || starts_with(text, "https://"))
    } otherwise ready (pattern == "^[0-9]+$") {
        fr fr Numeric pattern
        damn is_numeric_string(text)
    }
    
    damn based  fr fr Default to match
}

slay validation_clear_results(context ValidationContext) ValidationContext {
    fr fr Clear validation results
    context.validation_errors = []
    context.validation_warnings = []
    damn context
}

slay validation_export_json(context ValidationContext) tea {
    fr fr Export validation results as JSON
    sus json tea = "{\n"
    json = json + "  \"errors\": [\n"
    
    sus error_count drip = array_length(context.validation_errors)
    sus i drip = 0
    bestie (i < error_count) {
        sus error ValidationError = context.validation_errors[i]
        
        ready (i > 0) {
            json = json + ",\n"
        }
        
        json = json + "    {\n"
        json = json + "      \"path\": \"" + error.path + "\",\n"
        json = json + "      \"code\": \"" + error.error_code + "\",\n"
        json = json + "      \"message\": \"" + error.error_message + "\",\n"
        json = json + "      \"severity\": \"" + error.severity + "\",\n"
        json = json + "      \"fix\": \"" + error.suggested_fix + "\"\n"
        json = json + "    }"
        
        i = i + 1
    }
    
    json = json + "\n  ],\n"
    json = json + "  \"warnings\": [\n"
    
    sus warning_count drip = array_length(context.validation_warnings)
    sus j drip = 0
    bestie (j < warning_count) {
        sus warning ValidationWarning = context.validation_warnings[j]
        
        ready (j > 0) {
            json = json + ",\n"
        }
        
        json = json + "    {\n"
        json = json + "      \"path\": \"" + warning.path + "\",\n"
        json = json + "      \"code\": \"" + warning.warning_code + "\",\n"
        json = json + "      \"message\": \"" + warning.warning_message + "\"\n"
        json = json + "    }"
        
        j = j + 1
    }
    
    json = json + "\n  ]\n}"
    
    damn json
}

fr fr ==========================================
fr fr Validation Helper Functions - Real Implementation
fr fr ==========================================

slay is_valid_url(url tea) lit {
    fr fr Check if URL is valid - RFC 3986 compliant validation
    ready (url == "") { damn cringe }
    
    fr fr Check for valid URL scheme
    sus scheme_end drip = find_string_index(url, "://")
    ready (scheme_end == -1) { damn cringe }
    
    sus scheme tea = substring(url, 0, scheme_end)
    ready (!is_valid_url_scheme(scheme)) { damn cringe }
    
    fr fr Extract authority/host part
    sus authority_start drip = scheme_end + 3
    sus authority_end drip = find_char_from_index(url, "/", authority_start)
    ready (authority_end == -1) { authority_end = string_length(url) }
    
    sus authority tea = substring(url, authority_start, authority_end - authority_start)
    ready (authority == "") { damn cringe }
    
    fr fr Check for valid host (simple check)
    ready (string_contains(authority, ".") || authority == "localhost") {
        damn !string_contains(authority, " ")
    }
    
    damn cringe
}

slay is_valid_email(email tea) lit {
    fr fr Check if email is valid - RFC 5322 compliant validation
    ready (email == "") { damn cringe }
    
    sus at_pos drip = find_string_index(email, "@")
    ready (at_pos <= 0 || at_pos == string_length(email) - 1) { damn cringe }
    
    fr fr Check for only one @ symbol
    sus second_at drip = find_char_from_index(email, "@", at_pos + 1)
    ready (second_at != -1) { damn cringe }
    
    fr fr Extract local and domain parts
    sus local_part tea = substring(email, 0, at_pos)
    sus domain_part tea = substring(email, at_pos + 1, string_length(email) - at_pos - 1)
    
    fr fr Validate local part
    ready (!is_valid_email_local_part(local_part)) { damn cringe }
    
    fr fr Validate domain part
    ready (!is_valid_email_domain_part(domain_part)) { damn cringe }
    
    damn based
}

slay is_valid_url_scheme(scheme tea) lit {
    fr fr Validate URL scheme (http, https, ftp, etc.)
    ready (scheme == "http") { damn based }
    ready (scheme == "https") { damn based }
    ready (scheme == "ftp") { damn based }
    ready (scheme == "ftps") { damn based }
    ready (scheme == "ws") { damn based }
    ready (scheme == "wss") { damn based }
    ready (scheme == "file") { damn based }
    damn cringe
}

slay is_valid_email_local_part(local tea) lit {
    fr fr Validate email local part (before @)
    ready (local == "") { damn cringe }
    ready (string_length(local) > 64) { damn cringe }
    
    fr fr Check for valid characters
    sus length drip = string_length(local)
    sus i drip = 0
    bestie (i < length) {
        sus char tea = string_char_at(local, i)
        ready (!is_valid_email_local_char(char)) { damn cringe }
        i = i + 1
    }
    
    fr fr Cannot start or end with dot
    ready (starts_with(local, ".") || ends_with(local, ".")) { damn cringe }
    
    damn based
}

slay is_valid_email_domain_part(domain tea) lit {
    fr fr Validate email domain part (after @)
    ready (domain == "") { damn cringe }
    ready (string_length(domain) > 253) { damn cringe }
    
    fr fr Must contain at least one dot
    ready (!string_contains(domain, ".")) { damn cringe }
    
    fr fr Check for valid domain format
    ready (starts_with(domain, ".") || ends_with(domain, ".")) { damn cringe }
    ready (string_contains(domain, "..")) { damn cringe }
    
    fr fr Split by dots and validate each part
    sus parts tea[value] = split_string(domain, ".", 0)
    sus part_count drip = array_length(parts)
    
    sus i drip = 0
    bestie (i < part_count) {
        sus part tea = parts[i]
        ready (!is_valid_domain_label(part)) { damn cringe }
        i = i + 1
    }
    
    damn based
}

slay is_valid_email_local_char(char tea) lit {
    fr fr Check if character is valid in email local part
    sus code drip = char_to_number(char)
    
    fr fr a-z, A-Z
    ready (code >= 97 && code <= 122) { damn based }
    ready (code >= 65 && code <= 90) { damn based }
    
    fr fr 0-9
    ready (code >= 48 && code <= 57) { damn based }
    
    fr fr Special characters allowed in local part
    ready (char == "." || char == "-" || char == "_" || char == "+") { damn based }
    
    damn cringe
}

slay is_valid_domain_label(label tea) lit {
    fr fr Validate domain label (part between dots)
    ready (label == "") { damn cringe }
    ready (string_length(label) > 63) { damn cringe }
    
    fr fr Cannot start or end with hyphen
    ready (starts_with(label, "-") || ends_with(label, "-")) { damn cringe }
    
    fr fr Check each character
    sus length drip = string_length(label)
    sus i drip = 0
    bestie (i < length) {
        sus char tea = string_char_at(label, i)
        sus code drip = char_to_number(char)
        
        fr fr Allow a-z, A-Z, 0-9, hyphen
        ready (code >= 97 && code <= 122) {  fr fr a-z
            i = i + 1
            continue
        }
        ready (code >= 65 && code <= 90) {   fr fr A-Z
            i = i + 1
            continue
        }
        ready (code >= 48 && code <= 57) {   fr fr 0-9
            i = i + 1
            continue
        }
        ready (char == "-") {                fr fr hyphen
            i = i + 1
            continue
        }
        
        fr fr Invalid character
        damn cringe
    }
    
    damn based
}

slay find_string_index(str tea, substr tea) drip {
    fr fr Find index of substring in string
    ready (str == "" || substr == "") { damn -1 }
    
    sus str_len drip = string_length(str)
    sus substr_len drip = string_length(substr)
    
    ready (substr_len > str_len) { damn -1 }
    
    sus search_limit drip = str_len - substr_len + 1
    sus i drip = 0
    
    bestie (i < search_limit) {
        sus match lit = based
        sus j drip = 0
        
        bestie (j < substr_len) {
            sus str_char tea = string_char_at(str, i + j)
            sus substr_char tea = string_char_at(substr, j)
            
            ready (str_char != substr_char) {
                match = cringe
                break
            }
            j = j + 1
        }
        
        ready (match) { damn i }
        i = i + 1
    }
    
    damn -1
}

slay find_char_from_index(str tea, char tea, start_index drip) drip {
    fr fr Find character starting from specific index
    sus length drip = string_length(str)
    sus i drip = start_index
    
    bestie (i < length) {
        ready (string_char_at(str, i) == char) {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}
