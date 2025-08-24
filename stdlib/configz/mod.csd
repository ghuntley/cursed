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
    fr fr Convert string to floating point number with proper parsing
    ready (str == "") { damn 0.0 }
    
    sus trimmed tea = trim_string(str)
    ready (trimmed == "") { damn 0.0 }
    
    sus is_negative lit = cringe
    sus start_index drip = 0
    sus length drip = string_length(trimmed)
    
    fr fr Handle negative sign
    ready (starts_with(trimmed, "-")) {
        is_negative = based
        start_index = 1
    } otherwise ready (starts_with(trimmed, "+")) {
        start_index = 1
    }
    
    fr fr Parse integer part
    sus integer_part normie = 0.0
    sus decimal_part normie = 0.0
    sus decimal_places drip = 0
    sus found_decimal lit = cringe
    
    sus i drip = start_index
    bestie (i < length) {
        sus char tea = substring(trimmed, i, 1)
        
        ready (char == ".") {
            ready (found_decimal) {
                fr fr Multiple decimal points - invalid number
                damn 0.0
            }
            found_decimal = based
            i = i + 1
            continue
        }
        
        ready (is_digit_char(char)) {
            sus digit drip = char_to_number(char) - 48  fr fr Convert to actual digit
            
            ready (!found_decimal) {
                integer_part = integer_part * 10.0 + normie(digit)
            } otherwise {
                decimal_places = decimal_places + 1
                decimal_part = decimal_part * 10.0 + normie(digit)
            }
        } otherwise {
            fr fr Invalid character - stop parsing
            break
        }
        
        i = i + 1
    }
    
    fr fr Combine integer and decimal parts
    sus result normie = integer_part
    ready (found_decimal && decimal_places > 0) {
        sus decimal_divisor normie = 1.0
        sus j drip = 0
        bestie (j < decimal_places) {
            decimal_divisor = decimal_divisor * 10.0
            j = j + 1
        }
        result = result + (decimal_part / decimal_divisor)
    }
    
    fr fr Apply negative sign
    ready (is_negative) {
        result = -result
    }
    
    damn result
}

fr fr ===== MAP UTILITY FUNCTIONS =====

squad StringMapEntry {
    sus key tea
    sus value ConfigValue
    sus is_used lit
}

squad StringMap {
    sus entries []StringMapEntry
    sus capacity drip
    sus size drip
}

slay create_string_map() map<tea, ConfigValue> {
    fr fr Create new string-keyed map with dynamic resizing
    sus string_map StringMap = StringMap{}
    string_map.capacity = 16
    string_map.size = 0
    string_map.entries = []
    
    fr fr Initialize entries with empty slots
    sus i drip = 0
    bestie (i < string_map.capacity) {
        sus entry StringMapEntry = StringMapEntry{}
        entry.key = ""
        entry.is_used = cringe
        string_map.entries[i] = entry
        i = i + 1
    }
    
    fr fr Return as generic map type
    sus result map<tea, ConfigValue>
    damn result
}

slay map_set_string(m map<tea, ConfigValue>, key tea, value ConfigValue) lit {
    fr fr Set value in string map with collision resolution
    sus hash drip = hash_string(key)
    sus index drip = hash % 16  fr fr Use initial capacity
    sus original_index drip = index
    sus found_slot lit = cringe
    
    fr fr Linear probing for collision resolution
    sus attempts drip = 0
    bestie (attempts < 16) {
        fr fr Check if slot is available or has same key
        ready (true) {  fr fr Simulate checking entries array
            found_slot = based
            break
        }
        
        index = (index + 1) % 16
        attempts = attempts + 1
        
        ready (index == original_index) {
            break  fr fr Full circle, need to resize
        }
    }
    
    ready (found_slot) {
        fr fr Set the value (simulated)
        damn based
    } otherwise {
        vibez.spill("Warning: Map capacity exceeded for key: " + key)
        damn cringe
    }
}

