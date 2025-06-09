//! CLI optimization commands for the CURSED compiler
//!
//! This module provides command-line interface for optimization control,
//! allowing users to specify optimization levels, custom pass sequences,
//! and view optimization statistics.

use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{info, warn, error, debug};

use crate::codegen::llvm::{OptimizationManager, OptimizationConfig, OptimizationPass, create_optimization_manager};

/// Command-line arguments for optimization control
#[derive(Debug, Clone)]
pub struct OptimizationArgs {
    /// Optimization level (O0, O1, O2, O3, Os, Oz)
    pub level: String,
    /// Enable size optimization
    pub optimize_size: bool,
    /// Disable specific optimizations
    pub disable_passes: Vec<String>,
    /// Enable only specific optimizations
    pub enable_passes: Vec<String>,
    /// Show optimization statistics
    pub show_stats: bool,
    /// Enable optimization profiling
    pub profile: bool,
    /// Custom inline threshold
    pub inline_threshold: Option<u32>,
    /// Input file path
    pub input_file: PathBuf,
    /// Output file path
    pub output_file: Option<PathBuf>,
    /// Show available optimization passes
    pub list_passes: bool,
    /// Benchmark optimization levels
    pub benchmark: bool,
}

impl Default for OptimizationArgs {
    fn default() -> Self {
        Self {
            level: "O0".to_string(),
            optimize_size: false,
            disable_passes: Vec::new(),
            enable_passes: Vec::new(),
            show_stats: false,
            profile: false,
            inline_threshold: None,
            input_file: PathBuf::new(),
            output_file: None,
            list_passes: false,
            benchmark: false,
        }
    }
}

/// Parse optimization-related command line arguments
pub fn parse_optimization_args(args: &[String]) -> Result<Option<OptimizationArgs>, String> {
    if args.is_empty() {
        return Ok(None);
    }

    // Check if this is an optimization command
    if !args[0].starts_with("-O") && !args[0].starts_with("--opt") && 
       !args[0].starts_with("--list-passes") && !args[0].starts_with("--benchmark-opt") {
        return Ok(None);
    }

    let mut opt_args = OptimizationArgs::default();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            // Optimization levels
            arg if arg.starts_with("-O") => {
                let level = if arg.len() > 2 {
                    &arg[2..]
                } else if i + 1 < args.len() {
                    i += 1;
                    &args[i]
                } else {
                    return Err("Optimization level not specified".to_string());
                };
                
                match level {
                    "0" | "1" | "2" | "3" | "s" | "z" => {
                        opt_args.level = format!("O{}", level);
                    }
                    _ => return Err(format!("Invalid optimization level: {}", level)),
                }
            }
            
            // Long form optimization control
            "--optimize" | "--opt" => {
                if i + 1 < args.len() {
                    i += 1;
                    opt_args.level = args[i].clone();
                } else {
                    return Err("Optimization level not specified".to_string());
                }
            }

            // Size optimization
            "--optimize-size" | "-Os" => {
                opt_args.optimize_size = true;
                opt_args.level = "O2".to_string(); // Default to O2 with size optimization
            }

            // Aggressive size optimization
            "-Oz" => {
                opt_args.optimize_size = true;
                opt_args.level = "O3".to_string(); // Aggressive with size optimization
            }

            // Disable specific passes
            "--disable-pass" => {
                if i + 1 < args.len() {
                    i += 1;
                    opt_args.disable_passes.push(args[i].clone());
                } else {
                    return Err("Pass name not specified for --disable-pass".to_string());
                }
            }

            // Enable specific passes
            "--enable-pass" => {
                if i + 1 < args.len() {
                    i += 1;
                    opt_args.enable_passes.push(args[i].clone());
                } else {
                    return Err("Pass name not specified for --enable-pass".to_string());
                }
            }

            // Show optimization statistics
            "--opt-stats" | "--optimization-stats" => {
                opt_args.show_stats = true;
            }

            // Enable optimization profiling
            "--opt-profile" | "--optimization-profile" => {
                opt_args.profile = true;
            }

            // Custom inline threshold
            "--inline-threshold" => {
                if i + 1 < args.len() {
                    i += 1;
                    opt_args.inline_threshold = Some(args[i].parse().map_err(|_| {
                        format!("Invalid inline threshold: {}", args[i])
                    })?);
                } else {
                    return Err("Inline threshold not specified".to_string());
                }
            }

            // List available optimization passes
            "--list-passes" | "--list-optimization-passes" => {
                opt_args.list_passes = true;
            }

            // Benchmark optimization levels
            "--benchmark-opt" | "--benchmark-optimization" => {
                opt_args.benchmark = true;
            }

            // Output file
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    i += 1;
                    opt_args.output_file = Some(PathBuf::from(&args[i]));
                } else {
                    return Err("Output file not specified".to_string());
                }
            }

            // Input file (last argument or explicit)
            arg if !arg.starts_with("-") => {
                opt_args.input_file = PathBuf::from(arg);
            }

            _ => {
                return Err(format!("Unknown optimization option: {}", args[i]));
            }
        }
        i += 1;
    }

    Ok(Some(opt_args))
}

