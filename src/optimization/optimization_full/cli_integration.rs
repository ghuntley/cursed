// CLI integration for the CURSED optimization system

use crate::error::{CursedError, Result};
use crate::optimization::build_integration::{BuildOptimizer, create_build_optimizer_from_args, BuildOptimizationResult};

use std::path::PathBuf;
use std::time::Instant;
use tracing::{info, debug, warn, error};

/// CLI command for optimization
#[derive(Debug, Clone)]
pub struct OptimizeCommand {
impl Default for OptimizeCommand {
    fn default() -> Self {
        Self {
        }
    }
/// Execute the optimization command
pub fn execute_optimize_command(cmd: OptimizeCommand) -> Result<()> {
    let start_time = Instant::now();
    
    if cmd.verbose {
        info!("Starting CURSED optimization with configuration:");
        info!("  Project root: {:?}", cmd.project_root);
        info!("  Source files: {} files", cmd.source_files.len());
        info!("  Debug mode: {}", cmd.debug);
        info!("  Release mode: {}", cmd.release);
        info!("  Parallel jobs: {:?}", cmd.parallel);
        info!("  Cache disabled: {}", cmd.no_cache);
        info!("  Incremental disabled: {}", cmd.no_incremental);
    // Create build optimizer
    let mut optimizer = create_build_optimizer_from_args(
    )?;
    
    // Handle clean command
    if cmd.clean {
        info!("Cleaning build cache and temporary files...");
        optimizer.clean()?;
        println!("✅ Build cache cleaned successfully");
        return Ok(());
    // Run optimization
    info!("Running optimized build...");
    let result = optimizer.optimize_build()?;
    
    // Print results
    print_optimization_results(&result, cmd.verbose);
    
    // Print timing information
    let total_time = start_time.elapsed();
    if cmd.verbose {
        info!("Total command time: {:.2?}", total_time);
    // Print statistics if requested
    if cmd.verbose || cmd.profile {
        let stats = optimizer.get_statistics();
        print_optimization_statistics(&stats);
    // Exit with appropriate code
    if result.success {
        println!("✅ Optimization completed successfully");
        Ok(())
    } else {
        error!("❌ Optimization failed");
        for error in &result.errors {
            error!("  {}", error);
        }
        Err(CursedError::optimization_error("Optimization failed"))
    }
}

/// Print optimization results
fn print_optimization_results(result: &BuildOptimizationResult, verbose: bool) {
    println!("\n📊 Optimization Results:");
    println!("  ✅ Success: {}", if result.success { "Yes" } else { "No" });
    println!("  ⏱️  Total time: {:.2?}", result.total_time);
    println!("  🔧 Optimization time: {:.2?}", result.optimization_time);
    println!("  📁 Files compiled: {}", result.files_compiled);
    
    if result.files_cached > 0 {
        println!("  💾 Files from cache: {}", result.files_cached);
        println!("  📈 Cache hit rate: {:.1}%", result.cache_hit_rate * 100.0);
    if result.parallel_efficiency > 0.0 {
        println!("  🔄 Parallel efficiency: {:.1}%", result.parallel_efficiency * 100.0);
    if result.size_reduction_bytes > 0 {
                 result.size_reduction_bytes as f64 / 1024.0);
    // Show warnings if any
    if !result.warnings.is_empty() {
        println!("\n⚠️  Warnings:");
        for warning in &result.warnings {
            println!("  {}", warning);
        }
    }
    
    // Show errors if any
    if !result.errors.is_empty() {
        println!("\n❌ Errors:");
        for error in &result.errors {
            println!("  {}", error);
        }
    }
    
    // Show performance summary
    if verbose && !result.performance_summary.is_empty() {
        println!("\n📈 Performance Summary:");
        println!("  {}", result.performance_summary);
    }
}

/// Print optimization statistics
fn print_optimization_statistics(stats: &crate::optimization::build_integration::OptimizationStatistics) {
    println!("\n📊 Optimization Statistics:");
    println!("  📈 Total compilations: {}", stats.total_compilations);
    println!("  ✅ Successful compilations: {}", stats.successful_compilations);
    
    if stats.total_compilations > 0 {
        let success_rate = (stats.successful_compilations as f64 / stats.total_compilations as f64) * 100.0;
        println!("  🎯 Success rate: {:.1}%", success_rate);
    println!("  ⏱️  Average compilation time: {:.2?}", stats.average_compilation_time);
    
    println!("\n🔧 Optimization Features:");
    println!("  💾 Cache: {}", if stats.cache_enabled { "Enabled" } else { "Disabled" });
    println!("  🔄 Incremental: {}", if stats.incremental_enabled { "Enabled" } else { "Disabled" });
    println!("  ⚡ Parallel: {}", if stats.parallel_enabled { "Enabled" } else { "Disabled" });
    
    if stats.cached_units > 0 {
        println!("  📁 Cached units: {}", stats.cached_units);
    }
}

/// Create optimization command from CLI arguments
pub fn create_optimize_command_from_args(
) -> OptimizeCommand {
    let project_root = project_root.unwrap_or_else(|| 
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    );
    
    let source_files: Vec<PathBuf> = source_files.into_iter()
        .map(PathBuf::from)
        .collect();
    
    let output_dir = output_dir.map(PathBuf::from);
    
    OptimizeCommand {
    }
}

/// Print optimization help
pub fn print_optimization_help() {
        // TODO: implement
    }
    println!("CURSED Optimization System");
    println!();
    println!("USAGE:");
    println!("    cursed optimize [OPTIONS] [FILES...]");
    println!();
    println!("FLAGS:");
    println!("    -h, --help         Print help information");
    println!("    -v, --verbose      Enable verbose output");
    println!("    -d, --debug        Enable debug mode (faster compilation)");
    println!("    -r, --release      Enable release mode (better optimization)");
    println!("        --no-cache     Disable compilation caching");
    println!("        --no-incremental Disable incremental compilation");
    println!("        --benchmark    Run performance benchmarks");
    println!("        --clean        Clean build cache and exit");
    println!("        --profile      Show detailed performance profiling");
    println!();
    println!("OPTIONS:");
    println!("    -o, --output <DIR>           Output directory [default: target]");
    println!("    -t, --target <TARGET>        Target triple [default: native]");
    println!("    -O, --optimization <LEVEL>   Optimization level (O0, O1, O2, O3) [default: O2]");
    println!("    -j, --parallel <JOBS>        Number of parallel jobs [default: auto]");
    println!();
    println!("ARGS:");
    println!("    <FILES>...    Source files to compile");
    println!();
    println!("EXAMPLES:");
    println!("    cursed optimize main.csd lib.csd          Optimize specific files");
    println!("    cursed optimize --debug --verbose         Debug build with verbose output");
    println!("    cursed optimize --release -O3             Release build with aggressive optimization");
    println!("    cursed optimize --clean                    Clean build cache");
    println!("    cursed optimize --benchmark                Run performance benchmarks");
    println!("    cursed optimize --profile                  Show detailed profiling information");
    println!();
    println!("The CURSED optimization system provides:");
    println!("  • Intelligent compilation caching");
    println!("  • Incremental compilation for faster rebuilds");
    println!("  • Parallel compilation for better performance");
    println!("  • LLVM optimization passes integration");
    println!("  • Performance profiling and analysis");
    println!("  • Automated benchmarking");
/// Discover source files in a directory
pub fn discover_source_files(dir: &PathBuf, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut source_files = Vec::new();
    
    if !dir.exists() {
        return Err(CursedError::optimization_error(&format!("Directory does not exist: {:?}", dir)));
    if !dir.is_dir() {
        return Err(CursedError::optimization_error(&format!("Path is not a directory: {:?}", dir)));
    let entries = std::fs::read_dir(dir).map_err(|e| {
        CursedError::optimization_error(&format!("Failed to read directory {:?}: {}", dir, e))
    })?;
    
    for entry in entries {
        let entry = entry.map_err(|e| {
            CursedError::optimization_error(&format!("Failed to read directory entry: {}", e))
        })?;
        
        let path = entry.path();
        
        if path.is_file() {
            // Check if it's a CURSED source file
            if let Some(extension) = path.extension() {
                if extension == "csd" || extension == "cursed" {
                    source_files.push(path);
                }
            }
        } else if path.is_dir() && recursive {
            // Recursively search subdirectories
            let mut sub_files = discover_source_files(&path, recursive)?;
            source_files.append(&mut sub_files);
        }
    }
    
    source_files.sort();
    Ok(source_files)
