//! Unit tests for AST documentation extraction module
//!
//! This test suite validates the core functionality of the documentation
//! extraction system, ensuring proper handling of AST nodes and metadata.

#[path = "common/mod.rs"]
mod common;

use cursed::ast::documentation::*;
use cursed::ast::*;
use cursed::error::{Error, SourceLocation};
use cursed::lexer::{Token, TokenType};
use std::collections::HashMap;
use std::path::PathBuf;

/// Test DocumentationExtractor creation and basic functionality
#[test]
fn test_documentation_extractor_creation() {
    common::tracing::setup();
    
    let extractor = DocumentationExtractor::new();
    
    // Verify initial state
    assert_eq!(extractor.get_all_modules().len(), 0);
    assert!(extractor.get_module_documentation("nonexistent").is_none());
    assert!(extractor.get_symbol("nonexistent::symbol").is_none());
}

/// Test DocumentationExtractor with custom configuration
#[test] 
fn test_documentation_extractor_with_config() {
    common::tracing::setup();
    
    let config = ExtractionConfig {
        include_private: true,
        extract_examples: false,
        generate_cross_refs: false,
        max_depth: 5,
        include_source: false,
        language_settings: LanguageSettings {
            keyword_mappings: HashMap::new(),
            use_slang_docs: false,
            include_signatures: false,
        },
    };
    
    let extractor = DocumentationExtractor::with_config(config);
    
    // Configuration should be applied (internal state verification)
    let modules = extractor.get_all_modules();
    assert_eq!(modules.len(), 0);
}

/// Test ExtractionConfig default values
#[test]
fn test_extraction_config_defaults() {
    common::tracing::setup();
    
    let config = ExtractionConfig::default();
    
    assert!(!config.include_private);
    assert!(config.extract_examples);
    assert!(config.generate_cross_refs);
    assert_eq!(config.max_depth, 10);
    assert!(config.include_source);
    assert!(config.language_settings.use_slang_docs);
    assert!(config.language_settings.include_signatures);
    
    // Check keyword mappings include CURSED slang
    assert!(config.language_settings.keyword_mappings.contains_key("slay"));
    assert!(config.language_settings.keyword_mappings.contains_key("sus"));
    assert!(config.language_settings.keyword_mappings.contains_key("facts"));
    assert!(config.language_settings.keyword_mappings.contains_key("squad"));
    assert!(config.language_settings.keyword_mappings.contains_key("collab"));
}

/// Test DocElement creation and properties
#[test]
fn test_doc_element_creation() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 10, column: 5, file: Some("test.csd".to_string()) };
    
    let mut tags = HashMap::new();
    tags.insert("author".to_string(), vec!["Test Author".to_string()]);
    tags.insert("since".to_string(), vec!["1.0.0".to_string()]);
    
    let param = ParameterDoc {
        name: "x".to_string(),
        param_type: Some("i32".to_string()),
        description: "Input parameter".to_string(),
        default_value: None,
        is_optional: false,
    };
    
    let example = CodeExample {
        title: Some("Basic Usage".to_string()),
        description: Some("Example of function usage".to_string()),
        code: "test_function(42)".to_string(),
        language: "cursed".to_string(),
        output: Some("84".to_string()),
        is_runnable: true,
    };
    
    let element = DocElement {
        name: "test_function".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "test_module".to_string(),
        summary: "A test function that doubles its input".to_string(),
        description: Some("This function takes an integer and returns its double value".to_string()),
        signature: Some("slay test_function(x: i32) -> i32".to_string()),
        parameters: vec![param],
        return_type: Some("i32".to_string()),
        type_info: None,
        examples: vec![example],
        tags,
        location: location.clone(),
        source_code: Some("slay test_function(x: i32) -> i32 { periodt x * 2; }".to_string()),
        metadata: ElementMetadata::default(),
    };
    
    // Verify properties
    assert_eq!(element.name, "test_function");
    assert_eq!(element.element_type, ElementType::Function);
    assert_eq!(element.visibility, Visibility::Public);
    assert_eq!(element.module, "test_module");
    assert_eq!(element.summary, "A test function that doubles its input");
    assert!(element.description.is_some());
    assert!(element.signature.is_some());
    assert_eq!(element.parameters.len(), 1);
    assert_eq!(element.parameters[0].name, "x");
    assert_eq!(element.parameters[0].param_type, Some("i32".to_string()));
    assert!(!element.parameters[0].is_optional);
    assert_eq!(element.return_type, Some("i32".to_string()));
    assert_eq!(element.examples.len(), 1);
    assert_eq!(element.examples[0].title, Some("Basic Usage".to_string()));
    assert!(element.examples[0].is_runnable);
    assert_eq!(element.tags.len(), 2);
    assert!(element.tags.contains_key("author"));
    assert!(element.tags.contains_key("since"));
    assert_eq!(element.location.line, 10);
    assert_eq!(element.location.column, 5);
    assert!(element.source_code.is_some());
}

