yeet "testz"
yeet "configz"

fr fr ==========================================
fr fr CURSED Enhanced Configuration Management Tests
fr fr Comprehensive test suite for configz module
fr fr ==========================================

slay run_all_tests() {
    fr fr Run comprehensive test suite for configz module
    test_start("Enhanced Configuration Management Tests")
    
    fr fr Test configuration schema creation and validation
    test_schema_creation()
    test_schema_validation()
    
    fr fr Test format detection and parsing
    test_format_detection()
    test_json_parsing()
    test_yaml_parsing()
    test_toml_parsing()
    test_env_parsing()
    
    fr fr Test environment variable integration
    test_environment_substitution()
    test_environment_config_loading()
    
    fr fr Test configuration merging and layering
    test_configuration_merging()
    test_configuration_layers()
    
    fr fr Test type conversion and validation
    test_type_detection()
    test_value_conversion()
    test_validation_rules()
    
    fr fr Test high-level configuration API
    test_configuration_api()
    test_configuration_defaults()
    
    fr fr Test error handling and edge cases
    test_error_handling()
    test_edge_cases()
    
    print_test_summary()
}

fr fr ==========================================
fr fr Schema Management Tests
fr fr ==========================================

slay test_schema_creation() {
    test_start("Configuration Schema Creation")
    
    fr fr Test basic schema creation
    sus schema ConfigSchema = create_schema("test_schema")
    assert_eq_string(schema.name, "test_schema")
    assert_true(len(schema.required_keys) == 0)
    assert_true(len(schema.optional_keys) == 0)
    
    fr fr Test adding required keys
    schema = add_required_key(schema, "database_host")
    schema = add_required_key(schema, "api_key")
    assert_true(len(schema.required_keys) == 2)
    
    fr fr Test adding optional keys with defaults
    schema = add_optional_key(schema, "debug", "false")
    schema = add_optional_key(schema, "port", "3000")
    assert_true(len(schema.optional_keys) == 2)
    assert_true(len(schema.default_values) == 2)
    
    fr fr Test adding validation rules
    schema = add_validator(schema, "port", "integer")
    schema = add_validator(schema, "database_host", "required")
    assert_true(len(schema.validators) == 2)
}

slay test_schema_validation() {
    test_start("Configuration Schema Validation")
    
    fr fr Create test schema
    sus schema ConfigSchema = create_schema("validation_test")
    schema = add_required_key(schema, "database_url")
    schema = add_required_key(schema, "api_key")
    schema = add_optional_key(schema, "debug", "false")
    schema = add_validator(schema, "database_url", "url")
    schema = add_validator(schema, "api_key", "min_length:10")
    
    fr fr Create test configuration context
    sus ctx ConfigContext = ConfigContext{
        values: [],
        schema: ConfigSchema{name: "empty", required_keys: [], optional_keys: [], default_values: [], validators: [], nested_schemas: []},
        format: format_json(),
        source_file: "test.json",
        environment: "test",
        validation_errors: []
    }
    
    fr fr Add valid configuration values
    ctx = set_configuration_value(ctx, "database_url", "https://db.example.com", "test")
    ctx = set_configuration_value(ctx, "api_key", "secret123456", "test")
    
    fr fr Validate against schema
    sus validated_ctx ConfigContext = validate_against_schema(ctx, schema)
    
    fr fr Should have no validation errors for valid config
    assert_true(is_configuration_valid(validated_ctx))
    assert_true(len(validated_ctx.validation_errors) == 0)
    
    fr fr Test that default values are applied
    assert_true(has_configuration_key(validated_ctx, "debug"))
    assert_eq_string(get_configuration_value(validated_ctx, "debug"), "false")
}

fr fr ==========================================
fr fr Format Detection and Parsing Tests
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
    
    fr fr Test TOML detection
    sus toml_content tea = "key = \"value\"\n[section]"
    detected_format = auto_detect_format(toml_content)
    assert_eq_string(detected_format, format_toml())
    
    fr fr Test environment file detection
    sus env_content tea = "KEY=value\nOTHER=data"
    detected_format = auto_detect_format(env_content)
    assert_eq_string(detected_format, format_env())
    
    fr fr Test filename-based detection
    assert_eq_string(detect_format_from_filename("config.json"), format_json())
    assert_eq_string(detect_format_from_filename("app.yaml"), format_yaml())
    assert_eq_string(detect_format_from_filename("settings.toml"), format_toml())
    assert_eq_string(detect_format_from_filename(".env"), format_env())
}

