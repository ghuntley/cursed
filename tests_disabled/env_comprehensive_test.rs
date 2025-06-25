/// Comprehensive test suite for the CURSED environment variables module

use std::collections::HashMap;
use std::time::Duration;

// Import the environment module functions
use cursed::stdlib::env::{
    // Core operations
    get_env, set_env, remove_env, get_all_env, env_exists,
    get_env_with_default, clear_all_env, get_env_keys, get_env_values,
    get_current_dir, get_home_dir, get_temp_dir, get_username, get_hostname,
    
    // Platform utilities
    get_path_separator, is_case_sensitive_env, get_env_case_insensitive,
    
    // Parsing functions
    parse_env, parse_env_with_default, get_path_env, get_numeric_env,
    get_bool_env, get_float_env, get_int_env, parse_env_list,
    parse_env_colon_list, parse_env_semicolon_list, parse_env_path_list,
    parse_env_config, parse_env_duration, parse_env_memory_size,
    
    // Expansion functions
    expand_env_vars, expand_env_vars_with_defaults, has_env_vars,
    validate_env_syntax, extract_env_vars, substitute_env_vars,
    escape_env_value, unescape_env_value,
    
    // Error types
    EnvError, EnvResult,
};

#[test]
fn test_basic_env_operations() {
    let test_key = "CURSED_TEST_BASIC";
    let test_value = "test_value_123";
    
    // Test setting and getting
    assert!(set_env(test_key, test_value).is_ok());
    assert_eq!(get_env(test_key), Some(test_value.to_string()));
    assert!(env_exists(test_key));
    
    // Test with default
    assert_eq!(get_env_with_default(test_key, "default"), test_value);
    
    // Test removal
    assert!(remove_env(test_key).is_ok());
    assert_eq!(get_env(test_key), None);
    assert!(!env_exists(test_key));
    
    // Test default after removal
    assert_eq!(get_env_with_default(test_key, "default"), "default");
}

#[test]
fn test_get_all_env_operations() {
    let test_key1 = "CURSED_TEST_ALL_1";
    let test_key2 = "CURSED_TEST_ALL_2";
    
    set_env(test_key1, "value1").unwrap();
    set_env(test_key2, "value2").unwrap();
    
    let all_env = get_all_env();
    assert!(all_env.contains_key(test_key1));
    assert!(all_env.contains_key(test_key2));
    
    let keys = get_env_keys();
    let values = get_env_values();
    assert!(keys.contains(&test_key1.to_string()));
    assert!(values.contains(&"value1".to_string()));
    
    remove_env(test_key1).unwrap();
    remove_env(test_key2).unwrap();
}

#[test]
fn test_type_parsing() {
    let int_key = "CURSED_TEST_INT";
    let float_key = "CURSED_TEST_FLOAT";
    let bool_key = "CURSED_TEST_BOOL";
    
    // Test integer parsing
    set_env(int_key, "42").unwrap();
    assert_eq!(parse_env::<i32>(int_key).unwrap(), 42);
    assert_eq!(get_int_env(int_key).unwrap(), 42);
    
    // Test float parsing
    set_env(float_key, "3.14159").unwrap();
    assert_eq!(parse_env::<f64>(float_key).unwrap(), 3.14159);
    assert_eq!(get_float_env(float_key).unwrap(), 3.14159);
    
    // Test boolean parsing
    set_env(bool_key, "true").unwrap();
    assert_eq!(get_bool_env(bool_key).unwrap(), true);
    
    set_env(bool_key, "false").unwrap();
    assert_eq!(get_bool_env(bool_key).unwrap(), false);
    
    set_env(bool_key, "1").unwrap();
    assert_eq!(get_bool_env(bool_key).unwrap(), true);
    
    set_env(bool_key, "0").unwrap();
    assert_eq!(get_bool_env(bool_key).unwrap(), false);
    
    // Test with defaults
    remove_env(int_key).unwrap();
    assert_eq!(parse_env_with_default(int_key, 100).unwrap(), 100);
    
    // Cleanup
    remove_env(float_key).unwrap();
    remove_env(bool_key).unwrap();
}

