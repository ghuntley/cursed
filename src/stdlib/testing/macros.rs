use crate::error::Error;
/// Test macros and code generation for CURSED testing framework
/// 
/// Provides utility functions for generating test code and handling
/// test attribute macros in CURSED syntax.

use std::collections::HashMap;
use crate::crate::stdlib::errors_simple::CursedError;
use super::{TestError, TestFrameworkResult};

/// Generate a test function with proper CURSED syntax
pub fn test_function(name: &str, body: &str) -> String {
    format!(
        r#"#[test]
slay {}() {{
    {}
}}"#,
        name, body
    )
}

/// Generate an ignored test function
pub fn ignore_test(name: &str, body: &str, reason: Option<&str>) -> String {
    let ignore_attr = if let Some(reason) = reason {
        format!("#[ignore(\"{}\")]", reason)
    } else {
        "#[ignore]".to_string()
    };
    
    format!(
        r#"#[test]
{}
slay {}() {{
    {}
}}"#,
        ignore_attr, name, body
    )
}

/// Generate a test function that should panic
pub fn should_panic_test(name: &str, body: &str, expected_message: Option<&str>) -> String {
    let panic_attr = if let Some(message) = expected_message {
        format!("#[should_panic(\"{}\")]", message)
    } else {
        "#[should_panic]".to_string()
    };
    
    format!(
        r#"#[test]
{}
slay {}() {{
    {}
}}"#,
        panic_attr, name, body
    )
}

/// Generate a test function with timeout
pub fn timeout_test(name: &str, body: &str, timeout_ms: u64) -> String {
    format!(
        r#"#[test]
#[timeout({})]
slay {}() {{
    {}
}}"#,
        timeout_ms, name, body
    )
}

/// Generate a setup function
pub fn setup_function(name: &str, body: &str) -> String {
    format!(
        r#"#[setup]
slay {}() {{
    {}
}}"#,
        name, body
    )
}

/// Generate a teardown function
pub fn teardown_function(name: &str, body: &str) -> String {
    format!(
        r#"#[teardown]
slay {}() {{
    {}
}}"#,
        name, body
    )
}

/// Generate a complete test suite
pub fn test_suite_macro(suite_name: &str, tests: &[(String, String)]) -> String {
    let mut suite = format!("// Test suite: {}\n\n", suite_name);
    
    for (test_name, test_body) in tests {
        suite.push_str(&test_function(test_name, test_body));
        suite.push_str("\n\n");
    }
    
    suite
}

/// Test assertion macros for CURSED syntax
pub struct TestAssertions;

impl TestAssertions {
    /// Generate assert_eq macro call
    pub fn assert_eq(left: &str, right: &str) -> String {
        format!("assert_eq!({}, {});", left, right)
    }
    
    /// Generate assert_ne macro call
    pub fn assert_ne(left: &str, right: &str) -> String {
        format!("assert_ne!({}, {});", left, right)
    }
    
    /// Generate assert! macro call
    pub fn assert(condition: &str) -> String {
        format!("assert!({});", condition)
    }
    
    /// Generate assert! macro call with message
    pub fn assert_with_message(condition: &str, message: &str) -> String {
        format!("assert!({}, \"{}\");", condition, message)
    }
    
    /// Generate assert_true call using CURSED testing framework
    pub fn assert_true(value: &str) -> String {
        format!("assert_true({})?;", value)
    }
    
    /// Generate assert_false call using CURSED testing framework
    pub fn assert_false(value: &str) -> String {
        format!("assert_false({})?;", value)
    }
    
    /// Generate assert_null call
    pub fn assert_null(value: &str) -> String {
        format!("assert_null({})?;", value)
    }
    
    /// Generate assert_not_null call
    pub fn assert_not_null(value: &str) -> String {
        format!("assert_not_null({})?;", value)
    }
    
    /// Generate string assertion
    pub fn assert_contains(haystack: &str, needle: &str) -> String {
        format!("assert_contains({}, {})?;", haystack, needle)
    }
    
    /// Generate numeric comparison assertion
    pub fn assert_greater(left: &str, right: &str) -> String {
        format!("assert_greater({}, {})?;", left, right)
    }
    
    /// Generate error assertion
    pub fn assert_error(result: &str) -> String {
        format!("assert_error({})?;", result)
    }
}

/// Template for generating test code
#[derive(Debug, Clone)]
pub struct TestTemplate {
    /// Template name
    pub name: String,
    /// Template parameters
    pub parameters: HashMap<String, String>,
    /// Template body
    pub body: String,
}

impl TestTemplate {
    /// Create a new test template
    pub fn new(name: String, body: String) -> Self {
        Self {
            name,
            parameters: HashMap::new(),
            body,
        }
    }
    
