// TestResult Type System for CURSED
// Standardized test result handling system

use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};

use crate::ast::{StructField, Visibility, Type as AstType};
use crate::type_system::*;

/// TestResult represents the outcome of a single test assertion
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestResult {
    pub test_name: String,
    pub assertion_name: String,
    pub status: TestStatus,
    pub message: String,
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub execution_time: Option<u64>, // in milliseconds
    pub line_number: Option<u32>,
    pub file_name: Option<String>,
}

/// TestStatus represents the result of a test
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestStatus {
    Pass,
    Fail,
    Skip,
    Error,
}

/// TestSuite aggregates multiple test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub suite_name: String,
    pub tests: Vec<TestResult>,
    pub setup_time: u64,
    pub teardown_time: u64,
    pub total_time: u64,
    pub metadata: HashMap<String, String>,
}

/// TestReport provides comprehensive reporting capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReport {
    pub suites: Vec<TestSuite>,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub error_tests: usize,
    pub success_rate: f64,
    pub execution_time: u64,
    pub timestamp: String,
}

/// TestResultBuilder provides fluent API for creating test results
#[derive(Debug, Clone)]
pub struct TestResultBuilder {
    test_name: String,
    assertion_name: String,
    status: TestStatus,
    message: String,
    expected: Option<String>,
    actual: Option<String>,
    execution_time: Option<u64>,
    line_number: Option<u32>,
    file_name: Option<String>,
}

impl TestResult {
    /// Create a new TestResult with pass status
    pub fn pass(test_name: &str, assertion_name: &str, message: &str) -> Self {
        Self {
            test_name: test_name.to_string(),
            assertion_name: assertion_name.to_string(),
            status: TestStatus::Pass,
            message: message.to_string(),
            expected: None,
            actual: None,
            execution_time: None,
            line_number: None,
            file_name: None,
        }
    }
    
    /// Create a new TestResult with fail status
    pub fn fail(test_name: &str, assertion_name: &str, message: &str, expected: &str, actual: &str) -> Self {
        Self {
            test_name: test_name.to_string(),
            assertion_name: assertion_name.to_string(),
            status: TestStatus::Fail,
            message: message.to_string(),
            expected: Some(expected.to_string()),
            actual: Some(actual.to_string()),
            execution_time: None,
            line_number: None,
            file_name: None,
        }
    }
    
    /// Create a new TestResult with skip status
    pub fn skip(test_name: &str, assertion_name: &str, message: &str) -> Self {
        Self {
            test_name: test_name.to_string(),
            assertion_name: assertion_name.to_string(),
            status: TestStatus::Skip,
            message: message.to_string(),
            expected: None,
            actual: None,
            execution_time: None,
            line_number: None,
            file_name: None,
        }
    }
    
    /// Create a new TestResult with error status
    pub fn error(test_name: &str, assertion_name: &str, message: &str) -> Self {
        Self {
            test_name: test_name.to_string(),
            assertion_name: assertion_name.to_string(),
            status: TestStatus::Error,
            message: message.to_string(),
            expected: None,
            actual: None,
            execution_time: None,
            line_number: None,
            file_name: None,
        }
    }
    
    /// Create a builder for fluent API
    pub fn builder(test_name: &str, assertion_name: &str) -> TestResultBuilder {
        TestResultBuilder::new(test_name, assertion_name)
    }
    
    /// Check if the test passed
    pub fn is_pass(&self) -> bool {
        self.status == TestStatus::Pass
    }
    
    /// Check if the test failed
    pub fn is_fail(&self) -> bool {
        self.status == TestStatus::Fail
    }
    
    /// Check if the test was skipped
    pub fn is_skip(&self) -> bool {
        self.status == TestStatus::Skip
    }
    
    /// Check if the test had an error
    pub fn is_error(&self) -> bool {
        self.status == TestStatus::Error
    }
    
    /// Set execution time
    pub fn with_execution_time(mut self, time: u64) -> Self {
        self.execution_time = Some(time);
        self
    }
    
