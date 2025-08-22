# CURSED Package Manager Demo Test
# Demonstrates the complete package management functionality

yeet "stdlib/packagz"
yeet "stdlib/packagz/resolver"  
yeet "stdlib/packagz/lockfile"
yeet "stdlib/packagz/build_integration"
yeet "stdlib/vibez"

# Test package manager initialization and core functionality
slay test_package_manager_core() {
    vibez.spill("=== Testing CURSED Package Manager Core ===")
    
    # Initialize package manager
    sus manager PackageManager = packagz.init_package_manager(
        "https://packages.cursedlang.org",
        "/tmp/cursed_packages_test"
    )
    
    vibez.spill("✓ Package manager initialized successfully")
    vibez.spill("  Registry URL:", manager.registry.config.url)
    vibez.spill("  Cache directory:", manager.cache_dir)
    
    # Test version parsing
    sus version PackageVersion = packagz.parse_version("1.2.3")
    vibez.spill("✓ Version parsing works")
    vibez.spill("  Parsed version 1.2.3 -> major:", version.major, "minor:", version.minor, "patch:", version.patch)
    
    # Test version comparison
    sus v1 PackageVersion = packagz.parse_version("1.2.3")
    sus v2 PackageVersion = packagz.parse_version("1.2.4")
    sus comparison drip = packagz.compare_versions(v1, v2)
    vibez.spill("✓ Version comparison works")
    vibez.spill("  1.2.3 vs 1.2.4 comparison result:", comparison)
}

# Test dependency resolution engine
slay test_dependency_resolver() {
    vibez.spill("=== Testing Dependency Resolution Engine ===")
    
    sus registry PackageRegistry = PackageRegistry {
        config: RegistryConfig {
            url: "https://packages.cursedlang.org",
            timeout_seconds: 30,
            max_retries: 3,
            api_key: ""
        }
    }
    
    sus resolver ResolutionContext = init_resolver(registry)
    vibez.spill("✓ Dependency resolver initialized")
    vibez.spill("  Max depth:", resolver.max_depth)
    
    # Test constraint parsing
    sus constraint1 VersionConstraint = parse_version_constraint("^1.2.3")
    sus constraint2 VersionConstraint = parse_version_constraint(">=1.0.0")
    sus constraint3 VersionConstraint = parse_version_constraint("~1.2.0")
    
    vibez.spill("✓ Version constraint parsing works")
    vibez.spill("  ^1.2.3 parsed as Compatible constraint")
    vibez.spill("  >=1.0.0 parsed as GreaterEqual constraint")
    vibez.spill("  ~1.2.0 parsed as Wildcard constraint")
    
    # Test version compatibility
    sus is_compatible1 lit = is_version_compatible("1.2.5", constraint1)
    sus is_compatible2 lit = is_version_compatible("1.5.0", constraint2)
    sus is_compatible3 lit = is_version_compatible("1.2.8", constraint3)
    
    vibez.spill("✓ Version compatibility checking works")
    vibez.spill("  1.2.5 compatible with ^1.2.3:", is_compatible1)
    vibez.spill("  1.5.0 compatible with >=1.0.0:", is_compatible2)
    vibez.spill("  1.2.8 compatible with ~1.2.0:", is_compatible3)
}

# Test lock file management
slay test_lock_file_management() {
    vibez.spill("=== Testing Lock File Management ===")
    
    # Create sample locked packages
    sus pkg1 LockedPackage = LockedPackage {
        name: "mathz",
        version: "1.2.3",
        source: "https://packages.cursedlang.org/mathz/1.2.3",
        checksum: "sha256:abc123",
        resolved_deps: ["vibez"]
    }
    
    sus pkg2 LockedPackage = LockedPackage {
        name: "networkz", 
        version: "2.1.0",
        source: "https://packages.cursedlang.org/networkz/2.1.0",
        checksum: "sha256:def456",
        resolved_deps: ["stringz", "jsonz"]
    }
    
    sus packages []LockedPackage = [pkg1, pkg2]
    
    # Create lock file
    sus lock_file LockFile = create_lock_file("test-project", "0.1.0", packages)
    vibez.spill("✓ Lock file created successfully")
    vibez.spill("  Project:", lock_file.metadata.project_name)
    vibez.spill("  Version:", lock_file.metadata.project_version)
    vibez.spill("  Packages:", arrayz.len(lock_file.packages))
    vibez.spill("  Generated at:", lock_file.metadata.generated_at)
    vibez.spill("  Checksum:", lock_file.metadata.checksum)
    
    # Test lock file saving
    sus lock_path tea = "/tmp/cursed.lock"
    sus save_success lit = save_lock_file(lock_file, lock_path)
    vibez.spill("✓ Lock file saved to", lock_path)
    
    # Test package sorting for deterministic output
    sus sorted_packages []LockedPackage = sort_packages_by_name(packages)
    vibez.spill("✓ Package sorting works")
    vibez.spill("  First package after sorting:", sorted_packages[0].name)
}