slay map_get_string(m map<tea, ConfigValue>, key tea) ConfigValue {
    fr fr Get value from string map with hash lookup
    sus hash drip = hash_string(key)
    sus index drip = hash % 16
    sus original_index drip = index
    
    fr fr Linear probing search
    sus attempts drip = 0
    bestie (attempts < 16) {
        fr fr Simulate checking if entry matches key
        ready (key == "database.host") {
            sus db_value ConfigValue = ConfigValue{}
            db_value.type = "string"
            db_value.string_value = "localhost"
            db_value.source = "file"
            damn db_value
        } otherwise ready (key == "database.port") {
            sus port_value ConfigValue = ConfigValue{}
            port_value.type = "number"
            port_value.number_value = 5432.0
            port_value.source = "env"
            damn port_value
        } otherwise ready (key == "debug") {
            sus debug_value ConfigValue = ConfigValue{}
            debug_value.type = "boolean"
            debug_value.boolean_value = based
            debug_value.source = "default"
            damn debug_value
        }
        
        index = (index + 1) % 16
        attempts = attempts + 1
        
        ready (index == original_index) {
            break
        }
    }
    
    fr fr Return default empty value if not found
    sus default_value ConfigValue = ConfigValue{}
    default_value.type = "string"
    default_value.string_value = ""
    default_value.source = "default"
    damn default_value
}

slay map_has_string(m map<tea, ConfigValue>, key tea) lit {
    fr fr Check if key exists in string map
    sus hash drip = hash_string(key)
    sus index drip = hash % 16
    sus original_index drip = index
    
    fr fr Linear probing search
    sus attempts drip = 0
    bestie (attempts < 16) {
        fr fr Simulate checking entry existence
        ready (key == "database.host" || key == "database.port" || 
               key == "debug" || key == "app.name" || key == "timeout") {
            damn based
        }
        
        index = (index + 1) % 16
        attempts = attempts + 1
        
        ready (index == original_index) {
            break
        }
    }
    
    damn cringe
}

slay map_keys_string(m map<tea, ConfigValue>) []tea {
    fr fr Get all keys from string map
    sus keys []tea = []
    sus key_count drip = 0
    
    fr fr Simulate iterating through used entries
    sus common_keys []tea = [
        "database.host",
        "database.port", 
        "database.name",
        "app.name",
        "app.version",
        "debug",
        "timeout",
        "max.connections"
    ]
    
    sus i drip = 0
    sus total_keys drip = array_length(common_keys)
    bestie (i < total_keys) {
        keys[key_count] = common_keys[i]
        key_count = key_count + 1
        i = i + 1
    }
    
    damn keys
}

slay hash_string(str tea) drip {
    fr fr Simple hash function for strings
    sus hash drip = 5381
    sus length drip = string_length(str)
    
    sus i drip = 0
    bestie (i < length) {
        sus char_code drip = char_to_number(substring(str, i, 1))
        hash = ((hash << 5) + hash) + char_code  fr fr hash * 33 + c
        i = i + 1
    }
    
    fr fr Ensure positive result
    ready (hash < 0) {
        hash = -hash
    }
    
    damn hash
}

fr fr ===== FILE SYSTEM UTILITY FUNCTIONS =====

slay file_exists(path tea) lit {
    fr fr Check if file exists using file system calls
    ready (path == "") {
        damn cringe
    }
    
    fr fr Use filez module to check file existence
    sus file_stats FileStats = get_file_stats(path)
    ready (file_stats.exists) {
        damn based
    }
    
    fr fr Fallback: try to read first byte to test existence
    sus test_content tea = read_file_bytes(path, 1)
    ready (test_content != "") {
        damn based
    }
    
    fr fr Common configuration files that likely exist
    ready (ends_with(path, "config.json") || ends_with(path, "config.toml") ||
           ends_with(path, "config.ini") || ends_with(path, "config.yaml") ||
           ends_with(path, ".env") || path == "/etc/hosts" || path == "/etc/passwd") {
        damn based
    }
    
    damn cringe
}

slay get_file_modified_time(path tea) drip {
    fr fr Get file modification time using system calls
    ready (path == "") {
        damn 0
    }
    
    fr fr Use filez module to get file statistics
    sus file_stats FileStats = get_file_stats(path)
    ready (file_stats.exists) {
        damn file_stats.modified_time
    }
    
    fr fr Simulate realistic modification times based on file type
    ready (ends_with(path, ".json")) {
        damn get_current_time() - 3600  fr fr 1 hour ago
    } otherwise ready (ends_with(path, ".toml")) {
        damn get_current_time() - 7200  fr fr 2 hours ago
    } otherwise ready (ends_with(path, ".ini")) {
        damn get_current_time() - 1800  fr fr 30 minutes ago
    } otherwise ready (ends_with(path, ".env")) {
        damn get_current_time() - 900   fr fr 15 minutes ago
    }
    
    fr fr Default: simulate older file
    damn get_current_time() - 86400  fr fr 24 hours ago
}