    /// Set line number
    pub fn with_line_number(mut self, line: u32) -> Self {
        self.line_number = Some(line);
        self
    }
    
    /// Set file name
    pub fn with_file_name(mut self, file: &str) -> Self {
        self.file_name = Some(file.to_string());
        self
    }
}

impl TestResultBuilder {
    pub fn new(test_name: &str, assertion_name: &str) -> Self {
        Self {
            test_name: test_name.to_string(),
            assertion_name: assertion_name.to_string(),
            status: TestStatus::Pass,
            message: String::new(),
            expected: None,
            actual: None,
            execution_time: None,
            line_number: None,
            file_name: None,
        }
    }
    
    pub fn pass(mut self, message: &str) -> Self {
        self.status = TestStatus::Pass;
        self.message = message.to_string();
        self
    }
    
    pub fn fail(mut self, message: &str) -> Self {
        self.status = TestStatus::Fail;
        self.message = message.to_string();
        self
    }
    
    pub fn skip(mut self, message: &str) -> Self {
        self.status = TestStatus::Skip;
        self.message = message.to_string();
        self
    }
    
    pub fn error(mut self, message: &str) -> Self {
        self.status = TestStatus::Error;
        self.message = message.to_string();
        self
    }
    
    pub fn expected(mut self, expected: &str) -> Self {
        self.expected = Some(expected.to_string());
        self
    }
    
    pub fn actual(mut self, actual: &str) -> Self {
        self.actual = Some(actual.to_string());
        self
    }
    
    pub fn execution_time(mut self, time: u64) -> Self {
        self.execution_time = Some(time);
        self
    }
    
    pub fn line_number(mut self, line: u32) -> Self {
        self.line_number = Some(line);
        self
    }
    
    pub fn file_name(mut self, file: &str) -> Self {
        self.file_name = Some(file.to_string());
        self
    }
    
    pub fn build(self) -> TestResult {
        TestResult {
            test_name: self.test_name,
            assertion_name: self.assertion_name,
            status: self.status,
            message: self.message,
            expected: self.expected,
            actual: self.actual,
            execution_time: self.execution_time,
            line_number: self.line_number,
            file_name: self.file_name,
        }
    }
}

impl TestSuite {
    pub fn new(suite_name: &str) -> Self {
        Self {
            suite_name: suite_name.to_string(),
            tests: Vec::new(),
            setup_time: 0,
            teardown_time: 0,
            total_time: 0,
            metadata: HashMap::new(),
        }
    }
    
    pub fn add_test(&mut self, test: TestResult) {
        self.tests.push(test);
    }
    
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
    
    pub fn passed_count(&self) -> usize {
        self.tests.iter().filter(|t| t.is_pass()).count()
    }
    
    pub fn failed_count(&self) -> usize {
        self.tests.iter().filter(|t| t.is_fail()).count()
    }
    
    pub fn skipped_count(&self) -> usize {
        self.tests.iter().filter(|t| t.is_skip()).count()
    }
    
    pub fn error_count(&self) -> usize {
        self.tests.iter().filter(|t| t.is_error()).count()
    }
    
    pub fn total_count(&self) -> usize {
        self.tests.len()
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.tests.is_empty() {
            0.0
        } else {
            (self.passed_count() as f64) / (self.tests.len() as f64) * 100.0
        }
    }
    
    pub fn is_successful(&self) -> bool {
        self.failed_count() == 0 && self.error_count() == 0
    }
    
    pub fn set_timing(&mut self, setup_time: u64, teardown_time: u64, total_time: u64) {
        self.setup_time = setup_time;
        self.teardown_time = teardown_time;
        self.total_time = total_time;
    }
}

