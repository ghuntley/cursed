//! Core test for CURSED import system

use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CURSED Import System Core Test ===\n");

    // Test import resolver configuration
    let config = cursed::imports::ImportConfig::default();
    println!("✓ Created default import config");
    println!("  Search paths: {:?}", config.search_paths);
    println!("  Stdlib path: {:?}", config.stdlib_path);

    // Test import resolver creation
    match cursed::imports::ImportResolver::new() {
        Ok(resolver) => {
            println!("✓ Created import resolver successfully");
            
            // Test import classification
            let test_cases = vec![
                ("math_utils", "Package or Local"),
                ("./helpers/string_utils", "Local"),
                ("std::io", "Stdlib"),
                ("cursed::collections", "Stdlib"),
                ("http_client@1.2.3", "Package with version"),
            ];
            
            println!("\n--- Testing Import Classification ---");
            for (import_path, expected) in test_cases {
                match resolver.classify_import(import_path) {
                    Ok(source) => {
                        println!("✓ {} -> {:?} ({})", import_path, source, expected);
                    }
                    Err(e) => {
                        println!("✗ {} -> Error: {}", import_path, e);
                    }
                }
            }
            
            // Test stats
            let stats = resolver.get_stats();
            println!("\n--- Import Resolver Stats ---");
            println!("  Cached modules: {}", stats.cached_modules);
            println!("  Cached resolutions: {}", stats.cached_resolutions);
            println!("  Failed imports: {}", stats.failed_imports);
            println!("  Compilation depth: {}", stats.compilation_depth);
        }
        Err(e) => {
            println!("✗ Failed to create import resolver: {}", e);
        }
    }

    // Test module validation utilities
    println!("\n--- Testing Module Validation ---");
    
    let test_files = vec![
        "math_utils.csd",
        "helpers/string_utils.csd",
        "test_import_system.csd",
    ];
    
    for file_path in test_files {
        let path = PathBuf::from(file_path);
        if path.exists() {
            match cursed::imports::validate_module_file(&path) {
                Ok(true) => println!("✓ {} is a valid CURSED module", file_path),
                Ok(false) => println!("✗ {} is not a valid CURSED module", file_path),
                Err(e) => println!("✗ Error validating {}: {}", file_path, e),
            }
        } else {
            println!("⚠ {} does not exist", file_path);
        }
    }

    // Test module discovery
    println!("\n--- Testing Module Discovery ---");
    match cursed::imports::find_modules_in_directory(&PathBuf::from(".")) {
        Ok(modules) => {
            println!("✓ Found {} CURSED modules in current directory:", modules.len());
            for module in modules.iter().take(10) { // Show first 10
                println!("  - {}", module.display());
            }
            if modules.len() > 10 {
                println!("  ... and {} more", modules.len() - 10);
            }
        }
        Err(e) => {
            println!("✗ Error finding modules: {}", e);
        }
    }

    println!("\n=== Core Test Complete ===");
    Ok(())
}
