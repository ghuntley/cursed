# Test Package Manager Implementation
yeet "packagz"
yeet "vibez" 
yeet "filez"
yeet "jsonz"

# Test package manager initialization
slay test_package_manager_init() lit {
    vibez.spill("Testing package manager initialization...")
    
    sus registry_url tea = "https://test-registry.cursedlang.org"
    sus cache_dir tea = "/tmp/cursed-test-cache"
    
    sus manager PackageManager = packagz.init_package_manager(registry_url, cache_dir)
    
    ready (manager.registry.config.url == registry_url) {
        vibez.spill("✅ Registry URL initialized correctly")
    } otherwise {
        vibez.spill("❌ Registry URL mismatch")
        damn cap
    }
    
    ready (manager.cache_dir == cache_dir) {
        vibez.spill("✅ Cache directory initialized correctly")
    } otherwise {
        vibez.spill("❌ Cache directory mismatch")
        damn cap
    }
    
    vibez.spill("✅ Package manager initialization test passed")
    damn based
}

# Test version parsing and comparison
slay test_version_parsing() lit {
    vibez.spill("Testing version parsing and comparison...")
    
    sus v1 PackageVersion = packagz.parse_version("1.2.3")
    sus v2 PackageVersion = packagz.parse_version("1.2.4")
    sus v3 PackageVersion = packagz.parse_version("2.0.0")
    
    ready (v1.major == 1 && v1.minor == 2 && v1.patch == 3) {
        vibez.spill("✅ Version parsing works correctly")
    } otherwise {
        vibez.spill("❌ Version parsing failed")
        damn cap
    }
    
    sus cmp1 drip = packagz.compare_versions(v1, v2)
    sus cmp2 drip = packagz.compare_versions(v2, v3)
    sus cmp3 drip = packagz.compare_versions(v1, v1)
    
    ready (cmp1 == -1 && cmp2 == -1 && cmp3 == 0) {
        vibez.spill("✅ Version comparison works correctly")
    } otherwise {
        vibez.spill("❌ Version comparison failed")
        damn cap
    }
    
    vibez.spill("✅ Version parsing test passed")
    damn based
}

# Test package metadata parsing
slay test_metadata_parsing() lit {
    vibez.spill("Testing package metadata parsing...")
    
    # Create a mock JSON object for testing
    sus mock_json JsonValue = JsonValue {
        type: "object",
        string_value: "",
        number_value: 0,
        boolean_value: cap,
        array_values: [],
        object_keys: ["name", "version", "description", "authors", "license"],
        object_values: [
            JsonValue { type: "string", string_value: "test-package", number_value: 0, boolean_value: cap, array_values: [], object_keys: [], object_values: [] },
            JsonValue { type: "string", string_value: "1.0.0", number_value: 0, boolean_value: cap, array_values: [], object_keys: [], object_values: [] },
            JsonValue { type: "string", string_value: "A test package", number_value: 0, boolean_value: cap, array_values: [], object_keys: [], object_values: [] },
            JsonValue { 
                type: "array", 
                string_value: "", 
                number_value: 0, 
                boolean_value: cap, 
                array_values: [
                    JsonValue { type: "string", string_value: "Test Author", number_value: 0, boolean_value: cap, array_values: [], object_keys: [], object_values: [] }
                ], 
                object_keys: [], 
                object_values: [] 
            },
            JsonValue { type: "string", string_value: "MIT", number_value: 0, boolean_value: cap, array_values: [], object_keys: [], object_values: [] }
        ]
    }
    
    sus metadata PackageMetadata = packagz.parse_package_metadata(mock_json)
    
    ready (metadata.name == "test-package" && metadata.version == "1.0.0" && metadata.license == "MIT") {
        vibez.spill("✅ Package metadata parsing works correctly")
    } otherwise {
        vibez.spill("❌ Package metadata parsing failed")
        vibez.spill("Name:", metadata.name, "Version:", metadata.version, "License:", metadata.license)
        damn cap
    }
    
    vibez.spill("✅ Metadata parsing test passed")
    damn based
}

# Test installed package management
slay test_package_persistence() lit {
    vibez.spill("Testing package persistence...")
    
    sus manager PackageManager = packagz.init_package_manager("https://test.cursedlang.org", "/tmp/cursed-pkg-test")
    
    # Create a test installed package
    sus test_pkg InstalledPackage = InstalledPackage {
        name: "test-pkg",
        version: "1.0.0",
        install_path: "/tmp/cursed-pkg-test/installed/test-pkg-1.0.0",
        installed_at: "2025-08-21T12:00:00Z",
        dependencies: ["dep1", "dep2"]
    }
    
    manager.installer.installed_packages = [test_pkg]
    
    # Test saving and loading
    ready (packagz.save_installed_packages(manager)) {
        vibez.spill("✅ Package saving works")
    } otherwise {
        vibez.spill("❌ Package saving failed")
        damn cap
    }
    
    # Clear the packages and reload
    manager.installer.installed_packages = []
    
    ready (packagz.load_installed_packages(manager)) {
        vibez.spill("✅ Package loading works")
    } otherwise {
        vibez.spill("❌ Package loading failed")
        damn cap
    }
    
    # Verify the package was loaded correctly
    sus loaded_pkg InstalledPackage = packagz.get_installed_package(manager, "test-pkg")
    ready (loaded_pkg.name == "test-pkg" && loaded_pkg.version == "1.0.0") {
        vibez.spill("✅ Package persistence works correctly")
    } otherwise {
        vibez.spill("❌ Package persistence failed")
        damn cap
    }
    
    vibez.spill("✅ Package persistence test passed")
    damn based
}

# Main test runner
slay main() drip {
    vibez.spill("CURSED Package Manager Test Suite")
    vibez.spill("=================================")
    
    # Clean up any previous test files
    filez.remove_dir_all("/tmp/cursed-test-cache")
    filez.remove_dir_all("/tmp/cursed-pkg-test")
    
    sus tests_passed drip = 0
    sus total_tests drip = 4
    
    ready (test_package_manager_init()) {
        tests_passed = tests_passed + 1
    }
    
    ready (test_version_parsing()) {
        tests_passed = tests_passed + 1
    }
    
    ready (test_metadata_parsing()) {
        tests_passed = tests_passed + 1
    }
    
    ready (test_package_persistence()) {
        tests_passed = tests_passed + 1
    }
    
    vibez.spill("")
    vibez.spill("Test Results:")
    vibez.spill("=============")
    vibez.spill("Passed:", tests_passed, "/", total_tests)
    
    ready (tests_passed == total_tests) {
        vibez.spill("✅ All tests passed!")
        damn 0
    } otherwise {
        vibez.spill("❌ Some tests failed")
        damn 1
    }
}
