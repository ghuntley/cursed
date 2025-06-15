/// LLVM Optimization Passes Benchmark
/// 
/// Benchmarks for measuring the performance and effectiveness of
/// real LLVM optimization passes in the CURSED compiler.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use cursed::optimization::llvm_advanced::{AdvancedOptimizationManager, AdvancedOptimizationConfig};
use cursed::optimization::OptimizationConfig;

use inkwell::{
    context::Context,
    module::Module,
    builder::Builder,
    values::FunctionValue,
    types::BasicTypeEnum,
    IntPredicate,
};
use std::time::Duration;

/// Benchmark utilities for creating test IR
struct BenchmarkUtils<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> BenchmarkUtils<'ctx> {
    fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        Self {
            context,
            module,
            builder,
        }
    }
    
    /// Create function with many arithmetic operations for constant propagation
    fn create_arithmetic_heavy_function(&self, name: &str, operation_count: usize) -> FunctionValue<'ctx> {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function(name, fn_type, None);
        
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);
        
        // Chain of arithmetic operations with constants
        let mut result = i32_type.const_int(1, false);
        for i in 1..=operation_count {
            let const_val = i32_type.const_int(i as u64, false);
            let intermediate = self.builder.build_int_add(result, const_val, &format!("add_{}", i)).unwrap();
            let multiplier = i32_type.const_int(2, false);
            result = self.builder.build_int_mul(intermediate, multiplier, &format!("mul_{}", i)).unwrap();
        }
        
        self.builder.build_return(Some(&result)).unwrap();
        function
    }
    
    /// Create nested loops for loop optimization
    fn create_nested_loops_function(&self, name: &str, depth: usize, iterations: usize) -> FunctionValue<'ctx> {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function(name, fn_type, None);
        
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);
        
        // Create nested loop structure
        self.create_loop_nest(function, depth, iterations, 0);
        
        function
    }
    
    fn create_loop_nest(&self, function: FunctionValue<'ctx>, remaining_depth: usize, iterations: usize, level: usize) {
        if remaining_depth == 0 {
            // Innermost loop body
            let i32_type = self.context.i32_type();
            let zero = i32_type.const_int(0, false);
            self.builder.build_return(Some(&zero)).unwrap();
            return;
        }
        
        let i32_type = self.context.i32_type();
        let loop_header = self.context.append_basic_block(function, &format!("loop_header_{}", level));
        let loop_body = self.context.append_basic_block(function, &format!("loop_body_{}", level));
        let loop_exit = self.context.append_basic_block(function, &format!("loop_exit_{}", level));
        
        // Counter allocation
        let counter_alloca = self.builder.build_alloca(i32_type, &format!("counter_{}", level)).unwrap();
        let zero = i32_type.const_int(0, false);
        self.builder.build_store(counter_alloca, zero).unwrap();
        self.builder.build_unconditional_branch(loop_header).unwrap();
        
        // Loop header
        self.builder.position_at_end(loop_header);
        let counter_val = self.builder.build_load(i32_type, counter_alloca, &format!("counter_val_{}", level)).unwrap().into_int_value();
        let limit = i32_type.const_int(iterations as u64, false);
        let cmp = self.builder.build_int_compare(IntPredicate::SLT, counter_val, limit, &format!("cmp_{}", level)).unwrap();
        self.builder.build_conditional_branch(cmp, loop_body, loop_exit).unwrap();
        
        // Loop body
        self.builder.position_at_end(loop_body);
        let one = i32_type.const_int(1, false);
        let incremented = self.builder.build_int_add(counter_val, one, &format!("inc_{}", level)).unwrap();
        self.builder.build_store(counter_alloca, incremented).unwrap();
        
        if remaining_depth > 1 {
            self.create_loop_nest(function, remaining_depth - 1, iterations, level + 1);
        } else {
            self.builder.build_unconditional_branch(loop_header).unwrap();
        }
        
        // Loop exit
        self.builder.position_at_end(loop_exit);
        if level == 0 {
            self.builder.build_return(Some(&counter_val)).unwrap();
        }
    }
    
    /// Create function with many small functions to inline
    fn create_inlining_candidates(&self, name: &str, function_count: usize) -> Vec<FunctionValue<'ctx>> {
        let i32_type = self.context.i32_type();
        let mut functions = Vec::new();
        
        // Create small functions
        for i in 0..function_count {
            let fn_name = format!("{}_{}", name, i);
            let fn_type = i32_type.fn_type(&[i32_type.into()], false);
            let function = self.module.add_function(&fn_name, fn_type, None);
            
            let entry = self.context.append_basic_block(function, "entry");
            self.builder.position_at_end(entry);
            
            let param = function.get_nth_param(0).unwrap().into_int_value();
            let const_val = i32_type.const_int(i as u64 + 1, false);
            let result = self.builder.build_int_add(param, const_val, "result").unwrap();
            self.builder.build_return(Some(&result)).unwrap();
            
            functions.push(function);
        }
        
        // Create caller function
        let caller_name = format!("{}_caller", name);
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let caller = self.module.add_function(&caller_name, fn_type, None);
        
        let entry = self.context.append_basic_block(caller, "entry");
        self.builder.position_at_end(entry);
        
        let param = caller.get_nth_param(0).unwrap();
        let mut result = param;
        
        // Call each small function
        for function in &functions {
            let call = self.builder.build_call(*function, &[result.into()], "call").unwrap();
            result = call.try_as_basic_value().left().unwrap();
        }
        
        self.builder.build_return(Some(&result)).unwrap();
        functions.push(caller);
        
        functions
    }
    
    /// Create function with many dead code blocks
    fn create_dead_code_heavy_function(&self, name: &str, dead_block_count: usize) -> FunctionValue<'ctx> {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function(name, fn_type, None);
        
        let entry = self.context.append_basic_block(function, "entry");
        let exit = self.context.append_basic_block(function, "exit");
        
        // Entry goes directly to exit
        self.builder.position_at_end(entry);
        let result = i32_type.const_int(42, false);
        self.builder.build_unconditional_branch(exit).unwrap();
        
        // Create many dead blocks
        for i in 0..dead_block_count {
            let dead_block = self.context.append_basic_block(function, &format!("dead_{}", i));
            self.builder.position_at_end(dead_block);
            
            // Add some computation in dead block
            let val1 = i32_type.const_int(i as u64, false);
            let val2 = i32_type.const_int((i + 1) as u64, false);
            let dead_result = self.builder.build_int_add(val1, val2, &format!("dead_add_{}", i)).unwrap();
            self.builder.build_return(Some(&dead_result)).unwrap();
        }
        
        // Exit block
        self.builder.position_at_end(exit);
        self.builder.build_return(Some(&result)).unwrap();
        
        function
    }
}