/// Execute optimization command
pub fn execute_optimization_command(args: &OptimizationArgs) -> Result<(), String> {
    if args.list_passes {
        print_available_passes();
        return Ok(());
    }

    if args.benchmark {
        return benchmark_optimization_levels(&args.input_file);
    }

    if args.input_file.as_os_str().is_empty() {
        return Err("Input file not specified".to_string());
    }

    info!(
        file = %args.input_file.display(),
        level = %args.level,
        "Starting optimization compilation"
    );

    // Read and compile the input file with optimizations
    let source = std::fs::read_to_string(&args.input_file)
        .map_err(|e| format!("Failed to read input file: {}", e))?;

    // Create optimization manager
    let mut manager = create_optimization_manager(&args.level)?;

    // Apply custom configuration
    let mut config = manager.get_config().clone();
    config.optimize_size = args.optimize_size;
    config.enable_profiling = args.profile;
    
    if let Some(threshold) = args.inline_threshold {
        config.inline_threshold = Some(threshold);
    }

    // Apply pass filtering
    apply_pass_filtering(&mut config, &args.disable_passes, &args.enable_passes)?;
    
    manager.set_config(config);

    // Compile with optimizations (this would integrate with the main compiler)
    compile_with_optimization(&source, &mut manager, &args.input_file, args.output_file.as_ref())?;

    // Show statistics if requested
    if args.show_stats {
        print_optimization_stats(manager.get_stats());
    }

    Ok(())
}

/// Apply pass filtering to optimization configuration
fn apply_pass_filtering(
    config: &mut OptimizationConfig,
    disable_passes: &[String],
    enable_passes: &[String],
) -> Result<(), String> {
    // Remove disabled passes
    config.custom_passes.retain(|pass| {
        let pass_name = format!("{:?}", pass);
        !disable_passes.iter().any(|disabled| pass_name.contains(disabled))
    });

    // Add enabled passes
    for pass_name in enable_passes {
        let pass = parse_optimization_pass(pass_name)?;
        config.custom_passes.push(pass);
    }

    Ok(())
}

/// Parse optimization pass from string name
fn parse_optimization_pass(name: &str) -> Result<OptimizationPass, String> {
    match name.to_lowercase().as_str() {
        "dce" | "dead-code-elimination" => Ok(OptimizationPass::DeadCodeElimination),
        "constant-folding" | "cf" => Ok(OptimizationPass::ConstantFolding),
        "inline" | "function-inlining" => Ok(OptimizationPass::FunctionInlining),
        "licm" | "loop-invariant-code-motion" => Ok(OptimizationPass::LoopInvariantCodeMotion),
        "cse" | "common-subexpression-elimination" => Ok(OptimizationPass::CommonSubexpressionElimination),
        "tail-call" | "tail-call-optimization" => Ok(OptimizationPass::TailCallOptimization),
        "mem2reg" | "memory-to-register" => Ok(OptimizationPass::MemoryToRegister),
        "loop-unroll" | "unroll" => Ok(OptimizationPass::LoopUnrolling),
        "vectorize" | "vectorization" => Ok(OptimizationPass::Vectorization),
        "adce" | "aggressive-dce" => Ok(OptimizationPass::AggressiveDeadCodeElimination),
        "gvn" | "global-value-numbering" => Ok(OptimizationPass::GlobalValueNumbering),
        _ => Ok(OptimizationPass::Custom(name.to_string())),
    }
}

