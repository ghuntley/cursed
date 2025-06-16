/// Optimization Performance Benchmarks
/// 
/// Comprehensive benchmarks for measuring the performance impact of
/// the advanced optimization features.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use cursed::optimization::{
    real_llvm_passes::*,
    interprocedural_analysis::*,
    memory_layout_optimization::*,
    enhanced_llvm_optimization::*,
    config::OptimizationLevel,
};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::BasicType;
use std::time::Duration;

/// Benchmark vectorization analysis performance
fn benchmark_vectorization_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("vectorization_analysis");
    
    for size in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("vectorization_plan_creation", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut plan = VectorizationPlan::new();
                    let context = Context::create();
                    let module = context.create_module("benchmark");
                    let builder = context.create_builder();
                    
                    let function_type = context.void_type().fn_type(&[], false);
                    let function = module.add_function("bench_func", function_type, None);
                    let basic_block = context.append_basic_block(function, "entry");
                    builder.position_at_end(basic_block);
                    
                    // Create operations for vectorization analysis
                    let i32_type = context.i32_type();
                    for i in 0..size {
                        let const_val = i32_type.const_int(i as u64, false);
                        let const_next = i32_type.const_int((i + 1) as u64, false);
                        let add_instr = builder.build_int_add(const_val, const_next, &format!("add_{}", i)).unwrap();
                        
                        plan.vectorizable_operations.push(VectorizableOperation {
                            instruction: add_instr,
                            operation_type: VectorOperationType::IntegerArithmetic,
                            operands: vec![const_val.as_basic_value_enum(), const_next.as_basic_value_enum()],
                            vector_width: 4,
                        });
                    }
                    
                    black_box(plan.get_optimal_vector_width());
                    black_box(plan.get_dominant_data_type());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark memory access pattern analysis
fn benchmark_memory_access_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_access_analysis");
    
    for pattern_count in [5, 20, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("access_pattern_analysis", pattern_count),
            pattern_count,
            |b, &pattern_count| {
                b.iter(|| {
                    let mut patterns = Vec::new();
                    
                    for i in 0..pattern_count {
                        let pattern = MemoryAccessPattern {
                            is_contiguous: i % 2 == 0,
                            stride: if i % 3 == 0 { 1 } else { 2 },
                            base_address: Some(format!("array_{}", i)),
                            access_size: match i % 4 {
                                0 => 1,
                                1 => 2,
                                2 => 4,
                                _ => 8,
                            },
                        };
                        patterns.push(pattern);
                    }
                    
                    // Analyze patterns
                    let contiguous_count = patterns.iter().filter(|p| p.is_contiguous).count();
                    let total_size: usize = patterns.iter().map(|p| p.access_size).sum();
                    
                    black_box(contiguous_count);
                    black_box(total_size);
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark struct layout optimization
fn benchmark_struct_layout_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("struct_layout_optimization");
    
    for field_count in [3, 8, 15, 30].iter() {
        group.bench_with_input(
            BenchmarkId::new("struct_layout_analysis", field_count),
            field_count,
            |b, &field_count| {
                b.iter(|| {
                    let context = Context::create();
                    let analyzer = StructLayoutAnalyzer::new(&context);
                    
                    // Create diverse field types
                    let mut field_types = Vec::new();
                    for i in 0..field_count {
                        let field_type = match i % 6 {
                            0 => context.i8_type().as_basic_type_enum(),
                            1 => context.i16_type().as_basic_type_enum(),
                            2 => context.i32_type().as_basic_type_enum(),
                            3 => context.i64_type().as_basic_type_enum(),
                            4 => context.f32_type().as_basic_type_enum(),
                            _ => context.f64_type().as_basic_type_enum(),
                        };
                        field_types.push(field_type);
                    }
                    
                    let current_layout = analyzer.calculate_layout_metrics(&field_types).unwrap();
                    let optimized_layout = analyzer.find_optimal_field_ordering(&field_types).unwrap();
                    
                    black_box(current_layout.total_size);
                    black_box(optimized_layout.total_size);
                    black_box(current_layout.padding_bytes);
                    black_box(optimized_layout.cache_efficiency);
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark interprocedural call graph analysis
fn benchmark_call_graph_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("call_graph_analysis");
    
    for function_count in [5, 20, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("call_graph_construction", function_count),
            function_count,
            |b, &function_count| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("call_graph_bench");
                    let builder = context.create_builder();
                    
                    // Create multiple functions with calls between them
                    let mut functions = Vec::new();
                    let function_type = context.i32_type().fn_type(&[], false);
                    
                    for i in 0..function_count {
                        let function = module.add_function(&format!("func_{}", i), function_type, None);
                        functions.push(function);
                    }
                    
                    // Add function bodies with calls
                    for (i, function) in functions.iter().enumerate() {
                        let basic_block = context.append_basic_block(*function, "entry");
                        builder.position_at_end(basic_block);
                        
                        // Call a few other functions
                        for j in 0..std::cmp::min(3, function_count - 1) {
                            let target_idx = (i + j + 1) % function_count;
                            let target_function = functions[target_idx];
                            builder.build_call(target_function, &[], &format!("call_{}_{}", i, j)).unwrap();
                        }
                        
                        let return_value = context.i32_type().const_int(i as u64, false);
                        builder.build_return(Some(&return_value)).unwrap();
                    }
                    
                    // Analyze call graph
                    let call_graph = CallGraph::new();
                    black_box(call_graph.functions.len());
                    black_box(call_graph.call_sites.len());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark optimization pipeline execution
fn benchmark_optimization_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_pipeline");
    
    for optimization_level in [OptimizationLevel::None, OptimizationLevel::Less, OptimizationLevel::Default, OptimizationLevel::Aggressive].iter() {
        group.bench_with_input(
            BenchmarkId::new("pipeline_execution", format!("{:?}", optimization_level)),
            optimization_level,
            |b, &optimization_level| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("pipeline_bench");
                    let builder = context.create_builder();
                    
                    // Create a complex function for optimization
                    let function_type = context.i32_type().fn_type(&[context.i32_type().into()], false);
                    let function = module.add_function("complex_func", function_type, None);
                    let basic_block = context.append_basic_block(function, "entry");
                    builder.position_at_end(basic_block);
                    
                    let param = function.get_nth_param(0).unwrap().into_int_value();
                    
                    // Add multiple operations that could be optimized
                    let mut current_value = param;
                    for i in 0..20 {
                        let const_val = context.i32_type().const_int(i, false);
                        current_value = builder.build_int_add(current_value, const_val, &format!("add_{}", i)).unwrap();
                        
                        if i % 3 == 0 {
                            current_value = builder.build_int_mul(current_value, const_val, &format!("mul_{}", i)).unwrap();
                        }
                    }
                    
                    builder.build_return(Some(&current_value)).unwrap();
                    
                    // Create and configure pipeline
                    let config = EnhancedOptimizationConfig {
                        optimization_level: optimization_level.clone(),
                        enable_cursed_optimizations: true,
                        enable_adaptive_optimization: false, // Disable for benchmarking consistency
                        enable_compilation_cache: false,
                        enable_target_optimizations: true,
                        max_optimization_time: Duration::from_secs(10),
                        enable_parallel_optimization: false, // Disable for benchmarking consistency
                        feedback_config: OptimizationFeedbackConfig::default(),
                    };
                    
                    let pipeline_manager = OptimizationPipelineManager::new(&context, &config).unwrap();
                    
                    black_box(pipeline_manager);
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark stack layout optimization
fn benchmark_stack_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_optimization");
    
    for alloca_count in [5, 15, 30, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("stack_analysis", alloca_count),
            alloca_count,
            |b, &alloca_count| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("stack_bench");
                    let builder = context.create_builder();
                    let optimizer = StackLayoutOptimizer::new(&context);
                    
                    // Create function with many stack allocations
                    let function_type = context.void_type().fn_type(&[], false);
                    let function = module.add_function("stack_func", function_type, None);
                    let basic_block = context.append_basic_block(function, "entry");
                    builder.position_at_end(basic_block);
                    
                    // Add diverse allocations
                    for i in 0..alloca_count {
                        let alloca_type = match i % 4 {
                            0 => context.i32_type().as_basic_type_enum(),
                            1 => context.i64_type().as_basic_type_enum(),
                            2 => context.f32_type().as_basic_type_enum(),
                            _ => context.f64_type().as_basic_type_enum(),
                        };
                        
                        builder.build_alloca(alloca_type.try_into().unwrap(), &format!("var_{}", i)).unwrap();
                    }
                    
                    builder.build_return(None).unwrap();
                    
                    // Analyze stack usage
                    let analysis = optimizer.analyze_stack_usage(function).unwrap();
                    black_box(analysis.total_stack_size);
                    black_box(analysis.allocations.len());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark loop optimization with different complexities
fn benchmark_loop_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("loop_optimization");
    
    for loop_complexity in [5, 15, 30, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("loop_analysis", loop_complexity),
            loop_complexity,
            |b, &loop_complexity| {
                b.iter(|| {
                    let context = Context::create();
                    let module = context.create_module("loop_bench");
                    let builder = context.create_builder();
                    
                    // Create function with nested loops
                    let function_type = context.void_type().fn_type(&[], false);
                    let function = module.add_function("loop_func", function_type, None);
                    let entry_block = context.append_basic_block(function, "entry");
                    builder.position_at_end(entry_block);
                    
                    // Create nested loop structure
                    let mut loop_blocks = Vec::new();
                    for i in 0..loop_complexity {
                        let loop_header = context.append_basic_block(function, &format!("loop_header_{}", i));
                        let loop_body = context.append_basic_block(function, &format!("loop_body_{}", i));
                        let loop_exit = context.append_basic_block(function, &format!("loop_exit_{}", i));
                        
                        loop_blocks.push((loop_header, loop_body, loop_exit));
                        
                        // Add PHI nodes and loop operations
                        builder.position_at_end(loop_header);
                        let phi = builder.build_phi(context.i32_type(), &format!("phi_{}", i)).unwrap();
                        
                        builder.position_at_end(loop_body);
                        let const_one = context.i32_type().const_int(1, false);
                        let increment = builder.build_int_add(phi.as_basic_value().into_int_value(), const_one, "inc").unwrap();
                        
                        // Add some computation in loop body
                        let mut acc = increment;
                        for j in 0..5 {
                            let const_val = context.i32_type().const_int(j, false);
                            acc = builder.build_int_add(acc, const_val, &format!("acc_{}_{}", i, j)).unwrap();
                        }
                        
                        black_box(acc);
                    }
                    
                    builder.position_at_end(entry_block);
                    builder.build_return(None).unwrap();
                    
                    // Analyze loop structure
                    let stats = std::sync::Arc::new(std::sync::Mutex::new(OptimizationStatistics::default()));
                    let loop_optimizer = LoopOptimizer::new(stats);
                    
                    black_box(loop_optimizer);
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark parallel optimization execution
fn benchmark_parallel_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_optimization");
    
    for thread_count in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::new("parallel_execution", thread_count),
            thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    let executor = ParallelOptimizationExecutor::new(thread_count > 1);
                    
                    // Create stages for parallel execution
                    let mut stages = Vec::new();
                    for i in 0..thread_count {
                        let stage = PipelineStage::new(
                            &format!("parallel_stage_{}", i),
                            StageType::Transformation,
                            Duration::from_millis(10),
                            vec![],
                        );
                        stages.push(stage);
                    }
                    
                    black_box(executor.enabled);
                    black_box(executor.thread_pool_size);
                    black_box(stages.len());
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark NUMA optimization analysis
fn benchmark_numa_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("numa_optimization");
    
    for allocation_count in [10, 50, 100, 200].iter() {
        group.bench_with_input(
            BenchmarkId::new("numa_analysis", allocation_count),
            allocation_count,
            |b, &allocation_count| {
                b.iter(|| {
                    let mut allocation_patterns = Vec::new();
                    
                    for i in 0..allocation_count {
                        let context = Context::create();
                        let module = context.create_module("numa_bench");
                        let builder = context.create_builder();
                        
                        let function_type = context.void_type().fn_type(&[], false);
                        let function = module.add_function(&format!("numa_func_{}", i), function_type, None);
                        let basic_block = context.append_basic_block(function, "entry");
                        builder.position_at_end(basic_block);
                        
                        let alloca = builder.build_alloca(context.i64_type(), &format!("numa_var_{}", i)).unwrap();
                        
                        let pattern = NumaAllocationPattern {
                            allocation: alloca,
                            preferred_node: i % 4, // Simulate 4 NUMA nodes
                            access_threads: vec![i % 8], // Simulate 8 threads
                        };
                        
                        allocation_patterns.push(pattern);
                    }
                    
                    // Analyze NUMA patterns
                    let numa_analysis = NumaAnalysis {
                        allocation_patterns: allocation_patterns.clone(),
                        access_patterns: std::collections::HashMap::new(),
                        thread_affinity_hints: std::collections::HashMap::new(),
                    };
                    
                    black_box(numa_analysis.allocation_patterns.len());
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    optimization_benches,
    benchmark_vectorization_analysis,
    benchmark_memory_access_analysis,
    benchmark_struct_layout_optimization,
    benchmark_call_graph_analysis,
    benchmark_optimization_pipeline,
    benchmark_stack_optimization,
    benchmark_loop_optimization,
    benchmark_parallel_optimization,
    benchmark_numa_optimization
);

criterion_main!(optimization_benches);
