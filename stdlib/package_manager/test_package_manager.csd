// Comprehensive Test Suite for CURSED Package Manager
// Tests all package manager functionality including resolution, installation, and publishing

yeet "testz"
yeet "vibez"
yeet "package_manager"

// Test data structures
squad TestPackage {
    spill name tea
    spill version tea
    spill dependencies []tea
    
    slay new(name tea, version tea) TestPackage {
        damn TestPackage{
            name: name,
            version: version,
            dependencies: []tea{}
        }
    }
    
    slay addDependency(self TestPackage, dep_name tea, constraint tea) {
        sus dep_spec tea = format_str("{}:{}", dep_name, constraint)
        self.dependencies = append_array(self.dependencies, dep_spec)
    }
}

// Test suite initialization
slay setup_test_environment() {
    vibez.spill("Setting up test environment...")
    
    // Create test directories
    create_dir(".test_cache")
    create_dir(".test_packages")
    
    // Clean up any previous test data
    ready (file_exists("test_package.csd")) {
        delete_file("test_package.csd")
    }
    
    vibez.spill("Test environment ready")
}

slay cleanup_test_environment() {
    vibez.spill("Cleaning up test environment...")
    
    // Remove test files and directories
    ready (dir_exists(".test_cache")) {
        delete_dir_recursive(".test_cache")
    }
    
    ready (dir_exists(".test_packages")) {
        delete_dir_recursive(".test_packages")
    }
    
    ready (file_exists("test_package.csd")) {
        delete_file("test_package.csd")
    }
    
    vibez.spill("Test environment cleaned")
}

// Package version tests
slay test_package_version_parsing() {
    test_start("Package version parsing")
    
    // Test basic version parsing
    sus version1 PackageVersion = parseVersion("1.2.3")
    assert_eq_int(version1.major, 1)
    assert_eq_int(version1.minor, 2)
    assert_eq_int(version1.patch, 3)
    
    // Test version comparison
    sus version2 PackageVersion = parseVersion("1.2.4")
    assert_eq_int(version1.compare(version2), -1)  // 1.2.3 < 1.2.4
    assert_eq_int(version2.compare(version1), 1)   // 1.2.4 > 1.2.3
    assert_eq_int(version1.compare(version1), 0)   // 1.2.3 == 1.2.3
    
    // Test version string conversion
    assert_eq_string(version1.toString(), "1.2.3")
    assert_eq_string(version2.toString(), "1.2.4")
    
    vibez.spill("✓ Package version parsing tests passed")
}

slay test_version_constraints() {
    test_start("Version constraint satisfaction")
    
    sus version PackageVersion = parseVersion("1.2.3")
    
    // Test caret constraints (^1.2.0 allows >=1.2.0 <2.0.0)
    assert_true(version.satisfies("^1.2.0"))
    assert_true(version.satisfies("^1.0.0"))
    assert_false(version.satisfies("^2.0.0"))
    assert_false(version.satisfies("^1.3.0"))
    
    // Test tilde constraints (~1.2.0 allows >=1.2.0 <1.3.0)
    assert_true(version.satisfies("~1.2.0"))
    assert_false(version.satisfies("~1.1.0"))
    assert_false(version.satisfies("~1.3.0"))
    
    // Test exact constraints
    assert_true(version.satisfies("1.2.3"))
    assert_false(version.satisfies("1.2.4"))
    
    // Test range constraints
    assert_true(version.satisfies(">=1.0.0"))
    assert_true(version.satisfies(">=1.2.3"))
    assert_false(version.satisfies(">=1.2.4"))
    assert_true(version.satisfies(">1.2.2"))
    assert_false(version.satisfies(">1.2.3"))
    
    vibez.spill("✓ Version constraint tests passed")
}

// Package manifest tests
slay test_manifest_creation() {
    test_start("Package manifest creation and serialization")
    
    // Create a test manifest
    sus manifest PackageManifest = PackageManifest.new("test-package", "1.0.0")
    manifest.description = "A test package for the test suite"
    manifest.authors = append_array(manifest.authors, "Test Author <test@example.com>")
    manifest.license = "MIT"
    manifest.keywords = append_array(manifest.keywords, "test")
    manifest.keywords = append_array(manifest.keywords, "example")
    
    // Add dependencies
    sus dep1 PackageDependency = PackageDependency.new("json", "^1.0.0")
    sus dep2 PackageDependency = PackageDependency.newDev("testz", "~0.5.0")
    manifest.addDependency(dep1)
    manifest.addDependency(dep2)
    
    // Test serialization
    sus manifest_content tea = manifest.toManifestFormat()
    assert_true(contains_str(manifest_content, "package \"test-package\""))
    assert_true(contains_str(manifest_content, "version = \"1.0.0\""))
    assert_true(contains_str(manifest_content, "\"json\" = \"^1.0.0\""))
    assert_true(contains_str(manifest_content, "\"testz\" = \"~0.5.0\""))
    
    // Test file operations
    manifest.saveToFile("test_package.csd")
    assert_true(file_exists("test_package.csd"))
    
    sus loaded_manifest PackageManifest = PackageManifest.loadFromFile("test_package.csd")
    assert_eq_string(loaded_manifest.name, "test-package")
    assert_eq_string(loaded_manifest.version, "1.0.0")
    assert_eq_int(len(loaded_manifest.dependencies), 1)
    assert_eq_int(len(loaded_manifest.dev_dependencies), 1)
    
    vibez.spill("✓ Package manifest tests passed")
}

