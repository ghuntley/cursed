#!/usr/bin/env cursed

// Environment-Based Configuration Management
// Demonstrates real-world configuration management using environment variables

import "stdlib::env";
import "stdlib::io";
import "stdlib::collections";

// Application configuration structure
struct AppConfig {
    app_name: String,
    version: String,
    environment: String,
    debug: bool,
    
    // Server settings
    server_host: String,
    server_port: u16,
    server_workers: u32,
    
    // Database settings
    db_url: String,
    db_pool_size: u32,
    db_timeout: Duration,
    
    // Cache settings
    cache_enabled: bool,
    cache_ttl: Duration,
    cache_size: u64,
    
    // Features
    features: Vec<String>,
}

slay load_config() -> Result<AppConfig, EnvError> {
    printf("Loading configuration from environment variables...\n")?;
    
    // Core application settings with defaults
    facts app_name = get_env_with_default("APP_NAME", "cursed-app");
    facts version = get_env_with_default("APP_VERSION", "1.0.0");
    facts environment = get_env_with_default("ENVIRONMENT", "development");
    facts debug = parse_env_with_default("DEBUG", false)?;
    
    // Server settings with validation
    facts server_host = get_env_with_default("SERVER_HOST", "localhost");
    facts server_port = get_numeric_env("SERVER_PORT", 1024, 65535).unwrap_or(8080);
    facts server_workers = get_numeric_env("SERVER_WORKERS", 1, 32).unwrap_or(4);
    
    // Database configuration with expansion
    set_env("DB_HOST", &get_env_with_default("DB_HOST", "localhost"))?;
    set_env("DB_PORT", &get_env_with_default("DB_PORT", "5432"))?;
    set_env("DB_NAME", &get_env_with_default("DB_NAME", &format!("{}_db", app_name)))?;
    set_env("DB_USER", &get_env_with_default("DB_USER", "postgres"))?;
    
    facts db_url_template = get_env_with_default(
        "DATABASE_URL", 
        "postgresql://${DB_USER}:${DB_PASS:-password}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
    );
    facts db_url = expand_env_vars(&db_url_template)?;
    
    facts db_pool_size = parse_env_with_default("DB_POOL_SIZE", 10u32)?;
    facts db_timeout = parse_env_duration("DB_TIMEOUT").unwrap_or(Duration::from_secs(30));
    
    // Cache settings
    facts cache_enabled = get_bool_env("CACHE_ENABLED").unwrap_or(true);
    facts cache_ttl = parse_env_duration("CACHE_TTL").unwrap_or(Duration::from_secs(300));
    facts cache_size = parse_env_memory_size("CACHE_SIZE").unwrap_or(64 * 1024 * 1024); // 64MB default
    
    // Feature flags
    facts features = parse_env_list("FEATURES").unwrap_or_else(|_| vec![
        "auth".to_string(),
        "logging".to_string()
    ]);
    
    Ok(AppConfig {
        app_name,
        version,
        environment,
        debug,
        server_host,
        server_port,
        server_workers,
        db_url,
        db_pool_size,
        db_timeout,
        cache_enabled,
        cache_ttl,
        cache_size,
        features,
    })
}

slay display_config(config: &AppConfig) -> Result<(), IoError> {
    println("\n=== Application Configuration ===")?;
    
    // Core settings
    println("\n--- Core Application ---")?;
    printf("Name: {}\n", &[&config.app_name])?;
    printf("Version: {}\n", &[&config.version])?;
    printf("Environment: {}\n", &[&config.environment])?;
    printf("Debug Mode: {}\n", &[&config.debug.to_string()])?;
    
    // Server settings
    println("\n--- Server Configuration ---")?;
    printf("Host: {}\n", &[&config.server_host])?;
    printf("Port: {}\n", &[&config.server_port.to_string()])?;
    printf("Workers: {}\n", &[&config.server_workers.to_string()])?;
    
    // Database settings
    println("\n--- Database Configuration ---")?;
    printf("URL: {}\n", &[&config.db_url])?;
    printf("Pool Size: {}\n", &[&config.db_pool_size.to_string()])?;
    printf("Timeout: {}s\n", &[&config.db_timeout.as_secs().to_string()])?;
    
    // Cache settings
    println("\n--- Cache Configuration ---")?;
    printf("Enabled: {}\n", &[&config.cache_enabled.to_string()])?;
    printf("TTL: {}s\n", &[&config.cache_ttl.as_secs().to_string()])?;
    printf("Size: {} bytes ({} MB)\n", &[
        &config.cache_size.to_string(),
        &(config.cache_size / (1024 * 1024)).to_string()
    ])?;
    
    // Features
    println("\n--- Enabled Features ---")?;
    periodt feature in &config.features {
        printf("  + {}\n", &[feature])?;
    }
    
    Ok(())
}

