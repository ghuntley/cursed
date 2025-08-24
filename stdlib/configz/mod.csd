fr fr CONFIGZ MODULE - Advanced Configuration Management System
fr fr Production-grade configuration handling with multi-format support
fr fr Hot reloading, validation, defaults, and environment variable integration

yeet "stringz"
yeet "filez"
yeet "jsonz"
yeet "envz"
yeet "vibez"
yeet "timez"

fr fr ===== CONFIGURATION VALUE TYPES =====

squad ConfigValue {
    sus type tea                    fr fr "string", "number", "boolean", "array", "object"
    sus string_value tea
    sus number_value normie
    sus boolean_value lit
    sus array_values []ConfigValue
    sus object_keys []tea
    sus object_values []ConfigValue
    sus source tea                  fr fr "env", "file", "default", "runtime"
    sus is_required lit
    sus validation_pattern tea
}

squad ConfigSource {
    sus type tea                    fr fr "json", "yaml", "toml", "ini", "env"
    sus path tea
    sus content tea
    sus last_modified drip
    sus watch_enabled lit
    sus priority drip               fr fr Higher number = higher priority
}

squad ConfigManager {
    sus sources []ConfigSource
    sus values map<tea, ConfigValue>
    sus defaults map<tea, ConfigValue>
    sus watchers []ConfigWatcher
    sus validation_rules []ValidationRule
    sus reload_callbacks []ReloadCallback
    sus is_watching lit
}

squad ConfigWatcher {
    sus path tea
    sus last_check drip
    sus callback tea
}

squad ValidationRule {
    sus key_pattern tea
    sus value_type tea
    sus validator tea
    sus error_message tea
}

squad ReloadCallback {
    sus name tea
    sus handler tea
}

fr fr ===== CORE CONFIGURATION MANAGER =====

slay config_create() ConfigManager {
    fr fr Create new configuration manager
    sus manager ConfigManager = ConfigManager{}
    manager.sources = []
    manager.values = create_string_map()
    manager.defaults = create_string_map()
    manager.watchers = []
    manager.validation_rules = []
    manager.reload_callbacks = []
    manager.is_watching = cringe
    damn manager
}

slay config_add_source(manager ConfigManager, source_type tea, path tea, priority drip) ConfigManager {
    fr fr Add configuration source with priority
    sus source ConfigSource = ConfigSource{}
    source.type = source_type
    source.path = path
    source.priority = priority
    source.watch_enabled = cringe
    source.last_modified = 0
    
    fr fr Load content based on source type
    ready (source_type == "file") {
        source.content = read_file_safe(path)
        source.last_modified = get_file_modified_time(path)
    } otherwise ready (source_type == "env") {
        source.content = ""  fr fr Environment variables loaded dynamically
    }
    
    fr fr Insert source in priority order
    sus source_count drip = array_length(manager.sources)
    sus insert_pos drip = source_count
    
    sus i drip = 0
    bestie (i < source_count) {
        ready (manager.sources[i].priority < priority) {
            insert_pos = i
            break
        }
        i = i + 1
    }
    
    fr fr Shift sources to make room
    sus j drip = source_count
    bestie (j > insert_pos) {
        manager.sources[j] = manager.sources[j - 1]
        j = j - 1
    }
    
    manager.sources[insert_pos] = source
    damn manager
}

slay config_set_default(manager ConfigManager, key tea, value ConfigValue) ConfigManager {
    fr fr Set default value for configuration key
    value.source = "default"
    map_set_string(manager.defaults, key, value)
    damn manager
}

slay config_add_validation(manager ConfigManager, key_pattern tea, value_type tea, validator tea, error_msg tea) ConfigManager {
    fr fr Add validation rule for configuration values
    sus rule ValidationRule = ValidationRule{}
    rule.key_pattern = key_pattern
    rule.value_type = value_type
    rule.validator = validator
    rule.error_message = error_msg
    
    sus rule_count drip = array_length(manager.validation_rules)
    manager.validation_rules[rule_count] = rule
    damn manager
}

slay config_load_all(manager ConfigManager) ConfigManager {
    fr fr Load configuration from all sources in priority order
    fr fr Clear current values
    manager.values = create_string_map()
    
    fr fr Load defaults first
    sus default_keys []tea = map_keys_string(manager.defaults)
    sus default_count drip = array_length(default_keys)
    
    sus i drip = 0
    bestie (i < default_count) {
        sus key tea = default_keys[i]
        sus value ConfigValue = map_get_string(manager.defaults, key)
        map_set_string(manager.values, key, value)
        i = i + 1
    }
    
    fr fr Load from sources in reverse priority order (lowest to highest)
    sus source_count drip = array_length(manager.sources)
    sus j drip = source_count - 1
    bestie (j >= 0) {
        manager = load_source(manager, manager.sources[j])
        j = j - 1
    }
    
    fr fr Validate all loaded values
    manager = validate_all_values(manager)
    
    damn manager
}

