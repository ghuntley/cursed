#!/usr/bin/env -S cargo +nightly -Zscript

//! Test the new LLVM JIT integration

use cursed::runtime::jit_runtime::{initialize_global_jit_runtime, compile_global_function, OptimizationLevel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing CURSED LLVM JIT Integration");
    
    // Initialize the global JIT runtime
    initialize_global_jit_runtime()?;
    println!("✓ JIT runtime initialized");
    
    // Test basic function compilation
    let function_id = compile_global_function(
        "test_function",
        "func test() { print(\"Hello from JIT!\"); }",
        Some(OptimizationLevel::Standard)
    )?;
    println!("✓ Function compiled with ID: {}", function_id);
    
    // Test hot path compilation
    let hot_function_id = compile_global_function(
        "hot_function", 
        "func hot_loop() { for i in 0..1000 { print(i); } }",
        Some(OptimizationLevel::Aggressive)
    )?;
    println!("✓ Hot function compiled with ID: {}", hot_function_id);
    
    // Test CURSED language constructs
    let goroutine_function_id = compile_global_function(
        "goroutine_function",
        "func spawn_workers() { go worker(); go worker(); }",
        Some(OptimizationLevel::Basic)
    )?;
    println!("✓ Goroutine function compiled with ID: {}", goroutine_function_id);
    
    let channel_function_id = compile_global_function(
        "channel_function", 
        "func use_channels() { ch := make(chan int); ch <- 42; x := <-ch; }",
        Some(OptimizationLevel::Standard)
    )?;
    println!("✓ Channel function compiled with ID: {}", channel_function_id);
    
    let async_function_id = compile_global_function(
        "async_function",
        "async func async_work() { await fetch_data(); await process_data(); }",
        Some(OptimizationLevel::Aggressive)
    )?;
    println!("✓ Async function compiled with ID: {}", async_function_id);
    
    println!("\n🎉 All JIT compilation tests passed!");
    println!("The CURSED JIT engine successfully compiled:");
    println!("  - Basic functions");
    println!("  - Hot path optimization"); 
    println!("  - Goroutine constructs");
    println!("  - Channel operations");
    println!("  - Async/await patterns");
    
    Ok(())
}
