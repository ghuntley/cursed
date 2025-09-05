yeet "testz"
yeet "config"

fr fr ==========================================
fr fr CURSED Config Library Test Suite
fr fr Multi-Format Configuration Management Tests
fr fr ==========================================

slay test_format_detection() {
    test_start("Config Format Detection") fr fr Test JSON detection
    sus json_content tea = "{\"key\": \"value\"}"
    sus json_format tea = config.detect_format(json_content)
    assert_eq_string(json_format, "json") fr fr Test INI detection
    sus ini_content tea = "[section]\nkey=value"
    sus ini_format tea = config.detect_format(ini_content)
    assert_eq_string(ini_format, "ini") fr fr Test ENV detection
    sus env_content tea = "KEY=value"
    sus env_format tea = config.detect_format(env_content)
    assert_eq_string(env_format, "env")
}

slay test_filename_format_detection() {
    test_start("Config Filename Format Detection") fr fr Test file extension detection
    assert_eq_string(config.detect_format_from_filename("config.json"), "json")
    assert_eq_string(config.detect_format_from_filename("config.ini"), "ini")
    assert_eq_string(config.detect_format_from_filename(".env"), "env") fr fr Test default format
    assert_eq_string(config.detect_format_from_filename("unknown"), "json")
}

slay test_environment_variables() {
    test_start("Environment Variable Handling") fr fr Test getting environment variables
    sus home_value tea = config.get_env("HOME")
    assert_eq_string(home_value, "/home/user")
    
    sus user_value tea = config.get_env("USER")
    assert_eq_string(user_value, "cursed_user") fr fr Test checking environment variables
    assert_true(config.has_env("HOME"))
    assert_true(config.has_env("PATH"))
    assert_false(config.has_env("NONEXISTENT_VAR")) fr fr Test setting environment variables
    assert_true(config.set_env("TEST_VAR", "test_value"))
}

slay test_environment_variable_expansion() {
    test_start("Environment Variable Expansion") fr fr Test basic expansion
    sus template tea = "Path is ${HOME}/documents"
    sus expanded tea = config.expand_env_vars(template)
    assert_eq_string(expanded, "/home/user/documents") fr fr Test multiple expansions
    sus multi_template tea = "User ${USER} at ${HOME}"
    sus multi_expanded tea = config.expand_env_vars(multi_template)
    assert_eq_string(multi_expanded, "User cursed_user at /home/user") fr fr Test no expansion needed
    sus no_vars tea = "Just a plain string"
    sus no_expansion tea = config.expand_env_vars(no_vars)
    assert_eq_string(no_expansion, "Just a plain string")
}

slay test_json_config_parsing() {
    test_start("JSON Configuration Parsing") fr fr Test simple JSON parsing
    sus json_config tea = "{\"database\":\"localhost\"}"
    sus parsed tea = config.parse_json_config(json_config)
    assert_true(config.validate(parsed)) fr fr Test empty JSON
    sus empty_json tea = "{}"
    sus empty_parsed tea = config.parse_json_config(empty_json)
    assert_eq_string(empty_parsed, "{}")
}

slay test_config_parsing_formats() {
    test_start("Multi-Format Configuration Parsing") fr fr Test INI parsing
    sus ini_config tea = "[database]\nhost=localhost"
    sus ini_parsed tea = config.parse_ini_config(ini_config)
    assert_true(config.validate(ini_parsed)) fr fr Test YAML parsing
    sus yaml_config tea = "database:\n  host: localhost"
    sus yaml_parsed tea = config.parse_yaml_config(yaml_config)
    assert_true(config.validate(yaml_parsed)) fr fr Test ENV parsing
    sus env_config tea = "DATABASE_HOST=localhost"
    sus env_parsed tea = config.parse_env_config(env_config)
    assert_true(config.validate(env_parsed))
}

slay test_auto_format_loading() {
    test_start("Auto-Format Configuration Loading") fr fr Test JSON auto-loading
    sus json_content tea = "{\"key\": \"value\"}"
    sus json_loaded tea = config.load_config_auto(json_content)
    assert_true(config.validate(json_loaded)) fr fr Test INI auto-loading
    sus ini_content tea = "[section]\nkey=value"
    sus ini_loaded tea = config.load_config_auto(ini_content)
    assert_true(config.validate(ini_loaded)) fr fr Test ENV auto-loading
    sus env_content tea = "KEY=value"
    sus env_loaded tea = config.load_config_auto(env_content)
    assert_true(config.validate(env_loaded))
}

slay test_explicit_format_loading() {
    test_start("Explicit Format Configuration Loading") fr fr Test loading with specific formats
    sus content tea = "key=value"
    
    sus as_ini tea = config.load_config(content, "ini")
    assert_true(config.validate(as_ini))
    
    sus as_env tea = config.load_config(content, "env")
    assert_true(config.validate(as_env)) fr fr Test JSON format
    sus json_content tea = "{\"test\": \"data\"}"
    sus as_json tea = config.load_config(json_content, "json")
    assert_true(config.validate(as_json))
}

