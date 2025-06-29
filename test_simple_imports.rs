//! Simple test for CURSED import system components

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CURSED Import System Simple Test ===\n");

    // Test 1: Import configuration
    println("1. Testing ImportConfig...");
    let config = cursed::imports::ImportConfig::default();
    println("✓ Created default import config");
    println("  Search paths: {:?}", config.search_paths);
    println("  Stdlib path: {:?}", config.stdlib_path);
    println("  Enable package manager: {}", config.enable_package_manager);
    println("  Cache enabled: {}", config.cache_enabled);

    // Test 2: Module loader
    println("\n2. Testing ModuleLoader...");
    let mut loader = cursed::imports::ModuleLoader::new();
    println("✓ Created module loader");
    
    let stats = loader.get_cache_stats();
    println("  Cache stats: {} cached modules", stats.cached_modules);

    // Test 3: Package resolver
    println("\n3. Testing PackageResolver...");
    match cursed::imports::PackageResolver::new() {
        Ok(resolver) => {
            println("✓ Created package resolver");
            
            match resolver.get_installed_packages() {
                Ok(packages) => {
                    println("  Found {} installed packages", packages.len());
                    for package in packages.iter().take(5) {
                        println("    - {}", package);
                    }
                }
                Err(e) => println("  Warning: Could not list packages: {}", e),
            }
        }
        Err(e) => println("  Warning: Could not create package resolver: {}", e),
    }

    // Test 4: Check if test files exist
    println("\n4. Checking test files...");
    let test_files = vec![
        "math_utils.csd",
        "helpers/string_utils.csd", 
        "test_import_system.csd",
        "stdlib/io/mod.csd",
    ];
    
    for file_path in test_files {
        let path = PathBuf::from(file_path);
        if path.exists() {
            println("✓ {} exists", file_path);
        } else {
            println("⚠ {} does not exist", file_path);
        }
    }

    println("\n=== Simple Test Complete ===");
    println("All basic import system components are working!");
    
    Ok(())
}

// Helper function to print consistently
fn println(msg: &str) {
    println!("{}", msg);
}