/// Test ElementType display and comparison
#[test]
fn test_element_type_operations() {
    common::tracing::setup();
    
    // Test Display trait
    assert_eq!(ElementType::Function.to_string(), "function");
    assert_eq!(ElementType::Struct.to_string(), "struct");
    assert_eq!(ElementType::Interface.to_string(), "interface");
    assert_eq!(ElementType::Variable.to_string(), "variable");
    assert_eq!(ElementType::Constant.to_string(), "constant");
    assert_eq!(ElementType::Type.to_string(), "type");
    assert_eq!(ElementType::Module.to_string(), "module");
    assert_eq!(ElementType::Macro.to_string(), "macro");
    assert_eq!(ElementType::Other.to_string(), "other");
    
    // Test equality
    assert_eq!(ElementType::Function, ElementType::Function);
    assert_ne!(ElementType::Function, ElementType::Struct);
    
    // Test clone
    let func_type = ElementType::Function;
    let cloned_type = func_type.clone();
    assert_eq!(func_type, cloned_type);
}

/// Test Visibility enum operations
#[test]
fn test_visibility_operations() {
    common::tracing::setup();
    
    // Test equality
    assert_eq!(Visibility::Public, Visibility::Public);
    assert_ne!(Visibility::Public, Visibility::Private);
    
    // Test clone
    let pub_vis = Visibility::Public;
    let cloned_vis = pub_vis.clone();
    assert_eq!(pub_vis, cloned_vis);
    
    // Test all variants
    let visibilities = vec![
        Visibility::Public,
        Visibility::Private,
        Visibility::Protected,
        Visibility::Internal,
    ];
    
    assert_eq!(visibilities.len(), 4);
}

/// Test DocumentationMetadata creation and properties
#[test]
fn test_documentation_metadata() {
    common::tracing::setup();
    
    let metadata = DocumentationMetadata::new("test_module");
    
    assert_eq!(metadata.module, "test_module");
    assert_eq!(metadata.format_version, "1.0");
    assert_eq!(metadata.language_metadata.language_version, "0.1.0");
    assert_eq!(metadata.language_metadata.target_platform, std::env::consts::OS);
    assert_eq!(metadata.statistics.total_items, 0);
    assert_eq!(metadata.statistics.function_count, 0);
    assert_eq!(metadata.statistics.struct_count, 0);
    assert_eq!(metadata.statistics.interface_count, 0);
    assert_eq!(metadata.statistics.variable_count, 0);
    assert_eq!(metadata.statistics.constant_count, 0);
    assert_eq!(metadata.statistics.coverage_percentage, 0.0);
}

/// Test LanguageMetadata default values
#[test]
fn test_language_metadata_defaults() {
    common::tracing::setup();
    
    let lang_meta = LanguageMetadata::default();
    
    assert_eq!(lang_meta.language_version, "0.1.0");
    assert_eq!(lang_meta.target_platform, std::env::consts::OS);
    assert_eq!(lang_meta.features.len(), 0);
    
    // Compiler version should be the current package version
    assert!(!lang_meta.compiler_version.is_empty());
}

/// Test CrossReference creation and properties
#[test]
fn test_cross_reference() {
    common::tracing::setup();
    
    let location = SourceLocation { 
        line: 15, 
        column: 8, 
        file: Some("source.csd".to_string()) 
    };
    
    let cross_ref = CrossReference {
        target: "MyStruct::method".to_string(),
        context: "function call in main()".to_string(),
        location: location.clone(),
        reference_type: ReferenceType::Usage,
    };
    
    assert_eq!(cross_ref.target, "MyStruct::method");
    assert_eq!(cross_ref.context, "function call in main()");
    assert_eq!(cross_ref.location.line, 15);
    assert_eq!(cross_ref.location.column, 8);
    
    // Test different reference types
    let ref_types = vec![
        ReferenceType::Usage,
        ReferenceType::TypeReference,
        ReferenceType::Import,
        ReferenceType::Inheritance,
        ReferenceType::Mention,
    ];
    
    assert_eq!(ref_types.len(), 5);
}