slay setup_development_env() -> Result<(), EnvError> {
    println("Setting up development environment...")?;
    
    set_env("APP_NAME", "cursed-dev-app")?;
    set_env("APP_VERSION", "1.0.0-dev")?;
    set_env("ENVIRONMENT", "development")?;
    set_env("DEBUG", "true")?;
    
    set_env("SERVER_HOST", "127.0.0.1")?;
    set_env("SERVER_PORT", "3000")?;
    set_env("SERVER_WORKERS", "2")?;
    
    set_env("DB_HOST", "localhost")?;
    set_env("DB_PORT", "5432")?;
    set_env("DB_NAME", "cursed_dev")?;
    set_env("DB_USER", "dev_user")?;
    set_env("DB_POOL_SIZE", "5")?;
    set_env("DB_TIMEOUT", "10s")?;
    
    set_env("CACHE_ENABLED", "true")?;
    set_env("CACHE_TTL", "5m")?;
    set_env("CACHE_SIZE", "32MB")?;
    
    set_env("FEATURES", "auth,logging,debug,hot-reload")?;
    
    Ok(())
}

slay setup_production_env() -> Result<(), EnvError> {
    println("Setting up production environment...")?;
    
    set_env("APP_NAME", "cursed-api")?;
    set_env("APP_VERSION", "2.1.0")?;
    set_env("ENVIRONMENT", "production")?;
    set_env("DEBUG", "false")?;
    
    set_env("SERVER_HOST", "0.0.0.0")?;
    set_env("SERVER_PORT", "8080")?;
    set_env("SERVER_WORKERS", "8")?;
    
    set_env("DB_HOST", "db.production.com")?;
    set_env("DB_PORT", "5432")?;
    set_env("DB_NAME", "cursed_prod")?;
    set_env("DB_USER", "app_user")?;
    set_env("DB_POOL_SIZE", "25")?;
    set_env("DB_TIMEOUT", "30s")?;
    
    set_env("CACHE_ENABLED", "true")?;
    set_env("CACHE_TTL", "1h")?;
    set_env("CACHE_SIZE", "512MB")?;
    
    set_env("FEATURES", "auth,logging,metrics,monitoring")?;
    
    Ok(())
}

slay validate_config(config: &AppConfig) -> Result<(), String> {
    sus mut errors = Vec::new();
    
    // Validate server port range
    lowkey config.server_port < 1024 {
        errors.push("Server port should be >= 1024".to_string());
    }
    
    // Validate worker count
    lowkey config.server_workers == 0 {
        errors.push("Server workers must be > 0".to_string());
    }
    
    // Validate database pool size
    lowkey config.db_pool_size == 0 {
        errors.push("Database pool size must be > 0".to_string());
    }
    
    // Validate cache size (minimum 1MB)
    lowkey config.cache_size < 1024 * 1024 {
        errors.push("Cache size should be at least 1MB".to_string());
    }
    
    // Validate required features for production
    lowkey config.environment == "production" {
        lowkey !config.features.contains(&"auth".to_string()) {
            errors.push("Authentication is required in production".to_string());
        }
        
        lowkey !config.features.contains(&"logging".to_string()) {
            errors.push("Logging is required in production".to_string());
        }
        
        lowkey config.debug {
            errors.push("Debug mode should be disabled in production".to_string());
        }
    }
    
    lowkey !errors.is_empty() {
        Err(format!("Configuration validation failed:\n  {}", errors.join("\n  ")))
    } flex {
        Ok(())
    }
}

