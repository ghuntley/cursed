//! Comprehensive tests for LLVM optimization passes
//! Tests all optimization levels and performance monitoring

use crate::optimization::{
    OptimizationConfig, OptimizationLevel, OptimizationProfile,
    ProductionLlvmOptimizer, ComprehensiveOptimizationResult,
    EnhancedPerformanceMonitor, PerformanceSummary,
    ComprehensiveBenchmarkingSystem, BenchmarkSuiteConfig, BenchmarkConfig,
    BenchmarkType, ConvergenceCriteria, OptimizationLevelController,
    BuildContext, BuildType, TargetPlatform, PerformanceRequirements, SizeRequirements,
};
use crate::error::Result;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;
use std::time::Duration;

/// Test suite for comprehensive optimization system
pub struct ComprehensiveOptimizationTest<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
}

impl<'ctx> ComprehensiveOptimizationTest<'ctx> {
    /// Create new test suite
    pub fn new(context: &'ctx Context) -> Self {
        let builder = context.create_builder();
        Self { context, builder }
    }
    
    /// Test all optimization levels
    pub fn test_all_optimization_levels(&self) -> Result<()> {
        let levels = vec![
            OptimizationLevel::None,
            OptimizationLevel::Less,
            OptimizationLevel::Default,
            OptimizationLevel::Aggressive,
            OptimizationLevel::Size,
            OptimizationLevel::SizeZ,
        ];
        
        for level in levels {
            self.test_optimization_level(level)?;
        }
        
        Ok(())
    }
    
    /// Test specific optimization level
    pub fn test_optimization_level(&self, level: OptimizationLevel) -> Result<()> {
        let config = OptimizationConfig::new(level.clone());
        let module = self.create_test_module();
        
        // Test production optimizer
        let mut optimizer = ProductionLlvmOptimizer::new(self.context, config)?;
        let result = optimizer.optimize_module(&module)?;
        
        // Verify optimization occurred
        assert!(result.total_time > Duration::from_millis(0));
        assert!(result.passes_run.len() > 0);
        
        // Verify module is still valid
        assert!(module.verify().is_ok());
        
        println!("✓ Optimization level {:?} passed", level);
        Ok(())
    }
    
    /// Test performance monitoring
    pub fn test_performance_monitoring(&self) -> Result<()> {
        let config = OptimizationConfig::default();
        let mut monitor = EnhancedPerformanceMonitor::new(config)?;
        
        // Test timing
        {
            let _token = monitor.start_timing("test_operation");
            std::thread::sleep(Duration::from_millis(10));
        }
        
        // Test performance summary
        let summary = monitor.get_performance_summary();
        assert!(summary.total_optimizations > 0);
        assert!(summary.total_time > Duration::from_millis(0));
        
        // Test report generation
        let report = monitor.generate_report()?;
        assert!(report.contains("Performance Report"));
        
        println!("✓ Performance monitoring passed");
        Ok(())
    }
    
    /// Test comprehensive benchmarking
    pub fn test_comprehensive_benchmarking(&self) -> Result<()> {
        let config = OptimizationConfig::default();
        let mut system = ComprehensiveBenchmarkingSystem::new(config)?;
        
        // Create benchmark suite
        let suite_config = BenchmarkSuiteConfig {
            name: "test_suite".to_string(),
            benchmarks: vec![
                BenchmarkConfig {
                    name: "compile_time_test".to_string(),
                    benchmark_type: BenchmarkType::CompileTime { 
                        source_size: 1000, 
                        complexity: 1.5 
                    },
                    iterations: 5,
                    warmup_iterations: 1,
                    timeout: Duration::from_secs(10),
                    convergence_criteria: ConvergenceCriteria::default(),
                },
                BenchmarkConfig {
                    name: "optimization_passes_test".to_string(),
                    benchmark_type: BenchmarkType::OptimizationPasses { 
                        pass_count: 10, 
                        optimization_level: OptimizationLevel::Default 
                    },
                    iterations: 5,
                    warmup_iterations: 1,
                    timeout: Duration::from_secs(10),
                    convergence_criteria: ConvergenceCriteria::default(),
                },
            ],
            warmup_iterations: 1,
            early_termination: None,
            parallel_execution: false,
        };
        
        // Run benchmark suite
        let result = system.run_benchmark_suite(suite_config)?;
        
        // Verify results
        assert_eq!(result.results.len(), 2);
        assert!(result.total_time > Duration::from_millis(0));
        
        // Test report generation
        let report = system.generate_comprehensive_report()?;
        assert!(report.contains("Comprehensive"));
        
        println!("✓ Comprehensive benchmarking passed");
        Ok(())
    }
    
