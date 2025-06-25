use crate::error::CursedError;
// CURSED Optimization Profiler
// 
// A command-line tool for analyzing and reporting optimization performance
// across different optimization levels and compilation strategies.

use clap::{Arg, App, SubCommand};
use cursed::optimization::{
    CompilerPassManager
// };

use cursed::error::Result;
use cursed::optimization::config::OptimizationLevel;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::fs;
use serde_json;
use tracing::{info, warn, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let matches = App::new("CURSED Optimization Profiler")
        .version("1.0.0")
        .author("CURSED Team")
        .about("Analyze and profile CURSED compiler optimization performance")
        .subcommand(
            SubCommand::with_name("analyze")
                .about("Analyze a CURSED source file for optimization opportunities")
                .arg(
                    Arg::with_name("input")
                        .help("Input CURSED source file")
                        .required(true)
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("FILE")
                        .help("Output analysis report file")
                )
                .arg(
                    Arg::with_name("level")
                        .short("O")
                        .long("optimization-level")
                        .value_name("LEVEL")
                        .help("Optimization level (O0, O1, O2, O3)")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("json")
                        .long("json")
                        .help("Output results in JSON format")
        )
        .subcommand(
            SubCommand::with_name("benchmark")
                .about("Benchmark optimization performance across different levels")
                .arg(
                    Arg::with_name("input")
                        .help("Input CURSED source file or directory")
                        .required(true)
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("FILE")
                        .help("Output benchmark report file")
                )
                .arg(
                    Arg::with_name("iterations")
                        .short("i")
                        .long("iterations")
                        .value_name("N")
                        .help("Number of benchmark iterations")
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("compare")
                .about("Compare optimization performance between different configurations")
                .arg(
                    Arg::with_name("input")
                        .help("Input CURSED source file")
                        .required(true)
                )
                .arg(
                    Arg::with_name("baseline")
                        .long("baseline")
                        .value_name("LEVEL")
                        .help("Baseline optimization level")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("target")
                        .long("target")
                        .value_name("LEVEL")
                        .help("Target optimization level")
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("profile")
                .about("Profile compilation passes and their performance impact")
                .arg(
                    Arg::with_name("input")
                        .help("Input CURSED source file")
                        .required(true)
                )
                .arg(
                    Arg::with_name("passes")
                        .long("passes")
                        .help("Profile individual compiler passes")
                )
        )
        .get_matches();

    match matches.subcommand() {
        ("analyze", Some(sub_m)) => {
            let input_file = sub_m.value_of("input").unwrap();
            let output_file = sub_m.value_of("output");
            let opt_level = parse_optimization_level(sub_m.value_of("level").unwrap())?;
            let json_output = sub_m.is_present("json");
            
            run_analysis(input_file, output_file, opt_level, json_output).await?;
        }
        ("benchmark", Some(sub_m)) => {
            let input = sub_m.value_of("input").unwrap();
            let output_file = sub_m.value_of("output");
            let iterations: usize = sub_m.value_of("iterations").unwrap().parse()
                .map_err(|_| cursed::error::CursedError::General("Invalid iterations count".to_string()))?;
                
            run_benchmark(input, output_file, iterations).await?;
        }
        ("compare", Some(sub_m)) => {
            let input_file = sub_m.value_of("input").unwrap();
            let baseline = parse_optimization_level(sub_m.value_of("baseline").unwrap())?;
            let target = parse_optimization_level(sub_m.value_of("target").unwrap())?;
            
            run_comparison(input_file, baseline, target).await?;
        }
        ("profile", Some(sub_m)) => {
            let input_file = sub_m.value_of("input").unwrap();
            let profile_passes = sub_m.is_present("passes");
            
            run_profiling(input_file, profile_passes).await?;
        }
        _ => {
            eprintln!("No subcommand specified. Use --help for usage information.");
            std::process::exit(1);
        }
    }

    Ok(())
async fn run_analysis(
) -> Result<()> {
    info!("Starting optimization analysis for {}", input_file);

    // Read source file
    let source = fs::read_to_string(input_file)
        .map_err(|e| cursed::error::CursedError::General(format!("Failed to read input file: {}", e)))?;

    // Create enhanced performance analyzer
    let mut analyzer = EnhancedPerformanceAnalyzer::new();

    // Perform comprehensive analysis
    let analysis_result = analyzer.analyze_compilation(&source, input_file, opt_level).await?;

    // Generate report
    let report = if json_output {
        generate_json_report(&analysis_result)?
    } else {
        generate_text_report(&analysis_result)

    // Output report
    if let Some(output_path) = output_file {
        fs::write(output_path, &report)
            .map_err(|e| cursed::error::CursedError::General(format!("Failed to write report: {}", e)))?;
        info!("Analysis report written to {}", output_path);
    } else {
        println!("{}", report);
    Ok(())
async fn run_benchmark(input: &str, output_file: Option<&str>, iterations: usize) -> Result<()> {
    info!("Starting optimization benchmark for {} ({} iterations)", input, iterations);

    let optimization_levels = [
    ];

    let mut benchmark_results = Vec::new();

    for &opt_level in &optimization_levels {
        info!("Benchmarking optimization level {:?}", opt_level);
        
        let mut total_time = Duration::ZERO;
        let mut successful_runs = 0;

        for iteration in 0..iterations {
            match run_single_benchmark(input, opt_level).await {
                Ok(duration) => {
                    total_time += duration;
                    successful_runs += 1;
                }
                Err(e) => {
                    warn!("Benchmark iteration {} failed: {}", iteration + 1, e);
                }
            }
        if successful_runs > 0 {
            let average_time = total_time / successful_runs as u32;
            benchmark_results.push(BenchmarkResult {
            });
        }
    }

    // Generate benchmark report
    let report = generate_benchmark_report(&benchmark_results);

    // Output report
    if let Some(output_path) = output_file {
        fs::write(output_path, &report)
            .map_err(|e| cursed::error::CursedError::General(format!("Failed to write report: {}", e)))?;
        info!("Benchmark report written to {}", output_path);
    } else {
        println!("{}", report);
    Ok(())
async fn run_comparison(
) -> Result<()> {
    info!("Comparing optimization levels {:?} vs {:?}", baseline, target);

    // Read source file
    let source = fs::read_to_string(input_file)
        .map_err(|e| cursed::error::CursedError::General(format!("Failed to read input file: {}", e)))?;

    // Analyze both optimization levels
    let mut analyzer = EnhancedPerformanceAnalyzer::new();
    
    let baseline_result = analyzer.analyze_compilation(&source, input_file, baseline).await?;
    let target_result = analyzer.analyze_compilation(&source, input_file, target).await?;

    // Generate comparison report
    let report = generate_comparison_report(&baseline_result, &target_result, baseline, target);
    println!("{}", report);

    Ok(())
async fn run_profiling(input_file: &str, profile_passes: bool) -> Result<()> {
    info!("Profiling compilation for {}", input_file);

    if profile_passes {
        // Profile individual compiler passes
        let mut pass_manager = CompilerPassManager::create_standard_pipeline(OptimizationLevel::O3)?;
        
        // Create test program (simplified)
        let mut test_program = create_test_program(input_file)?;
        
        let start_time = Instant::now();
        pass_manager.run_all_passes(&mut test_program)?;
        let total_time = start_time.elapsed();
        
        let stats = pass_manager.get_stats();
        let pass_info = pass_manager.get_pass_info();
        
        println!("Compiler Pass Profiling Results");
        println!("===============================");
        println!("Total time: {:?}", total_time);
        println!("Passes run: {}", stats.total_passes_run);
        println!();
        
        for (i, info) in pass_info.iter().enumerate() {
            println!("Pass {}: {} ({:?})", i + 1, info.name, info.category);
            println!("  Description: {}", info.description);
            if !info.dependencies.is_empty() {
                println!("  Dependencies: {:?}", info.dependencies);
            }
            println!();
        }
    } else {
        // Profile overall compilation performance
        let source = fs::read_to_string(input_file)
            .map_err(|e| cursed::error::CursedError::General(format!("Failed to read input file: {}", e)))?;

        let mut analyzer = EnhancedPerformanceAnalyzer::new();
        let result = analyzer.analyze_compilation(&source, input_file, OptimizationLevel::O2).await?;
        
        println!("Compilation Profiling Results");
        println!("=============================");
        println!("Total time: {:?}", result.summary.total_time);
        println!("Performance score: {:.1}/100", result.summary.performance_score);
        println!("Efficiency rating: {:?}", result.summary.efficiency_rating);
        println!();
        
        if !result.bottlenecks.is_empty() {
            println!("Performance Bottlenecks:");
            for bottleneck in &result.bottlenecks {
                println!("  - {} ({}% impact)", bottleneck.description, bottleneck.impact_percentage);
            }
            println!();
        if !result.recommendations.is_empty() {
            println!("Optimization Recommendations:");
            for (i, rec) in result.recommendations.iter().enumerate() {
                    i + 1, rec.title, rec.priority, rec.expected_improvement * 100.0);
            }
        }
    Ok(())
async fn run_single_benchmark(input: &str, opt_level: OptimizationLevel) -> Result<Duration> {
    let start_time = Instant::now();
    
    // Simulate compilation with different optimization levels
    // In a real implementation, this would call the actual compiler
    let source = fs::read_to_string(input)
        .map_err(|e| cursed::error::CursedError::General(format!("Failed to read input: {}", e)))?;
    
    let mut analyzer = EnhancedPerformanceAnalyzer::new();
    let _result = analyzer.analyze_compilation(&source, input, opt_level).await?;
    
    Ok(start_time.elapsed())
fn parse_optimization_level(level_str: &str) -> Result<OptimizationLevel> {
    match level_str.to_uppercase().as_str() {
    }
}

fn generate_text_report(analysis: &cursed::optimization::EnhancedAnalysisResult) -> String {
    let mut report = String::new();
    
    report.push_str("# CURSED Optimization Analysis Report\n\n");
    report.push_str(&format!("**Total Time**: {:?}\n", analysis.summary.total_time));
    report.push_str(&format!("**Performance Score**: {:.1}/100\n", analysis.summary.performance_score));
    report.push_str(&format!("**Efficiency Rating**: {:?}\n", analysis.summary.efficiency_rating));
    
    if let Some(ref bottleneck) = analysis.summary.primary_bottleneck {
        report.push_str(&format!("**Primary Bottleneck**: {}\n", bottleneck));
    if let Some(ref recommendation) = analysis.summary.top_recommendation {
        report.push_str(&format!("**Top Recommendation**: {}\n", recommendation));
    report.push_str(&format!("**Improvement Potential**: {:.1}%\n\n", analysis.summary.improvement_potential * 100.0));
    
    // Phase analysis
    if !analysis.phase_analysis.is_empty() {
        report.push_str("## Compilation Phase Analysis\n\n");
        for (phase, metrics) in &analysis.phase_analysis {
            report.push_str(&format!("### {:?}\n", phase));
            report.push_str(&format!("- Execution time: {:?}\n", metrics.execution_time));
            report.push_str(&format!("- Memory usage: {} bytes\n", metrics.memory_usage));
            report.push_str(&format!("- CPU utilization: {:.1}%\n", metrics.cpu_utilization));
            report.push_str(&format!("- Efficiency score: {:.2}\n", metrics.efficiency_score));
            
            if !metrics.issues.is_empty() {
                report.push_str("- Issues:\n");
                for issue in &metrics.issues {
                        issue.issue_type, issue.severity, issue.description));
                }
            }
            report.push_str("\n");
        }
    }
    
    // Bottlenecks
    if !analysis.bottlenecks.is_empty() {
        report.push_str("## Performance Bottlenecks\n\n");
        for bottleneck in &analysis.bottlenecks {
            report.push_str(&format!("### {} ({}% impact)\n", bottleneck.description, bottleneck.impact_percentage));
            report.push_str(&format!("- Phase: {:?}\n", bottleneck.phase));
            report.push_str(&format!("- Severity: {}/10\n", bottleneck.severity));
            report.push_str(&format!("- Fix complexity: {:?}\n", bottleneck.fix_complexity));
            
            if !bottleneck.solutions.is_empty() {
                report.push_str("- Solutions:\n");
                for solution in &bottleneck.solutions {
                    report.push_str(&format!("  - {}\n", solution));
                }
            }
            report.push_str("\n");
        }
    }
    
    // Recommendations
    if !analysis.recommendations.is_empty() {
        report.push_str("## Optimization Recommendations\n\n");
        for (i, rec) in analysis.recommendations.iter().enumerate() {
            report.push_str(&format!("### {}. {} (Priority {})\n", i + 1, rec.title, rec.priority));
            report.push_str(&format!("{}\n\n", rec.description));
            report.push_str(&format!("- Expected improvement: {:.1}%\n", rec.expected_improvement * 100.0));
            report.push_str(&format!("- Effort level: {:?}\n", rec.effort_level));
            report.push_str(&format!("- Confidence: {:.1}%\n", rec.confidence * 100.0));
            
            if !rec.actions.is_empty() {
                report.push_str("- Actions:\n");
                for action in &rec.actions {
                    report.push_str(&format!("  - {:?}: {}\n", action.action_type, action.description));
                }
            }
            report.push_str("\n");
        }
    }
    
    report
fn generate_json_report(analysis: &cursed::optimization::EnhancedAnalysisResult) -> Result<String> {
    serde_json::to_string_pretty(analysis)
        .map_err(|e| cursed::error::CursedError::General(format!("JSON serialization failed: {}", e)))
#[derive(Debug)]
struct BenchmarkResult {
fn generate_benchmark_report(results: &[BenchmarkResult]) -> String {
    let mut report = String::new();
    
    report.push_str("# CURSED Optimization Benchmark Report\n\n");
    
    if results.is_empty() {
        report.push_str("No successful benchmark runs.\n");
        return report;
    // Find baseline (O0) for comparison
    let baseline = results.iter().find(|r| matches!(r.optimization_level, OptimizationLevel::O0));
    
    report.push_str("## Results\n\n");
    report.push_str("| Level | Average Time | Success Rate | Speedup |\n");
    report.push_str("|-------|--------------|--------------|--------|\n");
    
    for result in results {
        let success_rate = (result.successful_runs as f64 / result.total_iterations as f64) * 100.0;
        let speedup = if let Some(base) = baseline {
            base.average_time.as_secs_f64() / result.average_time.as_secs_f64()
        } else {
            1.0
        
        report.push_str(&format!(
            speedup
        ));
    report.push_str("\n");
    
    // Analysis
    if let Some(fastest) = results.iter().min_by_key(|r| r.average_time) {
            fastest.optimization_level, fastest.average_time));
    if let Some(baseline) = baseline {
        if let Some(best) = results.iter().min_by_key(|r| r.average_time) {
            let improvement = baseline.average_time.as_secs_f64() / best.average_time.as_secs_f64();
            report.push_str(&format!("**Best improvement**: {:.2}x over O0\n", improvement));
        }
    }
    
    report
fn generate_comparison_report(
) -> String {
    let mut report = String::new();
    
    report.push_str("# CURSED Optimization Comparison Report\n\n");
    report.push_str(&format!("Comparing {:?} vs {:?}\n\n", baseline_level, target_level));
    
    // Performance comparison
    report.push_str("## Performance Comparison\n\n");
    report.push_str("| Metric | Baseline | Target | Change |\n");
    report.push_str("|--------|----------|--------|---------|\n");
    
    let time_change = (target.summary.total_time.as_secs_f64() - baseline.summary.total_time.as_secs_f64()) 
        / baseline.summary.total_time.as_secs_f64() * 100.0;
    report.push_str(&format!(
        time_change
    ));
    
    let score_change = target.summary.performance_score - baseline.summary.performance_score;
    report.push_str(&format!(
        score_change
    ));
    
    let improvement_change = (target.summary.improvement_potential - baseline.summary.improvement_potential) * 100.0;
    report.push_str(&format!(
        improvement_change
    ));
    
    report.push_str("\n");
    
    // Summary
    report.push_str("## Summary\n\n");
    if score_change > 0.0 {
        report.push_str(&format!("✅ Performance improved by {:.1} points\n", score_change));
    } else if score_change < 0.0 {
        report.push_str(&format!("❌ Performance decreased by {:.1} points\n", -score_change));
    } else {
        report.push_str("➡️ Performance unchanged\n");
    if time_change < 0.0 {
        report.push_str(&format!("✅ Compilation time improved by {:.1}%\n", -time_change));
    } else if time_change > 0.0 {
        report.push_str(&format!("❌ Compilation time increased by {:.1}%\n", time_change));
    report
fn create_test_program(input_file: &str) -> Result<cursed::ast::Program> {
    // Create a simple test program for profiling
    // In a real implementation, this would parse the actual input file
    Ok(cursed::ast::Program {
        functions: vec![
            cursed::ast::Function {
                body: vec![
                    cursed::ast::Statement::VariableDeclaration(cursed::ast::VariableDeclaration {
            }
    })
}
