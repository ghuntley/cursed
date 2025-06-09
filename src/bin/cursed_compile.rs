//! CURSED Separate Compilation Tool
//!
//! A simple command-line tool for testing separate compilation functionality.
//! This tool supports basic compilation of CURSED packages separately.

use cursed::codegen::llvm::{PackageCompilationConfig, compile_single_package};
use cursed::codegen::separate_compilation_integration::{
    analyze_package_structure,
};
use inkwell::OptimizationLevel;
use std::env;
use std::path::PathBuf;
use std::process;
use tracing::{error, info};

fn main() {
    // Initialize tracing
    cursed::init_tracing();

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let result = match args[1].as_str() {
        "package" => {
            if args.len() < 3 {
                eprintln!("Error: package command requires input file");
                print_usage();
                process::exit(1);
            }
            handle_package_compilation(&args[2])
        }
        "analyze" => {
            if args.len() < 3 {
                eprintln!("Error: analyze command requires input files");
                print_usage();
                process::exit(1);
            }
            handle_package_analysis(&args[2..])
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", args[1]);
            print_usage();
            process::exit(1);
        }
    };

    if let Err(e) = result {
        error!(error = ?e, "Compilation failed");
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    info!("Compilation completed successfully");
}

fn print_usage() {
    println!("CURSED Separate Compilation Tool v{}", cursed::VERSION);
    println!();
    println!("Usage:");
    println!("  cursed-compile package <input.csd>      Compile a single package");
    println!("  cursed-compile analyze <files...>       Analyze package structure");
    println!();
    println!("Examples:");
    println!("  cursed-compile package math_utils.csd");
    println!("  cursed-compile analyze main.csd math_utils.csd string_utils.csd");
}

fn handle_package_compilation(input_file: &str) -> Result<(), cursed::error::Error> {
    let input_path = PathBuf::from(input_file);
    let output_dir = PathBuf::from("./build");
    
    let config = PackageCompilationConfig {
        optimization_level: OptimizationLevel::Default,
        target_triple: None,
        output_dir,
        debug_info: true,
        emit_ir: true,
        emit_object: false, // Skip object files for testing
    };

    info!(input = ?input_path, config = ?config, "Compiling single package");

    let compiled_package = compile_single_package(&input_path, config)?;
    
    println!("✅ Package '{}' compiled successfully", compiled_package.metadata.name);
    
    if let Some(ir_file) = &compiled_package.ir_file {
        println!("📄 LLVM IR: {}", ir_file.display());
    }
    
    if let Some(obj_file) = &compiled_package.object_file {
        println!("🔧 Object file: {}", obj_file.display());
    }

    Ok(())
}

fn handle_package_analysis(input_files: &[String]) -> Result<(), cursed::error::Error> {
    let input_paths: Vec<PathBuf> = input_files
        .iter()
        .map(PathBuf::from)
        .collect();

    info!(inputs = ?input_paths, "Analyzing package structure");

    let packages = analyze_package_structure(&input_paths)?;
    
    println!("📦 Package Analysis Results");
    println!("===========================");
    
    for package in &packages {
        println!("\nPackage: {}", package.name);
        println!("  File: {}", package.path.display());
        println!("  Main package: {}", package.is_main);
        
        if !package.dependencies.is_empty() {
            println!("  Dependencies:");
            for dep in &package.dependencies {
                println!("    - {}", dep);
            }
        } else {
            println!("  Dependencies: none");
        }
    }

    // Show dependency graph
    println!("\n🔗 Dependency Graph");
    println!("===================");
    
    for package in &packages {
        if !package.dependencies.is_empty() {
            for dep in &package.dependencies {
                println!("{} -> {}", package.name, dep);
            }
        }
    }

    // Check for potential issues
    println!("\n⚠️  Analysis Warnings");
    println!("====================");
    
    let mut has_warnings = false;
    
    // Check for missing dependencies
    let package_names: std::collections::HashSet<String> = packages.iter().map(|p| p.name.clone()).collect();
    for package in &packages {
        for dep in &package.dependencies {
            if !package_names.contains(dep) && !is_standard_library_package(dep) {
                println!("⚠️  Package '{}' depends on '{}' which is not provided", package.name, dep);
                has_warnings = true;
            }
        }
    }

    // Check for multiple main packages
    let main_packages: Vec<&str> = packages.iter()
        .filter(|p| p.is_main)
        .map(|p| p.name.as_str())
        .collect();
    
    if main_packages.len() > 1 {
        println!("⚠️  Multiple main packages found: {}", main_packages.join(", "));
        has_warnings = true;
    } else if main_packages.is_empty() {
        println!("⚠️  No main package found - you may need a main() function");
        has_warnings = true;
    }

    if !has_warnings {
        println!("✅ No issues detected");
    }

    Ok(())
}

fn is_standard_library_package(package_name: &str) -> bool {
    matches!(
        package_name,
        "fmt" | "strings" | "os" | "io" | "math" | "time" | "net" | "sync"
    )
}
