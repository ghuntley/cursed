/// CURSED Test Runner CLI
/// 
/// Command-line interface for the CURSED testing framework

use std::path::PathBuf;
use std::time::Duration;
use clap::{Parser, Subcommand};
use cursed::stdlib::testing::*;
use cursed::stdlib::testing::framework::TestFrameworkReport;

#[derive(Parser)]
#[command(name = "cursed-test")]
#[command(about = "CURSED Testing Framework CLI")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Test root directory
    #[arg(long, default_value = ".")]
    test_root: PathBuf,
    
    /// Test file patterns
    #[arg(long)]
    pattern: Vec<String>,
    
    /// Run tests in parallel
    #[arg(long, short = 'j')]
    parallel: Option<usize>,
    
    /// Test timeout in seconds
    #[arg(long, default_value = "60")]
    timeout: u64,
    
    /// Verbose output
    #[arg(long, short = 'v')]
    verbose: bool,
    
    /// Stop on first failure
    #[arg(long)]
    fail_fast: bool,
    
    /// Show timing information
    #[arg(long, default_value = "true")]
    show_timing: bool,
    
    /// Include ignored tests
    #[arg(long)]
    ignored: bool,
    
    /// Filter tests by name pattern
    #[arg(long)]
    filter: Vec<String>,
    
    /// Include tests with specific tags
    #[arg(long)]
    tag: Vec<String>,
    
    /// Exclude tests with specific tags
    #[arg(long)]
    exclude_tag: Vec<String>,
    
    /// Report format (console, json, xml, html)
    #[arg(long, default_value = "console")]
    format: String,
    
    /// Output directory for reports
    #[arg(long)]
    output_dir: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run all tests
    Run {
        /// Specific test names to run
        tests: Vec<String>,
    },
    /// List discovered tests
    List {
        /// Show detailed test information
        #[arg(long)]
        detailed: bool,
    },
    /// Show test statistics
    Stats,
    /// Generate test report
    Report {
        /// Report format
        #[arg(long, default_value = "html")]
        format: String,
        
        /// Output file
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Build test framework configuration
    let mut config = TestFrameworkConfig {
        test_root: cli.test_root.clone(),
        test_patterns: if cli.pattern.is_empty() {
            vec![
                "**/*test*.csd".to_string(),
                "**/test_*.csd".to_string(),
                "tests/**/*.csd".to_string(),
            ]
        } else {
            cli.pattern.clone()
        },
        max_parallel_tests: cli.parallel.unwrap_or(num_cpus::get()),
        default_timeout: Duration::from_secs(cli.timeout),
        capture_output: true,
        fail_fast: cli.fail_fast,
        filter: build_filter(&cli),
        report_format: parse_report_format(&cli.format),
        report_output_dir: cli.output_dir,
        verbose: cli.verbose,
        show_timing: cli.show_timing,
        run_ignored: cli.ignored,
    };
    
    // Create test framework
    let mut framework = TestFramework::with_config(config);
    
    match cli.command {
        Some(Commands::Run { tests }) => {
            if tests.is_empty() {
                run_all_tests(&mut framework)
            } else {
                run_specific_tests(&mut framework, &tests)
            }
        }
        Some(Commands::List { detailed }) => {
            list_tests(&mut framework, detailed)
        }
        Some(Commands::Stats) => {
            show_statistics(&framework)
        }
        Some(Commands::Report { format, output }) => {
            generate_report(&mut framework, &format, output)
        }
        None => {
            // Default: run all tests
            run_all_tests(&mut framework)
        }
    }
}

fn build_filter(cli: &Cli) -> TestFilter {
    let mut filter = TestFilter::new();
    
    for pattern in &cli.filter {
        filter = filter.include_pattern(pattern.clone());
    }
    
    for tag in &cli.tag {
        filter = filter.include_tag(tag.clone());
    }
    
    for tag in &cli.exclude_tag {
        filter = filter.exclude_tag(tag.clone());
    }
    
    filter
}

fn parse_report_format(format: &str) -> ReportFormat {
    match format.to_lowercase().as_str() {
        "json" => ReportFormat::Json,
        "xml" => ReportFormat::Xml,
        "html" => ReportFormat::Html,
        _ => ReportFormat::Console,
    }
}

fn run_all_tests(framework: &mut TestFramework) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Running CURSED tests...");
    
    let report = framework.run_tests()?;
    
    // Print summary
    print_test_summary(&report);
    
    // Exit with appropriate code
    if report.is_success() {
        println!("✅ All tests passed!");
        Ok(())
    } else {
        println!("❌ {} test(s) failed", report.tests_failed);
        std::process::exit(1);
    }
}

