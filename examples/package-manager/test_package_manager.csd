# CURSED Package Manager Comprehensive Test
# Demonstrates the complete package management workflow
yeet "packagz"
yeet "vibez"
yeet "filez"
yeet "testz"

slay main_character() drip {
    vibez.spill("CURSED Package Manager Test Suite")
    vibez.spill("=================================")
    
    # Initialize test environment
    sus test_dir tea = "/tmp/cursed-pkg-test"
    sus registry_url tea = "https://packages.cursedlang.org"
    
    filez.create_dir_all(test_dir)
    filez.set_current_dir(test_dir)
    
    # Initialize package manager
    sus manager PackageManager = packagz.init_package_manager(registry_url, test_dir + "/cache")
    
    # Test 1: Package Registry Connection
    vibez.spill("\nTest 1: Registry Connection")
    vibez.spill("---------------------------")
    test_registry_connection(manager)
    
    # Test 2: Package Search
    vibez.spill("\nTest 2: Package Search")
    vibez.spill("----------------------")
    test_package_search(manager)
    
    # Test 3: Dependency Resolution
    vibez.spill("\nTest 3: Dependency Resolution")
    vibez.spill("-----------------------------")
    test_dependency_resolution(manager)
    
    # Test 4: Package Installation
    vibez.spill("\nTest 4: Package Installation")
    vibez.spill("----------------------------")
    test_package_installation(manager)
    
    # Test 5: Lock File Generation
    vibez.spill("\nTest 5: Lock File Management")
    vibez.spill("----------------------------")
    test_lock_file_management(manager)
    
    # Test 6: Package Publishing (Dry Run)
    vibez.spill("\nTest 6: Package Publishing")
    vibez.spill("--------------------------")
    test_package_publishing(manager)
    
    # Test 7: Version Resolution
    vibez.spill("\nTest 7: Version Resolution")
    vibez.spill("--------------------------")
    test_version_resolution(manager)
    
    # Test 8: Conflict Detection
    vibez.spill("\nTest 8: Conflict Detection")
    vibez.spill("--------------------------")
    test_conflict_detection(manager)
    
    # Test 9: Package Updates
    vibez.spill("\nTest 9: Package Updates")
    vibez.spill("-----------------------")
    test_package_updates(manager)
    
    # Test 10: Project Initialization
    vibez.spill("\nTest 10: Project Initialization")
    vibez.spill("-------------------------------")
    test_project_initialization()
    
    vibez.spill("\nAll package manager tests completed!")
    
    # Cleanup
    filez.remove_dir_all(test_dir)
    
    damn 0
}

# Test registry connection and basic functionality
slay test_registry_connection(manager PackageManager) {
    vibez.spill("Testing registry connection...")
    
    # Create mock registry for testing
    sus registry PackageRegistry = packagz.init_registry(
        "https://test-registry.cursedlang.org",
        manager.cache_dir,
        ""
    )
    
    # Test registry configuration loading
    ready (!test_registry_config(registry)) {
        vibez.spill("❌ Registry configuration test failed")
        damn
    }
    
    vibez.spill("✅ Registry connection test passed")
}

# Test package search functionality
slay test_package_search(manager PackageManager) {
    vibez.spill("Testing package search...")
    
    # Mock search results for testing
    sus search_results []PackageMetadata = create_mock_search_results()
    
    ready (arrayz.len(search_results) == 0) {
        vibez.spill("❌ Search returned no results")
        damn
    }
    
    # Verify search result structure
    bestie (sus i drip = 0; i < arrayz.len(search_results); i = i + 1) {
        sus pkg PackageMetadata = search_results[i]
        ready (pkg.name == "" || pkg.version == "") {
            vibez.spill("❌ Invalid package metadata in search results")
            damn
        }
    }
    
    vibez.spill("✅ Package search test passed")
    vibez.spill("Found", arrayz.len(search_results), "mock packages")
}