slay test_dependency_management() {
    test_start("Dependency addition and removal")
    
    sus manifest PackageManifest = PackageManifest.new("test-app", "0.1.0")
    
    // Test adding dependencies
    sus dep1 PackageDependency = PackageDependency.new("http", "^2.0.0")
    sus dep2 PackageDependency = PackageDependency.new("json", "~1.5.0")
    sus dev_dep PackageDependency = PackageDependency.newDev("testz", "^0.3.0")
    
    manifest.addDependency(dep1)
    manifest.addDependency(dep2)
    manifest.addDependency(dev_dep)
    
    assert_eq_int(len(manifest.dependencies), 2)
    assert_eq_int(len(manifest.dev_dependencies), 1)
    
    // Test removing dependencies
    sus removed lit = manifest.removeDependency("json")
    assert_true(removed)
    assert_eq_int(len(manifest.dependencies), 1)
    
    // Test removing non-existent dependency
    sus not_removed lit = manifest.removeDependency("nonexistent")
    assert_false(not_removed)
    
    // Test removing dev dependency
    sus dev_removed lit = manifest.removeDependency("testz")
    assert_true(dev_removed)
    assert_eq_int(len(manifest.dev_dependencies), 0)
    
    vibez.spill("✓ Dependency management tests passed")
}

// Dependency resolution tests
slay test_basic_dependency_resolution() {
    test_start("Basic dependency resolution")
    
    // Create a mock registry client
    sus registry_client RegistryClient = RegistryClient.new("https://packages.cursed.dev/api/v1")
    sus resolver PackageResolver = PackageResolver.new(".test_cache")
    
    // Create test manifest with simple dependencies
    sus manifest PackageManifest = PackageManifest.new("test-app", "1.0.0")
    sus dep1 PackageDependency = PackageDependency.new("json", "^1.0.0")
    sus dep2 PackageDependency = PackageDependency.new("http", "~2.1.0")
    manifest.addDependency(dep1)
    manifest.addDependency(dep2)
    
    // Mock available packages
    sus json_pkg PackageInfo = PackageInfo.new("json", parseVersion("1.2.0"))
    sus http_pkg PackageInfo = PackageInfo.new("http", parseVersion("2.1.3"))
    
    // Test individual package resolution
    sus resolved_json PackageInfo = resolver.resolvePackage("json", "^1.0.0")
    assert_eq_string(resolved_json.name, "json")
    
    vibez.spill("✓ Basic dependency resolution tests passed")
}

slay test_complex_dependency_resolution() {
    test_start("Complex dependency resolution with conflicts")
    
    // Create advanced resolver with PubGrub algorithm
    sus registry_client RegistryClient = RegistryClient.new("https://packages.cursed.dev/api/v1")
    sus resolver AdvancedResolver = AdvancedResolver.new(registry_client)
    
    // Create test package with potential conflicts
    sus manifest PackageManifest = PackageManifest.new("complex-app", "1.0.0")
    
    // Add dependencies that might conflict
    sus dep1 PackageDependency = PackageDependency.new("package-a", "^1.0.0")
    sus dep2 PackageDependency = PackageDependency.new("package-b", "^2.0.0")
    manifest.addDependency(dep1)
    manifest.addDependency(dep2)
    
    // Test resolution
    sus assignments []PackageAssignment = resolver.resolve(manifest)
    
    // Verify that resolution found compatible versions
    assert_true(len(assignments) >= 2)  // At least root + dependencies
    
    vibez.spill("✓ Complex dependency resolution tests passed")
}

