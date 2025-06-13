//! Output format generation tests for AST documentation
//!
//! This test suite validates the generation of different output formats
//! from the extracted documentation data including JSON, Markdown, and HTML.

#[path = "common/mod.rs"]
mod common;

use cursed::ast::documentation::*;
use cursed::ast::*;
use cursed::error::{Error, SourceLocation};
use std::collections::HashMap;
use std::path::PathBuf;
use serde_json;

/// Test JSON serialization of documentation elements
#[test]
fn test_json_serialization() {
    common::tracing::setup();
    
    // Create a comprehensive documentation element
    let location = SourceLocation { 
        line: 10, 
        column: 5, 
        file: Some("example.csd".to_string()) 
    };
    
    let mut tags = HashMap::new();
    tags.insert("author".to_string(), vec!["Alice".to_string(), "Bob".to_string()]);
    tags.insert("since".to_string(), vec!["1.0.0".to_string()]);
    tags.insert("deprecated".to_string(), vec!["Use new_function instead".to_string()]);
    
    let param = ParameterDoc {
        name: "input".to_string(),
        param_type: Some("String".to_string()),
        description: "Input data to process".to_string(),
        default_value: Some("\"default\"".to_string()),
        is_optional: true,
    };
    
    let example = CodeExample {
        title: Some("Basic Usage".to_string()),
        description: Some("Demonstrates the function usage".to_string()),
        code: r#"sus result = process_data("hello");
println!("Result: {}", result);"#.to_string(),
        language: "cursed".to_string(),
        output: Some("Result: HELLO".to_string()),
        is_runnable: true,
    };
    
    let type_info = TypeInfo {
        base_type: "function".to_string(),
        generic_params: vec!["T".to_string()],
        constraints: vec!["T: Display".to_string()],
        fields: Vec::new(),
        methods: Vec::new(),
    };
    
    let mut metadata = ElementMetadata::default();
    metadata.since_version = Some("1.0.0".to_string());
    metadata.is_deprecated = true;
    metadata.deprecation_message = Some("Use new_function instead".to_string());
    metadata.stability = StabilityLevel::Deprecated;
    metadata.performance_notes.push("O(n) complexity".to_string());
    metadata.security_notes.push("Validates input".to_string());
    
    let doc_element = DocElement {
        name: "process_data".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "data_processing".to_string(),
        summary: "Processes input data and returns formatted result".to_string(),
        description: Some("This function takes input data, validates it, and returns a processed version. It supports various input formats and provides comprehensive error handling.".to_string()),
        signature: Some("slay process_data<T: Display>(input: String = \"default\") -> String".to_string()),
        parameters: vec![param],
        return_type: Some("String".to_string()),
        type_info: Some(type_info),
        examples: vec![example],
        tags,
        location,
        source_code: Some("slay process_data<T>(input: String) -> String { periodt input.to_uppercase(); }".to_string()),
        metadata,
    };
    
    // Test JSON serialization
    let json_result = serde_json::to_string_pretty(&doc_element);
    assert!(json_result.is_ok());
    
    let json_string = json_result.unwrap();
    
    // Verify key elements are present in JSON
    assert!(json_string.contains("\"name\": \"process_data\""));
    assert!(json_string.contains("\"element_type\": \"Function\""));
    assert!(json_string.contains("\"visibility\": \"Public\""));
    assert!(json_string.contains("\"module\": \"data_processing\""));
    assert!(json_string.contains("\"summary\": \"Processes input data"));
    assert!(json_string.contains("\"is_deprecated\": true"));
    assert!(json_string.contains("\"since_version\": \"1.0.0\""));
    assert!(json_string.contains("\"author\""));
    assert!(json_string.contains("\"Alice\""));
    assert!(json_string.contains("\"Bob\""));
    
    // Test deserialization
    let deserialized_result: Result<DocElement, _> = serde_json::from_str(&json_string);
    assert!(deserialized_result.is_ok());
    
    let deserialized = deserialized_result.unwrap();
    assert_eq!(deserialized.name, doc_element.name);
    assert_eq!(deserialized.element_type, doc_element.element_type);
    assert_eq!(deserialized.visibility, doc_element.visibility);
    assert_eq!(deserialized.module, doc_element.module);
    assert_eq!(deserialized.summary, doc_element.summary);
}