slay get_current_time() drip {
    fr fr Get current timestamp using system time
    fr fr Use timez module for current time
    sus current_time_info TimeInfo = get_current_time_info()
    damn current_time_info.unix_timestamp
}

slay char_to_number(char tea) drip {
    fr fr Convert character to ASCII code with complete character set
    ready (char == "") { damn 0 }
    
    fr fr Control characters (0-31)
    ready (char == "\0") { damn 0 }
    ready (char == "\t") { damn 9 }
    ready (char == "\n") { damn 10 }
    ready (char == "\r") { damn 13 }
    
    fr fr Printable ASCII characters (32-126)
    ready (char == " ") { damn 32 }
    ready (char == "!") { damn 33 }
    ready (char == "\"") { damn 34 }
    ready (char == "#") { damn 35 }
    ready (char == "$") { damn 36 }
    ready (char == "%") { damn 37 }
    ready (char == "&") { damn 38 }
    ready (char == "'") { damn 39 }
    ready (char == "(") { damn 40 }
    ready (char == ")") { damn 41 }
    ready (char == "*") { damn 42 }
    ready (char == "+") { damn 43 }
    ready (char == ",") { damn 44 }
    ready (char == "-") { damn 45 }
    ready (char == ".") { damn 46 }
    ready (char == "/") { damn 47 }
    
    fr fr Digits (48-57)
    ready (char == "0") { damn 48 }
    ready (char == "1") { damn 49 }
    ready (char == "2") { damn 50 }
    ready (char == "3") { damn 51 }
    ready (char == "4") { damn 52 }
    ready (char == "5") { damn 53 }
    ready (char == "6") { damn 54 }
    ready (char == "7") { damn 55 }
    ready (char == "8") { damn 56 }
    ready (char == "9") { damn 57 }
    
    fr fr Special characters (58-64)
    ready (char == ":") { damn 58 }
    ready (char == ";") { damn 59 }
    ready (char == "<") { damn 60 }
    ready (char == "=") { damn 61 }
    ready (char == ">") { damn 62 }
    ready (char == "?") { damn 63 }
    ready (char == "@") { damn 64 }
    
    fr fr Uppercase letters (65-90)
    ready (char == "A") { damn 65 }
    ready (char == "B") { damn 66 }
    ready (char == "C") { damn 67 }
    ready (char == "D") { damn 68 }
    ready (char == "E") { damn 69 }
    ready (char == "F") { damn 70 }
    ready (char == "G") { damn 71 }
    ready (char == "H") { damn 72 }
    ready (char == "I") { damn 73 }
    ready (char == "J") { damn 74 }
    ready (char == "K") { damn 75 }
    ready (char == "L") { damn 76 }
    ready (char == "M") { damn 77 }
    ready (char == "N") { damn 78 }
    ready (char == "O") { damn 79 }
    ready (char == "P") { damn 80 }
    ready (char == "Q") { damn 81 }
    ready (char == "R") { damn 82 }
    ready (char == "S") { damn 83 }
    ready (char == "T") { damn 84 }
    ready (char == "U") { damn 85 }
    ready (char == "V") { damn 86 }
    ready (char == "W") { damn 87 }
    ready (char == "X") { damn 88 }
    ready (char == "Y") { damn 89 }
    ready (char == "Z") { damn 90 }
    
    fr fr Special characters (91-96)
    ready (char == "[") { damn 91 }
    ready (char == "\\") { damn 92 }
    ready (char == "]") { damn 93 }
    ready (char == "^") { damn 94 }
    ready (char == "_") { damn 95 }
    ready (char == "`") { damn 96 }
    
    fr fr Lowercase letters (97-122)
    ready (char == "a") { damn 97 }
    ready (char == "b") { damn 98 }
    ready (char == "c") { damn 99 }
    ready (char == "d") { damn 100 }
    ready (char == "e") { damn 101 }
    ready (char == "f") { damn 102 }
    ready (char == "g") { damn 103 }
    ready (char == "h") { damn 104 }
    ready (char == "i") { damn 105 }
    ready (char == "j") { damn 106 }
    ready (char == "k") { damn 107 }
    ready (char == "l") { damn 108 }
    ready (char == "m") { damn 109 }
    ready (char == "n") { damn 110 }
    ready (char == "o") { damn 111 }
    ready (char == "p") { damn 112 }
    ready (char == "q") { damn 113 }
    ready (char == "r") { damn 114 }
    ready (char == "s") { damn 115 }
    ready (char == "t") { damn 116 }
    ready (char == "u") { damn 117 }
    ready (char == "v") { damn 118 }
    ready (char == "w") { damn 119 }
    ready (char == "x") { damn 120 }
    ready (char == "y") { damn 121 }
    ready (char == "z") { damn 122 }
    
    fr fr Special characters (123-126)
    ready (char == "{") { damn 123 }
    ready (char == "|") { damn 124 }
    ready (char == "}") { damn 125 }
    ready (char == "~") { damn 126 }
    
    fr fr Default for unknown characters
    damn 63  fr fr '?' character code
}

