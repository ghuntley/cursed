use cursed::codegen::llvm::optimization::{OptimizationConfig, OptimizationManager, OptimizationLevel};
use inkwell::context::Context;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing LLVM optimization system...");
    
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
    let _ = builder.build_store(alloca, param);
    let loaded = builder.build_load(alloca, "loaded");
    let _ = builder.build_return(Some(&loaded));
    
    println!("Module before optimization:");
    println!("{}", module.print_to_string().to_string());
    
    // Test optimization pipeline
    let config = OptimizationConfig::dev_config();
    let mut optimizer = OptimizationManager::new(&context, config);
    
    // Initialize and run optimization
    optimizer.initialize(&module)?;
    optimizer.optimize_module(&module)?;
    
    println!("\nModule after optimization:");
    println!("{}", module.print_to_string().to_string());
    
    // Check statistics
    let stats = optimizer.get_stats();
    println!("\nOptimization statistics:");
    println!("  - Functions optimized: {}", stats.functions_optimized);
    println!("  - Modules optimized: {}", stats.modules_optimized);
    println!("  - Optimization time: {:?}", stats.optimization_time);
    
    println!("\n✅ LLVM optimization system test completed successfully!");
    
    Ok(())
}