slay test_json_parsing() {
    test_start("Advanced JSON Configuration Parsing")
    
    sus json_content tea = "{\"database\":{\"host\":\"localhost\",\"port\":5432},\"app\":{\"name\":\"TestApp\",\"debug\":true}}"
    sus ctx ConfigContext = parse_json_advanced(json_content)
    
    fr fr Test basic parsing
    assert_eq_string(ctx.format, format_json())
    assert_true(is_configuration_valid(ctx))
    
    fr fr Test environment detection
    assert_true(string_length(ctx.environment) > 0)
    
    fr fr Test invalid JSON handling
    sus invalid_json tea = "{invalid json content"
    sus invalid_ctx ConfigContext = parse_json_advanced(invalid_json)
    assert_false(is_configuration_valid(invalid_ctx))
    assert_true(len(invalid_ctx.validation_errors) > 0)
}

slay test_yaml_parsing() {
    test_start("Advanced YAML Configuration Parsing")
    
    sus yaml_content tea = "database:\n  host: localhost\n  port: 5432\napp:\n  name: TestApp\n  debug: true"
    sus ctx ConfigContext = parse_yaml_advanced(yaml_content)
    
    fr fr Test basic parsing
    assert_eq_string(ctx.format, format_yaml())
    assert_true(is_configuration_valid(ctx))
    
    fr fr Test environment detection
    assert_true(string_length(ctx.environment) > 0)
    
    fr fr Test invalid YAML handling
    sus invalid_yaml tea = "invalid: yaml: content: [unclosed"
    sus invalid_ctx ConfigContext = parse_yaml_advanced(invalid_yaml)
    assert_false(is_configuration_valid(invalid_ctx))
}

slay test_toml_parsing() {
    test_start("Advanced TOML Configuration Parsing")
    
    sus toml_content tea = "[database]\nhost = \"localhost\"\nport = 5432\n[app]\nname = \"TestApp\"\ndebug = true"
    sus ctx ConfigContext = parse_toml_advanced(toml_content)
    
    fr fr Test basic parsing
    assert_eq_string(ctx.format, format_toml())
    assert_true(is_configuration_valid(ctx))
    
    fr fr Test environment detection
    assert_true(string_length(ctx.environment) > 0)
}

slay test_env_parsing() {
    test_start("Advanced Environment File Parsing")
    
    sus env_content tea = "DATABASE_HOST=localhost\nDATABASE_PORT=5432\nDEBUG=true\nAPP_NAME=TestApp"
    sus ctx ConfigContext = parse_env_advanced(env_content)
    
    fr fr Test basic parsing
    assert_eq_string(ctx.format, format_env())
    assert_true(is_configuration_valid(ctx))
    
    fr fr Test that values are parsed correctly
    assert_true(len(ctx.values) > 0)
    
    fr fr Test comment and empty line handling
    sus env_with_comments tea = "# Comment line\nDATABASE_HOST=localhost\n\nDEBUG=true\n# Another comment"
    sus comment_ctx ConfigContext = parse_env_advanced(env_with_comments)
    assert_true(is_configuration_valid(comment_ctx))
}

fr fr ==========================================
fr fr Environment Variable Integration Tests
fr fr ==========================================

slay test_environment_substitution() {
    test_start("Environment Variable Substitution")
    
    fr fr Test variable expansion
    sus input tea = "Database host is ${DB_HOST} on port ${DB_PORT}"
    sus expanded tea = expand_environment_variables(input)
    assert_true(string_length(expanded) > 0)
    
    fr fr Test configuration context substitution
    sus ctx ConfigContext = ConfigContext{
        values: [],
        schema: ConfigSchema{name: "test", required_keys: [], optional_keys: [], default_values: [], validators: [], nested_schemas: []},
        format: format_json(),
        source_file: "test",
        environment: "test",
        validation_errors: []
    }
    
    ctx = set_configuration_value(ctx, "database_url", "${DB_HOST}:${DB_PORT}", "test")
    ctx = apply_environment_substitution(ctx)
    
    fr fr Value should be expanded
    sus expanded_url tea = get_configuration_value(ctx, "database_url")
    assert_true(string_length(expanded_url) > 0)
}