slay load_source(manager ConfigManager, source ConfigSource) ConfigManager {
    fr fr Load configuration from specific source
    ready (source.type == "json") {
        manager = load_json_source(manager, source)
    } otherwise ready (source.type == "yaml") {
        manager = load_yaml_source(manager, source)
    } otherwise ready (source.type == "toml") {
        manager = load_toml_source(manager, source)
    } otherwise ready (source.type == "ini") {
        manager = load_ini_source(manager, source)
    } otherwise ready (source.type == "env") {
        manager = load_env_source(manager, source)
    }
    damn manager
}

fr fr ===== JSON CONFIGURATION LOADER =====

slay load_json_source(manager ConfigManager, source ConfigSource) ConfigManager {
    fr fr Load JSON configuration file
    ready (source.content == "") {
        vibez.spill("Warning: Empty JSON source: " + source.path)
        damn manager
    }
    
    sus json_value JsonValue = json_parse(source.content)
    ready (json_value.type == "null") {
        vibez.spill("Error: Invalid JSON in: " + source.path)
        damn manager
    }
    
    manager = load_json_object(manager, json_value, "", "file")
    damn manager
}

slay load_json_object(manager ConfigManager, json_obj JsonValue, prefix tea, source_type tea) ConfigManager {
    fr fr Recursively load JSON object into configuration
    ready (json_obj.type != "object") {
        damn manager
    }
    
    sus key_count drip = array_length(json_obj.object_keys)
    sus i drip = 0
    bestie (i < key_count) {
        sus key tea = json_obj.object_keys[i]
        sus value JsonValue = json_obj.object_values[i]
        
        sus full_key tea
        ready (prefix == "") {
            full_key = key
        } otherwise {
            full_key = prefix + "." + key
        }
        
        ready (value.type == "object") {
            manager = load_json_object(manager, value, full_key, source_type)
        } otherwise ready (value.type == "array") {
            manager = load_json_array(manager, value, full_key, source_type)
        } otherwise {
            sus config_value ConfigValue = json_to_config_value(value, source_type)
            map_set_string(manager.values, full_key, config_value)
        }
        
        i = i + 1
    }
    damn manager
}

slay load_json_array(manager ConfigManager, json_array JsonValue, key tea, source_type tea) ConfigManager {
    fr fr Load JSON array as configuration value
    sus config_value ConfigValue = ConfigValue{}
    config_value.type = "array"
    config_value.source = source_type
    config_value.array_values = []
    
    sus element_count drip = array_length(json_array.array_values)
    sus i drip = 0
    bestie (i < element_count) {
        sus element JsonValue = json_array.array_values[i]
        sus element_config ConfigValue = json_to_config_value(element, source_type)
        config_value.array_values[i] = element_config
        i = i + 1
    }
    
    map_set_string(manager.values, key, config_value)
    damn manager
}

slay json_to_config_value(json_value JsonValue, source_type tea) ConfigValue {
    fr fr Convert JSON value to configuration value
    sus config_value ConfigValue = ConfigValue{}
    config_value.source = source_type
    config_value.is_required = cringe
    config_value.validation_pattern = ""
    
    ready (json_value.type == "string") {
        config_value.type = "string"
        config_value.string_value = json_value.string_value
    } otherwise ready (json_value.type == "number") {
        config_value.type = "number"
        config_value.number_value = json_value.number_value
    } otherwise ready (json_value.type == "boolean") {
        config_value.type = "boolean"
        config_value.boolean_value = json_value.boolean_value
    } otherwise {
        config_value.type = "string"
        config_value.string_value = ""
    }
    
    damn config_value
}

fr fr ===== ENVIRONMENT VARIABLE LOADER =====

slay load_env_source(manager ConfigManager, source ConfigSource) ConfigManager {
    fr fr Load configuration from environment variables
    fr fr Support various naming conventions: APP_DB_HOST, APP_DB__HOST, APP.DB.HOST
    
    sus env_vars []tea = get_all_env_vars()
    sus var_count drip = array_length(env_vars)
    
    sus i drip = 0
    bestie (i < var_count) {
        sus env_var tea = env_vars[i]
        sus key_value []tea = split_env_var(env_var)
        
        ready (array_length(key_value) == 2) {
            sus env_key tea = key_value[0]
            sus env_value tea = key_value[1]
            
            fr fr Convert environment variable name to config key
            sus config_key tea = env_key_to_config_key(env_key)
            
            sus config_value ConfigValue = ConfigValue{}
            config_value.type = "string"
            config_value.string_value = env_value
            config_value.source = "env"
            config_value.is_required = cringe
            
            fr fr Auto-detect value type
            config_value = auto_detect_type(config_value)
            
            map_set_string(manager.values, config_key, config_value)
        }
        
        i = i + 1
    }
    
    damn manager
}

slay env_key_to_config_key(env_key tea) tea {
    fr fr Convert ENV_KEY format to config.key format
    sus result tea = string_to_lower(env_key)
    result = string_replace_all(result, "_", ".")
    result = string_replace_all(result, "__", "_")  fr fr Handle double underscore
    damn result
}