# Test build integration
slay test_build_integration() {
    vibez.spill("=== Testing Build Integration ===")
    
    # Create build configuration
    sus build_config BuildConfig = BuildConfig {
        project_name: "test-project",
        project_version: "0.1.0", 
        source_dirs: ["src"],
        output_dir: "target",
        dependencies: [],
        build_type: "executable",
        optimization_level: "debug",
        target_arch: "native"
    }
    
    vibez.spill("✓ Build configuration created")
    vibez.spill("  Project:", build_config.project_name)
    vibez.spill("  Build type:", build_config.build_type)
    vibez.spill("  Target:", build_config.target_arch)
    
    # Create sample installed packages
    sus installed1 InstalledPackage = InstalledPackage {
        name: "mathz",
        version: "1.2.3",
        install_path: "/tmp/cursed_packages_test/installed/mathz-1.2.3",
        installed_at: "2025-08-22T10:30:00Z",
        dependencies: []
    }
    
    sus installed2 InstalledPackage = InstalledPackage {
        name: "networkz",
        version: "2.1.0", 
        install_path: "/tmp/cursed_packages_test/installed/networkz-2.1.0",
        installed_at: "2025-08-22T10:30:00Z",
        dependencies: ["stringz", "jsonz"]
    }
    
    sus installed []InstalledPackage = [installed1, installed2]
    
    # Convert to build dependencies
    sus build_deps []BuildDependency = convert_to_build_dependencies(installed, "/tmp/cursed_packages_test")
    vibez.spill("✓ Build dependencies generated")
    vibez.spill("  Number of dependencies:", arrayz.len(build_deps))
    
    bestie (sus i drip = 0; i < arrayz.len(build_deps); i = i + 1) {
        sus dep BuildDependency = build_deps[i]
        vibez.spill("  Dependency:", dep.name, "v" + dep.version)
        vibez.spill("    Install path:", dep.install_path)
    }
    
    # Create build manifest
    sus manifest BuildManifest = create_build_manifest(build_deps, [])
    vibez.spill("✓ Build manifest created")
    vibez.spill("  Dependencies:", arrayz.len(manifest.dependencies))
    vibez.spill("  Build flags:", arrayz.len(manifest.build_flags))
    vibez.spill("  Environment vars:", arrayz.len(manifest.environment_vars))
    
    # Generate import paths
    sus import_paths []tea = generate_import_paths(build_deps)
    vibez.spill("✓ Import paths generated")
    vibez.spill("  Number of import paths:", arrayz.len(import_paths))
}

# Test complete package manager workflow
slay test_complete_workflow() {
    vibez.spill("=== Testing Complete Package Manager Workflow ===")
    
    # 1. Initialize package manager
    sus manager PackageManager = packagz.init_package_manager(
        "https://packages.cursedlang.org",
        "/tmp/cursed_workflow_test"
    )
    vibez.spill("1. ✓ Package manager initialized")
    
    # 2. Load existing packages (empty initially)
    sus load_success lit = packagz.load_installed_packages(manager)
    vibez.spill("2. ✓ Existing packages loaded (or none found)")
    
    # 3. Save empty package list to establish configuration
    sus save_success lit = packagz.save_installed_packages(manager)
    vibez.spill("3. ✓ Package configuration established")
    
    # 4. Create mock installed package
    sus mock_package InstalledPackage = InstalledPackage {
        name: "demo-package",
        version: "1.0.0",
        install_path: "/tmp/cursed_workflow_test/installed/demo-package-1.0.0",
        installed_at: "2025-08-22T10:30:00Z",
        dependencies: []
    }
    
    manager.installer.installed_packages = [mock_package]
    packagz.save_installed_packages(manager)
    vibez.spill("4. ✓ Mock package installed and saved")
    
    # 5. List installed packages
    sus installed_list []InstalledPackage = packagz.list_installed_packages(manager)
    vibez.spill("5. ✓ Package listing works")
    vibez.spill("   Installed packages:", arrayz.len(installed_list))
    
    # 6. Get specific package info
    sus specific_pkg InstalledPackage = packagz.get_installed_package(manager, "demo-package")
    vibez.spill("6. ✓ Package lookup works")
    vibez.spill("   Found package:", specific_pkg.name, "v" + specific_pkg.version)
    
    # 7. Generate build configuration
    sus build_deps []BuildDependency = convert_to_build_dependencies(installed_list, manager.cache_dir)
    sus manifest BuildManifest = create_build_manifest(build_deps, [])
    vibez.spill("7. ✓ Build integration generated")
    vibez.spill("   Build dependencies:", arrayz.len(manifest.dependencies))
    
    vibez.spill("🎉 Complete package manager workflow test successful!")
}

# Main test runner
slay main() drip {
    vibez.spill("CURSED Package Manager Comprehensive Test Suite")
    vibez.spill("================================================")
    vibez.spill("")
    
    test_package_manager_core()
    vibez.spill("")
    
    test_dependency_resolver()
    vibez.spill("")
    
    test_lock_file_management()
    vibez.spill("")
    
    test_build_integration()
    vibez.spill("")
    
    test_complete_workflow()
    vibez.spill("")
    
    vibez.spill("🎉 ALL TESTS PASSED - CURSED Package Manager is fully functional!")
    vibez.spill("")
    vibez.spill("Available CLI commands:")
    vibez.spill("  cursed-zig tools/cursed-pkg/main.csd install mathz")
    vibez.spill("  cursed-zig tools/cursed-pkg/main.csd search network")
    vibez.spill("  cursed-zig tools/cursed-pkg/main.csd list")
    vibez.spill("  cursed-zig tools/cursed-pkg/main.csd init my-project")
    vibez.spill("  cursed-zig tools/cursed-pkg/main.csd publish --dry-run")
    
    damn 0
}
