// CURSED Configuration Management Library Tests
// Comprehensive test suite for config module

yeet "config"
yeet "string" 
yeet "collections"

// ================================
// Test Framework (Simplified)
// ================================

sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

slay test_start(name tea) {
    test_count = test_count + 1
    vibez.spill("Running test: " + name)
}

slay test_pass(message tea) {
    test_passed = test_passed + 1
    vibez.spill("  ✓ PASS: " + message)
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ✗ FAIL: " + message)
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } else {
        test_fail("assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        test_pass("assert_true: value is based")
    } else {
        test_fail("assert_true failed: got " + string_from_bool(value) + ", expected based")
    }
}

slay assert_false(value lit) {
    lowkey value == cap {
        test_pass("assert_false: value is cap")
    } else {
        test_fail("assert_false failed: got " + string_from_bool(value) + ", expected cap")
    }
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + string_from_int(test_count))
    vibez.spill("Passed: " + string_from_int(test_passed))
    vibez.spill("Failed: " + string_from_int(test_failed))
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } else {
        vibez.spill("❌ Some tests failed")
    }
}

// ================================
// Test Utilities
// ================================

// Create sample INI content for testing
slay create_sample_ini() tea {
    damn "# Sample INI configuration\n" +
         "global_key=global_value\n" +
         "debug=true\n" +
         "\n" +
         "[database]\n" +
         "host=localhost\n" +
         "port=5432\n" +
         "name=myapp\n" +
         "username=admin\n" +
         "password=\"secret123\"\n" +
         "\n" +
         "[server]\n" +
         "host=0.0.0.0\n" +
         "port=8080\n" +
         "workers=4\n" +
         "timeout=30\n" +
         "\n" +
         "; Comment with semicolon\n" +
         "[logging]\n" +
         "level=info\n" +
         "file=/var/log/app.log\n" +
         "rotate=daily\n";
}

// Create sample environment content for testing
slay create_sample_env() tea {
    damn "# Environment variables\n" +
         "NODE_ENV=production\n" +
         "DEBUG=false\n" +
         "export DATABASE_URL=postgres://localhost:5432/myapp\n" +
         "API_KEY=\"sk-1234567890abcdef\"\n" +
         "REDIS_URL='redis://localhost:6379'\n" +
         "WORKERS=8\n" +
         "TIMEOUT=60\n" +
         "\n" +
         "# Web server configuration\n" +
         "WEB_HOST=0.0.0.0\n" +
         "WEB_PORT=3000\n" +
         "WEB_CORS_ORIGIN=*\n";
}

// ================================
// INI Format Tests
// ================================

slay test_ini_basic_parsing() {
    test_start("INI Basic Parsing");
    
    sus ini_content tea = create_sample_ini();
    sus config map = parse_ini(ini_content);
    
    // Test global keys
    assert_eq_string(get_value(config, "global_key"), "global_value");
    assert_eq_string(get_value(config, "debug"), "true");
    
    // Test section keys
    assert_eq_string(get_value(config, "database.host"), "localhost");
    assert_eq_string(get_value(config, "database.port"), "5432");
    assert_eq_string(get_value(config, "database.name"), "myapp");
    assert_eq_string(get_value(config, "database.username"), "admin");
    assert_eq_string(get_value(config, "database.password"), "secret123");
    
    // Test server section
    assert_eq_string(get_value(config, "server.host"), "0.0.0.0");
    assert_eq_string(get_value(config, "server.port"), "8080");
    assert_eq_string(get_value(config, "server.workers"), "4");
    assert_eq_string(get_value(config, "server.timeout"), "30");
    
    // Test logging section
    assert_eq_string(get_value(config, "logging.level"), "info");
    assert_eq_string(get_value(config, "logging.file"), "/var/log/app.log");
    assert_eq_string(get_value(config, "logging.rotate"), "daily");
}

slay test_ini_edge_cases() {
    test_start("INI Edge Cases");
    
    sus edge_content tea = "# Comment only file\n" +
                          "\n" +
                          "[empty_section]\n" +
                          "\n" +
                          "[section_with_spaces]\n" +
                          "key_with_spaces = value with spaces\n" +
                          "quoted_value=\"quoted string\"\n" +
                          "unquoted_value=simple\n" +
                          "\n" +
                          "# Global after section\n" +
                          "global_after=test\n";
    
    sus config map = parse_ini(edge_content);
    
    assert_eq_string(get_value(config, "section_with_spaces.key_with_spaces"), "value with spaces");
    assert_eq_string(get_value(config, "section_with_spaces.quoted_value"), "quoted string");
    assert_eq_string(get_value(config, "section_with_spaces.unquoted_value"), "simple");
    assert_eq_string(get_value(config, "global_after"), "test");
}