/// Benchmark function inlining performance
fn bench_function_inlining(c: &mut Criterion) {
    let mut group = c.benchmark_group("function_inlining");
    
    for function_count in [5, 10, 20, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("inline_functions", function_count),
            function_count,
            |b, &function_count| {
                b.iter(|| {
                    let context = Context::create();
                    let utils = BenchmarkUtils::new(&context, "inlining_test");
                    let _functions = utils.create_inlining_candidates("inline_test", function_count);
                    
                    let config = OptimizationConfig::default();
                    let manager = AdvancedOptimizationManager::new(&config).unwrap();
                    
                    black_box(manager.optimize_module(&utils.module, &context).unwrap());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark loop optimization performance
fn bench_loop_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("loop_optimization");
    
    for depth in [1, 2, 3, 4].iter() {
        group.bench_with_input(
            BenchmarkId::new("nested_loops", depth),
            depth,
            |b, &depth| {
                b.iter(|| {
                    let context = Context::create();
                    let utils = BenchmarkUtils::new(&context, "loop_test");
                    let _function = utils.create_nested_loops_function("loop_test", depth, 10);
                    
                    let config = OptimizationConfig::default();
                    let manager = AdvancedOptimizationManager::new(&config).unwrap();
                    
                    black_box(manager.optimize_module(&utils.module, &context).unwrap());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark dead code elimination performance
fn bench_dead_code_elimination(c: &mut Criterion) {
    let mut group = c.benchmark_group("dead_code_elimination");
    
    for dead_blocks in [10, 50, 100, 200].iter() {
        group.bench_with_input(
            BenchmarkId::new("dead_blocks", dead_blocks),
            dead_blocks,
            |b, &dead_blocks| {
                b.iter(|| {
                    let context = Context::create();
                    let utils = BenchmarkUtils::new(&context, "dce_test");
                    let _function = utils.create_dead_code_heavy_function("dce_test", dead_blocks);
                    
                    let config = OptimizationConfig::default();
                    let manager = AdvancedOptimizationManager::new(&config).unwrap();
                    
                    black_box(manager.optimize_module(&utils.module, &context).unwrap());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark constant propagation performance
fn bench_constant_propagation(c: &mut Criterion) {
    let mut group = c.benchmark_group("constant_propagation");
    
    for operation_count in [50, 100, 200, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("arithmetic_operations", operation_count),
            operation_count,
            |b, &operation_count| {
                b.iter(|| {
                    let context = Context::create();
                    let utils = BenchmarkUtils::new(&context, "const_prop_test");
                    let _function = utils.create_arithmetic_heavy_function("const_test", operation_count);
                    
                    let config = OptimizationConfig::default();
                    let manager = AdvancedOptimizationManager::new(&config).unwrap();
                    
                    black_box(manager.optimize_module(&utils.module, &context).unwrap());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark complete optimization pipeline
fn bench_full_optimization_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_pipeline");
    
    for complexity in [1, 2, 3, 4].iter() {
        group.bench_with_input(
            BenchmarkId::new("complex_module", complexity),
            complexity,
            |b, &complexity| {
                b.iter(|| {
                    let context = Context::create();
                    let utils = BenchmarkUtils::new(&context, "full_test");
                    
                    // Create multiple types of functions
                    let _arith_func = utils.create_arithmetic_heavy_function("arith", 50 * complexity);
                    let _loop_func = utils.create_nested_loops_function("loop", complexity, 10);
                    let _inline_funcs = utils.create_inlining_candidates("inline", 10 * complexity);
                    let _dead_func = utils.create_dead_code_heavy_function("dead", 20 * complexity);
                    
                    let config = OptimizationConfig::default();
                    let manager = AdvancedOptimizationManager::new(&config).unwrap();
                    
                    black_box(manager.optimize_module(&utils.module, &context).unwrap());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark optimization effectiveness (code size reduction)
fn bench_optimization_effectiveness(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_effectiveness");
    
    group.bench_function("code_size_reduction", |b| {
        b.iter(|| {
            let context = Context::create();
            let utils = BenchmarkUtils::new(&context, "effectiveness_test");
            
            // Create functions with optimization opportunities
            let _arith_func = utils.create_arithmetic_heavy_function("arith", 100);
            let _loop_func = utils.create_nested_loops_function("loop", 2, 10);
            let _dead_func = utils.create_dead_code_heavy_function("dead", 50);
            
            // Measure before optimization
            let size_before = utils.module.print_to_string().to_string().len();
            
            let config = OptimizationConfig::default();
            let manager = AdvancedOptimizationManager::new(&config).unwrap();
            manager.optimize_module(&utils.module, &context).unwrap();
            
            // Measure after optimization
            let size_after = utils.module.print_to_string().to_string().len();
            let stats = manager.get_statistics();
            
            black_box((size_before, size_after, stats.total_optimizations()));
        });
    });
    
    group.finish();
}

/// Benchmark optimization with different configuration levels
fn bench_optimization_levels(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_levels");
    
    let configs = vec![
        ("disabled_all", AdvancedOptimizationConfig {
            base: OptimizationConfig::default(),
            enable_inlining: false,
            enable_loop_optimization: false,
            enable_dead_code_elimination: false,
            enable_constant_propagation: false,
            enable_cse: false,
            enable_tail_calls: false,
            enable_memory_optimization: false,
            ..Default::default()
        }),
        ("enabled_all", AdvancedOptimizationConfig::default()),
        ("inlining_only", AdvancedOptimizationConfig {
            base: OptimizationConfig::default(),
            enable_inlining: true,
            enable_loop_optimization: false,
            enable_dead_code_elimination: false,
            enable_constant_propagation: false,
            enable_cse: false,
            enable_tail_calls: false,
            enable_memory_optimization: false,
            ..Default::default()
        }),
    ];
    
    for (config_name, config) in configs {
        group.bench_function(config_name, |b| {
            b.iter(|| {
                let context = Context::create();
                let utils = BenchmarkUtils::new(&context, "level_test");
                
                let _arith_func = utils.create_arithmetic_heavy_function("arith", 50);
                let _loop_func = utils.create_nested_loops_function("loop", 2, 5);
                let _inline_funcs = utils.create_inlining_candidates("inline", 5);
                
                let mut manager = AdvancedOptimizationManager::new(&config.base).unwrap();
                manager.update_config(config.clone()).unwrap();
                
                black_box(manager.optimize_module(&utils.module, &context).unwrap());
            });
        });
    }
    
    group.finish();
}

/// Benchmark memory usage during optimization
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    group.bench_function("large_module_optimization", |b| {
        b.iter(|| {
            let context = Context::create();
            let utils = BenchmarkUtils::new(&context, "memory_test");
            
            // Create a large module
            for i in 0..20 {
                let _arith_func = utils.create_arithmetic_heavy_function(&format!("arith_{}", i), 100);
                let _loop_func = utils.create_nested_loops_function(&format!("loop_{}", i), 2, 10);
            }
            
            let config = OptimizationConfig::default();
            let manager = AdvancedOptimizationManager::new(&config).unwrap();
            
            black_box(manager.optimize_module(&utils.module, &context).unwrap());
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_function_inlining,
    bench_loop_optimization,
    bench_dead_code_elimination,
    bench_constant_propagation,
    bench_full_optimization_pipeline,
    bench_optimization_effectiveness,
    bench_optimization_levels,
    bench_memory_usage
);

criterion_main!(benches);