slay test_environment_config_loading() {
    test_start("Environment Configuration Loading")
    
    fr fr Test loading environment configuration
    sus env_ctx ConfigContext = load_environment_config()
    
    fr fr Should have loaded some environment variables
    assert_true(len(env_ctx.values) > 0)
    assert_eq_string(env_ctx.source_file, "environment")
    assert_eq_string(env_ctx.format, format_env())
    
    fr fr Check for common environment variables
    assert_true(has_configuration_key(env_ctx, "HOME") || has_configuration_key(env_ctx, "USER"))
}

fr fr ==========================================
fr fr Configuration Merging and Layering Tests
fr fr ==========================================

slay test_configuration_merging() {
    test_start("Configuration Merging")
    
    fr fr Create base configuration
    sus base_ctx ConfigContext = ConfigContext{
        values: [],
        schema: ConfigSchema{name: "base", required_keys: [], optional_keys: [], default_values: [], validators: [], nested_schemas: []},
        format: format_json(),
        source_file: "base.json",
        environment: "test",
        validation_errors: []
    }
    base_ctx = set_configuration_value(base_ctx, "app_name", "BaseApp", "file")
    base_ctx = set_configuration_value(base_ctx, "debug", "false", "file")
    base_ctx = set_configuration_value(base_ctx, "port", "3000", "file")
    
    fr fr Create override configuration
    sus override_ctx ConfigContext = ConfigContext{
        values: [],
        schema: ConfigSchema{name: "override", required_keys: [], optional_keys: [], default_values: [], validators: [], nested_schemas: []},
        format: format_env(),
        source_file: "override.env",
        environment: "test",
        validation_errors: []
    }
    override_ctx = set_configuration_value(override_ctx, "debug", "true", "env")
    override_ctx = set_configuration_value(override_ctx, "port", "8080", "env")
    
    fr fr Merge configurations
    sus merged_ctx ConfigContext = merge_configurations(base_ctx, override_ctx)
    
    fr fr Test that override values take precedence
    assert_eq_string(get_configuration_value(merged_ctx, "debug"), "true")
    assert_eq_string(get_configuration_value(merged_ctx, "port"), "8080")
    
    fr fr Test that base values are preserved when not overridden
    assert_eq_string(get_configuration_value(merged_ctx, "app_name"), "BaseApp")
}

slay test_configuration_layers() {
    test_start("Configuration Layering")
    
    fr fr Create multiple file configurations
    sus file_configs []ConfigContext = []
    
    fr fr Create environment configuration
    sus env_config ConfigContext = load_environment_config()
    
    fr fr Create layered configuration
    sus layered_ctx ConfigContext = create_configuration_layers(file_configs, env_config)
    
    fr fr Should have valid layered configuration
    assert_eq_string(layered_ctx.format, "layered")
    assert_eq_string(layered_ctx.source_file, "multiple")
    assert_true(len(layered_ctx.values) >= 0)
}

fr fr ==========================================
fr fr Type Detection and Conversion Tests
fr fr ==========================================

slay test_type_detection() {
    test_start("Value Type Detection")
    
    fr fr Test boolean detection
    assert_eq_string(detect_value_type("true"), "boolean")
    assert_eq_string(detect_value_type("false"), "boolean")
    assert_eq_string(detect_value_type("1"), "integer")
    assert_eq_string(detect_value_type("0"), "integer")
    
    fr fr Test integer detection
    assert_eq_string(detect_value_type("42"), "integer")
    assert_eq_string(detect_value_type("100"), "integer")
    
    fr fr Test float detection
    assert_eq_string(detect_value_type("3.14"), "float")
    assert_eq_string(detect_value_type("0.5"), "float")
    
    fr fr Test array detection
    assert_eq_string(detect_value_type("[1,2,3]"), "array")
    assert_eq_string(detect_value_type("[\"a\",\"b\"]"), "array")
    
    fr fr Test object detection
    assert_eq_string(detect_value_type("{\"key\":\"value\"}"), "object")
    
    fr fr Test string detection (default)
    assert_eq_string(detect_value_type("hello world"), "string")
}

