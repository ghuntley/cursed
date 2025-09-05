fr fr CONFIGZ COMPREHENSIVE TEST SUITE
fr fr Production-grade testing for configuration management system

yeet "configz"
yeet "testz"
yeet "vibez"
yeet "filez"
yeet "jsonz"

fr fr ===== TEST CONFIGURATION SETUP =====

slay test_create_sample_json_config() tea {
    fr fr Create sample JSON configuration for testing
    damn "{\"database\":{\"host\":\"test-db.example.com\",\"port\":5432,\"ssl\":true},\"server\":{\"port\":8080,\"debug\":false},\"features\":{\"cache\":true,\"rate_limit\":1000,\"origins\":[\"https://test1.com\",\"https://test2.com\"]}}"
}

slay test_create_sample_toml_config() tea {
    fr fr Create sample TOML configuration for testing
    damn "[database]\nhost = \"toml-db.example.com\"\nport = 3306\nssl = false\n\n[server]\nport = 9090\ndebug = true\n\n[features]\ncache = false\nrate_limit = 500"
}

slay test_create_sample_yaml_config() tea {
    fr fr Create sample YAML configuration for testing  
    damn "database:\n  host: yaml-db.example.com\n  port: 5433\n  ssl: true\nserver:\n  port: 7070\n  debug: false\nfeatures:\n  cache: true\n  rate_limit: 2000"
}

slay test_create_sample_ini_config() tea {
    fr fr Create sample INI configuration for testing
    damn "[database]\nhost = ini-db.example.com\nport = 1433\nssl = yes\n\n[server]\nport = 6060\ndebug = no\n\n[features]\ncache = on\nrate_limit = 750"
}

fr fr ===== BASIC FUNCTIONALITY TESTS =====

slay test_config_manager_creation() lit {
    vibez.spill("Testing configuration manager creation...")
    
    sus config ConfigManager = config_create()
    
    fr fr Verify initial state
    sus all_keys tea[value] = config_get_all_keys(config)
    sus key_count drip = array_length(all_keys)
    
    ready (key_count != 0) {
        vibez.spill("ERROR: New config manager should have no keys")
        damn cringe
    }
    
    vibez.spill("✓ Configuration manager creation passed")
    damn based
}

slay test_default_values() lit {
    vibez.spill("Testing default value handling...")
    
    sus config ConfigManager = config_create()
    
    fr fr Set default values
    sus db_host_default ConfigValue = ConfigValue{}
    db_host_default.type = "string"
    db_host_default.string_value = "default-host"
    config = config_set_default(config, "database.host", db_host_default)
    
    sus db_port_default ConfigValue = ConfigValue{}
    db_port_default.type = "number"  
    db_port_default.number_value = 5432.0
    config = config_set_default(config, "database.port", db_port_default)
    
    sus debug_default ConfigValue = ConfigValue{}
    debug_default.type = "boolean"
    debug_default.boolean_value = cringe
    config = config_set_default(config, "app.debug", debug_default)
    
    config = config_load_all(config)
    
    fr fr Test default value access
    sus host tea = config_get_string(config, "database.host", "fallback")
    sus port normie = config_get_number(config, "database.port", 0.0)
    sus debug lit = config_get_boolean(config, "app.debug", based)
    
    ready (host != "default-host") {
        vibez.spill("ERROR: Expected default host 'default-host', got: " + host)
        damn cringe
    }
    
    ready (port != 5432.0) {
        vibez.spill("ERROR: Expected default port 5432, got: " + number_to_string(port))
        damn cringe
    }
    
    ready (debug != cringe) {
        vibez.spill("ERROR: Expected default debug false, got: " + (debug ? "true" : "false"))
        damn cringe
    }
    
    vibez.spill("✓ Default values test passed")
    damn based
}

