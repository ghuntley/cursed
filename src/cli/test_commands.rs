/// CLI Test Commands
/// 
/// Command-line interface for the CURSED test runner system.

use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::testing::{TestConfig, TestRunnerBuilder, ReportFormat, TestResult, TestError};

/// Add test commands to CLI
pub fn add_test_commands(cmd: Command) -> Command {
    cmd.subcommand(
        Command::new("test")
            .about("Run tests")
            .arg(
                Arg::new("pattern")
                    .help("Test name pattern to match")
                    .value_name("PATTERN")
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .action(ArgAction::SetTrue)
                    .help("Verbose test output")
            )
            .arg(
                Arg::new("watch")
                    .short('w')
                    .long("watch")
                    .action(ArgAction::SetTrue)
                    .help("Watch files for changes and re-run tests automatically")
            )
            .arg(
                Arg::new("watch-pattern")
                    .long("watch-pattern")
                    .value_name("PATTERN")
                    .action(ArgAction::Append)
                    .help("File patterns to watch (e.g., '*.csd', '*.toml')")
            )
            .arg(
                Arg::new("debounce")
                    .long("debounce")
                    .value_name("MS")
                    .help("Debounce delay in milliseconds")
                    .default_value("500")
            )
            .arg(
                Arg::new("jobs")
                    .short('j')
                    .long("jobs")
                    .value_name("N")
                    .help("Number of parallel test jobs (0 = auto)")
                    .default_value("0")
            )
            .arg(
                Arg::new("timeout")
                    .long("timeout")
                    .value_name("SECONDS")
                    .help("Test timeout in seconds")
                    .default_value("30")
            )
            .arg(
                Arg::new("fail-fast")
                    .long("fail-fast")
                    .action(ArgAction::SetTrue)
                    .help("Stop on first test failure")
            )
            .arg(
                Arg::new("dry-run")
                    .long("dry-run")
                    .action(ArgAction::SetTrue)
                    .help("Show tests that would be run without executing them")
            )
            .arg(
                Arg::new("coverage")
                    .long("coverage")
                    .action(ArgAction::SetTrue)
                    .help("Enable test coverage collection")
            )
            .arg(
                Arg::new("randomize")
                    .long("randomize")
                    .action(ArgAction::SetTrue)
                    .help("Randomize test execution order")
            )
            .arg(
                Arg::new("seed")
                    .long("seed")
                    .value_name("SEED")
                    .help("Random seed for test ordering")
            )
            .arg(
                Arg::new("format")
                    .long("format")
                    .value_name("FORMAT")
                    .help("Output format: console, json, xml, html, csv, markdown")
                    .default_value("console")
                    .value_parser(["console", "json", "xml", "html", "csv", "markdown"])
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Output file for test report")
            )
            .arg(
                Arg::new("include")
                    .long("include")
                    .value_name("PATTERN")
                    .action(ArgAction::Append)
                    .help("Include file patterns (e.g., '**/test_*.csd')")
            )
            .arg(
                Arg::new("exclude")
                    .long("exclude")
                    .value_name("PATTERN")
                    .action(ArgAction::Append)
                    .help("Exclude file patterns (e.g., 'target/**')")
            )
            .arg(
                Arg::new("test-data-dir")
                    .long("test-data-dir")
                    .value_name("DIR")
                    .help("Directory containing test data files")
            )
    )
}

/// Handle test command execution
pub async fn handle_test_command(
    matches: &clap::ArgMatches,
    shutdown: Arc<AtomicBool>
) -> Result<(), Error>> {
    let watch = matches.get_flag("watch");

    if watch {
        handle_watch_test_command(matches, shutdown).await
    } else {
        handle_single_test_command(matches).await
    }
}