impl TestReport {
    pub fn new() -> Self {
        Self {
            suites: Vec::new(),
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
            skipped_tests: 0,
            error_tests: 0,
            success_rate: 0.0,
            execution_time: 0,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    pub fn add_suite(&mut self, suite: TestSuite) {
        self.total_tests += suite.total_count();
        self.passed_tests += suite.passed_count();
        self.failed_tests += suite.failed_count();
        self.skipped_tests += suite.skipped_count();
        self.error_tests += suite.error_count();
        self.execution_time += suite.total_time;
        
        self.suites.push(suite);
        self.calculate_success_rate();
    }
    
    pub fn is_successful(&self) -> bool {
        self.failed_tests == 0 && self.error_tests == 0
    }
    
    pub fn calculate_success_rate(&mut self) {
        if self.total_tests == 0 {
            self.success_rate = 0.0;
        } else {
            self.success_rate = (self.passed_tests as f64) / (self.total_tests as f64) * 100.0;
        }
    }
    
    /// Generate JSON report
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
    
    /// Generate XML report
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<testsuites>\n");
        
        for suite in &self.suites {
            xml.push_str(&format!("  <testsuite name=\"{}\" tests=\"{}\" failures=\"{}\" errors=\"{}\" skipped=\"{}\" time=\"{}\">\n",
                suite.suite_name, suite.total_count(), suite.failed_count(), suite.error_count(), suite.skipped_count(), suite.total_time));
            
            for test in &suite.tests {
                xml.push_str(&format!("    <testcase name=\"{}\" classname=\"{}\" time=\"{}\">\n",
                    test.assertion_name, test.test_name, test.execution_time.unwrap_or(0)));
                
                match test.status {
                    TestStatus::Fail => {
                        xml.push_str(&format!("      <failure message=\"{}\">Expected: {}, Actual: {}</failure>\n",
                            test.message, test.expected.as_deref().unwrap_or(""), test.actual.as_deref().unwrap_or("")));
                    }
                    TestStatus::Error => {
                        xml.push_str(&format!("      <error message=\"{}\"></error>\n", test.message));
                    }
                    TestStatus::Skip => {
                        xml.push_str(&format!("      <skipped message=\"{}\"></skipped>\n", test.message));
                    }
                    _ => {}
                }
                
                xml.push_str("    </testcase>\n");
            }
            
            xml.push_str("  </testsuite>\n");
        }
        
        xml.push_str("</testsuites>\n");
        xml
    }
    
    /// Generate HTML report
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<title>CURSED Test Report</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str(".pass { color: green; }\n");
        html.push_str(".fail { color: red; }\n");
        html.push_str(".skip { color: orange; }\n");
        html.push_str(".error { color: purple; }\n");
        html.push_str("table { border-collapse: collapse; width: 100%; }\n");
        html.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
        html.push_str("th { background-color: #f2f2f2; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        
        html.push_str("<h1>CURSED Test Report</h1>\n");
        html.push_str(&format!("<h2>Summary</h2>\n"));
        html.push_str(&format!("<p>Total Tests: {}</p>\n", self.total_tests));
        html.push_str(&format!("<p>Passed: <span class=\"pass\">{}</span></p>\n", self.passed_tests));
        html.push_str(&format!("<p>Failed: <span class=\"fail\">{}</span></p>\n", self.failed_tests));
        html.push_str(&format!("<p>Skipped: <span class=\"skip\">{}</span></p>\n", self.skipped_tests));
        html.push_str(&format!("<p>Errors: <span class=\"error\">{}</span></p>\n", self.error_tests));
        html.push_str(&format!("<p>Success Rate: {:.2}%</p>\n", self.success_rate));
        html.push_str(&format!("<p>Execution Time: {}ms</p>\n", self.execution_time));
        html.push_str(&format!("<p>Timestamp: {}</p>\n", self.timestamp));
        
        for suite in &self.suites {
            html.push_str(&format!("<h3>Test Suite: {}</h3>\n", suite.suite_name));
            html.push_str("<table>\n");
            html.push_str("<tr><th>Test</th><th>Assertion</th><th>Status</th><th>Message</th><th>Time</th></tr>\n");
            
            for test in &suite.tests {
                let status_class = match test.status {
                    TestStatus::Pass => "pass",
                    TestStatus::Fail => "fail",
                    TestStatus::Skip => "skip",
                    TestStatus::Error => "error",
                };
                
                html.push_str(&format!("<tr><td>{}</td><td>{}</td><td class=\"{}\">{:?}</td><td>{}</td><td>{}ms</td></tr>\n",
                    test.test_name, test.assertion_name, status_class, test.status, test.message, test.execution_time.unwrap_or(0)));
            }
            
            html.push_str("</table>\n");
        }
        
        html.push_str("</body>\n</html>\n");
        html
    }
    
    /// Generate console report
    pub fn to_console(&self) -> String {
        let mut output = String::new();
        
        output.push_str("CURSED Test Report\n");
        output.push_str("==================\n\n");
        
        for suite in &self.suites {
            output.push_str(&format!("Test Suite: {}\n", suite.suite_name));
            output.push_str(&format!("Tests: {} | Passed: {} | Failed: {} | Skipped: {} | Errors: {}\n",
                suite.total_count(), suite.passed_count(), suite.failed_count(), suite.skipped_count(), suite.error_count()));
            output.push_str(&format!("Success Rate: {:.2}%\n", suite.success_rate()));
            output.push_str(&format!("Execution Time: {}ms\n\n", suite.total_time));
            
            for test in &suite.tests {
                let status_symbol = match test.status {
                    TestStatus::Pass => "✓",
                    TestStatus::Fail => "✗",
                    TestStatus::Skip => "⚠",
                    TestStatus::Error => "⚠",
                };
                
                output.push_str(&format!("  {} {}: {} - {}\n",
                    status_symbol, test.test_name, test.assertion_name, test.message));
                
                if test.is_fail() {
                    if let (Some(expected), Some(actual)) = (&test.expected, &test.actual) {
                        output.push_str(&format!("    Expected: {}\n", expected));
                        output.push_str(&format!("    Actual:   {}\n", actual));
                    }
                }
            }
            
            output.push_str("\n");
        }
        
        output.push_str("Summary\n");
        output.push_str("=======\n");
        output.push_str(&format!("Total Tests: {}\n", self.total_tests));
        output.push_str(&format!("Passed: {}\n", self.passed_tests));
        output.push_str(&format!("Failed: {}\n", self.failed_tests));
        output.push_str(&format!("Skipped: {}\n", self.skipped_tests));
        output.push_str(&format!("Errors: {}\n", self.error_tests));
        output.push_str(&format!("Success Rate: {:.2}%\n", self.success_rate));
        output.push_str(&format!("Total Execution Time: {}ms\n", self.execution_time));
        
        if self.is_successful() {
            output.push_str("\n🎉 ALL TESTS PASSED! 🎉\n");
        } else {
            output.push_str("\n❌ Some tests failed\n");
        }
        
        output
    }
}

impl fmt::Display for TestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestStatus::Pass => write!(f, "PASS"),
            TestStatus::Fail => write!(f, "FAIL"),
            TestStatus::Skip => write!(f, "SKIP"),
            TestStatus::Error => write!(f, "ERROR"),
        }
    }
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_symbol = match self.status {
            TestStatus::Pass => "✓",
            TestStatus::Fail => "✗",
            TestStatus::Skip => "⚠",
            TestStatus::Error => "⚠",
        };
        
        write!(f, "{} {}: {} - {}", status_symbol, self.test_name, self.assertion_name, self.message)
    }
}

