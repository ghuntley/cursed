yeet "plugin_system"

# Simple Plugin System Test
vibez.spill("🔌 Simple Plugin System Test")
vibez.spill("=============================")

# Initialize plugin system
plugin_system_init()
vibez.spill("✅ Plugin system initialized")

# Create a simple plugin
sus metadata := plugin_create_metadata("test_plugin", "1.0.0", "Test plugin", "CURSED Team")
plugin_register(metadata)
vibez.spill("✅ Plugin registered")

# Load the plugin
plugin_load("test_plugin")
vibez.spill("✅ Plugin loaded")

# Check plugin status
sus status := plugin_get_status("test_plugin")
vibez.spill("Plugin status: " + status)

# Register a hook
plugin_register_hook("test_plugin", "test_hook", "test_callback", PLUGIN_PRIORITY_HIGH)
vibez.spill("✅ Hook registered")

# Execute the hook
plugin_execute_hooks("test_hook", "test_context")
vibez.spill("✅ Hook executed")

# Register an event listener
plugin_register_event_listener("test_plugin", PLUGIN_EVENT_LOAD, "on_load")
vibez.spill("✅ Event listener registered")

# Trigger an event
plugin_trigger_event(PLUGIN_EVENT_LOAD, "test_plugin", "Test event")
vibez.spill("✅ Event triggered")

# Set and get configuration
plugin_set_config("test_plugin", "setting1", "value1")
sus config_value := plugin_get_config("test_plugin", "setting1")
vibez.spill("Configuration value: " + config_value)

# Check plugin health
sus health := plugin_health_check("test_plugin")
vibez.spill("Plugin health: " + health)

# Create API context
sus context := plugin_create_api_context("test_plugin")
vibez.spill("API context created for: " + context.plugin_name)

# System statistics
sus stats := plugin_get_system_stats()
vibez.spill("Total plugins: " + stats)

# Clean up
plugin_cleanup_all()
vibez.spill("✅ All plugins cleaned up")

vibez.spill("🎉 Plugin system test completed successfully!")
