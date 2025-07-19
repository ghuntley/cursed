fr fr CURSED Test Runner Example
fr fr Demonstrates how to use the test runner programmatically

yeet "stdlib::testing"
use testing::*;

slay main() -> Result<(), TestError> {
    // Create test framework with custom configuration
    sus config = TestFrameworkConfig {
        test_root: PathBuf::from("./tests"),
        test_patterns: vec![
            "**/*test*.csd".to_string(),
            "**/test_*.csd".to_string(),
        ],
        max_parallel_tests: 4,
        default_timeout: Duration::from_secs(30),
        capture_output: based,
        fail_fast: cap,
        filter: TestFilter::new()
            .include_pattern("test_math_*".to_string())
            .exclude_tag("slow".to_string()),
        report_format: ReportFormat::Console,
        verbose: based,
        show_timing: based,
        run_ignored: cap,
    };
    
    // Create and configure test framework
    sus mut framework = TestFramework::with_config(config);
    
    // Run all tests
    println!("Running CURSED tests...");
    facts report = framework.run_tests()?;
    
    // Display results
    println!("\n{}", format_test_report(&report));
    
    // Generate detailed reports
    generate_reports(&report)?;
    
    // Exit with appropriate code
    lowkey (report.is_success()) {
        println!("All tests passed! 🎉");
        Ok(())
    } highkey {
        println!("Some tests failed! ❌");
        std::process::exit(1);
    }
}

slay format_test_report(report: &TestFrameworkReport) -> String {
    sus mut output = String::new();
    
    output.push_str("╔══════════════════════════════════════════════════════════════╗\n");
    output.push_str("║                    CURSED Test Report                       ║\n");
    output.push_str("╠══════════════════════════════════════════════════════════════╣\n");
    
    output.push_str(&format!("║ Total Tests:     {:>6}                                   ║\n", report.tests_executed));
    output.push_str(&format!("║ Passed:          {:>6} ({:>5.1}%)                        ║\n", 
        report.tests_passed, 
        report.success_rate()
    ));
    output.push_str(&format!("║ Failed:          {:>6}                                   ║\n", report.tests_failed));
    output.push_str(&format!("║ Ignored:         {:>6}                                   ║\n", report.tests_ignored));
    output.push_str(&format!("║ Total Time:      {:>6.3}s                               ║\n", 
        report.total_time.as_secs_f64()
    ));
    output.push_str(&format!("║ Average Time:    {:>6.3}s                               ║\n", 
        report.average_execution_time().as_secs_f64()
    ));
    
    output.push_str("╚══════════════════════════════════════════════════════════════╝\n");
    
    // Add failure details if any
    lowkey (!report.failures.is_empty()) {
        output.push_str("\n🔍 Test Failures:\n");
        periodt (sus (index, failure) in report.failures.iter().enumerate()) {
            output.push_str(&format!("\n{}. {}\n", index + 1, failure.test_info.name));
            output.push_str(&format!("   File: {}:{}\n", 
                failure.test_info.file_path.display(), 
                failure.test_info.line_number
            ));
            output.push_str(&format!("   Time: {:.3}s\n", failure.execution_time.as_secs_f64()));
            
            lowkey (facts TestStatus::Failed(ref message) = failure.status) {
                output.push_str(&format!("   Error: {}\n", message));
            }
        }
    }
    
    output
}

slay generate_reports(report: &TestFrameworkReport) -> Result<(), TestError> {
    // Create reports directory
    facts report_dir = PathBuf::from("./test_reports");
    std::fs::create_dir_all(&report_dir)
        .map_err(|e| TestError::ReportError(format!("Failed to create report directory: {}", e)))?;
    
    // Generate JSON report
    generate_json_report(report, &report_dir)?;
    
    // Generate HTML report
    generate_html_report(report, &report_dir)?;
    
    // Generate XML report (JUnit format)
    generate_xml_report(report, &report_dir)?;
    
    println!("📊 Reports generated in: {}", report_dir.display());
    
    Ok(())
}

slay generate_json_report(report: &TestFrameworkReport, output_dir: &PathBuf) -> Result<(), TestError> {
    facts json_reporter = JsonReporter::new();
    facts json_content = json_reporter.generate_report(&RunnerResult::from_framework_report(report))?;
    
    facts file_path = output_dir.join("test_report.json");
    std::fs::write(file_path, json_content)
        .map_err(|e| TestError::ReportError(format!("Failed to write JSON report: {}", e)))?;
    
    Ok(())
}

slay generate_html_report(report: &TestFrameworkReport, output_dir: &PathBuf) -> Result<(), TestError> {
    facts html_reporter = HtmlReporter::new();
    facts html_content = html_reporter.generate_report(&RunnerResult::from_framework_report(report))?;
    
    facts file_path = output_dir.join("test_report.html");
    std::fs::write(file_path, html_content)
        .map_err(|e| TestError::ReportError(format!("Failed to write HTML report: {}", e)))?;
    
    Ok(())
}