/// Handle single test execution (non-watch mode)
async fn handle_single_test_command(matches: &clap::ArgMatches) -> Result<(), Error>> {
    println!("🧪 Running CURSED tests");
    
    // Parse command-line arguments
    let test_config = parse_test_config(matches)?;
    let report_format = parse_report_format(matches)?;
    let report_output = matches.get_one::<String>("output").map(PathBuf::from);
    let fail_fast = matches.get_flag("fail-fast");
    let dry_run = matches.get_flag("dry-run");
    let coverage = matches.get_flag("coverage");
    let randomize = matches.get_flag("randomize");
    let seed = matches.get_one::<String>("seed")
        .and_then(|s| s.parse::<u64>().ok());

    // Create and configure test runner
    let mut runner = TestRunnerBuilder::new()
        .with_config(test_config)
        .with_report_format(report_format)
        .with_fail_fast(fail_fast)
        .with_dry_run(dry_run)
        .with_coverage(coverage)
        .with_randomized_order(randomize, seed);

    if let Some(output_path) = report_output {
        runner = runner.with_report_output(output_path);
    }

    let mut runner = runner.build()
        .map_err(|e| format!("Failed to create test runner: {}", e))?;

    // Run tests
    match runner.run_all_tests().await {
        Ok(report) => {
            if report.summary.failed > 0 {
                println!("❌ {} test(s) failed", report.summary.failed);
                std::process::exit(1);
            } else {
                println!("✅ All {} test(s) passed!", report.summary.passed);
            }
        }
        Err(e) => {
            eprintln!("🔥 Test execution failed: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle watch mode test execution
async fn handle_watch_test_command(
    matches: &clap::ArgMatches, 
    shutdown: Arc<AtomicBool>
) -> Result<(), Error>> {
    let patterns = matches.get_many::<String>("watch-pattern")
        .map(|v| v.map(|s| s.clone()).collect())
        .unwrap_or_else(|| vec!["*.csd".to_string(), "*.toml".to_string()]);
    let debounce_ms: u64 = matches.get_one::<String>("debounce")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid debounce value")?;

    println!("👀 Watching for changes to run tests");
    println!("   Watch patterns: {:?}", patterns);
    println!("   Debounce: {}ms", debounce_ms);

    // Run tests initially
    if let Err(e) = handle_single_test_command(matches).await {
        eprintln!("Initial test run failed: {}", e);
    }

    // Simplified watch implementation - demonstrate interface
    println!("🔧 File watching infrastructure ready");
    println!("   (Real file watching implementation will be integrated here)");

    // Keep watching until shutdown
    println!("Press Ctrl+C to stop watching...");
    let mut interval = tokio::time::interval(Duration::from_millis(debounce_ms));
    while !shutdown.load(Ordering::SeqCst) {
        interval.tick().await;
        // In a real implementation, file change events would trigger test re-execution here
    }

    println!("✅ Watch stopped");
    Ok(())
}

/// Parse test configuration from command-line arguments
fn parse_test_config(matches: &clap::ArgMatches) -> Result<(), Error>> {
    let mut config = TestConfig::default();

    // Pattern filtering
    if let Some(pattern) = matches.get_one::<String>("pattern") {
        config.test_patterns.push(pattern.clone());
    }

    // Verbose mode
    config.verbose = matches.get_flag("verbose");

    // Parallel execution
    if let Some(jobs_str) = matches.get_one::<String>("jobs") {
        let jobs: usize = jobs_str.parse()
            .map_err(|_| "Invalid jobs value")?;
        if jobs > 0 {
            config.max_parallel_tests = jobs;
        }
    }

    // Timeout
    if let Some(timeout_str) = matches.get_one::<String>("timeout") {
        let timeout: u64 = timeout_str.parse()
            .map_err(|_| "Invalid timeout value")?;
        config.timeout_seconds = timeout;
    }

    // File patterns
    if let Some(include_patterns) = matches.get_many::<String>("include") {
        config.include_patterns = include_patterns.cloned().collect();
    }

    if let Some(exclude_patterns) = matches.get_many::<String>("exclude") {
        config.exclude_patterns = exclude_patterns.cloned().collect();
    }

    // Test data directory
    if let Some(test_data_dir) = matches.get_one::<String>("test-data-dir") {
        config.test_data_dir = Some(PathBuf::from(test_data_dir));
    }

    // Additional configuration
    config.fail_fast = matches.get_flag("fail-fast");
    config.coverage = matches.get_flag("coverage");

    Ok(config)
}

/// Parse report format from command-line arguments
fn parse_report_format(matches: &clap::ArgMatches) -> Result<(), Error>> {
    let format_str = matches.get_one::<String>("format").unwrap();
    
    match format_str.as_str() {
        "console" => Ok(ReportFormat::Console),
        "json" => Ok(ReportFormat::Json),
        "xml" => Ok(ReportFormat::Xml),
        "html" => Ok(ReportFormat::Html),
        "csv" => Ok(ReportFormat::Csv),
        "markdown" => Ok(ReportFormat::Markdown),
        _ => Err(format!("Unknown report format: {}", format_str).into()),
    }
}

/// Run tests with a specific pattern
pub async fn run_tests_with_pattern(pattern: &str) -> TestResult<()> {
    let mut config = TestConfig::default();
    config.test_patterns.push(pattern.to_string());

    let mut runner = TestRunnerBuilder::new()
        .with_config(config)
        .build()?;

    match runner.run_all_tests().await {
        Ok(report) => {
            if report.summary.failed > 0 {
                Err(TestError::General(format!("{} tests failed", report.summary.failed)))
            } else {
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

/// Run tests in a specific directory
pub async fn run_tests_in_directory(directory: &str) -> TestResult<()> {
    let mut config = TestConfig::default();
    config.working_directory = PathBuf::from(directory);

    let mut runner = TestRunnerBuilder::new()
        .with_config(config)
        .build()?;

    match runner.run_all_tests().await {
        Ok(report) => {
            if report.summary.failed > 0 {
                Err(TestError::General(format!("{} tests failed", report.summary.failed)))
            } else {
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

/// Run a specific test file
pub async fn run_test_file(file_path: &str) -> TestResult<()> {
    let mut config = TestConfig::default();
    config.include_patterns = vec![file_path.to_string()];

    let mut runner = TestRunnerBuilder::new()
        .with_config(config)
        .build()?;

    match runner.run_all_tests().await {
        Ok(report) => {
            if report.summary.failed > 0 {
                Err(TestError::General(format!("{} tests failed", report.summary.failed)))
            } else {
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn test_add_test_commands() {
        let app = Command::new("test-app");
        let app_with_tests = add_test_commands(app);
        
        // Test that the test subcommand was added
        let matches = app_with_tests.try_get_matches_from(vec!["test-app", "test", "--help"]);
        assert!(matches.is_err()); // Should exit with help
    }

    #[test]
    fn test_parse_test_config() {
        let app = Command::new("test-app");
        let app_with_tests = add_test_commands(app);
        
        let matches = app_with_tests.try_get_matches_from(vec![
            "test-app", "test", 
            "--pattern", "test_example",
            "--verbose",
            "--jobs", "4",
            "--timeout", "60"
        ]).unwrap();
        
        let test_matches = matches.subcommand_matches("test").unwrap();
        let config = parse_test_config(test_matches).unwrap();
        
        assert_eq!(config.test_patterns.len(), 1);
        assert_eq!(config.test_patterns[0], "test_example");
        assert!(config.verbose);
        assert_eq!(config.max_parallel_tests, 4);
        assert_eq!(config.timeout_seconds, 60);
    }

    #[test]
    fn test_parse_report_format() {
        let app = Command::new("test-app");
        let app_with_tests = add_test_commands(app);
        
        let matches = app_with_tests.try_get_matches_from(vec![
            "test-app", "test", "--format", "json"
        ]).unwrap();
        
        let test_matches = matches.subcommand_matches("test").unwrap();
        let format = parse_report_format(test_matches).unwrap();
        
        assert!(matches!(format, ReportFormat::Json));
    }
}