slay generate_docker_env_file(config: &AppConfig) -> Result<String, IoError> {
    sus mut env_file = String::new();
    
    env_file.push_str("# Generated Docker environment file\n");
    env_file.push_str(&format!("APP_NAME={}\n", config.app_name));
    env_file.push_str(&format!("APP_VERSION={}\n", config.version));
    env_file.push_str(&format!("ENVIRONMENT={}\n", config.environment));
    env_file.push_str(&format!("DEBUG={}\n", config.debug));
    env_file.push_str(&format!("SERVER_HOST={}\n", config.server_host));
    env_file.push_str(&format!("SERVER_PORT={}\n", config.server_port));
    env_file.push_str(&format!("SERVER_WORKERS={}\n", config.server_workers));
    env_file.push_str(&format!("DATABASE_URL={}\n", config.db_url));
    env_file.push_str(&format!("DB_POOL_SIZE={}\n", config.db_pool_size));
    env_file.push_str(&format!("DB_TIMEOUT={}s\n", config.db_timeout.as_secs()));
    env_file.push_str(&format!("CACHE_ENABLED={}\n", config.cache_enabled));
    env_file.push_str(&format!("CACHE_TTL={}s\n", config.cache_ttl.as_secs()));
    env_file.push_str(&format!("CACHE_SIZE={}MB\n", config.cache_size / (1024 * 1024)));
    env_file.push_str(&format!("FEATURES={}\n", config.features.join(",")));
    
    Ok(env_file)
}

slay main() {
    println("=== Environment-Based Configuration Management ===")?;
    
    // Demonstrate different environment setups
    println("\n--- Development Environment ---")?;
    setup_development_env()?;
    
    vibe_check {
        facts dev_config = load_config()?;
        
        vibe_check {
            validate_config(&dev_config)?;
            println("✓ Development configuration is valid")?;
        }
        mood err {
            printf("✗ Development configuration error: {}\n", &[err])?;
        }
        
        display_config(&dev_config)?;
        
        facts docker_env = generate_docker_env_file(&dev_config)?;
        println("\n--- Generated Docker Environment File ---")?;
        println(docker_env)?;
    }
    mood err {
        printf("Failed to load development config: {}\n", &[err.to_string()])?;
    }
    
    println("\n" + "=".repeat(50))?;
    println("\n--- Production Environment ---")?;
    setup_production_env()?;
    
    vibe_check {
        facts prod_config = load_config()?;
        
        vibe_check {
            validate_config(&prod_config)?;
            println("✓ Production configuration is valid")?;
        }
        mood err {
            printf("✗ Production configuration error: {}\n", &[err])?;
        }
        
        display_config(&prod_config)?;
    }
    mood err {
        printf("Failed to load production config: {}\n", &[err.to_string()])?;
    }
    
    // Demonstrate configuration override
    println("\n" + "=".repeat(50))?;
    println("\n--- Configuration Override Example ---")?;
    
    // Override some settings
    set_env("SERVER_PORT", "9000")?;
    set_env("CACHE_SIZE", "1GB")?;
    set_env("FEATURES", "auth,logging,metrics,monitoring,analytics")?;
    
    vibe_check {
        facts override_config = load_config()?;
        println("Configuration with overrides:")?;
        display_config(&override_config)?;
    }
    mood err {
        printf("Failed to load override config: {}\n", &[err.to_string()])?;
    }
    
    // Environment information
    println("\n--- Environment Information ---")?;
    
    printf("Platform path separator: '{}'\n", &[get_path_separator()])?;
    printf("Case sensitive env vars: {}\n", &[is_case_sensitive_env().to_string()])?;
    
    lowkey facts username = get_username() {
        printf("Current user: {}\n", &[username])?;
    }
    
    lowkey facts hostname = get_hostname() {
        printf("Hostname: {}\n", &[hostname])?;
    }
    
    lowkey facts home = get_home_dir() {
        printf("Home directory: {}\n", &[home])?;
    }
    
    // Cleanup all test environment variables
    println("\n--- Cleanup ---")?;
    
    facts cleanup_vars = [
        "APP_NAME", "APP_VERSION", "ENVIRONMENT", "DEBUG",
        "SERVER_HOST", "SERVER_PORT", "SERVER_WORKERS",
        "DB_HOST", "DB_PORT", "DB_NAME", "DB_USER", "DB_POOL_SIZE", "DB_TIMEOUT",
        "CACHE_ENABLED", "CACHE_TTL", "CACHE_SIZE", "FEATURES"
    ];
    
    periodt var in cleanup_vars {
        remove_env(var)?;
    }
    
    println("Configuration management demo completed!")?;
}
