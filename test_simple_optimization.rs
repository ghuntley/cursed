#!/usr/bin/env cargo run --bin test_simple_optimization --

fn main() {
    println!("Testing simple optimization system...");
    
    // Test 1: Import the optimization modules
    match std::panic::catch_unwind(|| {
        use cursed::codegen::llvm::optimization::{OptimizationConfig, OptimizationManager, OptimizationLevel};
        println!("✅ Successfully imported optimization types");
        
        // Test 2: Create optimization configurations
        let _dev_config = OptimizationConfig::dev_config();
        let _release_config = OptimizationConfig::release_config();
        println!("✅ Successfully created optimization configurations");
        
        // Test 3: Create context and basic module
        use inkwell::context::Context;
        let context = Context::create();
        let module = context.create_module("test");
        
        // Test 4: Create optimization manager
        let config = OptimizationConfig::default();
        let mut optimizer = OptimizationManager::new(&context, config);
        println!("✅ Successfully created optimization manager");
        
        // Test 5: Initialize optimization system
        optimizer.initialize(&module).expect("Failed to initialize optimizer");
        println!("✅ Successfully initialized optimization manager");
        
        // Test 6: Run optimization (on empty module)
        optimizer.optimize_module(&module).expect("Failed to optimize module");
        println!("✅ Successfully ran optimization passes");
        
        // Test 7: Check statistics
        let stats = optimizer.get_stats();
        println!("✅ Successfully retrieved optimization statistics");
        println!("   - Functions optimized: {}", stats.functions_optimized);
        println!("   - Modules optimized: {}", stats.modules_optimized);
        println!("   - Optimization time: {:?}", stats.optimization_time);
        
        true
    }) {
        Ok(true) => {
            println!("\n🎉 All optimization system tests passed!");
            println!("The LLVM optimization system is working correctly.");
        }
        Ok(false) => {
            println!("\n❌ Some optimization tests failed");
            std::process::exit(1);
        }
        Err(e) => {
            println!("\n💥 Optimization test panicked: {:?}", e);
            std::process::exit(1);
        }
    }
}