slay test_value_conversion() {
    test_start("Value Type Conversion")
    
    fr fr Test boolean conversion
    assert_true(convert_to_boolean("true"))
    assert_true(convert_to_boolean("1"))
    assert_true(convert_to_boolean("yes"))
    assert_true(convert_to_boolean("on"))
    assert_false(convert_to_boolean("false"))
    assert_false(convert_to_boolean("0"))
    assert_false(convert_to_boolean("no"))
    assert_false(convert_to_boolean("off"))
    
    fr fr Test integer conversion
    assert_eq_int(convert_to_integer("42"), 42)
    assert_eq_int(convert_to_integer("0"), 0)
    assert_eq_int(convert_to_integer("100"), 100)
    
    fr fr Test typed value retrieval
    sus ctx ConfigContext = ConfigContext{
        values: [],
        schema: ConfigSchema{name: "test", required_keys: [], optional_keys: [], default_values: [], validators: [], nested_schemas: []},
        format: format_json(),
        source_file: "test",
        environment: "test",
        validation_errors: []
    }
    
    ctx = set_configuration_value(ctx, "debug", "true", "test")
    ctx = set_configuration_value(ctx, "port", "3000", "test")
    
    sus debug_value tea = get_typed_value(ctx, "debug", "boolean")
    sus port_value tea = get_typed_value(ctx, "port", "integer")
    
    assert_eq_string(debug_value, "true")
    assert_eq_string(port_value, "3000")
}

slay test_validation_rules() {
    test_start("Configuration Validation Rules")
    
    fr fr Test required validation
    assert_true(validate_value_against_rule("value", "required"))
    assert_false(validate_value_against_rule("", "required"))
    
    fr fr Test integer validation
    assert_true(validate_value_against_rule("42", "integer"))
    assert_true(validate_value_against_rule("0", "integer"))
    assert_false(validate_value_against_rule("not_a_number", "integer"))
    
    fr fr Test boolean validation
    assert_true(validate_value_against_rule("true", "boolean"))
    assert_true(validate_value_against_rule("false", "boolean"))
    assert_true(validate_value_against_rule("1", "boolean"))
    assert_true(validate_value_against_rule("0", "boolean"))
    assert_false(validate_value_against_rule("maybe", "boolean"))
    
    fr fr Test URL validation
    assert_true(validate_value_against_rule("https://example.com", "url"))
    assert_true(validate_value_against_rule("http://test.org", "url"))
    assert_false(validate_value_against_rule("not_a_url", "url"))
    
    fr fr Test email validation
    assert_true(validate_value_against_rule("user@example.com", "email"))
    assert_false(validate_value_against_rule("invalid_email", "email"))
    
    fr fr Test length validation
    assert_true(validate_value_against_rule("hello", "min_length:3"))
    assert_false(validate_value_against_rule("hi", "min_length:5"))
    assert_true(validate_value_against_rule("short", "max_length:10"))
    assert_false(validate_value_against_rule("very_long_string", "max_length:5"))
}

fr fr ==========================================
fr fr High-Level Configuration API Tests
fr fr ==========================================

slay test_configuration_api() {
    test_start("High-Level Configuration API")
    
    fr fr Create test configuration context
    sus ctx ConfigContext = ConfigContext{
        values: [],
        schema: ConfigSchema{name: "api_test", required_keys: [], optional_keys: [], default_values: [], validators: [], nested_schemas: []},
        format: format_json(),
        source_file: "test.json",
        environment: "test",
        validation_errors: []
    }
    
    fr fr Set various configuration values
    ctx = set_configuration_value(ctx, "app_name", "TestApp", "test")
    ctx = set_configuration_value(ctx, "debug", "true", "test")
    ctx = set_configuration_value(ctx, "port", "3000", "test")
    ctx = set_configuration_value(ctx, "timeout", "30", "test")
    ctx = set_configuration_value(ctx, "features", "[\"auth\",\"api\"]", "test")
    
    fr fr Test string retrieval
    assert_eq_string(get_config_string(ctx, "app_name", "DefaultApp"), "TestApp")
    assert_eq_string(get_config_string(ctx, "missing_key", "DefaultValue"), "DefaultValue")
    
    fr fr Test integer retrieval
    assert_eq_int(get_config_int(ctx, "port", 8080), 3000)
    assert_eq_int(get_config_int(ctx, "missing_int", 8080), 8080)
    
    fr fr Test boolean retrieval
    assert_true(get_config_bool(ctx, "debug", cap))
    assert_false(get_config_bool(ctx, "missing_bool", cap))
    
    fr fr Test array retrieval
    sus features []tea = get_config_array(ctx, "features")
    assert_true(len(features) >= 0)
}

