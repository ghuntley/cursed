
//! CLI interface for CURSED comprehensive optimization system
//! 
//! Provides command-line access to all optimization features.

use std::path::{Path, PathBuf};
use std::time::Duration;
use clap::{Arg, ArgAction, Command, ArgMatches};
use tracing::{info, error};

use crate::optimization::comprehensive_optimization_enablement::{
    ComprehensiveOptimizationSystem, ComprehensiveOptimizationConfig,
    OptimizationResults,
};

use crate::common::optimization_level::OptimizationLevel;
use crate::error::{Result, CursedError};

/// CLI interface for comprehensive optimization system
pub struct OptimizationCLI;

impl OptimizationCLI {
    /// Add optimization-related arguments to a clap command
    pub fn add_optimization_args(command: Command) -> Command {
        command
            // Optimization level selection
            .arg(
                Arg::new("opt-level")
                    .short('O')
                    .long("opt-level")
                    .value_name("LEVEL")
                    .help("Optimization level (0=debug, 1=basic, 2=standard, 3=aggressive, s=size, z=aggressive-size)")
                    .default_value("2")
            )
            
            // Core optimization passes
            .arg(
                Arg::new("enable-inlining")
                    .long("enable-inlining")
                    .action(ArgAction::SetTrue)
                    .help("Enable function inlining optimization")
            )
            .arg(
                Arg::new("disable-inlining")
                    .long("disable-inlining")
                    .action(ArgAction::SetTrue)
                    .help("Disable function inlining optimization")
            )
            .arg(
                Arg::new("enable-vectorization")
                    .long("enable-vectorization")
                    .action(ArgAction::SetTrue)
                    .help("Enable vectorization optimization")
            )
            .arg(
                Arg::new("disable-vectorization")
                    .long("disable-vectorization")
                    .action(ArgAction::SetTrue)
                    .help("Disable vectorization optimization")
            )
            .arg(
                Arg::new("enable-loop-unrolling")
                    .long("enable-loop-unrolling")
                    .action(ArgAction::SetTrue)
                    .help("Enable loop unrolling optimization")
            )
            .arg(
                Arg::new("disable-loop-unrolling")
                    .long("disable-loop-unrolling")
                    .action(ArgAction::SetTrue)
                    .help("Disable loop unrolling optimization")
            )
            .arg(
                Arg::new("enable-cse")
                    .long("enable-cse")
                    .action(ArgAction::SetTrue)
                    .help("Enable common subexpression elimination")
            )
            .arg(
                Arg::new("disable-cse")
                    .long("disable-cse")
                    .action(ArgAction::SetTrue)
                    .help("Disable common subexpression elimination")
            )
            .arg(
                Arg::new("enable-tail-call")
                    .long("enable-tail-call")
                    .action(ArgAction::SetTrue)
                    .help("Enable tail call optimization")
            )
            .arg(
                Arg::new("disable-tail-call")
                    .long("disable-tail-call")
                    .action(ArgAction::SetTrue)
                    .help("Disable tail call optimization")
            )
            .arg(
                Arg::new("enable-lto")
                    .long("enable-lto")
                    .action(ArgAction::SetTrue)
                    .help("Enable link-time optimization")
            )
            .arg(
                Arg::new("disable-lto")
                    .long("disable-lto")
                    .action(ArgAction::SetTrue)
                    .help("Disable link-time optimization")
            )
            .arg(
                Arg::new("enable-ipa")
                    .long("enable-ipa")
                    .action(ArgAction::SetTrue)
                    .help("Enable interprocedural analysis")
            )
            .arg(
                Arg::new("disable-ipa")
                    .long("disable-ipa")
                    .action(ArgAction::SetTrue)
                    .help("Disable interprocedural analysis")
            )
            
            // Advanced optimization features
            .arg(
                Arg::new("enable-pgo")
                    .long("enable-pgo")
                    .action(ArgAction::SetTrue)
                    .help("Enable profile-guided optimization")
            )
            .arg(
                Arg::new("disable-pgo")
                    .long("disable-pgo")
                    .action(ArgAction::SetTrue)
                    .help("Disable profile-guided optimization")
            )
            .arg(
                Arg::new("enable-memory-layout")
                    .long("enable-memory-layout")
                    .action(ArgAction::SetTrue)
                    .help("Enable memory layout optimization")
            )
            .arg(
                Arg::new("enable-advanced-vectorization")
                    .long("enable-advanced-vectorization")
                    .action(ArgAction::SetTrue)
                    .help("Enable advanced vectorization")
            )
            .arg(
                Arg::new("enable-loop-fusion")
                    .long("enable-loop-fusion")
                    .action(ArgAction::SetTrue)
                    .help("Enable loop fusion optimization")
            )
            .arg(
                Arg::new("enable-prefetch")
                    .long("enable-prefetch")
                    .action(ArgAction::SetTrue)
                    .help("Enable prefetch insertion")
            )
            .arg(
                Arg::new("enable-numa")
                    .long("enable-numa")
                    .action(ArgAction::SetTrue)
                    .help("Enable NUMA optimization")
            )
            
            // Compilation speed improvements
            .arg(
                Arg::new("parallel-jobs")
                    .short('j')
                    .long("parallel-jobs")
                    .value_name("N")
                    .help("Number of parallel compilation jobs (0 = auto-detect)")
                    .default_value("0")
            )
            .arg(
                Arg::new("enable-incremental")
                    .long("enable-incremental")
                    .action(ArgAction::SetTrue)
                    .help("Enable incremental compilation")
            )
            .arg(
                Arg::new("disable-incremental")
                    .long("disable-incremental")
                    .action(ArgAction::SetTrue)
                    .help("Disable incremental compilation")
            )
            .arg(
                Arg::new("enable-caching")
                    .long("enable-caching")
                    .action(ArgAction::SetTrue)
                    .help("Enable compilation caching")
            )
            .arg(
                Arg::new("disable-caching")
                    .long("disable-caching")
                    .action(ArgAction::SetTrue)
                    .help("Disable compilation caching")
            )
            .arg(
                Arg::new("cache-dir")
                    .long("cache-dir")
                    .value_name("DIR")
                    .help("Directory for compilation cache")
                    .default_value("target/cursed-cache")
            )
            
            // Performance monitoring
            .arg(
                Arg::new("enable-benchmarking")
                    .long("enable-benchmarking")
                    .action(ArgAction::SetTrue)
                    .help("Enable performance benchmarking")
            )
            .arg(
                Arg::new("enable-profiling")
                    .long("enable-profiling")
                    .action(ArgAction::SetTrue)
                    .help("Enable runtime profiling")
            )
            .arg(
                Arg::new("profile-data-dir")
                    .long("profile-data-dir")
                    .value_name("DIR")
                    .help("Directory for profile data")
                    .default_value("target/pgo-data")
            )
            .arg(
                Arg::new("optimization-timeout")
                    .long("optimization-timeout")
                    .value_name("SECONDS")
                    .help("Optimization timeout in seconds")
                    .default_value("600")
            )
            
            // Smart optimization features
            .arg(
                Arg::new("enable-smart-selection")
                    .long("enable-smart-selection")
                    .action(ArgAction::SetTrue)
                    .help("Enable smart optimization selection based on code patterns")
            )
            .arg(
                Arg::new("enable-adaptive")
                    .long("enable-adaptive")
                    .action(ArgAction::SetTrue)
                    .help("Enable adaptive optimization levels")
            )
            
            // Reporting
            .arg(
                Arg::new("optimization-report")
                    .long("optimization-report")
                    .action(ArgAction::SetTrue)
                    .help("Generate optimization performance report")
            )
            .arg(
                Arg::new("verbose-optimization")
                    .long("verbose-optimization")
                    .action(ArgAction::SetTrue)
                    .help("Verbose optimization output")
            )
            
            // Presets for common use cases
            .arg(
                Arg::new("fast-compile")
                    .long("fast-compile")
                    .action(ArgAction::SetTrue)
                    .help("Optimize for fast compilation (equivalent to -O1 with parallel and caching)")
            )
            .arg(
                Arg::new("max-performance")
                    .long("max-performance")
                    .action(ArgAction::SetTrue)
                    .help("Optimize for maximum runtime performance (equivalent to -O3 with all features)")
            )
            .arg(
                Arg::new("min-size")
                    .long("min-size")
                    .action(ArgAction::SetTrue)
                    .help("Optimize for minimum binary size (equivalent to -Os)")
            )
    }
    
