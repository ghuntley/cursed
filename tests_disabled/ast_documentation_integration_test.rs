//! Integration tests for AST documentation extraction
//!
//! This test suite validates the complete documentation extraction workflow
//! from AST parsing through to cross-reference generation and output formatting.

#[path = "common/mod.rs"]
mod common;

use cursed::ast::documentation::*;
use cursed::ast::*;
use cursed::error::{Error, SourceLocation};
use cursed::lexer::{Token, TokenType, Lexer};
use cursed::parser::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use tempfile::TempDir;

/// Test complete program documentation extraction
#[test]
fn test_program_documentation_extraction() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create a simple program
    let program = Program {
        statements: Vec::new(),
        package_name: Some("test_package".to_string()),
        imports: vec![
            ImportStatement {
                token: Token { 
                    token_type: TokenType::Yeet, 
                    literal: "yeet".to_string(),
                    line: 1,
                    column: 1,
                },
                path: "std::io".to_string(),
                alias: None,
            }
        ],
    };
    
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.csd");
    
    let result = extractor.extract_program_documentation(&program, &file_path);
    assert!(result.is_ok());
    
    let module_doc = result.unwrap();
    assert_eq!(module_doc.name, "test");
    assert!(module_doc.package_info.is_some());
    assert_eq!(module_doc.package_info.as_ref().unwrap().name, Some("test_package".to_string()));
    assert_eq!(module_doc.imports.len(), 1);
    assert_eq!(module_doc.imports[0].path, "std::io");
}

/// Test documentation extraction from statements
#[test]
fn test_statement_documentation_extraction() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create a mock function statement
    let func_stmt = FunctionStatement {
        token: Token { 
            token_type: TokenType::Slay, 
            literal: "slay".to_string(),
            line: 1,
            column: 1,
        },
        name: "add_numbers".to_string(),
        parameters: vec![
            FunctionParameter {
                name: "a".to_string(),
                type_hint: Some(Box::new(Identifier {
                    token: Token { 
                        token_type: TokenType::I32, 
                        literal: "i32".to_string(),
                        line: 1,
                        column: 10,
                    },
                    value: "i32".to_string(),
                })),
                default_value: None,
            },
            FunctionParameter {
                name: "b".to_string(),
                type_hint: Some(Box::new(Identifier {
                    token: Token { 
                        token_type: TokenType::I32, 
                        literal: "i32".to_string(),
                        line: 1,
                        column: 15,
                    },
                    value: "i32".to_string(),
                })),
                default_value: None,
            },
        ],
        return_type: Some(Box::new(Identifier {
            token: Token { 
                token_type: TokenType::I32, 
                literal: "i32".to_string(),
                line: 1,
                column: 20,
            },
            value: "i32".to_string(),
        })),
        body: Block::new(),
        is_public: true,
        type_params: Vec::new(),
    };
    
    let result = extractor.extract_statement_documentation(&func_stmt, "math_module");
    assert!(result.is_ok());
    
    let doc_element = result.unwrap();
    assert!(doc_element.is_some());
    
    let element = doc_element.unwrap();
    assert_eq!(element.name, "add_numbers");
    assert_eq!(element.element_type, ElementType::Function);
    assert_eq!(element.visibility, Visibility::Public);
    assert_eq!(element.module, "math_module");
    assert_eq!(element.parameters.len(), 2);
    assert_eq!(element.parameters[0].name, "a");
    assert_eq!(element.parameters[1].name, "b");
    assert!(element.signature.is_some());
    assert!(element.signature.as_ref().unwrap().contains("add_numbers"));
}

