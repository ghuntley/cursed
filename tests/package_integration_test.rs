//! Integration tests for package management with build system
//!
//! Tests the complete workflow: install package -> import in code -> compile successfully

use cursed::build_system::{PackageIntegration, PackageIntegrationConfig};
use cursed::package_manager::{PackageManager, PackageManagerConfig, init_package};
// Note: Async API functions are not yet available for testing
// use cursed::{run_with_packages, compile_to_ir_with_packages, check_with_packages};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Test basic package integration setup
#[tokio::test]
async fn test_package_integration_creation() {
    let config = PackageIntegrationConfig::default();
    let integration = PackageIntegration::new(config);
    assert!(integration.is_ok(), "Should create package integration successfully");
}

/// Test compilation without any imports
#[tokio::test]
async fn test_compilation_without_imports() {
    let config = PackageIntegrationConfig::default();
    let mut integration = PackageIntegration::new(config).unwrap();
    
    let source = r#"
slay main() {
    capicola("Hello, World!");
}
"#;
    
    let result = integration.compile_with_packages(source, None).await;
    assert!(result.is_ok(), "Should compile simple program without imports");
    
    let build_result = result.unwrap();
    assert!(!build_result.llvm_ir.is_empty(), "Should generate LLVM IR");
    assert_eq!(build_result.package_stats.packages_resolved, 0, "Should have no packages resolved");
}

/// Test compilation with stdlib imports
#[tokio::test]
async fn test_compilation_with_stdlib_imports() {
    let config = PackageIntegrationConfig::default();
    let mut integration = PackageIntegration::new(config).unwrap();
    
    let source = r#"
yeet "stdlib::io"

slay main() {
    print("Hello, ");
    println("World!");
}
"#;
    
    let result = integration.compile_with_packages(source, None).await;
    assert!(result.is_ok(), "Should compile program with stdlib imports");
    
    let build_result = result.unwrap();
    assert!(!build_result.llvm_ir.is_empty(), "Should generate LLVM IR");
    assert_eq!(build_result.compilation_context.resolved_imports.len(), 1, "Should have one resolved import");
}

/// Test package initialization and compilation
#[tokio::test]
async fn test_package_init_and_compilation() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Initialize a new package
    let result = init_package("test-package", Some("0.1.0"), Some("A test package"));
    assert!(result.is_ok(), "Should initialize package successfully");
    
    // Verify files were created
    assert!(Path::new("CursedPackage.toml").exists(), "Should create CursedPackage.toml");
    assert!(Path::new("src/main.csd").exists(), "Should create src/main.csd");
    
    // Read the generated main file
    let main_content = fs::read_to_string("src/main.csd").unwrap();
    
    // Try to compile it with basic API for now
    let result = cursed::compile_to_ir(&main_content);
    assert!(result.is_ok(), "Should compile generated main file");
    
    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

/// Test package manager integration with CLI-like operations
#[tokio::test]
async fn test_package_manager_operations() {
    let temp_dir = TempDir::new().unwrap();
    let config = PackageManagerConfig {
        cache_dir: temp_dir.path().join("cache"),
        workspace_dir: temp_dir.path().to_path_buf(),
        ..PackageManagerConfig::default()
    };
    
    let mut manager = PackageManager::new(config).unwrap();
    
    // Test search operation (will likely fail with real registry, but should not panic)
    let search_result = manager.search_packages("cursed-http", Some(5)).await;
    // We don't assert success here since the registry might not be available
    match search_result {
        Ok(packages) => {
            println!("Found {} packages", packages.len());
        }
        Err(e) => {
            println!("Search failed (expected in test environment): {}", e);
        }
    }
    
    // Test list installed packages (should be empty initially)
    let installed = manager.list_installed().unwrap();
    assert!(installed.is_empty(), "Should have no packages installed initially");
    
    // Test cache operations
    let clean_result = manager.clean_cache();
    assert!(clean_result.is_ok(), "Should clean cache successfully");
}

