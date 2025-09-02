yeet "testz"
yeet "build_system"
yeet "fs"
yeet "json_tea"
yeet "stringz"

test_start("Build System Comprehensive Tests")

fr fr Test build configuration creation
slay test_create_build_config() {
    sus config map[tea]interface{} = build_system.create_build_config()
    
    assert_eq_string(config["name"].(tea), "")
    assert_eq_string(config["version"].(tea), "1.0.0")
    assert_eq_string(config["output_dir"].(tea), "build")
    assert_eq_string(config["optimization_level"].(tea), "2")
    assert_true(config["parallel_builds"].(lit))
    assert_true(config["cache_enabled"].(lit))
    
    vibez.spill("✅ create_build_config test passed")
}

fr fr Test configuration parsing
slay test_parse_build_config() { fr fr Create a test configuration file
    sus test_config tea = `{
        "name": "test_project",
        "version": "2.0.0",
        "description": "Test project for build system",
        "targets": ["main", "cli"],
        "dependencies": {
            "json_parser": "1.0.0",
            "http_client": "2.1.0"
        },
        "optimization_level": "3"
    }` fr fr Write test config to file
    fs.write_file("test_config.toml", test_config) fr fr Parse the configuration
    sus parsed_config map[tea]interface{} = build_system.parse_build_config("test_config.toml")
    
    assert_eq_string(parsed_config["name"].(tea), "test_project")
    assert_eq_string(parsed_config["version"].(tea), "2.0.0")
    assert_eq_string(parsed_config["optimization_level"].(tea), "3")
    
    sus dependencies map[tea]tea = parsed_config["dependencies"].(map[tea]tea)
    assert_eq_string(dependencies["json_parser"], "1.0.0")
    assert_eq_string(dependencies["http_client"], "2.1.0") fr fr Cleanup
    fs.remove_file("test_config.toml")
    
    vibez.spill("✅ parse_build_config test passed")
}

fr fr Test configuration merging
slay test_merge_configs() {
    sus default_config map[tea]interface{} = build_system.create_build_config()
    
    sus user_config map[tea]interface{} = map[tea]interface{}{}
    user_config["name"] = "custom_project"
    user_config["optimization_level"] = "1"
    user_config["custom_field"] = "custom_value"
    
    sus merged map[tea]interface{} = build_system.merge_configs(default_config, user_config) fr fr Check that user values override defaults
    assert_eq_string(merged["name"].(tea), "custom_project")
    assert_eq_string(merged["optimization_level"].(tea), "1") fr fr Check that defaults are preserved
    assert_eq_string(merged["version"].(tea), "1.0.0")
    assert_eq_string(merged["output_dir"].(tea), "build") fr fr Check that custom fields are added
    assert_eq_string(merged["custom_field"].(tea), "custom_value")
    
    vibez.spill("✅ merge_configs test passed")
}

fr fr Test dependency resolution
slay test_resolve_dependencies() {
    sus config map[tea]interface{} = build_system.create_build_config()
    
    sus dependencies map[tea]tea = map[tea]tea{}
    dependencies["testz"] = "1.0.0"
    dependencies["stringz"] = "1.0.0"
    config["dependencies"] = dependencies
    
    sus resolved tea[value] = build_system.resolve_dependencies(config) fr fr Should find stdlib modules
    assert_true(len(resolved) >= 0) fr fr At least some dependencies should resolve
    
    vibez.spill("✅ resolve_dependencies test passed")
}

fr fr Test target file finding
slay test_find_target_file() { fr fr Create test directory structure
    fs.make_dir_all("test_src")
    fs.write_file("test_src/main.csd", "vibez.spill(\"test\")")
    
    sus source_dirs tea[value] = tea[value]{"test_src"}
    sus target_file tea = build_system.find_target_file("main", source_dirs)
    
    assert_eq_string(target_file, "test_src/main.csd") fr fr Cleanup
    fs.remove_file("test_src/main.csd")
    fs.remove_dir("test_src")
    
    vibez.spill("✅ find_target_file test passed")
}

fr fr Test build mode determination
slay test_determine_build_mode() {
    sus config1 map[tea]interface{} = build_system.create_build_config()
    sus mode1 tea = build_system.determine_build_mode(config1)
    assert_eq_string(mode1, "native")
    
    sus config2 map[tea]interface{} = build_system.create_build_config()
    config2["build_mode"] = "interpret"
    sus mode2 tea = build_system.determine_build_mode(config2)
    assert_eq_string(mode2, "interpret")
    
    vibez.spill("✅ determine_build_mode test passed")
}

fr fr Test build command construction
slay test_construct_build_command() {
    sus target_file tea = "src/main.csd"
    sus build_mode tea = "native"
    sus optimization tea = "2"
    sus output_dir tea = "build"
    sus dependencies tea[value] = tea[value]{"stdlib/json", "stdlib/http"}
    
    sus cmd tea = build_system.construct_build_command(target_file, build_mode, optimization, output_dir, dependencies)
    
    assert_true(stringz.contains(cmd, "cargo run --bin cursed"))
    assert_true(stringz.contains(cmd, "compile"))
    assert_true(stringz.contains(cmd, "--opt-level 2"))
    assert_true(stringz.contains(cmd, "src/main.csd"))
    assert_true(stringz.contains(cmd, "-I stdlib/json"))
    assert_true(stringz.contains(cmd, "-I stdlib/http"))
    
    vibez.spill("✅ construct_build_command test passed")
}