    /// Add a parameter to the template
    pub fn with_parameter(mut self, key: String, value: String) -> Self {
        self.parameters.insert(key, value);
        self
    }
    
    /// Generate test code from template
    pub fn generate(&self, test_name: &str) -> TestFrameworkResult<String> {
        let mut result = self.body.clone();
        
        // Replace template parameters
        for (key, value) in &self.parameters {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        // Replace test name placeholder
        result = result.replace("{test_name}", test_name);
        
        Ok(result)
    }
}

/// Collection of predefined test templates
pub struct TestTemplateLibrary;

impl TestTemplateLibrary {
    /// Get basic test template
    pub fn basic_test() -> TestTemplate {
        TestTemplate::new(
            "basic_test".to_string(),
            r#"#[test]
slay {test_name}() {{
    // Test implementation goes here
    assert_true(true)?;
}}"#.to_string()
        )
    }
    
    /// Get arithmetic test template
    pub fn arithmetic_test() -> TestTemplate {
        TestTemplate::new(
            "arithmetic_test".to_string(),
            r#"#[test]
slay {test_name}() {{
    facts left = {left_value};
    facts right = {right_value};
    facts result = left + right;
    facts expected = {expected_value};
    
    assert_eq!(result, expected)?;
}}"#.to_string()
        )
    }
    
    /// Get string test template
    pub fn string_test() -> TestTemplate {
        TestTemplate::new(
            "string_test".to_string(),
            r#"#[test]
slay {test_name}() {{
    facts input = "{input_string}";
    facts expected = "{expected_string}";
    facts result = {operation}(input);
    
    assert_eq!(result, expected)?;
}}"#.to_string()
        )
    }
    
    /// Get error handling test template
    pub fn error_test() -> TestTemplate {
        TestTemplate::new(
            "error_test".to_string(),
            r#"#[test]
slay {test_name}() {{
    facts result = {operation}({input});
    assert_error!(result)?;
}}"#.to_string()
        )
    }
    
    /// Get performance test template
    pub fn performance_test() -> TestTemplate {
        TestTemplate::new(
            "performance_test".to_string(),
            r#"#[test]
#[timeout({timeout_ms})]
slay {test_name}() {{
    facts start_time = now();
    
    // Performance-critical operation
    {operation};
    
    facts end_time = now();
    facts duration = end_time - start_time;
    
    // Assert performance requirements
    assert_less!(duration, Duration::from_millis({max_duration_ms}))?;
}}"#.to_string()
        )
    }
    
    /// Get integration test template
    pub fn integration_test() -> TestTemplate {
        TestTemplate::new(
            "integration_test".to_string(),
            r#"#[test]
#[tag("integration")]
slay {test_name}() {{
    // Setup
    facts {setup_var} = {setup_operation};
    
    // Execute
    facts result = {test_operation}({setup_var});
    
    // Verify
    assert_eq!(result, {expected_result})?;
    
    // Cleanup (if needed)
    {cleanup_operation};
}}"#.to_string()
        )
    }
    
    /// Get all available templates
    pub fn all_templates() -> Vec<TestTemplate> {
        vec![
            Self::basic_test(),
            Self::arithmetic_test(),
            Self::string_test(),
            Self::error_test(),
            Self::performance_test(),
            Self::integration_test(),
        ]
    }
}

/// Test code generator
pub struct TestCodeGenerator {
    /// Available templates
    templates: HashMap<String, TestTemplate>,
}

impl TestCodeGenerator {
    /// Create a new code generator
    pub fn new() -> Self {
        let mut generator = Self {
            templates: HashMap::new(),
        };
        
        // Load predefined templates
        for template in TestTemplateLibrary::all_templates() {
            generator.add_template(template);
        }
        
        generator
    }
    
    /// Add a custom template
    pub fn add_template(&mut self, template: TestTemplate) {
        self.templates.insert(template.name.clone(), template);
    }
    
    /// Generate test code from template
    pub fn generate_test(&self, template_name: &str, test_name: &str, parameters: HashMap<String, String>) -> TestFrameworkResult<String> {
        let template = self.templates.get(template_name)
            .ok_or_else(|| TestError::ConfigError(format!("Template '{}' not found", template_name)))?;
        
        let mut template_with_params = template.clone();
        for (key, value) in parameters {
            template_with_params.parameters.insert(key, value);
        }
        
        template_with_params.generate(test_name)
    }
    
    /// Generate multiple tests from a template
    pub fn generate_tests(&self, template_name: &str, test_configs: &[(String, HashMap<String, String>)]) -> TestFrameworkResult<Vec<String>> {
        let mut tests = Vec::new();
        
        for (test_name, parameters) in test_configs {
            let test_code = self.generate_test(template_name, test_name, parameters.clone())?;
            tests.push(test_code);
        }
        
        Ok(tests)
    }
    