    /// Parse CLI arguments and create optimization configuration
    pub fn parse_optimization_config(matches: &ArgMatches) -> Result<ComprehensiveOptimizationConfig> {
        let mut config = ComprehensiveOptimizationConfig::default();
        
        // Handle presets first
        if matches.get_flag("fast-compile") {
            config = ComprehensiveOptimizationConfig::basic_config();
            config.enable_parallel_compilation = true;
            config.enable_caching_mechanisms = true;
            config.max_parallel_jobs = num_cpus::get();
        } else if matches.get_flag("max-performance") {
            config = ComprehensiveOptimizationConfig::aggressive_config();
        } else if matches.get_flag("min-size") {
            config = ComprehensiveOptimizationConfig::size_config();
        } else {
            // Parse optimization level
            if let Some(opt_level_str) = matches.get_one::<String>("opt-level") {
                config.optimization_level = match opt_level_str.as_str() {
                    "0" => OptimizationLevel::O0,
                    "1" => OptimizationLevel::O1,
                    "2" => OptimizationLevel::O2,
                    "3" => OptimizationLevel::O3,
                    "s" => OptimizationLevel::Os,
                    "z" => OptimizationLevel::Os, // Treat z as size for now
                    "fast" => OptimizationLevel::Fast,
                    _ => return Err(CursedError::generic(format!("Invalid optimization level: {}", opt_level_str))),
                };
                
                // Apply appropriate configuration for the level
                config = match config.optimization_level {
                    OptimizationLevel::O0 => ComprehensiveOptimizationConfig::debug_config(),
                    OptimizationLevel::O1 => ComprehensiveOptimizationConfig::basic_config(),
                    OptimizationLevel::O2 => ComprehensiveOptimizationConfig::standard_config(),
                    OptimizationLevel::O3 => ComprehensiveOptimizationConfig::aggressive_config(),
                    OptimizationLevel::Os => ComprehensiveOptimizationConfig::size_config(),
                    OptimizationLevel::Fast => ComprehensiveOptimizationConfig::aggressive_config(),
                };
            }
        }
        
        // Override with specific flags
        Self::apply_optimization_overrides(&mut config, matches)?;
        
        Ok(config)
    }
    
