fr fr CURSED Config Module Functionality Test
fr fr Tests the fixed config module implementations

yeet "stdlib/config"
yeet "testz"

test_start("Config Module Functionality Tests")

fr fr Test environment variable access
test_env_variables() {
    test_section("Environment Variable Functions")
    
    fr fr Test basic environment variables
    sus home_val tea = get_env("HOME")
    assert_ne_str(home_val, "", "HOME environment variable should not be empty")
    assert_eq_str(home_val, "/home/cursed", "HOME should return correct path")
    
    sus user_val tea = get_env("USER")
    assert_eq_str(user_val, "cursed_dev", "USER should return cursed_dev")
    
    sus path_val tea = get_env("PATH")
    assert_ne_str(path_val, "", "PATH should not be empty")
    
    fr fr Test application-specific variables
    sus debug_val tea = get_env("DEBUG")
    assert_eq_str(debug_val, "true", "DEBUG should return true")
    
    sus port_val tea = get_env("PORT")
    assert_eq_str(port_val, "8080", "PORT should return 8080")
    
    fr fr Test unknown variable
    sus unknown_val tea = get_env("UNKNOWN_VAR")
    assert_eq_str(unknown_val, "", "Unknown variables should return empty string")
    
    test_pass("Environment variable access working correctly")
}

fr fr Test has_env function
test_env_checking() {
    test_section("Environment Variable Checking")
    
    sus has_home lit = has_env("HOME")
    assert_eq_bool(has_home, based, "HOME should exist")
    
    sus has_user lit = has_env("USER")
    assert_eq_bool(has_user, based, "USER should exist")
    
    sus has_unknown lit = has_env("UNKNOWN_VAR")
    assert_eq_bool(has_unknown, cap, "Unknown variable should not exist")
    
    test_pass("Environment variable checking working correctly")
}

fr fr Test set_env function
test_env_setting() {
    test_section("Environment Variable Setting")
    
    fr fr Test valid key setting
    sus result1 lit = set_env("TEST_VAR", "test_value")
    assert_eq_bool(result1, based, "Valid environment variable setting should succeed")
    
    fr fr Test empty key (should fail)
    sus result2 lit = set_env("", "value")
    assert_eq_bool(result2, cap, "Empty key should fail")
    
    fr fr Test invalid key with equals (should fail)
    sus result3 lit = set_env("KEY=BAD", "value")
    assert_eq_bool(result3, cap, "Key with = should fail")
    
    test_pass("Environment variable setting validation working correctly")
}

fr fr Test environment variable expansion
test_env_expansion() {
    test_section("Environment Variable Expansion")
    
    fr fr Test basic expansion
    sus input1 tea = "Welcome to ${HOME}/documents"
    sus expanded1 tea = expand_env_vars(input1)
    assert_ne_str(expanded1, input1, "Environment variables should be expanded")
    
    fr fr Test with multiple variables
    sus input2 tea = "User ${USER} in ${HOME}"
    sus expanded2 tea = expand_env_vars(input2)
    assert_ne_str(expanded2, input2, "Multiple environment variables should be expanded")
    
    fr fr Test no variables
    sus input3 tea = "No variables here"
    sus expanded3 tea = expand_env_vars(input3)
    assert_eq_str(expanded3, input3, "Text without variables should remain unchanged")
    
    test_pass("Environment variable expansion working correctly")
}

fr fr Test configuration format detection
test_format_detection() {
    test_section("Configuration Format Detection")
    
    fr fr Test JSON detection
    sus json_content tea = "{\"key\": \"value\"}"
    sus json_format tea = detect_format(json_content)
    assert_eq_str(json_format, format_json(), "JSON format should be detected correctly")
    
    fr fr Test YAML detection
    sus yaml_content tea = "key: value\nother: data"
    sus yaml_format tea = detect_format(yaml_content)
    assert_eq_str(yaml_format, format_yaml(), "YAML format should be detected correctly")
    
    fr fr Test INI detection
    sus ini_content tea = "[section]\nkey=value"
    sus ini_format tea = detect_format(ini_content)
    assert_eq_str(ini_format, format_ini(), "INI format should be detected correctly")
    
    fr fr Test ENV detection
    sus env_content tea = "KEY=value"
    sus env_format tea = detect_format(env_content)
    assert_eq_str(env_format, format_env(), "ENV format should be detected correctly")
    
    test_pass("Configuration format detection working correctly")
}

fr fr Test filename-based format detection
test_filename_detection() {
    test_section("Filename-based Format Detection")
    
    sus json_filename_format tea = detect_format_from_filename("config.json")
    assert_eq_str(json_filename_format, format_json(), "JSON filename detection should work")
    
    sus yaml_filename_format tea = detect_format_from_filename("config.yaml")
    assert_eq_str(yaml_filename_format, format_yaml(), "YAML filename detection should work")
    
    sus ini_filename_format tea = detect_format_from_filename("config.ini")
    assert_eq_str(ini_filename_format, format_ini(), "INI filename detection should work")
    
    sus env_filename_format tea = detect_format_from_filename(".env")
    assert_eq_str(env_filename_format, format_env(), "ENV filename detection should work")
    
    test_pass("Filename-based format detection working correctly")
}

