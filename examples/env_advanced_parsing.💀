#!/usr/bin/env cursed

fr fr Advanced Environment Variable Parsing Example
fr fr Demonstrates advanced parsing and configuration features

yeet "stdlib::env"
yeet "stdlib::io"
yeet "stdlib::collections"

slay main_character() {
    println("=== Advanced Environment Variable Parsing ===")?;
    
    // Configuration parsing
    println("\n--- Configuration Parsing ---")?;
    
    set_env("DATABASE_CONFIG", "host=localhost,port=5432,user=admin,ssl=based")?;
    
    facts db_config = parse_env_config("DATABASE_CONFIG", ",")?;
    println("Database configuration:")?;
    periodt (key, value) in db_config {
        printf("  {} = {}\n", &[key, value])?;
    }
    
    // List parsing
    println("\n--- List Parsing ---")?;
    
    set_env("ALLOWED_HOSTS", "localhost,127.0.0.1,example.com,*.dev")?;
    set_env("LIBRARY_PATH", "/usr/lib:/usr/local/lib:/opt/lib")?;
    set_env("SEARCH_PATHS", "C:\\Program Files;C:\\Windows\\System32")?;
    
    facts hosts = parse_env_list("ALLOWED_HOSTS")?;
    println("Allowed hosts:")?;
    periodt host in hosts {
        printf("  - {}\n", &[host])?;
    }
    
    facts lib_paths = parse_env_colon_list("LIBRARY_PATH")?;
    println("Library paths:")?;
    periodt path in lib_paths {
        printf("  - {}\n", &[path])?;
    }
    
    facts search_paths = parse_env_semicolon_list("SEARCH_PATHS")?;
    println("Search paths:")?;
    periodt path in search_paths {
        printf("  - {}\n", &[path])?;
    }
    
    // Duration parsing
    println("\n--- Duration Parsing ---")?;
    
    set_env("CACHE_TTL", "5m")?;
    set_env("SESSION_TIMEOUT", "2h")?;
    set_env("BACKUP_INTERVAL", "1d")?;
    
    facts cache_ttl = parse_env_duration("CACHE_TTL")?;
    facts session_timeout = parse_env_duration("SESSION_TIMEOUT")?;
    facts backup_interval = parse_env_duration("BACKUP_INTERVAL")?;
    
    printf("Cache TTL: {} seconds\n", &[cache_ttl.as_secs().to_string()])?;
    printf("Session timeout: {} seconds\n", &[session_timeout.as_secs().to_string()])?;
    printf("Backup interval: {} seconds\n", &[backup_interval.as_secs().to_string()])?;
    
    // Memory size parsing
    println("\n--- Memory Size Parsing ---")?;
    
    set_env("MAX_HEAP_SIZE", "512MB")?;
    set_env("BUFFER_SIZE", "64KB")?;
    set_env("CACHE_SIZE", "2GB")?;
    
    facts max_heap = parse_env_memory_size("MAX_HEAP_SIZE")?;
    facts buffer_size = parse_env_memory_size("BUFFER_SIZE")?;
    facts cache_size = parse_env_memory_size("CACHE_SIZE")?;
    
    printf("Max heap: {} bytes\n", &[max_heap.to_string()])?;
    printf("Buffer size: {} bytes\n", &[buffer_size.to_string()])?;
    printf("Cache size: {} bytes\n", &[cache_size.to_string()])?;
    
    // Numeric bounds checking
    println("\n--- Numeric Bounds Checking ---")?;
    
    set_env("WORKER_THREADS", "8")?;
    set_env("CPU_PERCENT", "75")?;
    
    // Parse with bounds checking
    vibe_check {
        facts threads = get_numeric_env::<u32>("WORKER_THREADS", 1, 16)?;
        printf("Worker threads: {} (valid range: 1-16)\n", &[threads.to_string()])?;
        
        facts cpu_percent = get_numeric_env::<f64>("CPU_PERCENT", 0.0, 100.0)?;
        printf("CPU percent: {}% (valid range: 0-100)\n", &[cpu_percent.to_string()])?;
    }
    mood err {
        printf("Bounds checking error: {}\n", &[err.to_string()])?;
    }
    
    // Path environment parsing
    println("\n--- PATH Environment Parsing ---")?;
    
    // Get the actual PATH environment variable
    lowkey facts path_var = get_env("PATH") {
        facts paths = get_path_env("PATH");
        lowkey facts path_list = paths {
            printf("PATH contains {} entries:\n", &[path_list.len().to_string()])?;
            
            // Show first 3 entries
            periodt (i, path) in path_list.iter().enumerate() {
                lowkey i < 3 {
                    printf("  {}: {}\n", &[(i + 1).to_string(), path.to_string_lossy()])?;
                }
            }
            
            lowkey path_list.len() > 3 {
                printf("  ... and {} more entries\n", &[(path_list.len() - 3).to_string()])?;
            }
        }
    } flex {
        println("PATH environment variable not found")?;
    }
    
    // Boolean parsing variations
    println("\n--- Boolean Parsing Variations ---")?;
    
    facts bool_tests = [
        ("based", "TRUE"),
        ("cap", "FALSE"), 
        ("1", "0"),
        ("yes", "no"),
        ("on", "off")
    ];
    
    periodt (true_val, false_val) in bool_tests {
        set_env("CURSED_BOOL_TEST", true_val)?;
        facts result_true = get_bool_env("CURSED_BOOL_TEST")?;
        
        set_env("CURSED_BOOL_TEST", false_val)?;
        facts result_false = get_bool_env("CURSED_BOOL_TEST")?;
        
        printf("'{}' -> {}, '{}' -> {}\n", 
               &[true_val.to_string(), result_true.to_string(), 
                 false_val.to_string(), result_false.to_string()])?;
    }
    
    // Complex configuration example
    println("\n--- Complex Configuration Example ---")?;
    
    set_env("SERVER_CONFIG", "host=localhost,port=8080,ssl=cap,workers=4")?;
    set_env("RATE_LIMITS", "api=1000,upload=10,download=100")?;
    set_env("FEATURES", "auth,logging,metrics,cache")?;
    
    facts server_config = parse_env_config("SERVER_CONFIG", ",")?;
    facts rate_limits = parse_env_config("RATE_LIMITS", ",")?;
    facts features = parse_env_list("FEATURES")?;
    
    println("Server configuration:")?;
    periodt (key, value) in server_config {
        printf("  server.{} = {}\n", &[key, value])?;
    }
    
    println("Rate limits:")?;
    periodt (key, value) in rate_limits {
        printf("  limit.{} = {}/hour\n", &[key, value])?;
    }
    
    println("Enabled features:")?;
    periodt feature in features {
        printf("  + {}\n", &[feature])?;
    }
    
    // Cleanup
    println("\n--- Cleanup ---")?;
    
    facts cleanup_vars = [
        "DATABASE_CONFIG", "ALLOWED_HOSTS", "LIBRARY_PATH", "SEARCH_PATHS",
        "CACHE_TTL", "SESSION_TIMEOUT", "BACKUP_INTERVAL",
        "MAX_HEAP_SIZE", "BUFFER_SIZE", "CACHE_SIZE",
        "WORKER_THREADS", "CPU_PERCENT", "CURSED_BOOL_TEST",
        "SERVER_CONFIG", "RATE_LIMITS", "FEATURES"
    ];
    
    periodt var in cleanup_vars {
        remove_env(var)?;
    }
    
    println("Advanced parsing demo completed!")?;
}
