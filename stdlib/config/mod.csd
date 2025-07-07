// CURSED Configuration Management Library
// Production-ready configuration parsing and management

yeet "string"
yeet "json"
yeet "collections"

// ================================
// Core Configuration Functions
// ================================

// Load configuration from file with auto-format detection
slay load_file(filepath tea) map {
    sus content tea = file_read(filepath);
    sus lower_path tea = string_to_lower(filepath);
    
    // Auto-detect format based on file extension
    sus config map = map_create();
    
    lowkey string_contains(lower_path, ".json") {
        damn parse_json_config(content);
    }
    
    lowkey string_contains(lower_path, ".ini") {
        damn parse_ini(content);
    }
    
    lowkey string_contains(lower_path, ".env") {
        damn parse_env(content);
    }
    
    lowkey string_contains(lower_path, ".yaml") || string_contains(lower_path, ".yml") {
        damn parse_yaml_like(content);
    }
    
    // Default to INI format if no extension matches
    damn parse_ini(content);
}

// Save configuration to file
slay save_file(config map, filepath tea) lit {
    sus lower_path tea = string_to_lower(filepath);
    sus content tea = "";
    
    lowkey string_contains(lower_path, ".json") {
        content = json_stringify(config);
    } else lowkey string_contains(lower_path, ".ini") {
        content = stringify_ini(config);
    } else lowkey string_contains(lower_path, ".env") {
        content = stringify_env(config);
    } else {
        content = stringify_ini(config);
    }
    
    damn file_write(filepath, content);
}

// ================================
// INI Format Parsing
// ================================

// Parse INI format configuration
slay parse_ini(content tea) map {
    sus config map = map_create();
    sus current_section tea = "";
    sus lines [tea] = string_split(content, "\n");
    
    bestie i := 0; i < array_len(lines); i++ {
        sus line tea = string_trim(lines[i]);
        
        // Skip empty lines and comments
        lowkey string_is_empty(line) || string_starts_with(line, "#") || string_starts_with(line, ";") {
            continue;
        }
        
        // Check for section headers [section]
        lowkey string_starts_with(line, "[") && string_ends_with(line, "]") {
            current_section = string_slice(line, 1, string_len(line) - 1);
            continue;
        }
        
        // Parse key=value pairs
        sus eq_pos normie = string_find(line, "=");
        lowkey eq_pos > 0 {
            sus key tea = string_trim(string_slice(line, 0, eq_pos));
            sus value tea = string_trim(string_slice(line, eq_pos + 1, string_len(line)));
            
            // Remove quotes from value if present
            lowkey string_starts_with(value, "\"") && string_ends_with(value, "\"") {
                value = string_slice(value, 1, string_len(value) - 1);
            }
            
            // Build full key path
            sus full_key tea = key;
            lowkey !string_is_empty(current_section) {
                full_key = current_section + "." + key;
            }
            
            config = map_set(config, full_key, value);
        }
    }
    
    damn config;
}

// ================================
// Environment Variable Parsing
// ================================

// Parse environment variable format
slay parse_env(content tea) map {
    sus config map = map_create();
    sus lines [tea] = string_split(content, "\n");
    
    bestie i := 0; i < array_len(lines); i++ {
        sus line tea = string_trim(lines[i]);
        
        // Skip empty lines and comments
        lowkey string_is_empty(line) || string_starts_with(line, "#") {
            continue;
        }
        
        // Skip export statements
        lowkey string_starts_with(line, "export ") {
            line = string_slice(line, 7, string_len(line));
        }
        
        // Parse KEY=VALUE pairs
        sus eq_pos normie = string_find(line, "=");
        lowkey eq_pos > 0 {
            sus key tea = string_trim(string_slice(line, 0, eq_pos));
            sus value tea = string_trim(string_slice(line, eq_pos + 1, string_len(line)));
            
            // Remove quotes from value if present
            lowkey (string_starts_with(value, "\"") && string_ends_with(value, "\"")) ||
                 (string_starts_with(value, "'") && string_ends_with(value, "'")) {
                value = string_slice(value, 1, string_len(value) - 1);
            }
            
            config = map_set(config, key, value);
        }
    }
    
    damn config;
}

