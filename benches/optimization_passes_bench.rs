/// Performance benchmarks for CURSED LLVM optimization passes
/// 
/// This benchmark suite measures the performance characteristics of the critical
/// optimization passes and validates that they meet performance requirements.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use cursed::codegen::llvm::passes::*;
use cursed::error::Result;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use inkwell::types::BasicType;
use inkwell::AddressSpace;
use std::time::Duration;

/// Generate a complex test function for benchmarking
fn generate_complex_function<'ctx>(
    context: &'ctx Context,
    module: &Module<'ctx>,
    name: &str,
    complexity: usize,
) -> FunctionValue<'ctx> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function(name, fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let loop_header = context.append_basic_block(function, "loop_header");
    let loop_body = context.append_basic_block(function, "loop_body");
    let loop_exit = context.append_basic_block(function, "loop_exit");
    
    let builder = context.create_builder();
    
    // Entry block
    builder.position_at_end(entry_block);
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let zero = i32_type.const_int(0, false);
    let one = i32_type.const_int(1, false);
    
    // Create multiple allocas for SROA testing
    let mut allocas = Vec::new();
    for i in 0..complexity.min(10) {
        let alloca = builder.build_alloca(i32_type, &format!("alloca_{}", i)).unwrap();
        allocas.push(alloca);
        builder.build_store(alloca, param).unwrap();
    }
    
    builder.build_unconditional_branch(loop_header).unwrap();
    
    // Loop header with PHI nodes
    builder.position_at_end(loop_header);
    let counter_phi = builder.build_phi(i32_type, "counter").unwrap();
    let acc_phi = builder.build_phi(i32_type, "accumulator").unwrap();
    
    counter_phi.add_incoming(&[(&zero, entry_block)]);
    acc_phi.add_incoming(&[(&zero, entry_block)]);
    
    let counter = counter_phi.as_basic_value().into_int_value();
    let acc = acc_phi.as_basic_value().into_int_value();
    
    // Loop condition
    let limit = i32_type.const_int(complexity as u64, false);
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::SLT,
        counter,
        limit,
        "condition"
    ).unwrap();
    
    builder.build_conditional_branch(condition, loop_body, loop_exit).unwrap();
    
    // Loop body with complex operations
    builder.position_at_end(loop_body);
    let mut current_value = counter;
    
    // Create complex computation chain for optimization testing
    for i in 0..complexity.min(20) {
        let const_val = i32_type.const_int(i as u64 + 1, false);
        
        // Arithmetic operations
        current_value = builder.build_int_add(current_value, const_val, &format!("add_{}", i)).unwrap();
        current_value = builder.build_int_mul(current_value, const_val, &format!("mul_{}", i)).unwrap();
        
        // Memory operations with allocas
        if let Some(alloca) = allocas.get(i % allocas.len()) {
            builder.build_store(*alloca, current_value).unwrap();
            let loaded = builder.build_load(i32_type, *alloca, &format!("load_{}", i)).unwrap().into_int_value();
            current_value = builder.build_int_add(current_value, loaded, &format!("loaded_add_{}", i)).unwrap();
        }
        
        // More arithmetic for constant propagation opportunities
        if i % 2 == 0 {
            let redundant_calc = builder.build_int_add(const_val, const_val, &format!("redundant_{}", i)).unwrap();
            current_value = builder.build_int_add(current_value, redundant_calc, &format!("combined_{}", i)).unwrap();
        }
    }
    
    let new_acc = builder.build_int_add(acc, current_value, "new_acc").unwrap();
    let new_counter = builder.build_int_add(counter, one, "new_counter").unwrap();
    
    counter_phi.add_incoming(&[(&new_counter, loop_body)]);
    acc_phi.add_incoming(&[(&new_acc, loop_body)]);
    
    builder.build_unconditional_branch(loop_header).unwrap();
    
    // Loop exit
    builder.position_at_end(loop_exit);
    builder.build_return(Some(&acc)).unwrap();
    
    function
}

