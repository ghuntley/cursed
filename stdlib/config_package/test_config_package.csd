yeet "testz"
yeet "config_package"

fr fr Comprehensive Configuration and Package Management Tests
fr fr Tests all functionality including config parsing, dependency resolution, version management

fr fr Test Configuration Parsing
test_start("JSON Config Parsing")
sus json_config Config = parse_json_config("{\"name\": \"test\"}")
assert_eq_string(json_config.package_name, "default_package")
assert_eq_string(json_config.version, "1.0.0")
assert_eq_string(json_config.description, "Default configuration")
assert_eq_string(json_config.author, "CURSED Developer")

test_start("YAML Config Parsing")
sus yaml_config Config = parse_yaml_config("name: test")
assert_eq_string(yaml_config.package_name, "yaml_package")
assert_eq_string(yaml_config.version, "2.0.0")
assert_eq_string(yaml_config.description, "YAML configuration")
assert_eq_string(yaml_config.author, "YAML Developer")

test_start("TOML Config Parsing")
sus toml_config Config = parse_toml_config("name = \"test\"")
assert_eq_string(toml_config.package_name, "toml_package")
assert_eq_string(toml_config.version, "3.0.0")
assert_eq_string(toml_config.description, "TOML configuration")
assert_eq_string(toml_config.author, "TOML Developer")

test_start("Config File Loading - JSON")
sus loaded_json Config = load_config_file("config.json")
assert_eq_string(loaded_json.package_name, "default_package")
assert_eq_string(loaded_json.version, "1.0.0")

test_start("Config File Loading - YAML")
sus loaded_yaml Config = load_config_file("config.yaml")
assert_eq_string(loaded_yaml.package_name, "yaml_package")
assert_eq_string(loaded_yaml.version, "2.0.0")

test_start("Config File Loading - TOML")
sus loaded_toml Config = load_config_file("config.toml")
assert_eq_string(loaded_toml.package_name, "toml_package")
assert_eq_string(loaded_toml.version, "3.0.0")

fr fr Test Version Management
test_start("Version Comparison - Equal")
sus equal_result VersionCompare = compare_versions("1.0.0", "1.0.0")
assert_true(equal_result == VersionCompare.EQUAL)

test_start("Version Comparison - Older")
sus older_result VersionCompare = compare_versions("1.0.0", "2.0.0")
assert_true(older_result == VersionCompare.OLDER)

test_start("Version Comparison - Newer")
sus newer_result VersionCompare = compare_versions("2.0.0", "1.0.0")
assert_true(newer_result == VersionCompare.NEWER)

test_start("Version Compatibility Check - Compatible")
sus is_compat lit = is_compatible_version("1.0.0", "2.0.0")
assert_true(is_compat)

test_start("Version Compatibility Check - Equal")
sus is_equal_compat lit = is_compatible_version("1.0.0", "1.0.0")
assert_true(is_equal_compat)

test_start("Latest Version Lookup - cursed")
sus cursed_version tea = get_latest_version("cursed")
assert_eq_string(cursed_version, "27.0.0")

test_start("Latest Version Lookup - stdlib")
sus stdlib_version tea = get_latest_version("stdlib")
assert_eq_string(stdlib_version, "2.5.0")

test_start("Latest Version Lookup - unknown")
sus unknown_version tea = get_latest_version("unknown")
assert_eq_string(unknown_version, "1.0.0")

test_start("Semantic Version Validation - Valid")
sus valid_semver lit = validate_semver("1.2.3")
assert_true(valid_semver)

test_start("Semantic Version Validation - Invalid")
sus invalid_semver lit = validate_semver("1.2")
assert_false(invalid_semver)

fr fr Test Dependency Resolution
test_start("Single Dependency Resolution")
sus test_dep PackageDep
test_dep.name = "cursed"
test_dep.version = "27.0.0"
test_dep.source = "registry"
test_dep.dependencies = []
sus resolve_result lit = resolve_dependency(test_dep)
assert_true(resolve_result)