#[test]
fn test_numeric_bounds_checking() {
    let key = "CURSED_TEST_BOUNDS";
    
    // Test valid range
    set_env(key, "50").unwrap();
    assert_eq!(get_numeric_env::<i32>(key, 1, 100).unwrap(), 50);
    
    // Test out of range
    set_env(key, "150").unwrap();
    assert!(get_numeric_env::<i32>(key, 1, 100).is_err());
    
    set_env(key, "-10").unwrap();
    assert!(get_numeric_env::<i32>(key, 1, 100).is_err());
    
    remove_env(key).unwrap();
}

#[test]
fn test_list_parsing() {
    let key = "CURSED_TEST_LIST";
    
    // Test comma-separated list
    set_env(key, "item1,item2, item3 , item4").unwrap();
    let list = parse_env_list(key).unwrap();
    assert_eq!(list, vec!["item1", "item2", "item3", "item4"]);
    
    // Test colon-separated list
    set_env(key, "path1:path2: path3 :path4").unwrap();
    let colon_list = parse_env_colon_list(key).unwrap();
    assert_eq!(colon_list, vec!["path1", "path2", "path3", "path4"]);
    
    // Test semicolon-separated list
    set_env(key, "win1;win2; win3 ;win4").unwrap();
    let semi_list = parse_env_semicolon_list(key).unwrap();
    assert_eq!(semi_list, vec!["win1", "win2", "win3", "win4"]);
    
    remove_env(key).unwrap();
}

#[test]
fn test_config_parsing() {
    let key = "CURSED_TEST_CONFIG";
    
    set_env(key, "host=localhost,port=5432,user=admin,debug=true").unwrap();
    let config = parse_env_config(key, ",").unwrap();
    
    assert_eq!(config.get("host"), Some(&"localhost".to_string()));
    assert_eq!(config.get("port"), Some(&"5432".to_string()));
    assert_eq!(config.get("user"), Some(&"admin".to_string()));
    assert_eq!(config.get("debug"), Some(&"true".to_string()));
    
    remove_env(key).unwrap();
}

#[test]
fn test_duration_parsing() {
    let key = "CURSED_TEST_DURATION";
    
    // Test seconds
    set_env(key, "30s").unwrap();
    assert_eq!(parse_env_duration(key).unwrap(), Duration::from_secs(30));
    
    // Test minutes
    set_env(key, "5m").unwrap();
    assert_eq!(parse_env_duration(key).unwrap(), Duration::from_secs(300));
    
    // Test hours
    set_env(key, "2h").unwrap();
    assert_eq!(parse_env_duration(key).unwrap(), Duration::from_secs(7200));
    
    // Test days
    set_env(key, "1d").unwrap();
    assert_eq!(parse_env_duration(key).unwrap(), Duration::from_secs(86400));
    
    // Test default seconds (no suffix)
    set_env(key, "45").unwrap();
    assert_eq!(parse_env_duration(key).unwrap(), Duration::from_secs(45));
    
    remove_env(key).unwrap();
}

#[test]
fn test_memory_size_parsing() {
    let key = "CURSED_TEST_MEMORY";
    
    // Test bytes
    set_env(key, "1024B").unwrap();
    assert_eq!(parse_env_memory_size(key).unwrap(), 1024);
    
    // Test kilobytes
    set_env(key, "1KB").unwrap();
    assert_eq!(parse_env_memory_size(key).unwrap(), 1024);
    
    // Test megabytes
    set_env(key, "1MB").unwrap();
    assert_eq!(parse_env_memory_size(key).unwrap(), 1024 * 1024);
    
    // Test gigabytes
    set_env(key, "1GB").unwrap();
    assert_eq!(parse_env_memory_size(key).unwrap(), 1024 * 1024 * 1024);
    
    // Test default bytes (no suffix)
    set_env(key, "2048").unwrap();
    assert_eq!(parse_env_memory_size(key).unwrap(), 2048);
    
    remove_env(key).unwrap();
}