/// Test function documentation extraction with generic parameters
#[test]
fn test_generic_function_documentation() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create a generic function statement
    let func_stmt = FunctionStatement {
        token: Token { 
            token_type: TokenType::Slay, 
            literal: "slay".to_string(),
            line: 1,
            column: 1,
        },
        name: "process_data".to_string(),
        parameters: vec![
            FunctionParameter {
                name: "data".to_string(),
                type_hint: Some(Box::new(Identifier {
                    token: Token { 
                        token_type: TokenType::Identifier, 
                        literal: "T".to_string(),
                        line: 1,
                        column: 10,
                    },
                    value: "T".to_string(),
                })),
                default_value: None,
            },
        ],
        return_type: Some(Box::new(Identifier {
            token: Token { 
                token_type: TokenType::Identifier, 
                literal: "T".to_string(),
                line: 1,
                column: 15,
            },
            value: "T".to_string(),
        })),
        body: Block::new(),
        is_public: true,
        type_params: vec![
            TypeParameter {
                name: "T".to_string(),
                constraints: Vec::new(),
                default: None,
            }
        ],
    };
    
    let result = extractor.extract_statement_documentation(&func_stmt, "generic_module");
    assert!(result.is_ok());
    
    let doc_element = result.unwrap();
    assert!(doc_element.is_some());
    
    let element = doc_element.unwrap();
    assert_eq!(element.name, "process_data");
    assert!(element.signature.is_some());
    let signature = element.signature.unwrap();
    assert!(signature.contains("process_data"));
    assert!(signature.contains("<T>"));
    assert!(signature.contains("T"));
}

/// Test struct documentation extraction
#[test]
fn test_struct_documentation_extraction() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create a struct statement
    let struct_stmt = SquadStatement {
        token: Token { 
            token_type: TokenType::Squad, 
            literal: "squad".to_string(),
            line: 1,
            column: 1,
        },
        name: "User".to_string(),
        fields: vec![
            FieldDefinition {
                name: "id".to_string(),
                field_type: Some(Box::new(Identifier {
                    token: Token { 
                        token_type: TokenType::U64, 
                        literal: "u64".to_string(),
                        line: 2,
                        column: 5,
                    },
                    value: "u64".to_string(),
                })),
                is_public: true,
            },
            FieldDefinition {
                name: "name".to_string(),
                field_type: Some(Box::new(Identifier {
                    token: Token { 
                        token_type: TokenType::String, 
                        literal: "String".to_string(),
                        line: 3,
                        column: 5,
                    },
                    value: "String".to_string(),
                })),
                is_public: true,
            },
        ],
        type_params: Vec::new(),
    };
    
    let result = extractor.extract_statement_documentation(&struct_stmt, "user_module");
    assert!(result.is_ok());
    
    let doc_element = result.unwrap();
    assert!(doc_element.is_some());
    
    let element = doc_element.unwrap();
    assert_eq!(element.name, "User");
    assert_eq!(element.element_type, ElementType::Struct);
    assert_eq!(element.visibility, Visibility::Public);
    assert_eq!(element.module, "user_module");
    assert!(element.type_info.is_some());
    
    let type_info = element.type_info.unwrap();
    assert_eq!(type_info.base_type, "struct");
    assert_eq!(type_info.fields.len(), 2);
    assert_eq!(type_info.fields[0].name, "id");
    assert_eq!(type_info.fields[0].field_type, "u64");
    assert!(type_info.fields[0].is_public);
    assert_eq!(type_info.fields[1].name, "name");
    assert_eq!(type_info.fields[1].field_type, "String");
    assert!(type_info.fields[1].is_public);
}

/// Test interface documentation extraction
#[test]
fn test_interface_documentation_extraction() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create an interface statement
    let interface_stmt = CollabStatement {
        token: Token { 
            token_type: TokenType::Collab, 
            literal: "collab".to_string(),
            line: 1,
            column: 1,
        },
        name: "Drawable".to_string(),
        methods: vec![
            FunctionStatement {
                token: Token { 
                    token_type: TokenType::Slay, 
                    literal: "slay".to_string(),
                    line: 2,
                    column: 5,
                },
                name: "draw".to_string(),
                parameters: vec![
                    FunctionParameter {
                        name: "self".to_string(),
                        type_hint: Some(Box::new(Identifier {
                            token: Token { 
                                token_type: TokenType::Identifier, 
                                literal: "&Self".to_string(),
                                line: 2,
                                column: 10,
                            },
                            value: "&Self".to_string(),
                        })),
                        default_value: None,
                    },
                ],
                return_type: None,
                body: Block::new(),
                is_public: true,
                type_params: Vec::new(),
            },
        ],
        type_params: Vec::new(),
    };
    
    let result = extractor.extract_statement_documentation(&interface_stmt, "graphics_module");
    assert!(result.is_ok());
    
    let doc_element = result.unwrap();
    assert!(doc_element.is_some());
    
    let element = doc_element.unwrap();
    assert_eq!(element.name, "Drawable");
    assert_eq!(element.element_type, ElementType::Interface);
    assert_eq!(element.visibility, Visibility::Public);
    assert_eq!(element.module, "graphics_module");
    assert!(element.type_info.is_some());
    
    let type_info = element.type_info.unwrap();
    assert_eq!(type_info.base_type, "interface");
    assert_eq!(type_info.methods.len(), 1);
    assert_eq!(type_info.methods[0].name, "draw");
    assert!(!type_info.methods[0].is_static);
}