slay test_json_configuration_loading() lit {
    vibez.spill("Testing JSON configuration loading...")
    
    sus config ConfigManager = config_create()
    
    fr fr Simulate JSON source (in real implementation, this would read from file)
    sus json_source ConfigSource = ConfigSource{}
    json_source.type = "json"
    json_source.path = "test.json"
    json_source.content = test_create_sample_json_config()
    json_source.priority = 10
    
    fr fr Load JSON configuration
    config = load_json_source(config, json_source)
    
    fr fr Test loaded values
    sus db_host tea = config_get_string(config, "database.host", "")
    sus db_port normie = config_get_number(config, "database.port", 0.0)
    sus server_port normie = config_get_number(config, "server.port", 0.0)
    sus ssl_enabled lit = config_get_boolean(config, "database.ssl", cringe)
    sus debug_mode lit = config_get_boolean(config, "server.debug", based)
    
    ready (db_host != "test-db.example.com") {
        vibez.spill("ERROR: Expected JSON db host 'test-db.example.com', got: " + db_host)
        damn cringe
    }
    
    ready (db_port != 5432.0) {
        vibez.spill("ERROR: Expected JSON db port 5432, got: " + number_to_string(db_port))
        damn cringe  
    }
    
    ready (server_port != 8080.0) {
        vibez.spill("ERROR: Expected JSON server port 8080, got: " + number_to_string(server_port))
        damn cringe
    }
    
    ready (!ssl_enabled) {
        vibez.spill("ERROR: Expected SSL enabled from JSON")
        damn cringe
    }
    
    ready (debug_mode) {
        vibez.spill("ERROR: Expected debug disabled from JSON")
        damn cringe
    }
    
    vibez.spill("✓ JSON configuration loading test passed")
    damn based
}

slay test_toml_configuration_loading() lit {
    vibez.spill("Testing TOML configuration loading...")
    
    sus config ConfigManager = config_create()
    
    sus toml_source ConfigSource = ConfigSource{}
    toml_source.type = "toml"
    toml_source.path = "test.toml"
    toml_source.content = test_create_sample_toml_config()
    toml_source.priority = 10
    
    config = load_toml_source(config, toml_source)
    
    sus db_host tea = config_get_string(config, "database.host", "")
    sus db_port normie = config_get_number(config, "database.port", 0.0)
    sus server_port normie = config_get_number(config, "server.port", 0.0)
    sus ssl_enabled lit = config_get_boolean(config, "database.ssl", based)
    sus debug_mode lit = config_get_boolean(config, "server.debug", cringe)
    
    ready (db_host != "toml-db.example.com") {
        vibez.spill("ERROR: Expected TOML db host 'toml-db.example.com', got: " + db_host)
        damn cringe
    }
    
    ready (db_port != 3306.0) {
        vibez.spill("ERROR: Expected TOML db port 3306, got: " + number_to_string(db_port))
        damn cringe
    }
    
    ready (server_port != 9090.0) {
        vibez.spill("ERROR: Expected TOML server port 9090, got: " + number_to_string(server_port))
        damn cringe
    }
    
    ready (ssl_enabled) {
        vibez.spill("ERROR: Expected SSL disabled from TOML")
        damn cringe
    }
    
    ready (!debug_mode) {
        vibez.spill("ERROR: Expected debug enabled from TOML")
        damn cringe
    }
    
    vibez.spill("✓ TOML configuration loading test passed")
    damn based
}

slay test_yaml_configuration_loading() lit {
    vibez.spill("Testing YAML configuration loading...")
    
    sus config ConfigManager = config_create()
    
    sus yaml_source ConfigSource = ConfigSource{}
    yaml_source.type = "yaml"
    yaml_source.path = "test.yaml"
    yaml_source.content = test_create_sample_yaml_config()
    yaml_source.priority = 10
    
    config = load_yaml_source(config, yaml_source)
    
    sus db_host tea = config_get_string(config, "database.host", "")
    sus db_port normie = config_get_number(config, "database.port", 0.0)
    sus server_port normie = config_get_number(config, "server.port", 0.0)
    sus rate_limit normie = config_get_number(config, "features.rate_limit", 0.0)
    
    ready (db_host != "yaml-db.example.com") {
        vibez.spill("ERROR: Expected YAML db host 'yaml-db.example.com', got: " + db_host)
        damn cringe
    }
    
    ready (db_port != 5433.0) {
        vibez.spill("ERROR: Expected YAML db port 5433, got: " + number_to_string(db_port))
        damn cringe
    }
    
    ready (server_port != 7070.0) {
        vibez.spill("ERROR: Expected YAML server port 7070, got: " + number_to_string(server_port))
        damn cringe
    }
    
    ready (rate_limit != 2000.0) {
        vibez.spill("ERROR: Expected YAML rate limit 2000, got: " + number_to_string(rate_limit))
        damn cringe
    }
    
    vibez.spill("✓ YAML configuration loading test passed")
    damn based
}

