/// Comprehensive tests for the CURSED testing framework
/// 
/// Tests the testing framework functionality including assertions,
/// test discovery, execution, reporting, and statistics.

use std::time::Duration;
use std::collections::HashMap;
use crate::stdlib::testing::*;

#[test]
fn test_basic_assertions() {
    // Test basic boolean assertions
    assert!(assert_true(true).is_ok());
    assert!(assert_false(false).is_ok());
    assert!(assert_true(false).is_err());
    assert!(assert_false(true).is_err());
    
    // Test equality assertions
    assert!(assert_eq(5, 5).is_ok());
    assert!(assert_ne(5, 3).is_ok());
    assert!(assert_eq(5, 3).is_err());
    assert!(assert_ne(5, 5).is_err());
    
    // Test null assertions
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    
    assert!(assert_not_null(some_value).is_ok());
    assert!(assert_null(none_value).is_ok());
    assert!(assert_null(some_value).is_err());
    assert!(assert_not_null(none_value).is_err());
}

#[test]
fn test_numeric_assertions() {
    // Test comparison assertions
    assert!(assert_greater(10, 5).is_ok());
    assert!(assert_less(5, 10).is_ok());
    assert!(assert_greater_equal(10, 10).is_ok());
    assert!(assert_less_equal(5, 5).is_ok());
    
    assert!(assert_greater(5, 10).is_err());
    assert!(assert_less(10, 5).is_err());
    
    // Test floating point assertions
    assert!(assert_close_to(3.14159, 3.14, 0.01).is_ok());
    assert!(assert_close_to(3.14159, 3.14, 0.001).is_err());
    
    // Test range assertions
    assert!(assert_between(5, 1, 10).is_ok());
    assert!(assert_between(15, 1, 10).is_err());
    
    // Test sign assertions
    assert!(assert_positive(5).is_ok());
    assert!(assert_negative(-3).is_ok());
    assert!(assert_zero(0).is_ok());
    
    assert!(assert_positive(-5).is_err());
    assert!(assert_negative(3).is_err());
    assert!(assert_zero(5).is_err());
}

#[test]
fn test_string_assertions() {
    let text = "Hello, World!";
    
    // Test string content assertions
    assert!(assert_contains(text, "Hello").is_ok());
    assert!(assert_not_contains(text, "Goodbye").is_ok());
    assert!(assert_starts_with(text, "Hello").is_ok());
    assert!(assert_ends_with(text, "!").is_ok());
    
    assert!(assert_contains(text, "Goodbye").is_err());
    assert!(assert_not_contains(text, "Hello").is_err());
    assert!(assert_starts_with(text, "Goodbye").is_err());
    assert!(assert_ends_with(text, "?").is_err());
    
    // Test string properties
    assert!(assert_length(text, 13).is_ok());
    assert!(assert_length(text, 10).is_err());
    
    assert!(assert_empty_string("").is_ok());
    assert!(assert_empty_string("not empty").is_err());
    
    // Test pattern matching
    assert!(assert_matches_regex("test123", "*123").is_ok());
    assert!(assert_matches_regex("test123", "test*").is_ok());
    assert!(assert_matches_regex("test123", "*est*").is_ok());
    assert!(assert_matches_regex("test123", "xyz*").is_err());
}

#[test]
fn test_collection_assertions() {
    let numbers = vec![1, 2, 3, 4, 5];
    let empty_vec: Vec<i32> = vec![];
    let booleans = vec![true, true, true];
    let mixed_booleans = vec![true, false, true];
    let all_false = vec![false, false, false];
    
    // Test collection properties
    assert!(assert_not_empty(&numbers).is_ok());
    assert!(assert_empty(&empty_vec).is_ok());
    assert!(assert_has_length(&numbers, 5).is_ok());
    
    assert!(assert_empty(&numbers).is_err());
    assert!(assert_not_empty(&empty_vec).is_err());
    assert!(assert_has_length(&numbers, 3).is_err());
    
    // Test element presence
    assert!(assert_contains_element(&numbers, &3).is_ok());
    assert!(assert_not_contains_element(&numbers, &10).is_ok());
    
    assert!(assert_contains_element(&numbers, &10).is_err());
    assert!(assert_not_contains_element(&numbers, &3).is_err());
    
    // Test boolean collections
    assert!(assert_all_true(&booleans).is_ok());
    assert!(assert_any_true(&mixed_booleans).is_ok());
    assert!(assert_none_true(&all_false).is_ok());
    
    assert!(assert_all_true(&mixed_booleans).is_err());
    assert!(assert_any_true(&all_false).is_err());
    assert!(assert_none_true(&mixed_booleans).is_err());
}

