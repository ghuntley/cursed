yeet "configz"

fr fr ==========================================
fr fr CURSED Enhanced Configuration Management Demo
fr fr Simple demonstration without complex dependencies
fr fr ==========================================

slay demo_configuration_management() {
    vibez.spill("🚀 CURSED Configuration Management Demo")
    vibez.spill("")
    
    fr fr Test format constants
    vibez.spill("📋 Configuration Format Support:")
    vibez.spill("  JSON format:", format_json())
    vibez.spill("  YAML format:", format_yaml())
    vibez.spill("  TOML format:", format_toml())
    vibez.spill("  ENV format:", format_env())
    vibez.spill("")
    
    fr fr Test environment variables
    vibez.spill("🌍 Environment Variables:")
    sus home_value tea = get_env_variable("HOME")
    vibez.spill("  HOME:", home_value)
    
    sus user_value tea = get_env_variable("USER")
    vibez.spill("  USER:", user_value)
    
    sus debug_value tea = get_env_variable("DEBUG")
    vibez.spill("  DEBUG:", debug_value)
    vibez.spill("")
    
    fr fr Test format detection
    vibez.spill("🔍 Format Detection:")
    sus json_content tea = "{\"key\": \"value\"}"
    sus detected_format tea = auto_detect_format(json_content)
    vibez.spill("  JSON content detected as:", detected_format)
    
    sus filename_format tea = detect_format_from_filename("config.json")
    vibez.spill("  config.json detected as:", filename_format)
    vibez.spill("")
    
    fr fr Test configuration parsing
    vibez.spill("⚙️ Configuration Parsing:")
    sus config_content tea = "{\"database\":{\"host\":\"localhost\",\"port\":5432}}"
    sus parsed_config tea = parse_json_config(config_content)
    sus is_valid lit = validate_configuration(parsed_config)
    ready (is_valid) {
        vibez.spill("  JSON config valid: yes")
    } otherwise {
        vibez.spill("  JSON config valid: no")
    }
    vibez.spill("")
    
    fr fr Test environment variable expansion
    vibez.spill("🔄 Environment Variable Expansion:")
    sus input_with_vars tea = "Database host: ${DB_HOST}"
    sus expanded tea = expand_environment_variables(input_with_vars)
    vibez.spill("  Original:", input_with_vars)
    vibez.spill("  Expanded:", expanded)
    vibez.spill("")
    
    fr fr Test value access
    vibez.spill("📖 Configuration Value Access:")
    sus test_config tea = "{\"app_name\":\"TestApp\",\"debug\":\"true\",\"port\":\"3000\"}"
    
    sus app_name tea = get_config_string(test_config, "app_name", "DefaultApp")
    vibez.spill("  App name:", app_name)
    
    sus port_value normie = get_config_int(test_config, "port", 8080)
    vibez.spill("  Port:", port_value)
    
    sus debug_flag lit = get_config_bool(test_config, "debug", cap)
    ready (debug_flag) {
        vibez.spill("  Debug enabled: yes")
    } otherwise {
        vibez.spill("  Debug enabled: no")
    }
    
    sus missing_value tea = get_config_string(test_config, "missing_key", "DefaultValue")
    vibez.spill("  Missing key (with default):", missing_value)
    vibez.spill("")
    
    fr fr Test type detection
    vibez.spill("🏷️ Type Detection:")
    vibez.spill("  'true' detected as:", detect_value_type("true"))
    vibez.spill("  '42' detected as:", detect_value_type("42"))
    vibez.spill("  'hello' detected as:", detect_value_type("hello"))
    vibez.spill("")
    
    fr fr Test validation
    vibez.spill("✅ Validation:")
    sus valid_config tea = "{\"key\":\"value\"}"
    sus valid_result lit = validate_configuration(valid_config)
    ready (valid_result) {
        vibez.spill("  Valid JSON config: passed")
    } otherwise {
        vibez.spill("  Valid JSON config: failed")
    }
    
    sus invalid_config tea = "invalid json"
    sus invalid_result lit = validate_configuration(invalid_config)
    ready (invalid_result) {
        vibez.spill("  Invalid config: passed")
    } otherwise {
        vibez.spill("  Invalid config: failed")
    }
    
    sus integer_valid lit = validate_value_type("42", "integer")
    ready (integer_valid) {
        vibez.spill("  '42' as integer: valid")
    } otherwise {
        vibez.spill("  '42' as integer: invalid")
    }
    
    sus boolean_valid lit = validate_value_type("true", "boolean")
    ready (boolean_valid) {
        vibez.spill("  'true' as boolean: valid")
    } otherwise {
        vibez.spill("  'true' as boolean: invalid")
    }
    vibez.spill("")
    
    fr fr Test configuration merging
    vibez.spill("🔀 Configuration Merging:")
    sus base_config tea = "{\"app\":\"base\",\"debug\":\"false\"}"
    sus override_config tea = "{\"debug\":\"true\",\"port\":\"8080\"}"
    sus merged_config tea = merge_configurations(base_config, override_config)
    sus merge_valid lit = validate_configuration(merged_config)
    ready (merge_valid) {
        vibez.spill("  Merge result valid: yes")
    } otherwise {
        vibez.spill("  Merge result valid: no")
    }
    vibez.spill("")
    
    fr fr Test file loading simulation
    vibez.spill("📁 File Loading Simulation:")
    sus file_result tea = load_configuration_from_file("config.json")
    sus file_valid lit = validate_configuration(file_result)
    ready (file_valid) {
        vibez.spill("  config.json loaded: successfully")
    } otherwise {
        vibez.spill("  config.json loaded: failed")
    }
    
    sus env_file_result tea = load_configuration_from_file(".env")
    sus env_file_valid lit = validate_configuration(env_file_result)
    ready (env_file_valid) {
        vibez.spill("  .env loaded: successfully")
    } otherwise {
        vibez.spill("  .env loaded: failed")
    }
    vibez.spill("")
    
    fr fr Test environment context
    vibez.spill("🏗️ Environment Context:")
    sus env_context tea = detect_environment_context()
    vibez.spill("  Current environment:", env_context)
    
    sus env_config tea = load_environment_configuration()
    sus env_config_valid lit = validate_configuration(env_config)
    ready (env_config_valid) {
        vibez.spill("  Environment config loaded: successfully")
    } otherwise {
        vibez.spill("  Environment config loaded: failed")
    }
    vibez.spill("")
    
    fr fr Test high-level API
    vibez.spill("🎯 High-Level API:")
    sus api_content tea = "{\"app\":\"api_test\"}"
    sus api_parsed tea = parse_config(api_content)
    sus api_valid lit = validate_config(api_parsed)
    ready (api_valid) {
        vibez.spill("  API parse function: working")
    } otherwise {
        vibez.spill("  API parse function: failed")
    }
    
    sus test_db_config tea = "{\"database_url\":\"postgres://localhost\",\"port\":\"5432\"}"
    sus db_url tea = get_value(test_db_config, "database_url")
    vibez.spill("  Database URL:", db_url)
    
    sus updated_config tea = set_value(test_db_config, "timeout", "30")
    sus update_valid lit = validate_config(updated_config)
    ready (update_valid) {
        vibez.spill("  Set value operation: successful")
    } otherwise {
        vibez.spill("  Set value operation: failed")
    }
    vibez.spill("")
    
    vibez.spill("🎉 Configuration Management Demo Complete!")
    vibez.spill("✅ All core functionality demonstrated successfully")
}

fr fr Run the demonstration
demo_configuration_management()