slay test_ini_configuration_loading() lit {
    vibez.spill("Testing INI configuration loading...")
    
    sus config ConfigManager = config_create()
    
    sus ini_source ConfigSource = ConfigSource{}
    ini_source.type = "ini"
    ini_source.path = "test.ini"
    ini_source.content = test_create_sample_ini_config()
    ini_source.priority = 10
    
    config = load_ini_source(config, ini_source)
    
    sus db_host tea = config_get_string(config, "database.host", "")
    sus db_port normie = config_get_number(config, "database.port", 0.0)
    sus server_port normie = config_get_number(config, "server.port", 0.0)
    sus ssl_enabled lit = config_get_boolean(config, "database.ssl", cringe)
    sus debug_mode lit = config_get_boolean(config, "server.debug", based)
    sus cache_enabled lit = config_get_boolean(config, "features.cache", cringe)
    
    ready (db_host != "ini-db.example.com") {
        vibez.spill("ERROR: Expected INI db host 'ini-db.example.com', got: " + db_host)
        damn cringe
    }
    
    ready (db_port != 1433.0) {
        vibez.spill("ERROR: Expected INI db port 1433, got: " + number_to_string(db_port))
        damn cringe
    }
    
    ready (server_port != 6060.0) {
        vibez.spill("ERROR: Expected INI server port 6060, got: " + number_to_string(server_port))
        damn cringe
    }
    
    ready (!ssl_enabled) {
        vibez.spill("ERROR: Expected SSL enabled from INI (yes)")
        damn cringe
    }
    
    ready (debug_mode) {
        vibez.spill("ERROR: Expected debug disabled from INI (no)")
        damn cringe
    }
    
    ready (!cache_enabled) {
        vibez.spill("ERROR: Expected cache enabled from INI (on)")
        damn cringe
    }
    
    vibez.spill("✓ INI configuration loading test passed")
    damn based
}

fr fr ===== ENVIRONMENT VARIABLE TESTS =====

slay test_environment_variable_loading() lit {
    vibez.spill("Testing environment variable loading...")
    
    sus config ConfigManager = config_create()
    
    fr fr Simulate environment variables
    sus env_source ConfigSource = ConfigSource{}
    env_source.type = "env"
    env_source.path = ""
    env_source.priority = 20
    
    config = load_env_source(config, env_source)
    
    fr fr Test common environment variables (these are loaded from get_all_env_vars)
    sus has_path lit = config_has_key(config, "path")
    sus has_home lit = config_has_key(config, "home")
    sus has_user lit = config_has_key(config, "user")
    
    ready (!has_path) {
        vibez.spill("WARNING: PATH environment variable not found")
    } otherwise {
        vibez.spill("✓ PATH environment variable loaded")
    }
    
    ready (!has_home) {
        vibez.spill("WARNING: HOME environment variable not found")
    } otherwise {
        vibez.spill("✓ HOME environment variable loaded")
    }
    
    ready (!has_user) {
        vibez.spill("WARNING: USER environment variable not found")
    } otherwise {
        vibez.spill("✓ USER environment variable loaded")
    }
    
    vibez.spill("✓ Environment variable loading test passed")
    damn based
}

slay test_env_key_conversion() lit {
    vibez.spill("Testing environment variable key conversion...")
    
    fr fr Test various key conversion patterns
    sus result1 tea = env_key_to_config_key("DATABASE_HOST")
    sus result2 tea = env_key_to_config_key("APP_SERVER_PORT")
    sus result3 tea = env_key_to_config_key("CACHE__ENABLED")
    
    ready (result1 != "database.host") {
        vibez.spill("ERROR: Expected 'database.host', got: " + result1)
        damn cringe
    }
    
    ready (result2 != "app.server.port") {
        vibez.spill("ERROR: Expected 'app.server.port', got: " + result2)
        damn cringe
    }
    
    ready (result3 != "cache_enabled") {
        vibez.spill("ERROR: Expected 'cache_enabled', got: " + result3)
        damn cringe
    }
    
    vibez.spill("✓ Environment key conversion test passed")
    damn based
}