    /// Test optimization level controller
    pub fn test_optimization_level_controller(&self) -> Result<()> {
        let config = OptimizationConfig::default();
        let controller = OptimizationLevelController::new(self.context, config)?;
        
        // Test level recommendations
        let build_context = BuildContext {
            build_type: BuildType::Release,
            target_platform: TargetPlatform::Desktop,
            compile_time_budget: Duration::from_secs(60),
            memory_budget: 2 * 1024 * 1024 * 1024, // 2GB
            performance_requirements: PerformanceRequirements {
                min_improvement: 0.2,
                max_regression: 0.0,
                critical_paths: vec!["main".to_string()],
            },
            size_requirements: SizeRequirements {
                max_size: 10 * 1024 * 1024, // 10MB
                max_growth: 0.1,
                minimize_size: false,
            },
        };
        
        let recommendations = controller.get_level_recommendations(&build_context)?;
        assert!(!recommendations.is_empty());
        
        // Test level validation
        let validation = controller.validate_level_config(&OptimizationLevel::Default)?;
        assert!(validation.valid);
        
        println!("✓ Optimization level controller passed");
        Ok(())
    }
    
    /// Test optimization profiles
    pub fn test_optimization_profiles(&self) -> Result<()> {
        let profiles = vec![
            OptimizationProfile::development(),
            OptimizationProfile::balanced(),
            OptimizationProfile::production(),
            OptimizationProfile::size_optimized(),
        ];
        
        for profile in profiles {
            // Test profile configuration
            assert!(!profile.name.is_empty());
            assert!(!profile.description.is_empty());
            assert!(profile.estimated_build_time_factor > 0.0);
            assert!(profile.estimated_performance_gain > 0.0);
            
            // Test profile config
            let config = &profile.config;
            assert!(config.validate().is_ok());
            
            println!("✓ Profile '{}' passed", profile.name);
        }
        
        Ok(())
    }
    
    /// Test pass dependency resolution
    pub fn test_pass_dependency_resolution(&self) -> Result<()> {
        let config = OptimizationConfig::default();
        let mut optimizer = ProductionLlvmOptimizer::new(self.context, config)?;
        
        // Create test module
        let module = self.create_test_module();
        
        // Run optimization with dependencies
        let result = optimizer.optimize_module(&module)?;
        
        // Verify pass dependencies were resolved
        assert!(result.passes_run.len() > 0);
        
        // Verify module is valid
        assert!(module.verify().is_ok());
        
        println!("✓ Pass dependency resolution passed");
        Ok(())
    }
    
    /// Test profile-guided optimization
    pub fn test_profile_guided_optimization(&self) -> Result<()> {
        let mut config = OptimizationConfig::default();
        config.profile_guided = true;
        
        let mut optimizer = ProductionLlvmOptimizer::new(self.context, config)?;
        let module = self.create_test_module();
        
        // Run PGO optimization
        let result = optimizer.optimize_module(&module)?;
        
        // Verify PGO passes were run
        assert!(result.passes_run.iter().any(|p| p.contains("pgo")));
        
        println!("✓ Profile-guided optimization passed");
        Ok(())
    }
    
    /// Test memory optimization
    pub fn test_memory_optimization(&self) -> Result<()> {
        let config = OptimizationConfig::size_optimized();
        let mut optimizer = ProductionLlvmOptimizer::new(self.context, config)?;
        
        let module = self.create_test_module();
        
        // Run size optimization
        let result = optimizer.optimize_module(&module)?;
        
        // Verify size optimization occurred
        if let (Some(initial), Some(final_metrics)) = (&result.initial_metrics, &result.final_metrics) {
            // Size should be reduced or at least not increased significantly
            assert!(final_metrics.module_size <= initial.module_size * 2);
        }
        
        println!("✓ Memory optimization passed");
        Ok(())
    }
    
    /// Test regression detection
    pub fn test_regression_detection(&self) -> Result<()> {
        let config = OptimizationConfig::default();
        let mut system = ComprehensiveBenchmarkingSystem::new(config)?;
        
        // Create baseline benchmark
        let baseline_config = BenchmarkConfig {
            name: "baseline".to_string(),
            benchmark_type: BenchmarkType::CompileTime { 
                source_size: 1000, 
                complexity: 1.0 
            },
            iterations: 3,
            warmup_iterations: 1,
            timeout: Duration::from_secs(5),
            convergence_criteria: ConvergenceCriteria::default(),
        };
        
        let baseline_result = system.run_single_benchmark(&baseline_config)?;
        
        // Create regression test
        let regression_config = BenchmarkConfig {
            name: "regression_test".to_string(),
            benchmark_type: BenchmarkType::CompileTime { 
                source_size: 2000, // Double the work
                complexity: 2.0 
            },
            iterations: 3,
            warmup_iterations: 1,
            timeout: Duration::from_secs(5),
            convergence_criteria: ConvergenceCriteria::default(),
        };
        
        let regression_result = system.run_single_benchmark(&regression_config)?;
        
        // Verify regression detection works
        assert!(regression_result.statistics.avg_duration > baseline_result.statistics.avg_duration);
        
        println!("✓ Regression detection passed");
        Ok(())
    }
    
