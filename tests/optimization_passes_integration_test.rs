/// Integration tests for CURSED LLVM optimization passes
/// 
/// This test suite validates the critical optimization passes that form
/// the core of the CURSED compiler's optimization infrastructure.

use cursed::codegen::llvm::passes::*;
use cursed::error::Result;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::AddressSpace;
use std::time::Duration;
use tracing::{info, debug};

/// Initialize tracing for tests
fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .with_test_writer()
        .try_init();
}

/// Create a simple test function for optimization
fn create_test_function<'ctx>(
    context: &'ctx Context,
    module: &Module<'ctx>,
    name: &str,
) -> FunctionValue<'ctx> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function(name, fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    // Create a simple function: add parameters and return result
    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    let sum = builder.build_int_add(param1, param2, "sum").unwrap();
    builder.build_return(Some(&sum)).unwrap();
    
    function
}

/// Create a function with memory operations for testing SROA and Mem2Reg
fn create_memory_test_function<'ctx>(
    context: &'ctx Context,
    module: &Module<'ctx>,
    name: &str,
) -> FunctionValue<'ctx> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function(name, fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    // Create alloca, store, load pattern
    let alloca = builder.build_alloca(i32_type, "local_var").unwrap();
    let param = function.get_nth_param(0).unwrap().into_int_value();
    builder.build_store(alloca, param).unwrap();
    let loaded = builder.build_load(i32_type, alloca, "loaded").unwrap().into_int_value();
    
    // Do some computation with the loaded value
    let incremented = builder.build_int_add(
        loaded,
        i32_type.const_int(1, false),
        "incremented"
    ).unwrap();
    
    builder.build_return(Some(&incremented)).unwrap();
    
    function
}

/// Create a function with conditional branches for jump threading
fn create_conditional_test_function<'ctx>(
    context: &'ctx Context,
    module: &Module<'ctx>,
    name: &str,
) -> FunctionValue<'ctx> {
    let i32_type = context.i32_type();
    let bool_type = context.bool_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function(name, fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let then_block = context.append_basic_block(function, "then");
    let else_block = context.append_basic_block(function, "else");
    let merge_block = context.append_basic_block(function, "merge");
    
    let builder = context.create_builder();
    
    // Entry block: create condition
    builder.position_at_end(entry_block);
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let zero = i32_type.const_int(0, false);
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::SGT,
        param,
        zero,
        "condition"
    ).unwrap();
    builder.build_conditional_branch(condition, then_block, else_block).unwrap();
    
    // Then block
    builder.position_at_end(then_block);
    let then_value = builder.build_int_add(
        param,
        i32_type.const_int(10, false),
        "then_value"
    ).unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();
    
    // Else block
    builder.position_at_end(else_block);
    let else_value = builder.build_int_sub(
        param,
        i32_type.const_int(5, false),
        "else_value"
    ).unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();
    
    // Merge block with PHI
    builder.position_at_end(merge_block);
    let phi = builder.build_phi(i32_type, "result").unwrap();
    phi.add_incoming(&[(&then_value, then_block), (&else_value, else_block)]);
    builder.build_return(Some(&phi.as_basic_value())).unwrap();
    
    function
}

/// Create a recursive function for tail call optimization
fn create_recursive_test_function<'ctx>(
    context: &'ctx Context,
    module: &Module<'ctx>,
    name: &str,
) -> FunctionValue<'ctx> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function(name, fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let recursive_block = context.append_basic_block(function, "recursive");
    let base_block = context.append_basic_block(function, "base");
    
    let builder = context.create_builder();
    
    // Entry block: check base case
    builder.position_at_end(entry_block);
    let n = function.get_nth_param(0).unwrap().into_int_value();
    let acc = function.get_nth_param(1).unwrap().into_int_value();
    let zero = i32_type.const_int(0, false);
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::EQ,
        n,
        zero,
        "is_zero"
    ).unwrap();
    builder.build_conditional_branch(condition, base_block, recursive_block).unwrap();
    
    // Base case
    builder.position_at_end(base_block);
    builder.build_return(Some(&acc)).unwrap();
    
    // Recursive case (tail call)
    builder.position_at_end(recursive_block);
    let one = i32_type.const_int(1, false);
    let n_minus_1 = builder.build_int_sub(n, one, "n_minus_1").unwrap();
    let new_acc = builder.build_int_add(acc, n, "new_acc").unwrap();
    
    let recursive_call = builder.build_call(
        function,
        &[n_minus_1.into(), new_acc.into()],
        "recursive_call"
    ).unwrap();
    
    let result = recursive_call.try_as_basic_value().left().unwrap().into_int_value();
    builder.build_return(Some(&result)).unwrap();
    
    function
}

#[test]
fn test_mem2reg_pass() -> Result<()> {
    init_test_tracing();
    info!("Testing Mem2Reg pass");
    
    let context = Context::create();
    let module = context.create_module("test_mem2reg");
    let function = create_memory_test_function(&context, &module, "test_function");
    
    let mut pass = Mem2RegPass::new();
    let config = PassConfiguration::default();
    
    assert!(pass.should_run(&config));
    assert_eq!(pass.name(), "mem2reg");
    
    let result = pass.run_on_function(&function, &context)?;
    debug!("Mem2Reg result: {:?}", result);
    
    // In a real scenario, we'd verify the IR was actually transformed
    Ok(())
}

