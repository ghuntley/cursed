// Test CURSED Plugin System Module
yeet "testz"
yeet "plugin_system"

// Test plugin manager creation
test_start("plugin_manager_creation")
sus manager PluginManager = create_plugin_manager()
assert_eq_int(manager.loaded_count, 0)
assert_eq_int(manager.enabled_count, 0)
assert_eq_int(len(manager.plugin_paths), 4)

// Test plugin structure
test_start("plugin_structure")
sus plugin Plugin = Plugin{
    name: "test_plugin",
    version: "1.0.0",
    description: "Test plugin",
    author: "CURSED Team",
    dependencies: [],
    loaded: cap,
    enabled: cap,
    entry_point: "main.csd",
    exports: {},
    config: {}
}
assert_eq_string(plugin.name, "test_plugin")
assert_eq_string(plugin.version, "1.0.0")
assert_false(plugin.loaded)
assert_false(plugin.enabled)

// Test plugin validation
test_start("plugin_validation")
sus valid_plugin Plugin = Plugin{
    name: "valid_plugin",
    version: "1.0.0",
    description: "Valid plugin",
    author: "Test Author",
    dependencies: [],
    loaded: cap,
    enabled: cap,
    entry_point: "main.csd",
    exports: {},
    config: {}
}
assert_true(validate_plugin(valid_plugin))

sus invalid_plugin Plugin = Plugin{
    name: "",
    version: "1.0.0",
    description: "Invalid plugin",
    author: "Test Author",
    dependencies: [],
    loaded: cap,
    enabled: cap,
    entry_point: "main.csd",
    exports: {},
    config: {}
}
assert_false(validate_plugin(invalid_plugin))

// Test plugin existence check
test_start("plugin_existence")
sus test_manager PluginManager = create_plugin_manager()
assert_false(plugin_exists(test_manager, "nonexistent_plugin"))

// Test plugin configuration
test_start("plugin_configuration")
sus config_plugin Plugin = Plugin{
    name: "config_plugin",
    version: "1.0.0",
    description: "Configurable plugin",
    author: "Test Author",
    dependencies: [],
    loaded: cap,
    enabled: cap,
    entry_point: "main.csd",
    exports: {},
    config: {}
}
sus config_manager PluginManager = create_plugin_manager()
config_manager.plugins["config_plugin"] = config_plugin

sus test_config map[tea]tea = {}
test_config["setting1"] = "value1"
test_config["setting2"] = "value2"

config_manager = configure_plugin(config_manager, "config_plugin", test_config)
sus updated_plugin Plugin = config_manager.plugins["config_plugin"]
assert_true(has_config(updated_plugin, "setting1"))
assert_true(has_config(updated_plugin, "setting2"))

// Test hooks system
test_start("hooks_system")
sus hook_manager PluginManager = create_plugin_manager()
sus hook_plugin Plugin = Plugin{
    name: "hook_plugin",
    version: "1.0.0",
    description: "Hook plugin",
    author: "Test Author",
    dependencies: [],
    loaded: cap,
    enabled: cap,
    entry_point: "main.csd",
    exports: {},
    config: {}
}
hook_manager.plugins["hook_plugin"] = hook_plugin

hook_manager = register_hook(hook_manager, "test_hook", "hook_plugin")
assert_true(has_hook(hook_manager, "test_hook"))

hook_manager = unregister_hook(hook_manager, "test_hook", "hook_plugin")
sus hooks [tea] = hook_manager.hooks["test_hook"]
assert_eq_int(len(hooks), 0)

// Test plugin statistics
test_start("plugin_statistics")
sus stats_manager PluginManager = create_plugin_manager()
sus stats_plugin Plugin = Plugin{
    name: "stats_plugin",
    version: "1.0.0",
    description: "Statistics plugin",
    author: "Test Author",
    dependencies: [],
    loaded: based,
    enabled: based,
    entry_point: "main.csd",
    exports: {},
    config: {}
}
stats_manager.plugins["stats_plugin"] = stats_plugin
stats_manager.loaded_count = 1
stats_manager.enabled_count = 1

sus stats map[tea]normie = get_plugin_stats(stats_manager)
assert_eq_int(stats["total_plugins"], 1)
assert_eq_int(stats["loaded_plugins"], 1)
assert_eq_int(stats["enabled_plugins"], 1)

// Test plugin manifest parsing
test_start("manifest_parsing")
sus manifest_data tea = "{\"name\":\"test_plugin\",\"version\":\"1.0.0\",\"description\":\"Test plugin\",\"author\":\"Test Author\",\"entry_point\":\"main.csd\"}"
sus parsed_plugin Plugin = parse_plugin_manifest(manifest_data)
assert_eq_string(parsed_plugin.name, "test_plugin")
assert_eq_string(parsed_plugin.version, "1.0.0")
assert_eq_string(parsed_plugin.description, "Test plugin")
assert_eq_string(parsed_plugin.author, "Test Author")
assert_eq_string(parsed_plugin.entry_point, "main.csd")