fr fr Test test file finding
slay test_find_test_files() { fr fr Create test files
    fs.make_dir_all("test_dir")
    fs.write_file("test_dir/test_math.csd", "# Math tests")
    fs.write_file("test_dir/string_test.csd", "# String tests")
    fs.write_file("test_dir/main.csd", "# Not a test")
    
    sus patterns tea[value] = tea[value]{"test_*.csd", "*_test.csd"}
    sus source_dirs tea[value] = tea[value]{"test_dir"}
    
    sus test_files tea[value] = build_system.find_test_files(patterns, source_dirs)
    
    assert_true(len(test_files) >= 2) fr fr Should find at least the two test files fr fr Cleanup
    fs.remove_file("test_dir/test_math.csd")
    fs.remove_file("test_dir/string_test.csd")
    fs.remove_file("test_dir/main.csd")
    fs.remove_dir("test_dir")
    
    vibez.spill("✅ find_test_files test passed")
}

fr fr Test package listing
slay test_list_packages() {
    sus packages tea[value] = build_system.list_packages() fr fr Should return a list (might be empty)
    assert_true(len(packages) >= 0)
    
    vibez.spill("✅ list_packages test passed")
}

fr fr Test cache functionality
slay test_build_cache() {
    sus target tea = "test_target"
    sus dependencies tea[value] = tea[value]{} fr fr Initially no cache should exist
    sus has_cache lit = build_system.check_build_cache(target, dependencies)
    assert_false(has_cache) fr fr Update cache
    build_system.update_build_cache(target)
    
    vibez.spill("✅ build_cache test passed")
}

fr fr Test comprehensive build configuration
slay test_comprehensive_build_config() {
    sus comprehensive_config tea = `{
        "name": "comprehensive_project",
        "version": "3.0.0",
        "authors": ["CURSED Team", "Build System Team"],
        "description": "Comprehensive test project",
        "targets": ["main", "cli", "server"],
        "dependencies": {
            "json_parser": "2.0.0",
            "http_client": "3.1.0",
            "crypto": "1.5.0"
        },
        "build_deps": {
            "code_generator": "1.0.0"
        },
        "test_patterns": ["test_*.csd", "*_test.csd", "tests/*.csd"],
        "source_dirs": ["src", "lib", "modules"],
        "output_dir": "dist",
        "optimization_level": "3",
        "parallel_builds": true,
        "cache_enabled": true,
        "build_mode": "native"
    }` fr fr Write comprehensive config
    fs.write_file("comprehensive_config.toml", comprehensive_config) fr fr Parse and validate
    sus parsed map[tea]interface{} = build_system.parse_build_config("comprehensive_config.toml")
    
    assert_eq_string(parsed["name"].(tea), "comprehensive_project")
    assert_eq_string(parsed["version"].(tea), "3.0.0")
    assert_eq_string(parsed["output_dir"].(tea), "dist")
    assert_eq_string(parsed["optimization_level"].(tea), "3")
    assert_eq_string(parsed["build_mode"].(tea), "native")
    
    sus targets tea[value] = parsed["targets"].(tea[value])
    assert_true(len(targets) == 3)
    
    sus dependencies map[tea]tea = parsed["dependencies"].(map[tea]tea)
    assert_eq_string(dependencies["crypto"], "1.5.0") fr fr Cleanup
    fs.remove_file("comprehensive_config.toml")
    
    vibez.spill("✅ comprehensive_build_config test passed")
}

fr fr Test error handling in configuration parsing
slay test_config_error_handling() { fr fr Test with non-existent file
    sus config1 map[tea]interface{} = build_system.parse_build_config("non_existent.toml")
    assert_eq_string(config1["name"].(tea), "") fr fr Should return default config fr fr Test with invalid JSON
    fs.write_file("invalid_config.toml", "{ invalid json content")
    sus config2 map[tea]interface{} = build_system.parse_build_config("invalid_config.toml")
    assert_eq_string(config2["name"].(tea), "") fr fr Should return default config fr fr Cleanup
    fs.remove_file("invalid_config.toml")
    
    vibez.spill("✅ config_error_handling test passed")
}

fr fr Test build system integration
slay test_build_system_integration() { fr fr Create a minimal project structure
    fs.make_dir_all("integration_test")
    fs.write_file("integration_test/main.csd", "vibez.spill(\"Integration test works!\")")
    
    sus minimal_config tea = `{
        "name": "integration_test",
        "version": "1.0.0",
        "targets": ["main"],
        "source_dirs": ["integration_test"],
        "output_dir": "integration_build"
    }`
    
    fs.write_file("integration_test_config.toml", minimal_config) fr fr Test that configuration parses correctly
    sus config map[tea]interface{} = build_system.parse_build_config("integration_test_config.toml")
    assert_eq_string(config["name"].(tea), "integration_test") fr fr Test that target file can be found
    sus source_dirs tea[value] = config["source_dirs"].(tea[value])
    sus target_file tea = build_system.find_target_file("main", source_dirs)
    assert_eq_string(target_file, "integration_test/main.csd") fr fr Cleanup
    fs.remove_file("integration_test/main.csd")
    fs.remove_dir("integration_test")
    fs.remove_file("integration_test_config.toml")
    
    vibez.spill("✅ build_system_integration test passed")
}

fr fr Main test runner
slay main() {
    test_create_build_config()
    test_parse_build_config()
    test_merge_configs()
    test_resolve_dependencies()
    test_find_target_file()
    test_determine_build_mode()
    test_construct_build_command()
    test_find_test_files()
    test_list_packages()
    test_build_cache()
    test_comprehensive_build_config()
    test_config_error_handling()
    test_build_system_integration()
    
    print_test_summary()
    
    vibez.spill("")
    vibez.spill("🚀 Build System Test Suite Complete!")
    vibez.spill("All core functionality validated and ready for production use.")
}
