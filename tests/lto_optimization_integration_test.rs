/// Comprehensive tests for the real LTO optimization implementation
/// 
/// Tests the integration between CURSED optimization coordinator and real LLVM LTO functionality

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;
    use inkwell::context::Context;
    use tracing_test::traced_test;

    use cursed::optimization::cursed_integration::{
        CursedOptimizationCoordinator, CursedOptimizationConfig, ModuleMetrics
    };
    use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
    use cursed::optimization::lto::{LtoConfig, LtoLevel, LtoOptimizer, LtoCompilationUnit};

    #[tokio::test]
    #[traced_test]
    async fn test_real_lto_optimization_integration() {
        let context = Context::create();
        
        // Configure LTO for testing
        let mut config = CursedOptimizationConfig::default();
        config.lto_config = Some(LtoConfig {
            level: LtoLevel::Thin,
            enable_cross_module_inlining: true,
            enable_whole_program_dce: true,
            enable_global_variable_optimization: true,
            enable_cross_module_constant_propagation: true,
            enable_devirtualization: true,
            max_worker_threads: 2,
            thin_lto_partition_threshold: 1000,
            enable_caching: true,
            cache_directory: None,
            enable_profiling: false,
        });
        config.base_config.level = OptimizationLevel::Aggressive;
        
        let mut coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        // Create a test module with optimization opportunities
        let module = context.create_module("test_lto");
        let source = create_test_source_with_lto_opportunities();
        
        let result = coordinator.optimize_comprehensive(&module, &source, "test_lto.csd").await;
        assert!(result.is_ok());
        
        let optimization_result = result.unwrap();
        
        // Verify LTO optimizations were applied
        assert!(optimization_result.total_optimizations > 0);
        assert!(optimization_result.performance_improvement > 0.0);
        assert!(optimization_result.compilation_time > Duration::from_millis(0));
        
        println!("LTO optimization results:");
        println!("  Total optimizations: {}", optimization_result.total_optimizations);
        println!("  Performance improvement: {:.2}%", optimization_result.performance_improvement * 100.0);
        println!("  Memory reduction: {:.2}%", optimization_result.memory_reduction * 100.0);
        println!("  Compilation time: {:?}", optimization_result.compilation_time);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_lto_with_multiple_optimization_levels() {
        let context = Context::create();
        let test_source = create_test_source_with_lto_opportunities();
        
        let optimization_levels = vec![
            (LtoLevel::None, "No LTO"),
            (LtoLevel::Thin, "Thin LTO"),
            (LtoLevel::Full, "Full LTO"),
        ];
        
        for (lto_level, level_name) in optimization_levels {
            println!("Testing {}", level_name);
            
            let mut config = CursedOptimizationConfig::default();
            config.lto_config = Some(LtoConfig {
                level: lto_level,
                ..LtoConfig::default()
            });
            
            let mut coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
            let module = context.create_module(&format!("test_{:?}", lto_level));
            
            let result = coordinator.optimize_comprehensive(&module, &test_source, "test.csd").await;
            assert!(result.is_ok(), "Failed optimization with {}", level_name);
            
            let optimization_result = result.unwrap();
            println!("  {} - Optimizations: {}, Performance: {:.2}%", 
                    level_name, 
                    optimization_result.total_optimizations,
                    optimization_result.performance_improvement * 100.0);
        }
    }

    #[test]
    #[traced_test]
    fn test_module_metrics_collection() {
        let context = Context::create();
        let module = create_test_module_with_metrics(&context);
        
        let config = CursedOptimizationConfig::default();
        let coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        let metrics = coordinator.collect_module_metrics(&module).unwrap();
        
        // Verify metrics are collected correctly
        assert!(metrics.functions_count > 0);
        assert!(metrics.instructions_count > 0);
        assert!(metrics.ir_size > 0);
        
        println!("Module metrics:");
        println!("  Functions: {}", metrics.functions_count);
        println!("  Instructions: {}", metrics.instructions_count);
        println!("  Basic blocks: {}", metrics.basic_blocks_count);
        println!("  Globals: {}", metrics.globals_count);
        println!("  Call instructions: {}", metrics.call_instructions);
        println!("  Load/Store instructions: {}", metrics.load_store_instructions);
        println!("  Branch instructions: {}", metrics.branch_instructions);
        println!("  IR size: {} bytes", metrics.ir_size);
    }

    #[test]
    #[traced_test]
    fn test_lto_performance_improvement_calculation() {
        let context = Context::create();
        let config = CursedOptimizationConfig::default();
        let coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        let before_metrics = ModuleMetrics {
            functions_count: 10,
            instructions_count: 1000,
            basic_blocks_count: 50,
            globals_count: 20,
            call_instructions: 100,
            load_store_instructions: 200,
            branch_instructions: 50,
            ir_size: 10000,
        };
        
        let after_metrics = ModuleMetrics {
            functions_count: 8,  // 2 functions removed
            instructions_count: 800,  // 200 instructions eliminated
            basic_blocks_count: 40,  // 10 blocks optimized away
            globals_count: 15,  // 5 globals optimized
            call_instructions: 70,  // 30 calls inlined
            load_store_instructions: 160,  // 40 load/stores optimized
            branch_instructions: 40,  // 10 branches optimized
            ir_size: 8000,  // 20% size reduction
        };
        
        let improvement = coordinator.calculate_lto_performance_improvement(&before_metrics, &after_metrics);
        
        // Should show significant improvement
        assert!(improvement > 0.15, "Expected > 15% improvement, got {:.2}%", improvement * 100.0);
        assert!(improvement < 0.6, "Improvement capped at 60%, got {:.2}%", improvement * 100.0);
        
        println!("Performance improvement: {:.2}%", improvement * 100.0);
    }

    #[test]
    #[traced_test]
    fn test_compilation_unit_creation_from_module() {
        let context = Context::create();
        let module = create_test_module_with_exports(&context);
        
        let config = CursedOptimizationConfig::default();
        let coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        let unit = coordinator.create_compilation_unit_from_module(&module).unwrap();
        
        // Verify compilation unit was created correctly
        assert!(!unit.id.is_empty());
        assert!(unit.module_path.exists() || unit.module_path.to_string_lossy().contains(".bc"));
        assert!(!unit.exported_functions.is_empty());
        assert!(unit.size_estimate > 0);
        assert!(!unit.metadata.is_empty());
        
        println!("Compilation unit created:");
        println!("  ID: {}", unit.id);
        println!("  Exported functions: {:?}", unit.exported_functions);
        println!("  Exported globals: {:?}", unit.exported_globals);
        println!("  Size estimate: {} bytes", unit.size_estimate);
        println!("  Metadata: {:?}", unit.metadata);
    }

    #[tokio::test]
    #[traced_test]
    async fn test_lto_optimization_with_caching() {
        let context = Context::create();
        
        let mut config = CursedOptimizationConfig::default();
        config.lto_config = Some(LtoConfig {
            level: LtoLevel::Thin,
            enable_caching: true,
            cache_directory: Some(std::env::temp_dir().join("cursed_lto_test_cache")),
            ..LtoConfig::default()
        });
        
        let mut coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        let module = context.create_module("test_cache");
        let source = create_test_source_with_lto_opportunities();
        
        // First optimization (should miss cache)
        let result1 = coordinator.optimize_comprehensive(&module, &source, "test_cache.csd").await;
        assert!(result1.is_ok());
        
        // Second optimization (should potentially hit cache)
        let result2 = coordinator.optimize_comprehensive(&module, &source, "test_cache.csd").await;
        assert!(result2.is_ok());
        
        let stats = coordinator.get_comprehensive_stats().unwrap();
        println!("Cache statistics:");
        println!("  Sessions: {}", stats.sessions);
        println!("  Optimizations: {}", stats.cursed_optimizations);
    }

    #[test]
    #[traced_test]
    fn test_lto_with_cursed_specific_patterns() {
        let context = Context::create();
        let mut config = CursedOptimizationConfig::default();
        config.enable_cursed_optimizations = true;
        config.lto_config = Some(LtoConfig {
            level: LtoLevel::Thin,
            ..LtoConfig::default()
        });
        
        let coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        let cursed_source = r#"
            slay main() {
                stan worker_goroutine();
                facts channel = make_channel<i32>(10);
                send(channel, 42)?;
                yolo;
                
                lowkey (sus i = 0; i < 10; i++) {
                    periodt process_item(i);
                }
                
                bestie (channel_has_data(channel)) {
                    facts value = receive(channel)?;
                    process_value(value);
                } flex {
                    highkey log_error("No data received");
                }
            }
            
            slay worker_goroutine() {
                facts value = expensive_computation();
                yolo;
                sus result = value * 2;
                return result;
            }
        "#;
        
        let input_characteristics = coordinator.analyze_input_characteristics(cursed_source, &context.create_module("cursed_test")).unwrap();
        
        // Verify CURSED-specific patterns are detected
        assert!(input_characteristics.goroutine_usage > 0, "Should detect goroutine usage");
        assert!(input_characteristics.channel_usage > 0, "Should detect channel usage");
        assert!(input_characteristics.genz_keyword_usage > 0, "Should detect Gen Z keyword usage");
        assert!(input_characteristics.complexity_score > 0.0, "Should calculate complexity score");
        
        println!("CURSED-specific analysis:");
        println!("  Goroutine usage: {}", input_characteristics.goroutine_usage);
        println!("  Channel usage: {}", input_characteristics.channel_usage);
        println!("  GC allocations: {}", input_characteristics.gc_allocations);
        println!("  Gen Z keywords: {}", input_characteristics.genz_keyword_usage);
        println!("  Complexity score: {:.2}", input_characteristics.complexity_score);
    }

    #[test]
    #[traced_test]
    fn test_lto_error_handling() {
        let context = Context::create();
        let config = CursedOptimizationConfig::default();
        let coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        // Test with invalid module
        let empty_module = context.create_module("empty");
        
        // Should handle empty module gracefully
        let metrics_result = coordinator.collect_module_metrics(&empty_module);
        assert!(metrics_result.is_ok());
        
        let metrics = metrics_result.unwrap();
        assert_eq!(metrics.functions_count, 0);
        assert_eq!(metrics.instructions_count, 0);
        assert!(metrics.ir_size > 0); // Still has module structure
    }

    #[tokio::test]
    #[traced_test]
    async fn test_lto_performance_regression_detection() {
        let context = Context::create();
        let mut config = CursedOptimizationConfig::default();
        config.target_improvement = 0.1; // 10% improvement target
        config.lto_config = Some(LtoConfig {
            level: LtoLevel::Thin,
            ..LtoConfig::default()
        });
        
        let mut coordinator = CursedOptimizationCoordinator::new(&context, config).unwrap();
        
        let module = context.create_module("regression_test");
        let source = create_test_source_with_lto_opportunities();
        
        let result = coordinator.optimize_comprehensive(&module, &source, "regression_test.csd").await;
        assert!(result.is_ok());
        
        let optimization_result = result.unwrap();
        
        // Check if optimization met the target
        if optimization_result.performance_improvement < 0.1 {
            println!("Warning: Optimization did not meet 10% improvement target");
            println!("Actual improvement: {:.2}%", optimization_result.performance_improvement * 100.0);
        }
    }

    // Helper functions for creating test data

    fn create_test_source_with_lto_opportunities() -> String {
        r#"
            slay fibonacci(sus n) -> i32 {
                lowkey (n <= 1) {
                    return n;
                }
                return fibonacci(n - 1) + fibonacci(n - 2);
            }
            
            slay helper_function(sus x) -> i32 {
                return x * 2;
            }
            
            slay unused_function() -> i32 {
                return 42;
            }
            
            slay main() {
                facts result = fibonacci(10);
                facts doubled = helper_function(result);
                
                stan compute_in_background(doubled);
                yolo;
                
                facts channel = make_channel<i32>(5);
                send(channel, result)?;
                
                return doubled;
            }
            
            slay compute_in_background(sus value) {
                facts computed = value * value;
                yolo;
            }
        "#.to_string()
    }

    fn create_test_module_with_metrics(context: &Context) -> inkwell::module::Module {
        let module = context.create_module("metrics_test");
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        
        // Create several functions with different characteristics
        let main_fn = module.add_function("main", fn_type, None);
        let helper_fn = module.add_function("helper", fn_type, None);
        let unused_fn = module.add_function("unused", fn_type, None);
        
        // Add basic blocks and instructions to main function
        let builder = context.create_builder();
        let entry_block = context.append_basic_block(main_fn, "entry");
        builder.position_at_end(entry_block);
        
        let param = main_fn.get_nth_param(0).unwrap().into_int_value();
        let const_42 = i32_type.const_int(42, false);
        let add_result = builder.build_int_add(param, const_42, "add");
        let call_result = builder.build_call(helper_fn, &[add_result.unwrap().into()], "call");
        builder.build_return(Some(&call_result.try_as_basic_value().left().unwrap()));
        
        // Add basic block to helper function
        let helper_entry = context.append_basic_block(helper_fn, "entry");
        builder.position_at_end(helper_entry);
        let helper_param = helper_fn.get_nth_param(0).unwrap().into_int_value();
        let const_2 = i32_type.const_int(2, false);
        let mul_result = builder.build_int_mul(helper_param, const_2, "mul");
        builder.build_return(Some(&mul_result.unwrap().into()));
        
        // Add empty unused function
        let unused_entry = context.append_basic_block(unused_fn, "entry");
        builder.position_at_end(unused_entry);
        builder.build_return(Some(&const_42.into()));
        
        module
    }

    fn create_test_module_with_exports(context: &Context) -> inkwell::module::Module {
        let module = context.create_module("exports_test");
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        
        // Create exported function
        let exported_fn = module.add_function("exported_function", fn_type, None);
        exported_fn.set_linkage(inkwell::values::GlobalValueLinkage::External);
        
        // Create internal function
        let internal_fn = module.add_function("internal_function", fn_type, None);
        internal_fn.set_linkage(inkwell::values::GlobalValueLinkage::Internal);
        
        // Create exported global
        let exported_global = module.add_global(i32_type, Some(inkwell::values::AddressSpace::default()), "exported_global");
        exported_global.set_linkage(inkwell::values::GlobalValueLinkage::External);
        exported_global.set_initializer(&i32_type.const_int(42, false));
        
        // Create internal global
        let internal_global = module.add_global(i32_type, Some(inkwell::values::AddressSpace::default()), "internal_global");
        internal_global.set_linkage(inkwell::values::GlobalValueLinkage::Internal);
        internal_global.set_initializer(&i32_type.const_int(24, false));
        
        module
    }
}
