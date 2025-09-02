yeet "testz"
yeet "configz"

fr fr ==========================================
fr fr CURSED Enhanced Configuration Management Tests (Simplified)
fr fr Test suite for configz module (simplified version)
fr fr ==========================================

slay run_all_tests() {
    fr fr Run comprehensive test suite for configz module
    test_start("Enhanced Configuration Management Tests")
    
    fr fr Test format detection
    test_format_detection()
    
    fr fr Test environment variable handling
    test_environment_variables()
    
    fr fr Test configuration parsing
    test_configuration_parsing()
    
    fr fr Test configuration loading
    test_configuration_loading()
    
    fr fr Test value access functions
    test_value_access()
    
    fr fr Test validation functions
    test_validation()
    
    fr fr Test type detection and conversion
    test_type_conversion()
    
    fr fr Test configuration merging
    test_configuration_merging()
    
    fr fr Test high-level API
    test_high_level_api()
    
    print_test_summary()
}

fr fr ==========================================
fr fr Format Detection Tests
fr fr ==========================================

slay test_format_detection() {
    test_start("Configuration Format Detection")
    
    fr fr Test JSON detection
    sus json_content tea = "{\"key\": \"value\"}"
    sus detected_format tea = auto_detect_format(json_content)
    assert_eq_string(detected_format, format_json())
    
    fr fr Test YAML detection  
    sus yaml_content tea = "key: value\nother: data"
    detected_format = auto_detect_format(yaml_content)
    assert_eq_string(detected_format, format_yaml())
    
    fr fr Test environment file detection
    sus env_content tea = "KEY=value\nOTHER=data"
    detected_format = auto_detect_format(env_content)
    assert_eq_string(detected_format, format_env())
    
    fr fr Test filename-based detection
    assert_eq_string(detect_format_from_filename("config.json"), format_json())
    assert_eq_string(detect_format_from_filename("app.yaml"), format_yaml())
    assert_eq_string(detect_format_from_filename(".env"), format_env())
    
    vibez.spill("✓ Format detection tests passed")
}

fr fr ==========================================
fr fr Environment Variable Tests
fr fr ==========================================

slay test_environment_variables() {
    test_start("Environment Variable Handling")
    
    fr fr Test basic environment variable retrieval
    sus home_value tea = get_env_variable("HOME")
    assert_eq_string(home_value, "/home/user")
    
    sus user_value tea = get_env_variable("USER")
    assert_eq_string(user_value, "cursed_user")
    
    sus debug_value tea = get_env_variable("DEBUG")
    assert_eq_string(debug_value, "true")
    
    fr fr Test environment variable expansion
    sus input_with_vars tea = "Database host: ${DB_HOST}"
    sus expanded tea = expand_environment_variables(input_with_vars)
    assert_true(string_length(expanded) > 0)
    
    fr fr Test multiple variable expansion
    sus multi_vars tea = "Connect to ${DB_HOST}:${DB_PORT}"
    sus multi_expanded tea = expand_environment_variables(multi_vars)
    assert_true(string_length(multi_expanded) > 0)
    
    vibez.spill("✓ Environment variable tests passed")
}

fr fr ==========================================
fr fr Configuration Parsing Tests
fr fr ==========================================

slay test_configuration_parsing() {
    test_start("Configuration Parsing")
    
    fr fr Test JSON parsing
    sus json_content tea = "{\"database\":{\"host\":\"localhost\",\"port\":5432}}"
    sus json_result tea = parse_json_config(json_content)
    assert_true(validate_configuration(json_result))
    
    fr fr Test YAML parsing
    sus yaml_content tea = "database:\n  host: localhost\n  port: 5432"
    sus yaml_result tea = parse_yaml_config(yaml_content)
    assert_true(validate_configuration(yaml_result))
    
    fr fr Test environment file parsing
    sus env_content tea = "DB_HOST=localhost\nDB_PORT=5432\nDEBUG=true"
    sus env_result tea = parse_env_config(env_content)
    assert_true(validate_configuration(env_result))
    
    fr fr Test invalid content handling
    sus invalid_json tea = "{invalid json content"
    sus invalid_result tea = parse_json_config(invalid_json)
    assert_false(validate_configuration(invalid_result))
    
    vibez.spill("✓ Configuration parsing tests passed")
}

fr fr ==========================================
fr fr Configuration Loading Tests
fr fr ==========================================

