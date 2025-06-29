//! Integration tests for the CURSED import system
//! 
//! These tests verify that the entire import pipeline works correctly,
//! from parsing import statements to resolving and compiling modules.

use super::*;
use crate::ast::ImportStatement;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::path::PathBuf;

/// Test that the import system can resolve local modules
#[tokio::test]
async fn test_resolve_local_imports() -> Result<()> {
    // Create import resolver with current directory as search path
    let mut config = ImportConfig::default();
    config.search_paths = vec![PathBuf::from(".")];
    
    let mut resolver = ImportResolver::with_config(config)?;
    
    // Test resolving math_utils module
    let import = ImportStatement {
        path: "math_utils".to_string(),
        alias: None,
        items: vec![],
    };
    
    match resolver.resolve_single_import(&import).await {
        Ok(resolved) => {
            println!("✓ Successfully resolved math_utils module");
            println!("  Path: {}", resolved.path.display());
            println!("  Symbols: {:?}", resolved.symbols);
            assert!(resolved.symbols.contains(&"add".to_string()));
            assert!(resolved.symbols.contains(&"multiply".to_string()));
            assert!(resolved.symbols.contains(&"PI".to_string()));
        }
        Err(e) => {
            println!("✗ Failed to resolve math_utils: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

/// Test that the import system can resolve relative imports
#[tokio::test]
async fn test_resolve_relative_imports() -> Result<()> {
    let mut config = ImportConfig::default();
    config.search_paths = vec![PathBuf::from(".")];
    
    let mut resolver = ImportResolver::with_config(config)?;
    
    // Test resolving relative string_utils module
    let import = ImportStatement {
        path: "./helpers/string_utils".to_string(),
        alias: None,
        items: vec![],
    };
    
    match resolver.resolve_single_import(&import).await {
        Ok(resolved) => {
            println!("✓ Successfully resolved relative string_utils module");
            println!("  Path: {}", resolved.path.display());
            println!("  Symbols: {:?}", resolved.symbols);
            assert!(resolved.symbols.contains(&"concat".to_string()));
            assert!(resolved.symbols.contains(&"length".to_string()));
        }
        Err(e) => {
            println!("✗ Failed to resolve string_utils: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

/// Test that the import system can resolve stdlib imports
#[tokio::test]
async fn test_resolve_stdlib_imports() -> Result<()> {
    let mut config = ImportConfig::default();
    config.stdlib_path = PathBuf::from("stdlib");
    
    let mut resolver = ImportResolver::with_config(config)?;
    
    // Test resolving stdlib io module
    let import = ImportStatement {
        path: "std::io".to_string(),
        alias: None,
        items: vec![],
    };
    
    match resolver.resolve_single_import(&import).await {
        Ok(resolved) => {
            println!("✓ Successfully resolved std::io module");
            println!("  Path: {}", resolved.path.display());
            println!("  Symbols: {:?}", resolved.symbols);
            assert!(resolved.symbols.contains(&"print".to_string()));
            assert!(resolved.symbols.contains(&"read_line".to_string()));
        }
        Err(e) => {
            println!("✗ Failed to resolve std::io: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

/// Test parsing and resolving imports from a complete program
#[tokio::test]
async fn test_full_program_import_resolution() -> Result<()> {
    // Read and parse the test program
    let source = std::fs::read_to_string("test_import_system.csd")
        .map_err(|e| CursedError::ImportError(format!("Failed to read test file: {}", e)))?;
    
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    
    println!("✓ Successfully parsed test program");
    println!("  Found {} imports", program.imports.len());
    
    // Resolve all imports
    let resolved_imports = resolve_program_imports(&program).await?;
    
    println!("✓ Successfully resolved all imports");
    for (i, resolved) in resolved_imports.iter().enumerate() {
        println!("  Import {}: {} -> {}", i + 1, program.imports[i].path, resolved.path.display());
        println!("    Symbols: {:?}", resolved.symbols);
    }
    
    assert_eq!(resolved_imports.len(), 2);
    
    Ok(())
}

/// Test import classification
#[test]
fn test_import_classification() {
    let resolver = ImportResolver::new().expect("Failed to create resolver");
    
    // Test local imports
    match resolver.classify_import("./helpers/string_utils").expect("Failed to classify local import") {
        ImportSource::Local(path) => {
            println!("✓ Correctly classified local import: {}", path.display());
        }
        _ => panic!("Expected local import classification"),
    }
    
    // Test package imports
    match resolver.classify_import("http_client").expect("Failed to classify package import") {
        ImportSource::Package(name, version) => {
            println!("✓ Correctly classified package import: {} (version: {:?})", name, version);
            assert_eq!(name, "http_client");
            assert_eq!(version, None);
        }
        _ => panic!("Expected package import classification"),
    }
    
    // Test versioned package imports
    match resolver.classify_import("json_parser@1.2.3").expect("Failed to classify versioned import") {
        ImportSource::Package(name, version) => {
            println!("✓ Correctly classified versioned package import: {} @ {}", name, version.as_ref().unwrap());
            assert_eq!(name, "json_parser");
            assert_eq!(version, Some("1.2.3".to_string()));
        }
        _ => panic!("Expected versioned package import classification"),
    }
    
    // Test stdlib imports
    match resolver.classify_import("std::collections").expect("Failed to classify stdlib import") {
        ImportSource::Stdlib(name) => {
            println!("✓ Correctly classified stdlib import: {}", name);
            assert_eq!(name, "std::collections");
        }
        _ => panic!("Expected stdlib import classification"),
    }
}

/// Test module validation utilities
#[test]
fn test_module_validation() {
    // Test validating existing module
    let math_utils_path = PathBuf::from("math_utils.csd");
    if math_utils_path.exists() {
        match validate_module_file(&math_utils_path) {
            Ok(true) => println!("✓ math_utils.csd is a valid CURSED module"),
            Ok(false) => println!("✗ math_utils.csd is not a valid CURSED module"),
            Err(e) => println!("✗ Error validating math_utils.csd: {}", e),
        }
    }
    
    // Test finding modules in current directory
    match find_modules_in_directory(&PathBuf::from(".")) {
        Ok(modules) => {
            println!("✓ Found {} CURSED modules in current directory", modules.len());
            for module in modules {
                println!("  - {}", module.display());
            }
        }
        Err(e) => println!("✗ Error finding modules: {}", e),
    }
}

/// Test module cache functionality
#[tokio::test]
async fn test_module_caching() -> Result<()> {
    let mut resolver = ImportResolver::new()?;
    
    let import = ImportStatement {
        path: "math_utils".to_string(),
        alias: None,
        items: vec![],
    };
    
    // First resolution - should load from disk
    println!("Resolving math_utils for the first time...");
    let start_time = std::time::Instant::now();
    let _resolved1 = resolver.resolve_single_import(&import).await?;
    let first_duration = start_time.elapsed();
    
    println!("✓ First resolution took: {:?}", first_duration);
    assert!(resolver.is_cached(&import.path));
    
    // Second resolution - should use cache
    println!("Resolving math_utils from cache...");
    let start_time = std::time::Instant::now();
    let _resolved2 = resolver.resolve_single_import(&import).await?;
    let second_duration = start_time.elapsed();
    
    println!("✓ Second resolution took: {:?}", second_duration);
    
    // Cache should be faster (though this might not always be true in tests)
    println!("Cache performance: {} -> {}", 
        format!("{:?}", first_duration), 
        format!("{:?}", second_duration)
    );
    
    // Test cache statistics
    let stats = resolver.get_stats();
    println!("✓ Cache stats: {} cached modules, {} cached resolutions", 
        stats.cached_modules, stats.cached_resolutions);
    
    Ok(())
}

/// Run all integration tests
#[tokio::test]
async fn run_all_integration_tests() -> Result<()> {
    println!("=== CURSED Import System Integration Tests ===\n");
    
    println!("1. Testing import classification...");
    test_import_classification();
    
    println!("\n2. Testing module validation...");
    test_module_validation();
    
    println!("\n3. Testing local import resolution...");
    test_resolve_local_imports().await?;
    
    println!("\n4. Testing relative import resolution...");
    test_resolve_relative_imports().await?;
    
    println!("\n5. Testing stdlib import resolution...");
    test_resolve_stdlib_imports().await?;
    
    println!("\n6. Testing full program import resolution...");
    test_full_program_import_resolution().await?;
    
    println!("\n7. Testing module caching...");
    test_module_caching().await?;
    
    println!("\n=== All Import System Tests Passed! ===");
    
    Ok(())
}