#[test]
fn test_error_assertions() {
    let success_result: Result<i32, String> = Ok(42);
    let error_result: Result<i32, String> = Err("Test error".to_string());
    
    // Test result assertions
    assert!(assert_no_error(success_result.clone()).is_ok());
    assert!(assert_error(error_result.clone()).is_ok());
    
    assert!(assert_error(success_result.clone()).is_err());
    assert!(assert_no_error(error_result.clone()).is_err());
    
    // Test specific error type
    assert!(assert_error_type(error_result.clone(), "Test error".to_string()).is_ok());
    assert!(assert_error_type(error_result.clone(), "Different error".to_string()).is_err());
    
    // Test error message
    assert!(assert_error_message(error_result.clone(), "Test error").is_ok());
    assert!(assert_error_message(error_result.clone(), "Different error").is_err());
}

#[test]
fn test_panic_assertions() {
    // Test panic detection
    let panic_result = assert_panic(|| {
        panic!("Test panic");
    });
    assert!(panic_result.is_ok());
    
    let no_panic_result = assert_panic(|| {
        // Function that doesn't panic
        42
    });
    assert!(no_panic_result.is_err());
    
    // Test no panic assertion
    let no_panic_result2 = assert_no_panic(|| {
        42
    });
    assert!(no_panic_result2.is_ok());
    
    let panic_result2 = assert_no_panic(|| {
        panic!("Unexpected panic");
    });
    assert!(panic_result2.is_err());
}

#[test]
fn test_test_attributes() {
    // Test basic attribute parsing
    let test_attr = TestAttribute::parse("#[test]").unwrap();
    assert!(matches!(test_attr, TestAttribute::Test));
    
    let ignore_attr = TestAttribute::parse("#[ignore]").unwrap();
    assert!(matches!(ignore_attr, TestAttribute::Ignore(None)));
    
    let ignore_with_reason = TestAttribute::parse("#[ignore(\"Not implemented\")]").unwrap();
    assert!(matches!(ignore_with_reason, TestAttribute::Ignore(Some(_))));
    
    let panic_attr = TestAttribute::parse("#[should_panic]").unwrap();
    assert!(matches!(panic_attr, TestAttribute::ShouldPanic(None)));
    
    let timeout_attr = TestAttribute::parse("#[timeout(5000)]").unwrap();
    assert!(matches!(timeout_attr, TestAttribute::Timeout(_)));
    
    // Test custom attributes
    let custom_attr = TestAttribute::parse("#[custom_tag]").unwrap();
    assert!(matches!(custom_attr, TestAttribute::Custom(_, None)));
    
    let custom_with_value = TestAttribute::parse("#[tag(\"integration\")]").unwrap();
    assert!(matches!(custom_with_value, TestAttribute::Tag(_)));
}

#[test]
fn test_test_attributes_collection() {
    let mut attributes = TestAttributes::new();
    
    attributes.add(TestAttribute::Test);
    attributes.add(TestAttribute::Ignore(Some("Not ready".to_string())));
    attributes.add(TestAttribute::Tag("integration".to_string()));
    
    // Test attribute detection
    assert!(attributes.has_attribute("test"));
    assert!(attributes.has_attribute("ignore"));
    assert!(attributes.has_attribute("tag"));
    assert!(!attributes.has_attribute("timeout"));
    
    // Test metadata parsing
    let metadata = attributes.get_metadata();
    assert!(metadata.is_test);
    assert!(metadata.ignore);
    assert_eq!(metadata.ignore_reason, Some("Not ready".to_string()));
    assert!(metadata.tags.contains(&"integration".to_string()));
}

#[test]
fn test_test_discovery() {
    let discovery = TestDiscovery::new();
    
    // Test filter creation and matching
    let filter = TestFilter::new()
        .include_pattern("test_*".to_string())
        .exclude_tag("slow".to_string());
    
    // Create mock test info
    let test_info = TestInfo {
        name: "test_example".to_string(),
        file_path: std::path::PathBuf::from("test_file.csd"),
        line_number: 10,
        metadata: super::discovery::TestMetadata {
            ignore: false,
            should_panic: false,
            timeout: None,
            description: None,
            tags: vec!["fast".to_string()],
            setup: None,
            teardown: None,
            attributes: HashMap::new(),
        },
        module: "test_module".to_string(),
        discovered_at: std::time::SystemTime::now(),
    };
    
    // Test filter matching
    assert!(filter.matches(&test_info));
    
    // Test exclusion
    let mut slow_test = test_info.clone();
    slow_test.metadata.tags.push("slow".to_string());
    assert!(!filter.matches(&slow_test));
}

