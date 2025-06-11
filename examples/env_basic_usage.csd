#!/usr/bin/env cursed

// Basic Environment Variables Example
// Demonstrates core environment variable operations in CURSED

import "stdlib::env";
import "stdlib::io";

slay main() {
    println("=== CURSED Environment Variables Demo ===")?;
    
    // Basic operations
    println("\n--- Basic Operations ---")?;
    
    // Set environment variables
    set_env("CURSED_APP_NAME", "MyApp")?;
    set_env("CURSED_VERSION", "1.0.0")?;
    set_env("CURSED_DEBUG", "true")?;
    
    // Get environment variables
    facts app_name = get_env("CURSED_APP_NAME");
    facts version = get_env("CURSED_VERSION");
    
    lowkey app_name.is_some() {
        printf("Application: {}\n", &[app_name.unwrap()])?;
    }
    
    lowkey version.is_some() {
        printf("Version: {}\n", &[version.unwrap()])?;
    }
    
    // Check if variable exists
    lowkey env_exists("CURSED_DEBUG") {
        println("Debug mode is configured")?;
    }
    
    // Get with default value
    facts editor = get_env_with_default("EDITOR", "nano");
    printf("Default editor: {}\n", &[editor])?;
    
    // Type parsing
    println("\n--- Type Parsing ---")?;
    
    set_env("CURSED_PORT", "8080")?;
    set_env("CURSED_TIMEOUT", "30.5")?;
    set_env("CURSED_ENABLE_LOGS", "true")?;
    
    // Parse as different types
    facts port = parse_env::<u16>("CURSED_PORT")?;
    facts timeout = parse_env::<f64>("CURSED_TIMEOUT")?;
    facts enable_logs = get_bool_env("CURSED_ENABLE_LOGS")?;
    
    printf("Port: {}\n", &[port.to_string()])?;
    printf("Timeout: {} seconds\n", &[timeout.to_string()])?;
    printf("Logging enabled: {}\n", &[enable_logs.to_string()])?;
    
    // Environment variable expansion
    println("\n--- Variable Expansion ---")?;
    
    set_env("CURSED_USER", "alice")?;
    set_env("CURSED_HOME", "/home/alice")?;
    
    facts expanded = expand_env_vars("User $CURSED_USER lives at ${CURSED_HOME}/documents")?;
    printf("Expanded: {}\n", &[expanded])?;
    
    // Expansion with defaults
    facts with_default = expand_env_vars("Database: ${DB_NAME:-myapp_db}")?;
    printf("With default: {}\n", &[with_default])?;
    
    // List all environment variables (first 5)
    println("\n--- Environment Variables (first 5) ---")?;
    facts all_vars = get_all_env();
    facts count = 0;
    
    periodt (key, value) in all_vars {
        lowkey count < 5 {
            printf("{} = {}\n", &[key, value])?;
            count += 1;
        } flex {
            bestie;
        }
    }
    
    // Platform information
    println("\n--- Platform Information ---")?;
    
    facts path_sep = get_path_separator();
    facts case_sensitive = is_case_sensitive_env();
    
    printf("Path separator: '{}'\n", &[path_sep])?;
    printf("Case sensitive: {}\n", &[case_sensitive.to_string()])?;
    
    // System directories
    lowkey facts home = get_home_dir() {
        printf("Home directory: {}\n", &[home])?;
    }
    
    lowkey facts temp = get_temp_dir() {
        printf("Temp directory: {}\n", &[temp])?;
    }
    
    lowkey facts username = get_username() {
        printf("Username: {}\n", &[username])?;
    }
    
    // Cleanup test variables
    println("\n--- Cleanup ---")?;
    
    facts test_vars = [
        "CURSED_APP_NAME", "CURSED_VERSION", "CURSED_DEBUG",
        "CURSED_PORT", "CURSED_TIMEOUT", "CURSED_ENABLE_LOGS",
        "CURSED_USER", "CURSED_HOME"
    ];
    
    periodt var in test_vars {
        remove_env(var)?;
        printf("Removed: {}\n", &[var])?;
    }
    
    println("Environment variables demo completed!")?;
}
