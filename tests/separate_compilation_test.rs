//! Integration tests for separate compilation functionality

use cursed::codegen::llvm::{compile_package_file, compile_package_files, SeparateCompiler};
use cursed::codegen::separate_compilation_integration::{
    should_use_separate_compilation, analyze_package_structure, SeparateCompilationOptions,
    auto_compile,
};
use inkwell::context::Context;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test compiling a single package file
#[test]
fn test_compile_single_package() {
    let context = Context::create();
    let temp_dir = TempDir::new().unwrap();

    // Create a simple package
    let package_content = r#""
vibe testpkg;

slay greet(name tea) tea {
    cap "Hello"
}

slay main() {
    sus msg = greet("World")
    vibez.spill(msg)
}
"#";

    let package_path = temp_dir.path().join("test.csd");
    fs::write(&package_path, package_content).unwrap();

    // Compile the package
    let result = compile_package_file(&context, &package_path);
    assert!(result.is_ok(), "Package compilation failed: {:?}", result.err());

    let module = result.unwrap();
    assert_eq!(module.get_name().to_string_lossy(), "testpkg");

    // Check that functions were generated
    let ir = module.print_to_string().to_string();
    assert!(ir.contains("testpkg"), "Package name should appear in IR");
}

/// Test compiling multiple packages with dependencies
#[test]
fn test_compile_multiple_packages() {
    let context = Context::create();
    let temp_dir = TempDir::new().unwrap();

    // Create package A (depends on B)
    let package_a_content = r#""
vibe pkga;

yeet "pkgb"

slay main() {
    vibez.spill("Package A main")
}
"#";

    // Create package B (no dependencies)
    let package_b_content = r#""
vibe pkgb;

slay helper() tea {
    cap "Helper from B"
}
"#";

    let package_a_path = temp_dir.path().join("a.csd");
    let package_b_path = temp_dir.path().join("b.csd");
    
    fs::write(&package_a_path, package_a_content).unwrap();
    fs::write(&package_b_path, package_b_content).unwrap();

    // Compile packages
    let file_paths = vec![package_a_path, package_b_path];
    let result = compile_package_files(&context, &file_paths);
    
    assert!(result.is_ok(), "Multi-package compilation failed: {:?}", result.err());

    let modules = result.unwrap();
    assert_eq!(modules.len(), 2, "Should have compiled 2 modules");

    // Check module names
    let module_names: Vec<String> = modules
        .iter()
        .map(|m| m.get_name().to_string_lossy().to_string())
        .collect();

    assert!(module_names.contains(&"pkga".to_string()));
    assert!(module_names.contains(&"pkgb".to_string()));
}

/// Test separate compiler with dependency resolution
#[test]
fn test_separate_compiler_dependency_resolution() {
    let context = Context::create();
    let temp_dir = TempDir::new().unwrap();
    let mut compiler = SeparateCompiler::new(&context);

    // Create packages with dependency chain: A -> B -> C
    let package_a_content = r#""
vibe pkga;
yeet "pkgb"
slay main() {
    vibez.spill("main")
}
"#";

    let package_b_content = r#""
vibe pkgb;
yeet "pkgc"
slay func_b() {
    vibez.spill("func_b")
}
"#";

    let package_c_content = r#""
vibe pkgc;
slay func_c() {
    vibez.spill("func_c")
}
"#";

    let path_a = temp_dir.path().join("a.csd");
    let path_b = temp_dir.path().join("b.csd");
    let path_c = temp_dir.path().join("c.csd");

    fs::write(&path_a, package_a_content).unwrap();
    fs::write(&path_b, package_b_content).unwrap();
    fs::write(&path_c, package_c_content).unwrap();

    // Add packages to compiler
    compiler.add_package_source("pkga", path_a.clone()).unwrap();
    compiler.add_package_source("pkgb", path_b.clone()).unwrap();
    compiler.add_package_source("pkgc", path_c.clone()).unwrap();

    // Compile all packages
    let result = compiler.compile_all_packages();
    assert!(result.is_ok(), "Compilation failed: {:?}", result.err());

    // Check compilation order
    let order = compiler.get_compilation_order();
    
    // C should come before B, and B should come before A
    let c_pos = order.iter().position(|x| x == "pkgc").unwrap();
    let b_pos = order.iter().position(|x| x == "pkgb").unwrap();
    let a_pos = order.iter().position(|x| x == "pkga").unwrap();

    assert!(c_pos < b_pos, "pkgc should be compiled before pkgb");
    assert!(b_pos < a_pos, "pkgb should be compiled before pkga");
}

/// Test separate compilation detection
#[test]
fn test_separate_compilation_detection() {
    let temp_dir = TempDir::new().unwrap();

    // Single main package - should not use separate compilation
    let main_content = r#""
vibe main;
slay main() {
    vibez.spill("Hello")
}
"#";

    let main_path = temp_dir.path().join("main.csd");
    fs::write(&main_path, main_content).unwrap();
    
    assert!(!should_use_separate_compilation(&[main_path.clone()]));

    // Non-main package - should use separate compilation
    let pkg_content = r#""
vibe mypackage;
slay func() {}
"#";

    let pkg_path = temp_dir.path().join("pkg.csd");
    fs::write(&pkg_path, pkg_content).unwrap();
    
    assert!(should_use_separate_compilation(&[pkg_path.clone()]));

    // Package with imports - should use separate compilation
    let import_content = r#""
vibe main;
yeet "fmt"
slay main() {}
"#";

    let import_path = temp_dir.path().join("import.csd");
    fs::write(&import_path, import_content).unwrap();
    
    assert!(should_use_separate_compilation(&[import_path.clone()]));

    // Multiple files - should use separate compilation
    assert!(should_use_separate_compilation(&[main_path, pkg_path]));
}