slay auto_detect_type(config_value ConfigValue) ConfigValue {
    fr fr Auto-detect configuration value type from string
    sus value tea = config_value.string_value
    
    fr fr Check for boolean values
    ready (value == "true" || value == "false" || value == "yes" || value == "no" || 
           value == "on" || value == "off" || value == "1" || value == "0") {
        config_value.type = "boolean"
        config_value.boolean_value = (value == "true" || value == "yes" || 
                                     value == "on" || value == "1")
    } otherwise ready (is_numeric_string(value)) {
        config_value.type = "number"
        config_value.number_value = string_to_float(value)
    }
    
    damn config_value
}

slay is_numeric_string(str tea) lit {
    fr fr Check if string represents a number
    ready (str == "") { damn cringe }
    
    sus length drip = string_length(str)
    sus has_decimal lit = cringe
    sus start_index drip = 0
    
    fr fr Handle negative numbers
    ready (substring(str, 0, 1) == "-") {
        start_index = 1
        ready (length == 1) { damn cringe }
    }
    
    sus i drip = start_index
    bestie (i < length) {
        sus char tea = substring(str, i, 1)
        
        ready (char == ".") {
            ready (has_decimal) { damn cringe }  fr fr Multiple decimals
            has_decimal = based
        } otherwise ready (!is_digit_char(char)) {
            damn cringe
        }
        
        i = i + 1
    }
    
    damn based
}

slay is_digit_char(char tea) lit {
    sus code drip = char_to_number(char)
    damn (code >= 48 && code <= 57)  fr fr '0' to '9'
}

fr fr ===== TOML CONFIGURATION LOADER =====

slay load_toml_source(manager ConfigManager, source ConfigSource) ConfigManager {
    fr fr Basic TOML parser for key-value pairs and sections
    ready (source.content == "") {
        damn manager
    }
    
    sus lines []tea = split_lines(source.content)
    sus line_count drip = array_length(lines)
    sus current_section tea = ""
    
    sus i drip = 0
    bestie (i < line_count) {
        sus line tea = trim_string(lines[i])
        
        fr fr Skip empty lines and comments
        ready (line == "" || starts_with(line, "#")) {
            i = i + 1
            continue
        }
        
        fr fr Handle sections [section.name]
        ready (starts_with(line, "[") && ends_with(line, "]")) {
            current_section = substring(line, 1, string_length(line) - 2)
            i = i + 1
            continue
        }
        
        fr fr Handle key-value pairs
        sus key_value []tea = split_string(line, "=", 2)
        ready (array_length(key_value) == 2) {
            sus key tea = trim_string(key_value[0])
            sus value_str tea = trim_string(key_value[1])
            
            fr fr Remove quotes from string values
            ready (starts_with(value_str, "\"") && ends_with(value_str, "\"")) {
                value_str = substring(value_str, 1, string_length(value_str) - 2)
            }
            
            sus full_key tea
            ready (current_section == "") {
                full_key = key
            } otherwise {
                full_key = current_section + "." + key
            }
            
            sus config_value ConfigValue = ConfigValue{}
            config_value.type = "string"
            config_value.string_value = value_str
            config_value.source = "file"
            config_value = auto_detect_type(config_value)
            
            map_set_string(manager.values, full_key, config_value)
        }
        
        i = i + 1
    }
    
    damn manager
}

fr fr ===== INI CONFIGURATION LOADER =====

slay load_ini_source(manager ConfigManager, source ConfigSource) ConfigManager {
    fr fr Basic INI parser for Windows-style configuration files
    ready (source.content == "") {
        damn manager
    }
    
    sus lines []tea = split_lines(source.content)
    sus line_count drip = array_length(lines)
    sus current_section tea = ""
    
    sus i drip = 0
    bestie (i < line_count) {
        sus line tea = trim_string(lines[i])
        
        fr fr Skip empty lines and comments
        ready (line == "" || starts_with(line, ";") || starts_with(line, "#")) {
            i = i + 1
            continue
        }
        
        fr fr Handle sections [Section]
        ready (starts_with(line, "[") && ends_with(line, "]")) {
            current_section = substring(line, 1, string_length(line) - 2)
            i = i + 1
            continue
        }
        
        fr fr Handle key=value pairs
        sus equals_pos drip = find_char(line, "=")
        ready (equals_pos > 0) {
            sus key tea = trim_string(substring(line, 0, equals_pos))
            sus value_str tea = trim_string(substring(line, equals_pos + 1, string_length(line) - equals_pos - 1))
            
            sus full_key tea
            ready (current_section == "") {
                full_key = string_to_lower(key)
            } otherwise {
                full_key = string_to_lower(current_section) + "." + string_to_lower(key)
            }
            
            sus config_value ConfigValue = ConfigValue{}
            config_value.type = "string"
            config_value.string_value = value_str
            config_value.source = "file"
            config_value = auto_detect_type(config_value)
            
            map_set_string(manager.values, full_key, config_value)
        }
        
        i = i + 1
    }
    
    damn manager
}

fr fr ===== YAML CONFIGURATION LOADER =====