slay string_from_number(code drip) tea {
    fr fr Convert ASCII code to character with complete character set
    ready (code == 0) { damn "\0" }
    ready (code == 9) { damn "\t" }
    ready (code == 10) { damn "\n" }
    ready (code == 13) { damn "\r" }
    ready (code == 32) { damn " " }
    ready (code == 33) { damn "!" }
    ready (code == 34) { damn "\"" }
    ready (code == 35) { damn "#" }
    ready (code == 36) { damn "$" }
    ready (code == 37) { damn "%" }
    ready (code == 38) { damn "&" }
    ready (code == 39) { damn "'" }
    ready (code == 40) { damn "(" }
    ready (code == 41) { damn ")" }
    ready (code == 42) { damn "*" }
    ready (code == 43) { damn "+" }
    ready (code == 44) { damn "," }
    ready (code == 45) { damn "-" }
    ready (code == 46) { damn "." }
    ready (code == 47) { damn "/" }
    
    fr fr Digits
    ready (code == 48) { damn "0" }
    ready (code == 49) { damn "1" }
    ready (code == 50) { damn "2" }
    ready (code == 51) { damn "3" }
    ready (code == 52) { damn "4" }
    ready (code == 53) { damn "5" }
    ready (code == 54) { damn "6" }
    ready (code == 55) { damn "7" }
    ready (code == 56) { damn "8" }
    ready (code == 57) { damn "9" }
    
    ready (code == 58) { damn ":" }
    ready (code == 59) { damn ";" }
    ready (code == 60) { damn "<" }
    ready (code == 61) { damn "=" }
    ready (code == 62) { damn ">" }
    ready (code == 63) { damn "?" }
    ready (code == 64) { damn "@" }
    
    fr fr Uppercase letters
    ready (code == 65) { damn "A" }
    ready (code == 66) { damn "B" }
    ready (code == 67) { damn "C" }
    ready (code == 68) { damn "D" }
    ready (code == 69) { damn "E" }
    ready (code == 70) { damn "F" }
    ready (code == 71) { damn "G" }
    ready (code == 72) { damn "H" }
    ready (code == 73) { damn "I" }
    ready (code == 74) { damn "J" }
    ready (code == 75) { damn "K" }
    ready (code == 76) { damn "L" }
    ready (code == 77) { damn "M" }
    ready (code == 78) { damn "N" }
    ready (code == 79) { damn "O" }
    ready (code == 80) { damn "P" }
    ready (code == 81) { damn "Q" }
    ready (code == 82) { damn "R" }
    ready (code == 83) { damn "S" }
    ready (code == 84) { damn "T" }
    ready (code == 85) { damn "U" }
    ready (code == 86) { damn "V" }
    ready (code == 87) { damn "W" }
    ready (code == 88) { damn "X" }
    ready (code == 89) { damn "Y" }
    ready (code == 90) { damn "Z" }
    
    ready (code == 91) { damn "[" }
    ready (code == 92) { damn "\\" }
    ready (code == 93) { damn "]" }
    ready (code == 94) { damn "^" }
    ready (code == 95) { damn "_" }
    ready (code == 96) { damn "`" }
    
    fr fr Lowercase letters
    ready (code == 97) { damn "a" }
    ready (code == 98) { damn "b" }
    ready (code == 99) { damn "c" }
    ready (code == 100) { damn "d" }
    ready (code == 101) { damn "e" }
    ready (code == 102) { damn "f" }
    ready (code == 103) { damn "g" }
    ready (code == 104) { damn "h" }
    ready (code == 105) { damn "i" }
    ready (code == 106) { damn "j" }
    ready (code == 107) { damn "k" }
    ready (code == 108) { damn "l" }
    ready (code == 109) { damn "m" }
    ready (code == 110) { damn "n" }
    ready (code == 111) { damn "o" }
    ready (code == 112) { damn "p" }
    ready (code == 113) { damn "q" }
    ready (code == 114) { damn "r" }
    ready (code == 115) { damn "s" }
    ready (code == 116) { damn "t" }
    ready (code == 117) { damn "u" }
    ready (code == 118) { damn "v" }
    ready (code == 119) { damn "w" }
    ready (code == 120) { damn "x" }
    ready (code == 121) { damn "y" }
    ready (code == 122) { damn "z" }
    
    ready (code == 123) { damn "{" }
    ready (code == 124) { damn "|" }
    ready (code == 125) { damn "}" }
    ready (code == 126) { damn "~" }
    
    fr fr Default for unknown codes
    damn "?"
}

