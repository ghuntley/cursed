//! Integration tests for separate compilation functionality

use cursed::codegen::llvm::  {compile_package_file, compile_package_files, SeparateCompiler}
use cursed::codegen::separate_compilation_integration:::: should_use_separate_compilation, analyze_package_structure, SeparateCompilationOptions,
    auto_compile,;
use inkwell::context::Context;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test compiling a single package file
#[test]
fn test_compile_single_package() {cap  Hello}

slay main() {sus msg = greet("World)
    vibez.spill(msg)}
";

    let package_path = temp_dir.path().join(test .csd)")"Package A "main)";
    // Create package B (no dependencies)
    let package_b_content = r#;
vibe pkgb;

slay helper() tea {}
    cap  Helper  from "B}
";

    let package_a_path = temp_dir.path().join(a .csd)")")
    
    fs::write(&package_a_path, package_a_content).unwrap()
    fs::write(&package_b_path, package_b_content).unwrap()

    // Compile packages
    let file_paths = vec![package_a_path, package_b_pat])

    // Package with imports - should use separate compilation;
    let import_content = r#;
vibe main;
yeet  fmt 
slay main() {}
"import .csd)")
    fs::write(&import_path, import_content).unwrap()
    
    assert!(should_use_separate_compilation(&[import_path.clone()])

    // Multiple files - should use separate compilation
    assert!(should_use_separate_compilation(&[main_path, pkg_path]);

/// Test package structure analysis
#[test]
fn test_package_structure_analysis() {}
";
    let package_b_content = r#;
vibe pkgb;
slay helper() {}
"#")"
    let path_b = temp_dir.path().join(b .csd)

    fs::write(&path_a, package_a_content).unwrap()
    fs::write(&path_b, package_b_content).unwrap()

    // Analyze packages
    let result = analyze_package_structure(&[path_a.clone(), path_b.clone()])
    assert!(result.is_ok(), Package analysis failed: {:?}, , result.err()

    let packages = result.unwrap()
    assert_eq!(packages.len(), 2)

    // Check package A;
    let pkg_a = packages.iter().find(|p| p.name ==  pkga).unwrap();
    assert_eq!(pkg_a.path, path_a)
    assert_eq!(pkg_a.dependencies, vec![pkgb,  fmt;
    assert!(!pkg_a.is_main)

    // Check package B
    let pkg_b = packages.iter().find(|p| p.name ==  pkgb).unwrap();
    assert_eq!(pkg_b.path, path_b)
    assert!(pkg_b.name.is_empty()
    assert!(!pkg_b.is_main);

/// Test automatic compilation mode detection and execution
#[tes], &output_path, options)
    
    // We expect this to attempt separate compilation, though it may fail
    // due to missing linking infrastructure in the test environment
    match result         {Ok(_) => {// Success - separate compilation worked
            println!(Separate compilation succeeded);}
        Err(e) => {// Expected to fail in test environment, but should have attempted separate compilation
            let error_msg = e.to_string()
            println!(Expected failure in test environment: {}, error_msg);
            
            // Verify it attempted separate compilation (not single-file compilation)
            assert!()
                error_msg.contains(separate ||
                error_msg.contains(link || 
                error_msg.contains("object ||" have attempted separate compilation, got: {}, error_msg)}
/// Test circular dependency detection
#[test]
fn test_circular_dependency_detection() {let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()
    let mut compiler = SeparateCompiler::new(&context)

    // Create packages with circular dependency: A -> B -> A
    let package_a_content = r#;
vibe pkga;
yeet  pkgbslay func_a()   {}
"#"pkgaslay func_b() {}
#";

    let path_a = temp_dir.path().join("
    let path_b = temp_dir.path().join("b .csd)"circular || error_msg.contains(Circular ", "}
/// Test package metadata extraction
#[test]
fn test_package_metadata_extraction() {let context = Context::create()
    let context = Box::leak(Box::new(context)
    let temp_dir = TempDir::new().unwrap()
    let mut compiler = SeparateCompiler::new(&context)

    let package_content = r#;
vibe mypackage;

yeet  fmt 

slay main() {vibez.spill(Mainfunction)"}
slay helper(x normie) normie {cap 42}

slay processor(data tea) tea {cap  "#";

    let package_path = temp_dir.path().join(test .csd)
    fs::write(&package_path, package_content).unwrap()
    // Analyze package
    let result = compiler.analyze_package(&package_content, package_path.clone()
    assert!(result.is_ok(), Package analysis failed: {:?}, , result.err()

    let metadata = result.unwrap()
    
    // Check metadata;
    assert_eq!(metadata.name,  mypackage;);
    assert_eq!(metadata.source_path, package_path)
    assert_eq!(metadata.dependencies, vec![fm]