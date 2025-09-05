yeet "configz"

fr fr Simple demo of configz functionality
vibez.spill("Testing CURSED Configuration Management")

fr fr Test format constants
vibez.spill("JSON format:", format_json())
vibez.spill("YAML format:", format_yaml())

fr fr Test environment variables
sus home_dir tea = get_env_variable("HOME")
vibez.spill("Home directory:", home_dir)

sus user_name tea = get_env_variable("USER")
vibez.spill("User name:", user_name)

fr fr Test format detection
sus json_sample tea = "{\"test\": \"value\"}"
sus format tea = auto_detect_format(json_sample)
vibez.spill("Detected format:", format)

fr fr Test configuration parsing
sus config tea = parse_json_config(json_sample)
vibez.spill("Parsed config:", config)

fr fr Test validation
sus is_valid lit = validate_configuration(config)
ready (is_valid) {
    vibez.spill("Configuration is valid!")
} otherwise {
    vibez.spill("Configuration is invalid!")
}

fr fr Test value access
sus test_config tea = "{\"name\":\"test\",\"value\":\"42\"}"
sus name tea = get_config_string(test_config, "name", "default")
vibez.spill("Config name:", name)

sus value normie = get_config_int(test_config, "value", 0)
vibez.spill("Config value:", value)

vibez.spill("Configuration management demo completed!")