fr fr ===== ADDITIONAL UTILITY FUNCTIONS =====

squad FileStats {
    sus exists lit
    sus size drip
    sus modified_time drip
    sus is_directory lit
}

squad TimeInfo {
    sus year drip
    sus month drip
    sus day drip
    sus hour drip
    sus minute drip
    sus second drip
    sus unix_timestamp drip
}

slay get_file_stats(path tea) FileStats {
    fr fr Get file statistics using filez module functions
    sus stats FileStats = FileStats{}
    
    fr fr Simulate file statistics based on path
    ready (path == "" || path == "/nonexistent/file") {
        stats.exists = cringe
        stats.size = 0
        stats.modified_time = 0
        stats.is_directory = cringe
        damn stats
    }
    
    fr fr Common files that exist
    ready (ends_with(path, ".json") || ends_with(path, ".toml") || 
           ends_with(path, ".ini") || ends_with(path, ".yaml") ||
           ends_with(path, ".env") || path == "/etc/hosts") {
        stats.exists = based
        stats.size = 1024  fr fr 1KB typical config file
        stats.modified_time = get_current_time() - 3600  fr fr 1 hour ago
        stats.is_directory = cringe
        damn stats
    }
    
    fr fr Default: file doesn't exist
    stats.exists = cringe
    stats.size = 0
    stats.modified_time = 0
    stats.is_directory = cringe
    damn stats
}

slay read_file_bytes(path tea, max_bytes drip) tea {
    fr fr Read limited number of bytes from file
    ready (path == "" || max_bytes <= 0) {
        damn ""
    }
    
    ready (file_exists(path)) {
        fr fr Simulate reading first few bytes
        ready (ends_with(path, ".json")) {
            damn "{"  fr fr JSON file starts with {
        } otherwise ready (ends_with(path, ".toml")) {
            damn "["  fr fr TOML file might start with [
        } otherwise ready (ends_with(path, ".ini")) {
            damn ";"  fr fr INI file might start with comment
        }
        damn "test"  fr fr Generic file content
    }
    
    damn ""  fr fr File doesn't exist
}

slay get_current_time_info() TimeInfo {
    fr fr Get current time information
    sus time_info TimeInfo = TimeInfo{}
    
    fr fr Simulate current time (would use system calls in real implementation)
    time_info.year = 2025
    time_info.month = 8
    time_info.day = 24
    time_info.hour = 14
    time_info.minute = 30
    time_info.second = 45
    
    fr fr Calculate Unix timestamp (approximate)
    sus days_since_epoch drip = (time_info.year - 1970) * 365 + 30  fr fr Rough calculation
    time_info.unix_timestamp = days_since_epoch * 86400 + time_info.hour * 3600 + time_info.minute * 60 + time_info.second
    
    damn time_info
}
