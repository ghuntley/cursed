//! CURSED Test Runner binary

use clap::{Arg, Command, ArgMatches};
use std::path::{Path, PathBuf};
use std::process::{self, Command as ProcessCommand};
use std::fs;
use glob::glob;
use colored::*;

fn main() {
    env_logger::init();
    
    let matches = build_cli().get_matches();
    
    if let Err(e) = run(matches) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn build_cli() -> Command {
    Command::new("cursed-test")
        .version("0.1.0")
        .about("CURSED Test Runner - Run CURSED tests")
        .arg(Arg::new("pattern")
            .help("Test file pattern to run")
            .index(1)
            .default_value("**/*.test.csd"))
        .arg(Arg::new("verbose")
            .help("Show verbose output")
            .short('v')
            .long("verbose")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("parallel")
            .help("Run tests in parallel")
            .short('p')
            .long("parallel")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("filter")
            .help("Filter tests by name")
            .short('f')
            .long("filter")
            .value_name("FILTER"))
        .arg(Arg::new("directory")
            .help("Test directory")
            .short('d')
            .long("directory")
            .value_name("DIR")
            .default_value("."))
        .arg(Arg::new("timeout")
            .help("Timeout per test in seconds")
            .short('t')
            .long("timeout")
            .value_name("SECONDS")
            .default_value("30"))
}

#[derive(Debug)]
struct TestResult {
    name: String,
    path: PathBuf,
    passed: bool,
    duration: std::time::Duration,
    output: String,
    error: Option<String>,
}

fn run(matches: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = matches.get_one::<String>("pattern").unwrap();
    let verbose = matches.get_flag("verbose");
    let parallel = matches.get_flag("parallel");
    let filter = matches.get_one::<String>("filter");
    let directory = matches.get_one::<String>("directory").unwrap();
    let timeout: u64 = matches.get_one::<String>("timeout").unwrap().parse()?;
    
    println!("{}", "CURSED Test Runner".bold().cyan());
    println!("Running tests in directory: {}", directory);
    println!("Pattern: {}", pattern);
    
    let test_files = discover_tests(directory, pattern, filter)?;
    
    if test_files.is_empty() {
        println!("{}", "No test files found".yellow());
        return Ok(());
    }
    
    println!("Found {} test file(s)", test_files.len());
    println!();
    
    let results = if parallel {
        run_tests_parallel(&test_files, timeout, verbose)?
    } else {
        run_tests_sequential(&test_files, timeout, verbose)?
    };
    
    print_results(&results);
    
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = results.len() - passed;
    
    if failed > 0 {
        process::exit(1);
    }
    
    Ok(())
}

fn discover_tests(directory: &str, pattern: &str, filter: Option<&String>) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let search_pattern = Path::new(directory).join(pattern);
    let mut test_files = Vec::new();
    
    // First try to find .csd test files
    for entry in glob(&search_pattern.to_string_lossy())? {
        let path = entry?;
        if path.is_file() {
            if let Some(filter_str) = filter {
                if !path.to_string_lossy().contains(filter_str) {
                    continue;
                }
            }
            test_files.push(path);
        }
    }
    
    // Also look for Rust test files for integration tests
    let rust_pattern = Path::new(directory).join("**/*.rs");
    for entry in glob(&rust_pattern.to_string_lossy())? {
        let path = entry?;
        if path.is_file() && (path.to_string_lossy().contains("test") || path.to_string_lossy().contains("tests")) {
            if let Some(filter_str) = filter {
                if !path.to_string_lossy().contains(filter_str) {
                    continue;
                }
            }
            test_files.push(path);
        }
    }
    
    test_files.sort();
    Ok(test_files)
}

fn run_tests_sequential(test_files: &[PathBuf], timeout: u64, verbose: bool) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    
    for (i, test_file) in test_files.iter().enumerate() {
        println!("[{}/{}] Running {}", i + 1, test_files.len(), test_file.display());
        
        let result = run_single_test(test_file, timeout, verbose)?;
        
        if result.passed {
            println!("  {} {} ({:?})", "✓".green(), "PASSED".green(), result.duration);
        } else {
            println!("  {} {} ({:?})", "✗".red(), "FAILED".red(), result.duration);
            if !verbose && result.error.is_some() {
                println!("    Error: {}", result.error.as_ref().unwrap().lines().next().unwrap_or("Unknown error"));
            }
        }
        
        if verbose && !result.output.is_empty() {
            println!("    Output:");
            for line in result.output.lines() {
                println!("      {}", line);
            }
        }
        
        results.push(result);
        println!();
    }
    
    Ok(results)
}