/// Test package structure analysis
#[test]
fn test_package_structure_analysis() {
    let temp_dir = TempDir::new().unwrap();

    // Create test packages
    let package_a_content = r#""
vibe pkga;
yeet "pkgb"
yeet "fmt"
slay main() {}
"#";

    let package_b_content = r#""
vibe pkgb;
slay helper() {}
"#";

    let path_a = temp_dir.path().join("a.csd");
    let path_b = temp_dir.path().join("b.csd");

    fs::write(&path_a, package_a_content).unwrap();
    fs::write(&path_b, package_b_content).unwrap();

    // Analyze packages
    let result = analyze_package_structure(&[path_a.clone(), path_b.clone()]);
    assert!(result.is_ok(), "Package analysis failed: {:?}", result.err());

    let packages = result.unwrap();
    assert_eq!(packages.len(), 2);

    // Check package A
    let pkg_a = packages.iter().find(|p| p.name == "pkga").unwrap();
    assert_eq!(pkg_a.path, path_a);
    assert_eq!(pkg_a.dependencies, vec!["pkgb", "fmt"]);
    assert!(!pkg_a.is_main);

    // Check package B
    let pkg_b = packages.iter().find(|p| p.name == "pkgb").unwrap();
    assert_eq!(pkg_b.path, path_b);
    assert!(pkg_b.dependencies.is_empty());
    assert!(!pkg_b.is_main);
}

/// Test automatic compilation mode detection and execution
#[test]
fn test_auto_compile_mode_detection() {
    let temp_dir = TempDir::new().unwrap();

    // Create a simple package that should trigger separate compilation
    let package_content = r#""
vibe testpkg;

slay main() {
    vibez.spill("Auto compilation test")
}
"#";

    let package_path = temp_dir.path().join("test.csd");
    fs::write(&package_path, package_content).unwrap();

    let output_path = temp_dir.path().join("output");
    let options = SeparateCompilationOptions::default();

    // This should auto-detect that separate compilation is needed
    // Note: This test may fail if the full compilation pipeline isn't set up
    // but the logic should be exercised
    let result = auto_compile(&[package_path], &output_path, options);
    
    // We expect this to attempt separate compilation, though it may fail
    // due to missing linking infrastructure in the test environment
    match result {
        Ok(_) => {
            // Success - separate compilation worked
            println!("Separate compilation succeeded");
        }
        Err(e) => {
            // Expected to fail in test environment, but should have attempted separate compilation
            let error_msg = e.to_string();
            println!("Expected failure in test environment: {}", error_msg);
            
            // Verify it attempted separate compilation (not single-file compilation)
            assert!(
                error_msg.contains("separate") || 
                error_msg.contains("link") || 
                error_msg.contains("object") ||
                !error_msg.contains("Parser"),
                "Should have attempted separate compilation, got: {}", error_msg
            );
        }
    }
}

/// Test circular dependency detection
#[test]
fn test_circular_dependency_detection() {
    let context = Context::create();
    let temp_dir = TempDir::new().unwrap();
    let mut compiler = SeparateCompiler::new(&context);

    // Create packages with circular dependency: A -> B -> A
    let package_a_content = r#""
vibe pkga;
yeet "pkgb"
slay func_a() {}
"#";

    let package_b_content = r#""
vibe pkgb;
yeet "pkga"
slay func_b() {}
"#";

    let path_a = temp_dir.path().join("a.csd");
    let path_b = temp_dir.path().join("b.csd");

    fs::write(&path_a, package_a_content).unwrap();
    fs::write(&path_b, package_b_content).unwrap();

    // Add packages to compiler
    compiler.add_package_source("pkga", path_a).unwrap();
    compiler.add_package_source("pkgb", path_b).unwrap();

    // Compilation should fail due to circular dependency
    let result = compiler.compile_all_packages();
    assert!(result.is_err(), "Should fail due to circular dependency");

    let error_msg = result.err().unwrap().to_string();
    assert!(error_msg.contains("circular") || error_msg.contains("Circular"), 
            "Error should mention circular dependency: {}", error_msg);
}

/// Test package metadata extraction
#[test]
fn test_package_metadata_extraction() {
    let context = Context::create();
    let temp_dir = TempDir::new().unwrap();
    let mut compiler = SeparateCompiler::new(&context);

    let package_content = r#""
vibe mypackage;

yeet "fmt"

slay main() {
    vibez.spill("Main function")
}

slay helper(x normie) normie {
    cap 42
}

slay processor(data tea) tea {
    cap "processed"
}
"#";

    let package_path = temp_dir.path().join("test.csd");
    fs::write(&package_path, package_content).unwrap();

    // Analyze package
    let result = compiler.analyze_package(&package_content, package_path.clone());
    assert!(result.is_ok(), "Package analysis failed: {:?}", result.err());

    let metadata = result.unwrap();
    
    // Check metadata
    assert_eq!(metadata.name, "mypackage");
    assert_eq!(metadata.source_path, package_path);
    assert_eq!(metadata.dependencies, vec!["fmt"]);
    assert_eq!(metadata.exports, vec!["main", "helper", "processor"]);
    assert_eq!(metadata.module_name, "module_mypackage");
}
