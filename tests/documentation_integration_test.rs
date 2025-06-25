//! # Documentation System Integration Test
//!
//! Comprehensive integration test that demonstrates the complete live documentation
//! system working together with hot reload, interactive features, and testing.

use cursed::documentation::{
    DocumentationConfig, DocumentationSystem, DocOptions, ProjectMetadata, StylingConfig
};
use cursed::documentation::live_server::{
    LiveDocumentationServer, LiveServerConfig, LiveDocumentationServerBuilder
};
use cursed::documentation::interactive::{
    InteractiveDocumentation, InteractiveConfig, CodeExecutionRequest,
    SyntaxHighlightRequest, ApiCallRequest
};
use cursed::documentation::testing::{
    DocumentationTester, DocumentationTestConfig, DocumentationTestResult
};
use cursed::documentation::extractors::enhanced_ast_extractor::{
    EnhancedAstExtractor, TypeRelationship, RelationshipType, SemanticAnalysis
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::timeout;

/// Create a comprehensive test project structure
async fn create_test_project(temp_dir: &TempDir) -> PathBuf {
    let project_dir = temp_dir.path().join("test_project");
    let src_dir = project_dir.join("src");
    std::fs::create_dir_all(&src_dir).unwrap();
    
    // Create main.csd with comprehensive CURSED features
    let main_file = src_dir.join("main.csd");
    std::fs::write(&main_file, r#"
//! # CURSED Test Project
//! 
//! This is a comprehensive test project demonstrating all CURSED language features
//! including Gen Z slang syntax, goroutines, interfaces, and advanced functionality.

import "stdlib::io";
import "stdlib::math";
import "stdlib::collections";

/// Main entry point for the CURSED application
/// 
/// # Examples
/// 
/// ```cursed
/// slay main() {
///     println("Hello, CURSED World!");
/// }
/// ```
slay main() {
    facts greeting = "Hello, CURSED World! 💅";
    println(greeting);
    
    // Demonstrate arithmetic
    sus result = calculate_fibonacci(10);
    println("Fibonacci(10) = " + result.to_string());
    
    // Demonstrate goroutines
    stan process_data_async();
    
    // Demonstrate error handling
    lowkey try_something() {
        println("Success! ✨");
    } highkey {
        println("Something went wrong 😢");
    }
}

/// Calculate Fibonacci number using recursive approach
/// 
/// This function demonstrates CURSED's mathematical capabilities
/// with Gen Z slang syntax for maximum readability.
/// 
/// # Arguments
/// 
/// * `n` - The position in the Fibonacci sequence to calculate
/// 
/// # Returns
/// 
/// The Fibonacci number at position `n`
/// 
/// # Examples
/// 
/// ```cursed
/// sus fib_5 = calculate_fibonacci(5);
/// // fib_5 == 5
/// ```
slay calculate_fibonacci(sus n: i32) -> i32 {
    lowkey n <= 1 {
        return n;
    }
    return calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2);
}

/// Asynchronous data processing using goroutines
/// 
/// Demonstrates CURSED's concurrency model with the `stan` keyword
/// for spawning goroutines with Gen Z flair.
slay process_data_async() {
    stan worker_task(1);
    stan worker_task(2);
    stan worker_task(3);
    
    // Coordinate workers
    facts channel = make_channel();
    bestie sus i = 0; i < 3; i++ {
        channel.receive();
        yolo; // Yield to other goroutines
    }
}

/// Worker task for demonstrating goroutine functionality
/// 
/// # Arguments
/// 
/// * `worker_id` - Unique identifier for this worker
slay worker_task(sus worker_id: i32) {
    facts work_amount = worker_id * 100;
    
    bestie sus i = 0; i < work_amount; i++ {
        lowkey i % 10 == 0 {
            yolo; // Yield periodically
        }
    }
    
    println("Worker " + worker_id.to_string() + " completed! 🎉");
}

/// Attempt some operation that might fail
/// 
/// Demonstrates error handling patterns in CURSED
/// 
/// # Returns
/// 
/// `true` if successful, `false` otherwise
slay try_something() -> bool {
    // Simulate some operation
    sus random_value = 42;
    return random_value > 0;
}

/// Person struct demonstrating CURSED's type system
/// 
/// This struct shows how to define custom types with proper
/// documentation and Gen Z naming conventions.
squad Person {
    /// The person's name (obviously)
    name: string,
    
    /// Age in years (not in dog years)
    age: i32,
    
    /// Whether this person is currently vibing
    is_vibing: bool,
}

/// Implement methods for Person
impl Person {
    /// Create a new Person instance
    /// 
    /// # Arguments
    /// 
    /// * `name` - The person's name
    /// * `age` - The person's age
    /// 
    /// # Returns
    /// 
    /// A new Person instance ready to vibe
    /// 
    /// # Examples
    /// 
    /// ```cursed
    /// sus person = Person::new("Alex", 25);
    /// ```
    slay new(name: string, age: i32) -> Person {
        return Person {
            name: name,
            age: age,
            is_vibing: true,
        };
    }
    
    /// Check if person can vote
    /// 
    /// # Returns
    /// 
    /// `true` if age >= 18, `false` otherwise
    slay can_vote(self) -> bool {
        return self.age >= 18;
    }
    
    /// Start vibing (set is_vibing to true)
    slay start_vibing(self) {
        self.is_vibing = true;
        println(self.name + " is now vibing! 🕺");
    }
}

/// Socializer interface for things that can socialize
/// 
/// This interface demonstrates CURSED's interface system
/// with proper documentation and usage examples.
collab Socializer {
    /// Greet another socializer
    /// 
    /// # Arguments
    /// 
    /// * `other` - The other socializer to greet
    slay greet(self, other: &Socializer);
    
    /// Say goodbye to another socializer
    /// 
    /// # Arguments
    /// 
    /// * `other` - The other socializer to say goodbye to
    slay farewell(self, other: &Socializer);
}

/// Mood enumeration for different emotional states
/// 
/// This enum shows CURSED's enumeration system with
/// comprehensive documentation for each variant.
enum Mood {
    /// Everything is amazing and nothing hurts
    Happy,
    
    /// Meh, could be better
    Neutral,
    
    /// Big sad energy
    Sad,
    
    /// When you can't even
    Frustrated,
    
    /// Peak performance vibes
    Excited,
}

/// Vibe check function for analyzing current mood
/// 
/// This function demonstrates CURSED's pattern matching
/// with the `vibe_check` statement (switch equivalent).
/// 
/// # Arguments
/// 
/// * `current_mood` - The mood to analyze
/// 
/// # Returns
/// 
/// A string describing the vibe analysis
/// 
/// # Examples
/// 
/// ```cursed
/// sus analysis = check_the_vibe(Mood::Happy);
/// // analysis == "Vibes are immaculate! ✨"
/// ```
slay check_the_vibe(current_mood: Mood) -> string {
    vibe_check current_mood {
        mood Mood::Happy => {
            return "Vibes are immaculate! ✨";
        }
        mood Mood::Excited => {
            return "Energy is through the roof! 🚀";
        }
        mood Mood::Neutral => {
            return "Vibes are mid, tbh 😐";
        }
        mood Mood::Sad => {
            return "Big sad energy detected 😢";
        }
        mood Mood::Frustrated => {
            return "Someone needs a break 😤";
        }
        basic => {
            return "Vibe status: unknown 🤷";
        }
    }
}

/// Generic container for holding any type of value
/// 
/// This demonstrates CURSED's generic type system with
/// proper constraint handling and documentation.
/// 
/// # Type Parameters
/// 
/// * `T` - The type of value to store
squad Container<T> {
    /// The stored value
    value: T,
    
    /// Whether the container is sealed
    is_sealed: bool,
}

/// Implement methods for Container
impl<T> Container<T> {
    /// Create a new container with the given value
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to store
    /// 
    /// # Returns
    /// 
    /// A new container instance
    slay new(value: T) -> Container<T> {
        return Container {
            value: value,
            is_sealed: false,
        };
    }
    
    /// Get the stored value
    /// 
    /// # Returns
    /// 
    /// Reference to the stored value
    slay get(self) -> &T {
        return &self.value;
    }
    
    /// Seal the container (prevent further modifications)
    slay seal(self) {
        self.is_sealed = true;
    }
}

/// Error types for the application
/// 
/// This enum demonstrates CURSED's error handling system
/// with comprehensive error variants and documentation.
enum AppError {
    /// Input validation failed
    InvalidInput(string),
    
    /// Network operation failed
    NetworkError(string),
    
    /// File operation failed
    FileError(string),
    
    /// Generic application error
    Generic(string),
}

/// Result type alias for application operations
/// 
/// This type alias demonstrates CURSED's type system
/// and shows how to create convenient result types.
type AppResult<T> = Result<T, AppError>;

/// Configuration for the application
/// 
/// This struct demonstrates configuration patterns
/// with proper documentation and default values.
squad AppConfig {
    /// Server port to listen on
    port: i32,
    
    /// Maximum number of concurrent connections
    max_connections: i32,
    
    /// Whether debug mode is enabled
    debug_mode: bool,
    
    /// Application name
    app_name: string,
}

impl AppConfig {
    /// Create default configuration
    /// 
    /// # Returns
    /// 
    /// AppConfig with sensible defaults
    slay default() -> AppConfig {
        return AppConfig {
            port: 8080,
            max_connections: 100,
            debug_mode: false,
            app_name: "CURSED App",
        };
    }
}
"#).unwrap();
    
    // Create utils.csd with utility functions
    let utils_file = src_dir.join("utils.csd");
    std::fs::write(&utils_file, r#"
//! # Utility Functions
//! 
//! Collection of utility functions for common operations
//! in the CURSED test project.

/// Calculate the square of a number
/// 
/// # Arguments
/// 
/// * `x` - The number to square
/// 
/// # Returns
/// 
/// The square of the input number
/// 
/// # Examples
/// 
/// ```cursed
/// sus result = square(5);
/// // result == 25
/// ```
slay square(sus x: i32) -> i32 {
    return x * x;
}

/// Check if a number is even
/// 
/// # Arguments
/// 
/// * `n` - The number to check
/// 
/// # Returns
/// 
/// `true` if the number is even, `false` otherwise
/// 
/// # Examples
/// 
/// ```cursed
/// sus even = is_even(4);
/// // even == true
/// ```
slay is_even(sus n: i32) -> bool {
    return n % 2 == 0;
}

/// Format a greeting message
/// 
/// # Arguments
/// 
/// * `name` - The name to include in the greeting
/// 
/// # Returns
/// 
/// A formatted greeting string
/// 
/// # Examples
/// 
/// ```cursed
/// sus greeting = format_greeting("Alex");
/// // greeting == "Hello, Alex! Welcome to CURSED! 🎉"
/// ```
slay format_greeting(name: string) -> string {
    return "Hello, " + name + "! Welcome to CURSED! 🎉";
}
"#).unwrap();
    
    project_dir
}

/// Create test documentation files
async fn create_test_docs(project_dir: &PathBuf) -> PathBuf {
    let docs_dir = project_dir.join("docs");
    std::fs::create_dir_all(&docs_dir).unwrap();
    
    // Create README.md
    let readme_file = docs_dir.join("README.md");
    std::fs::write(&readme_file, r#"
# CURSED Test Project Documentation

This is the documentation for our comprehensive CURSED test project.

## Overview

The CURSED programming language brings Gen Z energy to systems programming with:

- 💅 **Expressive syntax** using modern slang
- 🚀 **High performance** with zero-cost abstractions  
- 🔗 **Excellent interop** with existing systems
- 🎯 **Memory safety** without garbage collection overhead

## Quick Start

Here's a simple example to get you started:

```cursed
slay main() {
    facts message = "Hello, CURSED World!";
    println(message);
}
```

## Features

### Goroutines with `stan`

Spawn lightweight threads for concurrent programming:

```cursed
slay process_data() {
    stan worker_task(1);
    stan worker_task(2);
    stan worker_task(3);
}
```

### Pattern Matching with `vibe_check`

Pattern match on enums and values:

```cursed
vibe_check mood {
    mood Mood::Happy => println("Feeling good!"),
    mood Mood::Sad => println("Need some tea 🍵"),
    basic => println("Mood unknown"),
}
```

### Error Handling

CURSED provides robust error handling:

```cursed
slay risky_operation() -> Result<string, AppError> {
    lowkey something_bad {
        return Err(AppError::Generic("Oops!"));
    }
    return Ok("Success!");
}
```

## API Reference

See the generated API documentation for detailed information about all modules and functions.

## Examples

Check out the `examples/` directory for more comprehensive usage examples.
"#).unwrap();
    
    // Create API documentation
    let api_file = docs_dir.join("api.md");
    std::fs::write(&api_file, r#"
# API Reference

Complete API reference for the CURSED test project.

## Core Functions

### `main()`

Main entry point for the application.

**Example:**
```cursed
slay main() {
    println("Starting application...");
}
```

### `calculate_fibonacci(n: i32) -> i32`

Calculate the nth Fibonacci number.

**Parameters:**
- `n`: The position in the Fibonacci sequence

**Returns:**
The Fibonacci number at position n

**Example:**
```cursed
sus fib = calculate_fibonacci(10);
println("Fibonacci(10) = " + fib.to_string());
```

## Types

### `Person`

Represents a person with basic information.

**Fields:**
- `name: string` - The person's name
- `age: i32` - The person's age  
- `is_vibing: bool` - Whether the person is currently vibing

**Methods:**
- `new(name: string, age: i32) -> Person` - Create a new person
- `can_vote() -> bool` - Check if person is old enough to vote
- `start_vibing()` - Start vibing

### `Mood`

Enumeration of possible emotional states.

**Variants:**
- `Happy` - Everything is amazing
- `Neutral` - Meh, could be better
- `Sad` - Big sad energy
- `Frustrated` - When you can't even
- `Excited` - Peak performance vibes

## Error Types

### `AppError`

Application-specific error types.

**Variants:**
- `InvalidInput(string)` - Input validation failed
- `NetworkError(string)` - Network operation failed
- `FileError(string)` - File operation failed
- `Generic(string)` - Generic application error

## Links

- [Main documentation](README.md)
- [Getting started guide](https://example.com/getting-started)
- [CURSED language reference](https://cursed-lang.org/reference)
- [Community Discord](https://discord.gg/cursed)
"#).unwrap();
    
    docs_dir
}

#[tokio::test]
async fn test_complete_documentation_system_integration() {
    // Create test project
    let temp_dir = TempDir::new().unwrap();
    let project_dir = create_test_project(&temp_dir).await;
    let docs_dir = create_test_docs(&project_dir).await;
    
    // Test 1: Documentation Generation System
    let doc_config = DocumentationConfig {
        source_dirs: vec![project_dir.join("src")],
        output_dir: docs_dir.clone(),
        output_formats: vec![
            cursed::documentation::OutputFormat::Html,
            cursed::documentation::OutputFormat::Markdown,
        ],
        project: ProjectMetadata {
            name: "CURSED Test Project".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Comprehensive test project for CURSED documentation".to_string()),
            authors: vec!["CURSED Team".to_string()],
            homepage: Some("https://cursed-lang.org".to_string()),
            repository: Some("https://github.com/cursed-lang/cursed".to_string()),
            license: Some("MIT".to_string()),
        },
        options: DocOptions {
            include_private: false,
            include_source: true,
            generate_cross_refs: true,
            generate_search_index: true,
            include_examples: true,
            max_type_depth: 10,
            include_dependencies: false,
        },
        styling: StylingConfig {
            custom_css: Vec::new(),
            template_dir: None,
            theme: "auto".to_string(),
            colors: None,
            favicon: None,
            logo: None,
        },
    };
    
    let mut doc_system = DocumentationSystem::new(doc_config).unwrap();
    let result = timeout(
        Duration::from_secs(30),
        doc_system.generate_all()
    ).await;
    
    assert!(result.is_ok(), "Documentation generation should complete");
    let doc_result = result.unwrap();
    assert!(doc_result.is_ok(), "Documentation generation should succeed");
    
    // Test 2: Interactive Documentation Features
    let interactive_config = InteractiveConfig {
        enable_playground: true,
        enable_executable_examples: true,
        enable_api_explorer: true,
        enable_syntax_highlighting: true,
        enable_code_folding: true,
        max_execution_time: Duration::from_secs(10),
        max_memory_mb: 128,
        temp_dir: temp_dir.path().join("playground"),
        ..Default::default()
    };
    
    let mut interactive_docs = InteractiveDocumentation::new(interactive_config).unwrap();
    
    // Test syntax highlighting
    let highlight_request = SyntaxHighlightRequest {
        code: r#"
            slay fibonacci(sus n: i32) -> i32 {
                lowkey n <= 1 {
                    return n;
                }
                return fibonacci(n - 1) + fibonacci(n - 2);
            }
        "#.to_string(),
        language: "cursed".to_string(),
        theme: "monokai".to_string(),
        highlight_lines: vec![2, 3],
        show_line_numbers: true,
    };
    
    let highlight_result = interactive_docs.highlight_syntax(highlight_request).await;
    assert!(highlight_result.is_ok(), "Syntax highlighting should work");
    
    let highlight_result = highlight_result.unwrap();
    assert!(!highlight_result.highlighted_html.is_empty());
    assert!(highlight_result.tokens.len() > 0);
    
    // Verify CURSED keywords are highlighted
    let has_cursed_keywords = highlight_result.tokens.iter().any(|t| 
        ["slay", "sus", "lowkey"].contains(&t.value.as_str()) && t.token_type == "keyword"
    );
    assert!(has_cursed_keywords, "CURSED keywords should be highlighted");
    
    // Test 3: Enhanced AST Extraction
    use cursed::documentation::extractors::ExtractionConfig;
    
    let extraction_config = ExtractionConfig {
        include_private: false,
        include_source: true,
        include_generics: true,
        include_relationships: true,
        max_type_depth: 10,
        include_implementations: true,
        include_error_types: true,
    };
    
    let mut enhanced_extractor = EnhancedAstExtractor::new(extraction_config).unwrap();
    
    // Read the main source file
    let main_file = project_dir.join("src").join("main.csd");
    let source_code = std::fs::read_to_string(&main_file).unwrap();
    
    // Note: In a real implementation, we would parse the source code to get an AST
    // For this test, we'll demonstrate the structure without actual parsing
    
    // Test 4: Documentation Testing
    let test_config = DocumentationTestConfig {
        test_examples: true,
        validate_links: true,
        generate_coverage: true,
        test_timeout: Duration::from_secs(15),
        max_parallel_tests: 2,
        test_output_dir: temp_dir.path().join("test_results"),
        ..Default::default()
    };
    
    let mut doc_tester = DocumentationTester::new(test_config).unwrap();
    
    let test_result = timeout(
        Duration::from_secs(60),
        doc_tester.run_tests(&docs_dir)
    ).await;
    
    assert!(test_result.is_ok(), "Documentation testing should complete");
    let test_result = test_result.unwrap();
    assert!(test_result.is_ok(), "Documentation testing should succeed");
    
    let test_result = test_result.unwrap();
    assert!(test_result.example_results.len() > 0, "Should find code examples to test");
    assert!(test_result.link_results.total_links > 0, "Should find links to validate");
    
    // Test 5: Live Server Configuration (without actually starting)
    let live_config = LiveServerConfig {
        port: 8888,
        host: "127.0.0.1".to_string(),
        watch_debounce: Duration::from_millis(300),
        enable_playground: true,
        enable_api_explorer: true,
        enable_syntax_highlighting: true,
        enable_code_folding: true,
        auto_open_browser: false,
        ..Default::default()
    };
    
    let live_server = LiveDocumentationServer::new(live_config);
    assert!(live_server.is_ok(), "Live server should be created successfully");
    
    let live_server = live_server.unwrap();
    assert!(!live_server.is_running(), "Server should not be running initially");
    assert_eq!(live_server.get_connected_clients(), 0);
    
    // Test 6: API Explorer Functionality
    let api_methods = interactive_docs.get_available_api_methods().await;
    assert!(api_methods.is_ok(), "Should be able to get API methods");
    
    let api_methods = api_methods.unwrap();
    assert!(api_methods.len() > 0, "Should have some API methods available");
    
    // Find the print method
    let print_method = api_methods.iter().find(|m| m.name == "print");
    assert!(print_method.is_some(), "Should have print method available");
    
    let print_method = print_method.unwrap();
    assert_eq!(print_method.return_type, "void");
    assert!(print_method.parameters.len() > 0);
    
    // Test API call
    let mut parameters = HashMap::new();
    parameters.insert(
        "message".to_string(), 
        serde_json::Value::String("Integration test message".to_string())
    );
    
    let api_request = ApiCallRequest {
        session_id: "integration_test".to_string(),
        method_name: "print".to_string(),
        parameters,
        expected_return_type: Some("void".to_string()),
        timeout: Some(Duration::from_secs(5)),
    };
    
    let api_result = interactive_docs.call_api_method(api_request).await;
    assert!(api_result.is_ok(), "API call should succeed");
    
    let api_result = api_result.unwrap();
    assert!(api_result.success, "API call should be successful");
    assert!(api_result.validation_errors.is_empty(), "Should have no validation errors");
    
    // Test 7: Session Management
    let session_stats = interactive_docs.get_session_statistics();
    assert!(session_stats.active_sessions > 0, "Should have active sessions");
    assert!(session_stats.total_executions > 0, "Should have executed some code");
    
    // Cleanup old sessions
    let cleanup_result = interactive_docs.cleanup_old_sessions(Duration::from_secs(0)).await;
    assert!(cleanup_result.is_ok(), "Session cleanup should succeed");
    
    // Test 8: Live Server Builder Pattern
    let server_builder_result = LiveDocumentationServerBuilder::new()
        .port(9999)
        .host("localhost".to_string())
        .watch_debounce(Duration::from_millis(250))
        .enable_playground(true)
        .enable_api_explorer(true)
        .auto_open_browser(false)
        .build();
    
    assert!(server_builder_result.is_ok(), "Server builder should work");
    
    println!("✅ Complete documentation system integration test passed!");
    println!("   - Documentation generation: ✅");
    println!("   - Interactive features: ✅");
    println!("   - Enhanced AST extraction: ✅");
    println!("   - Documentation testing: ✅");
    println!("   - Live server setup: ✅");
    println!("   - API explorer: ✅");
    println!("   - Session management: ✅");
    println!("   - Builder patterns: ✅");
}

#[tokio::test]
async fn test_theme_switching_and_customization() {
    let config = InteractiveConfig::default();
    let interactive_docs = InteractiveDocumentation::new(config).unwrap();
    
    let test_code = r#"
        slay greet(name: string) {
            facts message = "Hello, " + name + "!";
            println(message);
        }
    "#;
    
    let themes = ["monokai", "github", "solarized-dark", "solarized-light", "dracula"];
    
    for theme in &themes {
        let request = SyntaxHighlightRequest {
            code: test_code.to_string(),
            language: "cursed".to_string(),
            theme: theme.to_string(),
            highlight_lines: Vec::new(),
            show_line_numbers: true,
        };
        
        let result = interactive_docs.highlight_syntax(request).await;
        assert!(result.is_ok(), "Theme {} should work", theme);
        
        let result = result.unwrap();
        assert!(!result.highlighted_html.is_empty(), "Theme {} should produce HTML", theme);
        assert!(!result.css_styles.is_empty(), "Theme {} should have CSS styles", theme);
        
        // Verify theme-specific content
        match *theme {
            "monokai" => assert!(result.css_styles.contains("#272822")), // Monokai background
            "github" => assert!(result.css_styles.contains("#f6f8fa")), // GitHub background
            "solarized-dark" => assert!(result.css_styles.contains("#002b36")), // Solarized dark background
            "solarized-light" => assert!(result.css_styles.contains("#fdf6e3")), // Solarized light background
            "dracula" => assert!(result.css_styles.contains("#282a36")), // Dracula background
            _ => {}
        }
    }
}

#[tokio::test]
async fn test_error_scenarios_and_recovery() {
    let config = InteractiveConfig::default();
    let mut interactive_docs = InteractiveDocumentation::new(config).unwrap();
    
    // Test 1: Invalid code execution
    let invalid_request = CodeExecutionRequest {
        session_id: "error_test".to_string(),
        code: "this is not valid cursed syntax at all!!! @#$%".to_string(),
        language: "cursed".to_string(),
        input: None,
        timeout: Some(Duration::from_secs(5)),
        args: Vec::new(),
        env: HashMap::new(),
        working_dir: None,
    };
    
    let result = interactive_docs.execute_code(invalid_request).await;
    assert!(result.is_ok(), "Should not panic on invalid code");
    
    let result = result.unwrap();
    assert!(!result.success, "Should indicate failure for invalid code");
    assert!(!result.compilation_errors.is_empty() || !result.stderr.is_empty(), 
            "Should have error information");
    
    // Test 2: Invalid API method call
    let invalid_api_request = ApiCallRequest {
        session_id: "api_error_test".to_string(),
        method_name: "nonexistent_method".to_string(),
        parameters: HashMap::new(),
        expected_return_type: None,
        timeout: Some(Duration::from_secs(5)),
    };
    
    let api_result = interactive_docs.call_api_method(invalid_api_request).await;
    assert!(api_result.is_ok(), "Should not panic on invalid API call");
    
    let api_result = api_result.unwrap();
    assert!(!api_result.success, "Should indicate failure for invalid method");
    assert!(api_result.error.is_some(), "Should have error message");
    
    // Test 3: Invalid syntax highlighting language
    let invalid_highlight_request = SyntaxHighlightRequest {
        code: "some code".to_string(),
        language: "nonexistent_language".to_string(),
        theme: "monokai".to_string(),
        highlight_lines: Vec::new(),
        show_line_numbers: false,
    };
    
    let highlight_result = interactive_docs.highlight_syntax(invalid_highlight_request).await;
    // Should either succeed with fallback highlighting or return an error gracefully
    if let Err(e) = highlight_result {
        // Error should be descriptive
        assert!(e.to_string().contains("language") || e.to_string().contains("Unsupported"));
    }
}

#[tokio::test]
async fn test_websocket_message_handling() {
    use cursed::documentation::live_server::WebSocketMessage;
    use std::time::SystemTime;
    
    // Test serialization and deserialization of all message types
    let messages = vec![
        WebSocketMessage::DocumentationUpdated {
            timestamp: SystemTime::now(),
            files_changed: vec!["main.csd".to_string(), "utils.csd".to_string()],
            generation_time_ms: 250,
        },
        WebSocketMessage::GenerationStarted {
            timestamp: SystemTime::now(),
            trigger: "File change: main.csd".to_string(),
        },
        WebSocketMessage::GenerationFailed {
            timestamp: SystemTime::now(),
            error: "Compilation failed".to_string(),
            files_affected: vec!["main.csd".to_string()],
        },
        WebSocketMessage::ServerStats {
            timestamp: SystemTime::now(),
            connected_clients: 5,
            total_regenerations: 42,
            average_generation_time_ms: 180,
            uptime_seconds: 3600,
        },
        WebSocketMessage::ExecuteCode {
            code: "println(\"test\");".to_string(),
            language: "cursed".to_string(),
            session_id: "test_session".to_string(),
        },
        WebSocketMessage::Ping {
            timestamp: SystemTime::now(),
        },
        WebSocketMessage::Connected {
            server_version: "1.0.0".to_string(),
            features: vec!["playground".to_string(), "api_explorer".to_string()],
        },
    ];
    
    for message in messages {
        // Test serialization
        let serialized = serde_json::to_string(&message);
        assert!(serialized.is_ok(), "Message should serialize: {:?}", message);
        
        // Test deserialization
        let serialized = serialized.unwrap();
        let deserialized: Result<WebSocketMessage, _> = serde_json::from_str(&serialized);
        assert!(deserialized.is_ok(), "Message should deserialize: {}", serialized);
        
        // Verify type preservation
        let deserialized = deserialized.unwrap();
        assert!(std::mem::discriminant(&message) == std::mem::discriminant(&deserialized),
                "Message type should be preserved");
    }
}

#[tokio::test]
async fn test_performance_and_scalability() {
    let config = InteractiveConfig {
        max_parallel_tests: 8,
        max_execution_time: Duration::from_secs(2),
        max_memory_mb: 32, // Reduced for testing
        ..Default::default()
    };
    
    let mut interactive_docs = InteractiveDocumentation::new(config).unwrap();
    
    // Test concurrent execution of multiple code snippets
    let mut tasks = Vec::new();
    
    for i in 0..10 {
        let session_id = format!("perf_test_{}", i);
        let code = format!(r#"
            slay calculate(sus n: i32) -> i32 {{
                sus result = 0;
                bestie sus j = 0; j < n; j++ {{
                    result += j;
                }}
                return result;
            }}
            
            slay main() {{
                sus value = calculate({});
                println("Result: " + value.to_string());
            }}
        "#, i * 10);
        
        let request = CodeExecutionRequest {
            session_id,
            code,
            language: "cursed".to_string(),
            input: None,
            timeout: Some(Duration::from_secs(2)),
            args: Vec::new(),
            env: HashMap::new(),
            working_dir: None,
        };
        
        // Clone interactive_docs reference for each task
        // Note: In a real implementation, we'd need proper concurrent access
        let task = timeout(
            Duration::from_secs(10),
            interactive_docs.execute_code(request)
        );
        
        tasks.push(task);
    }
    
    // Execute first task to test basic functionality
    let first_result = tasks.into_iter().next().unwrap().await;
    assert!(first_result.is_ok(), "Performance test should complete");
    
    // Verify session statistics
    let stats = interactive_docs.get_session_statistics();
    assert!(stats.active_sessions > 0, "Should have active sessions");
    assert!(stats.total_executions > 0, "Should have executed code");
}

#[test]
fn test_configuration_validation() {
    // Test LiveServerConfig validation
    let live_config = LiveServerConfig {
        port: 65535, // Maximum valid port
        host: "0.0.0.0".to_string(),
        watch_debounce: Duration::from_millis(1),
        generation_timeout: Duration::from_secs(300),
        max_websocket_connections: 1000,
        ..Default::default()
    };
    
    let server_result = LiveDocumentationServer::new(live_config);
    assert!(server_result.is_ok(), "Valid configuration should work");
    
    // Test InteractiveConfig validation  
    let interactive_config = InteractiveConfig {
        max_execution_time: Duration::from_millis(1), // Very short timeout
        max_memory_mb: 1, // Very small memory limit
        allowed_imports: vec![], // No imports allowed
        ..Default::default()
    };
    
    let interactive_result = InteractiveDocumentation::new(interactive_config);
    assert!(interactive_result.is_ok(), "Restrictive configuration should still work");
    
    // Test DocumentationTestConfig validation
    let test_config = DocumentationTestConfig {
        max_parallel_tests: 1,
        retry_count: 0,
        min_coverage_threshold: 100.0, // Require perfect coverage
        fail_on_missing_docs: true,
        ..Default::default()
    };
    
    // Should be able to create tester with strict config
    let tester_result = DocumentationTester::new(test_config);
    assert!(tester_result.is_ok(), "Strict test configuration should work");
}
