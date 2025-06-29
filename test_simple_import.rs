// Simple test for import system without async

use cursed::imports::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing CURSED Import System (Simple)");
    
    // Test import manager creation
    let config = ImportConfig {
        search_paths: vec![PathBuf::from(".")],
        stdlib_path: PathBuf::from("stdlib"),
        enable_package_manager: false,
        cache_enabled: true,
    };
    
    let manager = ImportManager::with_config(config)?;
    println!("✅ Successfully created ImportManager");
    
    // Test import classification
    println!("\nTesting import classification:");
    
    let local_import = manager.classify_import("./test_utils.csd");
    println!("   ./test_utils.csd -> {:?}", local_import);
    
    let stdlib_import = manager.classify_import("std::io");
    println!("   std::io -> {:?}", stdlib_import);
    
    let package_import = manager.classify_import("some_package");
    println!("   some_package -> {:?}", package_import);
    
    // Test module loader
    println!("\nTesting module loading:");
    let mut loader = ModuleLoader::new();
    
    if PathBuf::from("test_utils.csd").exists() {
        match loader.get_or_load_module(&PathBuf::from("test_utils.csd")) {
            Ok(module) => {
                println!("✅ Successfully loaded module: {}", module.name);
                println!("   Symbols: {:?}", module.symbols);
            }
            Err(e) => {
                println!("❌ Failed to load module: {}", e);
            }
        }
    } else {
        println!("   test_utils.csd not found, skipping load test");
    }
    
    // Test package resolver
    println!("\nTesting package resolver:");
    let mut package_resolver = PackageImportResolver::new();
    package_resolver.add_package("test_package".to_string(), PathBuf::from("/test/path"));
    
    println!("   Has test_package: {}", package_resolver.has_package("test_package"));
    println!("   Has missing_package: {}", package_resolver.has_package("missing_package"));
    
    println!("\n🎉 Import system basic tests completed!");
    Ok(())
}