#[test]
fn test_sroa_pass() -> Result<()> {
    init_test_tracing();
    info!("Testing SROA pass");
    
    let context = Context::create();
    let module = context.create_module("test_sroa");
    let function = create_memory_test_function(&context, &module, "test_function");
    
    let mut pass = SroaPass::new();
    let config = PassConfiguration::default();
    
    assert!(pass.should_run(&config));
    assert_eq!(pass.name(), "sroa");
    
    let result = pass.run_on_function(&function, &context)?;
    debug!("SROA result: {:?}", result);
    
    Ok(())
}

#[test]
fn test_sccp_pass() -> Result<()> {
    init_test_tracing();
    info!("Testing SCCP pass");
    
    let context = Context::create();
    let module = context.create_module("test_sccp");
    let function = create_test_function(&context, &module, "test_function");
    
    let mut pass = SccpPass::new();
    let config = PassConfiguration::default();
    
    assert!(pass.should_run(&config));
    assert_eq!(pass.name(), "sccp");
    
    let result = pass.run_on_function(&function, &context)?;
    debug!("SCCP result: {:?}", result);
    
    Ok(())
}

#[test]
fn test_gvn_pass() -> Result<()> {
    init_test_tracing();
    info!("Testing GVN pass");
    
    let context = Context::create();
    let module = context.create_module("test_gvn");
    let function = create_test_function(&context, &module, "test_function");
    
    let mut pass = GvnPass::new();
    let config = PassConfiguration::default();
    
    assert!(pass.should_run(&config));
    assert_eq!(pass.name(), "gvn");
    
    let result = pass.run_on_function(&function, &context)?;
    debug!("GVN result: {:?}", result);
    
    Ok(())
}

#[test]
fn test_licm_pass() -> Result<()> {
    init_test_tracing();
    info!("Testing LICM pass");
    
    let context = Context::create();
    let module = context.create_module("test_licm");
    let function = create_test_function(&context, &module, "test_function");
    
    let mut pass = LicmPass::new();
    let config = PassConfiguration::default();
    
    assert!(pass.should_run(&config));
    assert_eq!(pass.name(), "licm");
    
    let result = pass.run_on_function(&function, &context)?;
    debug!("LICM result: {:?}", result);
    
    Ok(())
}

#[test]
fn test_tail_call_pass() -> Result<()> {
    init_test_tracing();
    info!("Testing Tail Call Optimization pass");
    
    let context = Context::create();
    let module = context.create_module("test_tail_call");
    let function = create_recursive_test_function(&context, &module, "factorial");
    
    let mut pass = TailCallPass::new();
    let config = PassConfiguration::default();
    
    assert!(pass.should_run(&config));
    assert_eq!(pass.name(), "tail_call");
    
    let result = pass.run_on_function(&function, &context)?;
    debug!("Tail Call result: {:?}", result);
    
    Ok(())
}

#[test]
fn test_jump_threading_pass() -> Result<()> {
    init_test_tracing();
    info!("Testing Jump Threading pass");
    
    let context = Context::create();
    let module = context.create_module("test_jump_threading");
    let function = create_conditional_test_function(&context, &module, "test_function");
    
    let mut pass = JumpThreadingPass::new();
    let config = PassConfiguration::default();
    
    assert!(pass.should_run(&config));
    assert_eq!(pass.name(), "jump_threading");
    
    let result = pass.run_on_function(&function, &context)?;
    debug!("Jump Threading result: {:?}", result);
    
    Ok(())
}

#[test]
fn test_optimization_pipeline_with_new_passes() -> Result<()> {
    init_test_tracing();
    info!("Testing optimization pipeline with new passes");
    
    let context = Context::create();
    let module = context.create_module("test_pipeline");
    let function = create_memory_test_function(&context, &module, "test_function");
    
    let config = PassConfiguration::default();
    let registry = std::sync::Arc::new(std::sync::Mutex::new(PassRegistry::new(config.clone())));
    
    // Register the new passes
    {
        let mut reg = registry.lock().unwrap();
        reg.register_pass(Mem2RegPass::new())?;
        reg.register_pass(SroaPass::new())?;
        reg.register_pass(SccpPass::new())?;
        reg.register_pass(GvnPass::new())?;
        reg.register_pass(LicmPass::new())?;
        reg.register_pass(TailCallPass::new())?;
        reg.register_pass(JumpThreadingPass::new())?;
    }
    
    // Build a pipeline with the new passes
    let mut pipeline = PipelineBuilder::new(registry, config)
        .add_pass_stage("memory_promotion", vec![
            "mem2reg".to_string(),
            "sroa".to_string(),
        ])
        .add_pass_stage("advanced_optimization", vec![
            "sccp".to_string(),
            "gvn".to_string(),
            "licm".to_string(),
        ])
        .add_pass_stage("function_optimization", vec![
            "tail_call".to_string(),
            "jump_threading".to_string(),
        ])
        .build();
    
    let result = pipeline.execute(&module, &context)?;
    info!("Pipeline execution completed: {:?}", result.success);
    info!("Stages executed: {}", result.stages_executed);
    info!("Total passes run: {}", result.total_passes_run);
    
    assert!(result.success);
    
    Ok(())
}