#[test]
fn test_environment_expansion() {
    let user_key = "CURSED_TEST_USER";
    let home_key = "CURSED_TEST_HOME";
    
    set_env(user_key, "alice").unwrap();
    set_env(home_key, "/home/alice").unwrap();
    
    // Test simple expansion
    let result = expand_env_vars("User: $CURSED_TEST_USER").unwrap();
    assert_eq!(result, "User: alice");
    
    // Test braced expansion
    let result = expand_env_vars("Home: ${CURSED_TEST_HOME}/documents").unwrap();
    assert_eq!(result, "Home: /home/alice/documents");
    
    // Test mixed expansion
    let result = expand_env_vars("$CURSED_TEST_USER lives at ${CURSED_TEST_HOME}").unwrap();
    assert_eq!(result, "alice lives at /home/alice");
    
    // Test expansion with defaults
    let result = expand_env_vars("${NONEXISTENT:-default_value}").unwrap();
    assert_eq!(result, "default_value");
    
    // Test conditional expansion
    let result = expand_env_vars("${CURSED_TEST_USER:+User is set}").unwrap();
    assert_eq!(result, "User is set");
    
    let result = expand_env_vars("${NONEXISTENT:+Should be empty}").unwrap();
    assert_eq!(result, "");
    
    remove_env(user_key).unwrap();
    remove_env(home_key).unwrap();
}

#[test]
fn test_expansion_with_custom_defaults() {
    let mut defaults = HashMap::new();
    defaults.insert("CUSTOM_VAR".to_string(), "custom_value".to_string());
    defaults.insert("ANOTHER_VAR".to_string(), "another_value".to_string());
    
    let result = expand_env_vars_with_defaults(
        "${CUSTOM_VAR} and ${ANOTHER_VAR}", 
        &defaults
    ).unwrap();
    assert_eq!(result, "custom_value and another_value");
}

#[test]
fn test_expansion_utilities() {
    // Test has_env_vars
    assert!(has_env_vars("$HOME/documents"));
    assert!(has_env_vars("${USER} is here"));
    assert!(!has_env_vars("No variables here"));
    
    // Test validate_env_syntax
    assert!(validate_env_syntax("${HOME}/docs").is_ok());
    assert!(validate_env_syntax("$USER works here").is_ok());
    assert!(validate_env_syntax("${VAR:-default}").is_ok());
    assert!(validate_env_syntax("${VAR:+alternative}").is_ok());
    
    // Test invalid syntax
    assert!(validate_env_syntax("${UNCLOSED").is_err());
    assert!(validate_env_syntax("${}").is_err());
    
    // Test extract_env_vars
    let vars = extract_env_vars("${USER} lives in $HOME/documents").unwrap();
    assert!(vars.contains(&"USER".to_string()));
    assert!(vars.contains(&"HOME".to_string()));
}

#[test]
fn test_variable_substitution() {
    let mut replacements = HashMap::new();
    replacements.insert("USER".to_string(), "bob".to_string());
    replacements.insert("HOME".to_string(), "/home/bob".to_string());
    replacements.insert("SHELL".to_string(), "/bin/bash".to_string());
    
    let result = substitute_env_vars(
        "$USER uses ${SHELL} from ${HOME}", 
        &replacements
    ).unwrap();
    assert_eq!(result, "bob uses /bin/bash from /home/bob");
}

#[test]
fn test_escape_unescape() {
    let original = "Value with $ and {} special chars";
    let escaped = escape_env_value(original);
    let unescaped = unescape_env_value(&escaped).unwrap();
    assert_eq!(original, unescaped);
    
    // Test specific escaping
    assert_eq!(escape_env_value("$HOME"), "\\$HOME");
    assert_eq!(escape_env_value("${VAR}"), "\\$\\{VAR\\}");
    assert_eq!(unescape_env_value("\\$HOME").unwrap(), "$HOME");
}

#[test]
fn test_platform_utilities() {
    // Test path separator
    let separator = get_path_separator();
    #[cfg(target_os = "windows")]
    assert_eq!(separator, ";");
    #[cfg(not(target_os = "windows"))]
    assert_eq!(separator, ":");
    
    // Test case sensitivity
    #[cfg(target_os = "windows")]
    assert!(!is_case_sensitive_env());
    #[cfg(not(target_os = "windows"))]
    assert!(is_case_sensitive_env());
}

#[test]
fn test_system_directories() {
    // These tests check that we can get system directories
    // The actual values depend on the system, so we just check they exist
    
    // Current directory should always be available
    assert!(get_current_dir().is_some());
    
    // These may or may not be available depending on the environment
    // We just test that the functions don't panic
    let _ = get_home_dir();
    let _ = get_temp_dir();
    let _ = get_username();
    let _ = get_hostname();
}