test_start("Multiple Dependencies Resolution - Success")
sus deps []PackageDep = []
sus dep1 PackageDep
dep1.name = "cursed"
dep1.version = "27.0.0"
dep1.source = "registry"
dep1.dependencies = []
deps = append(deps, dep1)

sus dep2 PackageDep
dep2.name = "stdlib"
dep2.version = "2.5.0"
dep2.source = "registry"
dep2.dependencies = []
deps = append(deps, dep2)

sus multi_resolve lit = resolve_dependencies(deps)
assert_true(multi_resolve)

test_start("Circular Dependency Check - No Circular")
sus no_circular lit = check_circular_dependencies(deps)
assert_true(no_circular)

test_start("Circular Dependency Check - With Circular")
sus circular_deps []PackageDep = []
sus circ_dep1 PackageDep
circ_dep1.name = "package_a"
circ_dep1.version = "1.0.0"
circular_deps = append(circular_deps, circ_dep1)

sus circ_dep2 PackageDep
circ_dep2.name = "package_a" fr fr Same name - circular
circ_dep2.version = "2.0.0"
circular_deps = append(circular_deps, circ_dep2)

sus has_circular lit = check_circular_dependencies(circular_deps)
assert_false(has_circular)

test_start("Dependency Tree Building")
sus config_for_tree Config
config_for_tree.dependencies = deps
sus dep_tree []PackageDep = build_dependency_tree(config_for_tree)
assert_true(len(dep_tree) == 2)

fr fr Test Package Manager Functions
test_start("Package Manager Creation")
sus manager PackageManager = create_package_manager("test_config.json")
assert_eq_string(manager.config.package_name, "default_package")
assert_eq_string(manager.cache_dir, ".cursed/cache")
assert_eq_string(manager.registry_url, "https://packages.cursed.dev")
assert_true(len(manager.installed_packages) == 0)

test_start("Package Installation")
sus install_success lit = install_package(&manager, "test_package", "1.0.0")
assert_true(install_success)
assert_true(len(manager.installed_packages) == 1)
assert_eq_string(manager.installed_packages[0].name, "test_package")
assert_eq_string(manager.installed_packages[0].version, "1.0.0")

test_start("Package Installation - Duplicate")
sus duplicate_install lit = install_package(&manager, "test_package", "2.0.0")
assert_false(duplicate_install) fr fr Should fail due to duplicate

test_start("Package Update")
sus update_success lit = update_package(&manager, "test_package")
assert_true(update_success)
assert_eq_string(manager.installed_packages[0].version, "1.0.0") fr fr Updated to latest

test_start("Package Uninstallation")
sus uninstall_success lit = uninstall_package(&manager, "test_package")
assert_true(uninstall_success)
assert_true(len(manager.installed_packages) == 0)

test_start("Package Uninstallation - Not Found")
sus uninstall_missing lit = uninstall_package(&manager, "missing_package")
assert_true(uninstall_missing) fr fr Should succeed (no-op)

test_start("List Installed Packages - Empty")
sus empty_list []tea = list_installed_packages(manager)
assert_true(len(empty_list) == 0)

test_start("List Installed Packages - With Packages")
install_package(&manager, "package1", "1.0.0")
install_package(&manager, "package2", "2.0.0")
sus package_list []tea = list_installed_packages(manager)
assert_true(len(package_list) == 2)

fr fr Test Build Automation
test_start("Build Script Execution - Success")
sus build_success lit = run_build_script("make build")
assert_true(build_success)

test_start("Build Script Execution - Failure")
sus build_failure lit = run_build_script("invalid command")
assert_false(build_failure)

test_start("Test Command Execution - Success")
sus test_success lit = run_test_command("npm test")
assert_true(test_success)

test_start("Test Command Execution - Failure")
sus test_failure lit = run_test_command("invalid test")
assert_false(test_failure)

test_start("Build Pipeline Execution")
sus pipeline_config Config
pipeline_config.build_scripts = ["make build", "npm run build"]
pipeline_config.test_commands = ["make test", "npm test"]
sus pipeline_success lit = execute_build_pipeline(pipeline_config)
assert_true(pipeline_success)