// ================================
// Configuration Serialization
// ================================

// Convert configuration to INI format
slay stringify_ini(config map) tea {
    sus result tea = "";
    sus keys [tea] = map_keys(config);
    sus sections map = map_create();
    sus global_keys [tea] = [];
    
    // Group keys by section
    bestie i := 0; i < array_len(keys); i++ {
        sus key tea = keys[i];
        sus dot_pos normie = string_find(key, ".");
        
        lowkey dot_pos > 0 {
            sus section tea = string_slice(key, 0, dot_pos);
            sus section_key tea = string_slice(key, dot_pos + 1, string_len(key));
            
            lowkey !map_has_key(sections, section) {
                sections = map_set(sections, section, []);
            }
            
            sus section_keys [tea] = map_get(sections, section).([tea]);
            section_keys = array_append(section_keys, section_key + "=" + map_get(config, key).(tea));
            sections = map_set(sections, section, section_keys);
        } else {
            global_keys = array_append(global_keys, key + "=" + map_get(config, key).(tea));
        }
    }
    
    // Write global keys first
    bestie i := 0; i < array_len(global_keys); i++ {
        result = result + global_keys[i] + "\n";
    }
    
    // Write sections
    sus section_names [tea] = map_keys(sections);
    bestie i := 0; i < array_len(section_names); i++ {
        sus section_name tea = section_names[i];
        sus section_keys [tea] = map_get(sections, section_name).([tea]);
        
        lowkey !string_is_empty(result) {
            result = result + "\n";
        }
        
        result = result + "[" + section_name + "]\n";
        
        bestie j := 0; j < array_len(section_keys); j++ {
            result = result + section_keys[j] + "\n";
        }
    }
    
    damn result;
}

// Convert configuration to environment variable format
slay stringify_env(config map) tea {
    sus result tea = "";
    sus keys [tea] = map_keys(config);
    
    bestie i := 0; i < array_len(keys); i++ {
        sus key tea = keys[i];
        sus value tea = map_get(config, key).(tea);
        
        // Convert dots to underscores and uppercase
        sus env_key tea = string_to_upper(string_replace(key, ".", "_"));
        
        // Quote value if it contains spaces
        lowkey string_contains(value, " ") {
            value = "\"" + value + "\"";
        }
        
        result = result + env_key + "=" + value + "\n";
    }
    
    damn result;
}

// ================================
// Configuration Access Functions
// ================================

// Get configuration value with path support
slay get_value(config map, key tea) tea {
    lowkey map_has_key(config, key) {
        damn map_get(config, key).(tea);
    }
    
    damn "";
}

// Set configuration value with path support
slay set_value(config map, key tea, value tea) map {
    damn map_set(config, key, value);
}

// Check if configuration key exists
slay has_key(config map, key tea) lit {
    damn map_has_key(config, key);
}

// Get configuration value with default
slay get_default(config map, key tea, default_value tea) tea {
    lowkey map_has_key(config, key) {
        damn map_get(config, key).(tea);
    }
    
    damn default_value;
}

// Get entire configuration section
slay get_section(config map, section tea) map {
    sus result map = map_create();
    sus keys [tea] = map_keys(config);
    sus section_prefix tea = section + ".";
    
    bestie i := 0; i < array_len(keys); i++ {
        sus key tea = keys[i];
        
        lowkey string_starts_with(key, section_prefix) {
            sus section_key tea = string_slice(key, string_len(section_prefix), string_len(key));
            sus value extra = map_get(config, key);
            result = map_set(result, section_key, value);
        }
    }
    
    damn result;
}

// ================================
// Configuration Merging
// ================================

