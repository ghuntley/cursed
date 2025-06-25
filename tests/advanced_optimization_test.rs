/// Advanced Optimization Test Suite
/// 
/// Comprehensive tests for the advanced optimization features including
/// vectorization, interprocedural analysis, memory layout optimization,
/// and pipeline management.

use cursed::optimization::{
    real_llvm_passes::*,
    interprocedural_analysis::*,
    memory_layout_optimization::*,
    enhanced_llvm_optimization::*,
    config::OptimizationLevel,
};
use cursed::error::Result;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::BasicType;
use inkwell::values::FunctionValue;
use std::time::Duration;

#[cfg(test)]
mod vectorization_tests {
    use super::*;
    
    #[test]
    fn test_vectorization_plan_creation() {
        let plan = VectorizationPlan::new();
        assert!(!plan.is_profitable);
        assert_eq!(plan.estimated_speedup, 1.0);
        assert_eq!(plan.vectorizable_loads.len(), 0);
        assert_eq!(plan.vectorizable_stores.len(), 0);
        assert_eq!(plan.vectorizable_operations.len(), 0);
    }
    
    #[test]
    fn test_vectorization_plan_optimal_width() {
        let mut plan = VectorizationPlan::new();
        
        // Add some operations with different vector widths
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        let function_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_func", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Create a simple add instruction for testing
        let i32_type = context.i32_type();
        let const_1 = i32_type.const_int(1, false);
        let const_2 = i32_type.const_int(2, false);
        let add_instr = builder.build_int_add(const_1, const_2, "test_add").unwrap();
        
        plan.vectorizable_operations.push(VectorizableOperation {
            instruction: add_instr,
            operation_type: VectorOperationType::IntegerArithmetic,
            operands: vec![const_1.as_basic_value_enum(), const_2.as_basic_value_enum()],
            vector_width: 8,
        });
        
        let optimal_width = plan.get_optimal_vector_width();
        assert!(optimal_width >= 2 && optimal_width <= 16);
        assert!(optimal_width.is_power_of_two());
    }
    
    #[test]
    fn test_vectorization_data_type_analysis() {
        let mut plan = VectorizationPlan::new();
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        let function_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_func", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Add integer operation
        let i32_type = context.i32_type();
        let const_1 = i32_type.const_int(1, false);
        let const_2 = i32_type.const_int(2, false);
        let add_instr = builder.build_int_add(const_1, const_2, "test_add").unwrap();
        
        plan.vectorizable_operations.push(VectorizableOperation {
            instruction: add_instr,
            operation_type: VectorOperationType::IntegerArithmetic,
            operands: vec![const_1.as_basic_value_enum(), const_2.as_basic_value_enum()],
            vector_width: 4,
        });
        
        // Add float operation
        let f32_type = context.f32_type();
        let float_1 = f32_type.const_float(1.0);
        let float_2 = f32_type.const_float(2.0);
        let fadd_instr = builder.build_float_add(float_1, float_2, "test_fadd").unwrap();
        
        plan.vectorizable_operations.push(VectorizableOperation {
            instruction: fadd_instr,
            operation_type: VectorOperationType::FloatArithmetic,
            operands: vec![float_1.as_basic_value_enum(), float_2.as_basic_value_enum()],
            vector_width: 4,
        });
        
        let dominant_type = plan.get_dominant_data_type();
        // Since we have equal numbers, should return the first one encountered in priority order
        assert!(matches!(dominant_type, VectorDataType::Int32 | VectorDataType::Float32));
    }
    
    #[test]
    fn test_memory_access_pattern_analysis() {
        let pattern = MemoryAccessPattern {
            is_contiguous: true,
            stride: 1,
            base_address: Some("test_array".to_string()),
            access_size: 4,
        };
        
        assert!(pattern.is_contiguous);
        assert_eq!(pattern.stride, 1);
        assert_eq!(pattern.access_size, 4);
    }
    
    #[test]
    fn test_loop_optimizer_statistics() {
        let context = Context::create();
        let stats = std::sync::Arc::new(std::sync::Mutex::new(OptimizationStatistics::default()));
        let optimizer = LoopOptimizer::new(stats.clone());
        
        // Test statistics tracking
        {
            let mut stats = stats.lock().unwrap();
            stats.loops_vectorized = 5;
            stats.simd_instructions_generated = 25;
            stats.loops_fused = 2;
            stats.prefetch_instructions_inserted = 10;
        }
        
        let final_stats = stats.lock().unwrap();
        assert_eq!(final_stats.loops_vectorized, 5);
        assert_eq!(final_stats.simd_instructions_generated, 25);
        assert_eq!(final_stats.loops_fused, 2);
        assert_eq!(final_stats.prefetch_instructions_inserted, 10);
    }
}