slay test_file_simulation() {
    test_start("Configuration File Simulation") fr fr Test JSON file simulation
    sus json_config tea = config.load_config_from_file("config.json")
    assert_true(config.validate(json_config)) fr fr Test INI file simulation
    sus ini_config tea = config.load_config_from_file("config.ini")
    assert_true(config.validate(ini_config)) fr fr Test ENV file simulation
    sus env_config tea = config.load_config_from_file(".env")
    assert_true(config.validate(env_config))
}

slay test_configuration_validation() {
    test_start("Configuration Validation") fr fr Test valid configuration
    sus valid_config tea = "{\"key\": \"value\"}"
    sus schema tea = "{\"key\": \"string\"}"
    assert_true(config.validate_config(valid_config, schema)) fr fr Test basic validation
    assert_true(config.validate(valid_config)) fr fr Test invalid configuration
    sus invalid_config tea = "invalid"
    assert_false(config.validate(invalid_config))
}

slay test_config_key_operations() {
    test_start("Configuration Key Operations")
    
    sus test_config tea = "{\"database\":\"localhost\",\"port\":\"5432\"}" fr fr Test key existence
    assert_true(config.has_key(test_config, "database"))
    assert_true(config.has_key(test_config, "port"))
    assert_false(config.has_key(test_config, "nonexistent")) fr fr Test getting values
    sus db_value tea = config.get_config_value(test_config, "database")
    assert_eq_string(db_value, "localhost")
}

slay test_config_value_setting() {
    test_start("Configuration Value Setting")
    
    sus base_config tea = "{\"existing\":\"value\"}" fr fr Test setting existing value
    sus updated tea = config.set_config_value(base_config, "existing", "new_value")
    assert_true(config.validate(updated)) fr fr Test adding new value
    sus with_new tea = config.set_config_value(base_config, "new_key", "new_value")
    assert_true(config.validate(with_new))
}

slay test_config_merging() {
    test_start("Configuration Merging")
    
    sus config1 tea = "{\"key1\":\"value1\"}"
    sus config2 tea = "{\"key2\":\"value2\"}"
    
    sus merged tea = config.merge_configs(config1, config2)
    assert_true(config.validate(merged)) fr fr Test merging with empty config
    sus empty_config tea = "{}"
    sus merged_empty tea = config.merge_configs(config1, empty_config)
    assert_eq_string(merged_empty, config1)
    
    sus empty_merged tea = config.merge_configs(empty_config, config2)
    assert_eq_string(empty_merged, config2)
}

slay test_high_level_api() {
    test_start("High-Level Configuration API") fr fr Test main parse function
    sus json_content tea = "{\"api\":\"endpoint\"}"
    sus parsed tea = config.parse(json_content)
    assert_true(config.validate(parsed)) fr fr Test parse with format
    sus ini_content tea = "[api]\nendpoint=localhost"
    sus parsed_ini tea = config.parse_with_format(ini_content, "ini")
    assert_true(config.validate(parsed_ini)) fr fr Test get/set value API
    sus test_config tea = "{\"key\":\"value\"}"
    sus value tea = config.get_value(test_config, "key")
    assert_eq_string(value, "value")
    
    sus updated tea = config.set_value(test_config, "key", "new_value")
    assert_true(config.validate(updated)) fr fr Test merge API
    sus config_a tea = "{\"a\":\"1\"}"
    sus config_b tea = "{\"b\":\"2\"}"
    sus merged tea = config.merge(config_a, config_b)
    assert_true(config.validate(merged))
}

slay test_variable_expansion_integration() {
    test_start("Variable Expansion Integration") fr fr Test expansion in different formats
    sus env_template tea = "DATABASE_URL=${HOME}/database.db"
    sus expanded tea = config.expand_variables(env_template)
    assert_true(string_length(expanded) > 0) fr fr Test expansion with config parsing
    sus template_config tea = "host=${HOME}"
    sus expanded_config tea = config.expand_variables(template_config)
    sus parsed_expanded tea = config.parse_with_format(expanded_config, "env")
    assert_true(config.validate(parsed_expanded))
}

slay test_edge_cases() {
    test_start("Configuration Edge Cases") fr fr Test empty configurations
    sus empty_content tea = ""
    sus empty_parsed tea = config.parse(empty_content)
    assert_true(config.validate(empty_parsed)) fr fr Test simple configurations
    sus simple tea = "key=value"
    sus simple_result tea = config.parse(simple)
    assert_true(config.validate(simple_result))
}

slay test_performance_basics() {
    test_start("Configuration Performance Basics") fr fr Test parsing performance with basic configs
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

fr fr Auto-run tests when this file is executed
run_all_config_tests()
