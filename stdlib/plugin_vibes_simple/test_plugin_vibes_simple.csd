yeet "testz"
yeet "plugin_vibes_simple"

test_start("Simple plugin test")
sus loaded := load_plugin("auth")
assert_true(loaded)

sus plugins := list_loaded_plugins()
assert_eq_string(plugins, "auth")

sus unloaded := unload_plugin("auth")
assert_true(unloaded)

print_test_summary()
