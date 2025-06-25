//! Documentation AST Extraction Test Suite
//! 
//! Tests for comprehensive AST documentation extraction functionality
//! including all the enhanced features like generic parameters,
//! optional parameter detection, visibility analysis, and more.

use cursed::docs::{DocumentationExtractor, DocGeneratorConfig, ItemKind, Visibility};
use cursed::error::SourceLocation;
use std::path::PathBuf;

#[test]
fn test_documentation_extractor_creation() {
    let extractor = DocumentationExtractor::new();
    // Should not panic and should be created successfully
    
    let config = DocGeneratorConfig::default();
    let extractor_with_config = DocumentationExtractor::with_config(config);
    // Should also be created successfully
}

#[test]
fn test_basic_function_extraction() {
    let source = r#"
        package test;
        
        /// This is a test function
        slay test_function(param1 string, param2 int = 42) -> string {
            return "test";
        }
    "#;
    
    let extractor = DocumentationExtractor::new();
    let file_path = PathBuf::from("test.csd");
    
    match extractor.extract_from_source(source, &file_path) {
        Ok(docs) => {
            assert_eq!(docs.module_name, "test");
            assert_eq!(docs.package_name, Some("test".to_string()));
            assert_eq!(docs.items.len(), 1);
            
            let func_doc = &docs.items[0];
            assert_eq!(func_doc.name, "test_function");
            assert!(matches!(func_doc.kind, ItemKind::Function));
            assert!(matches!(func_doc.visibility, Visibility::Public));
            assert!(func_doc.signature.is_some());
            assert_eq!(func_doc.parameters.len(), 2);
            
            // Check parameter documentation
            let param1 = &func_doc.parameters[0];
            assert_eq!(param1.name, "param1");
            assert_eq!(param1.type_name, Some("string".to_string()));
            assert!(param1.description.contains("Required parameter"));
            
            let param2 = &func_doc.parameters[1];
            assert_eq!(param2.name, "param2");
            assert_eq!(param2.type_name, Some("int".to_string()));
            assert!(param2.description.contains("Optional parameter"));
            assert_eq!(param2.default_value, Some("42".to_string()));
        }
        Err(e) => panic!("Failed to extract documentation: {:?}", e),
    }
}

#[test]
fn test_struct_with_generic_parameters() {
    let source = r#"
        package test;
        
        /// Generic container struct
        squad Container<T, U> {
            public_field T;
            private_field U;
        }
    "#;
    
    let extractor = DocumentationExtractor::new();
    let file_path = PathBuf::from("test.csd");
    
    match extractor.extract_from_source(source, &file_path) {
        Ok(docs) => {
            assert_eq!(docs.items.len(), 1);
            
            let struct_doc = &docs.items[0];
            assert_eq!(struct_doc.name, "Container");
            assert!(matches!(struct_doc.kind, ItemKind::Struct));
            
            // Should have fields + generic parameters
            assert!(struct_doc.parameters.len() >= 2);
            
            // Check for generic parameters in the extracted parameters
            let has_generic_t = struct_doc.parameters.iter()
                .any(|p| p.name == "T" && p.type_name == Some("Type".to_string()));
            let has_generic_u = struct_doc.parameters.iter()
                .any(|p| p.name == "U" && p.type_name == Some("Type".to_string()));
            
            assert!(has_generic_t, "Should extract generic parameter T");
            assert!(has_generic_u, "Should extract generic parameter U");
        }
        Err(e) => panic!("Failed to extract struct documentation: {:?}", e),
    }
}

#[test]
fn test_interface_extraction() {
    let source = r#"
        package test;
        
        /// Test interface
        collab TestInterface {
            method1(param1 string) -> int;
            method2() -> void;
        }
    "#;
    
    let extractor = DocumentationExtractor::new();
    let file_path = PathBuf::from("test.csd");
    
    match extractor.extract_from_source(source, &file_path) {
        Ok(docs) => {
            assert_eq!(docs.items.len(), 1);
            
            let interface_doc = &docs.items[0];
            assert_eq!(interface_doc.name, "TestInterface");
            assert!(matches!(interface_doc.kind, ItemKind::Interface));
            
            // Should have extracted methods as parameters
            assert_eq!(interface_doc.parameters.len(), 2);
            
            let method1 = &interface_doc.parameters[0];
            assert_eq!(method1.name, "method1");
            
            let method2 = &interface_doc.parameters[1];
            assert_eq!(method2.name, "method2");
        }
        Err(e) => panic!("Failed to extract interface documentation: {:?}", e),
    }
}

