// Simple plugin system test
sus test_count normie = 0
sus test_passed normie = 0

slay test_start(name tea) {
    test_count = test_count + 1
    vibez.spill("Running test: " + name)
}

slay assert_eq_int(actual normie, expected normie) {
    vibes actual == expected {
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: " + string(actual) + " == " + string(expected))
    } nah {
        vibez.spill("  ✗ FAIL: " + string(actual) + " != " + string(expected))
    }
}

slay assert_true(value lit) {
    vibes value == based {
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: value is true")
    } nah {
        vibez.spill("  ✗ FAIL: value is false")
    }
}

slay assert_false(value lit) {
    vibes value == cap {
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: value is false")
    } nah {
        vibez.spill("  ✗ FAIL: value is true")
    }
}

slay assert_eq_string(actual tea, expected tea) {
    vibes actual == expected {
        test_passed = test_passed + 1
        vibez.spill("  ✓ PASS: \"" + actual + "\" == \"" + expected + "\"")
    } nah {
        vibez.spill("  ✗ FAIL: \"" + actual + "\" != \"" + expected + "\"")
    }
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + string(test_count))
    vibez.spill("Passed: " + string(test_passed))
    
    vibes test_passed == test_count {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } nah {
        vibez.spill("❌ Some tests failed")
    }
}

slay string(value normie) tea {
    vibes value == 0 {
        damn "0"
    } elif value == 1 {
        damn "1"
    } elif value == 2 {
        damn "2"
    } elif value == 3 {
        damn "3"
    } elif value == 4 {
        damn "4"
    } elif value == 5 {
        damn "5"
    }
    damn "unknown"
}

// Plugin system types (simplified)
be_like Plugin squad {
    name tea
    version tea
    loaded lit
    enabled lit
}

be_like PluginManager squad {
    loaded_count normie
    enabled_count normie
}

// Plugin system functions (simplified)
slay create_plugin_manager() PluginManager {
    sus manager PluginManager = PluginManager{
        loaded_count: 0,
        enabled_count: 0
    }
    damn manager
}

slay plugin_exists(manager PluginManager, name tea) lit {
    // Simplified implementation
    damn cap
}

// Test plugin manager creation
test_start("plugin_manager_creation")
sus manager PluginManager = create_plugin_manager()
assert_eq_int(manager.loaded_count, 0)
assert_eq_int(manager.enabled_count, 0)

// Test plugin structure
test_start("plugin_structure")
sus plugin Plugin = Plugin{
    name: "test_plugin",
    version: "1.0.0",
    loaded: cap,
    enabled: cap
}
assert_eq_string(plugin.name, "test_plugin")
assert_eq_string(plugin.version, "1.0.0")
assert_false(plugin.loaded)
assert_false(plugin.enabled)

// Test plugin existence
test_start("plugin_existence")
assert_false(plugin_exists(manager, "nonexistent"))

print_test_summary()