    /// Apply specific optimization overrides from CLI flags
    fn apply_optimization_overrides(config: &mut ComprehensiveOptimizationConfig, matches: &ArgMatches) -> Result<()> {
        // Core optimization passes
        if matches.get_flag("enable-inlining") {
            config.enable_function_inlining = true;
        } else if matches.get_flag("disable-inlining") {
            config.enable_function_inlining = false;
        }
        
        if matches.get_flag("enable-vectorization") {
            config.enable_vectorization = true;
        } else if matches.get_flag("disable-vectorization") {
            config.enable_vectorization = false;
        }
        
        if matches.get_flag("enable-loop-unrolling") {
            config.enable_loop_unrolling = true;
        } else if matches.get_flag("disable-loop-unrolling") {
            config.enable_loop_unrolling = false;
        }
        
        if matches.get_flag("enable-cse") {
            config.enable_common_subexpression_elimination = true;
        } else if matches.get_flag("disable-cse") {
            config.enable_common_subexpression_elimination = false;
        }
        
        if matches.get_flag("enable-tail-call") {
            config.enable_tail_call_optimization = true;
        } else if matches.get_flag("disable-tail-call") {
            config.enable_tail_call_optimization = false;
        }
        
        if matches.get_flag("enable-lto") {
            config.enable_link_time_optimization = true;
        } else if matches.get_flag("disable-lto") {
            config.enable_link_time_optimization = false;
        }
        
        if matches.get_flag("enable-ipa") {
            config.enable_interprocedural_analysis = true;
        } else if matches.get_flag("disable-ipa") {
            config.enable_interprocedural_analysis = false;
        }
        
        // Advanced optimization features
        if matches.get_flag("enable-pgo") {
            config.enable_profile_guided_optimization = true;
        } else if matches.get_flag("disable-pgo") {
            config.enable_profile_guided_optimization = false;
        }
        
        if matches.get_flag("enable-memory-layout") {
            config.enable_memory_layout_optimization = true;
        }
        
        if matches.get_flag("enable-advanced-vectorization") {
            config.enable_advanced_vectorization = true;
        }
        
        if matches.get_flag("enable-loop-fusion") {
            config.enable_loop_fusion = true;
        }
        
        if matches.get_flag("enable-prefetch") {
            config.enable_prefetch_insertion = true;
        }
        
        if matches.get_flag("enable-numa") {
            config.enable_numa_optimization = true;
        }
        
        // Compilation speed improvements
        if let Some(jobs_str) = matches.get_one::<String>("parallel-jobs") {
            let jobs: usize = jobs_str.parse()
                .map_err(|_| CursedError::generic("Invalid parallel jobs value"))?;
            config.max_parallel_jobs = if jobs == 0 { num_cpus::get() } else { jobs };
            config.enable_parallel_compilation = config.max_parallel_jobs > 1;
        }
        
        if matches.get_flag("enable-incremental") {
            config.enable_incremental_compilation = true;
        } else if matches.get_flag("disable-incremental") {
            config.enable_incremental_compilation = false;
        }
        
        if matches.get_flag("enable-caching") {
            config.enable_caching_mechanisms = true;
        } else if matches.get_flag("disable-caching") {
            config.enable_caching_mechanisms = false;
        }
        
        if let Some(cache_dir) = matches.get_one::<String>("cache-dir") {
            config.cache_directory = Some(PathBuf::from(cache_dir));
        }
        
        // Performance monitoring
        if matches.get_flag("enable-benchmarking") {
            config.enable_benchmark_measurement = true;
        }
        
        if matches.get_flag("enable-profiling") {
            config.enable_profiling_integration = true;
        }
        
        if let Some(profile_dir) = matches.get_one::<String>("profile-data-dir") {
            config.profile_data_directory = Some(PathBuf::from(profile_dir));
        }
        
        if let Some(timeout_str) = matches.get_one::<String>("optimization-timeout") {
            let timeout_secs: u64 = timeout_str.parse()
                .map_err(|_| CursedError::generic("Invalid optimization timeout value"))?;
            config.optimization_timeout = Duration::from_secs(timeout_secs);
        }
        
        // Smart optimization features
        if matches.get_flag("enable-smart-selection") {
            config.enable_smart_optimization_selection = true;
        }
        
        Ok(())
    }
    