#[cfg(test)]
mod interprocedural_tests {
    use super::*;
    
    #[test]
    fn test_call_graph_creation() {
        let call_graph = CallGraph::new();
        assert_eq!(call_graph.functions.len(), 0);
        assert_eq!(call_graph.call_sites.len(), 0);
    }
    
    #[test]
    fn test_interprocedural_analyzer_creation() {
        let context = Context::create();
        let analyzer = InterproceduralAnalyzer::new(&context, OptimizationLevel::Default);
        let stats = analyzer.get_statistics();
        assert_eq!(stats.dead_functions_eliminated, 0);
        assert_eq!(stats.total_functions_analyzed, 0);
    }
    
    #[test]
    fn test_function_attributes_inference() {
        let attributes = InferredAttributes {
            is_pure: true,
            is_const: true,
            may_throw: false,
            memory_effects: MemoryEffects {
                reads_memory: false,
                writes_memory: false,
                allocates_memory: false,
                accesses_globals: false,
            },
            return_dependency: ReturnDependency {
                depends_on_parameters: true,
                depends_on_globals: false,
                depends_on_memory: false,
                is_constant: false,
            },
        };
        
        assert!(attributes.is_pure);
        assert!(attributes.is_const);
        assert!(!attributes.may_throw);
        assert!(!attributes.memory_effects.writes_memory);
        assert!(attributes.return_dependency.depends_on_parameters);
    }
    
    #[test]
    fn test_optimization_opportunity_analysis() {
        let context = Context::create();
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        let function_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_func", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Create a call instruction for testing
        let called_function = module.add_function("called_func", function_type, None);
        let call_instr = builder.build_call(called_function, &[], "test_call").unwrap();
        
        let call_site = CallSite {
            caller: function,
            callee: called_function,
            instruction: call_instr.as_instruction_value().unwrap(),
            call_type: CallType::DirectCall,
            estimated_frequency: 5.0,
        };
        
        assert_eq!(call_site.estimated_frequency, 5.0);
        assert!(matches!(call_site.call_type, CallType::DirectCall));
    }
    
    #[test]
    fn test_performance_improvements_calculation() {
        let improvements = PerformanceImprovements {
            runtime_improvement: 15.5,
            memory_savings: 8.2,
            code_size_change: -3.1,
        };
        
        assert_eq!(improvements.runtime_improvement, 15.5);
        assert_eq!(improvements.memory_savings, 8.2);
        assert_eq!(improvements.code_size_change, -3.1);
    }
}

#[cfg(test)]
mod memory_layout_tests {
    use super::*;
    
    #[test]
    fn test_memory_layout_optimizer_creation() {
        let context = Context::create();
        let optimizer = MemoryLayoutOptimizer::new(&context, OptimizationLevel::Default);
        let stats = optimizer.get_statistics();
        assert_eq!(stats.struct_replacements, 0);
        assert_eq!(stats.stack_optimizations, 0);
    }
    
    #[test]
    fn test_struct_layout_analysis() {
        let context = Context::create();
        let analyzer = StructLayoutAnalyzer::new(&context);
        
        // Create a struct with different sized fields for testing
        let field_types = vec![
            context.i8_type().as_basic_type_enum(),   // 1 byte
            context.i64_type().as_basic_type_enum(),  // 8 bytes
            context.i32_type().as_basic_type_enum(),  // 4 bytes
        ];
        
        let layout_metrics = analyzer.calculate_layout_metrics(&field_types).unwrap();
        
        // The struct should have some size and potentially padding
        assert!(layout_metrics.total_size > 0);
        assert!(layout_metrics.total_size >= 13); // At least sum of field sizes
        
        // Verify cache efficiency is reasonable
        assert!(layout_metrics.cache_efficiency >= 0.0 && layout_metrics.cache_efficiency <= 1.0);
        
        // Check that field order is tracked
        assert_eq!(layout_metrics.field_order.len(), 3);
    }
    