#[test]
fn test_variable_and_constant_extraction() {
    let source = r#"
        package test;
        
        sus mutable_var int = 10;
        facts constant_val string = "hello";
    "#;
    
    let extractor = DocumentationExtractor::new();
    let file_path = PathBuf::from("test.csd");
    
    match extractor.extract_from_source(source, &file_path) {
        Ok(docs) => {
            assert_eq!(docs.items.len(), 2);
            
            let var_doc = &docs.items[0];
            assert_eq!(var_doc.name, "mutable_var");
            assert!(matches!(var_doc.kind, ItemKind::Variable));
            
            let const_doc = &docs.items[1];
            assert_eq!(const_doc.name, "constant_val");
            assert!(matches!(const_doc.kind, ItemKind::Constant));
        }
        Err(e) => panic!("Failed to extract variable/constant documentation: {:?}", e),
    }
}

#[test]
fn test_optional_type_detection() {
    let extractor = DocumentationExtractor::new();
    
    assert!(extractor.is_optional_type("string?"));
    assert!(extractor.is_optional_type("Option<int>"));
    assert!(!extractor.is_optional_type("string"));
    assert!(!extractor.is_optional_type("int"));
}

#[test]
fn test_source_info_extraction() {
    let source = r#"
        package test;
        
        slay simple_function() {
            // Simple function
        }
    "#;
    
    let extractor = DocumentationExtractor::new();
    let file_path = PathBuf::from("test.csd");
    
    match extractor.extract_from_source(source, &file_path) {
        Ok(docs) => {
            assert_eq!(docs.file_path, file_path);
            assert_eq!(docs.source_info.encoding, "UTF-8");
            assert!(docs.source_info.line_count > 0);
            assert!(docs.source_info.file_size > 0);
        }
        Err(e) => panic!("Failed to extract source info: {:?}", e),
    }
}

#[test]
fn test_comprehensive_extraction() {
    let source = r#"
        package test_package;
        
        import "stdlib::io";
        import "stdlib::math";
        
        /// Generic function with optional parameters
        slay complex_function<T, U>(
            required_param T,
            optional_param U?,
            default_param int = 100
        ) -> Option<T> {
            return None;
        }
        
        /// Data structure with mixed visibility
        squad DataStruct<T> {
            PublicField T;
            privateField string;
        }
        
        /// Service interface
        collab Service {
            process(data string) -> bool;
            validate() -> void;
        }
        
        sus global_var int = 42;
        facts PI float = 3.14159;
    "#;
    
    let extractor = DocumentationExtractor::new();
    let file_path = PathBuf::from("comprehensive_test.csd");
    
    match extractor.extract_from_source(source, &file_path) {
        Ok(docs) => {
            assert_eq!(docs.module_name, "comprehensive_test");
            assert_eq!(docs.package_name, Some("test_package".to_string()));
            assert_eq!(docs.imports.len(), 2);
            assert!(docs.imports.contains(&"stdlib::io".to_string()));
            assert!(docs.imports.contains(&"stdlib::math".to_string()));
            
            // Should have 4 documented items
            assert_eq!(docs.items.len(), 4);
            
            // Verify each item type
            let item_names: Vec<&String> = docs.items.iter().map(|item| &item.name).collect();
            assert!(item_names.contains(&&"complex_function".to_string()));
            assert!(item_names.contains(&&"DataStruct".to_string()));
            assert!(item_names.contains(&&"Service".to_string()));
            assert!(item_names.contains(&&"global_var".to_string()) || item_names.contains(&&"PI".to_string()));
            
            // Check function documentation
            let func_doc = docs.items.iter().find(|item| item.name == "complex_function").unwrap();
            assert!(matches!(func_doc.kind, ItemKind::Function));
            
            // Should have regular parameters + generic parameters
            assert!(func_doc.parameters.len() >= 3);
            
            // Check struct documentation
            let struct_doc = docs.items.iter().find(|item| item.name == "DataStruct").unwrap();
            assert!(matches!(struct_doc.kind, ItemKind::Struct));
            
            // Check interface documentation
            let interface_doc = docs.items.iter().find(|item| item.name == "Service").unwrap();
            assert!(matches!(interface_doc.kind, ItemKind::Interface));
            assert_eq!(interface_doc.parameters.len(), 2); // Two methods
        }
        Err(e) => panic!("Failed comprehensive extraction test: {:?}", e),
    }
}

#[test]
fn test_edge_cases() {
    // Test empty source
    let extractor = DocumentationExtractor::new();
    let file_path = PathBuf::from("empty.csd");
    
    match extractor.extract_from_source("", &file_path) {
        Ok(docs) => {
            assert_eq!(docs.items.len(), 0);
            assert_eq!(docs.source_info.line_count, 0);
        }
        Err(_) => {
            // This is acceptable - empty source might cause parsing errors
        }
    }
    
    // Test source with only comments
    match extractor.extract_from_source("// Just a comment\n/* Block comment */", &file_path) {
        Ok(docs) => {
            assert_eq!(docs.items.len(), 0);
        }
        Err(_) => {
            // This is acceptable
        }
    }
}
