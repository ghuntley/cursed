yeet "testz"
yeet "configz"

fr fr ==========================================
fr fr Basic Configuration Management Tests
fr fr Simplified tests without array syntax
fr fr ==========================================

slay test_basic_functionality() {
    test_start("Basic Configuration Functions")
    
    fr fr Test format constants
    assert_eq_string(format_json(), "json")
    assert_eq_string(format_yaml(), "yaml")
    assert_eq_string(format_toml(), "toml")
    assert_eq_string(format_env(), "env")
    
    vibez.spill("✓ Format constants working")
}

slay test_environment_variables() {
    test_start("Environment Variables")
    
    fr fr Test basic environment variable retrieval
    sus home_value tea = get_env_variable("HOME")
    assert_eq_string(home_value, "/home/user")
    
    sus user_value tea = get_env_variable("USER")
    assert_eq_string(user_value, "cursed_user")
    
    sus debug_value tea = get_env_variable("DEBUG")
    assert_eq_string(debug_value, "true")
    
    vibez.spill("✓ Environment variables working")
}

slay test_format_detection() {
    test_start("Format Detection")
    
    fr fr Test JSON detection
    sus json_content tea = "{\"key\": \"value\"}"
    sus detected_format tea = auto_detect_format(json_content)
    assert_eq_string(detected_format, format_json())
    
    fr fr Test filename-based detection
    assert_eq_string(detect_format_from_filename("config.json"), format_json())
    assert_eq_string(detect_format_from_filename("app.yaml"), format_yaml())
    assert_eq_string(detect_format_from_filename(".env"), format_env())
    
    vibez.spill("✓ Format detection working")
}

slay test_configuration_parsing() {
    test_start("Configuration Parsing")
    
    fr fr Test JSON parsing
    sus json_content tea = "{\"database\":{\"host\":\"localhost\",\"port\":5432}}"
    sus json_result tea = parse_json_config(json_content)
    assert_true(validate_configuration(json_result))
    
    fr fr Test environment variable expansion
    sus input_with_vars tea = "Database host: ${DB_HOST}"
    sus expanded tea = expand_environment_variables(input_with_vars)
    assert_true(string_length(expanded) > 0)
    
    vibez.spill("✓ Configuration parsing working")
}

slay test_value_access() {
    test_start("Value Access")
    
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
    
    vibez.spill("✓ Value access working")
}

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
    
    assert_false(validate_value_type("not_a_number", "integer"))
    assert_false(validate_value_type("maybe", "boolean"))
    
    vibez.spill("✓ Validation working")
}

slay test_type_conversion() {
    test_start("Type Conversion")
    
    fr fr Test type detection
    assert_eq_string(detect_value_type("true"), "boolean")
    assert_eq_string(detect_value_type("42"), "integer")
    assert_eq_string(detect_value_type("hello"), "string")
    
    fr fr Test boolean conversion
    assert_true(parse_string_to_bool("true"))
    assert_true(parse_string_to_bool("1"))
    assert_false(parse_string_to_bool("false"))
    assert_false(parse_string_to_bool("0"))
    
    fr fr Test integer conversion
    assert_eq_int(parse_string_to_int("42"), 42)
    assert_eq_int(parse_string_to_int("0"), 0)
    assert_eq_int(parse_string_to_int("3000"), 3000)
    
    vibez.spill("✓ Type conversion working")
}

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
    
    vibez.spill("✓ Configuration merging working")
}

slay test_high_level_api() {
    test_start("High-Level API")
    
    fr fr Test parse functions
    sus content tea = "{\"app\":\"api_test\"}"
    sus parsed tea = parse_config(content)
    assert_true(validate_config(parsed))
    
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
    
    vibez.spill("✓ High-level API working")
}

slay test_file_loading() {
    test_start("File Loading Simulation")
    
    fr fr Test file loading
    sus file_result tea = load_configuration_from_file("config.json")
    assert_true(validate_configuration(file_result))
    
    sus env_file_result tea = load_configuration_from_file(".env")
    assert_true(validate_configuration(env_file_result))
    
    vibez.spill("✓ File loading working")
}

slay test_environment_context() {
    test_start("Environment Context")
    
    fr fr Test environment detection
    sus env_context tea = detect_environment_context()
    assert_true(env_context == "development" || env_context == "production" || env_context == "test")
    
    fr fr Test environment configuration loading
    sus env_config tea = load_environment_configuration()
    assert_true(validate_configuration(env_config))
    
    vibez.spill("✓ Environment context working")
}

slay run_all_basic_tests() {
    test_start("CURSED Configuration Management - Basic Tests")
    
    test_basic_functionality()
    test_environment_variables()
    test_format_detection()
    test_configuration_parsing()
    test_value_access()
    test_validation()
    test_type_conversion()
    test_configuration_merging()
    test_high_level_api()
    test_file_loading()
    test_environment_context()
    
    print_test_summary()
    
    vibez.spill("\n🎉 All basic configz tests completed successfully!")
    vibez.spill("✅ Core configuration management functionality verified")
}

fr fr Run the basic tests
run_all_basic_tests()