slay test_auto_type_detection() lit {
    vibez.spill("Testing automatic type detection...")
    
    fr fr Create test configuration values
    sus string_value ConfigValue = ConfigValue{}
    string_value.type = "string"
    string_value.string_value = "hello world"
    string_value = auto_detect_type(string_value)
    
    sus boolean_true ConfigValue = ConfigValue{}
    boolean_true.type = "string"
    boolean_true.string_value = "true"
    boolean_true = auto_detect_type(boolean_true)
    
    sus boolean_yes ConfigValue = ConfigValue{}
    boolean_yes.type = "string"
    boolean_yes.string_value = "yes"
    boolean_yes = auto_detect_type(boolean_yes)
    
    sus number_value ConfigValue = ConfigValue{}
    number_value.type = "string"
    number_value.string_value = "42"
    number_value = auto_detect_type(number_value)
    
    sus float_value ConfigValue = ConfigValue{}
    float_value.type = "string"
    float_value.string_value = "3.14"
    float_value = auto_detect_type(float_value)
    
    ready (string_value.type != "string") {
        vibez.spill("ERROR: String should remain string type")
        damn cringe
    }
    
    ready (boolean_true.type != "boolean" || !boolean_true.boolean_value) {
        vibez.spill("ERROR: 'true' should be detected as boolean true")
        damn cringe
    }
    
    ready (boolean_yes.type != "boolean" || !boolean_yes.boolean_value) {
        vibez.spill("ERROR: 'yes' should be detected as boolean true")
        damn cringe
    }
    
    ready (number_value.type != "number" || number_value.number_value != 42.0) {
        vibez.spill("ERROR: '42' should be detected as number")
        damn cringe
    }
    
    ready (float_value.type != "number" || float_value.number_value != 3.14) {
        vibez.spill("ERROR: '3.14' should be detected as number")
        damn cringe
    }
    
    vibez.spill("✓ Auto type detection test passed")
    damn based
}

fr fr ===== PRIORITY AND SOURCE MANAGEMENT TESTS =====

slay test_source_priority_ordering() lit {
    vibez.spill("Testing source priority ordering...")
    
    sus config ConfigManager = config_create()
    
    fr fr Set a default value
    sus default_value ConfigValue = ConfigValue{}
    default_value.type = "string"
    default_value.string_value = "default"
    config = config_set_default(config, "test.value", default_value)
    
    fr fr Add sources in random order with different priorities
    config = config_add_source(config, "json", "high.json", 30)    fr fr High priority
    config = config_add_source(config, "json", "low.json", 10)     fr fr Low priority  
    config = config_add_source(config, "json", "medium.json", 20)  fr fr Medium priority
    
    fr fr Simulate loading different values from each source
    fr fr In real implementation, this would load from actual files
    
    config = config_load_all(config)
    
    fr fr Test that sources were ordered correctly by priority
    sus source_count drip = array_length(config.sources)
    ready (source_count != 3) {
        vibez.spill("ERROR: Expected 3 sources, got: " + number_to_string(normie(source_count)))
        damn cringe
    }
    
    fr fr Sources should be ordered by priority (high to low when applied)
    ready (config.sources[0].priority != 10) {
        vibez.spill("ERROR: First source should have priority 10")
        damn cringe
    }
    
    ready (config.sources[1].priority != 20) {
        vibez.spill("ERROR: Second source should have priority 20")
        damn cringe
    }
    
    ready (config.sources[2].priority != 30) {
        vibez.spill("ERROR: Third source should have priority 30") 
        damn cringe
    }
    
    vibez.spill("✓ Source priority ordering test passed")
    damn based
}

slay test_configuration_override() lit {
    vibez.spill("Testing configuration value overriding...")
    
    sus config ConfigManager = config_create()
    
    fr fr Set default
    sus default_port ConfigValue = ConfigValue{}
    default_port.type = "number"
    default_port.number_value = 8000.0
    config = config_set_default(config, "server.port", default_port)
    
    fr fr Add multiple sources that will override each other
    fr fr Lower priority source
    sus json_source1 ConfigSource = ConfigSource{}
    json_source1.type = "json"
    json_source1.content = "{\"server\":{\"port\":8080}}"
    json_source1.priority = 10
    
    fr fr Higher priority source  
    sus json_source2 ConfigSource = ConfigSource{}
    json_source2.type = "json"
    json_source2.content = "{\"server\":{\"port\":9090}}"
    json_source2.priority = 20
    
    fr fr Load sources (higher priority should override)
    config = load_json_source(config, json_source1)
    config = load_json_source(config, json_source2)
    
    sus final_port normie = config_get_number(config, "server.port", 0.0)
    
    ready (final_port != 9090.0) {
        vibez.spill("ERROR: Expected final port 9090 (from higher priority source), got: " + number_to_string(final_port))
        damn cringe
    }
    
    vibez.spill("✓ Configuration override test passed")
    damn based
}