/// Generate a function with many conditional branches for jump threading
fn generate_branchy_function<'ctx>(
    context: &'ctx Context,
    module: &Module<'ctx>,
    name: &str,
    branches: usize,
) -> FunctionValue<'ctx> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function(name, fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let mut current_value = param;
    let mut current_block = entry_block;
    
    // Create chain of conditional branches
    for i in 0..branches {
        let then_block = context.append_basic_block(function, &format!("then_{}", i));
        let else_block = context.append_basic_block(function, &format!("else_{}", i));
        let merge_block = context.append_basic_block(function, &format!("merge_{}", i));
        
        builder.position_at_end(current_block);
        
        let threshold = i32_type.const_int(i as u64, false);
        let condition = builder.build_int_compare(
            inkwell::IntPredicate::SGT,
            current_value,
            threshold,
            &format!("condition_{}", i)
        ).unwrap();
        
        builder.build_conditional_branch(condition, then_block, else_block).unwrap();
        
        // Then block
        builder.position_at_end(then_block);
        let then_value = builder.build_int_add(
            current_value,
            i32_type.const_int(10, false),
            &format!("then_value_{}", i)
        ).unwrap();
        builder.build_unconditional_branch(merge_block).unwrap();
        
        // Else block
        builder.position_at_end(else_block);
        let else_value = builder.build_int_sub(
            current_value,
            i32_type.const_int(3, false),
            &format!("else_value_{}", i)
        ).unwrap();
        builder.build_unconditional_branch(merge_block).unwrap();
        
        // Merge block
        builder.position_at_end(merge_block);
        let phi = builder.build_phi(i32_type, &format!("phi_{}", i)).unwrap();
        phi.add_incoming(&[(&then_value, then_block), (&else_value, else_block)]);
        
        current_value = phi.as_basic_value().into_int_value();
        current_block = merge_block;
    }
    
    builder.position_at_end(current_block);
    builder.build_return(Some(&current_value)).unwrap();
    
    function
}