    /// Optimize source code using CLI configuration
    pub fn optimize_source_code(
        source_code: &str,
        target_path: &Path,
        matches: &ArgMatches,
    ) -> Result<OptimizationResults> {
        let config = Self::parse_optimization_config(matches)?;
        let mut system = ComprehensiveOptimizationSystem::with_config(config)?;
        
        if matches.get_flag("verbose-optimization") {
            info!("Starting optimization with configuration:");
            info!("  Optimization level: {:?}", system.config.optimization_level);
            info!("  Function inlining: {}", system.config.enable_function_inlining);
            info!("  Vectorization: {}", system.config.enable_vectorization);
            info!("  Loop unrolling: {}", system.config.enable_loop_unrolling);
            info!("  Link-time optimization: {}", system.config.enable_link_time_optimization);
            info!("  Profile-guided optimization: {}", system.config.enable_profile_guided_optimization);
            info!("  Parallel compilation: {} (jobs: {})", system.config.enable_parallel_compilation, system.config.max_parallel_jobs);
        }
        
        let results = system.optimize_source_code(source_code, target_path)?;
        
        if matches.get_flag("optimization-report") {
            let report = system.generate_performance_report()?;
            println!("\n{}", report);
        }
        
        if matches.get_flag("verbose-optimization") {
            println!("\nOptimization Results:");
            println!("  Overall improvement: {:.1}%", results.overall_improvement * 100.0);
            println!("  Function inlining improvement: {:.1}%", results.function_inlining_improvement * 100.0);
            println!("  Vectorization improvement: {:.1}%", results.vectorization_improvement * 100.0);
            println!("  Loop optimization improvement: {:.1}%", results.loop_optimization_improvement * 100.0);
            println!("  Link-time optimization improvement: {:.1}%", results.lto_improvement * 100.0);
            println!("  Profile-guided optimization improvement: {:.1}%", results.pgo_improvement * 100.0);
            println!("  Cache hit rate: {:.1}%", results.cache_hit_rate * 100.0);
            println!("  Parallel efficiency: {:.1}%", results.parallel_efficiency * 100.0);
            println!("  Optimizations applied: {}", results.optimizations_applied);
        }
        
        Ok(results)
    }
    