#[test]
fn test_path_environment_parsing() {
    let key = "CURSED_TEST_PATH";
    let separator = get_path_separator();
    
    // Create a test PATH-like variable
    let test_paths = format!("/usr/bin{}/bin{}/usr/local/bin", separator, separator);
    set_env(key, &test_paths).unwrap();
    
    if let Some(paths) = get_path_env(key) {
        assert_eq!(paths.len(), 3);
        assert_eq!(paths[0].to_string_lossy(), "/usr/bin");
        assert_eq!(paths[1].to_string_lossy(), "/bin");
        assert_eq!(paths[2].to_string_lossy(), "/usr/local/bin");
    }
    
    // Test platform-specific path list parsing
    let path_list = parse_env_path_list(key).unwrap();
    assert_eq!(path_list, vec!["/usr/bin", "/bin", "/usr/local/bin"]);
    
    remove_env(key).unwrap();
}

#[test]
fn test_error_handling() {
    // Test nonexistent variable
    assert!(parse_env::<i32>("CURSED_NONEXISTENT_VAR").is_err());
    
    // Test invalid type conversion
    set_env("CURSED_INVALID_INT", "not_a_number").unwrap();
    assert!(parse_env::<i32>("CURSED_INVALID_INT").is_err());
    
    // Test invalid boolean
    set_env("CURSED_INVALID_BOOL", "maybe").unwrap();
    assert!(get_bool_env("CURSED_INVALID_BOOL").is_err());
    
    // Test invalid key
    assert!(set_env("", "value").is_err());
    assert!(set_env("key\0with\0nulls", "value").is_err());
    
    // Test invalid value
    assert!(set_env("VALID_KEY", "value\0with\0nulls").is_err());
    
    // Cleanup
    remove_env("CURSED_INVALID_INT").unwrap();
    remove_env("CURSED_INVALID_BOOL").unwrap();
}

#[test] 
fn test_case_insensitive_lookup() {
    let key = "CURSED_CASE_TEST";
    set_env(key, "test_value").unwrap();
    
    // Test exact match
    assert_eq!(get_env_case_insensitive(key), Some("test_value".to_string()));
    
    // Test case insensitive lookup behavior
    #[cfg(target_os = "windows")]
    {
        // On Windows, should find with different case
        assert_eq!(get_env_case_insensitive("cursed_case_test"), Some("test_value".to_string()));
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // On Unix, should be case sensitive
        assert_eq!(get_env_case_insensitive("cursed_case_test"), None);
    }
    
    remove_env(key).unwrap();
}

#[test]
fn test_complex_expansion_scenarios() {
    set_env("CURSED_PREFIX", "app").unwrap();
    set_env("CURSED_VERSION", "1.0.0").unwrap();
    
    // Test complex expansion with multiple variables and modifiers
    let complex = "${CURSED_PREFIX:-default}_v${CURSED_VERSION:+1.0.0}_${NONEXISTENT:-dev}";
    let result = expand_env_vars(complex).unwrap();
    assert_eq!(result, "app_v1.0.0_dev");
    
    // Test nested-like behavior
    let nested = "${CURSED_PREFIX}_${CURSED_VERSION}_final";
    let result = expand_env_vars(nested).unwrap();
    assert_eq!(result, "app_1.0.0_final");
    
    remove_env("CURSED_PREFIX").unwrap();
    remove_env("CURSED_VERSION").unwrap();
}

#[test]
fn test_env_vars_in_config() {
    set_env("DB_HOST", "localhost").unwrap();
    set_env("DB_PORT", "5432").unwrap();
    
    let config_template = "host=${DB_HOST},port=${DB_PORT},ssl=true";
    let expanded = expand_env_vars(config_template).unwrap();
    
    // Parse the expanded config
    set_env("CURSED_EXPANDED_CONFIG", &expanded).unwrap();
    let config = parse_env_config("CURSED_EXPANDED_CONFIG", ",").unwrap();
    
    assert_eq!(config.get("host"), Some(&"localhost".to_string()));
    assert_eq!(config.get("port"), Some(&"5432".to_string()));
    assert_eq!(config.get("ssl"), Some(&"true".to_string()));
    
    remove_env("DB_HOST").unwrap();
    remove_env("DB_PORT").unwrap();
    remove_env("CURSED_EXPANDED_CONFIG").unwrap();
}