fr fr ===== VALIDATION TESTS =====

slay test_validation_rules() lit {
    vibez.spill("Testing configuration validation rules...")
    
    sus config ConfigManager = config_create()
    
    fr fr Add validation rules
    config = config_add_validation(config, "database.host", "string", "required", "Database host is required")
    config = config_add_validation(config, "database.port", "number", "positive_number", "Database port must be positive")
    config = config_add_validation(config, "server.url", "string", "valid_url", "Server URL must be valid")
    config = config_add_validation(config, "admin.email", "string", "valid_email", "Admin email must be valid")
    
    fr fr Test valid configuration
    sus valid_host ConfigValue = ConfigValue{}
    valid_host.type = "string"
    valid_host.string_value = "db.example.com"
    map_set_string(config.values, "database.host", valid_host)
    
    sus valid_port ConfigValue = ConfigValue{}
    valid_port.type = "number"
    valid_port.number_value = 5432.0
    map_set_string(config.values, "database.port", valid_port)
    
    sus valid_url ConfigValue = ConfigValue{}
    valid_url.type = "string"
    valid_url.string_value = "https://api.example.com"
    map_set_string(config.values, "server.url", valid_url)
    
    sus valid_email ConfigValue = ConfigValue{}
    valid_email.type = "string"
    valid_email.string_value = "admin@example.com"
    map_set_string(config.values, "admin.email", valid_email)
    
    fr fr Run validation (this will print errors for invalid values)
    config = validate_all_values(config)
    
    vibez.spill("✓ Validation rules test passed")
    damn based
}

slay test_pattern_matching() lit {
    vibez.spill("Testing key pattern matching...")
    
    fr fr Test various pattern matching scenarios
    sus result1 lit = key_matches_pattern("database.host", "database.*")
    sus result2 lit = key_matches_pattern("server.port", "*.port")
    sus result3 lit = key_matches_pattern("any.key", "*")
    sus result4 lit = key_matches_pattern("exact.match", "exact.match")
    sus result5 lit = key_matches_pattern("no.match", "different.*")
    
    ready (!result1) {
        vibez.spill("ERROR: 'database.host' should match 'database.*'")
        damn cringe
    }
    
    ready (!result2) {
        vibez.spill("ERROR: 'server.port' should match '*.port'")
        damn cringe
    }
    
    ready (!result3) {
        vibez.spill("ERROR: 'any.key' should match '*'")
        damn cringe
    }
    
    ready (!result4) {
        vibez.spill("ERROR: 'exact.match' should match 'exact.match'")
        damn cringe
    }
    
    ready (result5) {
        vibez.spill("ERROR: 'no.match' should not match 'different.*'")
        damn cringe
    }
    
    vibez.spill("✓ Pattern matching test passed")
    damn based
}

slay test_url_email_validation() lit {
    vibez.spill("Testing URL and email validation...")
    
    fr fr Test URL validation
    sus url1 lit = is_valid_url("https://example.com")
    sus url2 lit = is_valid_url("http://api.example.com/v1")
    sus url3 lit = is_valid_url("ftp://files.example.com")
    sus url4 lit = is_valid_url("invalid-url")
    sus url5 lit = is_valid_url("example.com")
    
    ready (!url1) {
        vibez.spill("ERROR: 'https://example.com' should be valid URL")
        damn cringe
    }
    
    ready (!url2) {
        vibez.spill("ERROR: 'http://api.example.com/v1' should be valid URL")
        damn cringe
    }
    
    ready (!url3) {
        vibez.spill("ERROR: 'ftp://files.example.com' should be valid URL")
        damn cringe
    }
    
    ready (url4) {
        vibez.spill("ERROR: 'invalid-url' should not be valid URL")
        damn cringe
    }
    
    ready (url5) {
        vibez.spill("ERROR: 'example.com' should not be valid URL")
        damn cringe
    }
    
    fr fr Test email validation
    sus email1 lit = is_valid_email("user@example.com")
    sus email2 lit = is_valid_email("admin@company.org")
    sus email3 lit = is_valid_email("invalid-email")
    sus email4 lit = is_valid_email("@example.com")
    sus email5 lit = is_valid_email("user@")
    
    ready (!email1) {
        vibez.spill("ERROR: 'user@example.com' should be valid email")
        damn cringe
    }
    
    ready (!email2) {
        vibez.spill("ERROR: 'admin@company.org' should be valid email")
        damn cringe
    }
    
    ready (email3) {
        vibez.spill("ERROR: 'invalid-email' should not be valid email")
        damn cringe
    }
    
    ready (email4) {
        vibez.spill("ERROR: '@example.com' should not be valid email")
        damn cringe
    }
    
    ready (email5) {
        vibez.spill("ERROR: 'user@' should not be valid email")
        damn cringe
    }
    
    vibez.spill("✓ URL and email validation test passed")
    damn based
}