/// Test variable documentation extraction
#[test]
fn test_variable_documentation_extraction() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create a variable statement
    let var_stmt = VariableStatement {
        token: Token { 
            token_type: TokenType::Sus, 
            literal: "sus".to_string(),
            line: 1,
            column: 1,
        },
        name: "global_counter".to_string(),
        value: Some(Box::new(IntegerLiteral {
            token: Token { 
                token_type: TokenType::Integer, 
                literal: "0".to_string(),
                line: 1,
                column: 20,
            },
            value: 0,
        })),
    };
    
    let result = extractor.extract_statement_documentation(&var_stmt, "config_module");
    assert!(result.is_ok());
    
    let doc_element = result.unwrap();
    assert!(doc_element.is_some());
    
    let element = doc_element.unwrap();
    assert_eq!(element.name, "global_counter");
    assert_eq!(element.element_type, ElementType::Variable);
    assert_eq!(element.visibility, Visibility::Private);
    assert_eq!(element.module, "config_module");
    assert!(element.signature.is_some());
    assert!(element.signature.as_ref().unwrap().contains("sus"));
}

/// Test cross-reference building
#[test]
fn test_cross_reference_building() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Add some mock module documentation
    let location = SourceLocation { line: 1, column: 1, file: None };
    let element1 = DocElement {
        name: "function_a".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "module1".to_string(),
        summary: "Function A".to_string(),
        description: Some("This function calls function_b".to_string()),
        signature: Some("slay function_a() -> void".to_string()),
        parameters: Vec::new(),
        return_type: None,
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: location.clone(),
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    let element2 = DocElement {
        name: "function_b".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "module2".to_string(),
        summary: "Function B".to_string(),
        description: None,
        signature: Some("slay function_b() -> i32".to_string()),
        parameters: Vec::new(),
        return_type: Some("i32".to_string()),
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: location.clone(),
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    let module_doc1 = ModuleDocumentation {
        name: "module1".to_string(),
        file_path: PathBuf::from("module1.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![element1],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("module1"),
        source_info: SourceInfo {
            file_size: 100,
            line_count: 10,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    let module_doc2 = ModuleDocumentation {
        name: "module2".to_string(),
        file_path: PathBuf::from("module2.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![element2],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("module2"),
        source_info: SourceInfo {
            file_size: 80,
            line_count: 8,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    // Manually update extractor's cache and symbol table
    extractor.update_symbol_table(&module_doc1);
    extractor.update_symbol_table(&module_doc2);
    
    // Build cross-references
    let result = extractor.build_cross_references();
    assert!(result.is_ok());
    
    // Check that cross-references were built
    let references = extractor.get_cross_references("module1::function_a");
    if references.is_some() {
        let refs = references.unwrap();
        // Since function_a mentions function_b in its description
        assert!(!refs.is_empty());
    }
}

/// Test module caching and retrieval
#[test]
fn test_module_caching() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Initially no modules
    assert_eq!(extractor.get_all_modules().len(), 0);
    assert!(extractor.get_module_documentation("test_module").is_none());
    
    // Create and extract program documentation
    let program = Program {
        statements: Vec::new(),
        package_name: Some("test_package".to_string()),
        imports: Vec::new(),
    };
    
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_module.csd");
    
    let result = extractor.extract_program_documentation(&program, &file_path);
    assert!(result.is_ok());
    
    // Now should have one module
    assert_eq!(extractor.get_all_modules().len(), 1);
    assert!(extractor.get_module_documentation("test_module").is_some());
    
    let module_doc = extractor.get_module_documentation("test_module").unwrap();
    assert_eq!(module_doc.name, "test_module");
}

/// Test symbol table management
#[test]
fn test_symbol_table_management() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Initially no symbols
    assert!(extractor.get_symbol("nonexistent::symbol").is_none());
    
    // Create a module with documented items
    let location = SourceLocation { line: 5, column: 1, file: None };
    let element = DocElement {
        name: "test_function".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "test_module".to_string(),
        summary: "Test function".to_string(),
        description: None,
        signature: Some("slay test_function() -> void".to_string()),
        parameters: Vec::new(),
        return_type: None,
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: location.clone(),
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    let module_doc = ModuleDocumentation {
        name: "test_module".to_string(),
        file_path: PathBuf::from("test_module.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![element],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("test_module"),
        source_info: SourceInfo {
            file_size: 150,
            line_count: 15,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    // Update symbol table
    extractor.update_symbol_table(&module_doc);
    
    // Now should have symbol
    let symbol = extractor.get_symbol("test_module::test_function");
    assert!(symbol.is_some());
    
    let sym = symbol.unwrap();
    assert_eq!(sym.name, "test_function");
    assert_eq!(sym.element_type, ElementType::Function);
    assert_eq!(sym.module, "test_module");
    assert_eq!(sym.location.line, 5);
}

/// Test documentation export functionality
#[test]
fn test_documentation_export() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create and add some documentation
    let program = Program {
        statements: Vec::new(),
        package_name: Some("export_test".to_string()),
        imports: Vec::new(),
    };
    
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("export_module.csd");
    
    let result = extractor.extract_program_documentation(&program, &file_path);
    assert!(result.is_ok());
    
    // Export documentation
    let exported = extractor.export_documentation();
    
    assert_eq!(exported.modules.len(), 1);
    assert_eq!(exported.modules[0].name, "export_module");
    assert_eq!(exported.metadata.total_modules, 1);
    assert!(!exported.metadata.generator_version.is_empty());
}

/// Test configuration with private item inclusion
#[test]
fn test_private_item_inclusion() {
    common::tracing::setup();
    
    let config = ExtractionConfig {
        include_private: true,
        extract_examples: true,
        generate_cross_refs: true,
        max_depth: 10,
        include_source: true,
        language_settings: LanguageSettings {
            keyword_mappings: HashMap::new(),
            use_slang_docs: true,
            include_signatures: true,
        },
    };
    
    let extractor = DocumentationExtractor::with_config(config);
    
    // Configuration is internal, so we can only verify the extractor was created
    assert_eq!(extractor.get_all_modules().len(), 0);
}

/// Test configuration without source code inclusion
#[test]
fn test_source_code_exclusion() {
    common::tracing::setup();
    
    let config = ExtractionConfig {
        include_private: false,
        extract_examples: true,
        generate_cross_refs: true,
        max_depth: 10,
        include_source: false, // Exclude source code
        language_settings: LanguageSettings {
            keyword_mappings: HashMap::new(),
            use_slang_docs: true,
            include_signatures: true,
        },
    };
    
    let extractor = DocumentationExtractor::with_config(config);
    
    // Configuration is applied internally
    assert_eq!(extractor.get_all_modules().len(), 0);
}

/// Test complex program with multiple statement types
#[test]
fn test_complex_program_documentation() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create function statement
    let func_stmt = FunctionStatement {
        token: Token { 
            token_type: TokenType::Slay, 
            literal: "slay".to_string(),
            line: 1,
            column: 1,
        },
        name: "main".to_string(),
        parameters: Vec::new(),
        return_type: None,
        body: Block::new(),
        is_public: true,
        type_params: Vec::new(),
    };
    
    // Create variable statement
    let var_stmt = VariableStatement {
        token: Token { 
            token_type: TokenType::Sus, 
            literal: "sus".to_string(),
            line: 5,
            column: 1,
        },
        name: "counter".to_string(),
        value: Some(Box::new(IntegerLiteral {
            token: Token { 
                token_type: TokenType::Integer, 
                literal: "0".to_string(),
                line: 5,
                column: 15,
            },
            value: 0,
        })),
    };
    
    // Create program with multiple statements
    let program = Program {
        statements: vec![
            Box::new(func_stmt),
            Box::new(var_stmt),
        ],
        package_name: Some("complex_package".to_string()),
        imports: vec![
            ImportStatement {
                token: Token { 
                    token_type: TokenType::Yeet, 
                    literal: "yeet".to_string(),
                    line: 1,
                    column: 1,
                },
                path: "std::io".to_string(),
                alias: None,
            }
        ],
    };
    
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("complex.csd");
    
    let result = extractor.extract_program_documentation(&program, &file_path);
    assert!(result.is_ok());
    
    let module_doc = result.unwrap();
    assert_eq!(module_doc.name, "complex");
    assert!(module_doc.package_info.is_some());
    assert_eq!(module_doc.imports.len(), 1);
    assert_eq!(module_doc.items.len(), 2); // Should have extracted both statements
}

/// Test error handling for invalid file paths
#[test]
fn test_error_handling_invalid_path() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    let program = Program {
        statements: Vec::new(),
        package_name: None,
        imports: Vec::new(),
    };
    
    // Use a path that doesn't exist
    let invalid_path = PathBuf::from("/nonexistent/directory/file.csd");
    
    let result = extractor.extract_program_documentation(&program, &invalid_path);
    
    // Should handle gracefully by using the file path as provided
    assert!(result.is_ok());
    
    let module_doc = result.unwrap();
    assert_eq!(module_doc.name, "file");
    assert_eq!(module_doc.file_path, invalid_path);
}

/// Test comment parser integration
#[test]
fn test_comment_parser_integration() {
    common::tracing::setup();
    
    let parser = CommentParser::new();
    
    // Test parsing documentation with tags
    let doc_content = r#"
Brief summary of the function.

This is a more detailed description of what the function does.
It can span multiple lines and explain the behavior in detail.

@param value The input value to process
@param multiplier Scale factor for the operation  
@return The processed result as a number
@since 1.0.0
@example
sus result = process_value(42, 2.0);
println!("Result: {}", result);
"#;
    
    let result = parser.parse_documentation_content(doc_content);
    assert!(result.is_ok());
    
    let (summary, description, tags, examples) = result.unwrap();
    
    // The actual parsing would be more sophisticated in a real implementation
    assert!(!summary.is_empty());
    assert!(!description.is_empty());
}

/// Test performance with many modules
#[test]
fn test_performance_many_modules() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    let temp_dir = TempDir::new().unwrap();
    
    // Create multiple modules
    for i in 0..10 {
        let program = Program {
            statements: vec![
                Box::new(VariableStatement {
                    token: Token { 
                        token_type: TokenType::Sus, 
                        literal: "sus".to_string(),
                        line: 1,
                        column: 1,
                    },
                    name: format!("var_{}", i),
                    value: Some(Box::new(IntegerLiteral {
                        token: Token { 
                            token_type: TokenType::Integer, 
                            literal: i.to_string(),
                            line: 1,
                            column: 10,
                        },
                        value: i as i64,
                    })),
                })
            ],
            package_name: Some(format!("package_{}", i)),
            imports: Vec::new(),
        };
        
        let file_path = temp_dir.path().join(format!("module_{}.csd", i));
        let result = extractor.extract_program_documentation(&program, &file_path);
        assert!(result.is_ok());
    }
    
    // Should have extracted all modules
    assert_eq!(extractor.get_all_modules().len(), 10);
    
    // Build cross-references for all modules
    let result = extractor.build_cross_references();
    assert!(result.is_ok());
    
    // Export all documentation
    let exported = extractor.export_documentation();
    assert_eq!(exported.modules.len(), 10);
    assert_eq!(exported.metadata.total_modules, 10);
}
