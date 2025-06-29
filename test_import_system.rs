// Test the import system implementation

use cursed::imports::*;
use cursed::ast::*;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing CURSED Import System");
    
    // Create import manager
    let config = ImportConfig {
        search_paths: vec![std::path::PathBuf::from(".")],
        stdlib_path: std::path::PathBuf::from("stdlib"),
        enable_package_manager: false,
        cache_enabled: true,
    };
    
    let mut manager = ImportManager::with_config(config)?;
    
    // Test basic import
    let import = ImportStatement {
        path: "test_utils.csd".to_string(),
        alias: None,
        items: vec![],
    };
    
    println!("Resolving import: {}", import.path);
    
    let rt = tokio::runtime::Runtime::new()?;
    let result = rt.block_on(async {
        let mut processing = HashSet::new();
        manager.resolve_single_import(&import, &mut processing).await
    });
    
    match result {
        Ok(resolved) => {
            println!("✅ Successfully resolved import!");
            println!("   Module name: {}", resolved.module.name);
            println!("   Module path: {}", resolved.path.display());
            println!("   Exported symbols: {:?}", resolved.module.symbols);
        }
        Err(e) => {
            println!("❌ Failed to resolve import: {}", e);
            return Err(e.into());
        }
    }
    
    // Test multiple imports
    let imports = vec![
        ImportStatement {
            path: "test_utils.csd".to_string(),
            alias: None,
            items: vec![],
        },
        ImportStatement {
            path: "test_math_helpers.csd".to_string(),
            alias: None,
            items: vec![],
        },
    ];
    
    println!("\nResolving multiple imports...");
    
    let result = rt.block_on(async {
        manager.resolve_imports(&imports).await
    });
    
    match result {
        Ok(resolved_imports) => {
            println!("✅ Successfully resolved {} imports!", resolved_imports.len());
            for (i, resolved) in resolved_imports.iter().enumerate() {
                println!("   Import {}: {} (symbols: {:?})", 
                    i + 1, 
                    resolved.module.name, 
                    resolved.module.symbols
                );
            }
        }
        Err(e) => {
            println!("❌ Failed to resolve imports: {}", e);
            return Err(e.into());
        }
    }
    
    // Test caching
    println!("\nTesting import caching...");
    println!("   test_utils.csd cached: {}", manager.is_cached("test_utils.csd"));
    println!("   test_math_helpers.csd cached: {}", manager.is_cached("test_math_helpers.csd"));
    
    println!("\n🎉 Import system test completed successfully!");
    Ok(())
}