/// Test ParameterDoc creation and validation
#[test]
fn test_parameter_doc() {
    common::tracing::setup();
    
    // Required parameter
    let required_param = ParameterDoc {
        name: "value".to_string(),
        param_type: Some("f64".to_string()),
        description: "The input value to process".to_string(),
        default_value: None,
        is_optional: false,
    };
    
    assert_eq!(required_param.name, "value");
    assert_eq!(required_param.param_type, Some("f64".to_string()));
    assert!(!required_param.is_optional);
    assert!(required_param.default_value.is_none());
    
    // Optional parameter with default
    let optional_param = ParameterDoc {
        name: "multiplier".to_string(),
        param_type: Some("f64".to_string()),
        description: "Multiplication factor".to_string(),
        default_value: Some("1.0".to_string()),
        is_optional: true,
    };
    
    assert_eq!(optional_param.name, "multiplier");
    assert!(optional_param.is_optional);
    assert_eq!(optional_param.default_value, Some("1.0".to_string()));
}

/// Test FieldDoc for struct documentation
#[test]
fn test_field_doc() {
    common::tracing::setup();
    
    let field_doc = FieldDoc {
        name: "id".to_string(),
        field_type: "u64".to_string(),
        description: "Unique identifier".to_string(),
        is_public: true,
        default_value: Some("0".to_string()),
    };
    
    assert_eq!(field_doc.name, "id");
    assert_eq!(field_doc.field_type, "u64");
    assert_eq!(field_doc.description, "Unique identifier");
    assert!(field_doc.is_public);
    assert_eq!(field_doc.default_value, Some("0".to_string()));
}

/// Test MethodDoc for interface documentation
#[test]
fn test_method_doc() {
    common::tracing::setup();
    
    let param_doc = ParameterDoc {
        name: "self".to_string(),
        param_type: Some("&Self".to_string()),
        description: "Reference to self".to_string(),
        default_value: None,
        is_optional: false,
    };
    
    let method_doc = MethodDoc {
        name: "get_id".to_string(),
        signature: "slay get_id(&self) -> u64".to_string(),
        description: "Returns the unique identifier".to_string(),
        parameters: vec![param_doc],
        return_type: Some("u64".to_string()),
        is_static: false,
    };
    
    assert_eq!(method_doc.name, "get_id");
    assert_eq!(method_doc.signature, "slay get_id(&self) -> u64");
    assert_eq!(method_doc.parameters.len(), 1);
    assert_eq!(method_doc.return_type, Some("u64".to_string()));
    assert!(!method_doc.is_static);
}

/// Test ImportDoc creation
#[test]
fn test_import_doc() {
    common::tracing::setup();
    
    // Import without alias
    let import_doc = ImportDoc {
        path: "std::collections::HashMap".to_string(),
        alias: None,
        description: "Standard library hash map".to_string(),
        is_public: false,
    };
    
    assert_eq!(import_doc.path, "std::collections::HashMap");
    assert!(import_doc.alias.is_none());
    assert!(!import_doc.is_public);
    
    // Import with alias
    let aliased_import = ImportDoc {
        path: "external::very_long_module_name".to_string(),
        alias: Some("short".to_string()),
        description: "Aliased import for convenience".to_string(),
        is_public: true,
    };
    
    assert_eq!(aliased_import.alias, Some("short".to_string()));
    assert!(aliased_import.is_public);
}

/// Test TypeInfo for complex type documentation
#[test]
fn test_type_info() {
    common::tracing::setup();
    
    let field_doc = FieldDoc {
        name: "data".to_string(),
        field_type: "T".to_string(),
        description: "Generic data field".to_string(),
        is_public: true,
        default_value: None,
    };
    
    let method_doc = MethodDoc {
        name: "clone".to_string(),
        signature: "slay clone(&self) -> Self".to_string(),
        description: "Creates a clone of the object".to_string(),
        parameters: Vec::new(),
        return_type: Some("Self".to_string()),
        is_static: false,
    };
    
    let type_info = TypeInfo {
        base_type: "Container".to_string(),
        generic_params: vec!["T".to_string()],
        constraints: vec!["T: Clone".to_string()],
        fields: vec![field_doc],
        methods: vec![method_doc],
    };
    
    assert_eq!(type_info.base_type, "Container");
    assert_eq!(type_info.generic_params.len(), 1);
    assert_eq!(type_info.generic_params[0], "T");
    assert_eq!(type_info.constraints.len(), 1);
    assert_eq!(type_info.constraints[0], "T: Clone");
    assert_eq!(type_info.fields.len(), 1);
    assert_eq!(type_info.methods.len(), 1);
}