fr fr Test Configuration Validation
test_start("Config Validation - Valid Config")
sus valid_config Config
valid_config.package_name = "valid_package"
valid_config.version = "1.0.0"
valid_config.dependencies = []
sus config_valid lit = validate_config(valid_config)
assert_true(config_valid)

test_start("Config Validation - Invalid Package Name")
sus invalid_name_config Config
invalid_name_config.package_name = ""
invalid_name_config.version = "1.0.0"
invalid_name_config.dependencies = []
sus name_invalid lit = validate_config(invalid_name_config)
assert_false(name_invalid)

test_start("Config Validation - Invalid Version")
sus invalid_version_config Config
invalid_version_config.package_name = "test"
invalid_version_config.version = "invalid"
invalid_version_config.dependencies = []
sus version_invalid lit = validate_config(invalid_version_config)
assert_false(version_invalid)

test_start("Package Manager Validation - Valid")
sus valid_manager PackageManager = create_package_manager("test.json")
sus manager_valid lit = validate_package_manager(valid_manager)
assert_true(manager_valid)

fr fr Test Utility Functions
test_start("Lock File Generation")
sus lock_manager PackageManager = create_package_manager("test.json")
install_package(&lock_manager, "dep1", "1.0.0")
install_package(&lock_manager, "dep2", "2.0.0")
sus lock_content tea = generate_lock_file(lock_manager)
assert_true(len(lock_content) > 0)

test_start("Cache Cleanup")
sus cleanup_success lit = clean_cache(&lock_manager)
assert_true(cleanup_success)

test_start("Package Info Retrieval")
sus package_info PackageDep = get_package_info("cursed")
assert_eq_string(package_info.name, "cursed")
assert_eq_string(package_info.version, "27.0.0")
assert_eq_string(package_info.source, "registry")

fr fr Test Registry Functions
test_start("Package Search - cursed")
sus cursed_search []tea = search_packages("cursed")
assert_true(len(cursed_search) == 2)

test_start("Package Search - web")
sus web_search []tea = search_packages("web")
assert_true(len(web_search) == 2)

test_start("Package Search - no results")
sus empty_search []tea = search_packages("nonexistent")
assert_true(len(empty_search) == 0)

test_start("Package Publishing")
sus publish_manager PackageManager = create_package_manager("test.json")
sus publish_success lit = publish_package(publish_manager)
assert_true(publish_success)

fr fr Integration Tests
test_start("Complete Package Workflow")
sus workflow_manager PackageManager = create_package_manager("workflow.json")

fr fr Install packages
assert_true(install_package(&workflow_manager, "dependency1", "1.0.0"))
assert_true(install_package(&workflow_manager, "dependency2", "2.0.0"))

fr fr Validate installation
sus installed_list []tea = list_installed_packages(workflow_manager)
assert_true(len(installed_list) == 2)

fr fr Update packages
assert_true(update_package(&workflow_manager, "dependency1"))

fr fr Generate lock file
sus workflow_lock tea = generate_lock_file(workflow_manager)
assert_true(len(workflow_lock) > 0)

fr fr Clean up
assert_true(uninstall_package(&workflow_manager, "dependency1"))
assert_true(uninstall_package(&workflow_manager, "dependency2"))

test_start("Configuration Format Detection")
fr fr Test different config file extensions
sus json_detected Config = load_config_file("test.json")
assert_eq_string(json_detected.package_name, "default_package")

sus yaml_detected Config = load_config_file("test.yaml")
assert_eq_string(yaml_detected.package_name, "yaml_package")

sus yml_detected Config = load_config_file("test.yml")
assert_eq_string(yml_detected.package_name, "yaml_package")

sus toml_detected Config = load_config_file("test.toml")
assert_eq_string(toml_detected.package_name, "toml_package")

sus default_detected Config = load_config_file("test.unknown")
assert_eq_string(default_detected.package_name, "default_package")

print_test_summary()