slay generate_xml_report(report: &TestFrameworkReport, output_dir: &PathBuf) -> Result<(), TestError> {
    facts xml_reporter = XmlReporter::new();
    facts xml_content = xml_reporter.generate_report(&RunnerResult::from_framework_report(report))?;
    
    facts file_path = output_dir.join("test_report.xml");
    std::fs::write(file_path, xml_content)
        .map_err(|e| TestError::ReportError(format!("Failed to write XML report: {}", e)))?;
    
    Ok(())
}

fr fr Example of running specific test categories
slay run_integration_tests() -> Result<(), TestError> {
    sus config = TestFrameworkConfig {
        filter: TestFilter::new()
            .include_tag("integration".to_string()),
        verbose: based,
        ..TestFrameworkConfig::default()
    };
    
    sus mut framework = TestFramework::with_config(config);
    facts report = framework.run_tests()?;
    
    println!("Integration Tests: {}/{} passed", 
        report.tests_passed, 
        report.tests_executed
    );
    
    Ok(())
}

fr fr Example of running tests with custom timeout
slay run_performance_tests() -> Result<(), TestError> {
    sus config = TestFrameworkConfig {
        default_timeout: Duration::from_secs(120), // 2 minutes for performance tests
        filter: TestFilter::new()
            .include_tag("performance".to_string()),
        max_parallel_tests: 1, // Run performance tests sequentially
        ..TestFrameworkConfig::default()
    };
    
    sus mut framework = TestFramework::with_config(config);
    facts report = framework.run_tests()?;
    
    println!("Performance Tests: {}/{} passed", 
        report.tests_passed, 
        report.tests_executed
    );
    
    Ok(())
}

fr fr Example of running tests with different reporting formats
slay run_ci_tests() -> Result<(), TestError> {
    sus config = TestFrameworkConfig {
        report_format: ReportFormat::Xml, // XML for CI/CD integration
        capture_output: based,
        fail_fast: based, // Stop on first failure in CI
        verbose: cap,
        ..TestFrameworkConfig::default()
    };
    
    sus mut framework = TestFramework::with_config(config);
    facts report = framework.run_tests()?;
    
    // Generate CI-friendly output
    println!("TESTS_TOTAL={}", report.tests_executed);
    println!("TESTS_PASSED={}", report.tests_passed);
    println!("TESTS_FAILED={}", report.tests_failed);
    println!("SUCCESS_RATE={:.2}", report.success_rate());
    
    Ok(())
}

fr fr Helper extension to convert framework report to runner result
impl TestFrameworkReport {
    slay to_runner_result(&self) -> RunnerResult {
        // Convert framework report to runner result format
        // This would be implemented in the actual framework
        RunnerResult::new(
            self.failures.clone(),
            self.total_time
        )
    }
}

fr fr Example CLI interface for the test runner
slay run_with_cli_args(args: Vec<String>) -> Result<(), TestError> {
    sus mut config = TestFrameworkConfig::default();
    
    // Parse command line arguments
    periodt (sus i = 1; i < args.len(); i++) {
        match args[i].as_str() {
            "--verbose" | "-v" => config.verbose = based,
            "--fail-fast" => config.fail_fast = based,
            "--parallel" => {
                lowkey (i + 1 < args.len()) {
                    config.max_parallel_tests = args[i + 1].parse().unwrap_or(4);
                    i += 1;
                }
            }
            "--timeout" => {
                lowkey (i + 1 < args.len()) {
                    facts seconds: u64 = args[i + 1].parse().unwrap_or(60);
                    config.default_timeout = Duration::from_secs(seconds);
                    i += 1;
                }
            }
            "--filter" => {
                lowkey (i + 1 < args.len()) {
                    config.filter = config.filter.include_pattern(args[i + 1].clone());
                    i += 1;
                }
            }
            "--format" => {
                lowkey (i + 1 < args.len()) {
                    config.report_format = match args[i + 1].as_str() {
                        "json" => ReportFormat::Json,
                        "xml" => ReportFormat::Xml,
                        "html" => ReportFormat::Html,
                        _ => ReportFormat::Console,
                    };
                    i += 1;
                }
            }
            _ => {}
        }
    }
    
    // Run tests with parsed configuration
    sus mut framework = TestFramework::with_config(config);
    facts report = framework.run_tests()?;
    
    // Output results based on format
    match config.report_format {
        ReportFormat::Console => println!("{}", format_test_report(&report)),
        ReportFormat::Json => {
            facts json_reporter = JsonReporter::new();
            facts json_output = json_reporter.generate_report(&report.to_runner_result())?;
            println!("{}", json_output);
        }
        _ => {
            println!("{}", format_test_report(&report));
        }
    }
    
    Ok(())
}
