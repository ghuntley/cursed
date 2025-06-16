//! Enhanced Documentation Generation Tests
//!
//! Tests for the enhanced documentation generation system with real parameter
//! and return type parsing using AST infrastructure.

use cursed::documentation::generator::{DocumentationGenerator, DocGeneratorConfig, OutputFormat};
use cursed::documentation::{DocumentationConfig, DocumentationOptions, ProjectInfo};
use cursed::error::Error;
use std::path::PathBuf;
use tempfile::TempDir;
use tracing::info;

#[path = "common.rs"]
mod common;

/// Test enhanced method parameter parsing
#[tokio::test]
async fn test_enhanced_method_parameter_parsing() -> Result<(), Error> {
    common::tracing::setup();
    info!("Testing enhanced method parameter parsing");

    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(temp_dir.path());
    let generator = DocumentationGenerator::new(config)?;

    // Sample CURSED code with various parameter patterns
    let source_code = r#"
/// Test struct for documentation
squad TestStruct {
    name: String,
    count: i32,
}

impl TestStruct {
    /// Create a new TestStruct instance
    /// @param name The name to assign
    /// @param count The initial count value
    pub fn new(name: String, count: i32) -> TestStruct {
        TestStruct { name, count }
    }

    /// Update with optional parameters
    pub fn update(&mut self, name: Option<String>, count: i32 = 0) -> bool {
        if let Some(n) = name {
            self.name = n;
        }
        self.count = count;
        true
    }

    /// Generic method with constraints
    pub fn process<T: Clone + Send>(self, data: T, callback: fn(T) -> String) -> Result<String, Error> {
        let result = callback(data);
        Ok(result)
    }

    /// Async method with complex parameters
    pub async fn fetch_data(&self, url: &str, headers: Vec<(String, String)>) -> Result<Vec<u8>, NetworkError> {
        // Implementation would go here
        Ok(vec![])
    }

    /// Method with self variants
    fn consume(self) -> String {
        self.name
    }

    /// Method with mutable reference
    fn modify(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    /// Method with immutable reference  
    fn get_info(&self) -> (String, i32) {
        (self.name.clone(), self.count)
    }
}
"#;

    let file_path = PathBuf::from("test_struct.csd");
    
    // Create a mock AST node for testing
    use cursed::ast::{AstNode, AstNodeType, Program};
    let mock_ast = AstNode {
        node_type: AstNodeType::Program(Program {
            statements: Vec::new(),
        }),
        location: cursed::error::SourceLocation {
            file: file_path.to_string_lossy().to_string(),
            line: 1,
            column: 1,
        },
    };

    // Extract documentation from the AST
    let extracted = generator.extract_from_ast(&mock_ast, &file_path, source_code).await?;

    // Verify method extraction and parameter parsing
    assert!(!extracted.types.is_empty(), "Should extract TestStruct type");
    
    let test_struct = &extracted.types[0];
    assert_eq!(test_struct.name, "TestStruct");
    assert!(!test_struct.methods.is_empty(), "Should extract methods from impl block");

    // Find specific methods and verify their parameters
    let new_method = test_struct.methods.iter()
        .find(|m| m.name == "new")
        .expect("Should find 'new' method");
    
    // Verify new method parameters
    assert_eq!(new_method.parameters.len(), 2, "new() should have 2 parameters");
    assert_eq!(new_method.parameters[0].name, "name");
    assert_eq!(new_method.parameters[0].param_type, "String");
    assert_eq!(new_method.parameters[1].name, "count");
    assert_eq!(new_method.parameters[1].param_type, "i32");

    // Verify return type
    assert!(new_method.return_type.is_some(), "new() should have return type");
    assert_eq!(new_method.return_type.as_ref().unwrap().name, "TestStruct");

    // Find update method and verify optional parameters
    let update_method = test_struct.methods.iter()
        .find(|m| m.name == "update")
        .expect("Should find 'update' method");
    
    assert_eq!(update_method.parameters.len(), 3, "update() should have 3 parameters including self");
    
    // Verify self parameter
    assert_eq!(update_method.parameters[0].name, "self");
    assert_eq!(update_method.parameters[0].param_type, "Self");
    
    // Verify optional parameter
    let name_param = &update_method.parameters[1];
    assert_eq!(name_param.name, "name");
    assert_eq!(name_param.param_type, "Option<String>");
    
    // Verify parameter with default value
    let count_param = &update_method.parameters[2];
    assert_eq!(count_param.name, "count");
    assert_eq!(count_param.param_type, "i32");
    assert!(count_param.is_optional, "count parameter should be optional");
    assert_eq!(count_param.default_value.as_ref().unwrap(), "0");

    info!("Enhanced method parameter parsing test completed successfully");
    Ok(())
}

/// Test enhanced generic type parsing
#[tokio::test]
async fn test_enhanced_generic_type_parsing() -> Result<(), Error> {
    common::tracing::setup();
    info!("Testing enhanced generic type parsing");

    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(temp_dir.path());
    let generator = DocumentationGenerator::new(config)?;

    let source_code = r#"
/// Generic container struct
squad Container<T, E = Error> {
    value: T,
    error: Option<E>,
}

impl<T: Clone, E: std::error::Error> Container<T, E> {
    /// Create new container with generic constraints
    pub fn new<U: Into<T>>(input: U) -> Container<T, E> {
        Container {
            value: input.into(),
            error: None,
        }
    }

    /// Map function with complex generics
    pub fn map<U, F>(self, f: F) -> Container<U, E> 
    where 
        F: FnOnce(T) -> U,
        U: Clone + Send + Sync,
    {
        Container {
            value: f(self.value),
            error: self.error,
        }
    }

    /// Method with nested generics
    pub fn combine<Other>(self, other: Container<Other, E>) -> Container<(T, Other), E> {
        Container {
            value: (self.value, other.value),
            error: self.error.or(other.error),
        }
    }
}
"#;

    let file_path = PathBuf::from("test_generic.csd");
    
    // Create a mock AST node
    use cursed::ast::{AstNode, AstNodeType, Program};
    let mock_ast = AstNode {
        node_type: AstNodeType::Program(Program {
            statements: Vec::new(),
        }),
        location: cursed::error::SourceLocation {
            file: file_path.to_string_lossy().to_string(),
            line: 1,
            column: 1,
        },
    };

    let extracted = generator.extract_from_ast(&mock_ast, &file_path, source_code).await?;

    // Verify generic type extraction
    assert!(!extracted.types.is_empty(), "Should extract Container type");
    
    let container_type = &extracted.types[0];
    assert_eq!(container_type.name, "Container");
    assert!(!container_type.generic_params.is_empty(), "Container should have generic parameters");
    
    // Verify generic parameters
    assert!(container_type.generic_params.contains(&"T".to_string()));
    assert!(container_type.generic_params.contains(&"E".to_string()));

    // Verify methods with generic parameters
    let new_method = container_type.methods.iter()
        .find(|m| m.name == "new")
        .expect("Should find 'new' method");
    
    assert!(!new_method.generic_params.is_empty(), "new() should have generic parameters");
    assert!(new_method.generic_params.contains(&"U".to_string()));

    // Verify complex return types with generics
    assert!(new_method.return_type.is_some());
    let return_type = new_method.return_type.as_ref().unwrap();
    assert!(return_type.name.contains("Container"), "Return type should be Container<T, E>");

    info!("Enhanced generic type parsing test completed successfully");
    Ok(())
}

/// Test CURSED-specific syntax parsing
#[tokio::test]
async fn test_cursed_syntax_parsing() -> Result<(), Error> {
    common::tracing::setup();
    info!("Testing CURSED-specific syntax parsing");

    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(temp_dir.path());
    let generator = DocumentationGenerator::new(config)?;

    let source_code = r#"
/// CURSED HTTP server implementation
squad HttpServer {
    port: u16,
    routes: Vec<Route>,
}

impl HttpServer {
    /// Create new server - slay the initialization!
    pub slay new(port: u16) -> HttpServer {
        HttpServer {
            port,
            routes: Vec::new(),
        }
    }

    /// Start server with yolo error handling
    pub async yolo start(&mut self) -> Result<(), ServerError> {
        // Server startup logic
        Ok(())
    }

    /// Add route with facts (immutable) configuration
    pub facts add_route(&mut self, path: String, handler: fn() -> String) -> bool {
        let route = Route { path, handler };
        self.routes.push(route);
        true
    }

    /// Check server status - periodt!
    pub periodt is_running(&self) -> bool {
        // Check if server is running
        true
    }

    /// Stop server with bestie/flex pattern
    pub bestie stop(self) -> flex Result<(), Error> {
        // Stop server logic
        Ok(())
    }
}

/// Route configuration
squad Route {
    path: String,
    handler: fn() -> String,
}
"#;

    let file_path = PathBuf::from("test_cursed_syntax.csd");
    
    use cursed::ast::{AstNode, AstNodeType, Program};
    let mock_ast = AstNode {
        node_type: AstNodeType::Program(Program {
            statements: Vec::new(),
        }),
        location: cursed::error::SourceLocation {
            file: file_path.to_string_lossy().to_string(),
            line: 1,
            column: 1,
        },
    };

    let extracted = generator.extract_from_ast(&mock_ast, &file_path, source_code).await?;

    // Verify CURSED syntax recognition
    assert!(!extracted.types.is_empty(), "Should extract types with CURSED syntax");
    
    let http_server = &extracted.types[0];
    assert_eq!(http_server.name, "HttpServer");
    
    // Verify methods with CURSED keywords are recognized
    let method_names: Vec<&str> = http_server.methods.iter()
        .map(|m| m.name.as_str())
        .collect();
    
    assert!(method_names.contains(&"new"), "Should find 'new' method with 'slay' keyword");
    assert!(method_names.contains(&"start"), "Should find 'start' method with 'yolo' keyword");
    assert!(method_names.contains(&"add_route"), "Should find 'add_route' method with 'facts' keyword");
    assert!(method_names.contains(&"is_running"), "Should find 'is_running' method with 'periodt' keyword");
    assert!(method_names.contains(&"stop"), "Should find 'stop' method with 'bestie/flex' keywords");

    // Verify async method recognition
    let start_method = http_server.methods.iter()
        .find(|m| m.name == "start")
        .expect("Should find start method");
    assert!(start_method.is_async, "start() method should be recognized as async");

    info!("CURSED syntax parsing test completed successfully");
    Ok(())
}

/// Test complex type signatures and error handling
#[tokio::test]
async fn test_complex_type_signatures() -> Result<(), Error> {
    common::tracing::setup();
    info!("Testing complex type signatures");

    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(temp_dir.path());
    let generator = DocumentationGenerator::new(config)?;

    let source_code = r#"
/// Database connection manager
squad DatabaseManager {
    connections: HashMap<String, Connection>,
}

impl DatabaseManager {
    /// Execute query with complex return type
    pub async fn execute_query<T, E>(
        &self, 
        query: &str, 
        params: Vec<Box<dyn ToSql + Send>>,
        mapper: impl Fn(Row) -> Result<T, E> + Send + Sync
    ) -> Result<Vec<T>, Box<dyn Error + Send + Sync>> {
        // Query execution logic
        Ok(vec![])
    }

    /// Transaction with nested closure types
    pub fn with_transaction<F, R, E>(
        &mut self,
        f: F
    ) -> Result<R, TransactionError<E>>
    where
        F: FnOnce(&mut Transaction) -> Result<R, E>,
        E: std::error::Error + Send + Sync + 'static,
        R: Send + Sync,
    {
        // Transaction logic
        todo!()
    }

    /// Method with lifetime parameters
    pub fn get_connection<'a>(
        &'a self, 
        name: &str
    ) -> Option<&'a Connection> {
        self.connections.get(name)
    }

    /// Method with complex trait bounds
    pub fn stream_results<T, S>(
        &self,
        query: String
    ) -> impl Stream<Item = Result<T, DatabaseError>> + Send + '_
    where
        T: DeserializeOwned + Send + Sync,
        S: Serialize + Send,
    {
        // Stream implementation
        futures::stream::empty()
    }
}
"#;

    let file_path = PathBuf::from("test_complex_types.csd");
    
    use cursed::ast::{AstNode, AstNodeType, Program};
    let mock_ast = AstNode {
        node_type: AstNodeType::Program(Program {
            statements: Vec::new(),
        }),
        location: cursed::error::SourceLocation {
            file: file_path.to_string_lossy().to_string(),
            line: 1,
            column: 1,
        },
    };

    let extracted = generator.extract_from_ast(&mock_ast, &file_path, source_code).await?;

    assert!(!extracted.types.is_empty(), "Should extract DatabaseManager type");
    
    let db_manager = &extracted.types[0];
    assert_eq!(db_manager.name, "DatabaseManager");

    // Verify complex method signatures are parsed
    let execute_query = db_manager.methods.iter()
        .find(|m| m.name == "execute_query")
        .expect("Should find execute_query method");

    // Verify complex parameter types
    assert!(!execute_query.parameters.is_empty(), "execute_query should have parameters");
    
    // Verify complex return type
    assert!(execute_query.return_type.is_some(), "execute_query should have return type");
    let return_type = execute_query.return_type.as_ref().unwrap();
    assert!(return_type.name.contains("Result"), "Return type should be Result type");
    assert!(return_type.name.contains("Vec"), "Return type should contain Vec");

    // Verify generic parameters
    assert!(!execute_query.generic_params.is_empty(), "execute_query should have generic parameters");
    assert!(execute_query.generic_params.contains(&"T".to_string()));
    assert!(execute_query.generic_params.contains(&"E".to_string()));

    info!("Complex type signatures test completed successfully");
    Ok(())
}

