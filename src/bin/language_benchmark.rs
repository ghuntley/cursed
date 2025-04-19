//! Comprehensive cross-language benchmark runner for the CURSED programming language

use cursed::benchmark;
use cursed::benchmark::language_comparison::{Language, Algorithm};
use cursed::benchmark::BenchmarkReporter;
use cursed::benchmark::harness::{BenchmarkSuite, Benchmark, BenchmarkConfig};
use tracing::{instrument, info, debug, Level};
use tracing_subscriber::FmtSubscriber;
use std::env;
use std::process;
use std::path::Path;
use std::fs;
use chrono;
use std::time::SystemTime;

/// Run a comprehensive benchmark suite that includes all languages and all benchmarks
#[instrument(skip_all, fields(suite_name = "comprehensive"))]
fn run_comprehensive_benchmark_suite() -> benchmark::harness::BenchmarkResults {
    info!("Creating comprehensive language comparison benchmark suite");
    
    // Create a new benchmark suite
    let mut suite = BenchmarkSuite::new("comprehensive_language_comparison", 
                                      "Comprehensive language comparison benchmark suite");
    
    // Get all available languages from the benchmark directory
    let benchmark_dir = Path::new("benchmarks");
    let languages = get_available_languages(benchmark_dir);
    debug!("Found {} languages for benchmarking", languages.len());
    
    // Add benchmarks for all available languages
    for language in languages {
        add_language_benchmarks(&mut suite, language);
    }
    
    // Run the suite
    info!("Running comprehensive benchmark suite with {} benchmarks", suite.benchmarks.len());
    let results = suite.run();
    results
}

/// Get all available languages from the benchmark directory
fn get_available_languages(benchmark_dir: &Path) -> Vec<Language> {
    let mut languages = Vec::new();
    
    // Try to read the benchmark directory
    if let Ok(entries) = fs::read_dir(benchmark_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    // Match directory name to Language enum
                    let language = match dir_name.to_lowercase().as_str() {
                        "cursed" => Some(Language::Cursed),
                        "rust" => Some(Language::Rust),
                        "c" => Some(Language::C),
                        "csharp" => Some(Language::CSharp),
                        "go" => Some(Language::Go),
                        "java" => Some(Language::Java),
                        "javascript" => Some(Language::JavaScript),
                        "php" => Some(Language::PHP),
                        "perl" => Some(Language::Perl),
                        "haskell" => Some(Language::Haskell),
                        "swift" => Some(Language::Swift),
                        "pascal" => Some(Language::Pascal),
                        "ocaml" => Some(Language::OCaml),
                        "clojure" => Some(Language::Clojure),
                        "cplusplus" => Some(Language::CPlusPlus),
                        "zig" => Some(Language::Zig),
                        "erlang" => Some(Language::Erlang),
                        "fortran" => Some(Language::Fortran),
                        "ruby" => Some(Language::Ruby),
                        "kotlin" => Some(Language::Kotlin),
                        "python" => Some(Language::Python),
                        _ => None,
                    };
                    
                    if let Some(lang) = language {
                        debug!("Found language directory: {}", dir_name);
                        languages.push(lang);
                    }
                }
            }
        }
    }
    
    languages
}

/// Add benchmarks for a specific language
#[instrument(skip_all, fields(language = ?language))]
fn add_language_benchmarks(suite: &mut BenchmarkSuite, language: Language) {
    // Get all available benchmarks for this language
    let benchmarks = get_available_benchmarks(language);
    debug!("Found {} benchmarks for language {:?}", benchmarks.len(), language);
    
    // Add each benchmark
    for algorithm in benchmarks {
        add_benchmark(suite, language, algorithm);
    }
}

/// Get all available benchmarks for a specific language
fn get_available_benchmarks(language: Language) -> Vec<Algorithm> {
    let mut algorithms = Vec::new();
    let benchmark_dir = Path::new("benchmarks").join(language.to_string().to_lowercase());
    
    // Common algorithms to check for
    let all_algorithms = vec![
        Algorithm::BinaryTrees,
        Algorithm::NBodies,
        Algorithm::Mandelbrot,
        Algorithm::Fannkuch,
        Algorithm::Fasta,
        Algorithm::StringProcessing,
    ];
    
    // Check if each algorithm's file exists
    for algorithm in all_algorithms {
        let file_exists = benchmark_file_exists(language, algorithm);
        if file_exists {
            debug!("Found benchmark {:?} for language {:?}", algorithm, language);
            algorithms.push(algorithm);
        }
    }
    
    algorithms
}