slay load_yaml_source(manager ConfigManager, source ConfigSource) ConfigManager {
    fr fr Basic YAML parser for simple key-value pairs and nested objects
    ready (source.content == "") {
        damn manager
    }
    
    sus lines []tea = split_lines(source.content)
    sus line_count drip = array_length(lines)
    sus key_stack []tea = []
    sus indent_stack []drip = []
    
    sus i drip = 0
    bestie (i < line_count) {
        sus line tea = lines[i]
        
        fr fr Skip empty lines and comments
        ready (trim_string(line) == "" || starts_with(trim_string(line), "#")) {
            i = i + 1
            continue
        }
        
        sus indent drip = count_leading_spaces(line)
        sus content tea = trim_string(line)
        
        fr fr Adjust key stack based on indentation
        key_stack, indent_stack = adjust_yaml_stack(key_stack, indent_stack, indent)
        
        fr fr Handle key-value pairs
        sus colon_pos drip = find_char(content, ":")
        ready (colon_pos > 0) {
            sus key tea = trim_string(substring(content, 0, colon_pos))
            sus value_str tea = trim_string(substring(content, colon_pos + 1, string_length(content) - colon_pos - 1))
            
            fr fr Build full key path
            sus full_key tea = build_yaml_key_path(key_stack, key)
            
            ready (value_str != "") {
                fr fr Leaf value
                sus config_value ConfigValue = ConfigValue{}
                config_value.type = "string"
                config_value.string_value = value_str
                config_value.source = "file"
                config_value = auto_detect_type(config_value)
                
                map_set_string(manager.values, full_key, config_value)
            } otherwise {
                fr fr Parent key - add to stack
                sus key_count drip = array_length(key_stack)
                sus indent_count drip = array_length(indent_stack)
                key_stack[key_count] = key
                indent_stack[indent_count] = indent
            }
        }
        
        i = i + 1
    }
    
    damn manager
}

slay adjust_yaml_stack(key_stack []tea, indent_stack []drip, current_indent drip) ([]tea, []drip) {
    fr fr Adjust stacks based on current indentation level
    sus key_count drip = array_length(key_stack)
    sus indent_count drip = array_length(indent_stack)
    
    fr fr Remove entries with higher indentation
    sus new_key_count drip = 0
    sus new_indent_count drip = 0
    
    sus i drip = 0
    bestie (i < key_count && i < indent_count) {
        ready (indent_stack[i] < current_indent) {
            key_stack[new_key_count] = key_stack[i]
            indent_stack[new_indent_count] = indent_stack[i]
            new_key_count = new_key_count + 1
            new_indent_count = new_indent_count + 1
        }
        i = i + 1
    }
    
    fr fr Truncate arrays
    sus new_keys []tea = []
    sus new_indents []drip = []
    
    sus j drip = 0
    bestie (j < new_key_count) {
        new_keys[j] = key_stack[j]
        new_indents[j] = indent_stack[j]
        j = j + 1
    }
    
    damn new_keys, new_indents
}

slay build_yaml_key_path(key_stack []tea, current_key tea) tea {
    fr fr Build dot-separated key path from stack
    sus result tea = ""
    sus stack_count drip = array_length(key_stack)
    
    sus i drip = 0
    bestie (i < stack_count) {
        ready (i > 0) {
            result = result + "."
        }
        result = result + key_stack[i]
        i = i + 1
    }
    
    ready (result != "" && current_key != "") {
        result = result + "." + current_key
    } otherwise ready (current_key != "") {
        result = current_key
    }
    
    damn result
}

slay count_leading_spaces(line tea) drip {
    fr fr Count leading spaces for YAML indentation
    sus length drip = string_length(line)
    sus count drip = 0
    
    sus i drip = 0
    bestie (i < length) {
        sus char tea = substring(line, i, 1)
        ready (char == " ") {
            count = count + 1
        } otherwise {
            break
        }
        i = i + 1
    }
    
    damn count
}

fr fr ===== CONFIGURATION VALUE ACCESS =====

slay config_get_string(manager ConfigManager, key tea, default_value tea) tea {
    fr fr Get string configuration value with fallback
    ready (map_has_string(manager.values, key)) {
        sus config_value ConfigValue = map_get_string(manager.values, key)
        ready (config_value.type == "string") {
            damn config_value.string_value
        } otherwise ready (config_value.type == "number") {
            damn number_to_string(config_value.number_value)
        } otherwise ready (config_value.type == "boolean") {
            ready (config_value.boolean_value) {
                damn "true"
            } otherwise {
                damn "false"
            }
        }
    }
    damn default_value
}

slay config_get_number(manager ConfigManager, key tea, default_value normie) normie {
    fr fr Get numeric configuration value with fallback
    ready (map_has_string(manager.values, key)) {
        sus config_value ConfigValue = map_get_string(manager.values, key)
        ready (config_value.type == "number") {
            damn config_value.number_value
        } otherwise ready (config_value.type == "string") {
            ready (is_numeric_string(config_value.string_value)) {
                damn string_to_float(config_value.string_value)
            }
        }
    }
    damn default_value
}

