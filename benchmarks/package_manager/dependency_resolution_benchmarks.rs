//! Comprehensive benchmarks for dependency resolution performance
//!
//! This module provides benchmarks to measure the performance improvement
//! of the optimized resolver compared to the original O(N²) implementation.

use cursed::package_manager::resolver::{PackageResolver, ResolutionConfig, ConflictResolutionStrategy};
use cursed::package_manager::optimized_resolver::OptimizedPackageResolver;
use cursed::package_manager::registry::{PackageRegistry, RegistryConfig};
use cursed::package_manager::version::{Version, VersionReq};
use std::time::Instant;
use std::collections::HashMap;

/// Benchmark configuration
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub package_count: usize,
    pub max_dependencies: usize,
    pub conflict_rate: f64,
    pub iterations: usize,
}

/// Benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub original_time_ms: u128,
    pub optimized_time_ms: u128,
    pub speedup_factor: f64,
    pub original_memory_mb: usize,
    pub optimized_memory_mb: usize,
    pub memory_improvement: f64,
    pub packages_resolved: usize,
    pub conflicts_resolved: usize,
}

/// Benchmark suite for dependency resolution
pub struct DependencyResolutionBenchmark {
    registry: PackageRegistry,
    test_graphs: Vec<TestGraph>,
}

/// Test dependency graph
#[derive(Debug, Clone)]
pub struct TestGraph {
    pub name: String,
    pub root_packages: Vec<(String, VersionReq)>,
    pub expected_package_count: usize,
    pub complexity: GraphComplexity,
}

/// Graph complexity classification
#[derive(Debug, Clone)]
pub enum GraphComplexity {
    Simple,      // Linear dependency chains
    Moderate,    // Some shared dependencies
    Complex,     // Diamond dependencies, conflicts
    Massive,     // 1000+ packages with deep conflicts
}

impl DependencyResolutionBenchmark {
    /// Create new benchmark suite
    pub fn new() -> cursed::error::Result<Self> {
        let registry = PackageRegistry::new(RegistryConfig::default())?;
        let test_graphs = Self::generate_test_graphs();
        
        Ok(Self {
            registry,
            test_graphs,
        })
    }

    /// Run comprehensive benchmarks
    pub async fn run_benchmarks(&mut self) -> cursed::error::Result<Vec<BenchmarkResult>> {
        let mut results = Vec::new();
        
        for test_graph in &self.test_graphs {
            println!("Running benchmark: {}", test_graph.name);
            
            let result = self.benchmark_graph(test_graph).await?;
            
            println!("  Original: {}ms", result.original_time_ms);
            println!("  Optimized: {}ms", result.optimized_time_ms);
            println!("  Speedup: {:.2}x", result.speedup_factor);
            println!("  Memory improvement: {:.2}%", result.memory_improvement * 100.0);
            println!();
            
            results.push(result);
        }
        
        Ok(results)
    }

    /// Benchmark a specific dependency graph
    async fn benchmark_graph(&mut self, test_graph: &TestGraph) -> cursed::error::Result<BenchmarkResult> {
        const ITERATIONS: usize = 5;
        
        let mut original_times = Vec::new();
        let mut optimized_times = Vec::new();
        let mut packages_resolved = 0;
        let mut conflicts_resolved = 0;

        // Benchmark original resolver
        for _ in 0..ITERATIONS {
            let mut resolver = PackageResolver::new(self.registry.clone());
            let start = Instant::now();
            
            match resolver.resolve_dependencies(
                test_graph.root_packages.clone(),
                ResolutionConfig::default(),
            ).await {
                Ok(result) => {
                    let duration = start.elapsed().as_millis();
                    original_times.push(duration);
                    packages_resolved = result.resolved_packages.len();
                    conflicts_resolved = result.conflicts.len();
                },
                Err(e) => {
                    eprintln!("Original resolver failed: {}", e);
                    original_times.push(u128::MAX); // Mark as failed
                }
            }
        }

        // Benchmark optimized resolver
        for _ in 0..ITERATIONS {
            let mut resolver = OptimizedPackageResolver::new(self.registry.clone());
            let start = Instant::now();
            
            match resolver.resolve_dependencies(
                test_graph.root_packages.clone(),
                ResolutionConfig::default(),
            ).await {
                Ok((result, _metrics)) => {
                    let duration = start.elapsed().as_millis();
                    optimized_times.push(duration);
                    packages_resolved = result.resolved_packages.len();
                    conflicts_resolved = result.conflicts.len();
                },
                Err(e) => {
                    eprintln!("Optimized resolver failed: {}", e);
                    optimized_times.push(u128::MAX); // Mark as failed
                }
            }
        }

        // Calculate averages (excluding failures)
        let original_avg = original_times.iter()
            .filter(|&&t| t != u128::MAX)
            .sum::<u128>() / original_times.len() as u128;
        
        let optimized_avg = optimized_times.iter()
            .filter(|&&t| t != u128::MAX)
            .sum::<u128>() / optimized_times.len() as u128;

        let speedup = if optimized_avg > 0 {
            original_avg as f64 / optimized_avg as f64
        } else {
            0.0
        };

        Ok(BenchmarkResult {
            original_time_ms: original_avg,
            optimized_time_ms: optimized_avg,
            speedup_factor: speedup,
            original_memory_mb: 0, // Memory measurement would require additional tooling
            optimized_memory_mb: 0,
            memory_improvement: 0.0,
            packages_resolved,
            conflicts_resolved,
        })
    }

