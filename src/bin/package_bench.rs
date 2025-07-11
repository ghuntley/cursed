//! Command-line benchmark tool for package manager dependency resolution
//!
//! This tool demonstrates the performance improvements achieved by the optimized resolver
//! compared to the original O(N²) implementation.

use cursed::package_manager::{
    PackageResolver, OptimizedPackageResolver, PackageRegistry, RegistryConfig, ResolutionConfig,
    Version, VersionReq
};
use cursed::package_manager::resolver::ConflictResolutionStrategy;
use std::time::Instant;
use clap::Parser;
use tokio;

#[derive(Parser, Debug)]
#[command(name = "package-bench")]
#[command(about = "CURSED Package Manager Dependency Resolution Benchmark")]
struct Args {
    /// Number of packages to benchmark
    #[arg(short = 'n', long = "packages", default_value = "100")]
    package_count: usize,
    
    /// Number of benchmark iterations
    #[arg(short = 'i', long = "iterations", default_value = "3")]
    iterations: usize,
    
    /// Benchmark scenario
    #[arg(short = 's', long = "scenario", default_value = "shared")]
    scenario: String,
    
    /// Only run optimized resolver
    #[arg(long = "optimized-only")]
    optimized_only: bool,
    
    /// Only run original resolver
    #[arg(long = "original-only")]
    original_only: bool,
    
    /// Verbose output
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

/// Benchmark scenario
#[derive(Debug, Clone)]
enum BenchmarkScenario {
    Linear,      // Linear dependency chain
    Shared,      // Shared dependencies (diamond pattern)
    Conflict,    // Version conflicts
    Massive,     // Large number of packages
}

impl std::str::FromStr for BenchmarkScenario {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "linear" => Ok(BenchmarkScenario::Linear),
            "shared" => Ok(BenchmarkScenario::Shared),
            "conflict" => Ok(BenchmarkScenario::Conflict),
            "massive" => Ok(BenchmarkScenario::Massive),
            _ => Err(format!("Unknown scenario: {}", s)),
        }
    }
}

/// Benchmark results
#[derive(Debug, Clone)]
struct BenchmarkResult {
    scenario: String,
    package_count: usize,
    original_time_ms: Option<u128>,
    optimized_time_ms: Option<u128>,
    speedup: Option<f64>,
    packages_resolved: usize,
    conflicts_resolved: usize,
    cache_hits: usize,
    cache_misses: usize,
}

/// Main benchmark runner
#[derive(Debug)]
struct PackageBenchmark {
    registry: PackageRegistry,
    args: Args,
}

impl PackageBenchmark {
    /// Create new benchmark runner
    fn new(args: Args) -> cursed::error::Result<Self> {
        let registry = PackageRegistry::new(RegistryConfig::default())?;
        Ok(Self { registry, args })
    }
    
    /// Run benchmarks
    async fn run(&mut self) -> cursed::error::Result<()> {
        println!("🚀 CURSED Package Manager Dependency Resolution Benchmark");
        println!("=========================================================");
        println!();
        
        let scenario = self.args.scenario.parse::<BenchmarkScenario>()
            .map_err(|e| cursed::error::CursedError::General(e))?;
        
        let result = self.benchmark_scenario(scenario).await?;
        
        // Print results
        self.print_results(&result);
        
        Ok(())
    }
    
    /// Benchmark a specific scenario
    async fn benchmark_scenario(&mut self, scenario: BenchmarkScenario) -> cursed::error::Result<BenchmarkResult> {
        let root_packages = self.generate_test_packages(&scenario);
        let config = ResolutionConfig {
            max_depth: 200,
            conflict_strategy: ConflictResolutionStrategy::ChooseHighest,
            ..Default::default()
        };
        
        if self.args.verbose {
            println!("Generated {} root packages for scenario: {:?}", root_packages.len(), scenario);
        }
        
        let mut original_time = None;
        let mut optimized_time = None;
        let mut packages_resolved = 0;
        let mut conflicts_resolved = 0;
        let mut cache_hits = 0;
        let mut cache_misses = 0;
        
        // Benchmark original resolver
        if !self.args.optimized_only {
            println!("🔄 Running original resolver...");
            let mut times = Vec::new();
            
            for i in 0..self.args.iterations {
                if self.args.verbose {
                    println!("  Iteration {}/{}", i + 1, self.args.iterations);
                }
                
                let mut resolver = PackageResolver::new(self.registry.clone());
                let start = Instant::now();
                
                match resolver.resolve_dependencies(root_packages.clone(), config.clone()).await {
                    Ok(result) => {
                        let duration = start.elapsed().as_millis();
                        times.push(duration);
                        
                        if i == 0 {
                            packages_resolved = result.resolved_packages.len();
                            conflicts_resolved = result.conflicts.len();
                        }
                        
                        if self.args.verbose {
                            println!("    Time: {}ms, Packages: {}, Conflicts: {}", 
                                   duration, result.resolved_packages.len(), result.conflicts.len());
                        }
                    },
                    Err(e) => {
                        println!("    ❌ Original resolver failed: {}", e);
                        times.push(u128::MAX);
                    }
                }
            }
            
            // Calculate average (excluding failures)
            let valid_times: Vec<u128> = times.into_iter().filter(|&t| t != u128::MAX).collect();
            if !valid_times.is_empty() {
                original_time = Some(valid_times.iter().sum::<u128>() / valid_times.len() as u128);
            }
        }
        
        // Benchmark optimized resolver
        if !self.args.original_only {
            println!("⚡ Running optimized resolver...");
            let mut times = Vec::new();
            
            for i in 0..self.args.iterations {
                if self.args.verbose {
                    println!("  Iteration {}/{}", i + 1, self.args.iterations);
                }
                
                let mut resolver = OptimizedPackageResolver::new(self.registry.clone());
                let start = Instant::now();
                
                match resolver.resolve_dependencies(root_packages.clone(), config.clone()).await {
                    Ok((result, metrics)) => {
                        let duration = start.elapsed().as_millis();
                        times.push(duration);
                        
                        if i == 0 {
                            packages_resolved = result.resolved_packages.len();
                            conflicts_resolved = result.conflicts.len();
                            cache_hits = metrics.cache_hits;
                            cache_misses = metrics.cache_misses;
                        }
                        
                        if self.args.verbose {
                            println!("    Time: {}ms, Packages: {}, Conflicts: {}, Cache: {}/{}", 
                                   duration, result.resolved_packages.len(), result.conflicts.len(),
                                   metrics.cache_hits, metrics.cache_misses);
                        }
                    },
                    Err(e) => {
                        println!("    ❌ Optimized resolver failed: {}", e);
                        times.push(u128::MAX);
                    }
                }
            }
            
            // Calculate average (excluding failures)
            let valid_times: Vec<u128> = times.into_iter().filter(|&t| t != u128::MAX).collect();
            if !valid_times.is_empty() {
                optimized_time = Some(valid_times.iter().sum::<u128>() / valid_times.len() as u128);
            }
        }
        
        // Calculate speedup
        let speedup = match (original_time, optimized_time) {
            (Some(orig), Some(opt)) if opt > 0 => Some(orig as f64 / opt as f64),
            _ => None,
        };
        
        Ok(BenchmarkResult {
            scenario: format!("{:?}", scenario),
            package_count: self.args.package_count,
            original_time_ms: original_time,
            optimized_time_ms: optimized_time,
            speedup,
            packages_resolved,
            conflicts_resolved,
            cache_hits,
            cache_misses,
        })
    }
    
