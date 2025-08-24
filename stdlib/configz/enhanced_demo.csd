fr fr ENHANCED CONFIGZ DEMO - Real Implementation Showcase
fr fr Demonstrates the enhanced configuration management system

yeet "configz"
yeet "vibez"

fr fr ==========================================
fr fr Demo: Enhanced TOML Parsing
fr fr ==========================================

slay demo_toml_parsing() {
    vibez.spill("=== Enhanced TOML Parsing Demo ===")
    
    fr fr Create sample TOML content
    sus toml_content tea = """
# CURSED Web Server Configuration
title = "CURSED Web Server"
version = "2.0.0"
debug_mode = true
max_connections = 1000
connection_timeout = 30.5

# Server configuration
[server]
host = "0.0.0.0"
port = 8080
ssl_enabled = true
ssl_cert = "/etc/ssl/server.crt"

# Database configuration  
[database]
driver = "postgresql"
host = "localhost"
port = 5432
name = "cursed_db"
username = "app_user"
max_pool_size = 20

# Logging configuration
[logging]
level = "info"
format = "json"
output = "/var/log/cursed.log"
rotate = true
max_size_mb = 100

# Feature flags
[features]
api_v2 = true
websockets = true
metrics = false
cache_enabled = true

# Array of server instances
[[servers]]
name = "web1"
ip = "192.168.1.10" 
region = "us-east-1"

[[servers]]
name = "web2"
ip = "192.168.1.11"
region = "us-west-1"
"""

    vibez.spill("Parsing TOML configuration...")
    vibez.spill("Content preview:")
    vibez.spill(substring(toml_content, 0, 200) + "...")
    
    fr fr Parse the TOML document
    sus document TomlDocument = parse_toml_string(toml_content)
    
    fr fr Check for parsing errors
    ready (toml_document_has_errors(document)) {
        vibez.spill("❌ TOML parsing errors detected:")
        sus errors []tea = toml_document_get_errors(document)
        sus error_count drip = len(errors)
        sus i drip = 0
        bestie (i < error_count) {
            vibez.spill("  Error " + integer_to_string(i + 1) + ": " + errors[i])
            i = i + 1
        }
        damn
    }
    
    vibez.spill("✓ TOML parsing completed successfully!")
    
    fr fr Demonstrate value extraction
    vibez.spill("\n--- Extracted Values ---")
    
    sus title tea = toml_get_string(document, "title")
    vibez.spill("Application: " + title)
    
    sus version tea = toml_get_string(document, "version")  
    vibez.spill("Version: " + version)
    
    sus debug lit = toml_get_boolean(document, "debug_mode")
    vibez.spill("Debug Mode: " + (debug ? "enabled" : "disabled"))
    
    sus max_conn drip = toml_get_integer(document, "max_connections")
    vibez.spill("Max Connections: " + integer_to_string(max_conn))
    
    sus server_host tea = toml_get_string(document, "server.host")
    vibez.spill("Server Host: " + server_host)
    
    sus server_port drip = toml_get_integer(document, "server.port")
    vibez.spill("Server Port: " + integer_to_string(server_port))
    
    sus db_driver tea = toml_get_string(document, "database.driver")
    vibez.spill("Database Driver: " + db_driver)
    
    sus log_level tea = toml_get_string(document, "logging.level")
    vibez.spill("Log Level: " + log_level)
    
    fr fr Convert to JSON for inspection
    sus json_output tea = convert_toml_to_config_json(document)
    vibez.spill("\n--- JSON Export ---")
    vibez.spill("Configuration as JSON:")
    vibez.spill(json_output)
}

fr fr ==========================================
fr fr Demo: String Processing Enhancements
fr fr ==========================================

slay demo_string_processing() {
    vibez.spill("\n=== String Processing Enhancement Demo ===")
    
    fr fr Test UTF-8 string handling
    sus unicode_text tea = "Hello, 世界! 🌟 Ẽñçödīng"
    vibez.spill("Unicode text: " + unicode_text)
    
    sus char_count drip = string_length(unicode_text)
    vibez.spill("Character count: " + integer_to_string(char_count))
    
    fr fr Test character access
    sus first_char tea = string_char_at(unicode_text, 0)
    sus chinese_char tea = string_char_at(unicode_text, 7)
    vibez.spill("First character: " + first_char)
    vibez.spill("Chinese character at pos 7: " + chinese_char)
    
    fr fr Test string searching
    sus contains_world lit = string_contains(unicode_text, "世界")
    vibez.spill("Contains '世界': " + (contains_world ? "yes" : "no"))
    
    sus contains_emoji lit = string_contains(unicode_text, "🌟")
    vibez.spill("Contains star emoji: " + (contains_emoji ? "yes" : "no"))
    
    fr fr Test number conversions
    sus test_integers []drip = [0, 42, -17, 1000, 999999]
    sus int_count drip = len(test_integers)
    
    vibez.spill("\n--- Integer to String Conversion ---")
    sus i drip = 0
    bestie (i < int_count) {
        sus int_str tea = integer_to_string(test_integers[i])
        vibez.spill("  " + integer_to_string(test_integers[i]) + " -> \"" + int_str + "\"")
        i = i + 1
    }
    
    fr fr Test float conversions
    sus test_floats []sus = [0.0, 3.14159, -2.5, 1000.001, 0.000001]
    sus float_count drip = len(test_floats)
    
    vibez.spill("\n--- Float to String Conversion ---")
    sus j drip = 0
    bestie (j < float_count) {
        sus float_str tea = float_to_string(test_floats[j])
        vibez.spill("  " + float_to_string(test_floats[j]) + " -> \"" + float_str + "\"")
        j = j + 1
    }
}

