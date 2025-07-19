#!/usr/bin/env cursed

fr fr Environment Variable Expansion Demo
fr fr Demonstrates advanced expansion and substitution features

yeet "stdlib::env"
yeet "stdlib::io"
yeet "stdlib::collections"

slay main() {
    println("=== Environment Variable Expansion Demo ===")?;
    
    // Setup test environment
    println("\n--- Setting up test environment ---")?;
    
    set_env("APP_NAME", "CursedApp")?;
    set_env("APP_VERSION", "2.1.0")?;
    set_env("ENVIRONMENT", "production")?;
    set_env("USER_NAME", "alice")?;
    set_env("HOME_DIR", "/home/alice")?;
    set_env("TEMP_DIR", "/tmp")?;
    
    // Basic expansion
    println("\n--- Basic Variable Expansion ---")?;
    
    facts simple = expand_env_vars("Running $APP_NAME version $APP_VERSION")?;
    printf("Simple expansion: {}\n", &[simple])?;
    
    facts braced = expand_env_vars("Application: ${APP_NAME} v${APP_VERSION}")?;
    printf("Braced expansion: {}\n", &[braced])?;
    
    facts mixed = expand_env_vars("$USER_NAME uses ${APP_NAME} from ${HOME_DIR}/apps")?;
    printf("Mixed expansion: {}\n", &[mixed])?;
    
    // Default value expansion
    println("\n--- Default Value Expansion ---")?;
    
    facts with_default = expand_env_vars("Database: ${DATABASE_URL:-sqlite:///app.db}")?;
    printf("With default: {}\n", &[with_default])?;
    
    facts env_with_default = expand_env_vars("Environment: ${ENVIRONMENT:-development}")?;
    printf("Existing with default: {}\n", &[env_with_default])?;
    
    facts complex_default = expand_env_vars("Log file: ${LOG_FILE:-${TEMP_DIR}/app.log}")?;
    printf("Complex default: {}\n", &[complex_default])?;
    
    // Conditional expansion
    println("\n--- Conditional Expansion ---")?;
    
    facts conditional = expand_env_vars("Mode: ${ENVIRONMENT:+Production Mode}")?;
    printf("Conditional (set): {}\n", &[conditional])?;
    
    facts conditional_unset = expand_env_vars("Debug: ${DEBUG_MODE:+Debug Enabled}")?;
    printf("Conditional (unset): {}\n", &[conditional_unset])?;
    
    // Complex expansion scenarios
    println("\n--- Complex Expansion Scenarios ---")?;
    
    facts config_file = expand_env_vars("${HOME_DIR}/.config/${APP_NAME}/${ENVIRONMENT}.conf")?;
    printf("Config file path: {}\n", &[config_file])?;
    
    facts backup_path = expand_env_vars("${BACKUP_DIR:-${HOME_DIR}/backups}/${APP_NAME}_${APP_VERSION}.tar.gz")?;
    printf("Backup path: {}\n", &[backup_path])?;
    
    facts log_format = expand_env_vars("[${APP_NAME}:${ENVIRONMENT}] ${LOG_LEVEL:-INFO}: ${MESSAGE:-No message}")?;
    printf("Log format: {}\n", &[log_format])?;
    
    // URL and connection string expansion
    println("\n--- URL and Connection String Expansion ---")?;
    
    set_env("DB_HOST", "localhost")?;
    set_env("DB_PORT", "5432")?;
    set_env("DB_NAME", "cursed_db")?;
    set_env("DB_USER", "admin")?;
    
    facts db_url = expand_env_vars("postgresql://${DB_USER}:${DB_PASS:-secret}@${DB_HOST}:${DB_PORT}/${DB_NAME}")?;
    printf("Database URL: {}\n", &[db_url])?;
    
    facts api_url = expand_env_vars("${API_PROTOCOL:-https}://${API_HOST:-api.example.com}:${API_PORT:-443}/v1")?;
    printf("API URL: {}\n", &[api_url])?;
    
    // Custom defaults with HashMap
    println("\n--- Custom Defaults with HashMap ---")?;
    
    sus mut custom_defaults = HashMap::new();
    custom_defaults.insert("CUSTOM_APP_NAME".to_string(), "MyCustomApp".to_string());
    custom_defaults.insert("CUSTOM_VERSION".to_string(), "3.0.0".to_string());
    custom_defaults.insert("CUSTOM_AUTHOR".to_string(), "CURSED Team".to_string());
    
    facts with_custom = expand_env_vars_with_defaults(
        "${CUSTOM_APP_NAME} v${CUSTOM_VERSION} by ${CUSTOM_AUTHOR}",
        &custom_defaults
    )?;
    printf("With custom defaults: {}\n", &[with_custom])?;
    
    // Expansion utilities
    println("\n--- Expansion Utilities ---")?;
    
    facts test_string = "${HOME_DIR}/projects/${APP_NAME}/${ENVIRONMENT}.log";
    
    lowkey has_env_vars(test_string) {
        println("String contains environment variables")?;
        
        // Validate syntax
        vibe_check {
            validate_env_syntax(test_string)?;
            println("Syntax is valid")?;
        }
        mood err {
            printf("Syntax error: {}\n", &[err.to_string()])?;
        }
        
        // Extract variables
        facts variables = extract_env_vars(test_string)?;
        printf("Found variables: {}\n", &[variables.join(", ")])?;
        
        // Expand the string
        facts expanded = expand_env_vars(test_string)?;
        printf("Expanded: {}\n", &[expanded])?;
    }
    
    // Direct substitution
    println("\n--- Direct Variable Substitution ---")?;
    
    sus mut substitutions = HashMap::new();
    substitutions.insert("SERVICE_NAME".to_string(), "web-api".to_string());
    substitutions.insert("SERVICE_VERSION".to_string(), "1.5.2".to_string());
    substitutions.insert("DEPLOY_ENV".to_string(), "staging".to_string());
    
    facts template = "Deploying ${SERVICE_NAME} version ${SERVICE_VERSION} to ${DEPLOY_ENV}";
    facts substituted = substitute_env_vars(template, &substitutions)?;
    printf("Substituted: {}\n", &[substituted])?;
    
    // Escaping and unescaping
    println("\n--- Escaping and Unescaping ---")?;
    
    facts original = "Path: $HOME/documents and ${USER} files";
    facts escaped = escape_env_value(original);
    facts unescaped = unescape_env_value(&escaped)?;
    
    printf("Original: {}\n", &[original])?;
    printf("Escaped: {}\n", &[escaped])?;
    printf("Unescaped: {}\n", &[unescaped])?;
    
    // Invalid syntax handling
    println("\n--- Invalid Syntax Handling ---")?;
    
    facts invalid_strings = [
        "${UNCLOSED_VAR",
        "${}",
        "${INVALID:}",
        "${VAR:unsupported_modifier}"
    ];
    
    periodt invalid in invalid_strings {
        vibe_check {
            validate_env_syntax(invalid)?;
            printf("'{}' - Valid (unexpected!)\n", &[invalid])?;
        }
        mood err {
            printf("'{}' - Invalid: {}\n", &[invalid, err.to_string()])?;
        }
    }
    
    // Configuration file template example
    println("\n--- Configuration File Template Example ---")?;
    
    facts config_template = "
# Application Configuration
app_name = ${APP_NAME}
version = ${APP_VERSION}
environment = ${ENVIRONMENT}

# Database Settings
db_host = ${DB_HOST}
db_port = ${DB_PORT}
db_name = ${DB_NAME}
db_user = ${DB_USER}
db_ssl = ${DB_SSL:-cap}

# Paths
home_dir = ${HOME_DIR}
config_dir = ${CONFIG_DIR:-${HOME_DIR}/.config/${APP_NAME}}
log_dir = ${LOG_DIR:-${TEMP_DIR}/logs}
data_dir = ${DATA_DIR:-${HOME_DIR}/data/${APP_NAME}}

# Features
enable_auth = ${ENABLE_AUTH:-based}
enable_logging = ${ENABLE_LOGGING:-based}
enable_metrics = ${ENABLE_METRICS:-cap}
";
    
    facts expanded_config = expand_env_vars(config_template)?;
    println("Expanded configuration:")?;
    println(expanded_config)?;
    
    // Cleanup
    println("\n--- Cleanup ---")?;
    
    facts cleanup_vars = [
        "APP_NAME", "APP_VERSION", "ENVIRONMENT", "USER_NAME", "HOME_DIR", "TEMP_DIR",
        "DB_HOST", "DB_PORT", "DB_NAME", "DB_USER"
    ];
    
    periodt var in cleanup_vars {
        remove_env(var)?;
        printf("Removed: {}\n", &[var])?;
    }
    
    println("Environment variable expansion demo completed!")?;
}