#[test]
fn test_test_executor() {
    let config = TestExecutorConfig {
        default_timeout: Duration::from_secs(30),
        capture_output: true,
        max_parallel_tests: 1,
        fail_fast: false,
    };
    
    let executor = crate::stdlib::testing::executor::SequentialExecutor::with_config(config);
    
    // Create a simple test that should pass
    let passing_test = TestInfo {
        name: "test_pass".to_string(),
        file_path: std::path::PathBuf::from("test.csd"),
        line_number: 1,
        metadata: super::discovery::TestMetadata::default(),
        module: "test".to_string(),
        discovered_at: std::time::SystemTime::now(),
    };
    
    let result = executor.execute_test(passing_test).unwrap();
    assert!(result.status.is_success());
    
    // Create a test that should fail
    let failing_test = TestInfo {
        name: "test_fail".to_string(),
        file_path: std::path::PathBuf::from("test.csd"),
        line_number: 2,
        metadata: super::discovery::TestMetadata::default(),
        module: "test".to_string(),
        discovered_at: std::time::SystemTime::now(),
    };
    
    let result = executor.execute_test(failing_test).unwrap();
    assert!(result.status.is_failure());
}

#[test]
fn test_test_runner() {
    let config = TestRunnerConfig {
        execution_mode: crate::stdlib::testing::framework::TestExecutionMode::Sequential,
        fail_fast: false,
        verbose: false,
        show_timing: true,
    };
    
    let runner = TestRunner::with_config(config);
    let executor = crate::stdlib::testing::executor::SequentialExecutor::new();
    
    // Create test suite
    let tests = vec![
        TestInfo {
            name: "test_1".to_string(),
            file_path: std::path::PathBuf::from("test.csd"),
            line_number: 1,
            metadata: super::discovery::TestMetadata::default(),
            module: "test".to_string(),
            discovered_at: std::time::SystemTime::now(),
        },
        TestInfo {
            name: "test_2".to_string(),
            file_path: std::path::PathBuf::from("test.csd"),
            line_number: 10,
            metadata: super::discovery::TestMetadata::default(),
            module: "test".to_string(),
            discovered_at: std::time::SystemTime::now(),
        },
    ];
    
    let result = runner.run_tests(tests, &executor).unwrap();
    assert_eq!(result.test_results.len(), 2);
    assert!(result.total_time > Duration::from_secs(0));
}

#[test]
fn test_statistics_collection() {
    let mut stats = TestStatistics::new();
    
    // Record some test executions
    stats.record_test_execution("test_1", Duration::from_millis(100), true);
    stats.record_test_execution("test_2", Duration::from_millis(200), false);
    stats.record_test_execution("test_1", Duration::from_millis(150), true);
    
    // Check statistics
    let summary = stats.get_summary();
    assert_eq!(summary.total_tests, 2);
    assert_eq!(summary.total_executions, 3);
    
    // Check test-specific metrics
    let test1_metrics = stats.get_test_metrics("test_1").unwrap();
    assert_eq!(test1_metrics.execution_count, 2);
    
    // Check performance trends
    let trend = stats.get_performance_trend("test_1");
    assert!(trend.is_some());
    assert_eq!(trend.unwrap().len(), 2);
}

#[test]
fn test_performance_stats() {
    let timings = vec![
        Duration::from_millis(100),
        Duration::from_millis(150),
        Duration::from_millis(200),
        Duration::from_millis(125),
        Duration::from_millis(175),
    ];
    
    let stats = PerformanceStats::from_timings(&timings);
    
    assert_eq!(stats.sample_count, 5);
    assert_eq!(stats.min_time, Duration::from_millis(100));
    assert_eq!(stats.max_time, Duration::from_millis(200));
    assert_eq!(stats.median_time, Duration::from_millis(150));
}

#[test]
fn test_test_reporters() {
    // Test console reporter
    let console_reporter = ConsoleReporter::new();
    assert!(console_reporter.report_discovery_start().is_ok());
    assert!(console_reporter.report_discovery_complete(5).is_ok());
    assert!(console_reporter.report_execution_start(5).is_ok());
    
    // Test JSON reporter
    let json_reporter = JsonReporter::new();
    let mock_result = create_mock_runner_result();
    let json_report = json_reporter.generate_report(&mock_result);
    assert!(json_report.is_ok());
    
    // Test XML reporter
    let xml_reporter = XmlReporter::new();
    let xml_report = xml_reporter.generate_report(&mock_result);
    assert!(xml_report.is_ok());
    
    // Test HTML reporter
    let html_reporter = HtmlReporter::new();
    let html_report = html_reporter.generate_report(&mock_result);
    assert!(html_report.is_ok());
}