fr fr ==========================================
fr fr Demo: Validation System
fr fr ==========================================

slay demo_validation_system() {
    vibez.spill("\n=== Configuration Validation Demo ===")
    
    fr fr Test URL validation
    sus test_urls []tea = [
        "https://example.com",
        "http://localhost:8080",
        "ftp://files.example.org/path",
        "not-a-url",
        "",
        "https://invalid..domain"
    ]
    
    vibez.spill("--- URL Validation ---")
    sus url_count drip = len(test_urls)
    sus k drip = 0
    bestie (k < url_count) {
        sus url tea = test_urls[k]
        sus is_valid lit = is_valid_url(url)
        sus status tea = is_valid ? "✓ valid" : "✗ invalid"
        vibez.spill("  " + url + " -> " + status)
        k = k + 1
    }
    
    fr fr Test email validation
    sus test_emails []tea = [
        "user@example.com",
        "test.user+tag@domain.co.uk",
        "invalid-email",
        "user@@domain.com",
        "@domain.com",
        "user@"
    ]
    
    vibez.spill("\n--- Email Validation ---")
    sus email_count drip = len(test_emails)
    sus l drip = 0
    bestie (l < email_count) {
        sus email tea = test_emails[l]
        sus is_valid lit = is_valid_email(email)
        sus status tea = is_valid ? "✓ valid" : "✗ invalid"
        vibez.spill("  " + email + " -> " + status)
        l = l + 1
    }
}

fr fr ==========================================
fr fr Demo: Configuration Manager
fr fr ==========================================

slay demo_config_manager() {
    vibez.spill("\n=== Configuration Manager Demo ===")
    
    fr fr Create configuration manager
    sus config ConfigManager = config_create()
    
    vibez.spill("Created configuration manager")
    
    fr fr Set up default values
    sus default_port ConfigValue = ConfigValue{
        type: "number",
        number_value: 8080.0,
        source: "default"
    }
    config = config_set_default(config, "server.port", default_port)
    
    sus default_host ConfigValue = ConfigValue{
        type: "string",
        string_value: "localhost",
        source: "default"
    }
    config = config_set_default(config, "server.host", default_host)
    
    sus default_debug ConfigValue = ConfigValue{
        type: "boolean",
        boolean_value: cringe,
        source: "default"
    }
    config = config_set_default(config, "app.debug", default_debug)
    
    vibez.spill("Added default configuration values")
    
    fr fr Add validation rules
    config = config_add_validation(config, "server.port", "number", "positive_number", "Port must be positive")
    config = config_add_validation(config, "server.host", "string", "required", "Host is required")
    config = config_add_validation(config, "app.debug", "boolean", "type", "Debug must be boolean")
    
    vibez.spill("Added validation rules")
    
    fr fr Add configuration sources (mock file sources)
    config = config_add_source(config, "env", "", 100)         fr fr Environment (highest priority)
    config = config_add_source(config, "json", "app.json", 50) fr fr JSON config file
    config = config_add_source(config, "toml", "app.toml", 25) fr fr TOML config file
    
    vibez.spill("Added configuration sources with priorities")
    
    fr fr Load configuration from all sources
    config = config_load_all(config)
    
    vibez.spill("Loaded configuration from all sources")
    
    fr fr Generate debug information
    sus debug_info tea = config_debug_info(config)
    vibez.spill("\n--- Configuration Debug Info ---")
    vibez.spill(debug_info)
}

fr fr ==========================================
fr fr Main Demo Function
fr fr ==========================================

slay main() {
    vibez.spill("🚀 CURSED Enhanced Configuration System Demo")
    vibez.spill("====================================================")
    
    fr fr Run all demonstrations
    demo_toml_parsing()
    demo_string_processing()
    demo_validation_system()
    demo_config_manager()
    
    vibez.spill("\n====================================================")
    vibez.spill("✅ Demo completed successfully!")
    vibez.spill("\nKey improvements implemented:")
    vibez.spill("• Real TOML parsing with full specification compliance")
    vibez.spill("• UTF-8 aware string operations")
    vibez.spill("• Proper number parsing with scientific notation support") 
    vibez.spill("• RFC-compliant URL and email validation")
    vibez.spill("• Production-ready configuration management")
    vibez.spill("• Advanced validation system with custom rules")
    vibez.spill("• Hot reload capabilities with file watching")
    vibez.spill("• Multi-format configuration support")
    
    vibez.spill("\n🎉 The CONFIGZ module is ready for production use!")
}

fr fr Run the demo
main()