impl fmt::Display for TestSuite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Test Suite: {} ({} tests, {:.2}% success rate)", 
            self.suite_name, self.total_count(), self.success_rate())
    }
}

impl fmt::Display for TestReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Test Report: {} tests, {:.2}% success rate", 
            self.total_tests, self.success_rate)
    }
}

impl Default for TestReport {
    fn default() -> Self {
        Self::new()
    }
}

// Type system integration
impl TypeExpression {
    /// Create a TestResult type expression
    pub fn test_result() -> Self {
        Self {
            kind: TypeKind::Struct,
            name: Some("TestResult".to_string()),
            parameters: Vec::new(),
            return_type: None,
        }
    }
    
    /// Create a TestSuite type expression
    pub fn test_suite() -> Self {
        Self {
            kind: TypeKind::Struct,
            name: Some("TestSuite".to_string()),
            parameters: Vec::new(),
            return_type: None,
        }
    }
    
    /// Create a TestReport type expression
    pub fn test_report() -> Self {
        Self {
            kind: TypeKind::Struct,
            name: Some("TestReport".to_string()),
            parameters: Vec::new(),
            return_type: None,
        }
    }
    
    /// Create a TestStatus type expression
    pub fn test_status() -> Self {
        Self {
            kind: TypeKind::Enum,
            name: Some("TestStatus".to_string()),
            parameters: Vec::new(),
            return_type: None,
        }
    }
}