#[test]
fn test_test_framework_integration() {
    let config = TestFrameworkConfig {
        test_root: std::path::PathBuf::from("./tests"),
        test_patterns: vec!["**/*test*.csd".to_string()],
        max_parallel_tests: 1,
        default_timeout: Duration::from_secs(30),
        capture_output: true,
        fail_fast: false,
        filter: TestFilter::default(),
        report_format: ReportFormat::Console,
        report_output_dir: None,
        verbose: false,
        show_timing: true,
        run_ignored: false,
    };
    
    let mut framework = TestFramework::with_config(config);
    
    // Since we don't have actual test files in the filesystem,
    // this will return an empty result, but we can test the framework setup
    let stats = framework.get_statistics();
    assert_eq!(stats.get_summary().total_tests, 0);
}

// Helper function to create mock runner result for testing
fn create_mock_runner_result() -> RunnerResult {
    let test_results = vec![
        TestResult {
            test_info: TestInfo {
                name: "test_success".to_string(),
                file_path: std::path::PathBuf::from("test.csd"),
                line_number: 1,
                metadata: super::discovery::TestMetadata::default(),
                module: "test".to_string(),
                discovered_at: std::time::SystemTime::now(),
            },
            status: TestStatus::Passed,
            execution_time: Duration::from_millis(100),
            output: Some("Test passed".to_string()),
            error_output: None,
            memory_usage: None,
            metadata: HashMap::new(),
        },
        TestResult {
            test_info: TestInfo {
                name: "test_failure".to_string(),
                file_path: std::path::PathBuf::from("test.csd"),
                line_number: 10,
                metadata: super::discovery::TestMetadata::default(),
                module: "test".to_string(),
                discovered_at: std::time::SystemTime::now(),
            },
            status: TestStatus::Failed("Assertion failed".to_string()),
            execution_time: Duration::from_millis(50),
            output: None,
            error_output: Some("Error output".to_string()),
            memory_usage: None,
            metadata: HashMap::new(),
        },
    ];
    
    RunnerResult::new(test_results, Duration::from_millis(200))
}

#[test]
fn test_timeout_handling() {
    let timeout = TestTimeout::default()
        .set_timeout("slow_test".to_string(), Duration::from_secs(120));
    
    assert_eq!(timeout.get_timeout("slow_test"), Duration::from_secs(120));
    assert_eq!(timeout.get_timeout("normal_test"), Duration::from_secs(60)); // default
}

#[test]
fn test_ignore_conditions() {
    let context = super::attributes::IgnoreContext {
        platform: Some("linux".to_string()),
        features: vec!["experimental".to_string()],
        env_vars: {
            let mut env = HashMap::new();
            env.insert("CI".to_string(), "true".to_string());
            env
        },
        custom_data: HashMap::new(),
    };
    
    let platform_condition = super::attributes::IgnoreCondition::Platform("linux".to_string());
    assert!(platform_condition.matches(&context));
    
    let env_condition = super::attributes::IgnoreCondition::EnvVar("CI".to_string(), Some("true".to_string()));
    assert!(env_condition.matches(&context));
    
    let feature_condition = super::attributes::IgnoreCondition::Feature("experimental".to_string());
    assert!(feature_condition.matches(&context));
    
    let wrong_platform = super::attributes::IgnoreCondition::Platform("windows".to_string());
    assert!(!wrong_platform.matches(&context));
}

#[test]
fn test_eventually_assertion() {
    use std::sync::{Arc, Mutex};
    
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = Arc::clone(&counter);
    
    let result = assert_eventually(move || {
        let mut count = counter_clone.lock().unwrap();
        *count += 1;
        *count > 5
    }, Duration::from_millis(100));
    
    assert!(result.is_ok());
}

#[test]
fn test_file_assertions() {
    use std::fs;
    
    let test_file = "test_assertion_file.txt";
    let content = "Test content for assertions";
    
    // Create test file
    fs::write(test_file, content).unwrap();
    
    // Test file existence
    assert!(assert_file_exists(test_file).is_ok());
    assert!(assert_file_exists("nonexistent_file.txt").is_err());
    
    // Test file content
    assert!(assert_file_content(test_file, content).is_ok());
    assert!(assert_file_content(test_file, "wrong content").is_err());
    
    // Cleanup
    fs::remove_file(test_file).unwrap();
}