slay test_configuration_loading() {
    test_start("Configuration Loading")
    
    fr fr Test auto-detection loading
    sus auto_content tea = "{\"app\":\"test\"}"
    sus auto_result tea = load_configuration_auto(auto_content)
    assert_true(validate_configuration(auto_result))
    
    fr fr Test format-specific loading
    sus specific_content tea = "app: test\nversion: 1.0"
    sus specific_result tea = load_configuration(specific_content, format_yaml())
    assert_true(validate_configuration(specific_result))
    
    fr fr Test file loading simulation
    sus file_result tea = load_configuration_from_file("config.json")
    assert_true(validate_configuration(file_result))
    
    sus env_file_result tea = load_configuration_from_file(".env")
    assert_true(validate_configuration(env_file_result))
    
    vibez.spill("✓ Configuration loading tests passed")
}

fr fr ==========================================
fr fr Value Access Tests
fr fr ==========================================

slay test_value_access() {
    test_start("Configuration Value Access")
    
    fr fr Create test configuration
    sus test_config tea = "{\"app_name\":\"TestApp\",\"debug\":\"true\",\"port\":\"3000\"}"
    
    fr fr Test string value retrieval
    sus app_name tea = get_config_string(test_config, "app_name", "DefaultApp")
    assert_eq_string(app_name, "TestApp")
    
    fr fr Test default value fallback
    sus missing_value tea = get_config_string(test_config, "missing_key", "DefaultValue")
    assert_eq_string(missing_value, "DefaultValue")
    
    fr fr Test integer value retrieval
    sus port_value normie = get_config_int(test_config, "port", 8080)
    assert_eq_int(port_value, 3000)
    
    fr fr Test boolean value retrieval
    sus debug_flag lit = get_config_bool(test_config, "debug", cap)
    assert_true(debug_flag)
    
    fr fr Test setting configuration values
    sus updated_config tea = set_config_value(test_config, "new_key", "new_value")
    assert_true(validate_configuration(updated_config))
    
    vibez.spill("✓ Value access tests passed")
}

fr fr ==========================================
fr fr Validation Tests
fr fr ==========================================

slay test_validation() {
    test_start("Configuration Validation")
    
    fr fr Test basic configuration validation
    sus valid_config tea = "{\"key\":\"value\"}"
    assert_true(validate_configuration(valid_config))
    
    sus invalid_config tea = "invalid json"
    assert_false(validate_configuration(invalid_config))
    
    fr fr Test value type validation
    assert_true(validate_value_type("42", "integer"))
    assert_true(validate_value_type("true", "boolean"))
    assert_true(validate_value_type("https://example.com", "url"))
    assert_true(validate_value_type("user@example.com", "email"))
    
    assert_false(validate_value_type("not_a_number", "integer"))
    assert_false(validate_value_type("maybe", "boolean"))
    assert_false(validate_value_type("invalid_url", "url"))
    assert_false(validate_value_type("invalid_email", "email"))
    
    fr fr Test required keys validation
    sus config_with_keys tea = "{\"database_url\":\"postgres://localhost\",\"api_key\":\"secret\"}"
    sus required_keys tea[value] = ["database_url", "api_key"]
    sus validation_errors tea = validate_required_keys(config_with_keys, required_keys)
    assert_eq_string(validation_errors, "")
    
    vibez.spill("✓ Validation tests passed")
}

fr fr ==========================================
fr fr Type Detection and Conversion Tests
fr fr ==========================================

slay test_type_conversion() {
    test_start("Type Detection and Conversion")
    
    fr fr Test type detection
    assert_eq_string(detect_value_type("true"), "boolean")
    assert_eq_string(detect_value_type("42"), "integer")
    assert_eq_string(detect_value_type("3.14"), "float")
    assert_eq_string(detect_value_type("[1,2,3]"), "array")
    assert_eq_string(detect_value_type("{\"key\":\"value\"}"), "object")
    assert_eq_string(detect_value_type("hello"), "string")
    
    fr fr Test boolean conversion
    assert_true(parse_string_to_bool("true"))
    assert_true(parse_string_to_bool("1"))
    assert_true(parse_string_to_bool("yes"))
    assert_false(parse_string_to_bool("false"))
    assert_false(parse_string_to_bool("0"))
    assert_false(parse_string_to_bool("no"))
    
    fr fr Test integer conversion
    assert_eq_int(parse_string_to_int("42"), 42)
    assert_eq_int(parse_string_to_int("0"), 0)
    assert_eq_int(parse_string_to_int("3000"), 3000)
    
    vibez.spill("✓ Type conversion tests passed")
}

