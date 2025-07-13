yeet "testz"
yeet "config"

# ==========================================
# CURSED Config Library Test Suite
# Multi-Format Configuration Management Tests
# ==========================================

slay test_format_detection() {
    test_start("Config Format Detection")
    
    # Test JSON detection
    sus json_content tea = "{\"key\": \"value\"}"
    sus json_format tea = config.detect_format(json_content)
    assert_eq_string(json_format, "json")
    
    # Test INI detection
    sus ini_content tea = "[section]\nkey=value"
    sus ini_format tea = config.detect_format(ini_content)
    assert_eq_string(ini_format, "ini")
    
    # Test ENV detection
    sus env_content tea = "KEY=value"
    sus env_format tea = config.detect_format(env_content)
    assert_eq_string(env_format, "env")
}

slay test_filename_format_detection() {
    test_start("Config Filename Format Detection")
    
    # Test file extension detection
    assert_eq_string(config.detect_format_from_filename("config.json"), "json")
    assert_eq_string(config.detect_format_from_filename("config.ini"), "ini")
    assert_eq_string(config.detect_format_from_filename(".env"), "env")
    
    # Test default format
    assert_eq_string(config.detect_format_from_filename("unknown"), "json")
}

slay test_environment_variables() {
    test_start("Environment Variable Handling")
    
    # Test getting environment variables
    sus home_value tea = config.get_env("HOME")
    assert_eq_string(home_value, "/home/user")
    
    sus user_value tea = config.get_env("USER")
    assert_eq_string(user_value, "cursed_user")
    
    # Test checking environment variables
    assert_true(config.has_env("HOME"))
    assert_true(config.has_env("PATH"))
    assert_false(config.has_env("NONEXISTENT_VAR"))
    
    # Test setting environment variables
    assert_true(config.set_env("TEST_VAR", "test_value"))
}

slay test_environment_variable_expansion() {
    test_start("Environment Variable Expansion")
    
    # Test basic expansion
    sus template tea = "Path is ${HOME}/documents"
    sus expanded tea = config.expand_env_vars(template)
    assert_eq_string(expanded, "/home/user/documents")
    
    # Test multiple expansions
    sus multi_template tea = "User ${USER} at ${HOME}"
    sus multi_expanded tea = config.expand_env_vars(multi_template)
    assert_eq_string(multi_expanded, "User cursed_user at /home/user")
    
    # Test no expansion needed
    sus no_vars tea = "Just a plain string"
    sus no_expansion tea = config.expand_env_vars(no_vars)
    assert_eq_string(no_expansion, "Just a plain string")
}

slay test_json_config_parsing() {
    test_start("JSON Configuration Parsing")
    
    # Test simple JSON parsing
    sus json_config tea = "{\"database\":\"localhost\"}"
    sus parsed tea = config.parse_json_config(json_config)
    assert_true(config.validate(parsed))
    
    # Test empty JSON
    sus empty_json tea = "{}"
    sus empty_parsed tea = config.parse_json_config(empty_json)
    assert_eq_string(empty_parsed, "{}")
}

slay test_config_parsing_formats() {
    test_start("Multi-Format Configuration Parsing")
    
    # Test INI parsing
    sus ini_config tea = "[database]\nhost=localhost"
    sus ini_parsed tea = config.parse_ini_config(ini_config)
    assert_true(config.validate(ini_parsed))
    
    # Test YAML parsing
    sus yaml_config tea = "database:\n  host: localhost"
    sus yaml_parsed tea = config.parse_yaml_config(yaml_config)
    assert_true(config.validate(yaml_parsed))
    
    # Test ENV parsing
    sus env_config tea = "DATABASE_HOST=localhost"
    sus env_parsed tea = config.parse_env_config(env_config)
    assert_true(config.validate(env_parsed))
}

slay test_auto_format_loading() {
    test_start("Auto-Format Configuration Loading")
    
    # Test JSON auto-loading
    sus json_content tea = "{\"key\": \"value\"}"
    sus json_loaded tea = config.load_config_auto(json_content)
    assert_true(config.validate(json_loaded))
    
    # Test INI auto-loading
    sus ini_content tea = "[section]\nkey=value"
    sus ini_loaded tea = config.load_config_auto(ini_content)
    assert_true(config.validate(ini_loaded))
    
    # Test ENV auto-loading
    sus env_content tea = "KEY=value"
    sus env_loaded tea = config.load_config_auto(env_content)
    assert_true(config.validate(env_loaded))
}

slay test_explicit_format_loading() {
    test_start("Explicit Format Configuration Loading")
    
    # Test loading with specific formats
    sus content tea = "key=value"
    
    sus as_ini tea = config.load_config(content, "ini")
    assert_true(config.validate(as_ini))
    
    sus as_env tea = config.load_config(content, "env")
    assert_true(config.validate(as_env))
    
    # Test JSON format
    sus json_content tea = "{\"test\": \"data\"}"
    sus as_json tea = config.load_config(json_content, "json")
    assert_true(config.validate(as_json))
}

slay test_file_simulation() {
    test_start("Configuration File Simulation")
    
    # Test JSON file simulation
    sus json_config tea = config.load_config_from_file("config.json")
    assert_true(config.validate(json_config))
    
    # Test INI file simulation
    sus ini_config tea = config.load_config_from_file("config.ini")
    assert_true(config.validate(ini_config))
    
    # Test ENV file simulation
    sus env_config tea = config.load_config_from_file(".env")
    assert_true(config.validate(env_config))
}