    /// Display optimization help information
    pub fn display_optimization_help() {
        println!("CURSED Comprehensive Optimization System");
        println!("=======================================");
        println!();
        println!("Optimization Levels:");
        println!("  -O0  Debug: Fast compilation, minimal optimization");
        println!("  -O1  Basic: Enable core optimizations");
        println!("  -O2  Standard: Enable most optimizations (default)");
        println!("  -O3  Aggressive: Enable all optimizations");
        println!("  -Os  Size: Optimize for binary size");
        println!("  -Oz  Aggressive Size: Aggressively optimize for size");
        println!();
        println!("Core Optimization Passes (enabled by default in O1+):");
        println!("  Function Inlining:              --enable-inlining / --disable-inlining");
        println!("  Vectorization:                  --enable-vectorization / --disable-vectorization");
        println!("  Loop Unrolling:                 --enable-loop-unrolling / --disable-loop-unrolling");
        println!("  Common Subexpression Elim:      --enable-cse / --disable-cse");
        println!("  Tail Call Optimization:         --enable-tail-call / --disable-tail-call");
        println!("  Link-time Optimization:         --enable-lto / --disable-lto");
        println!("  Interprocedural Analysis:       --enable-ipa / --disable-ipa");
        println!();
        println!("Advanced Optimization Features:");
        println!("  Profile-guided Optimization:    --enable-pgo / --disable-pgo");
        println!("  Memory Layout Optimization:     --enable-memory-layout");
        println!("  Advanced Vectorization:         --enable-advanced-vectorization");
        println!("  Loop Fusion:                    --enable-loop-fusion");
        println!("  Prefetch Insertion:             --enable-prefetch");
        println!("  NUMA Optimization:              --enable-numa");
        println!();
        println!("Compilation Speed Improvements:");
        println!("  Parallel Compilation:           -j N / --parallel-jobs N");
        println!("  Incremental Compilation:        --enable-incremental / --disable-incremental");
        println!("  Caching Mechanisms:             --enable-caching / --disable-caching");
        println!("  Cache Directory:                --cache-dir DIR");
        println!();
        println!("Performance Monitoring:");
        println!("  Benchmark Measurement:          --enable-benchmarking");
        println!("  Runtime Profiling:              --enable-profiling");
        println!("  Profile Data Directory:         --profile-data-dir DIR");
        println!("  Optimization Timeout:           --optimization-timeout SECONDS");
        println!();
        println!("Smart Features:");
        println!("  Smart Optimization Selection:   --enable-smart-selection");
        println!("  Adaptive Optimization:          --enable-adaptive");
        println!();
        println!("Presets:");
        println!("  Fast Compilation:               --fast-compile");
        println!("  Maximum Performance:            --max-performance");
        println!("  Minimum Size:                   --min-size");
        println!();
        println!("Reporting:");
        println!("  Optimization Report:            --optimization-report");
        println!("  Verbose Output:                 --verbose-optimization");
        println!();
        println!("Examples:");
        println!("  cursed compile -O3 --max-performance myfile.csd");
        println!("  cursed compile --fast-compile -j 8 myfile.csd");
        println!("  cursed compile --min-size --enable-lto myfile.csd");
        println!("  cursed compile --enable-pgo --optimization-report myfile.csd");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;
    
    #[test]
    fn test_cli_optimization_args() {
        let app = OptimizationCLI::add_optimization_args(
            Command::new("test").arg(Arg::new("input").required(true))
        );
        
        // Test basic functionality
        let matches = app.try_get_matches_from(vec!["test", "input.csd", "-O3"]).unwrap();
        let config = OptimizationCLI::parse_optimization_config(&matches).unwrap();
        
        assert_eq!(config.optimization_level, OptimizationLevel::O3);
    }
    
    #[test]
    fn test_preset_configurations() {
        let app = OptimizationCLI::add_optimization_args(Command::new("test"));
        
        // Test fast compile preset
        let matches = app.clone().try_get_matches_from(vec!["test", "--fast-compile"]).unwrap();
        let config = OptimizationCLI::parse_optimization_config(&matches).unwrap();
        assert_eq!(config.optimization_level, OptimizationLevel::O1);
        assert!(config.enable_parallel_compilation);
        assert!(config.enable_caching_mechanisms);
        
        // Test max performance preset
        let matches = app.clone().try_get_matches_from(vec!["test", "--max-performance"]).unwrap();
        let config = OptimizationCLI::parse_optimization_config(&matches).unwrap();
        assert_eq!(config.optimization_level, OptimizationLevel::O3);
        
        // Test min size preset
        let matches = app.clone().try_get_matches_from(vec!["test", "--min-size"]).unwrap();
        let config = OptimizationCLI::parse_optimization_config(&matches).unwrap();
        assert_eq!(config.optimization_level, OptimizationLevel::Os);
    }
    
    #[test]
    fn test_optimization_overrides() {
        let app = OptimizationCLI::add_optimization_args(Command::new("test"));
        
        let matches = app.try_get_matches_from(vec![
            "test",
            "-O2",
            "--disable-inlining",
            "--enable-pgo",
            "--parallel-jobs", "4",
            "--cache-dir", "/tmp/test-cache",
        ]).unwrap();
        
        let config = OptimizationCLI::parse_optimization_config(&matches).unwrap();
        
        assert_eq!(config.optimization_level, OptimizationLevel::O2);
        assert!(!config.enable_function_inlining); // Override
        assert!(config.enable_profile_guided_optimization); // Override
        assert_eq!(config.max_parallel_jobs, 4);
        assert_eq!(config.cache_directory, Some(PathBuf::from("/tmp/test-cache")));
    }
}