// Merge two configurations with override support
slay merge_configs(base map, override map) map {
    sus result map = map_create();
    
    // Copy base config
    sus base_keys [tea] = map_keys(base);
    bestie i := 0; i < array_len(base_keys); i++ {
        sus key tea = base_keys[i];
        sus value extra = map_get(base, key);
        result = map_set(result, key, value);
    }
    
    // Apply overrides
    sus override_keys [tea] = map_keys(override);
    bestie i := 0; i < array_len(override_keys); i++ {
        sus key tea = override_keys[i];
        sus value extra = map_get(override, key);
        result = map_set(result, key, value);
    }
    
    damn result;
}

// Apply command-line style overrides
slay apply_overrides(config map, overrides map) map {
    damn merge_configs(config, overrides);
}

// ================================
// Schema Validation
// ================================

// Validate configuration against schema
slay validate_schema(config map, schema map) lit {
    sus required_keys [tea] = map_get(schema, "required").([tea]);
    
    // Check required keys
    bestie i := 0; i < array_len(required_keys); i++ {
        sus key tea = required_keys[i];
        
        lowkey !map_has_key(config, key) {
            damn cap;
        }
    }
    
    damn based;
}

// ================================
// Variable Expansion
// ================================

// Expand ${VAR} and ${VAR:default} variables
slay expand_variables(config map) map {
    sus result map = map_create();
    sus keys [tea] = map_keys(config);
    
    bestie i := 0; i < array_len(keys); i++ {
        sus key tea = keys[i];
        sus value tea = map_get(config, key).(tea);
        sus expanded_value tea = expand_string_variables(value, config);
        result = map_set(result, key, expanded_value);
    }
    
    damn result;
}

// Expand variables in a single string
slay expand_string_variables(value tea, config map) tea {
    sus result tea = value;
    sus start_pos normie = 0;
    
    bestie {
        sus var_start normie = string_find_from(result, "${", start_pos);
        
        lowkey var_start < 0 {
            break;
        }
        
        sus var_end normie = string_find_from(result, "}", var_start);
        
        lowkey var_end < 0 {
            break;
        }
        
        sus var_expr tea = string_slice(result, var_start + 2, var_end);
        sus var_name tea = var_expr;
        sus default_value tea = "";
        
        // Check for default value syntax VAR:default
        sus colon_pos normie = string_find(var_expr, ":");
        lowkey colon_pos > 0 {
            var_name = string_slice(var_expr, 0, colon_pos);
            default_value = string_slice(var_expr, colon_pos + 1, string_len(var_expr));
        }
        
        // Get variable value
        sus replacement tea = get_default(config, var_name, default_value);
        
        // Replace variable in result
        sus before tea = string_slice(result, 0, var_start);
        sus after tea = string_slice(result, var_end + 1, string_len(result));
        result = before + replacement + after;
        
        start_pos = var_start + string_len(replacement);
    }
    
    damn result;
}

// ================================
// Type Conversion Utilities
// ================================

// Get integer configuration value
slay get_int_value(config map, key tea, default_value normie) normie {
    sus value tea = get_value(config, key);
    
    lowkey string_is_empty(value) {
        damn default_value;
    }
    
    lowkey string_is_numeric(value) {
        damn string_to_int(value);
    }
    
    damn default_value;
}

// Get boolean configuration value
slay get_bool_value(config map, key tea, default_value lit) lit {
    sus value tea = get_value(config, key);
    
    lowkey string_is_empty(value) {
        damn default_value;
    }
    
    sus lower_value tea = string_to_lower(value);
    
    lowkey lower_value == "true" || lower_value == "yes" || lower_value == "on" || lower_value == "1" {
        damn based;
    }
    
    lowkey lower_value == "false" || lower_value == "no" || lower_value == "off" || lower_value == "0" {
        damn cap;
    }
    
    damn default_value;
}

// Get float configuration value
slay get_float_value(config map, key tea, default_value meal) meal {
    sus value tea = get_value(config, key);
    
    lowkey string_is_empty(value) {
        damn default_value;
    }
    
    lowkey string_is_numeric(value) {
        damn string_to_float(value);
    }
    
    damn default_value;
}