#[test]
fn test_pass_configuration_compatibility() {
    init_test_tracing();
    info!("Testing pass configuration compatibility");
    
    let configs = vec![
        OptimizationLevel::None.default_config(),
        OptimizationLevel::Basic.default_config(),
        OptimizationLevel::Default.default_config(),
        OptimizationLevel::Aggressive.default_config(),
        OptimizationLevel::Size.default_config(),
        OptimizationLevel::MinSize.default_config(),
    ];
    
    for config in configs {
        let mem2reg = Mem2RegPass::new();
        let sroa = SroaPass::new();
        let sccp = SccpPass::new();
        let gvn = GvnPass::new();
        let licm = LicmPass::new();
        let tail_call = TailCallPass::new();
        let jump_threading = JumpThreadingPass::new();
        
        debug!("Testing config for optimization level: {:?}", config.optimization_level);
        
        // Test that passes respect optimization level requirements
        if config.optimization_level >= OptimizationLevel::Basic {
            assert!(mem2reg.should_run(&config));
            assert!(sroa.should_run(&config));
        }
        
        if config.optimization_level >= OptimizationLevel::Default {
            assert!(sccp.should_run(&config));
            assert!(gvn.should_run(&config));
            assert!(licm.should_run(&config));
            assert!(tail_call.should_run(&config));
            assert!(jump_threading.should_run(&config));
        }
    }
}

#[test]
fn test_pass_dependencies() {
    init_test_tracing();
    info!("Testing pass dependencies");
    
    // Test that passes declare their dependencies correctly
    let sccp = SccpPass::new();
    let licm = LicmPass::new();
    let gvn = GvnPass::new();
    let sroa = SroaPass::new();
    let mem2reg = Mem2RegPass::new();
    let tail_call = TailCallPass::new();
    let jump_threading = JumpThreadingPass::new();
    
    // Check key dependencies
    assert!(sccp.dependencies().contains(&"mem2reg".to_string()));
    assert!(gvn.dependencies().contains(&"mem2reg".to_string()));
    assert!(licm.dependencies().contains(&"loop_simplify".to_string()));
    assert!(sroa.dependencies().contains(&"instcombine".to_string()));
    assert!(tail_call.dependencies().contains(&"mem2reg".to_string()));
    assert!(jump_threading.dependencies().contains(&"instcombine".to_string()));
    
    // Mem2reg should have no dependencies (base pass)
    assert!(mem2reg.dependencies().is_empty());
}

#[test]
fn test_pass_execution_time_estimates() {
    init_test_tracing();
    info!("Testing pass execution time estimates");
    
    let passes: Vec<Box<dyn OptimizationPass<'_>>> = vec![
        Box::new(Mem2RegPass::new()),
        Box::new(SroaPass::new()),
        Box::new(SccpPass::new()),
        Box::new(GvnPass::new()),
        Box::new(LicmPass::new()),
        Box::new(TailCallPass::new()),
        Box::new(JumpThreadingPass::new()),
    ];
    
    let mut total_time = Duration::from_secs(0);
    
    for pass in &passes {
        let estimated_time = pass.estimated_execution_time();
        total_time += estimated_time;
        
        debug!("Pass {} estimated time: {:?}", pass.name(), estimated_time);
        
        // Ensure reasonable estimates (between 1ms and 1s)
        assert!(estimated_time >= Duration::from_millis(1));
        assert!(estimated_time <= Duration::from_secs(1));
    }
    
    info!("Total estimated execution time: {:?}", total_time);
    
    // Ensure total time is reasonable for a complete optimization pipeline
    assert!(total_time <= Duration::from_secs(5));
}

#[test]
fn test_pass_statistics_tracking() -> Result<()> {
    init_test_tracing();
    info!("Testing pass statistics tracking");
    
    let context = Context::create();
    let module = context.create_module("test_stats");
    let function = create_test_function(&context, &module, "test_function");
    
    let mut sccp = SccpPass::new();
    let mut gvn = GvnPass::new();
    
    // Run passes and check statistics
    let sccp_result = sccp.run_on_function(&function, &context)?;
    let gvn_result = gvn.run_on_function(&function, &context)?;
    
    let sccp_stats = sccp.get_statistics();
    let gvn_stats = gvn.get_statistics();
    
    debug!("SCCP stats: {:?}", sccp_stats);
    debug!("GVN stats: {:?}", gvn_stats);
    
    // Verify statistics structure
    assert!(sccp_stats.total_executions >= 1);
    assert!(gvn_stats.total_executions >= 1);
    
    Ok(())
}