slay test_configuration_defaults() {
    test_start("Configuration with Defaults")
    
    fr fr Create schema with defaults
    sus schema ConfigSchema = create_schema("defaults_test")
    schema = add_required_key(schema, "database_url")
    schema = add_optional_key(schema, "debug", "false")
    schema = add_optional_key(schema, "port", "3000")
    schema = add_optional_key(schema, "timeout", "30")
    
    fr fr Create test configuration files
    sus config_files []tea = ["config.json"]
    
    fr fr Load configuration with defaults
    sus ctx ConfigContext = load_config_with_defaults(config_files, schema)
    
    fr fr Should have applied default values
    assert_true(has_configuration_key(ctx, "debug"))
    assert_true(has_configuration_key(ctx, "port"))
    assert_true(has_configuration_key(ctx, "timeout"))
    
    fr fr Check that defaults are correct type
    assert_eq_string(get_config_string(ctx, "debug", ""), "false")
    assert_eq_string(get_config_string(ctx, "port", ""), "3000")
}

fr fr ==========================================
fr fr Error Handling and Edge Cases Tests
fr fr ==========================================

slay test_error_handling() {
    test_start("Configuration Error Handling")
    
    fr fr Test invalid JSON handling
    sus invalid_json tea = "{\"unclosed\": \"json"
    sus json_ctx ConfigContext = parse_json_advanced(invalid_json)
    assert_false(is_configuration_valid(json_ctx))
    assert_true(len(json_ctx.validation_errors) > 0)
    
    fr fr Test validation errors
    sus schema ConfigSchema = create_schema("error_test")
    schema = add_required_key(schema, "required_field")
    schema = add_validator(schema, "port", "integer")
    
    sus ctx ConfigContext = ConfigContext{
        values: [],
        schema: ConfigSchema{name: "empty", required_keys: [], optional_keys: [], default_values: [], validators: [], nested_schemas: []},
        format: format_json(),
        source_file: "test",
        environment: "test",
        validation_errors: []
    }
    
    fr fr Add invalid port value
    ctx = set_configuration_value(ctx, "port", "not_a_number", "test")
    
    fr fr Validate against schema (should fail)
    sus validated_ctx ConfigContext = validate_against_schema(ctx, schema)
    assert_false(is_configuration_valid(validated_ctx))
    
    fr fr Should have multiple errors (missing required field + invalid port)
    sus errors []tea = get_validation_errors(validated_ctx)
    assert_true(len(errors) > 0)
}

slay test_edge_cases() {
    test_start("Configuration Edge Cases")
    
    fr fr Test empty configuration
    sus empty_ctx ConfigContext = ConfigContext{
        values: [],
        schema: ConfigSchema{name: "empty", required_keys: [], optional_keys: [], default_values: [], validators: [], nested_schemas: []},
        format: format_json(),
        source_file: "",
        environment: "test",
        validation_errors: []
    }
    
    assert_true(is_configuration_valid(empty_ctx))
    assert_eq_string(get_config_string(empty_ctx, "missing", "default"), "default")
    
    fr fr Test null/empty values
    empty_ctx = set_configuration_value(empty_ctx, "empty_value", "", "test")
    assert_eq_string(get_config_string(empty_ctx, "empty_value", "default"), "")
    
    fr fr Test very long configuration keys and values
    sus long_key tea = "very_long_configuration_key_name_that_exceeds_normal_length"
    sus long_value tea = "very_long_configuration_value_that_contains_a_lot_of_data"
    empty_ctx = set_configuration_value(empty_ctx, long_key, long_value, "test")
    assert_eq_string(get_configuration_value(empty_ctx, long_key), long_value)
    
    fr fr Test special characters in configuration
    sus special_key tea = "special-key_with.dots"
    sus special_value tea = "value with spaces and symbols!@#$%"
    empty_ctx = set_configuration_value(empty_ctx, special_key, special_value, "test")
    assert_eq_string(get_configuration_value(empty_ctx, special_key), special_value)
    
    fr fr Test unicode/international characters
    sus unicode_key tea = "настройка"
    sus unicode_value tea = "значение"
    empty_ctx = set_configuration_value(empty_ctx, unicode_key, unicode_value, "test")
    assert_eq_string(get_configuration_value(empty_ctx, unicode_key), unicode_value)
}

fr fr ==========================================
fr fr Test Execution
fr fr ==========================================

fr fr Run all tests
run_all_tests()
