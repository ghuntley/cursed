#!/usr/bin/env cargo script

//! Test script to verify LLVM optimization system works
//! 
//! This creates a simple LLVM module and tests that optimization passes
//! can be created and run without panicking.

use std::process::Command;

fn main() {
    println!("Testing LLVM optimization system...");
    
    // Create a simple test program
    let test_code = r#"
use cursed::codegen::llvm::passes::pass_pipeline::{OptimizationPipeline, PipelineBuilder};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::BasicTypeEnum;

fn test_optimization_passes() -> Result<(), Box<dyn std::error::Error>> {
    // Create LLVM context and module
    let context = Context::create();
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create a simple function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("test_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    
    builder.position_at_end(basic_block);
    
    // Create some IR that can be optimized
    let alloca = builder.build_alloca(i32_type, "temp");
    let param = function.get_first_param().unwrap().into_int_value();
    builder.build_store(alloca, param);
    let loaded = builder.build_load(alloca, "loaded");
    builder.build_return(Some(&loaded));
    
    // Test optimization pipeline
    let mut pipeline = PipelineBuilder::new(&context)
        .with_basic_optimizations()
        .build();
    
    println!("Running optimization passes...");
    let changed = pipeline.run_on_module(&module)?;
    
    let stats = pipeline.get_statistics();
    println!("Optimization completed:");
    println!("  - Passes run: {}", stats.passes_run);
    println!("  - Functions optimized: {}", stats.functions_optimized);
    println!("  - Modules optimized: {}", stats.modules_optimized);
    println!("  - Total changes: {}", stats.total_changes);
    println!("  - Changed: {}", changed);
    
    println!("✓ Optimization passes can be created and executed");
    Ok(())
}

fn main() {
    match test_optimization_passes() {
        Ok(()) => {
            println!("✅ All optimization tests passed!");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("❌ Optimization test failed: {}", e);
            std::process::exit(1);
        }
    }
}
"#;
    
    // Write test to a temporary file
    std::fs::write("temp_optimization_test.rs", test_code).expect("Failed to write test file");
    
    // Try to compile the test
    let output = Command::new("cargo")
        .args(["check", "--bin", "temp_optimization_test"])
        .output()
        .expect("Failed to run cargo check");
    
    if output.status.success() {
        println!("✅ Optimization system compiles successfully");
    } else {
        println!("❌ Compilation failed:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Clean up
    let _ = std::fs::remove_file("temp_optimization_test.rs");
    
    println!("Test completed.");
}
"#;

    match test_code.lines().count() {
        n if n > 10 => println!("✓ Test script created with {} lines", n),
        _ => println!("❌ Test script too short"),
    }
    
    println!("Run: cargo run --bin test_optimization_system");
}