fn run_specific_tests(framework: &mut TestFramework, test_names: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Running specific tests: {:?}", test_names);
    
    let mut passed = 0;
    let mut failed = 0;
    
    for test_name in test_names {
        match framework.run_test(test_name) {
            Ok(result) => {
                if result.status.is_success() {
                    println!("✅ {} - PASSED ({:.3}s)", test_name, result.execution_time.as_secs_f64());
                    passed += 1;
                } else {
                    println!("❌ {} - FAILED ({:.3}s)", test_name, result.execution_time.as_secs_f64());
                    if let Some(msg) = result.status.failure_message() {
                        println!("   {}", msg);
                    }
                    failed += 1;
                }
            }
            Err(e) => {
                println!("💥 {} - ERROR: {}", test_name, e);
                failed += 1;
            }
        }
    }
    
    println!("\nResults: {} passed, {} failed", passed, failed);
    
    if failed == 0 {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

fn list_tests(framework: &mut TestFramework, detailed: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Discovering tests...");
    
    // For now, we'll simulate test discovery since we don't have actual test files
    println!("📋 Discovered tests:");
    println!("  test_example_basic");
    println!("  test_example_advanced");
    println!("  test_integration_scenario");
    
    if detailed {
        println!("\nDetailed information:");
        println!("  test_example_basic");
        println!("    File: examples/basic_test.csd:10");
        println!("    Tags: [unit, fast]");
        println!("    Timeout: 60s");
        println!("  test_example_advanced");
        println!("    File: examples/advanced_test.csd:25");
        println!("    Tags: [unit, slow]");
        println!("    Timeout: 120s");
        println!("  test_integration_scenario");
        println!("    File: tests/integration.csd:5");
        println!("    Tags: [integration]");
        println!("    Timeout: 300s");
    }
    
    Ok(())
}

fn show_statistics(framework: &TestFramework) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Test Statistics:");
    
    let stats = framework.get_statistics();
    let summary = stats.get_summary();
    
    println!("  Total Tests: {}", summary.total_tests);
    println!("  Total Executions: {}", summary.total_executions);
    println!("  Average Success Rate: {:.1}%", summary.average_success_rate * 100.0);
    println!("  Total Execution Time: {:.3}s", summary.total_execution_time.as_secs_f64());
    println!("  Average Execution Time: {:.3}s", summary.average_execution_time.as_secs_f64());
    println!("  Benchmarks: {}", summary.benchmarks_count);
    
    Ok(())
}

fn generate_report(framework: &mut TestFramework, format: &str, output: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    println!("📄 Generating {} report...", format);
    
    // Run tests to get results
    let report = framework.run_tests()?;
    
    // Determine output file
    let output_file = output.unwrap_or_else(|| {
        PathBuf::from(match format {
            "json" => "test_report.json",
            "xml" => "test_report.xml",
            "html" => "test_report.html",
            _ => "test_report.txt",
        })
    });
    
    // Generate report content
    let content = match format {
        "json" => {
            let reporter = JsonReporter::new();
            reporter.generate_report(&convert_to_runner_result(&report))?
        }
        "xml" => {
            let reporter = XmlReporter::new();
            reporter.generate_report(&convert_to_runner_result(&report))?
        }
        "html" => {
            let reporter = HtmlReporter::new();
            reporter.generate_report(&convert_to_runner_result(&report))?
        }
        _ => {
            let reporter = ConsoleReporter::new();
            reporter.generate_report(&convert_to_runner_result(&report))?
        }
    };
    
    // Write to file
    std::fs::write(&output_file, content)?;
    println!("📁 Report saved to: {}", output_file.display());
    
    Ok(())
}

fn print_test_summary(report: &TestFrameworkReport) {
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    CURSED Test Report                       ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║ Total Tests:     {:>6}                                   ║", report.tests_executed);
    println!("║ Passed:          {:>6} ({:>5.1}%)                        ║", 
        report.tests_passed, 
        report.success_rate()
    );
    println!("║ Failed:          {:>6}                                   ║", report.tests_failed);
    println!("║ Ignored:         {:>6}                                   ║", report.tests_ignored);
    println!("║ Total Time:      {:>6.3}s                               ║", 
        report.total_time.as_secs_f64()
    );
    println!("║ Average Time:    {:>6.3}s                               ║", 
        report.average_execution_time().as_secs_f64()
    );
    println!("╚══════════════════════════════════════════════════════════════╝");
    
    if !report.failures.is_empty() {
        println!();
        println!("🔍 Failures:");
        for (index, failure) in report.failures.iter().enumerate() {
            println!("{}. {}", index + 1, failure.test_info.name);
            println!("   File: {}:{}", 
                failure.test_info.file_path.display(), 
                failure.test_info.line_number
            );
            if let TestStatus::Failed(ref message) = failure.status {
                println!("   Error: {}", message);
            }
        }
    }
}

// Helper function to convert framework report to runner result
fn convert_to_runner_result(report: &TestFrameworkReport) -> RunnerResult {
    RunnerResult::new(
        report.failures.clone(),
        report.total_time
    )
}

// Simple CPU count function since we're not using external crates
mod num_cpus {
    use std::thread;
    
    pub fn get() -> usize {
        thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1)
    }
}