fr fr Test configuration parsing
test_config_parsing() {
    test_section("Configuration Parsing")
    
    fr fr Test JSON parsing
    sus json_input tea = "{\"database\": {\"host\": \"localhost\"}}"
    sus json_result tea = parse_json_config(json_input)
    assert_ne_str(json_result, "", "JSON parsing should return valid result")
    
    fr fr Test INI parsing
    sus ini_input tea = "[database]\nhost=localhost"
    sus ini_result tea = parse_ini_config(ini_input)
    assert_ne_str(ini_result, "", "INI parsing should return valid result")
    
    fr fr Test YAML parsing
    sus yaml_input tea = "database:\n  host: localhost"
    sus yaml_result tea = parse_yaml_config(yaml_input)
    assert_ne_str(yaml_result, "", "YAML parsing should return valid result")
    
    fr fr Test ENV parsing
    sus env_input tea = "DATABASE_HOST=localhost\nDATABASE_PORT=5432"
    sus env_result tea = parse_env_config(env_input)
    assert_ne_str(env_result, "", "ENV parsing should return valid result")
    
    test_pass("Configuration parsing working correctly")
}

fr fr Test configuration validation
test_config_validation() {
    test_section("Configuration Validation")
    
    fr fr Test valid JSON configuration
    sus valid_config tea = "{\"key\": \"value\"}"
    sus is_valid lit = validate(valid_config)
    assert_eq_bool(is_valid, based, "Valid JSON config should pass validation")
    
    fr fr Test invalid configuration
    sus invalid_config tea = "not json"
    sus is_invalid lit = validate(invalid_config)
    assert_eq_bool(is_invalid, cap, "Invalid config should fail validation")
    
    fr fr Test key existence
    sus config_with_key tea = "{\"database\": \"localhost\"}"
    sus has_db_key lit = has_key(config_with_key, "database")
    assert_eq_bool(has_db_key, based, "Configuration should have database key")
    
    sus has_missing_key lit = has_key(config_with_key, "missing")
    assert_eq_bool(has_missing_key, cap, "Configuration should not have missing key")
    
    test_pass("Configuration validation working correctly")
}

fr fr Test configuration value operations
test_config_values() {
    test_section("Configuration Value Operations")
    
    fr fr Test getting values
    sus config tea = "{\"app\": \"MyApp\", \"debug\": \"true\"}"
    sus app_value tea = get_value(config, "app")
    assert_ne_str(app_value, "", "Should be able to get config values")
    
    sus debug_value tea = get_value(config, "debug")
    assert_ne_str(debug_value, "", "Should be able to get debug value")
    
    fr fr Test missing value
    sus missing_value tea = get_value(config, "missing")
    assert_eq_str(missing_value, "", "Missing values should return empty string")
    
    fr fr Test setting values
    sus updated_config tea = set_value(config, "port", "8080")
    assert_ne_str(updated_config, "", "Setting values should return updated config")
    assert_ne_str(updated_config, config, "Updated config should be different")
    
    test_pass("Configuration value operations working correctly")
}

fr fr Test configuration merging
test_config_merging() {
    test_section("Configuration Merging")
    
    sus config1 tea = "{\"app\": \"MyApp\"}"
    sus config2 tea = "{\"port\": \"8080\"}"
    
    sus merged tea = merge(config1, config2)
    assert_ne_str(merged, "", "Merged config should not be empty")
    assert_ne_str(merged, config1, "Merged config should be different from config1")
    assert_ne_str(merged, config2, "Merged config should be different from config2")
    
    fr fr Test merging with empty config
    sus empty_config tea = "{}"
    sus merged_with_empty tea = merge(config1, empty_config)
    assert_eq_str(merged_with_empty, config1, "Merging with empty should return original")
    
    test_pass("Configuration merging working correctly")
}

fr fr Test high-level API
test_high_level_api() {
    test_section("High-Level API Functions")
    
    fr fr Test auto-parsing
    sus json_content tea = "{\"service\": \"web\", \"port\": \"3000\"}"
    sus parsed tea = parse(json_content)
    assert_ne_str(parsed, "", "Auto-parsing should work")
    
    fr fr Test format-specific parsing
    sus parsed_with_format tea = parse_with_format(json_content, format_json())
    assert_ne_str(parsed_with_format, "", "Format-specific parsing should work")
    
    fr fr Test variable expansion
    sus content_with_vars tea = "Database at ${HOME}/db"
    sus expanded tea = expand_variables(content_with_vars)
    assert_ne_str(expanded, content_with_vars, "Variable expansion should modify content")
    
    test_pass("High-level API working correctly")
}

fr fr Run all tests
main() {
    vibez.spill("Starting Config Module Functionality Tests...")
    
    test_env_variables()
    test_env_checking()
    test_env_setting()
    test_env_expansion()
    test_format_detection()
    test_filename_detection()
    test_config_parsing()
    test_config_validation()
    test_config_values()
    test_config_merging()
    test_high_level_api()
    
    print_test_summary()
    vibez.spill("Config Module functionality test completed!")
}

main()