    /// Generate test dependency graphs of various complexities
    fn generate_test_graphs() -> Vec<TestGraph> {
        vec![
            // Simple linear dependency chain
            TestGraph {
                name: "Linear Chain (50 packages)".to_string(),
                root_packages: vec![
                    ("linear-root".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                ],
                expected_package_count: 50,
                complexity: GraphComplexity::Simple,
            },
            
            // Moderate complexity with shared dependencies
            TestGraph {
                name: "Shared Dependencies (200 packages)".to_string(),
                root_packages: vec![
                    ("shared-root-1".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                    ("shared-root-2".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                    ("shared-root-3".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                ],
                expected_package_count: 200,
                complexity: GraphComplexity::Moderate,
            },
            
            // Complex diamond dependencies with conflicts
            TestGraph {
                name: "Diamond Dependencies (500 packages)".to_string(),
                root_packages: vec![
                    ("diamond-root".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                ],
                expected_package_count: 500,
                complexity: GraphComplexity::Complex,
            },
            
            // Massive graph with 1000+ packages
            TestGraph {
                name: "Massive Graph (1000+ packages)".to_string(),
                root_packages: vec![
                    ("massive-root".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                ],
                expected_package_count: 1000,
                complexity: GraphComplexity::Massive,
            },
            
            // Real-world scenario: web framework with many dependencies
            TestGraph {
                name: "Web Framework Dependencies".to_string(),
                root_packages: vec![
                    ("web-framework".to_string(), VersionReq::parse("^2.0.0").unwrap()),
                    ("database-orm".to_string(), VersionReq::parse("^1.5.0").unwrap()),
                    ("template-engine".to_string(), VersionReq::parse("^3.0.0").unwrap()),
                    ("auth-middleware".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                ],
                expected_package_count: 150,
                complexity: GraphComplexity::Complex,
            },
            
            // Stress test: conflicting version requirements
            TestGraph {
                name: "Version Conflicts Stress Test".to_string(),
                root_packages: vec![
                    ("conflict-root-1".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                    ("conflict-root-2".to_string(), VersionReq::parse("^2.0.0").unwrap()),
                    ("conflict-root-3".to_string(), VersionReq::parse("^3.0.0").unwrap()),
                ],
                expected_package_count: 300,
                complexity: GraphComplexity::Complex,
            },
        ]
    }

    /// Generate detailed performance report
    pub fn generate_report(&self, results: &[BenchmarkResult]) -> String {
        let mut report = String::new();
        
        report.push_str("# Dependency Resolution Performance Benchmark Report\n\n");
        report.push_str("## Summary\n\n");
        
        let avg_speedup = results.iter().map(|r| r.speedup_factor).sum::<f64>() / results.len() as f64;
        let total_packages = results.iter().map(|r| r.packages_resolved).sum::<usize>();
        
        report.push_str(&format!("- Average speedup: {:.2}x\n", avg_speedup));
        report.push_str(&format!("- Total packages resolved: {}\n", total_packages));
        report.push_str(&format!("- Benchmark scenarios: {}\n\n", results.len()));
        
        report.push_str("## Detailed Results\n\n");
        report.push_str("| Scenario | Original (ms) | Optimized (ms) | Speedup | Packages | Conflicts |\n");
        report.push_str("|----------|---------------|----------------|---------|----------|----------|\n");
        
        for (i, result) in results.iter().enumerate() {
            let scenario_name = &self.test_graphs[i].name;
            report.push_str(&format!(
                "| {} | {} | {} | {:.2}x | {} | {} |\n",
                scenario_name,
                result.original_time_ms,
                result.optimized_time_ms,
                result.speedup_factor,
                result.packages_resolved,
                result.conflicts_resolved
            ));
        }
        
        report.push_str("\n## Analysis\n\n");
        
        if avg_speedup >= 4.0 {
            report.push_str("✅ **Performance target achieved**: The optimized resolver meets the 4x speedup goal.\n\n");
        } else {
            report.push_str("❌ **Performance target not met**: Further optimization is needed to achieve 4x speedup.\n\n");
        }
        
        report.push_str("### Key Performance Improvements\n\n");
        report.push_str("- **SAT-based constraint solving**: Eliminates O(N²) complexity\n");
        report.push_str("- **Aggressive caching**: Reduces redundant version resolution\n");
        report.push_str("- **Conflict analysis**: Learns from conflicts to avoid repeated work\n");
        report.push_str("- **Unit propagation**: Optimizes constraint satisfaction\n\n");
        
        report
    }
}

/// Stress test for large dependency graphs
pub struct LargeGraphStressTest;

impl LargeGraphStressTest {
    /// Generate a stress test with 1000+ packages
    pub async fn run_1k_package_stress_test() -> cursed::error::Result<BenchmarkResult> {
        println!("Running 1000-package stress test...");
        
        let registry = PackageRegistry::new(RegistryConfig::default())?;
        
        // Generate a complex graph with 1000+ packages
        let root_packages = Self::generate_1k_package_graph();
        
        // Test original resolver
        let mut original_resolver = PackageResolver::new(registry.clone());
        let start = Instant::now();
        
        let original_result = original_resolver.resolve_dependencies(
            root_packages.clone(),
            ResolutionConfig {
                max_depth: 200,
                conflict_strategy: ConflictResolutionStrategy::ChooseHighest,
                ..Default::default()
            },
        ).await;
        
        let original_time = start.elapsed().as_millis();
        
        // Test optimized resolver
        let mut optimized_resolver = OptimizedPackageResolver::new(registry);
        let start = Instant::now();
        
        let optimized_result = optimized_resolver.resolve_dependencies(
            root_packages,
            ResolutionConfig {
                max_depth: 200,
                conflict_strategy: ConflictResolutionStrategy::ChooseHighest,
                ..Default::default()
            },
        ).await;
        
        let optimized_time = start.elapsed().as_millis();
        
        let speedup = if optimized_time > 0 {
            original_time as f64 / optimized_time as f64
        } else {
            0.0
        };
        
        let packages_resolved = match (&original_result, &optimized_result) {
            (Ok(orig), Ok((opt, _))) => orig.resolved_packages.len().max(opt.resolved_packages.len()),
            (Ok(orig), _) => orig.resolved_packages.len(),
            (_, Ok((opt, _))) => opt.resolved_packages.len(),
            _ => 0,
        };
        
        println!("1000-package stress test completed:");
        println!("  Original: {}ms", original_time);
        println!("  Optimized: {}ms", optimized_time);
        println!("  Speedup: {:.2}x", speedup);
        println!("  Packages resolved: {}", packages_resolved);
        
        Ok(BenchmarkResult {
            original_time_ms: original_time,
            optimized_time_ms: optimized_time,
            speedup_factor: speedup,
            original_memory_mb: 0,
            optimized_memory_mb: 0,
            memory_improvement: 0.0,
            packages_resolved,
            conflicts_resolved: 0,
        })
    }
    
    /// Generate a complex dependency graph with 1000+ packages
    fn generate_1k_package_graph() -> Vec<(String, VersionReq)> {
        vec![
            ("mega-framework".to_string(), VersionReq::parse("^1.0.0").unwrap()),
            ("data-processing".to_string(), VersionReq::parse("^2.0.0").unwrap()),
            ("ml-toolkit".to_string(), VersionReq::parse("^1.5.0").unwrap()),
            ("crypto-suite".to_string(), VersionReq::parse("^3.0.0").unwrap()),
            ("web-stack".to_string(), VersionReq::parse("^2.5.0").unwrap()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_benchmark_suite() {
        let mut benchmark = DependencyResolutionBenchmark::new().unwrap();
        let results = benchmark.run_benchmarks().await.unwrap();
        
        // Verify benchmarks completed
        assert!(!results.is_empty());
        
        // Check that some speedup was achieved
        let avg_speedup = results.iter().map(|r| r.speedup_factor).sum::<f64>() / results.len() as f64;
        assert!(avg_speedup > 1.0);
        
        // Generate report
        let report = benchmark.generate_report(&results);
        assert!(report.contains("Performance Benchmark Report"));
        
        println!("{}", report);
    }

    #[tokio::test]
    async fn test_1k_package_stress_test() {
        let result = LargeGraphStressTest::run_1k_package_stress_test().await.unwrap();
        
        // Verify stress test completed
        assert!(result.original_time_ms > 0 || result.optimized_time_ms > 0);
        assert!(result.packages_resolved > 0);
        
        // Check for performance improvement
        if result.original_time_ms > 0 && result.optimized_time_ms > 0 {
            assert!(result.speedup_factor > 1.0);
        }
    }
}