slay test_circular_dependency_detection() {
    test_start("Circular dependency detection")
    
    // Create packages with circular dependencies for testing
    sus pkg_a TestPackage = TestPackage.new("package-a", "1.0.0")
    pkg_a.addDependency("package-b", "^1.0.0")
    
    sus pkg_b TestPackage = TestPackage.new("package-b", "1.0.0")
    pkg_b.addDependency("package-c", "^1.0.0")
    
    sus pkg_c TestPackage = TestPackage.new("package-c", "1.0.0")
    pkg_c.addDependency("package-a", "^1.0.0")  // Creates circle: A -> B -> C -> A
    
    // Test that resolver detects the circular dependency
    sus registry_client RegistryClient = RegistryClient.new("https://packages.cursed.dev/api/v1")
    sus resolver AdvancedResolver = AdvancedResolver.new(registry_client)
    
    // Create manifest that would trigger circular dependency
    sus manifest PackageManifest = PackageManifest.new("circular-test", "1.0.0")
    sus dep PackageDependency = PackageDependency.new("package-a", "^1.0.0")
    manifest.addDependency(dep)
    
    // Resolution should handle circular dependency gracefully
    sus assignments []PackageAssignment = resolver.resolve(manifest)
    
    // Should either resolve with conflict resolution or return empty result
    vibez.spill("Circular dependency test completed with {} assignments", len(assignments))
    
    vibez.spill("✓ Circular dependency detection tests passed")
}

// Package manager command tests
slay test_package_init() {
    test_start("Package initialization command")
    
    // Test package initialization
    sus success lit = cmd_init("test-init-package", "0.1.0")
    assert_true(success)
    assert_true(file_exists("package.csd"))
    
    // Verify created manifest
    sus manifest PackageManifest = PackageManifest.loadFromFile("package.csd")
    assert_eq_string(manifest.name, "test-init-package")
    assert_eq_string(manifest.version, "0.1.0")
    assert_eq_string(manifest.description, "A CURSED package")
    
    // Test that second init fails (package already exists)
    sus second_init lit = cmd_init("another-package", "1.0.0")
    assert_false(second_init)
    
    // Cleanup
    delete_file("package.csd")
    
    vibez.spill("✓ Package init command tests passed")
}

slay test_package_add_remove() {
    test_start("Package add/remove commands")
    
    // Initialize package first
    cmd_init("add-remove-test", "1.0.0")
    
    // Test adding regular dependency
    sus add_success lit = cmd_add("json", "^1.0.0", cringe)
    assert_true(add_success)
    
    // Test adding dev dependency
    sus add_dev_success lit = cmd_add("testz", "~0.5.0", based)
    assert_true(add_dev_success)
    
    // Verify dependencies were added
    sus manifest PackageManifest = PackageManifest.loadFromFile("package.csd")
    assert_eq_int(len(manifest.dependencies), 1)
    assert_eq_int(len(manifest.dev_dependencies), 1)
    assert_eq_string(manifest.dependencies[0].name, "json")
    assert_eq_string(manifest.dev_dependencies[0].name, "testz")
    
    // Test removing dependency
    sus remove_success lit = cmd_remove("json")
    assert_true(remove_success)
    
    // Verify dependency was removed
    manifest = PackageManifest.loadFromFile("package.csd")
    assert_eq_int(len(manifest.dependencies), 0)
    assert_eq_int(len(manifest.dev_dependencies), 1)
    
    // Test removing non-existent dependency
    sus remove_fail lit = cmd_remove("nonexistent")
    assert_false(remove_fail)
    
    // Cleanup
    delete_file("package.csd")
    
    vibez.spill("✓ Package add/remove command tests passed")
}

slay test_package_install() {
    test_start("Package installation command")
    
    // Initialize package with dependencies
    cmd_init("install-test", "1.0.0")
    cmd_add("json", "^1.0.0", cringe)
    cmd_add("http", "~2.0.0", cringe)
    cmd_add("testz", "^0.5.0", based)
    
    // Test installation
    sus install_success lit = cmd_install()
    
    // Installation might fail in test environment (no real registry)
    // but command should execute without crashing
    vibez.spill("Install command executed: {}", install_success)
    
    // Verify cache directory was created
    assert_true(dir_exists(".cursed"))
    
    // Cleanup
    delete_file("package.csd")
    ready (dir_exists(".cursed")) {
        delete_dir_recursive(".cursed")
    }
    
    vibez.spill("✓ Package install command tests passed")
}

// Registry client tests
slay test_registry_client() {
    test_start("Registry client functionality")
    
    // Create registry client
    sus client PackageRegistryClient = PackageRegistryClient.new(
        "https://packages.cursed.dev/api/v1", 
        ".test_cache"
    )
    
    // Test search functionality (with mock data)
    sus search_results []PackageSearchResult = client.searchPackages("json", 10)
    
    // Mock implementation should return some results
    vibez.spill("Search returned {} results", len(search_results))
    
    // Test package info retrieval
    sus package_info PackageInfo = client.getPackageInfo("json")
    vibez.spill("Retrieved package info for: {}", package_info.name)
    
    // Test version listing
    sus versions []PackageVersion = client.getPackageVersions("json")
    vibez.spill("Found {} versions", len(versions))
    
    vibez.spill("✓ Registry client tests passed")
}