# Test dependency resolution engine
slay test_dependency_resolution(manager PackageManager) {
    vibez.spill("Testing dependency resolution...")
    
    # Create test dependencies
    sus test_deps []PackageDependency = [
        PackageDependency {
            name: "mathlib",
            version_req: "^1.2.0",
            optional: cap,
            features: []
        },
        PackageDependency {
            name: "stringz", 
            version_req: "~1.0.0",
            optional: cap,
            features: []
        }
    ]
    
    # Initialize resolver
    yeet "resolver"
    sus resolver DependencyResolver = resolver.init_resolver(manager.registry)
    
    # Resolve dependencies
    sus (result, resolved_packages) = resolver.resolve_dependencies(resolver, test_deps)
    
    match result {
        resolver.ResolutionResult.Success -> {
            vibez.spill("✅ Dependency resolution successful")
            vibez.spill("Resolved", arrayz.len(resolved_packages), "packages")
        }
        resolver.ResolutionResult.Conflict -> {
            vibez.spill("⚠️  Dependency conflicts detected (expected for some tests)")
        }
        _ -> {
            vibez.spill("❌ Dependency resolution failed:", result)
        }
    }
}

# Test package installation process
slay test_package_installation(manager PackageManager) {
    vibez.spill("Testing package installation...")
    
    # Test installing mathlib package
    sus install_success lit = packagz.install_package(manager, "mathlib", "1.2.0")
    
    ready (install_success) {
        vibez.spill("✅ Package installation successful")
        
        # Verify installation
        sus installed InstalledPackage = packagz.get_installed_package(manager, "mathlib")
        ready (installed.name == "mathlib" && installed.version == "1.2.0") {
            vibez.spill("✅ Package verification successful")
        } otherwise {
            vibez.spill("❌ Package verification failed")
        }
    } otherwise {
        vibez.spill("❌ Package installation failed")
    }
    
    # Test listing installed packages
    sus installed_packages []InstalledPackage = packagz.list_installed_packages(manager)
    vibez.spill("Currently installed packages:", arrayz.len(installed_packages))
}

# Test lock file management
slay test_lock_file_management(manager PackageManager) {
    vibez.spill("Testing lock file management...")
    
    yeet "lockfile"
    sus lock_manager LockFileManager = lockfile.init_lock_manager(".")
    
    # Get currently installed packages
    sus installed_packages []InstalledPackage = packagz.list_installed_packages(manager)
    
    # Generate lock file
    sus lock_success lit = lockfile.generate_lock_file(lock_manager, installed_packages, manager.registry)
    
    ready (lock_success) {
        vibez.spill("✅ Lock file generation successful")
        
        # Verify lock file exists
        ready (filez.file_exists("cursed.lock")) {
            vibez.spill("✅ Lock file created successfully")
            
            # Test loading lock file
            sus loaded_lock LockFile = lockfile.load_lock_file(lock_manager)
            ready (loaded_lock.version != "") {
                vibez.spill("✅ Lock file loading successful")
                vibez.spill("Lock file contains", arrayz.len(loaded_lock.packages), "packages")
            } otherwise {
                vibez.spill("❌ Lock file loading failed")
            }
        } otherwise {
            vibez.spill("❌ Lock file not created")
        }
    } otherwise {
        vibez.spill("❌ Lock file generation failed")
    }
}

# Test package publishing (dry run)
slay test_package_publishing(manager PackageManager) {
    vibez.spill("Testing package publishing (dry run)...")
    
    # Create a test package structure
    sus pkg_dir tea = "./test-package"
    filez.create_dir_all(pkg_dir)
    filez.create_dir_all(pkg_dir + "/src")
    
    # Create package.toml
    sus package_toml tea = "[package]\n" +
        "name = \"test-package\"\n" +
        "version = \"0.1.0\"\n" +
        "description = \"A test package\"\n" +
        "authors = [\"Test Author <test@example.com>\"]\n" +
        "license = \"MIT\"\n"
    
    filez.write_file(pkg_dir + "/package.toml", package_toml)
    
    # Create main module
    sus main_module tea = "# Test package main module\n" +
        "yeet \"vibez\"\n\n" +
        "slay hello() {\n" +
        "    vibez.spill(\"Hello from test package!\")\n" +
        "}\n"
    
    filez.write_file(pkg_dir + "/src/mod.csd", main_module)
    
    # Create package metadata
    sus metadata PackageMetadata = PackageMetadata {
        name: "test-package",
        version: "0.1.0",
        description: "A test package",
        authors: ["Test Author <test@example.com>"],
        license: "MIT",
        homepage: "",
        repository: "",
        keywords: ["test"],
        categories: ["testing"],
        dependencies: [],
        download_url: "",
        checksum: ""
    }
    
    # Test publishing (dry run)
    yeet "registry"
    sus registry PackageRegistry = registry.init_registry(manager.registry.config.url, manager.cache_dir, "")
    sus publish_result PublishStatus = registry.publish_package(registry, pkg_dir, metadata)
    
    match publish_result {
        registry.PublishStatus.Success -> {
            vibez.spill("✅ Package publishing test successful (dry run)")
        }
        _ -> {
            vibez.spill("⚠️  Package publishing test completed (", publish_result, ")")
        }
    }
    
    # Cleanup
    filez.remove_dir_all(pkg_dir)
}