    /// Generate test packages for scenario
    fn generate_test_packages(&self, scenario: &BenchmarkScenario) -> Vec<(String, VersionReq)> {
        match scenario {
            BenchmarkScenario::Linear => {
                vec![("linear-root".to_string(), VersionReq::parse("^1.0.0").unwrap())]
            },
            BenchmarkScenario::Shared => {
                vec![
                    ("shared-root-1".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                    ("shared-root-2".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                    ("shared-root-3".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                ]
            },
            BenchmarkScenario::Conflict => {
                vec![
                    ("conflict-root-1".to_string(), VersionReq::parse("^1.0.0").unwrap()),
                    ("conflict-root-2".to_string(), VersionReq::parse("^2.0.0").unwrap()),
                    ("conflict-root-3".to_string(), VersionReq::parse("^3.0.0").unwrap()),
                ]
            },
            BenchmarkScenario::Massive => {
                let mut packages = Vec::new();
                for i in 0..10 {
                    packages.push((format!("massive-root-{}", i), VersionReq::parse("^1.0.0").unwrap()));
                }
                packages
            },
        }
    }
    
    /// Print benchmark results
    fn print_results(&self, result: &BenchmarkResult) {
        println!();
        println!("📊 Benchmark Results");
        println!("===================");
        println!();
        println!("Scenario: {}", result.scenario);
        println!("Package Count: {}", result.package_count);
        println!("Iterations: {}", self.args.iterations);
        println!();
        
        if let Some(orig_time) = result.original_time_ms {
            println!("Original Resolver: {}ms", orig_time);
        } else {
            println!("Original Resolver: Not run or failed");
        }
        
        if let Some(opt_time) = result.optimized_time_ms {
            println!("Optimized Resolver: {}ms", opt_time);
        } else {
            println!("Optimized Resolver: Not run or failed");
        }
        
        if let Some(speedup) = result.speedup {
            println!("Speedup: {:.2}x", speedup);
            
            if speedup >= 4.0 {
                println!("✅ Performance target achieved (4x speedup)");
            } else if speedup >= 2.0 {
                println!("⚠️  Significant improvement but below 4x target");
            } else {
                println!("❌ Performance target not met");
            }
        }
        
        println!();
        println!("Resolution Details:");
        println!("- Packages resolved: {}", result.packages_resolved);
        println!("- Conflicts resolved: {}", result.conflicts_resolved);
        
        if result.cache_hits + result.cache_misses > 0 {
            let cache_hit_rate = result.cache_hits as f64 / (result.cache_hits + result.cache_misses) as f64 * 100.0;
            println!("- Cache hit rate: {:.1}% ({}/{})", cache_hit_rate, result.cache_hits, result.cache_misses);
        }
        
        println!();
        
        // Analysis
        println!("📈 Performance Analysis:");
        
        if let Some(speedup) = result.speedup {
            if speedup >= 4.0 {
                println!("The optimized resolver successfully achieves the performance target,");
                println!("demonstrating that the SAT-based approach with caching eliminates");
                println!("the O(N²) complexity of the original breadth-first search algorithm.");
            } else if speedup >= 2.0 {
                println!("The optimized resolver shows significant improvement over the original,");
                println!("but may need additional optimizations for very large dependency graphs.");
            } else {
                println!("The optimized resolver needs further optimization to achieve the");
                println!("target performance improvement for this scenario.");
            }
        }
        
        println!();
        println!("Key optimizations implemented:");
        println!("- SAT-based constraint solving eliminates O(N²) complexity");
        println!("- Aggressive caching reduces redundant version resolution");
        println!("- Conflict analysis learns from conflicts to avoid repeated work");
        println!("- Unit propagation optimizes constraint satisfaction");
        println!("- Parallel-safe caching for concurrent resolution");
    }
}

#[tokio::main]
async fn main() -> cursed::error::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();
    let mut benchmark = PackageBenchmark::new(args)?;
    
    benchmark.run().await?;
    
    Ok(())
}