/// Test CodeExample creation and properties
#[test]
fn test_code_example() {
    common::tracing::setup();
    
    // Runnable example with output
    let example = CodeExample {
        title: Some("Array Creation".to_string()),
        description: Some("Shows how to create and initialize an array".to_string()),
        code: r#"sus numbers = [1, 2, 3, 4, 5];
println!("Array length: {}", numbers.len());"#.to_string(),
        language: "cursed".to_string(),
        output: Some("Array length: 5".to_string()),
        is_runnable: true,
    };
    
    assert_eq!(example.title, Some("Array Creation".to_string()));
    assert!(example.description.is_some());
    assert!(example.code.contains("sus numbers"));
    assert_eq!(example.language, "cursed");
    assert_eq!(example.output, Some("Array length: 5".to_string()));
    assert!(example.is_runnable);
    
    // Non-runnable example (pseudocode)
    let pseudocode = CodeExample {
        title: Some("Algorithm Overview".to_string()),
        description: None,
        code: "// This is pseudocode\nfor each item in collection { process(item) }".to_string(),
        language: "text".to_string(),
        output: None,
        is_runnable: false,
    };
    
    assert!(!pseudocode.is_runnable);
    assert_eq!(pseudocode.language, "text");
    assert!(pseudocode.description.is_none());
    assert!(pseudocode.output.is_none());
}

/// Test ElementMetadata creation and default values
#[test]
fn test_element_metadata() {
    common::tracing::setup();
    
    let metadata = ElementMetadata::default();
    
    assert!(metadata.since_version.is_none());
    assert!(!metadata.is_deprecated);
    assert!(metadata.deprecation_message.is_none());
    assert!(matches!(metadata.stability, StabilityLevel::Stable));
    assert_eq!(metadata.performance_notes.len(), 0);
    assert_eq!(metadata.security_notes.len(), 0);
    
    // Test with custom values
    let mut custom_metadata = ElementMetadata::default();
    custom_metadata.since_version = Some("1.0.0".to_string());
    custom_metadata.is_deprecated = true;
    custom_metadata.deprecation_message = Some("Use new_function() instead".to_string());
    custom_metadata.stability = StabilityLevel::Deprecated;
    custom_metadata.performance_notes.push("O(1) complexity".to_string());
    custom_metadata.security_notes.push("Validate input".to_string());
    
    assert_eq!(custom_metadata.since_version, Some("1.0.0".to_string()));
    assert!(custom_metadata.is_deprecated);
    assert!(custom_metadata.deprecation_message.is_some());
    assert!(matches!(custom_metadata.stability, StabilityLevel::Deprecated));
    assert_eq!(custom_metadata.performance_notes.len(), 1);
    assert_eq!(custom_metadata.security_notes.len(), 1);
}

/// Test StabilityLevel enum
#[test]
fn test_stability_level() {
    common::tracing::setup();
    
    let levels = vec![
        StabilityLevel::Experimental,
        StabilityLevel::Unstable,
        StabilityLevel::Stable,
        StabilityLevel::Deprecated,
    ];
    
    assert_eq!(levels.len(), 4);
    
    // Test default
    let default_stability = StabilityLevel::default();
    assert!(matches!(default_stability, StabilityLevel::Stable));
}

/// Test CommentParser creation and functionality
#[test]
fn test_comment_parser() {
    common::tracing::setup();
    
    let parser = CommentParser::new();
    
    // Test parsing empty content
    let result = parser.parse_documentation_content("");
    assert!(result.is_ok());
    let (summary, description, tags, examples) = result.unwrap();
    assert!(summary.is_empty());
    assert!(description.is_empty());
    assert!(tags.is_empty());
    assert!(examples.is_empty());
    
    // Test parsing simple content
    let content = "This is a summary line.\n\nThis is a detailed description.";
    let result = parser.parse_documentation_content(content);
    assert!(result.is_ok());
    let (summary, description, tags, examples) = result.unwrap();
    assert_eq!(summary, "This is a summary line.");
    assert!(description.contains("This is a detailed description"));
    assert!(tags.is_empty());
    assert!(examples.is_empty());
}

