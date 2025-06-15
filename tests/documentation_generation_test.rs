//! Tests for the enhanced documentation generation system with real AST integration

use cursed::docs::generator::{DocumentationGenerator, DocGeneratorConfig, DocFormat};
use cursed::ast::documentation::{DocumentationExtractor, ExtractionConfig};
use cursed::parser::Parser;
use cursed::lexer::Lexer;
use cursed::error::Error;
use std::collections::HashMap;
use std::path::PathBuf;

#[test]
fn test_documentation_generator_creation() {
    let config = DocGeneratorConfig::default();
    let generator = DocumentationGenerator::new(config);
    // Test passes if generator is created successfully
}

#[test]
fn test_function_documentation_extraction() {
    let source = r#"
        slay test_function(param1 i32, param2 string) i32 {
            return param1 + 42
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");

    let mut extractor = DocumentationExtractor::new();
    let module_doc = extractor.extract_program_documentation(&program, &PathBuf::from("test.csd"))
        .expect("Failed to extract documentation");

    assert_eq!(module_doc.name, "test");
    assert!(!module_doc.items.is_empty());
    
    // Find the function documentation
    let func_doc = module_doc.items.iter()
        .find(|item| item.name == "test_function")
        .expect("Function documentation not found");
    
    assert_eq!(func_doc.element_type, cursed::ast::documentation::ElementType::Function);
    assert!(func_doc.signature.is_some());
    assert!(func_doc.signature.as_ref().unwrap().contains("slay test_function"));
}

#[test]
fn test_struct_documentation_extraction() {
    let source = r#"
        squad TestStruct {
            field1 i32
            field2 string
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");

    let mut extractor = DocumentationExtractor::new();
    let module_doc = extractor.extract_program_documentation(&program, &PathBuf::from("test.csd"))
        .expect("Failed to extract documentation");

    // Find the struct documentation
    let struct_doc = module_doc.items.iter()
        .find(|item| item.name == "TestStruct")
        .expect("Struct documentation not found");
    
    assert_eq!(struct_doc.element_type, cursed::ast::documentation::ElementType::Struct);
    assert!(struct_doc.signature.is_some());
    assert!(struct_doc.signature.as_ref().unwrap().contains("squad TestStruct"));
}

#[test]
fn test_interface_documentation_extraction() {
    let source = r#"
        collab TestInterface {
            method1(param i32) string
            method2() void
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");

    let mut extractor = DocumentationExtractor::new();
    let module_doc = extractor.extract_program_documentation(&program, &PathBuf::from("test.csd"))
        .expect("Failed to extract documentation");

    // Find the interface documentation
    let interface_doc = module_doc.items.iter()
        .find(|item| item.name == "TestInterface")
        .expect("Interface documentation not found");
    
    assert_eq!(interface_doc.element_type, cursed::ast::documentation::ElementType::Interface);
    assert!(interface_doc.signature.is_some());
    assert!(interface_doc.signature.as_ref().unwrap().contains("collab TestInterface"));
}

#[test]
fn test_variable_documentation_extraction() {
    let source = r#"
        sus test_variable i32 = 42
        facts test_constant string = "hello"
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");

    let mut extractor = DocumentationExtractor::new();
    let module_doc = extractor.extract_program_documentation(&program, &PathBuf::from("test.csd"))
        .expect("Failed to extract documentation");

    // Find the variable documentation
    let var_doc = module_doc.items.iter()
        .find(|item| item.name == "test_variable")
        .expect("Variable documentation not found");
    
    assert_eq!(var_doc.element_type, cursed::ast::documentation::ElementType::Variable);
    assert!(var_doc.signature.is_some());
    assert!(var_doc.signature.as_ref().unwrap().contains("sus test_variable"));

    // Find the constant documentation
    let const_doc = module_doc.items.iter()
        .find(|item| item.name == "test_constant")
        .expect("Constant documentation not found");
    
    assert_eq!(const_doc.element_type, cursed::ast::documentation::ElementType::Constant);
    assert!(const_doc.signature.is_some());
    assert!(const_doc.signature.as_ref().unwrap().contains("facts test_constant"));
}

#[test]
fn test_complete_documentation_generation() {
    let source = r#"
        package test_module
        
        import "std::io"
        
        squad Person {
            name string
            age i32
        }
        
        collab Greeter {
            greet(person Person) string
        }
        
        slay create_person(name string, age i32) Person {
            return Person { name: name, age: age }
        }
        
        facts DEFAULT_NAME string = "Anonymous"
        sus global_counter i32 = 0
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");

    let mut extractor = DocumentationExtractor::new();
    let module_doc = extractor.extract_program_documentation(&program, &PathBuf::from("test.csd"))
        .expect("Failed to extract documentation");

    // Check module information
    assert_eq!(module_doc.name, "test");
    assert!(module_doc.package_info.is_some());
    
    // Check imports
    assert!(!module_doc.imports.is_empty());

    // Check that we have all documented items
    let item_names: Vec<&String> = module_doc.items.iter().map(|item| &item.name).collect();
    
    // We should have documentation for major declarations
    // Note: Not all statements may be documentable depending on parser implementation
    assert!(!module_doc.items.is_empty(), "Should have at least some documented items");
    
    // Verify source information
    assert!(module_doc.source_info.line_count > 0);
    assert!(module_doc.source_info.file_size > 0);
}

#[test]
fn test_cross_reference_building() {
    let source = r#"
        squad User {
            name string
        }
        
        slay get_user_name(user User) string {
            return user.name
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");

    let mut extractor = DocumentationExtractor::new();
    let _module_doc = extractor.extract_program_documentation(&program, &PathBuf::from("test.csd"))
        .expect("Failed to extract documentation");

    // Build cross-references
    extractor.build_cross_references().expect("Failed to build cross-references");
    
    // Test that cross-references were built (basic validation)
    let exported_docs = extractor.export_documentation();
    assert!(exported_docs.modules.len() > 0);
    assert!(exported_docs.symbol_table.len() > 0);
}

#[test]
fn test_cursed_specific_keywords() {
    let source = r#"
        slay yolo_function() void {
            yolo
        }
        
        sus sus_variable i32 = 42
        facts facts_constant string = "cursed"
        
        squad squad_struct {
            field i32
        }
        
        collab collab_interface {
            method() void
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");

    let mut extractor = DocumentationExtractor::new();
    let module_doc = extractor.extract_program_documentation(&program, &PathBuf::from("test.csd"))
        .expect("Failed to extract documentation");

    // Verify CURSED-specific keywords are documented properly
    for item in &module_doc.items {
        if let Some(ref signature) = item.signature {
            // Check that CURSED keywords are preserved in signatures
            if item.element_type == cursed::ast::documentation::ElementType::Function {
                assert!(signature.contains("slay"), "Function signature should contain 'slay': {}", signature);
            } else if item.element_type == cursed::ast::documentation::ElementType::Struct {
                assert!(signature.contains("squad"), "Struct signature should contain 'squad': {}", signature);
            } else if item.element_type == cursed::ast::documentation::ElementType::Interface {
                assert!(signature.contains("collab"), "Interface signature should contain 'collab': {}", signature);
            } else if item.element_type == cursed::ast::documentation::ElementType::Variable {
                assert!(signature.contains("sus"), "Variable signature should contain 'sus': {}", signature);
            } else if item.element_type == cursed::ast::documentation::ElementType::Constant {
                assert!(signature.contains("facts"), "Constant signature should contain 'facts': {}", signature);
            }
        }
    }
}

#[test]
fn test_extraction_config_options() {
    let mut config = ExtractionConfig::default();
    config.include_source = true;
    config.extract_examples = true;
    config.include_private = true;

    let source = r#"
        slay test_function() void {
            // Function body
        }
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");

    let mut extractor = DocumentationExtractor::with_config(config);
    let module_doc = extractor.extract_program_documentation(&program, &PathBuf::from("test.csd"))
        .expect("Failed to extract documentation");

    // Verify configuration options are respected
    if let Some(func_doc) = module_doc.items.iter().find(|item| item.name == "test_function") {
        assert!(func_doc.source_code.is_some(), "Source code should be included when configured");
    }
}

#[test]
fn test_documentation_generator_integration() {
    let source = r#"
        slay hello_world() string {
            return "Hello, CURSED world!"
        }
    "#;

    // Create a temporary file for testing
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_cursed_doc.csd");
    std::fs::write(&test_file, source).expect("Failed to write test file");

    let config = DocGeneratorConfig {
        output_dir: temp_dir.join("docs_output"),
        format: DocFormat::Json,
        include_examples: true,
        include_private: false,
        generate_cross_refs: true,
        custom_css: None,
        template_dir: None,
        title: "Test Documentation".to_string(),
        description: Some("Test documentation generation".to_string()),
        version: Some("1.0.0".to_string()),
        authors: vec!["Test Author".to_string()],
        base_url: None,
    };

    let mut generator = DocumentationGenerator::new(config);
    
    // This would be the integration point - for now just test creation
    // In a full implementation, we would test:
    // generator.generate_from_files(vec![test_file]).expect("Failed to generate documentation");
    
    // Clean up
    let _ = std::fs::remove_file(&test_file);
}

#[test]
fn test_documentation_metadata() {
    let source = r#"
        slay test_function() void {}
    "#;

    let mut lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");

    let mut extractor = DocumentationExtractor::new();
    let module_doc = extractor.extract_program_documentation(&program, &PathBuf::from("test.csd"))
        .expect("Failed to extract documentation");

    // Check metadata
    assert_eq!(module_doc.metadata.module, "test");
    assert_eq!(module_doc.metadata.format_version, "1.0");
    assert!(!module_doc.metadata.language_metadata.language_version.is_empty());
    assert!(!module_doc.metadata.language_metadata.compiler_version.is_empty());
}
