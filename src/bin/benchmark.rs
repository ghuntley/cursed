//! Benchmark runner for the CURSED programming language

use cursed::benchmark;
use cursed::benchmark::language_comparison;
use cursed::benchmark::BenchmarkReporter;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use std::env;
use std::process;

fn main() {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default tracer");
    
    let args: Vec<String> = env::args().collect();
    
    // Print usage if no arguments provided
    if args.len() < 2 {
        println!("CURSED Benchmark Runner");
        println!("Usage: {} <benchmark_suite> [output_format] [output_file]", args[0]);
        println!("\nAvailable benchmark suites:");
        println!("  standard    - Standard language benchmarks");
        println!("  gc          - Garbage collector benchmarks");
        println!("  concurrency - Concurrency benchmarks");
        println!("  language    - Language comparison benchmarks");
        println!("  all         - Run all benchmark suites");
        println!("\nOutput formats:");
        println!("  console     - Output to console (default)");
        println!("  json        - Output to JSON file");
        println!("  csv         - Output to CSV file");
        process::exit(1);
    }
    
    let suite_name = &args[1];
    let output_format = args.get(2).map(|s| s.as_str()).unwrap_or("console");
    let output_file = args.get(3).map(|s| s.as_str()).unwrap_or("benchmark_results");
    
    // Run the requested benchmark suite
    match suite_name.as_str() {
        "standard" => {
            let results = benchmark::run_standard_suite();
            output_results(&results, output_format, output_file);
        },
        "gc" => {
            let results = benchmark::run_gc_suite();
            output_results(&results, output_format, output_file);
        },
        "concurrency" => {
            let results = benchmark::run_concurrency_suite();
            output_results(&results, output_format, output_file);
        },
        "language" => {
            let results = benchmark::run_language_comparison_suite();
            output_results(&results, output_format, output_file);
        },
        "all" => {
            println!("Running all benchmark suites...");
            println!("\n=== Standard Benchmarks ===");
            let std_results = benchmark::run_standard_suite();
            
            println!("\n=== GC Benchmarks ===");
            let gc_results = benchmark::run_gc_suite();
            
            println!("\n=== Concurrency Benchmarks ===");
            let con_results = benchmark::run_concurrency_suite();
            
            println!("\n=== Language Comparison Benchmarks ===");
            let lang_results = benchmark::run_language_comparison_suite();
            
            // Output all results
            output_results(&std_results, output_format, &format!("{}_standard", output_file));
            output_results(&gc_results, output_format, &format!("{}_gc", output_file));
            output_results(&con_results, output_format, &format!("{}_concurrency", output_file));
            output_results(&lang_results, output_format, &format!("{}_language", output_file));
        },
        _ => {
            eprintln!("Unknown benchmark suite: {}", suite_name);
            process::exit(1);
        }
    }
}

fn output_results(results: &benchmark::harness::BenchmarkResults, format: &str, file: &str) {
    match format {
        "console" => {
            let reporter = benchmark::reporters::ConsoleReporter::verbose();
            reporter.report(results);
        },
        "json" => {
            let output_path = format!("{}.json", file);
            let reporter = benchmark::reporters::JsonReporter::new(&output_path);
            reporter.report(results);
            println!("Results written to {}", output_path);
        },
        "csv" => {
            let output_path = format!("{}.csv", file);
            let reporter = benchmark::reporters::CsvReporter::new(&output_path);
            reporter.report(results);
            println!("Results written to {}", output_path);
        },
        _ => {
            eprintln!("Unknown output format: {}", format);
            process::exit(1);
        }
    }
}