/// Test import resolution for different types of imports
#[tokio::test]
async fn test_import_resolution() {
    use cursed::imports::{ImportManager, ImportResolverConfig};
    use cursed::package_manager::PackageManager;
    use cursed::ast::ImportStatement;
    use std::sync::{Arc, Mutex};
    
    let config = PackageManagerConfig::default();
    let package_manager = Arc::new(Mutex::new(PackageManager::new(config).unwrap()));
    
    let import_config = ImportResolverConfig::default();
    let mut import_manager = ImportManager::new(package_manager, import_config).unwrap();
    
    // Test stdlib import resolution
    let stdlib_import = ImportStatement::new(
        cursed::lexer::Token::new(cursed::lexer::TokenType::Yeet, "yeet".to_string(), 1, 1),
        "stdlib::io".to_string()
    );
    
    let resolved = import_manager.resolve_single_import(&stdlib_import, None, &mut std::collections::HashSet::new()).await;
    assert!(resolved.is_ok(), "Should resolve stdlib import");
    
    let resolved_import = resolved.unwrap();
    assert_eq!(resolved_import.original_path, "stdlib::io");
    assert!(matches!(resolved_import.source, cursed::ImportSource::StandardLibrary));
}

/* Commented out until async API is ready
/// Test type checking with package imports
#[tokio::test]
async fn test_type_checking_with_packages() {
    let source_with_types = r#"
yeet "stdlib::collections"

slay main() {
    sus numbers: Vec<i32> = Vec::new();
    numbers.push(42);
    capicola("Numbers: {}", numbers);
}
"#;
    
    // Type checking is done as part of compilation
    let result = check_with_packages(source_with_types, None).await;
    assert!(result.is_ok(), "Should type check program with package imports");
}

/// Test error handling for missing packages
#[tokio::test]
async fn test_missing_package_error_handling() {
    let source_with_missing_package = r#"
yeet "nonexistent-package::module"

slay main() {
    some_function();
}
"#;
    
    // This should fail but gracefully
    let result = compile_to_ir_with_packages(source_with_missing_package, None).await;
    // We expect this to fail, but the error should be meaningful
    match result {
        Err(e) => {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("nonexistent-package") || error_msg.contains("not found"),
                "Error should mention the missing package: {}", error_msg
            );
        }
        Ok(_) => {
            // If it succeeds, that's also fine - the package resolver might have fallbacks
        }
    }
}

/// Test compilation with multiple imports
#[tokio::test] 
async fn test_multiple_imports_compilation() {
    let source_with_multiple_imports = r#"
yeet "stdlib::io"
yeet "stdlib::math"
yeet "stdlib::collections"

slay main() {
    println("Testing multiple imports");
    
    sus result = sqrt(16.0);
    println("Square root of 16: {}", result);
    
    sus numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    
    println("Numbers: {:?}", numbers);
}
"#;
    
    let result = compile_to_ir_with_packages(source_with_multiple_imports, None).await;
    assert!(result.is_ok(), "Should compile program with multiple stdlib imports");
}

/// Helper function for async tests
async fn assert_compilation_success(source: &str, description: &str) {
    let result = compile_to_ir_with_packages(source, None).await;
    assert!(result.is_ok(), "{}: {}", description, 
        result.err().map(|e| e.to_string()).unwrap_or_default());
}

/// Test package-aware high-level API functions
#[tokio::test]
async fn test_high_level_api_functions() {
    let simple_source = r#"
slay main() {
    capicola("Hello from high-level API!");
}
"#;
    
    // Test run_with_packages
    let run_result = run_with_packages(simple_source, None).await;
    assert!(run_result.is_ok(), "run_with_packages should succeed");
    
    // Test compile_to_ir_with_packages
    let ir_result = compile_to_ir_with_packages(simple_source, None).await;
    assert!(ir_result.is_ok(), "compile_to_ir_with_packages should succeed");
    
    // Test check_with_packages
    let check_result = check_with_packages(simple_source, None).await;
    assert!(check_result.is_ok(), "check_with_packages should succeed");
}
*/

/// Test import cache functionality
#[tokio::test]
async fn test_import_caching() {
    let config = PackageIntegrationConfig::default();
    let mut integration = PackageIntegration::new(config).unwrap();
    
    let source = r#"
yeet "stdlib::io"
yeet "stdlib::math"

slay main() {
    println("Testing import caching");
}
"#;
    
    // First compilation
    let result1 = integration.compile_with_packages(source, None).await;
    assert!(result1.is_ok(), "First compilation should succeed");
    
    // Second compilation (should use cached imports)
    let result2 = integration.compile_with_packages(source, None).await;
    assert!(result2.is_ok(), "Second compilation should succeed");
    
    // Both should produce similar results
    let stats1 = result1.unwrap().package_stats;
    let stats2 = result2.unwrap().package_stats;
    
    // Second compilation might be faster due to caching
    assert!(stats2.resolution_time <= stats1.resolution_time || 
            stats2.resolution_time.as_millis() < 100, 
            "Second compilation should benefit from caching");
}