/// Test documentation output format consistency
#[tokio::test]
async fn test_output_format_consistency() -> Result<(), Error> {
    common::tracing::setup();
    info!("Testing output format consistency");

    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(temp_dir.path());
    let generator = DocumentationGenerator::new(config)?;

    let source_code = r#"
/// Sample API module
squad ApiClient {
    base_url: String,
    timeout: Duration,
}

impl ApiClient {
    /// Create new API client
    pub fn new(base_url: String, timeout: Duration) -> ApiClient {
        ApiClient { base_url, timeout }
    }

    /// Make HTTP request
    pub async fn request<T: Serialize>(
        &self, 
        method: HttpMethod, 
        path: &str, 
        body: Option<T>
    ) -> Result<Response, ApiError> {
        // HTTP request logic
        Ok(Response::new())
    }
}
"#;

    let file_path = PathBuf::from("test_api.csd");
    
    use cursed::ast::{AstNode, AstNodeType, Program};
    let mock_ast = AstNode {
        node_type: AstNodeType::Program(Program {
            statements: Vec::new(),
        }),
        location: cursed::error::SourceLocation {
            file: file_path.to_string_lossy().to_string(),
            line: 1,
            column: 1,
        },
    };

    let extracted = generator.extract_from_ast(&mock_ast, &file_path, source_code).await?;
    let extracted_docs = vec![extracted];

    // Test all output formats
    let formats = vec![
        OutputFormat::Html,
        OutputFormat::Markdown,
        OutputFormat::Json,
        OutputFormat::Xml,
        OutputFormat::LaTeX,
    ];

    for format in formats {
        info!("Testing {} output format", format);
        
        let output_files = generator.generate_output(
            &extracted_docs,
            &std::collections::HashMap::new(),
            &[],
            format.clone(),
        ).await?;

        assert!(!output_files.is_empty(), "Should generate output files for {}", format);
        
        // Verify files were created
        for file_path in &output_files {
            assert!(file_path.exists(), "Output file should exist: {:?}", file_path);
            
            let content = std::fs::read_to_string(file_path)
                .expect("Should be able to read output file");
            assert!(!content.is_empty(), "Output file should not be empty");
            
            // Verify content contains method information
            assert!(content.contains("ApiClient"), "Should contain type name");
            assert!(content.contains("new"), "Should contain method name");
            assert!(content.contains("request"), "Should contain method name");
        }
    }

    info!("Output format consistency test completed successfully");
    Ok(())
}