slay test_ini_stringify() {
    test_start("INI Stringify");
    
    sus config map = map_create();
    config = set_value(config, "global", "value");
    config = set_value(config, "database.host", "localhost");
    config = set_value(config, "database.port", "5432");
    config = set_value(config, "server.host", "0.0.0.0");
    config = set_value(config, "server.port", "8080");
    
    sus ini_output tea = stringify_ini(config);
    
    // Parse it back to verify
    sus parsed_config map = parse_ini(ini_output);
    
    assert_eq_string(get_value(parsed_config, "global"), "value");
    assert_eq_string(get_value(parsed_config, "database.host"), "localhost");
    assert_eq_string(get_value(parsed_config, "database.port"), "5432");
    assert_eq_string(get_value(parsed_config, "server.host"), "0.0.0.0");
    assert_eq_string(get_value(parsed_config, "server.port"), "8080");
}

// ================================
// Environment Variable Tests
// ================================

slay test_env_parsing() {
    test_start("Environment Variable Parsing");
    
    sus env_content tea = create_sample_env();
    sus config map = parse_env(env_content);
    
    assert_eq_string(get_value(config, "NODE_ENV"), "production");
    assert_eq_string(get_value(config, "DEBUG"), "false");
    assert_eq_string(get_value(config, "DATABASE_URL"), "postgres://localhost:5432/myapp");
    assert_eq_string(get_value(config, "API_KEY"), "sk-1234567890abcdef");
    assert_eq_string(get_value(config, "REDIS_URL"), "redis://localhost:6379");
    assert_eq_string(get_value(config, "WORKERS"), "8");
    assert_eq_string(get_value(config, "TIMEOUT"), "60");
    assert_eq_string(get_value(config, "WEB_HOST"), "0.0.0.0");
    assert_eq_string(get_value(config, "WEB_PORT"), "3000");
    assert_eq_string(get_value(config, "WEB_CORS_ORIGIN"), "*");
}

slay test_env_stringify() {
    test_start("Environment Stringify");
    
    sus config map = map_create();
    config = set_value(config, "database.host", "localhost");
    config = set_value(config, "database.port", "5432");
    config = set_value(config, "server.host", "0.0.0.0");
    config = set_value(config, "app.name", "My App");
    
    sus env_output tea = stringify_env(config);
    
    // Check that it contains expected environment variables
    assert_true(string_contains(env_output, "DATABASE_HOST=localhost"));
    assert_true(string_contains(env_output, "DATABASE_PORT=5432"));
    assert_true(string_contains(env_output, "SERVER_HOST=0.0.0.0"));
    assert_true(string_contains(env_output, "APP_NAME=\"My App\""));
}

// ================================
// Configuration Access Tests
// ================================

slay test_config_access() {
    test_start("Configuration Access Functions");
    
    sus config map = map_create();
    config = set_value(config, "database.host", "localhost");
    config = set_value(config, "database.port", "5432");
    config = set_value(config, "server.debug", "true");
    config = set_value(config, "app.name", "TestApp");
    
    // Test get_value
    assert_eq_string(get_value(config, "database.host"), "localhost");
    assert_eq_string(get_value(config, "database.port"), "5432");
    assert_eq_string(get_value(config, "nonexistent"), "");
    
    // Test has_key
    assert_true(has_key(config, "database.host"));
    assert_true(has_key(config, "server.debug"));
    assert_false(has_key(config, "nonexistent.key"));
    
    // Test get_default
    assert_eq_string(get_default(config, "database.host", "default"), "localhost");
    assert_eq_string(get_default(config, "nonexistent", "default"), "default");
    
    // Test set_value
    sus updated_config map = set_value(config, "new.key", "new_value");
    assert_eq_string(get_value(updated_config, "new.key"), "new_value");
}

slay test_section_access() {
    test_start("Section Access");
    
    sus config map = map_create();
    config = set_value(config, "database.host", "localhost");
    config = set_value(config, "database.port", "5432");
    config = set_value(config, "database.name", "myapp");
    config = set_value(config, "server.host", "0.0.0.0");
    config = set_value(config, "server.port", "8080");
    
    // Test get_section
    sus db_section map = get_section(config, "database");
    assert_eq_string(get_value(db_section, "host"), "localhost");
    assert_eq_string(get_value(db_section, "port"), "5432");
    assert_eq_string(get_value(db_section, "name"), "myapp");
    assert_false(has_key(db_section, "server.host"));
    
    sus server_section map = get_section(config, "server");
    assert_eq_string(get_value(server_section, "host"), "0.0.0.0");
    assert_eq_string(get_value(server_section, "port"), "8080");
    assert_false(has_key(server_section, "database.host"));
}