    /// Generate a complete test file
    pub fn generate_test_file(&self, file_name: &str, tests: &[String]) -> String {
        let mut file_content = format!("// Generated test file: {}\n", file_name);
        file_content.push_str("// Auto-generated by CURSED Testing Framework\n\n");
        
        // Add imports
        file_content.push_str("import \"stdlib::testing\";\n");
        file_content.push_str("use testing::*;\n\n");
        
        // Add tests
        for test in tests {
            file_content.push_str(test);
            file_content.push_str("\n\n");
        }
        
        file_content
    }
    
    /// List available templates
    pub fn list_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }
}

impl Default for TestCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for CURSED test syntax
pub struct CursedTestSyntax;

impl CursedTestSyntax {
    /// Convert standard Rust assert to CURSED assert
    pub fn convert_assert(rust_assert: &str) -> String {
        if rust_assert.starts_with("assert_eq!") {
            // Extract parameters and convert to CURSED syntax
            let params = extract_assert_params(rust_assert, "assert_eq!");
            if params.len() == 2 {
                return format!("assert_eq({}, {})?;", params[0], params[1]);
            }
        } else if rust_assert.starts_with("assert_ne!") {
            let params = extract_assert_params(rust_assert, "assert_ne!");
            if params.len() == 2 {
                return format!("assert_ne({}, {})?;", params[0], params[1]);
            }
        } else if rust_assert.starts_with("assert!") {
            let params = extract_assert_params(rust_assert, "assert!");
            if !params.is_empty() {
                return format!("assert_true({})?;", params[0]);
            }
        }
        
        // Fallback to original
        rust_assert.to_string()
    }
    
    /// Generate CURSED variable declaration
    pub fn declare_variable(name: &str, value: &str, mutable: bool) -> String {
        if mutable {
            format!("sus {} = {};", name, value)
        } else {
            format!("facts {} = {};", name, value)
        }
    }
    
    /// Generate CURSED function declaration
    pub fn declare_function(name: &str, params: &[(&str, &str)], return_type: Option<&str>, body: &str) -> String {
        let param_list = params.iter()
            .map(|(name, type_)| format!("{}: {}", name, type_))
            .collect::<Vec<_>>()
            .join(", ");
        
        let return_annotation = if let Some(ret_type) = return_type {
            format!(" -> {}", ret_type)
        } else {
            String::new()
        };
        
        format!(
            "slay {}({}){} {{\n{}\n}}",
            name, param_list, return_annotation, body
        )
    }
    
    /// Generate CURSED if statement
    pub fn if_statement(condition: &str, then_body: &str, else_body: Option<&str>) -> String {
        if let Some(else_part) = else_body {
            format!("lowkey ({}) {{\n{}\n}} highkey {{\n{}\n}}", condition, then_body, else_part)
        } else {
            format!("lowkey ({}) {{\n{}\n}}", condition, then_body)
        }
    }
    
    /// Generate CURSED loop
    pub fn for_loop(var: &str, iterable: &str, body: &str) -> String {
        format!("periodt ({} in {}) {{\n{}\n}}", var, iterable, body)
    }
    
    /// Generate CURSED while loop
    pub fn while_loop(condition: &str, body: &str) -> String {
        format!("bestie ({}) {{\n{}\n}}", condition, body)
    }
}

/// Helper function to extract parameters from assert macros
fn extract_assert_params(assert_str: &str, macro_name: &str) -> Vec<String> {
    let prefix_len = macro_name.len();
    if assert_str.len() <= prefix_len + 2 {
        return Vec::new();
    }
    
    let params_str = &assert_str[prefix_len + 1..assert_str.len() - 2]; // Remove macro!( and )
    
    // Simple parameter parsing (doesn't handle nested parentheses perfectly)
    let mut params = Vec::new();
    let mut current_param = String::new();
    let mut paren_depth = 0;
    let mut in_string = false;
    let mut escape_next = false;
    
    for ch in params_str.chars() {
        if escape_next {
            current_param.push(ch);
            escape_next = false;
            continue;
        }
        
        match ch {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '(' if !in_string => paren_depth += 1,
            ')' if !in_string => paren_depth -= 1,
            ',' if !in_string && paren_depth == 0 => {
                params.push(current_param.trim().to_string());
                current_param.clear();
                continue;
            }
            _ => {}
        }
        
        current_param.push(ch);
    }
    
    if !current_param.trim().is_empty() {
        params.push(current_param.trim().to_string());
    }
    
    params
}
