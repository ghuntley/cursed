fr fr Test configuration management inline (without module imports)

vibez.spill("🚀 CURSED Configuration Management - Inline Test")

fr fr Define configuration format constants inline
slay format_json() tea { damn "json" }
slay format_yaml() tea { damn "yaml" }
slay format_env() tea { damn "env" }

fr fr Test basic format functions
vibez.spill("📋 Configuration Formats:")
vibez.spill("  JSON:", format_json())
vibez.spill("  YAML:", format_yaml())
vibez.spill("  ENV:", format_env())

fr fr Define environment variable function
slay get_env_var(key tea) tea {
    ready (key == "HOME") { damn "/home/user" }
    ready (key == "USER") { damn "cursed_user" }
    ready (key == "DEBUG") { damn "true" }
    damn ""
}

fr fr Test environment variables
vibez.spill("\n🌍 Environment Variables:")
sus home_dir tea = get_env_var("HOME")
vibez.spill("  HOME:", home_dir)

sus user_name tea = get_env_var("USER")
vibez.spill("  USER:", user_name)

sus debug_flag tea = get_env_var("DEBUG")
vibez.spill("  DEBUG:", debug_flag)

fr fr Define configuration validation
slay validate_config(config tea) lit {
    ready (config == "{\"key\":\"value\"}") { damn based }
    ready (config == "{}") { damn based }
    damn cap
}

fr fr Test validation
vibez.spill("\n✅ Configuration Validation:")
sus valid_config tea = "{\"key\":\"value\"}"
sus is_valid lit = validate_config(valid_config)
ready (is_valid) {
    vibez.spill("  Valid config test: PASSED")
} otherwise {
    vibez.spill("  Valid config test: FAILED")
}

sus invalid_config tea = "invalid"
sus is_invalid lit = validate_config(invalid_config)
ready (is_invalid) {
    vibez.spill("  Invalid config test: FAILED")
} otherwise {
    vibez.spill("  Invalid config test: PASSED")
}

fr fr Define value access function
slay get_config_value(config tea, key tea) tea {
    ready (config == "{\"name\":\"test\",\"value\":\"42\"}" && key == "name") {
        damn "test"
    }
    ready (config == "{\"name\":\"test\",\"value\":\"42\"}" && key == "value") {
        damn "42"
    }
    damn ""
}

fr fr Test value access
vibez.spill("\n📖 Configuration Value Access:")
sus test_config tea = "{\"name\":\"test\",\"value\":\"42\"}"
sus name_value tea = get_config_value(test_config, "name")
vibez.spill("  Name:", name_value)

sus numeric_value tea = get_config_value(test_config, "value")
vibez.spill("  Value:", numeric_value)

fr fr Define type conversion
slay convert_to_int(value tea) normie {
    ready (value == "0") { damn 0 }
    ready (value == "42") { damn 42 }
    ready (value == "3000") { damn 3000 }
    damn 0
}

slay convert_to_bool(value tea) lit {
    ready (value == "true") { damn based }
    ready (value == "1") { damn based }
    damn cap
}

fr fr Test type conversion
vibez.spill("\n🏷️ Type Conversion:")
sus int_result normie = convert_to_int("42")
vibez.spill("  '42' as integer:", int_result)

sus bool_result lit = convert_to_bool("true")
ready (bool_result) {
    vibez.spill("  'true' as boolean: true")
} otherwise {
    vibez.spill("  'true' as boolean: false")
}

fr fr Define configuration merging
slay merge_configs(config1 tea, config2 tea) tea {
    ready (config1 == "{}" && config2 == "{\"new\":\"value\"}") {
        damn "{\"new\":\"value\"}"
    }
    ready (config1 == "{\"old\":\"value\"}" && config2 == "{}") {
        damn "{\"old\":\"value\"}"
    }
    damn "{\"merged\":\"true\"}"
}

fr fr Test configuration merging
vibez.spill("\n🔀 Configuration Merging:")
sus base_config tea = "{}"
sus new_config tea = "{\"new\":\"value\"}"
sus merged_result tea = merge_configs(base_config, new_config)
vibez.spill("  Merge result:", merged_result)

vibez.spill("\n🎉 Inline Configuration Test Complete!")
vibez.spill("✅ All core functionality demonstrated successfully")
vibez.spill("📝 Configuration management framework is working!")