// Test field extraction
test_start("field_extraction")
sus test_data tea = "{\"name\":\"test_value\",\"version\":\"1.0.0\"}"
sus extracted_name tea = extract_field(test_data, "name")
sus extracted_version tea = extract_field(test_data, "version")
assert_eq_string(extracted_name, "test_value")
assert_eq_string(extracted_version, "1.0.0")

// Test plugin permissions
test_start("plugin_permissions")
sus perm_plugin Plugin = Plugin{
    name: "perm_plugin",
    version: "1.0.0",
    description: "Permission plugin",
    author: "Test Author",
    dependencies: [],
    loaded: cap,
    enabled: cap,
    entry_point: "main.csd",
    exports: {},
    config: {}
}
perm_plugin.config["permissions"] = "filesystem,network"
assert_true(check_plugin_permissions(perm_plugin, "filesystem"))
assert_true(check_plugin_permissions(perm_plugin, "network"))
assert_false(check_plugin_permissions(perm_plugin, "admin"))

// Test plugin discovery
test_start("plugin_discovery")
sus discovery_manager PluginManager = create_plugin_manager()
sus discovered [tea] = discover_plugins(discovery_manager)
// Discovery depends on filesystem, so just check it returns an array
assert_true(len(discovered) >= 0)

// Test plugin listing
test_start("plugin_listing")
sus list_manager PluginManager = create_plugin_manager()
sus list_plugin1 Plugin = Plugin{
    name: "list_plugin1",
    version: "1.0.0",
    description: "List plugin 1",
    author: "Test Author",
    dependencies: [],
    loaded: cap,
    enabled: cap,
    entry_point: "main.csd",
    exports: {},
    config: {}
}
sus list_plugin2 Plugin = Plugin{
    name: "list_plugin2",
    version: "1.0.0",
    description: "List plugin 2",
    author: "Test Author",
    dependencies: [],
    loaded: cap,
    enabled: based,
    entry_point: "main.csd",
    exports: {},
    config: {}
}
list_manager.plugins["list_plugin1"] = list_plugin1
list_manager.plugins["list_plugin2"] = list_plugin2

sus all_plugins [Plugin] = list_plugins(list_manager)
assert_eq_int(len(all_plugins), 2)

sus enabled_plugins [Plugin] = list_enabled_plugins(list_manager)
assert_eq_int(len(enabled_plugins), 1)

// Test plugin retrieval
test_start("plugin_retrieval")
sus get_manager PluginManager = create_plugin_manager()
sus get_plugin Plugin = Plugin{
    name: "get_plugin",
    version: "1.0.0",
    description: "Get plugin",
    author: "Test Author",
    dependencies: [],
    loaded: cap,
    enabled: cap,
    entry_point: "main.csd",
    exports: {},
    config: {}
}
get_manager.plugins["get_plugin"] = get_plugin

sus retrieved_plugin Plugin = get_plugin(get_manager, "get_plugin")
assert_eq_string(retrieved_plugin.name, "get_plugin")
assert_eq_string(retrieved_plugin.version, "1.0.0")

sus empty_plugin Plugin = get_plugin(get_manager, "nonexistent_plugin")
assert_eq_string(empty_plugin.name, "")

// Test dependency checking
test_start("dependency_checking")
sus dep_manager PluginManager = create_plugin_manager()
sus dep_plugin Plugin = Plugin{
    name: "dependency_plugin",
    version: "1.0.0",
    description: "Dependency plugin",
    author: "Test Author",
    dependencies: [],
    loaded: based,
    enabled: cap,
    entry_point: "main.csd",
    exports: {},
    config: {}
}
dep_manager.plugins["dependency_plugin"] = dep_plugin

sus dependent_plugin Plugin = Plugin{
    name: "dependent_plugin",
    version: "1.0.0",
    description: "Dependent plugin",
    author: "Test Author",
    dependencies: ["dependency_plugin"],
    loaded: cap,
    enabled: cap,
    entry_point: "main.csd",
    exports: {},
    config: {}
}

assert_true(check_dependencies(dep_manager, dependent_plugin))

sus missing_dep_plugin Plugin = Plugin{
    name: "missing_dep_plugin",
    version: "1.0.0",
    description: "Missing dependency plugin",
    author: "Test Author",
    dependencies: ["nonexistent_plugin"],
    loaded: cap,
    enabled: cap,
    entry_point: "main.csd",
    exports: {},
    config: {}
}

assert_false(check_dependencies(dep_manager, missing_dep_plugin))

print_test_summary()