/// Check if a benchmark file exists for a given language and algorithm
fn benchmark_file_exists(language: Language, algorithm: Algorithm) -> bool {
    let file_extension = match language {
        Language::Cursed => "csd",
        Language::Rust => "rs",
        Language::C => "c",
        Language::CSharp => "cs",
        Language::Go => "go",
        Language::Java => "java",
        Language::JavaScript => "js",
        Language::PHP => "php",
        Language::Perl => "pl",
        Language::Haskell => "hs",
        Language::Swift => "swift",
        Language::Pascal => "pas",
        Language::OCaml => "ml",
        Language::Clojure => "clj",
        Language::CPlusPlus => "cpp",
        Language::Zig => "zig",
        Language::Erlang => "erl",
        Language::Fortran => "f90",
        Language::Ruby => "rb",
        Language::Kotlin => "kt",
        Language::Python => "py",
    };
    
    let filename = format!("{}.{}", algorithm.to_string(), file_extension);
    let path = Path::new("benchmarks").join(language.to_string().to_lowercase()).join(&filename);
    
    path.exists()
}

/// Add a specific benchmark to the suite
#[instrument(skip_all, fields(language = ?language, algorithm = ?algorithm))]
fn add_benchmark(suite: &mut BenchmarkSuite, language: Language, algorithm: Algorithm) {
    let name = format!("{:?}_{:?}", language, algorithm);
    let description = format!("{:?} algorithm in {:?}", algorithm, language);
    let name_for_metric = name.clone();
    
    let benchmark = Benchmark::new(&name, &description, move || {
        // Run the language benchmark and wrap the result as metrics
        let (duration, output) = cursed::benchmark::language_comparison::run_language_benchmark(language, algorithm);
        vec![Box::new(cursed::benchmark::metrics::TimingMetric { name: name_for_metric.clone(), duration }) as Box<dyn cursed::benchmark::metrics::Metric>]
    })
    .with_config(BenchmarkConfig {
        iterations: 3,
        warmup: 1,
        ..Default::default()
    });
    
    suite.add_benchmark(benchmark);
}

fn main() {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default tracer");
    
    let args: Vec<String> = env::args().collect();
    
    // Parse arguments
    let output_format = args.get(1).map(|s| s.as_str()).unwrap_or("console");
    let output_file = args.get(2).map(|s| s.as_str()).unwrap_or("language_comparison_results");
    
    // Print usage if help is requested
    if output_format == "help" || output_format == "-h" || output_format == "--help" {
        print_usage(&args[0]);
        process::exit(0);
    }
    
    // Validate arguments
    if !vec!["console", "json", "csv", "markdown"].contains(&output_format) {
        eprintln!("Unknown output format: {}", output_format);
        print_usage(&args[0]);
        process::exit(1);
    }
    
    // Run the comprehensive benchmark suite
    println!("Running comprehensive cross-language benchmark comparison...");
    println!("This will benchmark CURSED against all available languages in the benchmark directory.");
    println!("\nNote: For best results, make sure you've built CURSED in release mode with 'cargo build --release'.");
    
    // Create and run the comprehensive benchmark suite
    let results = run_comprehensive_benchmark_suite();
    
    // Output results based on format
    match output_format {
        "console" => {
            let reporter = benchmark::reporters::ConsoleReporter::verbose();
            reporter.report(&results);
        },
        "json" => {
            let output_path = format!("{}.json", output_file);
            let reporter = benchmark::reporters::JsonReporter::new(&output_path);
            reporter.report(&results);
            println!("Results written to {}", output_path);
        },
        "csv" => {
            let output_path = format!("{}.csv", output_file);
            let reporter = benchmark::reporters::CsvReporter::new(&output_path);
            reporter.report(&results);
            println!("Results written to {}", output_path);
        },
        "markdown" => {
            let output_path = format!("{}.md", output_file);
            generate_markdown_report(&results, &output_path);
            println!("Results written to {}", output_path);
        },
        _ => {
            // This should never happen due to validation above
            eprintln!("Unknown output format: {}", output_format);
            process::exit(1);
        }
    }
}

/// Print usage information
fn print_usage(program_name: &str) {
    println!("CURSED Language Benchmark Comparison Runner");
    println!("Usage: {} [output_format] [output_file]", program_name);
    println!("\nOutput formats:");
    println!("  console     - Output to console (default)");
    println!("  json        - Output to JSON file");
    println!("  csv         - Output to CSV file");
    println!("  markdown    - Output to Markdown file");
    println!("\nExamples:");
    println!("  {} csv results            - Run benchmarks with CSV output to results.csv", program_name);
    println!("  {} markdown benchmark_data - Run benchmarks with markdown output to benchmark_data.md", program_name);
}