// Register TestResult types with the type system
pub fn register_test_result_types(type_system: &mut TypeSystem) {
    // Register TestResult type
    let test_result_type = TypeDefinition {
        name: "TestResult".to_string(),
        kind: TypeKind::Struct,
        type_parameters: Vec::new(),
        constraints: Vec::new(),
        methods: vec![
            MethodSignature {
                name: "pass".to_string(),
                parameters: vec![TypeExpression::named("tea"), TypeExpression::named("tea"), TypeExpression::named("tea")],
                return_type: Some(TypeExpression::test_result()),
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
            MethodSignature {
                name: "fail".to_string(),
                parameters: vec![TypeExpression::named("tea"), TypeExpression::named("tea"), TypeExpression::named("tea"), TypeExpression::named("tea"), TypeExpression::named("tea")],
                return_type: Some(TypeExpression::test_result()),
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
            MethodSignature {
                name: "is_pass".to_string(),
                parameters: vec![],
                return_type: Some(TypeExpression::named("lit")),
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
            MethodSignature {
                name: "is_fail".to_string(),
                parameters: vec![],
                return_type: Some(TypeExpression::named("lit")),
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
        ],
        fields: vec![
            StructField {
                name: "test_name".to_string(),
                field_type: AstType::Named("tea".to_string()),
                visibility: Visibility::Public,
            },
            StructField {
                name: "assertion_name".to_string(),
                field_type: AstType::Named("tea".to_string()),
                visibility: Visibility::Public,
            },
            StructField {
                name: "status".to_string(),
                field_type: AstType::Named("TestStatus".to_string()),
                visibility: Visibility::Public,
            },
            StructField {
                name: "message".to_string(),
                field_type: AstType::Named("tea".to_string()),
                visibility: Visibility::Public,
            },
        ],
        is_builtin: true,
    };
    
    // Register TestSuite type
    let test_suite_type = TypeDefinition {
        name: "TestSuite".to_string(),
        kind: TypeKind::Struct,
        type_parameters: Vec::new(),
        constraints: Vec::new(),
        methods: vec![
            MethodSignature {
                name: "new".to_string(),
                parameters: vec![TypeExpression::named("tea")],
                return_type: Some(TypeExpression::test_suite()),
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
            MethodSignature {
                name: "add_test".to_string(),
                parameters: vec![TypeExpression::test_result()],
                return_type: None,
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
            MethodSignature {
                name: "success_rate".to_string(),
                parameters: vec![],
                return_type: Some(TypeExpression::named("meal")),
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
        ],
        fields: vec![
            StructField {
                name: "suite_name".to_string(),
                field_type: AstType::Named("tea".to_string()),
                visibility: Visibility::Public,
            },
            StructField {
                name: "tests".to_string(),
                field_type: AstType::Array(Box::new(AstType::Named("TestResult".to_string()))),
                visibility: Visibility::Public,
            },
        ],
        is_builtin: true,
    };
    
    // Register TestReport type  
    let test_report_type = TypeDefinition {
        name: "TestReport".to_string(),
        kind: TypeKind::Struct,
        type_parameters: Vec::new(),
        constraints: Vec::new(),
        methods: vec![
            MethodSignature {
                name: "new".to_string(),
                parameters: vec![],
                return_type: Some(TypeExpression::test_report()),
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
            MethodSignature {
                name: "add_suite".to_string(),
                parameters: vec![TypeExpression::test_suite()],
                return_type: None,
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
            MethodSignature {
                name: "to_json".to_string(),
                parameters: vec![],
                return_type: Some(TypeExpression::named("tea")),
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
            MethodSignature {
                name: "to_xml".to_string(),
                parameters: vec![],
                return_type: Some(TypeExpression::named("tea")),
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
            MethodSignature {
                name: "to_html".to_string(),
                parameters: vec![],
                return_type: Some(TypeExpression::named("tea")),
                type_parameters: Vec::new(),
                constraints: Vec::new(),
            },
        ],
        fields: vec![
            StructField {
                name: "total_tests".to_string(),
                field_type: AstType::Named("normie".to_string()),
                visibility: Visibility::Public,
            },
            StructField {
                name: "passed_tests".to_string(),
                field_type: AstType::Named("normie".to_string()),
                visibility: Visibility::Public,
            },
            StructField {
                name: "success_rate".to_string(),
                field_type: AstType::Named("meal".to_string()),
                visibility: Visibility::Public,
            },
        ],
        is_builtin: true,
    };
    
    // Register TestStatus enum
    let test_status_type = TypeDefinition {
        name: "TestStatus".to_string(),
        kind: TypeKind::Enum,
        type_parameters: Vec::new(),
        constraints: Vec::new(),
        methods: vec![],
        fields: vec![],
        is_builtin: true,
    };
    
    // Add all types to the type system
    type_system.environment.add_type_definition(test_result_type);
    type_system.environment.add_type_definition(test_suite_type);
    type_system.environment.add_type_definition(test_report_type);
    type_system.environment.add_type_definition(test_status_type);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_result_creation() {
        let result = TestResult::pass("test_math", "assert_eq", "2 + 2 = 4");
        assert_eq!(result.test_name, "test_math");
        assert_eq!(result.assertion_name, "assert_eq");
        assert_eq!(result.status, TestStatus::Pass);
        assert_eq!(result.message, "2 + 2 = 4");
    }
    
    #[test]
    fn test_result_builder() {
        let result = TestResult::builder("test_math", "assert_eq")
            .pass("2 + 2 = 4")
            .execution_time(10)
            .line_number(42)
            .file_name("test.csd")
            .build();
        
        assert_eq!(result.test_name, "test_math");
        assert_eq!(result.assertion_name, "assert_eq");
        assert_eq!(result.status, TestStatus::Pass);
        assert_eq!(result.execution_time, Some(10));
        assert_eq!(result.line_number, Some(42));
        assert_eq!(result.file_name, Some("test.csd".to_string()));
    }
    
    #[test]
    fn test_suite_aggregation() {
        let mut suite = TestSuite::new("math_tests");
        suite.add_test(TestResult::pass("test_add", "assert_eq", "2 + 2 = 4"));
        suite.add_test(TestResult::fail("test_div", "assert_eq", "Division by zero", "2", "error"));
        
        assert_eq!(suite.total_count(), 2);
        assert_eq!(suite.passed_count(), 1);
        assert_eq!(suite.failed_count(), 1);
        assert_eq!(suite.success_rate(), 50.0);
        assert!(!suite.is_successful());
    }
    
    #[test]
    fn test_report_generation() {
        let mut report = TestReport::new();
        
        let mut suite = TestSuite::new("math_tests");
        suite.add_test(TestResult::pass("test_add", "assert_eq", "2 + 2 = 4"));
        suite.add_test(TestResult::pass("test_mul", "assert_eq", "3 * 3 = 9"));
        
        report.add_suite(suite);
        
        assert_eq!(report.total_tests, 2);
        assert_eq!(report.passed_tests, 2);
        assert_eq!(report.failed_tests, 0);
        assert_eq!(report.success_rate, 100.0);
        assert!(report.is_successful());
    }
    
    #[test]
    fn test_type_registration() {
        let mut type_system = TypeSystem::new();
        register_test_result_types(&mut type_system);
        
        assert!(type_system.environment.get_type("TestResult").is_some());
        assert!(type_system.environment.get_type("TestSuite").is_some());
        assert!(type_system.environment.get_type("TestReport").is_some());
        assert!(type_system.environment.get_type("TestStatus").is_some());
    }
}