/// Benchmark Mem2Reg pass performance
fn bench_mem2reg_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("mem2reg_pass");
    
    for complexity in [10, 50, 100, 200].iter() {
        group.throughput(Throughput::Elements(*complexity as u64));
        group.bench_with_input(
            BenchmarkId::new("complexity", complexity),
            complexity,
            |b, &complexity| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("bench_mem2reg");
                    let function = generate_complex_function(&context, &module, "test", complexity);
                    
                    let mut pass = Mem2RegPass::new();
                    let _result = pass.run_on_function(&function, &context).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark SROA pass performance
fn bench_sroa_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("sroa_pass");
    
    for complexity in [10, 50, 100, 200].iter() {
        group.throughput(Throughput::Elements(*complexity as u64));
        group.bench_with_input(
            BenchmarkId::new("complexity", complexity),
            complexity,
            |b, &complexity| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("bench_sroa");
                    let function = generate_complex_function(&context, &module, "test", complexity);
                    
                    let mut pass = SroaPass::new();
                    let _result = pass.run_on_function(&function, &context).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark SCCP pass performance
fn bench_sccp_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("sccp_pass");
    
    for complexity in [10, 50, 100, 200].iter() {
        group.throughput(Throughput::Elements(*complexity as u64));
        group.bench_with_input(
            BenchmarkId::new("complexity", complexity),
            complexity,
            |b, &complexity| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("bench_sccp");
                    let function = generate_complex_function(&context, &module, "test", complexity);
                    
                    let mut pass = SccpPass::new();
                    let _result = pass.run_on_function(&function, &context).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark GVN pass performance
fn bench_gvn_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("gvn_pass");
    
    for complexity in [10, 50, 100, 200].iter() {
        group.throughput(Throughput::Elements(*complexity as u64));
        group.bench_with_input(
            BenchmarkId::new("complexity", complexity),
            complexity,
            |b, &complexity| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("bench_gvn");
                    let function = generate_complex_function(&context, &module, "test", complexity);
                    
                    let mut pass = GvnPass::new();
                    let _result = pass.run_on_function(&function, &context).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark LICM pass performance
fn bench_licm_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("licm_pass");
    
    for complexity in [10, 50, 100, 200].iter() {
        group.throughput(Throughput::Elements(*complexity as u64));
        group.bench_with_input(
            BenchmarkId::new("complexity", complexity),
            complexity,
            |b, &complexity| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("bench_licm");
                    let function = generate_complex_function(&context, &module, "test", complexity);
                    
                    let mut pass = LicmPass::new();
                    let _result = pass.run_on_function(&function, &context).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark Tail Call Optimization pass performance
fn bench_tail_call_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("tail_call_pass");
    
    for complexity in [10, 50, 100, 200].iter() {
        group.throughput(Throughput::Elements(*complexity as u64));
        group.bench_with_input(
            BenchmarkId::new("complexity", complexity),
            complexity,
            |b, &complexity| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("bench_tail_call");
                    let function = generate_complex_function(&context, &module, "test", complexity);
                    
                    let mut pass = TailCallPass::new();
                    let _result = pass.run_on_function(&function, &context).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark Jump Threading pass performance
fn bench_jump_threading_pass(c: &mut Criterion) {
    let mut group = c.benchmark_group("jump_threading_pass");
    
    for branches in [5, 10, 20, 40].iter() {
        group.throughput(Throughput::Elements(*branches as u64));
        group.bench_with_input(
            BenchmarkId::new("branches", branches),
            branches,
            |b, &branches| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("bench_jump_threading");
                    let function = generate_branchy_function(&context, &module, "test", branches);
                    
                    let mut pass = JumpThreadingPass::new();
                    let _result = pass.run_on_function(&function, &context).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark complete optimization pipeline
fn bench_optimization_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_pipeline");
    group.sample_size(20); // Reduce sample size for complex benchmarks
    
    for complexity in [50, 100, 200].iter() {
        group.throughput(Throughput::Elements(*complexity as u64));
        group.bench_with_input(
            BenchmarkId::new("complexity", complexity),
            complexity,
            |b, &complexity| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("bench_pipeline");
                    let _function = generate_complex_function(&context, &module, "test", complexity);
                    
                    let config = PassConfiguration::default();
                    let registry = std::sync::Arc::new(std::sync::Mutex::new(PassRegistry::new(config.clone())));
                    
                    // Register passes
                    {
                        let mut reg = registry.lock().unwrap();
                        let _ = reg.register_pass(Mem2RegPass::new());
                        let _ = reg.register_pass(SroaPass::new());
                        let _ = reg.register_pass(SccpPass::new());
                        let _ = reg.register_pass(GvnPass::new());
                        let _ = reg.register_pass(LicmPass::new());
                        let _ = reg.register_pass(TailCallPass::new());
                        let _ = reg.register_pass(JumpThreadingPass::new());
                    }
                    
                    // Run pipeline
                    let mut pipeline = PipelineBuilder::new(registry, config)
                        .with_optimization_level(OptimizationLevel::Default)
                        .build();
                    
                    let _result = pipeline.execute(&module, &context).unwrap();
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark pass effectiveness (optimization ratio vs time)
fn bench_pass_effectiveness(c: &mut Criterion) {
    let mut group = c.benchmark_group("pass_effectiveness");
    
    let passes: Vec<(&str, Box<dyn Fn() -> Box<dyn OptimizationPass<'_>>>)> = vec![
        ("mem2reg", Box::new(|| Box::new(Mem2RegPass::new()))),
        ("sroa", Box::new(|| Box::new(SroaPass::new()))),
        ("sccp", Box::new(|| Box::new(SccpPass::new()))),
        ("gvn", Box::new(|| Box::new(GvnPass::new()))),
        ("licm", Box::new(|| Box::new(LicmPass::new()))),
        ("tail_call", Box::new(|| Box::new(TailCallPass::new()))),
        ("jump_threading", Box::new(|| Box::new(JumpThreadingPass::new()))),
    ];
    
    for (pass_name, pass_factory) in passes {
        group.bench_function(pass_name, |b| {
            b.iter(|| {
                let context = Context::create();
                let module = context.create_module("bench_effectiveness");
                let function = generate_complex_function(&context, &module, "test", 100);
                
                let mut pass = pass_factory();
                let result = pass.run_on_function(&function, &context).unwrap();
                
                // Return result for effectiveness analysis
                result
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    optimization_passes,
    bench_mem2reg_pass,
    bench_sroa_pass,
    bench_sccp_pass,
    bench_gvn_pass,
    bench_licm_pass,
    bench_tail_call_pass,
    bench_jump_threading_pass,
    bench_optimization_pipeline,
    bench_pass_effectiveness
);

criterion_main!(optimization_passes);