/// Generate a markdown report from benchmark results
fn generate_markdown_report(results: &benchmark::harness::BenchmarkResults, output_path: &str) {
    use std::fs::File;
    use std::io::Write;
    
    let mut file = File::create(output_path).expect("Failed to create output file");
    
    // Write header
    writeln!(file, "# CURSED Language Benchmark Comparison").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "This report compares the performance of CURSED with all available languages.").unwrap();
    writeln!(file, "").unwrap();
    
    // For each benchmark, create a section
    for benchmark_result in &results.results {
        // Parse the benchmark name to get language and algorithm
        let parts: Vec<&str> = benchmark_result.name.split('_').collect();
        
        let language = if !parts.is_empty() { parts[0] } else { "Unknown" };
        let algorithm_parts = if parts.len() > 1 { &parts[1..] } else { &[] };
        let algorithm_str = algorithm_parts.join("_");
        
        // Format the algorithm name for display
        let algo_display = if !algorithm_str.is_empty() {
            algorithm_str
        } else {
            String::from("Benchmark")
        };
        
        writeln!(file, "## {} - {}", language, algo_display).unwrap();
        writeln!(file, "").unwrap();
        // Get description from algorithm part
        let description = match algo_display.as_str() {
            "BinaryTrees" => "Allocate and deallocate many binary trees",
            "NBodies" => "N-body simulation of Jovian planets",
            "Mandelbrot" => "Mandelbrot set calculation",
            "Fannkuch" => "Fannkuch redux benchmark",
            "Fasta" => "Generate and write random DNA sequences",
            "StringProcessing" => "String manipulation and processing",
            _ => "Benchmark comparing CURSED with other languages",
        };
        writeln!(file, "{}", description).unwrap();
        writeln!(file, "").unwrap();
        
        // Collect timing metrics
        let mut timing_metrics = Vec::new();
        for metric in &benchmark_result.metrics {
            if metric.name().ends_with("_time") {
                timing_metrics.push(metric);
            }
        }
        
        // Sort by performance (ascending)
        timing_metrics.sort_by(|a, b| {
            match (a.value(), b.value()) {
                (benchmark::metrics::MetricValue::Duration(a_time), 
                 benchmark::metrics::MetricValue::Duration(b_time)) => {
                    a_time.as_secs_f64().partial_cmp(&b_time.as_secs_f64()).unwrap()
                },
                _ => std::cmp::Ordering::Equal
            }
        });
        
        // Create timing comparison table
        if !timing_metrics.is_empty() {
            writeln!(file, "### Execution Time Comparison").unwrap();
            writeln!(file, "").unwrap();
            writeln!(file, "| Language | Time (ms) | Relative Performance |").unwrap();
            writeln!(file, "| --- | ---: | ---: |").unwrap();
            
            let fastest_time = if let benchmark::metrics::MetricValue::Duration(time) = timing_metrics[0].value() {
                time.as_secs_f64()
            } else {
                0.0
            };
            
            for metric in timing_metrics {
                if let benchmark::metrics::MetricValue::Duration(time) = metric.value() {
                    let name = metric.name().replace("_time", "");
                    let relative = time.as_secs_f64() / fastest_time;
                    let name_capitalized = name[0..1].to_uppercase() + &name[1..];
                    writeln!(file, "| {} | {:.2} | {:.2}x |", 
                             name_capitalized,
                             time.as_millis(),
                             relative).unwrap();
                }
            }
            writeln!(file, "").unwrap();
        }
        
        // Collect memory metrics
        let mut memory_metrics = Vec::new();
        for metric in &benchmark_result.metrics {
            if metric.name().ends_with("_memory") {
                memory_metrics.push(metric);
            }
        }
        
        // Sort by memory usage (ascending)
        memory_metrics.sort_by(|a, b| {
            match (a.value(), b.value()) {
                (benchmark::metrics::MetricValue::UInteger(a_mem), 
                 benchmark::metrics::MetricValue::UInteger(b_mem)) => {
                    a_mem.partial_cmp(&b_mem).unwrap()
                },
                _ => std::cmp::Ordering::Equal
            }
        });
        
        // Create memory comparison table
        if !memory_metrics.is_empty() {
            writeln!(file, "### Memory Usage Comparison").unwrap();
            writeln!(file, "").unwrap();
            writeln!(file, "| Language | Memory (MB) | Relative Memory Usage |").unwrap();
            writeln!(file, "| --- | ---: | ---: |").unwrap();
            
            let lowest_memory = if let benchmark::metrics::MetricValue::UInteger(mem) = memory_metrics[0].value() {
                mem
            } else {
                0
            };
            
            for metric in memory_metrics {
                if let benchmark::metrics::MetricValue::UInteger(mem) = metric.value() {
                    let name = metric.name().replace("_memory", "");
                    let relative = mem as f64 / lowest_memory as f64;
                    let name_capitalized = name[0..1].to_uppercase() + &name[1..];
                    writeln!(file, "| {} | {:.2} | {:.2}x |", 
                             name_capitalized,
                             mem as f64 / (1024.0 * 1024.0),
                             relative).unwrap();
                }
            }
            writeln!(file, "").unwrap();
        }
    }
    
    // Add timestamp
    let now = SystemTime::now();
    let now_str = match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => format!("{}", n.as_secs()),
        Err(_) => "Unknown time".to_string(),
    };
    writeln!(file, "\n---\nBenchmark run completed at {}", now_str).unwrap();
}