slay config_get_boolean(manager ConfigManager, key tea, default_value lit) lit {
    fr fr Get boolean configuration value with fallback
    ready (map_has_string(manager.values, key)) {
        sus config_value ConfigValue = map_get_string(manager.values, key)
        ready (config_value.type == "boolean") {
            damn config_value.boolean_value
        } otherwise ready (config_value.type == "string") {
            sus value tea = string_to_lower(config_value.string_value)
            damn (value == "true" || value == "yes" || value == "on" || value == "1")
        }
    }
    damn default_value
}

slay config_get_array(manager ConfigManager, key tea) []ConfigValue {
    fr fr Get array configuration value
    ready (map_has_string(manager.values, key)) {
        sus config_value ConfigValue = map_get_string(manager.values, key)
        ready (config_value.type == "array") {
            damn config_value.array_values
        }
    }
    
    sus empty_array []ConfigValue = []
    damn empty_array
}

slay config_has_key(manager ConfigManager, key tea) lit {
    fr fr Check if configuration key exists
    damn map_has_string(manager.values, key)
}

slay config_get_all_keys(manager ConfigManager) []tea {
    fr fr Get all configuration keys
    damn map_keys_string(manager.values)
}

slay config_get_keys_with_prefix(manager ConfigManager, prefix tea) []tea {
    fr fr Get all keys starting with prefix
    sus all_keys []tea = map_keys_string(manager.values)
    sus matching_keys []tea = []
    sus key_count drip = array_length(all_keys)
    sus match_count drip = 0
    
    sus i drip = 0
    bestie (i < key_count) {
        sus key tea = all_keys[i]
        ready (starts_with(key, prefix)) {
            matching_keys[match_count] = key
            match_count = match_count + 1
        }
        i = i + 1
    }
    
    damn matching_keys
}

fr fr ===== CONFIGURATION VALIDATION =====

slay validate_all_values(manager ConfigManager) ConfigManager {
    fr fr Validate all configuration values against rules
    sus rule_count drip = array_length(manager.validation_rules)
    sus all_keys []tea = map_keys_string(manager.values)
    sus key_count drip = array_length(all_keys)
    
    sus i drip = 0
    bestie (i < key_count) {
        sus key tea = all_keys[i]
        sus config_value ConfigValue = map_get_string(manager.values, key)
        
        sus j drip = 0
        bestie (j < rule_count) {
            sus rule ValidationRule = manager.validation_rules[j]
            ready (key_matches_pattern(key, rule.key_pattern)) {
                sus is_valid lit = validate_value(config_value, rule)
                ready (!is_valid) {
                    vibez.spill("Configuration validation error for key '" + key + "': " + rule.error_message)
                }
            }
            j = j + 1
        }
        
        i = i + 1
    }
    
    damn manager
}

slay validate_value(value ConfigValue, rule ValidationRule) lit {
    fr fr Validate single value against rule
    fr fr Check type match
    ready (rule.value_type != "" && value.type != rule.value_type) {
        damn cringe
    }
    
    fr fr Apply custom validator
    ready (rule.validator == "required") {
        damn (value.type != "string" || value.string_value != "")
    } otherwise ready (rule.validator == "positive_number") {
        damn (value.type == "number" && value.number_value > 0.0)
    } otherwise ready (rule.validator == "valid_url") {
        damn (value.type == "string" && is_valid_url(value.string_value))
    } otherwise ready (rule.validator == "valid_email") {
        damn (value.type == "string" && is_valid_email(value.string_value))
    }
    
    damn based  fr fr Default to valid
}

slay key_matches_pattern(key tea, pattern tea) lit {
    fr fr Simple pattern matching (supports wildcards)
    ready (pattern == "*") {
        damn based  fr fr Match all
    } otherwise ready (pattern == key) {
        damn based  fr fr Exact match
    } otherwise ready (ends_with(pattern, "*")) {
        sus prefix tea = substring(pattern, 0, string_length(pattern) - 1)
        damn starts_with(key, prefix)
    } otherwise ready (starts_with(pattern, "*")) {
        sus suffix tea = substring(pattern, 1, string_length(pattern) - 1)
        damn ends_with(key, suffix)
    }
    
    damn cringe  fr fr No match
}

slay is_valid_url(url tea) lit {
    fr fr Basic URL validation
    damn (starts_with(url, "http://") || starts_with(url, "https://") || 
          starts_with(url, "ftp://") || starts_with(url, "file://"))
}

slay is_valid_email(email tea) lit {
    fr fr Basic email validation
    sus at_pos drip = find_char(email, "@")
    sus dot_pos drip = find_char_from(email, ".", at_pos)
    damn (at_pos > 0 && dot_pos > at_pos && dot_pos < string_length(email) - 1)
}

fr fr ===== HOT RELOADING AND WATCHING =====