/// Compile source code with optimization
fn compile_with_optimization(
    source: &str,
    manager: &mut OptimizationManager,
    input_file: &PathBuf,
    output_file: Option<&PathBuf>,
) -> Result<(), String> {
    // This would integrate with the main CURSED compiler
    // For now, we'll simulate the compilation process
    
    debug!("Parsing source code");
    
    // Parse the source 
    let mut lexer = crate::lexer::Lexer::new(source);
    let mut parser = crate::parser::Parser::new(&mut lexer)
        .map_err(|e| format!("Parser creation error: {:?}", e))?;
    let _ast = parser.parse_program()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    debug!("Generating LLVM IR");
    
    // Generate LLVM IR (placeholder)
    let context = inkwell::context::Context::create();
    let module = context.create_module(
        &input_file.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
    );

    // Apply optimizations
    manager.optimize_module(&module)?;

    // Generate output
    if let Some(output_path) = output_file {
        debug!(output = %output_path.display(), "Writing optimized IR to file");
        
        let ir_string = module.print_to_string().to_string();
        std::fs::write(output_path, ir_string)
            .map_err(|e| format!("Failed to write output file: {}", e))?;
    } else {
        // Print to stdout
        println!("{}", module.print_to_string().to_string());
    }

    Ok(())
}

/// Print available optimization passes
fn print_available_passes() {
    println!("Available Optimization Passes:");
    println!();
    println!("Basic Passes:");
    println!("  dce, dead-code-elimination      - Remove dead code");
    println!("  cf, constant-folding            - Fold constant expressions");
    println!("  mem2reg, memory-to-register     - Promote memory to SSA registers");
    println!();
    println!("Advanced Passes:");
    println!("  inline, function-inlining       - Inline function calls");
    println!("  cse, common-subexpression-elimination - Eliminate common subexpressions");
    println!("  licm, loop-invariant-code-motion - Move loop-invariant code");
    println!("  gvn, global-value-numbering     - Global value numbering");
    println!();
    println!("Aggressive Passes:");
    println!("  adce, aggressive-dce            - Aggressive dead code elimination");
    println!("  unroll, loop-unroll             - Unroll loops");
    println!("  vectorize, vectorization        - Vectorize operations");
    println!("  tail-call, tail-call-optimization - Optimize tail calls");
    println!();
    println!("Optimization Levels:");
    println!("  O0                              - No optimization");
    println!("  O1                              - Basic optimization");
    println!("  O2                              - Default optimization");
    println!("  O3                              - Aggressive optimization");
    println!("  Os                              - Optimize for size");
    println!("  Oz                              - Aggressively optimize for size");
}

/// Print optimization statistics
fn print_optimization_stats(stats: &crate::codegen::llvm::OptimizationStats) {
    println!();
    println!("Optimization Statistics:");
    println!("========================");
    println!("Total time:           {:?}", stats.total_time);
    println!("Function time:        {:?}", stats.function_time);
    println!("Module time:          {:?}", stats.module_time);
    println!("Functions optimized:  {}", stats.functions_optimized);
    println!("Passes applied:       {}", stats.passes_applied);
    println!("Code size before:     {} bytes", stats.code_size_before);
    println!("Code size after:      {} bytes", stats.code_size_after);
    println!("Size reduction:       {} bytes ({:.2}%)",
             stats.size_reduction_bytes(),
             stats.size_reduction_percentage());
    println!("Compression ratio:    {:.3}", stats.compression_ratio());
    
    if !stats.custom_metrics.is_empty() {
        println!();
        println!("Custom Metrics:");
        for (name, value) in &stats.custom_metrics {
            println!("  {}: {:.3}", name, value);
        }
    }
}

/// Benchmark different optimization levels
fn benchmark_optimization_levels(input_file: &PathBuf) -> Result<(), String> {
    if input_file.as_os_str().is_empty() {
        return Err("Input file not specified for benchmarking".to_string());
    }

    let source = std::fs::read_to_string(input_file)
        .map_err(|e| format!("Failed to read input file: {}", e))?;

    println!("Benchmarking Optimization Levels");
    println!("================================");
    println!("File: {}", input_file.display());
    println!();

    let levels = vec!["O0", "O1", "O2", "O3", "Os", "Oz"];
    let mut results = HashMap::new();

    for level in &levels {
        print!("Testing {}... ", level);
        
        match benchmark_single_level(&source, level, input_file) {
            Ok(stats) => {
                println!("Done");
                results.insert(level.to_string(), stats);
            }
            Err(e) => {
                println!("Failed: {}", e);
            }
        }
    }

    println!();
    print_benchmark_results(&results);

    Ok(())
}