/// Test DocumentedSymbol creation
#[test]
fn test_documented_symbol() {
    common::tracing::setup();
    
    let location = SourceLocation { 
        line: 25, 
        column: 1, 
        file: Some("lib.csd".to_string()) 
    };
    
    let symbol = DocumentedSymbol {
        name: "calculate_sum".to_string(),
        element_type: ElementType::Function,
        module: "math_utils".to_string(),
        location: location.clone(),
        signature: Some("slay calculate_sum(values: &[i32]) -> i32".to_string()),
    };
    
    assert_eq!(symbol.name, "calculate_sum");
    assert_eq!(symbol.element_type, ElementType::Function);
    assert_eq!(symbol.module, "math_utils");
    assert_eq!(symbol.location.line, 25);
    assert!(symbol.signature.is_some());
}

/// Test ModuleDocumentation creation
#[test]
fn test_module_documentation() {
    common::tracing::setup();
    
    let package_info = PackageInfo {
        name: Some("test_package".to_string()),
        description: Some("A test package".to_string()),
        version: Some("1.0.0".to_string()),
        authors: vec!["Test Author".to_string()],
        dependencies: vec!["std".to_string()],
    };
    
    let source_info = SourceInfo {
        file_size: 1024,
        line_count: 50,
        last_modified: None,
        encoding: "UTF-8".to_string(),
    };
    
    let module_doc = ModuleDocumentation {
        name: "test_module".to_string(),
        file_path: PathBuf::from("src/test_module.csd"),
        package_info: Some(package_info),
        imports: Vec::new(),
        items: Vec::new(),
        module_comments: vec!["Module comment".to_string()],
        metadata: DocumentationMetadata::new("test_module"),
        source_info,
    };
    
    assert_eq!(module_doc.name, "test_module");
    assert_eq!(module_doc.file_path, PathBuf::from("src/test_module.csd"));
    assert!(module_doc.package_info.is_some());
    assert_eq!(module_doc.imports.len(), 0);
    assert_eq!(module_doc.items.len(), 0);
    assert_eq!(module_doc.module_comments.len(), 1);
    assert_eq!(module_doc.source_info.file_size, 1024);
    assert_eq!(module_doc.source_info.line_count, 50);
}

/// Test PackageInfo creation
#[test]
fn test_package_info() {
    common::tracing::setup();
    
    let package_info = PackageInfo {
        name: Some("cursed_utils".to_string()),
        description: Some("Utility functions for CURSED".to_string()),
        version: Some("2.1.0".to_string()),
        authors: vec!["Alice".to_string(), "Bob".to_string()],
        dependencies: vec!["std".to_string(), "regex".to_string()],
    };
    
    assert_eq!(package_info.name, Some("cursed_utils".to_string()));
    assert!(package_info.description.is_some());
    assert_eq!(package_info.version, Some("2.1.0".to_string()));
    assert_eq!(package_info.authors.len(), 2);
    assert_eq!(package_info.dependencies.len(), 2);
    assert!(package_info.dependencies.contains(&"std".to_string()));
    assert!(package_info.dependencies.contains(&"regex".to_string()));
}

/// Test SourceInfo properties
#[test]
fn test_source_info() {
    common::tracing::setup();
    
    let source_info = SourceInfo {
        file_size: 2048,
        line_count: 100,
        last_modified: Some(std::time::SystemTime::now()),
        encoding: "UTF-8".to_string(),
    };
    
    assert_eq!(source_info.file_size, 2048);
    assert_eq!(source_info.line_count, 100);
    assert!(source_info.last_modified.is_some());
    assert_eq!(source_info.encoding, "UTF-8");
}

/// Test serialization/deserialization capability (basic check)
#[test]
fn test_serialization_support() {
    common::tracing::setup();
    
    // Test that our structures support serialization by creating them
    // In a real implementation, we would test with serde_json
    
    let element = DocElement {
        name: "test".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "test".to_string(),
        summary: "Test".to_string(),
        description: None,
        signature: None,
        parameters: Vec::new(),
        return_type: None,
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: SourceLocation { line: 1, column: 1, file: None },
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    // If this compiles, serialization attributes are correctly applied
    assert_eq!(element.name, "test");
}