/// Test error handling and malformed code
#[tokio::test]
async fn test_error_handling_and_malformed_code() -> Result<(), Error> {
    common::tracing::setup();
    info!("Testing error handling with malformed code");

    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(temp_dir.path());
    let generator = DocumentationGenerator::new(config)?;

    // Test various malformed code scenarios
    let malformed_cases = vec![
        // Missing closing brace
        r#"
        squad TestStruct {
            field: String,
        
        impl TestStruct {
            pub fn test() -> String {
                "test".to_string()
            // Missing closing brace
        "#,
        
        // Invalid parameter syntax
        r#"
        squad TestStruct {}
        
        impl TestStruct {
            pub fn invalid_params(param1: , param2 String) -> bool {
                true
            }
        }
        "#,
        
        // Incomplete method signature
        r#"
        squad TestStruct {}
        
        impl TestStruct {
            pub fn incomplete(
        }
        "#,
    ];

    for (i, malformed_code) in malformed_cases.iter().enumerate() {
        info!("Testing malformed case {}", i + 1);
        
        let file_path = PathBuf::from(format!("test_malformed_{}.csd", i));
        
        use cursed::ast::{AstNode, AstNodeType, Program};
        let mock_ast = AstNode {
            node_type: AstNodeType::Program(Program {
                statements: Vec::new(),
            }),
            location: cursed::error::SourceLocation {
                file: file_path.to_string_lossy().to_string(),
                line: 1,
                column: 1,
            },
        };

        // Should not panic or crash, even with malformed code
        let result = generator.extract_from_ast(&mock_ast, &file_path, malformed_code).await;
        
        // Either succeeds with partial extraction or fails gracefully
        match result {
            Ok(extracted) => {
                info!("Malformed case {} handled gracefully with {} items", i + 1, extracted.metadata.item_count);
            }
            Err(error) => {
                info!("Malformed case {} failed gracefully: {:?}", i + 1, error);
            }
        }
    }

    info!("Error handling test completed successfully");
    Ok(())
}

/// Helper function to create test configuration
fn create_test_config(output_dir: &std::path::Path) -> DocumentationConfig {
    DocumentationConfig {
        project: ProjectInfo {
            name: "Test Project".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test project for documentation generation".to_string()),
            authors: vec!["Test Author".to_string()],
            homepage: Some("https://example.com".to_string()),
            repository: Some("https://github.com/example/test".to_string()),
        },
        output_dir: output_dir.to_path_buf(),
        options: DocumentationOptions {
            include_private: true,
            include_source: true,
            generate_cross_refs: true,
            generate_search_index: true,
            max_type_depth: 10,
            include_examples: true,
            output_formats: vec![OutputFormat::Html, OutputFormat::Markdown],
            template_dir: None,
            custom_css: None,
            custom_js: None,
        },
    }
}

/// Test parameter parsing edge cases
#[tokio::test]
async fn test_parameter_parsing_edge_cases() -> Result<(), Error> {
    common::tracing::setup();
    info!("Testing parameter parsing edge cases");

    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(temp_dir.path());
    let generator = DocumentationGenerator::new(config)?;

    let source_code = r#"
/// Edge case testing struct
squad EdgeCaseStruct {}

impl EdgeCaseStruct {
    /// Method with no parameters
    pub fn no_params() -> bool {
        true
    }

    /// Method with only self
    pub fn only_self(&self) -> String {
        "test".to_string()
    }

    /// Method with trailing comma
    pub fn trailing_comma(&self, param1: String, param2: i32,) -> bool {
        true
    }

    /// Method with complex default values
    pub fn complex_defaults(
        &self,
        name: String = "default".to_string(),
        config: Config = Config::default(),
        numbers: Vec<i32> = vec![1, 2, 3],
    ) -> Result<(), Error> {
        Ok(())
    }

    /// Method with function pointer parameters
    pub fn with_function_pointers(
        callback: fn(String) -> bool,
        complex_callback: fn(&str, i32) -> Result<String, Error>,
    ) -> bool {
        true
    }

    /// Method with lifetime parameters in types
    pub fn with_lifetimes<'a>(
        &'a self,
        data: &'a str,
        optional: Option<&'a mut String>,
    ) -> &'a str {
        data
    }
}
"#;

    let file_path = PathBuf::from("test_edge_cases.csd");
    
    use cursed::ast::{AstNode, AstNodeType, Program};
    let mock_ast = AstNode {
        node_type: AstNodeType::Program(Program {
            statements: Vec::new(),
        }),
        location: cursed::error::SourceLocation {
            file: file_path.to_string_lossy().to_string(),
            line: 1,
            column: 1,
        },
    };

    let extracted = generator.extract_from_ast(&mock_ast, &file_path, source_code).await?;

    assert!(!extracted.types.is_empty(), "Should extract EdgeCaseStruct");
    
    let edge_struct = &extracted.types[0];
    assert_eq!(edge_struct.name, "EdgeCaseStruct");

    // Test no parameters method
    let no_params = edge_struct.methods.iter()
        .find(|m| m.name == "no_params")
        .expect("Should find no_params method");
    assert!(no_params.parameters.is_empty(), "no_params should have no parameters");

    // Test only self method
    let only_self = edge_struct.methods.iter()
        .find(|m| m.name == "only_self")
        .expect("Should find only_self method");
    assert_eq!(only_self.parameters.len(), 1, "only_self should have 1 parameter");
    assert_eq!(only_self.parameters[0].name, "self");

    // Test trailing comma handling
    let trailing_comma = edge_struct.methods.iter()
        .find(|m| m.name == "trailing_comma")
        .expect("Should find trailing_comma method");
    assert_eq!(trailing_comma.parameters.len(), 3, "trailing_comma should handle trailing comma correctly");

    // Test complex default values
    let complex_defaults = edge_struct.methods.iter()
        .find(|m| m.name == "complex_defaults")
        .expect("Should find complex_defaults method");
    
    // Should handle complex default values gracefully
    let has_defaults = complex_defaults.parameters.iter()
        .any(|p| p.is_optional && p.default_value.is_some());
    assert!(has_defaults, "Should detect parameters with default values");

    info!("Parameter parsing edge cases test completed successfully");
    Ok(())
}
