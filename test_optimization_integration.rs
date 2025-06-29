// Integration test for LLVM optimization passes
use cursed::optimization::real_llvm_passes::RealLlvmPassManager;
use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
use cursed::error::Result;
use inkwell::context::Context;

#[test]
fn test_constant_propagation_integration() -> Result<()> {
    let context = Context::create();
    let module = context.create_module("test_module");
    
    // Create a simple function for testing
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("test_add", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Create some constants that can be folded
    let const_1 = i32_type.const_int(5, false);
    let const_2 = i32_type.const_int(3, false);
    let add_result = builder.build_int_add(const_1, const_2, "add_result").unwrap();
    
    builder.build_return(Some(&add_result)).unwrap();
    
    // Create optimization configuration
    let config = OptimizationConfig::new(OptimizationLevel::Aggressive);
    let mut pass_manager = RealLlvmPassManager::new(&context, config)?;
    
    // Run optimization passes
    let result = pass_manager.optimize_module(&module)?;
    
    // Verify that optimizations were performed
    assert!(result.total_optimizations() > 0);
    println!("Constant propagation test passed: {} optimizations applied", result.total_optimizations());
    
    Ok(())
}

#[test]
fn test_dead_code_elimination_integration() -> Result<()> {
    let context = Context::create();
    let module = context.create_module("test_dce_module");
    
    // Create a function with dead code
    let i32_type = context.i32_type();
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_dce", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Create some dead code (unused computation)
    let const_1 = i32_type.const_int(10, false);
    let const_2 = i32_type.const_int(20, false);
    let _dead_add = builder.build_int_add(const_1, const_2, "dead_add").unwrap();
    
    // End with return (no use of dead_add)
    builder.build_return(None).unwrap();
    
    let config = OptimizationConfig::new(OptimizationLevel::Aggressive);
    let mut pass_manager = RealLlvmPassManager::new(&context, config)?;
    
    let result = pass_manager.optimize_module(&module)?;
    
    // DCE should have eliminated something
    if let Some(ref dce_result) = result.dead_code_result {
        println!("Dead code elimination test: {} instructions eliminated", dce_result.total_eliminated());
    }
    
    Ok(())
}

#[test]
fn test_gvn_integration() -> Result<()> {
    let context = Context::create();
    let module = context.create_module("test_gvn_module");
    
    // Create a function with redundant expressions
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("test_gvn", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let const_val = i32_type.const_int(42, false);
    
    // Create redundant expressions
    let add1 = builder.build_int_add(param, const_val, "add1").unwrap();
    let add2 = builder.build_int_add(param, const_val, "add2").unwrap(); // Same as add1
    let result = builder.build_int_add(add1, add2, "result").unwrap();
    
    builder.build_return(Some(&result)).unwrap();
    
    let config = OptimizationConfig::new(OptimizationLevel::Default);
    let mut pass_manager = RealLlvmPassManager::new(&context, config)?;
    
    let result = pass_manager.optimize_module(&module)?;
    
    // GVN should have found redundant expressions
    if let Some(ref gvn_result) = result.gvn_result {
        println!("GVN test: {} total optimizations", gvn_result.total_optimizations());
    }
    
    Ok(())
}

#[test]
fn test_comprehensive_optimization() -> Result<()> {
    let context = Context::create();
    let module = context.create_module("comprehensive_test");
    
    // Create a more complex function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("complex_function", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let then_block = context.append_basic_block(function, "then");
    let else_block = context.append_basic_block(function, "else");
    let merge_block = context.append_basic_block(function, "merge");
    
    let builder = context.create_builder();
    
    // Entry block
    builder.position_at_end(entry_block);
    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    
    // Constant operations that can be folded
    let const_5 = i32_type.const_int(5, false);
    let const_0 = i32_type.const_int(0, false);
    let const_calc = builder.build_int_add(const_5, const_0, "const_calc").unwrap(); // 5 + 0 = 5
    
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::SGT,
        param1,
        const_calc,
        "condition"
    ).unwrap();
    
    builder.build_conditional_branch(condition, then_block, else_block).unwrap();
    
    // Then block
    builder.position_at_end(then_block);
    let then_val = builder.build_int_add(param1, param2, "then_val").unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();
    
    // Else block
    builder.position_at_end(else_block);
    let else_val = builder.build_int_sub(param1, param2, "else_val").unwrap();
    // Dead code in else block
    let _dead_mul = builder.build_int_mul(param1, const_5, "dead_mul").unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();
    
    // Merge block
    builder.position_at_end(merge_block);
    let phi = builder.build_phi(i32_type, "result").unwrap();
    phi.add_incoming(&[(&then_val, then_block), (&else_val, else_block)]);
    
    builder.build_return(Some(&phi.as_basic_value())).unwrap();
    
    // Run comprehensive optimization
    let config = OptimizationConfig::release();
    let mut pass_manager = RealLlvmPassManager::new(&context, config)?;
    
    let result = pass_manager.optimize_module(&module)?;
    
    println!("Comprehensive optimization results:");
    println!("  Total optimizations: {}", result.total_optimizations());
    println!("  Execution time: {:?}", result.total_execution_time);
    println!("  Effectiveness score: {:.2}", result.effectiveness_score());
    
    if let Some(ref const_prop) = result.constant_propagation_result {
        println!("  Constant propagation: {}", const_prop);
    }
    
    if let Some(ref dce) = result.dead_code_result {
        println!("  Dead code eliminated: {}", dce.total_eliminated());
    }
    
    if let Some(ref gvn) = result.gvn_result {
        println!("  GVN optimizations: {}", gvn.total_optimizations());
    }
    
    if let Some(ref inlining) = result.inlining_result {
        println!("  Functions inlined: {}", inlining.functions_inlined);
    }
    
    assert!(result.total_optimizations() > 0, "No optimizations were applied");
    
    Ok(())
}

fn main() {
    println!("Running LLVM Optimization Passes Integration Tests");
    
    if let Err(e) = test_constant_propagation_integration() {
        eprintln!("Constant propagation test failed: {}", e);
    }
    
    if let Err(e) = test_dead_code_elimination_integration() {
        eprintln!("Dead code elimination test failed: {}", e);
    }
    
    if let Err(e) = test_gvn_integration() {
        eprintln!("GVN test failed: {}", e);
    }
    
    if let Err(e) = test_comprehensive_optimization() {
        eprintln!("Comprehensive optimization test failed: {}", e);
    }
    
    println!("All optimization integration tests completed!");
}