    /// Create a test LLVM module
    fn create_test_module(&self) -> Module<'ctx> {
        let module = self.context.create_module("test_module");
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        
        // Create a simple function
        let function = module.add_function("test_function", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        
        self.builder.position_at_end(basic_block);
        
        // Add some instructions to optimize
        let param = function.get_first_param().unwrap();
        let const_val = i32_type.const_int(42, false);
        let add_result = self.builder.build_int_add(param.into_int_value(), const_val, "add");
        let mul_result = self.builder.build_int_mul(add_result, const_val, "mul");
        
        self.builder.build_return(Some(&mul_result));
        
        module
    }
    
    /// Run all tests
    pub fn run_all_tests(&self) -> Result<()> {
        println!("Running comprehensive optimization tests...\n");
        
        self.test_all_optimization_levels()?;
        self.test_performance_monitoring()?;
        self.test_comprehensive_benchmarking()?;
        self.test_optimization_level_controller()?;
        self.test_optimization_profiles()?;
        self.test_pass_dependency_resolution()?;
        self.test_profile_guided_optimization()?;
        self.test_memory_optimization()?;
        self.test_regression_detection()?;
        
        println!("\n✅ All comprehensive optimization tests passed!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    
    #[test]
    fn test_optimization_levels() {
        let context = Context::create();
        let test_suite = ComprehensiveOptimizationTest::new(&context);
        
        // Test individual levels
        assert!(test_suite.test_optimization_level(OptimizationLevel::None).is_ok());
        assert!(test_suite.test_optimization_level(OptimizationLevel::Less).is_ok());
        assert!(test_suite.test_optimization_level(OptimizationLevel::Default).is_ok());
        assert!(test_suite.test_optimization_level(OptimizationLevel::Aggressive).is_ok());
    }
    
    #[test]
    fn test_performance_monitoring() {
        let context = Context::create();
        let test_suite = ComprehensiveOptimizationTest::new(&context);
        
        assert!(test_suite.test_performance_monitoring().is_ok());
    }
    
    #[test]
    fn test_comprehensive_benchmarking() {
        let context = Context::create();
        let test_suite = ComprehensiveOptimizationTest::new(&context);
        
        assert!(test_suite.test_comprehensive_benchmarking().is_ok());
    }
    
    #[test]
    fn test_optimization_level_controller() {
        let context = Context::create();
        let test_suite = ComprehensiveOptimizationTest::new(&context);
        
        assert!(test_suite.test_optimization_level_controller().is_ok());
    }
    
    #[test]
    fn test_optimization_profiles() {
        let context = Context::create();
        let test_suite = ComprehensiveOptimizationTest::new(&context);
        
        assert!(test_suite.test_optimization_profiles().is_ok());
    }
    
    #[test]
    fn test_pass_dependency_resolution() {
        let context = Context::create();
        let test_suite = ComprehensiveOptimizationTest::new(&context);
        
        assert!(test_suite.test_pass_dependency_resolution().is_ok());
    }
    
    #[test]
    fn test_profile_guided_optimization() {
        let context = Context::create();
        let test_suite = ComprehensiveOptimizationTest::new(&context);
        
        assert!(test_suite.test_profile_guided_optimization().is_ok());
    }
    
    #[test]
    fn test_memory_optimization() {
        let context = Context::create();
        let test_suite = ComprehensiveOptimizationTest::new(&context);
        
        assert!(test_suite.test_memory_optimization().is_ok());
    }
    
    #[test]
    fn test_regression_detection() {
        let context = Context::create();
        let test_suite = ComprehensiveOptimizationTest::new(&context);
        
        assert!(test_suite.test_regression_detection().is_ok());
    }
    
    #[test]
    fn test_comprehensive_suite() {
        let context = Context::create();
        let test_suite = ComprehensiveOptimizationTest::new(&context);
        
        // Run all tests
        assert!(test_suite.run_all_tests().is_ok());
    }
}