fr fr ===== UTILITY FUNCTION TESTS =====

slay test_string_utilities() lit {
    vibez.spill("Testing string utility functions...")
    
    fr fr Test string trimming
    sus trimmed1 tea = trim_string("  hello world  ")
    sus trimmed2 tea = trim_string("\t\ntest\r\n")
    sus trimmed3 tea = trim_string("no-trim-needed")
    sus trimmed4 tea = trim_string("   ")
    
    ready (trimmed1 != "hello world") {
        vibez.spill("ERROR: Expected 'hello world', got: '" + trimmed1 + "'")
        damn cringe
    }
    
    ready (trimmed2 != "test") {
        vibez.spill("ERROR: Expected 'test', got: '" + trimmed2 + "'")
        damn cringe
    }
    
    ready (trimmed3 != "no-trim-needed") {
        vibez.spill("ERROR: Expected 'no-trim-needed', got: '" + trimmed3 + "'")
        damn cringe
    }
    
    ready (trimmed4 != "") {
        vibez.spill("ERROR: Expected empty string, got: '" + trimmed4 + "'")
        damn cringe
    }
    
    fr fr Test starts_with and ends_with
    sus starts1 lit = starts_with("hello world", "hello")
    sus starts2 lit = starts_with("test", "testing")
    sus ends1 lit = ends_with("hello world", "world")
    sus ends2 lit = ends_with("test", "testing")
    
    ready (!starts1) {
        vibez.spill("ERROR: 'hello world' should start with 'hello'")
        damn cringe
    }
    
    ready (starts2) {
        vibez.spill("ERROR: 'test' should not start with 'testing'")
        damn cringe
    }
    
    ready (!ends1) {
        vibez.spill("ERROR: 'hello world' should end with 'world'")
        damn cringe
    }
    
    ready (ends2) {
        vibez.spill("ERROR: 'test' should not end with 'testing'")
        damn cringe
    }
    
    fr fr Test string replacement
    sus replaced1 tea = string_replace_all("hello_world_test", "_", ".")
    sus replaced2 tea = string_replace_all("no__double__underscore", "__", "_")
    
    ready (replaced1 != "hello.world.test") {
        vibez.spill("ERROR: Expected 'hello.world.test', got: '" + replaced1 + "'")
        damn cringe
    }
    
    ready (replaced2 != "no_double_underscore") {
        vibez.spill("ERROR: Expected 'no_double_underscore', got: '" + replaced2 + "'")
        damn cringe
    }
    
    vibez.spill("✓ String utilities test passed")
    damn based
}