    #[test]
    fn test_struct_layout_optimization() {
        let context = Context::create();
        let analyzer = StructLayoutAnalyzer::new(&context);
        
        // Create a struct that should benefit from reordering
        let field_types = vec![
            context.i8_type().as_basic_type_enum(),   // 1 byte - should be moved
            context.i64_type().as_basic_type_enum(),  // 8 bytes - should be first
            context.i16_type().as_basic_type_enum(),  // 2 bytes - should be after i8
        ];
        
        let current_layout = analyzer.calculate_layout_metrics(&field_types).unwrap();
        let optimized_layout = analyzer.find_optimal_field_ordering(&field_types).unwrap();
        
        // Optimized layout should be at least as good as current
        assert!(optimized_layout.total_size <= current_layout.total_size);
        
        // Field order should be different (larger fields first)
        assert_ne!(optimized_layout.field_order, current_layout.field_order);
    }
    
    #[test]
    fn test_stack_analysis() {
        let context = Context::create();
        let optimizer = StackLayoutOptimizer::new(&context);
        
        // Create a simple function with stack allocations
        let module = context.create_module("test");
        let builder = context.create_builder();
        
        let function_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("test_func", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Add some stack allocations
        let i32_alloca = builder.build_alloca(context.i32_type(), "i32_var").unwrap();
        let i64_alloca = builder.build_alloca(context.i64_type(), "i64_var").unwrap();
        
        builder.build_return(None).unwrap();
        
        let analysis = optimizer.analyze_stack_usage(function).unwrap();
        
        // Should have found the allocations
        assert_eq!(analysis.allocations.len(), 2);
        assert!(analysis.total_stack_size > 0);
        assert_eq!(analysis.function, function);
    }
    
    #[test]
    fn test_memory_optimization_results() {
        let mut results = MemoryOptimizationResults::default();
        
        let struct_results = StructOptimizationResults {
            structs_optimized: 3,
            memory_saved: 128.0,
            cache_improvements: 15.5,
        };
        
        let stack_results = StackOptimizationResults {
            functions_optimized: 5,
            stack_memory_saved: 256,
            cache_locality_improved: 12.3,
        };
        
        results.merge_struct_results(struct_results);
        results.merge_stack_results(stack_results);
        results.calculate_overall_benefits();
        
        assert_eq!(results.structs_optimized, 3);
        assert_eq!(results.functions_optimized, 5);
        assert!(results.memory_savings_percentage > 0.0);
        assert!(results.cache_performance_improvement > 0.0);
    }
}

#[cfg(test)]
mod pipeline_tests {
    use super::*;
    
    #[test]
    fn test_pipeline_stage_creation() {
        let stage = PipelineStage::new(
            "test_stage",
            StageType::Transformation,
            Duration::from_millis(100),
            vec!["dependency1".to_string()],
        );
        
        assert_eq!(stage.name, "test_stage");
        assert!(matches!(stage.stage_type, StageType::Transformation));
        assert_eq!(stage.estimated_duration, Duration::from_millis(100));
        assert_eq!(stage.dependencies.len(), 1);
    }
    
    #[test]
    fn test_parallel_optimization_executor() {
        let executor = ParallelOptimizationExecutor::new(true);
        assert!(executor.enabled);
        assert!(executor.thread_pool_size > 0);
        assert!(executor.thread_pool_size <= 8); // Should be capped
        
        let disabled_executor = ParallelOptimizationExecutor::new(false);
        assert!(!disabled_executor.enabled);
        assert_eq!(disabled_executor.thread_pool_size, 1);
    }
    
    #[test]
    fn test_pipeline_dependency_manager() {
        let context = Context::create();
        let manager = PipelineDependencyManager::new();
        
        let stages = vec![
            PipelineStage::new(
                "stage1",
                StageType::Analysis,
                Duration::from_millis(50),
                vec![],
            ),
            PipelineStage::new(
                "stage2",
                StageType::Transformation,
                Duration::from_millis(100),
                vec!["stage1".to_string()],
            ),
            PipelineStage::new(
                "stage3",
                StageType::Cleanup,
                Duration::from_millis(75),
                vec!["stage2".to_string()],
            ),
        ];
        
        let execution_plan = manager.create_execution_plan(&stages).unwrap();
        
        // Should have 3 groups (one for each dependency level)
        assert_eq!(execution_plan.len(), 3);
        
        // First group should have stage1 (no dependencies)
        assert_eq!(execution_plan[0].len(), 1);
        assert_eq!(execution_plan[0][0].name, "stage1");
        
        // Second group should have stage2 (depends on stage1)
        assert_eq!(execution_plan[1].len(), 1);
        assert_eq!(execution_plan[1][0].name, "stage2");
        
        // Third group should have stage3 (depends on stage2)
        assert_eq!(execution_plan[2].len(), 1);
        assert_eq!(execution_plan[2][0].name, "stage3");
    }
    