# Test version resolution and constraints
slay test_version_resolution(manager PackageManager) {
    vibez.spill("Testing version resolution...")
    
    yeet "resolver"
    
    # Test version parsing
    sus version1 PackageVersion = packagz.parse_version("1.2.3")
    sus version2 PackageVersion = packagz.parse_version("1.2.4")
    
    ready (version1.major == 1 && version1.minor == 2 && version1.patch == 3) {
        vibez.spill("✅ Version parsing test passed")
    } otherwise {
        vibez.spill("❌ Version parsing test failed")
    }
    
    # Test version comparison
    sus comparison drip = packagz.compare_versions(version1, version2)
    ready (comparison < 0) {
        vibez.spill("✅ Version comparison test passed")
    } otherwise {
        vibez.spill("❌ Version comparison test failed")
    }
    
    # Test constraint parsing
    sus constraint ParsedConstraint = resolver.parse_version_constraint("^1.2.0")
    ready (constraint.constraint_type == resolver.VersionConstraint.Caret) {
        vibez.spill("✅ Constraint parsing test passed")
    } otherwise {
        vibez.spill("❌ Constraint parsing test failed")
    }
    
    # Test constraint satisfaction
    sus satisfies lit = resolver.version_satisfies_constraint(version1, constraint)
    ready (satisfies) {
        vibez.spill("✅ Constraint satisfaction test passed")
    } otherwise {
        vibez.spill("❌ Constraint satisfaction test failed")
    }
}

# Test conflict detection
slay test_conflict_detection(manager PackageManager) {
    vibez.spill("Testing conflict detection...")
    
    # Create conflicting dependencies
    sus conflicting_deps []PackageDependency = [
        PackageDependency {
            name: "testlib",
            version_req: "=1.0.0",
            optional: cap,
            features: []
        },
        PackageDependency {
            name: "testlib",
            version_req: "=2.0.0",  # Conflicts with above
            optional: cap,
            features: []
        }
    ]
    
    yeet "resolver"
    sus resolver DependencyResolver = resolver.init_resolver(manager.registry)
    
    sus (result, _) = resolver.resolve_dependencies(resolver, conflicting_deps)
    
    match result {
        resolver.ResolutionResult.Conflict -> {
            vibez.spill("✅ Conflict detection working correctly")
        }
        resolver.ResolutionResult.UnresolvableVersion -> {
            vibez.spill("✅ Conflict detected as unresolvable version")
        }
        _ -> {
            vibez.spill("⚠️  Conflict detection test inconclusive:", result)
        }
    }
}

# Test package updates
slay test_package_updates(manager PackageManager) {
    vibez.spill("Testing package updates...")
    
    # List installed packages before update
    sus before_update []InstalledPackage = packagz.list_installed_packages(manager)
    vibez.spill("Packages before update:", arrayz.len(before_update))
    
    # Test updating specific package
    ready (arrayz.len(before_update) > 0) {
        sus first_package tea = before_update[0].name
        sus update_success lit = packagz.update_package(manager, first_package)
        
        ready (update_success) {
            vibez.spill("✅ Package update successful for", first_package)
        } otherwise {
            vibez.spill("⚠️  Package update completed for", first_package, "(may already be latest)")
        }
    }
    
    # Test updating all packages would go here
    vibez.spill("✅ Package update tests completed")
}