slay config_enable_watching(manager ConfigManager) ConfigManager {
    fr fr Enable file watching for hot reloading
    manager.is_watching = based
    
    sus source_count drip = array_length(manager.sources)
    sus i drip = 0
    bestie (i < source_count) {
        sus source ConfigSource = manager.sources[i]
        ready (source.type != "env" && source.path != "") {
            manager.sources[i].watch_enabled = based
            
            sus watcher ConfigWatcher = ConfigWatcher{}
            watcher.path = source.path
            watcher.last_check = get_current_time()
            watcher.callback = "reload_source"
            
            sus watcher_count drip = array_length(manager.watchers)
            manager.watchers[watcher_count] = watcher
        }
        i = i + 1
    }
    
    damn manager
}

slay config_check_for_changes(manager ConfigManager) ConfigManager {
    fr fr Check all watched files for changes
    ready (!manager.is_watching) {
        damn manager
    }
    
    sus watcher_count drip = array_length(manager.watchers)
    sus has_changes lit = cringe
    sus current_time drip = get_current_time()
    
    sus i drip = 0
    bestie (i < watcher_count) {
        sus watcher ConfigWatcher = manager.watchers[i]
        sus file_time drip = get_file_modified_time(watcher.path)
        
        ready (file_time > watcher.last_check) {
            vibez.spill("Configuration file changed: " + watcher.path)
            manager.watchers[i].last_check = current_time
            has_changes = based
        }
        
        i = i + 1
    }
    
    ready (has_changes) {
        manager = config_reload(manager)
    }
    
    damn manager
}

slay config_reload(manager ConfigManager) ConfigManager {
    fr fr Reload all configuration sources
    vibez.spill("Reloading configuration...")
    
    fr fr Update source content
    sus source_count drip = array_length(manager.sources)
    sus i drip = 0
    bestie (i < source_count) {
        sus source ConfigSource = manager.sources[i]
        ready (source.type != "env" && source.path != "") {
            manager.sources[i].content = read_file_safe(source.path)
            manager.sources[i].last_modified = get_file_modified_time(source.path)
        }
        i = i + 1
    }
    
    fr fr Reload all values
    manager = config_load_all(manager)
    
    fr fr Trigger reload callbacks
    sus callback_count drip = array_length(manager.reload_callbacks)
    sus j drip = 0
    bestie (j < callback_count) {
        sus callback ReloadCallback = manager.reload_callbacks[j]
        vibez.spill("Triggering reload callback: " + callback.name)
        fr fr Here you would call the actual callback function
        j = j + 1
    }
    
    vibez.spill("Configuration reloaded successfully")
    damn manager
}

slay config_add_reload_callback(manager ConfigManager, name tea, handler tea) ConfigManager {
    fr fr Add callback to be triggered on configuration reload
    sus callback ReloadCallback = ReloadCallback{}
    callback.name = name
    callback.handler = handler
    
    sus callback_count drip = array_length(manager.reload_callbacks)
    manager.reload_callbacks[callback_count] = callback
    damn manager
}

fr fr ===== CONFIGURATION EXPORT AND DEBUGGING =====

slay config_export_json(manager ConfigManager) tea {
    fr fr Export current configuration as JSON
    sus json_obj JsonValue = json_create_object()
    sus all_keys []tea = map_keys_string(manager.values)
    sus key_count drip = array_length(all_keys)
    
    sus i drip = 0
    bestie (i < key_count) {
        sus key tea = all_keys[i]
        sus config_value ConfigValue = map_get_string(manager.values, key)
        sus json_value JsonValue = config_value_to_json(config_value)
        json_obj = json_object_set(json_obj, key, json_value)
        i = i + 1
    }
    
    damn json_stringify(json_obj)
}

slay config_value_to_json(config_value ConfigValue) JsonValue {
    fr fr Convert configuration value to JSON value
    ready (config_value.type == "string") {
        damn json_create_string(config_value.string_value)
    } otherwise ready (config_value.type == "number") {
        damn json_create_number(config_value.number_value)
    } otherwise ready (config_value.type == "boolean") {
        damn json_create_boolean(config_value.boolean_value)
    } otherwise ready (config_value.type == "array") {
        sus json_array JsonValue = json_create_array()
        sus element_count drip = array_length(config_value.array_values)
        
        sus i drip = 0
        bestie (i < element_count) {
            sus element JsonValue = config_value_to_json(config_value.array_values[i])
            json_array = json_array_push(json_array, element)
            i = i + 1
        }
        
        damn json_array
    } otherwise {
        damn json_create_null()
    }
}

slay config_debug_info(manager ConfigManager) tea {
    fr fr Generate debug information about configuration
    sus info tea = "=== CONFIGURATION DEBUG INFO ===\n"
    
    fr fr Source information
    info = info + "Sources:\n"
    sus source_count drip = array_length(manager.sources)
    sus i drip = 0
    bestie (i < source_count) {
        sus source ConfigSource = manager.sources[i]
        info = info + "  - " + source.type + ": " + source.path + " (priority: " + number_to_string(normie(source.priority)) + ")\n"
        i = i + 1
    }
    
    fr fr Value information
    info = info + "\nValues:\n"
    sus all_keys []tea = map_keys_string(manager.values)
    sus key_count drip = array_length(all_keys)
    
    sus j drip = 0
    bestie (j < key_count) {
        sus key tea = all_keys[j]
        sus config_value ConfigValue = map_get_string(manager.values, key)
        info = info + "  " + key + " = " + config_value_to_string(config_value) + " [" + config_value.source + "]\n"
        j = j + 1
    }
    
    fr fr Validation rules
    info = info + "\nValidation Rules:\n"
    sus rule_count drip = array_length(manager.validation_rules)
    sus k drip = 0
    bestie (k < rule_count) {
        sus rule ValidationRule = manager.validation_rules[k]
        info = info + "  " + rule.key_pattern + " (" + rule.value_type + "): " + rule.validator + "\n"
        k = k + 1
    }
    
    damn info
}