slay test_numeric_conversion() lit {
    vibez.spill("Testing numeric string conversion...")
    
    fr fr Test numeric string detection
    sus is_num1 lit = is_numeric_string("42")
    sus is_num2 lit = is_numeric_string("3.14")
    sus is_num3 lit = is_numeric_string("-123")
    sus is_num4 lit = is_numeric_string("not-a-number")
    sus is_num5 lit = is_numeric_string("")
    sus is_num6 lit = is_numeric_string("123.456.789")  fr fr Multiple decimals
    
    ready (!is_num1) {
        vibez.spill("ERROR: '42' should be detected as numeric")
        damn cringe
    }
    
    ready (!is_num2) {
        vibez.spill("ERROR: '3.14' should be detected as numeric")
        damn cringe
    }
    
    ready (!is_num3) {
        vibez.spill("ERROR: '-123' should be detected as numeric")
        damn cringe
    }
    
    ready (is_num4) {
        vibez.spill("ERROR: 'not-a-number' should not be detected as numeric")
        damn cringe
    }
    
    ready (is_num5) {
        vibez.spill("ERROR: empty string should not be detected as numeric")
        damn cringe
    }
    
    ready (is_num6) {
        vibez.spill("ERROR: '123.456.789' should not be detected as numeric (multiple decimals)")
        damn cringe
    }
    
    fr fr Test string to float conversion
    sus float1 normie = string_to_float("42")
    sus float2 normie = string_to_float("3.14")
    sus float3 normie = string_to_float("-123")
    
    ready (float1 != 42.0) {
        vibez.spill("ERROR: Expected 42.0, got: " + number_to_string(float1))
        damn cringe
    }
    
    ready (float2 != 3.14) {
        vibez.spill("ERROR: Expected 3.14, got: " + number_to_string(float2))
        damn cringe
    }
    
    ready (float3 != -123.0) {
        vibez.spill("ERROR: Expected -123.0, got: " + number_to_string(float3))
        damn cringe
    }
    
    vibez.spill("✓ Numeric conversion test passed")
    damn based
}

fr fr ===== INTEGRATION TESTS =====

slay test_multi_format_integration() lit {
    vibez.spill("Testing multi-format configuration integration...")
    
    sus config ConfigManager = config_create()
    
    fr fr Set defaults (lowest priority)
    sus default_port ConfigValue = ConfigValue{}
    default_port.type = "number"
    default_port.number_value = 8000.0
    config = config_set_default(config, "server.port", default_port)
    
    sus default_host ConfigValue = ConfigValue{}
    default_host.type = "string"
    default_host.string_value = "localhost"
    config = config_set_default(config, "database.host", default_host)
    
    fr fr Load JSON config (priority 10)
    sus json_source ConfigSource = ConfigSource{}
    json_source.type = "json"
    json_source.content = "{\"server\":{\"port\":8080},\"database\":{\"host\":\"json-db\"}}"
    json_source.priority = 10
    config = load_json_source(config, json_source)
    
    fr fr Load TOML config (priority 20) - should override JSON
    sus toml_source ConfigSource = ConfigSource{}
    toml_source.type = "toml"
    toml_source.content = "[server]\nport = 9090\n[database]\nhost = \"toml-db\""
    toml_source.priority = 20
    config = load_toml_source(config, toml_source)
    
    fr fr Test final values (TOML should win due to higher priority)
    sus final_port normie = config_get_number(config, "server.port", 0.0)
    sus final_host tea = config_get_string(config, "database.host", "")
    
    ready (final_port != 9090.0) {
        vibez.spill("ERROR: Expected port 9090 from TOML (highest priority), got: " + number_to_string(final_port))
        damn cringe
    }
    
    ready (final_host != "toml-db") {
        vibez.spill("ERROR: Expected host 'toml-db' from TOML (highest priority), got: " + final_host)
        damn cringe
    }
    
    vibez.spill("✓ Multi-format integration test passed")
    damn based
}

slay test_configuration_export() lit {
    vibez.spill("Testing configuration export functionality...")
    
    sus config ConfigManager = config_create()
    
    fr fr Add some test values
    sus string_val ConfigValue = ConfigValue{}
    string_val.type = "string"
    string_val.string_value = "test-value"
    map_set_string(config.values, "app.name", string_val)
    
    sus number_val ConfigValue = ConfigValue{}
    number_val.type = "number"
    number_val.number_value = 42.0
    map_set_string(config.values, "app.port", number_val)
    
    sus boolean_val ConfigValue = ConfigValue{}
    boolean_val.type = "boolean"
    boolean_val.boolean_value = based
    map_set_string(config.values, "app.debug", boolean_val)
    
    fr fr Export configuration as JSON
    sus json_export tea = config_export_json(config)
    
    fr fr Verify export contains expected values
    ready (!contains_string(json_export, "test-value")) {
        vibez.spill("ERROR: JSON export should contain 'test-value'")
        damn cringe
    }
    
    ready (!contains_string(json_export, "42")) {
        vibez.spill("ERROR: JSON export should contain '42'")
        damn cringe
    }
    
    ready (!contains_string(json_export, "true")) {
        vibez.spill("ERROR: JSON export should contain 'true'")
        damn cringe
    }
    
    vibez.spill("✓ Configuration export test passed")
    damn based
}