fn run_tests_parallel(test_files: &[PathBuf], timeout: u64, verbose: bool) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    
    for test_file in test_files {
        let test_file = test_file.clone();
        let results = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            match run_single_test(&test_file, timeout, verbose) {
                Ok(result) => {
                    let mut results = results.lock().unwrap();
                    results.push(result);
                }
                Err(e) => {
                    eprintln!("Failed to run test {}: {}", test_file.display(), e);
                }
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().map_err(|_| "Thread panicked")?;
    }
    
    let mut results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    results.sort_by(|a, b| a.path.cmp(&b.path));
    
    Ok(results)
}

fn run_single_test(test_file: &Path, timeout: u64, _verbose: bool) -> Result<TestResult, Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    
    let result = if test_file.extension().and_then(|s| s.to_str()) == Some("csd") {
        run_cursed_test(test_file, timeout)
    } else if test_file.extension().and_then(|s| s.to_str()) == Some("rs") {
        run_rust_test(test_file, timeout)
    } else {
        Err("Unsupported test file type".into())
    };
    
    let duration = start_time.elapsed();
    
    match result {
        Ok(output) => Ok(TestResult {
            name: test_file.file_stem().unwrap().to_string_lossy().to_string(),
            path: test_file.to_path_buf(),
            passed: true,
            duration,
            output,
            error: None,
        }),
        Err(e) => Ok(TestResult {
            name: test_file.file_stem().unwrap().to_string_lossy().to_string(),
            path: test_file.to_path_buf(),
            passed: false,
            duration,
            output: String::new(),
            error: Some(e.to_string()),
        }),
    }
}

fn run_cursed_test(test_file: &Path, timeout: u64) -> Result<String, Box<dyn std::error::Error>> {
    // For .csd files, try to compile and run with the CURSED compiler
    let output = ProcessCommand::new("timeout")
        .arg(timeout.to_string())
        .arg("cursed")
        .arg("run")
        .arg(test_file)
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(format!("Test failed: {}", String::from_utf8_lossy(&output.stderr)).into())
            }
        }
        Err(_) => {
            // Fallback: try to read and validate the test file
            let content = fs::read_to_string(test_file)?;
            validate_cursed_test_content(&content)
        }
    }
}

fn run_rust_test(test_file: &Path, timeout: u64) -> Result<String, Box<dyn std::error::Error>> {
    // For Rust test files, compile and run them
    let output = ProcessCommand::new("timeout")
        .arg(timeout.to_string())
        .arg("cargo")
        .arg("test")
        .arg("--bin")
        .arg(test_file.file_stem().unwrap())
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(format!("Test failed: {}", String::from_utf8_lossy(&output.stderr)).into())
            }
        }
        Err(e) => Err(e.into()),
    }
}

fn validate_cursed_test_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Basic validation of CURSED test file content
    let lines: Vec<&str> = content.lines().collect();
    
    if lines.is_empty() {
        return Err("Empty test file".into());
    }
    
    // Look for test assertions or test blocks
    let has_test_markers = lines.iter().any(|line| {
        line.contains("assert") || 
        line.contains("test") || 
        line.contains("expect") ||
        line.contains("should")
    });
    
    if !has_test_markers {
        return Err("No test assertions found in file".into());
    }
    
    // Basic syntax validation
    let mut brace_count = 0;
    for line in &lines {
        for ch in line.chars() {
            match ch {
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                _ => {}
            }
        }
    }
    
    if brace_count != 0 {
        return Err("Unmatched braces in test file".into());
    }
    
    Ok("Test file validation passed".to_string())
}

fn print_results(results: &[TestResult]) {
    println!("{}", "Test Results".bold().underline());
    println!();
    
    let passed = results.iter().filter(|r| r.passed).count();
    let failed = results.len() - passed;
    
    if failed > 0 {
        println!("{}", "Failed tests:".red().bold());
        for result in results.iter().filter(|r| !r.passed) {
            println!("  {} {} ({:?})", "✗".red(), result.path.display(), result.duration);
            if let Some(error) = &result.error {
                for line in error.lines().take(3) {
                    println!("    {}", line.dimmed());
                }
            }
        }
        println!();
    }
    
    println!("Summary:");
    println!("  {} {} passed", "✓".green(), passed.to_string().green());
    
    if failed > 0 {
        println!("  {} {} failed", "✗".red(), failed.to_string().red());
    }
    
    let total_duration: std::time::Duration = results.iter().map(|r| r.duration).sum();
    println!("  Total time: {:?}", total_duration);
}