slay test_configuration_validation() {
    test_start("Configuration Validation")
    
    # Test valid configuration
    sus valid_config tea = "{\"key\": \"value\"}"
    sus schema tea = "{\"key\": \"string\"}"
    assert_true(config.validate_config(valid_config, schema))
    
    # Test basic validation
    assert_true(config.validate(valid_config))
    
    # Test invalid configuration
    sus invalid_config tea = "invalid"
    assert_false(config.validate(invalid_config))
}

slay test_config_key_operations() {
    test_start("Configuration Key Operations")
    
    sus test_config tea = "{\"database\":\"localhost\",\"port\":\"5432\"}"
    
    # Test key existence
    assert_true(config.has_key(test_config, "database"))
    assert_true(config.has_key(test_config, "port"))
    assert_false(config.has_key(test_config, "nonexistent"))
    
    # Test getting values
    sus db_value tea = config.get_config_value(test_config, "database")
    assert_eq_string(db_value, "localhost")
}

slay test_config_value_setting() {
    test_start("Configuration Value Setting")
    
    sus base_config tea = "{\"existing\":\"value\"}"
    
    # Test setting existing value
    sus updated tea = config.set_config_value(base_config, "existing", "new_value")
    assert_true(config.validate(updated))
    
    # Test adding new value
    sus with_new tea = config.set_config_value(base_config, "new_key", "new_value")
    assert_true(config.validate(with_new))
}

slay test_config_merging() {
    test_start("Configuration Merging")
    
    sus config1 tea = "{\"key1\":\"value1\"}"
    sus config2 tea = "{\"key2\":\"value2\"}"
    
    sus merged tea = config.merge_configs(config1, config2)
    assert_true(config.validate(merged))
    
    # Test merging with empty config
    sus empty_config tea = "{}"
    sus merged_empty tea = config.merge_configs(config1, empty_config)
    assert_eq_string(merged_empty, config1)
    
    sus empty_merged tea = config.merge_configs(empty_config, config2)
    assert_eq_string(empty_merged, config2)
}

slay test_high_level_api() {
    test_start("High-Level Configuration API")
    
    # Test main parse function
    sus json_content tea = "{\"api\":\"endpoint\"}"
    sus parsed tea = config.parse(json_content)
    assert_true(config.validate(parsed))
    
    # Test parse with format
    sus ini_content tea = "[api]\nendpoint=localhost"
    sus parsed_ini tea = config.parse_with_format(ini_content, "ini")
    assert_true(config.validate(parsed_ini))
    
    # Test get/set value API
    sus test_config tea = "{\"key\":\"value\"}"
    sus value tea = config.get_value(test_config, "key")
    assert_eq_string(value, "value")
    
    sus updated tea = config.set_value(test_config, "key", "new_value")
    assert_true(config.validate(updated))
    
    # Test merge API
    sus config_a tea = "{\"a\":\"1\"}"
    sus config_b tea = "{\"b\":\"2\"}"
    sus merged tea = config.merge(config_a, config_b)
    assert_true(config.validate(merged))
}

slay test_variable_expansion_integration() {
    test_start("Variable Expansion Integration")
    
    # Test expansion in different formats
    sus env_template tea = "DATABASE_URL=${HOME}/database.db"
    sus expanded tea = config.expand_variables(env_template)
    assert_true(string_length(expanded) > 0)
    
    # Test expansion with config parsing
    sus template_config tea = "host=${HOME}"
    sus expanded_config tea = config.expand_variables(template_config)
    sus parsed_expanded tea = config.parse_with_format(expanded_config, "env")
    assert_true(config.validate(parsed_expanded))
}

slay test_edge_cases() {
    test_start("Configuration Edge Cases")
    
    # Test empty configurations
    sus empty_content tea = ""
    sus empty_parsed tea = config.parse(empty_content)
    assert_true(config.validate(empty_parsed))
    
    # Test simple configurations
    sus simple tea = "key=value"
    sus simple_result tea = config.parse(simple)
    assert_true(config.validate(simple_result))
}

slay test_performance_basics() {
    test_start("Configuration Performance Basics")
    
    # Test parsing performance with basic configs
    sus config1 tea = "{\"key1\":\"value1\"}"
    sus config2 tea = "{\"key2\":\"value2\"}"
    sus config3 tea = "{\"key3\":\"value3\"}"
    
    sus parsed1 tea = config.parse(config1)
    sus parsed2 tea = config.parse(config2)
    sus parsed3 tea = config.parse(config3)
    
    assert_true(config.validate(parsed1))
    assert_true(config.validate(parsed2))
    assert_true(config.validate(parsed3))
}

slay run_all_config_tests() {
    vibez.spill("🔧 Running CURSED Config Library Tests")
    vibez.spill("======================================")
    
    test_format_detection()
    test_filename_format_detection()
    test_environment_variables()
    test_environment_variable_expansion()
    test_json_config_parsing()
    test_config_parsing_formats()
    test_auto_format_loading()
    test_explicit_format_loading()
    test_file_simulation()
    test_configuration_validation()
    test_config_key_operations()
    test_config_value_setting()
    test_config_merging()
    test_high_level_api()
    test_variable_expansion_integration()
    test_edge_cases()
    test_performance_basics()
    
    print_test_summary()
}

# Auto-run tests when this file is executed
run_all_config_tests()