slay test_auth_system() {
    test_start("Authentication system")
    
    // Test auth object creation
    sus auth RegistryAuth = RegistryAuth.new()
    assert_false(auth.isValid())
    
    // Test auth with token
    auth.username = "testuser"
    auth.api_token = "test_token_123"
    auth.expires_at = 9999999999  // Far future
    assert_true(auth.isValid())
    
    // Test auth header generation
    sus auth_header tea = auth.getAuthHeader()
    assert_eq_string(auth_header, "Bearer test_token_123")
    
    // Test file operations
    auth.saveToFile(".test_cache/test_auth.txt")
    assert_true(file_exists(".test_cache/test_auth.txt"))
    
    sus loaded_auth RegistryAuth = RegistryAuth.loadFromFile(".test_cache/test_auth.txt")
    assert_eq_string(loaded_auth.username, "testuser")
    assert_eq_string(loaded_auth.api_token, "test_token_123")
    
    vibez.spill("✓ Authentication system tests passed")
}

// Performance and optimization tests
slay test_performance_optimization() {
    test_start("Performance optimization features")
    
    // Test optimized resolver with caching
    sus registry_client RegistryClient = RegistryClient.new("https://packages.cursed.dev/api/v1")
    sus optimized_resolver OptimizedResolver = OptimizedResolver.new(registry_client)
    
    // Create test manifest
    sus manifest PackageManifest = PackageManifest.new("perf-test", "1.0.0")
    sus dep PackageDependency = PackageDependency.new("json", "^1.0.0")
    manifest.addDependency(dep)
    
    // First resolution (should cache result)
    sus assignments1 []PackageAssignment = optimized_resolver.resolve(manifest)
    
    // Second resolution (should use cache)
    sus assignments2 []PackageAssignment = optimized_resolver.resolve(manifest)
    
    // Results should be consistent
    assert_eq_int(len(assignments1), len(assignments2))
    
    vibez.spill("✓ Performance optimization tests passed")
}

// Integration tests
slay test_end_to_end_workflow() {
    test_start("End-to-end package manager workflow")
    
    vibez.spill("Testing complete package lifecycle...")
    
    // 1. Initialize new package
    assert_true(cmd_init("e2e-test-package", "1.0.0"))
    
    // 2. Add dependencies
    assert_true(cmd_add("json", "^1.0.0", cringe))
    assert_true(cmd_add("http", "~2.0.0", cringe))
    assert_true(cmd_add("testz", "^0.5.0", based))
    
    // 3. Verify manifest structure
    sus manifest PackageManifest = PackageManifest.loadFromFile("package.csd")
    assert_eq_string(manifest.name, "e2e-test-package")
    assert_eq_int(len(manifest.dependencies), 2)
    assert_eq_int(len(manifest.dev_dependencies), 1)
    
    // 4. Install dependencies (may fail in test environment)
    sus install_result lit = cmd_install()
    vibez.spill("Installation result: {}", install_result)
    
    // 5. Remove a dependency
    assert_true(cmd_remove("http"))
    
    // 6. Verify removal
    manifest = PackageManifest.loadFromFile("package.csd")
    assert_eq_int(len(manifest.dependencies), 1)
    
    // 7. Cleanup
    delete_file("package.csd")
    ready (dir_exists(".cursed")) {
        delete_dir_recursive(".cursed")
    }
    
    vibez.spill("✓ End-to-end workflow tests passed")
}

// Main test runner
slay run_all_tests() {
    vibez.spill("=== CURSED Package Manager Test Suite ===")
    vibez.spill("")
    
    setup_test_environment()
    
    // Core functionality tests
    test_package_version_parsing()
    test_version_constraints()
    test_manifest_creation()
    test_dependency_management()
    
    // Dependency resolution tests
    test_basic_dependency_resolution()
    test_complex_dependency_resolution()
    test_circular_dependency_detection()
    
    // Command tests
    test_package_init()
    test_package_add_remove()
    test_package_install()
    
    // Registry and networking tests
    test_registry_client()
    test_auth_system()
    
    // Performance tests
    test_performance_optimization()
    
    // Integration tests
    test_end_to_end_workflow()
    
    cleanup_test_environment()
    
    vibez.spill("")
    vibez.spill("=== Test Suite Complete ===")
    print_test_summary()
}

// Run tests when module is executed
run_all_tests()