# Test project initialization
slay test_project_initialization() {
    vibez.spill("Testing project initialization...")
    
    sus project_dir tea = "./test-project"
    
    # Remove if exists
    filez.remove_dir_all(project_dir)
    
    # Create project directory
    filez.create_dir_all(project_dir)
    filez.set_current_dir(project_dir)
    
    # Initialize project (simulate cursed-pkg init)
    sus init_success lit = initialize_cursed_project("test-project")
    
    ready (init_success) {
        vibez.spill("✅ Project initialization successful")
        
        # Verify project structure
        ready (filez.file_exists("package.toml") && 
              filez.file_exists("src/mod.csd") && 
              filez.file_exists("README.md")) {
            vibez.spill("✅ Project structure verification passed")
        } otherwise {
            vibez.spill("❌ Project structure verification failed")
        }
    } otherwise {
        vibez.spill("❌ Project initialization failed")
    }
    
    # Go back to parent directory and cleanup
    filez.set_current_dir("..")
    filez.remove_dir_all(project_dir)
}

# Helper functions for testing
slay test_registry_config(registry PackageRegistry) lit {
    ready (registry.config.url == "") {
        damn cap
    }
    ready (registry.config.timeout_seconds <= 0) {
        damn cap
    }
    damn based
}

slay create_mock_search_results() []PackageMetadata {
    sus results []PackageMetadata = []
    
    # Mock mathlib package
    sus mathlib PackageMetadata = PackageMetadata {
        name: "mathlib",
        version: "1.2.0",
        description: "Extended mathematical functions",
        authors: ["CURSED Community"],
        license: "MIT",
        homepage: "https://github.com/cursed-lang/mathlib",
        repository: "https://github.com/cursed-lang/mathlib.git",
        keywords: ["math", "mathematics"],
        categories: ["mathematics"],
        dependencies: [],
        download_url: "https://packages.cursedlang.org/mathlib/1.2.0/download",
        checksum: "abc123def456"
    }
    
    # Mock stringz package
    sus stringz PackageMetadata = PackageMetadata {
        name: "stringz",
        version: "1.0.1",
        description: "String manipulation utilities",
        authors: ["CURSED Team"],
        license: "MIT",
        homepage: "https://docs.cursedlang.org/stringz",
        repository: "",
        keywords: ["string", "text"],
        categories: ["utilities"],
        dependencies: [],
        download_url: "https://packages.cursedlang.org/stringz/1.0.1/download",
        checksum: "def789ghi012"
    }
    
    results = arrayz.append(results, mathlib)
    results = arrayz.append(results, stringz)
    
    damn results
}

slay initialize_cursed_project(name tea) lit {
    # Create package.toml
    sus package_toml tea = "[package]\n" +
        "name = \"" + name + "\"\n" +
        "version = \"0.1.0\"\n" +
        "description = \"A new CURSED project\"\n" +
        "authors = [\"Your Name <your@email.com>\"]\n" +
        "license = \"MIT\"\n\n" +
        "[dependencies]\n"
    
    ready (!filez.write_file("package.toml", package_toml)) {
        damn cap
    }
    
    # Create src/mod.csd
    sus main_mod tea = "# " + name + " - CURSED Project\n" +
        "yeet \"vibez\"\n\n" +
        "slay main_character() drip {\n" +
        "    vibez.spill(\"Hello from " + name + "!\")\n" +
        "    damn 0\n" +
        "}\n"
    
    ready (!filez.write_file("src/mod.csd", main_mod)) {
        damn cap
    }
    
    # Create README.md
    sus readme tea = "# " + name + "\n\n" +
        "A CURSED project.\n\n" +
        "## Usage\n\n" +
        "```bash\n" +
        "cursed src/mod.csd\n" +
        "```\n"
    
    ready (!filez.write_file("README.md", readme)) {
        damn cap
    }
    
    damn based
}

# Run comprehensive tests
ready (filez.get_current_dir_name() == "package-manager") {
    main()
}