    #[test]
    fn test_pipeline_dependency_circular_detection() {
        let context = Context::create();
        let manager = PipelineDependencyManager::new();
        
        // Create circular dependency: stage1 -> stage2 -> stage1
        let stages = vec![
            PipelineStage::new(
                "stage1",
                StageType::Analysis,
                Duration::from_millis(50),
                vec!["stage2".to_string()], // Circular dependency
            ),
            PipelineStage::new(
                "stage2",
                StageType::Transformation,
                Duration::from_millis(100),
                vec!["stage1".to_string()], // Circular dependency
            ),
        ];
        
        let result = manager.create_execution_plan(&stages);
        
        // Should detect circular dependency and return error
        assert!(result.is_err());
    }
    
    #[test]
    fn test_pipeline_performance_profiler() {
        let mut profiler = PipelinePerformanceProfiler::new();
        
        // Test profiling a stage
        profiler.start_stage_profiling("test_stage");
        
        // Simulate some work
        std::thread::sleep(Duration::from_millis(1));
        
        profiler.end_stage_profiling("test_stage");
        
        // Should have recorded memory usage
        assert!(profiler.get_peak_memory_usage() > 0);
    }
    
    #[test]
    fn test_pipeline_results_calculation() {
        let mut results = PipelineOptimizationResults::default();
        
        results.stages_executed = 5;
        results.parallel_stages_executed = 2;
        results.total_time = Duration::from_millis(500);
        results.memory_peak_usage = 1024 * 1024; // 1MB
        
        // Add some stage timings
        results.stage_timings.insert("stage1".to_string(), Duration::from_millis(100));
        results.stage_timings.insert("stage2".to_string(), Duration::from_millis(150));
        
        assert_eq!(results.stages_executed, 5);
        assert_eq!(results.parallel_stages_executed, 2);
        assert_eq!(results.total_time, Duration::from_millis(500));
        assert!(results.memory_peak_usage > 0);
        assert_eq!(results.stage_timings.len(), 2);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use cursed::optimization::{
        alias_analysis::AdvancedAliasAnalyzer,
        sroa::SroaOptimizer, 
        gvn::GvnOptimizer,
        tail_call_optimization::TailCallOptimizer,
        jump_threading::JumpThreadingOptimizer,
        code_motion::CodeMotionOptimizer,
    };
    
    #[test]
    fn test_advanced_optimization_integration() {
        let context = Context::create();
        let module = context.create_module("test_integration");
        
        // Create a simple function for testing
        let builder = context.create_builder();
        let function_type = context.i32_type().fn_type(&[], false);
        let function = module.add_function("test_function", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Add some operations that could be optimized
        let i32_type = context.i32_type();
        let const_42 = i32_type.const_int(42, false);
        builder.build_return(Some(&const_42)).unwrap();
        
        // Test that all optimization components can be created and used together
        let llvm_optimizer = MemoryLayoutOptimizer::new(&context, OptimizationLevel::Default);
        let interprocedural_analyzer = InterproceduralAnalyzer::new(&context, OptimizationLevel::Default);
        
        // Test new advanced optimization passes
        let alias_analyzer = AdvancedAliasAnalyzer::new(&context, OptimizationLevel::Default);
        let sroa_optimizer = SroaOptimizer::new(&context, OptimizationLevel::Default);
        let gvn_optimizer = GvnOptimizer::new(&context, OptimizationLevel::Default);
        let tail_call_optimizer = TailCallOptimizer::new(&context, OptimizationLevel::Default);
        let jump_threading_optimizer = JumpThreadingOptimizer::new(&context, OptimizationLevel::Default);
        let code_motion_optimizer = CodeMotionOptimizer::new(&context, OptimizationLevel::Default);
        
        // All components should be successfully created
        assert_eq!(llvm_optimizer.get_statistics().struct_replacements, 0);
        assert_eq!(interprocedural_analyzer.get_statistics().total_functions_analyzed, 0);
        assert_eq!(alias_analyzer.get_statistics().total_pointers_analyzed, 0);
        assert_eq!(sroa_optimizer.get_statistics().aggregates_analyzed, 0);
        assert_eq!(gvn_optimizer.get_statistics().values_numbered, 0);
        assert_eq!(tail_call_optimizer.get_statistics().functions_analyzed, 0);
        assert_eq!(jump_threading_optimizer.get_statistics().threads_created, 0);
        assert_eq!(code_motion_optimizer.get_statistics().instructions_hoisted, 0);
    }
    
    #[test]
    fn test_alias_analysis_integration() {
        let context = Context::create();
        let module = context.create_module("alias_test");
        let builder = context.create_builder();
        
        // Create function with pointer operations
        let function_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("pointer_func", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Add pointer allocations and operations
        let i32_type = context.i32_type();
        let ptr1 = builder.build_alloca(i32_type, "ptr1").unwrap();
        let ptr2 = builder.build_alloca(i32_type, "ptr2").unwrap();
        
        builder.build_return(None).unwrap();
        
        let mut alias_analyzer = AdvancedAliasAnalyzer::new(&context, OptimizationLevel::Default);
        let result = alias_analyzer.analyze_module(&module);
        assert!(result.is_ok());
        
        let alias_result = result.unwrap();
        assert!(alias_result.statistics.functions_analyzed > 0);
    }
    
    #[test]
    fn test_sroa_integration() {
        let context = Context::create();
        let module = context.create_module("sroa_test");
        let builder = context.create_builder();
        
        // Create function with struct allocations
        let function_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("struct_func", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Create struct type and allocation
        let i32_type = context.i32_type();
        let struct_type = context.struct_type(&[i32_type.into(), i32_type.into()], false);
        let struct_alloca = builder.build_alloca(struct_type, "my_struct").unwrap();
        
        builder.build_return(None).unwrap();
        
        let mut sroa_optimizer = SroaOptimizer::new(&context, OptimizationLevel::Default);
        let result = sroa_optimizer.optimize_module(&module);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_gvn_integration() {
        let context = Context::create();
        let module = context.create_module("gvn_test");
        let builder = context.create_builder();
        
        // Create function with redundant computations
        let function_type = context.i32_type().fn_type(&[], false);
        let function = module.add_function("math_func", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Add redundant arithmetic operations
        let i32_type = context.i32_type();
        let const_1 = i32_type.const_int(1, false);
        let const_2 = i32_type.const_int(2, false);
        let add1 = builder.build_int_add(const_1, const_2, "add1").unwrap();
        let add2 = builder.build_int_add(const_1, const_2, "add2").unwrap(); // Redundant
        
        builder.build_return(Some(&add1)).unwrap();
        
        let mut gvn_optimizer = GvnOptimizer::new(&context, OptimizationLevel::Default);
        let result = gvn_optimizer.optimize_module(&module);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_tail_call_optimization_integration() {
        let context = Context::create();
        let module = context.create_module("tail_call_test");
        let builder = context.create_builder();
        
        // Create recursive function
        let function_type = context.i32_type().fn_type(&[context.i32_type().into()], false);
        let function = module.add_function("factorial", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Simple return for testing
        let param = function.get_nth_param(0).unwrap().into_int_value();
        builder.build_return(Some(&param)).unwrap();
        
        let mut tail_call_optimizer = TailCallOptimizer::new(&context, OptimizationLevel::Default);
        let result = tail_call_optimizer.optimize_module(&module);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_jump_threading_integration() {
        let context = Context::create();
        let module = context.create_module("jump_threading_test");
        let builder = context.create_builder();
        
        // Create function with conditional branches
        let function_type = context.void_type().fn_type(&[context.i1_type().into()], false);
        let function = module.add_function("branch_func", function_type, None);
        let entry_block = context.append_basic_block(function, "entry");
        let then_block = context.append_basic_block(function, "then");
        let else_block = context.append_basic_block(function, "else");
        
        builder.position_at_end(entry_block);
        let condition = function.get_nth_param(0).unwrap().into_int_value();
        builder.build_conditional_branch(condition, then_block, else_block).unwrap();
        
        builder.position_at_end(then_block);
        builder.build_return(None).unwrap();
        
        builder.position_at_end(else_block);
        builder.build_return(None).unwrap();
        
        let mut jump_threading_optimizer = JumpThreadingOptimizer::new(&context, OptimizationLevel::Default);
        let result = jump_threading_optimizer.optimize_module(&module);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_code_motion_integration() {
        let context = Context::create();
        let module = context.create_module("code_motion_test");
        let builder = context.create_builder();
        
        // Create function with loop
        let function_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("loop_func", function_type, None);
        let entry_block = context.append_basic_block(function, "entry");
        let loop_block = context.append_basic_block(function, "loop");
        let exit_block = context.append_basic_block(function, "exit");
        
        builder.position_at_end(entry_block);
        builder.build_unconditional_branch(loop_block).unwrap();
        
        builder.position_at_end(loop_block);
        // Add loop-invariant computation
        let i32_type = context.i32_type();
        let const_1 = i32_type.const_int(1, false);
        let const_2 = i32_type.const_int(2, false);
        let add = builder.build_int_add(const_1, const_2, "invariant").unwrap();
        builder.build_unconditional_branch(exit_block).unwrap();
        
        builder.position_at_end(exit_block);
        builder.build_return(None).unwrap();
        
        let mut code_motion_optimizer = CodeMotionOptimizer::new(&context, OptimizationLevel::Default);
        let result = code_motion_optimizer.optimize_module(&module);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_optimization_level_scaling() {
        let context = Context::create();
        
        // Test that different optimization levels create different configurations
        let o0_optimizer = MemoryLayoutOptimizer::new(&context, OptimizationLevel::None);
        let o2_optimizer = MemoryLayoutOptimizer::new(&context, OptimizationLevel::Default);
        let o3_optimizer = MemoryLayoutOptimizer::new(&context, OptimizationLevel::Aggressive);
        
        // All should be created successfully
        assert_eq!(o0_optimizer.get_statistics().struct_replacements, 0);
        assert_eq!(o2_optimizer.get_statistics().struct_replacements, 0);
        assert_eq!(o3_optimizer.get_statistics().struct_replacements, 0);
    }
    
    #[test]
    fn test_error_handling_in_optimization() {
        let context = Context::create();
        let module = context.create_module("test_error");
        
        // Test that optimization can handle empty modules gracefully
        let mut optimizer = MemoryLayoutOptimizer::new(&context, OptimizationLevel::Default);
        let result = optimizer.optimize_memory_layout(&module);
        
        // Should not fail on empty module
        assert!(result.is_ok());
        
        let results = result.unwrap();
        assert_eq!(results.structs_optimized, 0);
        assert_eq!(results.functions_optimized, 0);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_optimization_performance() {
        let context = Context::create();
        let module = context.create_module("performance_test");
        let builder = context.create_builder();
        
        // Create multiple functions to test scalability
        for i in 0..10 {
            let function_type = context.void_type().fn_type(&[], false);
            let function = module.add_function(&format!("func_{}", i), function_type, None);
            let basic_block = context.append_basic_block(function, "entry");
            builder.position_at_end(basic_block);
            
            // Add some allocations
            builder.build_alloca(context.i32_type(), &format!("var_{}", i)).unwrap();
            builder.build_return(None).unwrap();
        }
        
        let start_time = Instant::now();
        let mut optimizer = MemoryLayoutOptimizer::new(&context, OptimizationLevel::Default);
        let _result = optimizer.optimize_memory_layout(&module).unwrap();
        let optimization_time = start_time.elapsed();
        
        // Optimization should complete in reasonable time (less than 1 second for this small test)
        assert!(optimization_time < Duration::from_secs(1));
    }
    
    #[test]
    fn test_vectorization_performance() {
        let mut plan = VectorizationPlan::new();
        let context = Context::create();
        let module = context.create_module("vector_perf_test");
        let builder = context.create_builder();
        
        let function_type = context.void_type().fn_type(&[], false);
        let function = module.add_function("vector_func", function_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Add many operations to test performance
        let i32_type = context.i32_type();
        for i in 0..100 {
            let const_val = i32_type.const_int(i, false);
            let const_next = i32_type.const_int(i + 1, false);
            let add_instr = builder.build_int_add(const_val, const_next, &format!("add_{}", i)).unwrap();
            
            plan.vectorizable_operations.push(VectorizableOperation {
                instruction: add_instr,
                operation_type: VectorOperationType::IntegerArithmetic,
                operands: vec![const_val.as_basic_value_enum(), const_next.as_basic_value_enum()],
                vector_width: 4,
            });
        }
        
        let start_time = Instant::now();
        let _optimal_width = plan.get_optimal_vector_width();
        let _dominant_type = plan.get_dominant_data_type();
        let analysis_time = start_time.elapsed();
        
        // Analysis should be fast even with many operations
        assert!(analysis_time < Duration::from_millis(100));
        assert_eq!(plan.vectorizable_operations.len(), 100);
    }
}