// ================================
// Configuration Merging Tests
// ================================

slay test_config_merging() {
    test_start("Configuration Merging");
    
    sus base_config map = map_create();
    base_config = set_value(base_config, "database.host", "localhost");
    base_config = set_value(base_config, "database.port", "5432");
    base_config = set_value(base_config, "server.host", "0.0.0.0");
    base_config = set_value(base_config, "server.port", "8080");
    
    sus override_config map = map_create();
    override_config = set_value(override_config, "database.host", "db.example.com");
    override_config = set_value(override_config, "server.port", "9090");
    override_config = set_value(override_config, "new.key", "new_value");
    
    sus merged_config map = merge_configs(base_config, override_config);
    
    // Check overridden values
    assert_eq_string(get_value(merged_config, "database.host"), "db.example.com");
    assert_eq_string(get_value(merged_config, "server.port"), "9090");
    
    // Check preserved values
    assert_eq_string(get_value(merged_config, "database.port"), "5432");
    assert_eq_string(get_value(merged_config, "server.host"), "0.0.0.0");
    
    // Check new values
    assert_eq_string(get_value(merged_config, "new.key"), "new_value");
}

// ================================
// Schema Validation Tests
// ================================

slay test_schema_validation() {
    test_start("Schema Validation");
    
    sus config map = map_create();
    config = set_value(config, "database.host", "localhost");
    config = set_value(config, "database.port", "5432");
    config = set_value(config, "server.host", "0.0.0.0");
    
    // Create schema with required keys
    sus schema map = map_create();
    sus required_keys [tea] = ["database.host", "database.port", "server.host"];
    schema = map_set(schema, "required", required_keys);
    
    // Test valid configuration
    assert_true(validate_schema(config, schema));
    
    // Test invalid configuration (missing required key)
    sus invalid_config map = map_create();
    invalid_config = set_value(invalid_config, "database.host", "localhost");
    // Missing database.port and server.host
    
    assert_false(validate_schema(invalid_config, schema));
}

// ================================
// Type Conversion Tests
// ================================

slay test_type_conversion() {
    test_start("Type Conversion");
    
    sus config map = map_create();
    config = set_value(config, "debug", "true");
    config = set_value(config, "enabled", "false");
    config = set_value(config, "port", "8080");
    config = set_value(config, "timeout", "30.5");
    config = set_value(config, "name", "MyApp");
    
    // Test boolean conversion
    assert_true(get_bool_value(config, "debug", cap));
    assert_false(get_bool_value(config, "enabled", based));
    assert_false(get_bool_value(config, "nonexistent", cap));
    
    // Test integer conversion - using string comparison for now
    sus port_val normie = get_int_value(config, "port", 0);
    assert_eq_string(string_from_int(port_val), "8080");
    
    sus default_val normie = get_int_value(config, "nonexistent", 9999);
    assert_eq_string(string_from_int(default_val), "9999");
}

// ================================
// Edge Cases Tests
// ================================

slay test_empty_config() {
    test_start("Empty Configuration");
    
    sus empty_ini tea = "";
    sus config map = parse_ini(empty_ini);
    
    assert_false(has_key(config, "any_key"));
    assert_eq_string(get_value(config, "any_key"), "");
    assert_eq_string(get_default(config, "any_key", "default"), "default");
}

slay test_malformed_config() {
    test_start("Malformed Configuration");
    
    sus malformed_ini tea = "[section_without_closing_bracket\n" +
                           "key_without_equals\n" +
                           "=value_without_key\n" +
                           "[valid_section]\n" +
                           "valid_key=valid_value\n";
    
    sus config map = parse_ini(malformed_ini);
    
    // Should still parse valid parts
    assert_eq_string(get_value(config, "valid_section.valid_key"), "valid_value");
}

// ================================
// Main Test Runner
// ================================

slay main() {
    vibez.spill("Running CURSED Configuration Management Tests...");
    
    // INI format tests
    test_ini_basic_parsing();
    test_ini_edge_cases();
    test_ini_stringify();
    
    // Environment variable tests
    test_env_parsing();
    test_env_stringify();
    
    // Configuration access tests
    test_config_access();
    test_section_access();
    
    // Configuration merging tests
    test_config_merging();
    
    // Schema validation tests
    test_schema_validation();
    
    // Type conversion tests
    test_type_conversion();
    
    // Edge cases
    test_empty_config();
    test_malformed_config();
    
    print_test_summary();
    
    vibez.spill("Configuration Management Tests Complete!");
}