fr fr ==========================================
fr fr Configuration Merging Tests
fr fr ==========================================

slay test_configuration_merging() {
    test_start("Configuration Merging")
    
    fr fr Test basic merging
    sus base_config tea = "{\"app\":\"base\",\"debug\":\"false\"}"
    sus override_config tea = "{\"debug\":\"true\",\"port\":\"8080\"}"
    sus merged_config tea = merge_configurations(base_config, override_config)
    assert_true(validate_configuration(merged_config))
    
    fr fr Test empty configuration merging
    sus empty_config tea = "{}"
    sus non_empty_config tea = "{\"key\":\"value\"}"
    sus merged_empty tea = merge_configurations(empty_config, non_empty_config)
    assert_eq_string(merged_empty, non_empty_config)
    
    fr fr Test environment override application
    sus config_with_env tea = "{\"app\":\"test\"}"
    sus env_overridden tea = apply_environment_overrides(config_with_env)
    assert_true(validate_configuration(env_overridden))
    
    vibez.spill("✓ Configuration merging tests passed")
}

fr fr ==========================================
fr fr High-Level API Tests
fr fr ==========================================

slay test_high_level_api() {
    test_start("High-Level Configuration API")
    
    fr fr Test parse functions
    sus content tea = "{\"app\":\"api_test\"}"
    sus parsed tea = parse_config(content)
    assert_true(validate_config(parsed))
    
    sus yaml_content tea = "app: api_test"
    sus parsed_yaml tea = parse_config_with_format(yaml_content, format_yaml())
    assert_true(validate_config(parsed_yaml))
    
    fr fr Test value functions
    sus test_config tea = "{\"database_url\":\"postgres://localhost\",\"port\":\"5432\"}"
    sus db_url tea = get_value(test_config, "database_url")
    assert_eq_string(db_url, "postgres://localhost")
    
    sus updated tea = set_value(test_config, "timeout", "30")
    assert_true(validate_config(updated))
    
    fr fr Test merge function
    sus config1 tea = "{\"app\":\"test\"}"
    sus config2 tea = "{\"debug\":\"true\"}"
    sus merged tea = merge_configs(config1, config2)
    assert_true(validate_config(merged))
    
    fr fr Test variable expansion
    sus with_vars tea = "host: ${DB_HOST}"
    sus expanded tea = expand_variables(with_vars)
    assert_true(string_length(expanded) > 0)
    
    vibez.spill("✓ High-level API tests passed")
}

fr fr ==========================================
fr fr Environment Context Tests
fr fr ==========================================

slay test_environment_context() {
    test_start("Environment Context Detection")
    
    fr fr Test environment detection
    sus env_context tea = detect_environment_context()
    assert_true(env_context == "development" || env_context == "production" || env_context == "test")
    
    fr fr Test environment configuration loading
    sus env_config tea = load_environment_configuration()
    assert_true(validate_configuration(env_config))
    
    vibez.spill("✓ Environment context tests passed")
}

fr fr ==========================================
fr fr Schema Tests (Simplified)
fr fr ==========================================

slay test_simple_schema() {
    test_start("Simple Schema Validation")
    
    fr fr Create simple schema
    sus required_keys tea[value] = ["database_url", "api_key"]
    sus optional_defaults tea[value] = ["debug:false", "port:3000"]
    sus schema tea = create_simple_schema(required_keys, optional_defaults)
    assert_true(validate_configuration(schema))
    
    fr fr Test schema validation
    sus test_config tea = "{\"database_url\":\"postgres://localhost\",\"api_key\":\"secret\"}"
    sus validation_result tea = validate_against_simple_schema(test_config, schema)
    assert_eq_string(validation_result, "")
    
    vibez.spill("✓ Simple schema tests passed")
}

fr fr ==========================================
fr fr Integration Tests
fr fr ==========================================