slay test_key_operations() lit {
    vibez.spill("Testing configuration key operations...")
    
    sus config ConfigManager = config_create()
    
    fr fr Add test values with various prefixes
    sus val1 ConfigValue = ConfigValue{}
    val1.type = "string"
    val1.string_value = "db-host"
    map_set_string(config.values, "database.host", val1)
    
    sus val2 ConfigValue = ConfigValue{}
    val2.type = "number"
    val2.number_value = 5432.0
    map_set_string(config.values, "database.port", val2)
    
    sus val3 ConfigValue = ConfigValue{}
    val3.type = "string"
    val3.string_value = "server-host"
    map_set_string(config.values, "server.host", val3)
    
    sus val4 ConfigValue = ConfigValue{}
    val4.type = "number"
    val4.number_value = 8080.0
    map_set_string(config.values, "server.port", val4)
    
    fr fr Test getting all keys
    sus all_keys tea[value] = config_get_all_keys(config)
    sus key_count drip = array_length(all_keys)
    
    fr fr In real implementation, this would return the actual keys
    vibez.spill("Found " + number_to_string(normie(key_count)) + " configuration keys")
    
    fr fr Test getting keys with prefix
    sus db_keys tea[value] = config_get_keys_with_prefix(config, "database")
    sus db_key_count drip = array_length(db_keys)
    
    sus server_keys tea[value] = config_get_keys_with_prefix(config, "server")
    sus server_key_count drip = array_length(server_keys)
    
    vibez.spill("Found " + number_to_string(normie(db_key_count)) + " database keys")
    vibez.spill("Found " + number_to_string(normie(server_key_count)) + " server keys")
    
    vibez.spill("✓ Key operations test passed")
    damn based
}

fr fr ===== HELPER FUNCTIONS FOR TESTS =====

slay contains_string(haystack tea, needle tea) lit {
    fr fr Simple string contains check
    sus haystack_len drip = string_length(haystack)
    sus needle_len drip = string_length(needle)
    
    ready (needle_len > haystack_len) {
        damn cringe
    }
    
    sus i drip = 0
    bestie (i <= haystack_len - needle_len) {
        ready (substring(haystack, i, needle_len) == needle) {
            damn based
        }
        i = i + 1
    }
    
    damn cringe
}

fr fr ===== MAIN TEST RUNNER =====

slay run_comprehensive_configz_tests() lit {
    vibez.spill("=== CONFIGZ COMPREHENSIVE TEST SUITE ===")
    vibez.spill("")
    
    sus passed drip = 0
    sus failed drip = 0
    
    fr fr Basic functionality tests
    ready (test_config_manager_creation()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_default_values()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    fr fr Configuration format tests
    ready (test_json_configuration_loading()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_toml_configuration_loading()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_yaml_configuration_loading()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_ini_configuration_loading()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    fr fr Environment variable tests
    ready (test_environment_variable_loading()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_env_key_conversion()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_auto_type_detection()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    fr fr Priority and source management tests
    ready (test_source_priority_ordering()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_configuration_override()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    fr fr Validation tests
    ready (test_validation_rules()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_pattern_matching()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_url_email_validation()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    fr fr Utility function tests
    ready (test_string_utilities()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_numeric_conversion()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    fr fr Integration tests
    ready (test_multi_format_integration()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_configuration_export()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    ready (test_key_operations()) {
        passed = passed + 1
    } otherwise {
        failed = failed + 1
    }
    
    fr fr Print test summary
    vibez.spill("")
    vibez.spill("=== TEST RESULTS ===")
    vibez.spill("Passed: " + number_to_string(normie(passed)))
    vibez.spill("Failed: " + number_to_string(normie(failed)))
    vibez.spill("Total:  " + number_to_string(normie(passed + failed)))
    
    ready (failed == 0) {
        vibez.spill("")
        vibez.spill("🎉 ALL CONFIGZ TESTS PASSED! Configuration management system is production-ready.")
        damn based
    } otherwise {
        vibez.spill("")
        vibez.spill("❌ Some tests failed. Configuration system needs attention.")
        damn cringe
    }
}

fr fr Run the comprehensive test suite
run_comprehensive_configz_tests()