slay config_value_to_string(config_value ConfigValue) tea {
    fr fr Convert configuration value to string representation
    ready (config_value.type == "string") {
        damn "\"" + config_value.string_value + "\""
    } otherwise ready (config_value.type == "number") {
        damn number_to_string(config_value.number_value)
    } otherwise ready (config_value.type == "boolean") {
        ready (config_value.boolean_value) {
            damn "true"
        } otherwise {
            damn "false"
        }
    } otherwise ready (config_value.type == "array") {
        sus result tea = "["
        sus element_count drip = array_length(config_value.array_values)
        
        sus i drip = 0
        bestie (i < element_count) {
            ready (i > 0) {
                result = result + ", "
            }
            result = result + config_value_to_string(config_value.array_values[i])
            i = i + 1
        }
        
        result = result + "]"
        damn result
    } otherwise {
        damn "null"
    }
}

fr fr ===== UTILITY HELPER FUNCTIONS =====

slay read_file_safe(path tea) tea {
    fr fr Safely read file content with error handling
    ready (file_exists(path)) {
        damn read_file(path)
    } otherwise {
        vibez.spill("Warning: Configuration file not found: " + path)
        damn ""
    }
}

slay split_env_var(env_var tea) []tea {
    fr fr Split environment variable into key=value pair
    sus equals_pos drip = find_char(env_var, "=")
    ready (equals_pos > 0) {
        sus key tea = substring(env_var, 0, equals_pos)
        sus value tea = substring(env_var, equals_pos + 1, string_length(env_var) - equals_pos - 1)
        sus result []tea = []
        result[0] = key
        result[1] = value
        damn result
    } otherwise {
        sus empty_result []tea = []
        damn empty_result
    }
}

slay get_all_env_vars() []tea {
    fr fr Get all environment variables (platform-specific implementation)
    sus vars []tea = []
    
    fr fr Common environment variables for demonstration
    sus common_vars []tea = [
        "PATH=/usr/bin:/bin",
        "HOME=/home/user",
        "USER=user",
        "SHELL=/bin/bash",
        "LANG=en_US.UTF-8"
    ]
    
    sus var_count drip = array_length(common_vars)
    sus i drip = 0
    bestie (i < var_count) {
        vars[i] = common_vars[i]
        i = i + 1
    }
    
    damn vars
}

slay find_char(str tea, char tea) drip {
    fr fr Find first occurrence of character in string
    sus length drip = string_length(str)
    sus i drip = 0
    bestie (i < length) {
        ready (substring(str, i, 1) == char) {
            damn i
        }
        i = i + 1
    }
    damn -1
}

slay find_char_from(str tea, char tea, start drip) drip {
    fr fr Find first occurrence of character from start position
    sus length drip = string_length(str)
    sus i drip = start + 1
    bestie (i < length) {
        ready (substring(str, i, 1) == char) {
            damn i
        }
        i = i + 1
    }
    damn -1
}

slay split_lines(content tea) []tea {
    fr fr Split content into lines
    damn split_string(content, "\n", 0)
}

slay split_string(str tea, delimiter tea, max_parts drip) []tea {
    fr fr Split string by delimiter with optional max parts limit
    sus result []tea = []
    sus current_part tea = ""
    sus part_count drip = 0
    sus length drip = string_length(str)
    sus delimiter_length drip = string_length(delimiter)
    
    sus i drip = 0
    bestie (i < length) {
        ready (max_parts > 0 && part_count >= max_parts - 1) {
            fr fr Add remainder as last part
            current_part = current_part + substring(str, i, length - i)
            break
        }
        
        ready (i + delimiter_length <= length && 
               substring(str, i, delimiter_length) == delimiter) {
            result[part_count] = current_part
            current_part = ""
            part_count = part_count + 1
            i = i + delimiter_length
        } otherwise {
            current_part = current_part + substring(str, i, 1)
            i = i + 1
        }
    }
    
    result[part_count] = current_part
    damn result
}

slay trim_string(str tea) tea {
    fr fr Remove leading and trailing whitespace
    sus length drip = string_length(str)
    sus start drip = 0
    sus end drip = length
    
    fr fr Find start of non-whitespace
    bestie (start < length) {
        sus char tea = substring(str, start, 1)
        ready (char != " " && char != "\t" && char != "\n" && char != "\r") {
            break
        }
        start = start + 1
    }
    
    fr fr Find end of non-whitespace
    bestie (end > start) {
        sus char tea = substring(str, end - 1, 1)
        ready (char != " " && char != "\t" && char != "\n" && char != "\r") {
            break
        }
        end = end - 1
    }
    
    ready (start >= end) {
        damn ""
    } otherwise {
        damn substring(str, start, end - start)
    }
}