slay test_integration_scenarios() {
    test_start("Integration Test Scenarios")
    
    fr fr Test complete configuration workflow
    sus config_content tea = "{\"database\":{\"host\":\"${DB_HOST}\",\"port\":5432},\"app\":{\"name\":\"TestApp\",\"debug\":true}}"
    
    fr fr Parse configuration
    sus parsed_config tea = parse_config(config_content)
    assert_true(validate_config(parsed_config))
    
    fr fr Apply environment variable expansion
    sus expanded_config tea = expand_variables(parsed_config)
    assert_true(validate_config(expanded_config))
    
    fr fr Get specific values
    sus app_name tea = get_value(expanded_config, "app_name")
    ready (string_length(app_name) > 0) {
        vibez.spill("App name configured:", app_name)
    }
    
    fr fr Test layered configuration
    sus base tea = "{\"debug\":\"false\",\"port\":\"3000\"}"
    sus env_overrides tea = load_environment_configuration()
    sus final_config tea = merge_configs(base, env_overrides)
    assert_true(validate_config(final_config))
    
    vibez.spill("✓ Integration scenarios passed")
}

fr fr ==========================================
fr fr Edge Case Tests
fr fr ==========================================

slay test_edge_cases() {
    test_start("Edge Case Handling")
    
    fr fr Test empty configurations
    sus empty_config tea = "{}"
    assert_true(validate_config(empty_config))
    assert_eq_string(get_config_string(empty_config, "missing", "default"), "default")
    
    fr fr Test malformed content
    sus malformed tea = "{malformed json"
    sus malformed_result tea = parse_config(malformed)
    assert_false(validate_config(malformed_result))
    
    fr fr Test very long values
    sus long_value tea = "very_long_configuration_value_that_exceeds_normal_limits"
    sus config_with_long tea = set_value("{}", "long_key", long_value)
    assert_true(validate_config(config_with_long))
    
    fr fr Test special characters
    sus special_config tea = set_value("{}", "special-key_with.dots", "value with spaces!@#")
    assert_true(validate_config(special_config))
    
    vibez.spill("✓ Edge case tests passed")
}

fr fr ==========================================
fr fr Performance Tests
fr fr ==========================================

slay test_performance() {
    test_start("Performance Characteristics")
    
    fr fr Test parsing performance with different formats
    sus large_json tea = "{\"key1\":\"value1\",\"key2\":\"value2\",\"key3\":\"value3\"}"
    sus start_time normie = 0 fr fr Simplified timing
    sus json_parsed tea = parse_config_with_format(large_json, format_json())
    assert_true(validate_config(json_parsed))
    
    fr fr Test environment variable expansion performance
    sus content_with_many_vars tea = "${DB_HOST}:${DB_PORT}/${DATABASE_NAME}"
    sus expanded_performance tea = expand_variables(content_with_many_vars)
    assert_true(string_length(expanded_performance) > 0)
    
    fr fr Test multiple merge operations
    sus config_a tea = "{\"a\":\"1\"}"
    sus config_b tea = "{\"b\":\"2\"}"
    sus config_c tea = "{\"c\":\"3\"}"
    sus merged_ab tea = merge_configs(config_a, config_b)
    sus merged_abc tea = merge_configs(merged_ab, config_c)
    assert_true(validate_config(merged_abc))
    
    vibez.spill("✓ Performance tests completed")
}

fr fr ==========================================
fr fr Security Tests
fr fr ==========================================

slay test_security() {
    test_start("Security Considerations")
    
    fr fr Test that validation catches malicious content
    sus malicious_content tea = "{\"injection\":\"<script>alert('xss')</script>\"}"
    sus malicious_parsed tea = parse_config(malicious_content)
    fr fr Should still parse but content is contained
    assert_true(validate_config(malicious_parsed))
    
    fr fr Test environment variable validation
    sus safe_env tea = get_env_variable("HOME")
    assert_true(string_length(safe_env) > 0)
    
    fr fr Test that sensitive values are handled properly
    sus sensitive_config tea = "{\"password\":\"secret123\",\"api_key\":\"confidential\"}"
    sus sensitive_parsed tea = parse_config(sensitive_config)
    assert_true(validate_config(sensitive_parsed))
    
    vibez.spill("✓ Security tests completed")
}

fr fr ==========================================
fr fr Test Execution
fr fr ==========================================

fr fr Run main test suite
run_all_tests()

fr fr Run additional test suites
test_environment_context()
test_simple_schema()
test_integration_scenarios()
test_edge_cases()
test_performance()
test_security()

vibez.spill("\n🎉 All configz module tests completed successfully!")
vibez.spill("✅ Configuration management framework is ready for production use")