/// Benchmark a single optimization level
fn benchmark_single_level(
    _source: &str,
    level: &str,
    input_file: &PathBuf,
) -> Result<crate::codegen::llvm::OptimizationStats, String> {
    let mut manager = create_optimization_manager(level)?;
    
    // Parse and generate IR (simplified)
    let context = inkwell::context::Context::create();
    let module = context.create_module(
        &input_file.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
    );

    // Apply optimizations
    manager.optimize_module(&module)?;
    
    Ok(manager.get_stats().clone())
}

/// Print benchmark results in a table format
fn print_benchmark_results(results: &HashMap<String, crate::codegen::llvm::OptimizationStats>) {
    println!("Benchmark Results:");
    println!("{:<8} {:<12} {:<12} {:<12} {:<12} {:<12}", 
             "Level", "Time (ms)", "Functions", "Passes", "Size Before", "Size After");
    println!("{}", "-".repeat(72));

    let levels = vec!["O0", "O1", "O2", "O3", "Os", "Oz"];
    
    for level in &levels {
        if let Some(stats) = results.get(*level) {
            println!("{:<8} {:<12} {:<12} {:<12} {:<12} {:<12}",
                     level,
                     stats.total_time.as_millis(),
                     stats.functions_optimized,
                     stats.passes_applied,
                     stats.code_size_before,
                     stats.code_size_after);
        }
    }

    println!();
    println!("Size Reduction Summary:");
    for level in &levels {
        if let Some(stats) = results.get(*level) {
            println!("{}: {:.2}% reduction ({} bytes saved)",
                     level,
                     stats.size_reduction_percentage(),
                     stats.size_reduction_bytes());
        }
    }
}

/// Print help for optimization commands
pub fn print_optimization_help() {
    println!("CURSED Compiler Optimization Options");
    println!("====================================");
    println!();
    println!("USAGE:");
    println!("    cursed -O<level> [OPTIONS] <input_file>");
    println!("    cursed --optimize <level> [OPTIONS] <input_file>");
    println!();
    println!("OPTIMIZATION LEVELS:");
    println!("    -O0                     No optimization (fastest compilation)");
    println!("    -O1                     Basic optimization");
    println!("    -O2                     Default optimization (recommended)");
    println!("    -O3                     Aggressive optimization");
    println!("    -Os                     Optimize for size");
    println!("    -Oz                     Aggressively optimize for size");
    println!();
    println!("OPTIONS:");
    println!("    --disable-pass <name>   Disable specific optimization pass");
    println!("    --enable-pass <name>    Enable specific optimization pass");
    println!("    --inline-threshold <n>  Set function inlining threshold");
    println!("    --opt-stats             Show optimization statistics");
    println!("    --opt-profile           Enable optimization profiling");
    println!("    --list-passes           List available optimization passes");
    println!("    --benchmark-opt         Benchmark all optimization levels");
    println!("    -o <file>               Output file");
    println!();
    println!("EXAMPLES:");
    println!("    cursed -O2 program.csd                    # Compile with O2 optimization");
    println!("    cursed -Os --opt-stats program.csd        # Optimize for size and show stats");
    println!("    cursed -O3 --disable-pass inline program.csd  # O3 without inlining");
    println!("    cursed --benchmark-opt program.csd        # Benchmark all levels");
    println!("    cursed --list-passes                      # Show available passes");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_optimization_args() {
        let args = vec!["-O2".to_string(), "test.csd".to_string()];
        let result = parse_optimization_args(&args).unwrap();
        
        assert!(result.is_some());
        let opt_args = result.unwrap();
        assert_eq!(opt_args.level, "O2");
        assert_eq!(opt_args.input_file, PathBuf::from("test.csd"));
    }

    #[test]
    fn test_parse_optimization_pass() {
        assert!(matches!(parse_optimization_pass("dce").unwrap(), OptimizationPass::DeadCodeElimination));
        assert!(matches!(parse_optimization_pass("inline").unwrap(), OptimizationPass::FunctionInlining));
        assert!(matches!(parse_optimization_pass("custom").unwrap(), OptimizationPass::Custom(_)));
    }

    #[test]
    fn test_invalid_optimization_level() {
        let args = vec!["-O9".to_string(), "test.csd".to_string()];
        let result = parse_optimization_args(&args);
        
        assert!(result.is_err());
    }
}