/// Test module documentation JSON serialization
#[test]
fn test_module_documentation_json() {
    common::tracing::setup();
    
    let package_info = PackageInfo {
        name: Some("test_package".to_string()),
        description: Some("A comprehensive test package".to_string()),
        version: Some("2.1.0".to_string()),
        authors: vec!["Test Author".to_string()],
        dependencies: vec!["std".to_string(), "serde".to_string()],
    };
    
    let import_doc = ImportDoc {
        path: "std::collections::HashMap".to_string(),
        alias: Some("Map".to_string()),
        description: "Standard library hash map".to_string(),
        is_public: false,
    };
    
    let location = SourceLocation { line: 5, column: 1, file: Some("module.csd".to_string()) };
    let func_element = DocElement {
        name: "helper_function".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Private,
        module: "test_module".to_string(),
        summary: "Helper function for internal use".to_string(),
        description: None,
        signature: Some("slay helper_function() -> i32".to_string()),
        parameters: Vec::new(),
        return_type: Some("i32".to_string()),
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location,
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    let source_info = SourceInfo {
        file_size: 1024,
        line_count: 100,
        last_modified: Some(std::time::SystemTime::now()),
        encoding: "UTF-8".to_string(),
    };
    
    let module_doc = ModuleDocumentation {
        name: "test_module".to_string(),
        file_path: PathBuf::from("src/test_module.csd"),
        package_info: Some(package_info),
        imports: vec![import_doc],
        items: vec![func_element],
        module_comments: vec!["This is a test module".to_string(), "It demonstrates documentation".to_string()],
        metadata: DocumentationMetadata::new("test_module"),
        source_info,
    };
    
    // Test JSON serialization
    let json_result = serde_json::to_string_pretty(&module_doc);
    assert!(json_result.is_ok());
    
    let json_string = json_result.unwrap();
    
    // Verify module structure in JSON
    assert!(json_string.contains("\"name\": \"test_module\""));
    assert!(json_string.contains("\"package_info\""));
    assert!(json_string.contains("\"test_package\""));
    assert!(json_string.contains("\"version\": \"2.1.0\""));
    assert!(json_string.contains("\"imports\""));
    assert!(json_string.contains("\"std::collections::HashMap\""));
    assert!(json_string.contains("\"alias\": \"Map\""));
    assert!(json_string.contains("\"items\""));
    assert!(json_string.contains("\"helper_function\""));
    assert!(json_string.contains("\"module_comments\""));
    assert!(json_string.contains("\"This is a test module\""));
    assert!(json_string.contains("\"file_size\": 1024"));
    assert!(json_string.contains("\"line_count\": 100"));
    
    // Test deserialization
    let deserialized_result: Result<ModuleDocumentation, _> = serde_json::from_str(&json_string);
    assert!(deserialized_result.is_ok());
    
    let deserialized = deserialized_result.unwrap();
    assert_eq!(deserialized.name, module_doc.name);
    assert_eq!(deserialized.imports.len(), 1);
    assert_eq!(deserialized.items.len(), 1);
    assert_eq!(deserialized.module_comments.len(), 2);
}

/// Test exported documentation JSON structure
#[test]
fn test_exported_documentation_json() {
    common::tracing::setup();
    
    // Create multiple modules
    let module1 = ModuleDocumentation {
        name: "module1".to_string(),
        file_path: PathBuf::from("module1.csd"),
        package_info: None,
        imports: Vec::new(),
        items: Vec::new(),
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("module1"),
        source_info: SourceInfo {
            file_size: 100,
            line_count: 10,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    let module2 = ModuleDocumentation {
        name: "module2".to_string(),
        file_path: PathBuf::from("module2.csd"),
        package_info: None,
        imports: Vec::new(),
        items: Vec::new(),
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("module2"),
        source_info: SourceInfo {
            file_size: 200,
            line_count: 20,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    // Create cross-references
    let mut cross_references = HashMap::new();
    let location = SourceLocation { line: 1, column: 1, file: None };
    cross_references.insert("module1::function_a".to_string(), vec![
        CrossReference {
            target: "module2::function_b".to_string(),
            context: "function call".to_string(),
            location: location.clone(),
            reference_type: ReferenceType::Usage,
        }
    ]);
    
    // Create symbol table
    let mut symbol_table = HashMap::new();
    symbol_table.insert("module1::function_a".to_string(), DocumentedSymbol {
        name: "function_a".to_string(),
        element_type: ElementType::Function,
        module: "module1".to_string(),
        location: location.clone(),
        signature: Some("slay function_a() -> void".to_string()),
    });
    
    let exported_doc = ExportedDocumentation {
        modules: vec![module1, module2],
        cross_references,
        symbol_table,
        metadata: ExportMetadata {
            generator_version: "1.0.0".to_string(),
            generated_at: chrono::Utc::now(),
            total_modules: 2,
            total_items: 0,
        },
    };
    
    // Test JSON serialization
    let json_result = serde_json::to_string_pretty(&exported_doc);
    assert!(json_result.is_ok());
    
    let json_string = json_result.unwrap();
    
    // Verify exported structure
    assert!(json_string.contains("\"modules\""));
    assert!(json_string.contains("\"cross_references\""));
    assert!(json_string.contains("\"symbol_table\""));
    assert!(json_string.contains("\"metadata\""));
    assert!(json_string.contains("\"generator_version\": \"1.0.0\""));
    assert!(json_string.contains("\"total_modules\": 2"));
    assert!(json_string.contains("\"module1\""));
    assert!(json_string.contains("\"module2\""));
    assert!(json_string.contains("\"function_a\""));
    assert!(json_string.contains("\"function_b\""));
    
    // Test deserialization
    let deserialized_result: Result<ExportedDocumentation, _> = serde_json::from_str(&json_string);
    assert!(deserialized_result.is_ok());
    
    let deserialized = deserialized_result.unwrap();
    assert_eq!(deserialized.modules.len(), 2);
    assert_eq!(deserialized.metadata.total_modules, 2);
    assert_eq!(deserialized.metadata.generator_version, "1.0.0");
}

/// Test markdown-like string generation from documentation
#[test]
fn test_markdown_string_generation() {
    common::tracing::setup();
    
    // Create a function with comprehensive documentation
    let location = SourceLocation { line: 15, column: 1, file: Some("math.csd".to_string()) };
    
    let param1 = ParameterDoc {
        name: "x".to_string(),
        param_type: Some("f64".to_string()),
        description: "First number".to_string(),
        default_value: None,
        is_optional: false,
    };
    
    let param2 = ParameterDoc {
        name: "y".to_string(),
        param_type: Some("f64".to_string()),
        description: "Second number".to_string(),
        default_value: Some("0.0".to_string()),
        is_optional: true,
    };
    
    let example = CodeExample {
        title: Some("Basic Addition".to_string()),
        description: Some("Add two numbers together".to_string()),
        code: r#"sus result = add_numbers(5.0, 3.0);
println!("Result: {}", result);"#.to_string(),
        language: "cursed".to_string(),
        output: Some("Result: 8.0".to_string()),
        is_runnable: true,
    };
    
    let mut tags = HashMap::new();
    tags.insert("author".to_string(), vec!["Math Team".to_string()]);
    tags.insert("since".to_string(), vec!["1.0.0".to_string()]);
    tags.insert("performance".to_string(), vec!["O(1) constant time".to_string()]);
    
    let func_element = DocElement {
        name: "add_numbers".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "math".to_string(),
        summary: "Adds two floating-point numbers".to_string(),
        description: Some("This function performs addition of two floating-point numbers with optional second parameter. It handles edge cases like infinity and NaN values gracefully.".to_string()),
        signature: Some("slay add_numbers(x: f64, y: f64 = 0.0) -> f64".to_string()),
        parameters: vec![param1, param2],
        return_type: Some("f64".to_string()),
        type_info: None,
        examples: vec![example],
        tags,
        location,
        source_code: Some("slay add_numbers(x: f64, y: f64 = 0.0) -> f64 { periodt x + y; }".to_string()),
        metadata: ElementMetadata::default(),
    };
    
    // Generate markdown-like string representation
    let markdown = generate_markdown_for_element(&func_element);
    
    // Verify markdown content
    assert!(markdown.contains("# add_numbers"));
    assert!(markdown.contains("**Function**"));
    assert!(markdown.contains("**Module:** math"));
    assert!(markdown.contains("**Visibility:** Public"));
    assert!(markdown.contains("Adds two floating-point numbers"));
    assert!(markdown.contains("This function performs addition"));
    assert!(markdown.contains("## Signature"));
    assert!(markdown.contains("slay add_numbers(x: f64, y: f64 = 0.0) -> f64"));
    assert!(markdown.contains("## Parameters"));
    assert!(markdown.contains("- **x** (`f64`): First number"));
    assert!(markdown.contains("- **y** (`f64`, optional): Second number"));
    assert!(markdown.contains("## Return Type"));
    assert!(markdown.contains("f64"));
    assert!(markdown.contains("## Examples"));
    assert!(markdown.contains("### Basic Addition"));
    assert!(markdown.contains("```cursed"));
    assert!(markdown.contains("sus result = add_numbers(5.0, 3.0);"));
    assert!(markdown.contains("## Tags"));
    assert!(markdown.contains("**author:** Math Team"));
    assert!(markdown.contains("**since:** 1.0.0"));
    assert!(markdown.contains("**performance:** O(1) constant time"));
}

/// Helper function to generate markdown representation
fn generate_markdown_for_element(element: &DocElement) -> String {
    let mut md = String::new();
    
    // Title
    md.push_str(&format!("# {}\n\n", element.name));
    
    // Element type and basic info
    md.push_str(&format!("**{}** - **Module:** {} - **Visibility:** {:?}\n\n", 
        element.element_type, element.module, element.visibility));
    
    // Summary
    md.push_str(&format!("{}\n\n", element.summary));
    
    // Description
    if let Some(description) = &element.description {
        md.push_str(&format!("{}\n\n", description));
    }
    
    // Signature
    if let Some(signature) = &element.signature {
        md.push_str("## Signature\n\n");
        md.push_str(&format!("```cursed\n{}\n```\n\n", signature));
    }
    
    // Parameters
    if !element.parameters.is_empty() {
        md.push_str("## Parameters\n\n");
        for param in &element.parameters {
            let optional = if param.is_optional { ", optional" } else { "" };
            let param_type = param.param_type.as_ref().map(|t| format!(" (`{}`)", t)).unwrap_or_default();
            md.push_str(&format!("- **{}**{}{}: {}\n", param.name, param_type, optional, param.description));
            
            if let Some(default) = &param.default_value {
                md.push_str(&format!("  - Default: `{}`\n", default));
            }
        }
        md.push('\n');
    }
    
    // Return type
    if let Some(return_type) = &element.return_type {
        md.push_str(&format!("## Return Type\n\n{}\n\n", return_type));
    }
    
    // Examples
    if !element.examples.is_empty() {
        md.push_str("## Examples\n\n");
        for example in &element.examples {
            if let Some(title) = &example.title {
                md.push_str(&format!("### {}\n\n", title));
            }
            
            if let Some(description) = &example.description {
                md.push_str(&format!("{}\n\n", description));
            }
            
            md.push_str(&format!("```{}\n{}\n```\n\n", example.language, example.code));
            
            if let Some(output) = &example.output {
                md.push_str(&format!("**Output:**\n```\n{}\n```\n\n", output));
            }
        }
    }
    
    // Tags
    if !element.tags.is_empty() {
        md.push_str("## Tags\n\n");
        for (tag, values) in &element.tags {
            md.push_str(&format!("**{}:** {}\n", tag, values.join(", ")));
        }
        md.push('\n');
    }
    
    // Source code
    if let Some(source) = &element.source_code {
        md.push_str("## Source Code\n\n");
        md.push_str(&format!("```cursed\n{}\n```\n\n", source));
    }
    
    md
}

/// Test HTML-like string generation from documentation
#[test]
fn test_html_string_generation() {
    common::tracing::setup();
    
    // Create a struct with documentation
    let location = SourceLocation { line: 10, column: 1, file: Some("user.csd".to_string()) };
    
    let field1 = FieldDoc {
        name: "id".to_string(),
        field_type: "u64".to_string(),
        description: "Unique user identifier".to_string(),
        is_public: true,
        default_value: None,
    };
    
    let field2 = FieldDoc {
        name: "name".to_string(),
        field_type: "String".to_string(),
        description: "User's display name".to_string(),
        is_public: true,
        default_value: Some("\"Anonymous\"".to_string()),
    };
    
    let method = MethodDoc {
        name: "get_display_name".to_string(),
        signature: "slay get_display_name(&self) -> &str".to_string(),
        description: "Returns the user's display name".to_string(),
        parameters: Vec::new(),
        return_type: Some("&str".to_string()),
        is_static: false,
    };
    
    let type_info = TypeInfo {
        base_type: "struct".to_string(),
        generic_params: Vec::new(),
        constraints: Vec::new(),
        fields: vec![field1, field2],
        methods: vec![method],
    };
    
    let struct_element = DocElement {
        name: "User".to_string(),
        element_type: ElementType::Struct,
        visibility: Visibility::Public,
        module: "user".to_string(),
        summary: "Represents a user in the system".to_string(),
        description: Some("The User struct contains all information about a registered user, including their unique identifier and display name.".to_string()),
        signature: Some("squad User".to_string()),
        parameters: Vec::new(),
        return_type: None,
        type_info: Some(type_info),
        examples: Vec::new(),
        tags: HashMap::new(),
        location,
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    // Generate HTML-like string representation
    let html = generate_html_for_element(&struct_element);
    
    // Verify HTML content
    assert!(html.contains("<h1>User</h1>"));
    assert!(html.contains("<span class=\"element-type\">Struct</span>"));
    assert!(html.contains("<span class=\"module\">user</span>"));
    assert!(html.contains("<span class=\"visibility\">Public</span>"));
    assert!(html.contains("Represents a user in the system"));
    assert!(html.contains("The User struct contains all information"));
    assert!(html.contains("<h2>Signature</h2>"));
    assert!(html.contains("<code>squad User</code>"));
    assert!(html.contains("<h2>Fields</h2>"));
    assert!(html.contains("<li><strong>id</strong> (<code>u64</code>): Unique user identifier</li>"));
    assert!(html.contains("<li><strong>name</strong> (<code>String</code>): User's display name"));
    assert!(html.contains("Default: <code>\"Anonymous\"</code>"));
    assert!(html.contains("<h2>Methods</h2>"));
    assert!(html.contains("<strong>get_display_name</strong>"));
    assert!(html.contains("Returns the user's display name"));
}

/// Helper function to generate HTML representation
fn generate_html_for_element(element: &DocElement) -> String {
    let mut html = String::new();
    
    // Title with metadata
    html.push_str(&format!("<h1>{}</h1>\n", element.name));
    html.push_str(&format!("<div class=\"metadata\">\n"));
    html.push_str(&format!("  <span class=\"element-type\">{}</span>\n", element.element_type));
    html.push_str(&format!("  <span class=\"module\">{}</span>\n", element.module));
    html.push_str(&format!("  <span class=\"visibility\">{:?}</span>\n", element.visibility));
    html.push_str(&format!("</div>\n\n"));
    
    // Summary
    html.push_str(&format!("<p class=\"summary\">{}</p>\n\n", element.summary));
    
    // Description
    if let Some(description) = &element.description {
        html.push_str(&format!("<p class=\"description\">{}</p>\n\n", description));
    }
    
    // Signature
    if let Some(signature) = &element.signature {
        html.push_str("<h2>Signature</h2>\n");
        html.push_str(&format!("<pre><code>{}</code></pre>\n\n", signature));
    }
    
    // Type-specific information
    if let Some(type_info) = &element.type_info {
        // Fields
        if !type_info.fields.is_empty() {
            html.push_str("<h2>Fields</h2>\n<ul>\n");
            for field in &type_info.fields {
                html.push_str(&format!("  <li><strong>{}</strong> (<code>{}</code>): {}", 
                    field.name, field.field_type, field.description));
                
                if let Some(default) = &field.default_value {
                    html.push_str(&format!(" - Default: <code>{}</code>", default));
                }
                
                html.push_str("</li>\n");
            }
            html.push_str("</ul>\n\n");
        }
        
        // Methods
        if !type_info.methods.is_empty() {
            html.push_str("<h2>Methods</h2>\n<ul>\n");
            for method in &type_info.methods {
                html.push_str(&format!("  <li><strong>{}</strong>: {}</li>\n", 
                    method.name, method.description));
            }
            html.push_str("</ul>\n\n");
        }
    }
    
    // Parameters (for functions)
    if !element.parameters.is_empty() {
        html.push_str("<h2>Parameters</h2>\n<ul>\n");
        for param in &element.parameters {
            let param_type = param.param_type.as_ref().map(|t| format!(" (<code>{}</code>)", t)).unwrap_or_default();
            html.push_str(&format!("  <li><strong>{}</strong>{}: {}</li>\n", 
                param.name, param_type, param.description));
        }
        html.push_str("</ul>\n\n");
    }
    
    html
}

/// Test cross-reference link generation
#[test]
fn test_cross_reference_links() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 5, column: 10, file: Some("main.csd".to_string()) };
    
    // Create cross-references with different types
    let usage_ref = CrossReference {
        target: "utils::helper_function".to_string(),
        context: "function call in main()".to_string(),
        location: location.clone(),
        reference_type: ReferenceType::Usage,
    };
    
    let type_ref = CrossReference {
        target: "types::User".to_string(),
        context: "parameter type annotation".to_string(),
        location: location.clone(),
        reference_type: ReferenceType::TypeReference,
    };
    
    let import_ref = CrossReference {
        target: "external::library".to_string(),
        context: "import statement".to_string(),
        location: location.clone(),
        reference_type: ReferenceType::Import,
    };
    
    // Generate links for different output formats
    let usage_markdown = generate_markdown_link(&usage_ref);
    let type_markdown = generate_markdown_link(&type_ref);
    let import_markdown = generate_markdown_link(&import_ref);
    
    assert!(usage_markdown.contains("[helper_function]"));
    assert!(usage_markdown.contains("utils-helper_function"));
    assert!(type_markdown.contains("[User]"));
    assert!(type_markdown.contains("types-User"));
    assert!(import_markdown.contains("[library]"));
    assert!(import_markdown.contains("external-library"));
    
    let usage_html = generate_html_link(&usage_ref);
    let type_html = generate_html_link(&type_ref);
    let import_html = generate_html_link(&import_ref);
    
    assert!(usage_html.contains("<a href="));
    assert!(usage_html.contains("helper_function"));
    assert!(usage_html.contains("class=\"usage-link\""));
    assert!(type_html.contains("class=\"type-link\""));
    assert!(import_html.contains("class=\"import-link\""));
}

/// Helper function to generate markdown links
fn generate_markdown_link(cross_ref: &CrossReference) -> String {
    let target_parts: Vec<&str> = cross_ref.target.split("::").collect();
    let link_text = target_parts.last().unwrap_or(&cross_ref.target);
    let anchor = cross_ref.target.replace("::", "-");
    
    format!("[{}](#{}) ({})", link_text, anchor, cross_ref.context)
}

/// Helper function to generate HTML links
fn generate_html_link(cross_ref: &CrossReference) -> String {
    let target_parts: Vec<&str> = cross_ref.target.split("::").collect();
    let link_text = target_parts.last().unwrap_or(&cross_ref.target);
    let anchor = cross_ref.target.replace("::", "-");
    
    let class = match cross_ref.reference_type {
        ReferenceType::Usage => "usage-link",
        ReferenceType::TypeReference => "type-link",
        ReferenceType::Import => "import-link",
        ReferenceType::Inheritance => "inheritance-link",
        ReferenceType::Mention => "mention-link",
    };
    
    format!("<a href=\"#{}\" class=\"{}\" title=\"{}\">{}</a>", 
        anchor, class, cross_ref.context, link_text)
}

/// Test comprehensive documentation export to JSON
#[test]
fn test_comprehensive_json_export() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create a comprehensive module with various elements
    let location = SourceLocation { line: 1, column: 1, file: Some("comprehensive.csd".to_string()) };
    
    // Function
    let func_element = DocElement {
        name: "main_function".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "comprehensive".to_string(),
        summary: "Main entry point".to_string(),
        description: Some("Application main function".to_string()),
        signature: Some("slay main_function()".to_string()),
        parameters: Vec::new(),
        return_type: None,
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: location.clone(),
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    // Struct
    let struct_element = DocElement {
        name: "DataStruct".to_string(),
        element_type: ElementType::Struct,
        visibility: Visibility::Public,
        module: "comprehensive".to_string(),
        summary: "Data container".to_string(),
        description: Some("Holds application data".to_string()),
        signature: Some("squad DataStruct".to_string()),
        parameters: Vec::new(),
        return_type: None,
        type_info: Some(TypeInfo {
            base_type: "struct".to_string(),
            generic_params: Vec::new(),
            constraints: Vec::new(),
            fields: vec![
                FieldDoc {
                    name: "value".to_string(),
                    field_type: "i32".to_string(),
                    description: "Integer value".to_string(),
                    is_public: true,
                    default_value: None,
                }
            ],
            methods: Vec::new(),
        }),
        examples: Vec::new(),
        tags: HashMap::new(),
        location: location.clone(),
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    // Variable
    let var_element = DocElement {
        name: "global_var".to_string(),
        element_type: ElementType::Variable,
        visibility: Visibility::Private,
        module: "comprehensive".to_string(),
        summary: "Global variable".to_string(),
        description: None,
        signature: Some("sus global_var".to_string()),
        parameters: Vec::new(),
        return_type: Some("String".to_string()),
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: location.clone(),
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    let module_doc = ModuleDocumentation {
        name: "comprehensive".to_string(),
        file_path: PathBuf::from("comprehensive.csd"),
        package_info: Some(PackageInfo {
            name: Some("comprehensive_test".to_string()),
            description: Some("Test package".to_string()),
            version: Some("1.0.0".to_string()),
            authors: vec!["Test Author".to_string()],
            dependencies: Vec::new(),
        }),
        imports: Vec::new(),
        items: vec![func_element, struct_element, var_element],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("comprehensive"),
        source_info: SourceInfo {
            file_size: 500,
            line_count: 50,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    // Update extractor
    extractor.update_symbol_table(&module_doc);
    let _ = extractor.build_cross_references();
    
    // Export documentation
    let exported = extractor.export_documentation();
    
    // Test JSON serialization of complete export
    let json_result = serde_json::to_string_pretty(&exported);
    assert!(json_result.is_ok());
    
    let json_string = json_result.unwrap();
    
    // Verify comprehensive structure
    assert!(json_string.contains("\"main_function\""));
    assert!(json_string.contains("\"DataStruct\""));
    assert!(json_string.contains("\"global_var\""));
    assert!(json_string.contains("\"Function\""));
    assert!(json_string.contains("\"Struct\""));
    assert!(json_string.contains("\"Variable\""));
    assert!(json_string.contains("\"comprehensive_test\""));
    assert!(json_string.contains("\"generator_version\""));
    assert!(json_string.contains("\"total_modules\": 1"));
    
    // Verify the JSON is valid and can be deserialized
    let deserialized_result: Result<ExportedDocumentation, _> = serde_json::from_str(&json_string);
    assert!(deserialized_result.is_ok());
    
    let deserialized = deserialized_result.unwrap();
    assert_eq!(deserialized.modules.len(), 1);
    assert_eq!(deserialized.modules[0].items.len(), 3);
    assert_eq!(deserialized.metadata.total_modules, 1);
}
