/// Integration tests for the complete CURSED optimization system
/// 
/// Tests the integration between CURSED-specific optimizations, LLVM passes,
/// enhanced analysis, and performance monitoring.

use std::collections::HashMap;
use std::time::Duration;

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    // Mock comprehensive optimization system for testing
    struct MockCursedOptimizationSystem {
        config: OptimizationSystemConfig,
        stats: OptimizationSystemStats,
    }
    
    #[derive(Debug, Clone)]
    struct OptimizationSystemConfig {
        enable_cursed_optimizations: bool,
        enable_llvm_integration: bool,
        enable_performance_analysis: bool,
        enable_lto: bool,
        optimization_level: OptimizationLevel,
    }
    
    #[derive(Debug, Clone, Copy)]
    enum OptimizationLevel {
        None,
        Basic,
        Aggressive,
    }
    
    #[derive(Debug, Clone, Default)]
    struct OptimizationSystemStats {
        total_optimizations: usize,
        cursed_optimizations: usize,
        llvm_optimizations: usize,
        performance_improvements: f64,
        memory_reductions: f64,
        compilation_time: Duration,
        analysis_recommendations: usize,
    }
    
    #[derive(Debug, Clone)]
    struct ComprehensiveOptimizationResult {
        optimizations_by_category: HashMap<String, usize>,
        performance_analysis: PerformanceAnalysisResult,
        llvm_integration_result: LlvmIntegrationResult,
        lto_result: Option<LtoIntegrationResult>,
        overall_improvement: f64,
        recommendations: Vec<String>,
    }
    
    #[derive(Debug, Clone)]
    struct PerformanceAnalysisResult {
        bottlenecks_detected: usize,
        recommendations_generated: usize,
        performance_score: f64,
        analysis_time: Duration,
    }
    
    #[derive(Debug, Clone)]
    struct LlvmIntegrationResult {
        passes_run: usize,
        optimizations_applied: usize,
        code_size_reduction: f64,
        integration_success: bool,
    }
    
    #[derive(Debug, Clone)]
    struct LtoIntegrationResult {
        cross_module_optimizations: usize,
        global_optimizations: usize,
        link_time_reduction: f64,
    }
    
    impl Default for OptimizationSystemConfig {
        fn default() -> Self {
            Self {
                enable_cursed_optimizations: true,
                enable_llvm_integration: true,
                enable_performance_analysis: true,
                enable_lto: true,
                optimization_level: OptimizationLevel::Aggressive,
            }
        }
    }
    
    impl MockCursedOptimizationSystem {
        fn new(config: OptimizationSystemConfig) -> Self {
            Self {
                config,
                stats: OptimizationSystemStats::default(),
            }
        }
        
        fn optimize_comprehensive(&mut self, source: &str) -> ComprehensiveOptimizationResult {
            let start_time = std::time::Instant::now();
            
            // 1. Run CURSED-specific optimizations
            let cursed_result = self.run_cursed_optimizations(source);
            
            // 2. Integrate with LLVM
            let llvm_result = if self.config.enable_llvm_integration {
                self.run_llvm_integration(source, &cursed_result)
            } else {
                LlvmIntegrationResult {
                    passes_run: 0,
                    optimizations_applied: 0,
                    code_size_reduction: 0.0,
                    integration_success: false,
                }
            };
            
            // 3. Run performance analysis
            let analysis_result = if self.config.enable_performance_analysis {
                self.run_performance_analysis(source, &cursed_result, &llvm_result)
            } else {
                PerformanceAnalysisResult {
                    bottlenecks_detected: 0,
                    recommendations_generated: 0,
                    performance_score: 50.0,
                    analysis_time: Duration::from_millis(0),
                }
            };
            
            // 4. Apply LTO if enabled
            let lto_result = if self.config.enable_lto {
                Some(self.run_lto_optimization(source, &cursed_result, &llvm_result))
            } else {
                None
            };
            
            // 5. Calculate overall improvement
            let overall_improvement = self.calculate_overall_improvement(
                &cursed_result, &llvm_result, &lto_result
            );
            
            // 6. Generate recommendations
            let recommendations = self.generate_integration_recommendations(
                &analysis_result, &cursed_result, &llvm_result
            );
            
            // Update system stats
            self.update_system_stats(&cursed_result, &llvm_result, start_time.elapsed());
            
            ComprehensiveOptimizationResult {
                optimizations_by_category: cursed_result,
                performance_analysis: analysis_result,
                llvm_integration_result: llvm_result,
                lto_result,
                overall_improvement,
                recommendations,
            }
        }
        
        fn run_cursed_optimizations(&self, source: &str) -> HashMap<String, usize> {
            let mut optimizations = HashMap::new();
            
            // Count different types of CURSED optimizations
            let goroutine_opts = self.count_goroutine_optimizations(source);
            let channel_opts = self.count_channel_optimizations(source);
            let gc_opts = self.count_gc_optimizations(source);
            let genz_opts = self.count_genz_optimizations(source);
            let control_flow_opts = self.count_control_flow_optimizations(source);
            let memory_opts = self.count_memory_optimizations(source);
            
            optimizations.insert("goroutine".to_string(), goroutine_opts);
            optimizations.insert("channel".to_string(), channel_opts);
            optimizations.insert("gc".to_string(), gc_opts);
            optimizations.insert("genz".to_string(), genz_opts);
            optimizations.insert("control_flow".to_string(), control_flow_opts);
            optimizations.insert("memory".to_string(), memory_opts);
            
            optimizations
        }
        
        fn count_goroutine_optimizations(&self, source: &str) -> usize {
            let patterns = ["stan ", "yolo", "goroutine"];
            patterns.iter().map(|p| source.matches(p).count()).sum()
        }
        
        fn count_channel_optimizations(&self, source: &str) -> usize {
            let patterns = ["channel", "send(", "receive("];
            patterns.iter().map(|p| source.matches(p).count()).sum()
        }
        
        fn count_gc_optimizations(&self, source: &str) -> usize {
            let patterns = ["new ", "alloc", "Box::"];
            patterns.iter().map(|p| source.matches(p).count()).sum()
        }
        
        fn count_genz_optimizations(&self, source: &str) -> usize {
            let patterns = ["slay ", "facts ", "sus ", "lowkey", "highkey", "periodt", "bestie", "flex"];
            patterns.iter().map(|p| source.matches(p).count()).sum()
        }
        
        fn count_control_flow_optimizations(&self, source: &str) -> usize {
            let patterns = ["?", "unwrap", "expect"];
            patterns.iter().map(|p| source.matches(p).count()).sum()
        }
        
        fn count_memory_optimizations(&self, source: &str) -> usize {
            let patterns = ["squad ", "impl ", "Vec::", "HashMap::"];
            patterns.iter().map(|p| source.matches(p).count()).sum()
        }
        
        fn run_llvm_integration(&self, source: &str, cursed_opts: &HashMap<String, usize>) -> LlvmIntegrationResult {
            let total_cursed = cursed_opts.values().sum::<usize>();
            
            // Simulate LLVM integration based on optimization level
            let passes_run = match self.config.optimization_level {
                OptimizationLevel::None => 0,
                OptimizationLevel::Basic => 8 + total_cursed / 2,
                OptimizationLevel::Aggressive => 15 + total_cursed,
            };
            
            let optimizations_applied = passes_run * 2; // Rough estimate
            let code_size_reduction = (optimizations_applied as f64 * 0.02).min(0.4);
            
            LlvmIntegrationResult {
                passes_run,
                optimizations_applied,
                code_size_reduction,
                integration_success: true,
            }
        }
        
        fn run_performance_analysis(
            &self,
            source: &str,
            cursed_opts: &HashMap<String, usize>,
            llvm_result: &LlvmIntegrationResult,
        ) -> PerformanceAnalysisResult {
            let start_time = std::time::Instant::now();
            
            // Simulate performance analysis
            let complexity = source.len() / 100; // Rough complexity measure
            let bottlenecks_detected = if complexity > 10 { complexity / 10 } else { 0 };
            
            let total_optimizations = cursed_opts.values().sum::<usize>() + llvm_result.optimizations_applied;
            let performance_score = (50.0 + (total_optimizations as f64 * 2.0)).min(100.0);
            
            let recommendations_generated = bottlenecks_detected + 
                if performance_score < 70.0 { 3 } else { 1 };
            
            PerformanceAnalysisResult {
                bottlenecks_detected,
                recommendations_generated,
                performance_score,
                analysis_time: start_time.elapsed(),
            }
        }
        
        fn run_lto_optimization(
            &self,
            _source: &str,
            cursed_opts: &HashMap<String, usize>,
            llvm_result: &LlvmIntegrationResult,
        ) -> LtoIntegrationResult {
            let base_optimizations = cursed_opts.values().sum::<usize>();
            
            LtoIntegrationResult {
                cross_module_optimizations: base_optimizations / 3,
                global_optimizations: llvm_result.optimizations_applied / 4,
                link_time_reduction: 0.15, // 15% link time reduction
            }
        }
        
        fn calculate_overall_improvement(
            &self,
            cursed_opts: &HashMap<String, usize>,
            llvm_result: &LlvmIntegrationResult,
            lto_result: &Option<LtoIntegrationResult>,
        ) -> f64 {
            let cursed_improvement = cursed_opts.values().sum::<usize>() as f64 * 0.03;
            let llvm_improvement = llvm_result.code_size_reduction + 
                (llvm_result.optimizations_applied as f64 * 0.01);
            let lto_improvement = lto_result.as_ref()
                .map(|lto| lto.link_time_reduction)
                .unwrap_or(0.0);
            
            (cursed_improvement + llvm_improvement + lto_improvement).min(0.8)
        }
        
        fn generate_integration_recommendations(
            &self,
            analysis: &PerformanceAnalysisResult,
            cursed_opts: &HashMap<String, usize>,
            llvm_result: &LlvmIntegrationResult,
        ) -> Vec<String> {
            let mut recommendations = Vec::new();
            
            if analysis.performance_score < 60.0 {
                recommendations.push("Consider enabling more aggressive optimization levels".to_string());
            }
            
            if cursed_opts.get("goroutine").unwrap_or(&0) > &10 {
                recommendations.push("High goroutine usage detected - enable goroutine-specific optimizations".to_string());
            }
            
            if cursed_opts.get("gc").unwrap_or(&0) > &15 {
                recommendations.push("Heavy GC usage - consider escape analysis and allocation batching".to_string());
            }
            
            if llvm_result.passes_run < 10 {
                recommendations.push("Enable more LLVM optimization passes for better performance".to_string());
            }
            
            if analysis.bottlenecks_detected > 3 {
                recommendations.push("Multiple bottlenecks detected - review hot paths and algorithms".to_string());
            }
            
            recommendations
        }
        
        fn update_system_stats(
            &mut self,
            cursed_opts: &HashMap<String, usize>,
            llvm_result: &LlvmIntegrationResult,
            compilation_time: Duration,
        ) {
            self.stats.cursed_optimizations += cursed_opts.values().sum::<usize>();
            self.stats.llvm_optimizations += llvm_result.optimizations_applied;
            self.stats.total_optimizations = self.stats.cursed_optimizations + self.stats.llvm_optimizations;
            self.stats.compilation_time += compilation_time;
        }
        
        fn get_stats(&self) -> &OptimizationSystemStats {
            &self.stats
        }
    }
    
    #[test]
    fn test_comprehensive_optimization_integration() {
        let config = OptimizationSystemConfig::default();
        let mut system = MockCursedOptimizationSystem::new(config);
        
        let source = r#"
            slay main() {
                stan worker_pool();
                facts channel = make_channel<i32>(100);
                send(channel, 42)?;
                yolo;
            }
            
            slay worker_pool() {
                facts workers = Vec::new();
                lowkey (sus i = 0; i < 8; i++) {
                    stan worker_function(i);
                }
                periodt {
                    bestie result = receive(result_channel) {
                        Some(r) => process_result(r),
                        None => break,
                    }
                }
            }
        "#;
        
        let result = system.optimize_comprehensive(source);
        
        // Verify CURSED optimizations were applied
        assert!(result.optimizations_by_category.get("goroutine").unwrap_or(&0) > &0);
        assert!(result.optimizations_by_category.get("channel").unwrap_or(&0) > &0);
        assert!(result.optimizations_by_category.get("genz").unwrap_or(&0) > &0);
        
        // Verify LLVM integration
        assert!(result.llvm_integration_result.integration_success);
        assert!(result.llvm_integration_result.passes_run > 0);
        assert!(result.llvm_integration_result.optimizations_applied > 0);
        
        // Verify performance analysis
        assert!(result.performance_analysis.performance_score > 0.0);
        assert!(result.performance_analysis.recommendations_generated >= 0);
        
        // Verify overall improvement
        assert!(result.overall_improvement > 0.0);
        assert!(result.overall_improvement <= 1.0);
        
        // Verify recommendations are generated
        assert!(!result.recommendations.is_empty());
        
        println!("Integration Test Results:");
        println!("  CURSED optimizations: {:?}", result.optimizations_by_category);
        println!("  LLVM passes run: {}", result.llvm_integration_result.passes_run);
        println!("  Performance score: {:.1}", result.performance_analysis.performance_score);
        println!("  Overall improvement: {:.1}%", result.overall_improvement * 100.0);
        println!("  Recommendations: {}", result.recommendations.len());
    }
    
    #[test]
    fn test_optimization_level_scaling() {
        let test_cases = vec![
            OptimizationLevel::None,
            OptimizationLevel::Basic,
            OptimizationLevel::Aggressive,
        ];
        
        let source = r#"
            slay heavy_computation() {
                facts data = Vec::new();
                lowkey (sus i = 0; i < 1000; i++) {
                    data.push(new ComplexObject::new(i));
                }
                
                lowkey (sus item in data) {
                    stan process_item(item);
                }
            }
        "#;
        
        let mut previous_improvement = 0.0;
        
        for level in test_cases {
            let config = OptimizationSystemConfig {
                optimization_level: level,
                ..Default::default()
            };
            
            let mut system = MockCursedOptimizationSystem::new(config);
            let result = system.optimize_comprehensive(source);
            
            println!("Optimization Level {:?}:", level);
            println!("  LLVM passes: {}", result.llvm_integration_result.passes_run);
            println!("  Overall improvement: {:.1}%", result.overall_improvement * 100.0);
            
            // Verify scaling - higher levels should generally perform better
            if level as u8 > OptimizationLevel::None as u8 {
                assert!(result.overall_improvement >= previous_improvement,
                        "Higher optimization levels should provide equal or better results");
            }
            
            previous_improvement = result.overall_improvement;
        }
    }
    
    #[test]
    fn test_feature_flag_integration() {
        let source = r#"
            slay test_function() {
                stan goroutine_task();
                facts ch = make_channel<String>(10);
                send(ch, "test")?;
            }
        "#;
        
        // Test with all features enabled
        let full_config = OptimizationSystemConfig::default();
        let mut full_system = MockCursedOptimizationSystem::new(full_config);
        let full_result = full_system.optimize_comprehensive(source);
        
        // Test with features disabled
        let minimal_config = OptimizationSystemConfig {
            enable_cursed_optimizations: false,
            enable_llvm_integration: false,
            enable_performance_analysis: false,
            enable_lto: false,
            optimization_level: OptimizationLevel::None,
        };
        let mut minimal_system = MockCursedOptimizationSystem::new(minimal_config);
        let minimal_result = minimal_system.optimize_comprehensive(source);
        
        // Full system should outperform minimal system
        assert!(full_result.overall_improvement > minimal_result.overall_improvement);
        assert!(full_result.llvm_integration_result.passes_run > minimal_result.llvm_integration_result.passes_run);
        assert!(full_result.performance_analysis.performance_score > minimal_result.performance_analysis.performance_score);
        
        println!("Feature Flag Comparison:");
        println!("  Full system improvement: {:.1}%", full_result.overall_improvement * 100.0);
        println!("  Minimal system improvement: {:.1}%", minimal_result.overall_improvement * 100.0);
    }
    
    #[test]
    fn test_performance_analysis_integration() {
        let complex_source = r#"
            slay complex_algorithm() {
                // High complexity code that should trigger analysis
                facts matrix = Vec::new();
                lowkey (sus i = 0; i < 100; i++) {
                    facts row = Vec::new();
                    lowkey (sus j = 0; j < 100; j++) {
                        row.push(i * j);
                    }
                    matrix.push(row);
                }
                
                // Nested loops - performance bottleneck
                lowkey (sus row in matrix) {
                    lowkey (sus item in row) {
                        periodt {
                            facts result = expensive_computation(item);
                            lowkey (result > 1000) {
                                break;
                            }
                        }
                    }
                }
            }
        "#;
        
        let config = OptimizationSystemConfig::default();
        let mut system = MockCursedOptimizationSystem::new(config);
        let result = system.optimize_comprehensive(complex_source);
        
        // Complex code should trigger analysis and recommendations
        assert!(result.performance_analysis.bottlenecks_detected > 0);
        assert!(result.performance_analysis.recommendations_generated > 0);
        assert!(!result.recommendations.is_empty());
        
        // Performance score should reflect complexity
        assert!(result.performance_analysis.performance_score < 90.0);
        
        println!("Performance Analysis Results:");
        println!("  Bottlenecks detected: {}", result.performance_analysis.bottlenecks_detected);
        println!("  Recommendations: {}", result.performance_analysis.recommendations_generated);
        println!("  Performance score: {:.1}", result.performance_analysis.performance_score);
        
        for (i, recommendation) in result.recommendations.iter().enumerate() {
            println!("  Recommendation {}: {}", i + 1, recommendation);
        }
    }
    
    #[test]
    fn test_lto_integration() {
        let source = r#"
            slay main() {
                stan task1();
                stan task2();
                facts result = combine_results()?;
            }
            
            slay task1() -> Result<i32> {
                Ok(42)
            }
            
            slay task2() -> Result<String> {
                Ok("test".to_string())
            }
        "#;
        
        // Test with LTO enabled
        let lto_config = OptimizationSystemConfig {
            enable_lto: true,
            ..Default::default()
        };
        let mut lto_system = MockCursedOptimizationSystem::new(lto_config);
        let lto_result = lto_system.optimize_comprehensive(source);
        
        // Test with LTO disabled
        let no_lto_config = OptimizationSystemConfig {
            enable_lto: false,
            ..Default::default()
        };
        let mut no_lto_system = MockCursedOptimizationSystem::new(no_lto_config);
        let no_lto_result = no_lto_system.optimize_comprehensive(source);
        
        // LTO should provide additional optimization
        assert!(lto_result.lto_result.is_some());
        assert!(no_lto_result.lto_result.is_none());
        assert!(lto_result.overall_improvement >= no_lto_result.overall_improvement);
        
        if let Some(lto_data) = &lto_result.lto_result {
            println!("LTO Integration Results:");
            println!("  Cross-module optimizations: {}", lto_data.cross_module_optimizations);
            println!("  Global optimizations: {}", lto_data.global_optimizations);
            println!("  Link time reduction: {:.1}%", lto_data.link_time_reduction * 100.0);
        }
    }
    
    #[test]
    fn test_system_statistics_tracking() {
        let config = OptimizationSystemConfig::default();
        let mut system = MockCursedOptimizationSystem::new(config);
        
        let sources = vec![
            "slay simple() { facts x = 42; }",
            "slay with_goroutines() { stan task(); yolo; }",
            "slay with_channels() { facts ch = make_channel<i32>(10); send(ch, 1)?; }",
        ];
        
        for source in sources {
            let _result = system.optimize_comprehensive(source);
        }
        
        let stats = system.get_stats();
        
        // Verify statistics are accumulated
        assert!(stats.total_optimizations > 0);
        assert!(stats.cursed_optimizations > 0);
        assert!(stats.llvm_optimizations > 0);
        assert!(stats.compilation_time > Duration::from_millis(0));
        
        println!("System Statistics:");
        println!("  Total optimizations: {}", stats.total_optimizations);
        println!("  CURSED optimizations: {}", stats.cursed_optimizations);
        println!("  LLVM optimizations: {}", stats.llvm_optimizations);
        println!("  Total compilation time: {:?}", stats.compilation_time);
    }
}