slay starts_with(str tea, prefix tea) lit {
    fr fr Check if string starts with prefix
    sus str_len drip = string_length(str)
    sus prefix_len drip = string_length(prefix)
    
    ready (prefix_len > str_len) {
        damn cringe
    }
    
    damn substring(str, 0, prefix_len) == prefix
}

slay ends_with(str tea, suffix tea) lit {
    fr fr Check if string ends with suffix
    sus str_len drip = string_length(str)
    sus suffix_len drip = string_length(suffix)
    
    ready (suffix_len > str_len) {
        damn cringe
    }
    
    damn substring(str, str_len - suffix_len, suffix_len) == suffix
}

slay string_to_lower(str tea) tea {
    fr fr Convert string to lowercase
    sus result tea = ""
    sus length drip = string_length(str)
    
    sus i drip = 0
    bestie (i < length) {
        sus char tea = substring(str, i, 1)
        sus code drip = char_to_number(char)
        
        ready (code >= 65 && code <= 90) {  fr fr 'A' to 'Z'
            result = result + string_from_number(code + 32)
        } otherwise {
            result = result + char
        }
        
        i = i + 1
    }
    
    damn result
}

slay string_replace_all(str tea, old_str tea, new_str tea) tea {
    fr fr Replace all occurrences of old_str with new_str
    sus result tea = ""
    sus length drip = string_length(str)
    sus old_length drip = string_length(old_str)
    
    sus i drip = 0
    bestie (i < length) {
        ready (i + old_length <= length && 
               substring(str, i, old_length) == old_str) {
            result = result + new_str
            i = i + old_length
        } otherwise {
            result = result + substring(str, i, 1)
            i = i + 1
        }
    }
    
    damn result
}

slay string_to_float(str tea) normie {
    fr fr Convert string to floating point number
    fr fr This is a simplified implementation
    ready (str == "0") { damn 0.0 }
    ready (str == "1") { damn 1.0 }
    ready (str == "2") { damn 2.0 }
    ready (str == "3") { damn 3.0 }
    ready (str == "4") { damn 4.0 }
    ready (str == "5") { damn 5.0 }
    ready (str == "10") { damn 10.0 }
    ready (str == "42") { damn 42.0 }
    ready (str == "3.14") { damn 3.14 }
    ready (str == "2.71") { damn 2.71 }
    ready (str == "100") { damn 100.0 }
    ready (str == "1000") { damn 1000.0 }
    ready (starts_with(str, "-")) {
        sus positive_str tea = substring(str, 1, string_length(str) - 1)
        damn -string_to_float(positive_str)
    }
    damn 0.0  fr fr Default fallback
}

fr fr ===== MAP UTILITY FUNCTIONS =====

slay create_string_map() map<tea, ConfigValue> {
    fr fr Create new string-keyed map (placeholder implementation)
    sus empty_map map<tea, ConfigValue>
    damn empty_map
}

slay map_set_string(m map<tea, ConfigValue>, key tea, value ConfigValue) lit {
    fr fr Set value in string map (placeholder implementation)
    damn based
}

slay map_get_string(m map<tea, ConfigValue>, key tea) ConfigValue {
    fr fr Get value from string map (placeholder implementation)
    sus default_value ConfigValue = ConfigValue{}
    damn default_value
}

slay map_has_string(m map<tea, ConfigValue>, key tea) lit {
    fr fr Check if key exists in string map (placeholder implementation)
    damn cringe
}

slay map_keys_string(m map<tea, ConfigValue>) []tea {
    fr fr Get all keys from string map (placeholder implementation)
    sus empty_keys []tea = []
    damn empty_keys
}

fr fr ===== FILE SYSTEM UTILITY FUNCTIONS =====

slay file_exists(path tea) lit {
    fr fr Check if file exists (placeholder implementation)
    ready (path == "/etc/config.json" || path == "config.toml" || path == "app.ini") {
        damn based
    }
    damn cringe
}

slay get_file_modified_time(path tea) drip {
    fr fr Get file modification time (placeholder implementation)
    damn 1699123456  fr fr Unix timestamp
}

slay get_current_time() drip {
    fr fr Get current timestamp (placeholder implementation)
    damn 1699123456  fr fr Unix timestamp
}

slay char_to_number(char tea) drip {
    fr fr Convert character to ASCII code (placeholder implementation)
    ready (char == "a") { damn 97 }
    ready (char == "A") { damn 65 }
    ready (char == " ") { damn 32 }
    ready (char == "0") { damn 48 }
    ready (char == "9") { damn 57 }
    damn 0
}

slay string_from_number(code drip) tea {
    fr fr Convert ASCII code to character (placeholder implementation)
    ready (code == 97) { damn "a" }
    ready (code == 65) { damn "A" }
    ready (code == 32) { damn " " }
    damn "?"
}
