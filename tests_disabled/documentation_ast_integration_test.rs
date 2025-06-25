//! Comprehensive AST Integration Tests for CURSED Documentation System
//! 
//! This test suite validates the enhanced AST integration for documentation
//! generation, ensuring complete extraction of all language constructs.

use cursed::documentation::extractors::{AstExtractor, ExtractionConfig, EnhancedDocumentationItem};
use cursed::documentation::extractors::ast_node_support::*;
use cursed::documentation::{DocumentationSystem, DocumentationConfig, ProjectMetadata, DocOptions, StylingConfig};
use cursed::error::SourceLocation;
use cursed::ast::{AstNode, AstNodeType, Program};
use std::collections::HashMap;
use std::path::PathBuf;

#[tokio::test]
async fn test_enhanced_function_extraction() {
    let config = ExtractionConfig::default();
    let extractor = AstExtractor::new(config).expect("Failed to create AST extractor");

    // Create a sample function declaration
    let func_decl = FunctionDeclaration {
        name: "test_function".to_string(),
        parameters: vec![
            Parameter {
                name: "x".to_string(),
                param_type: Some(Expression {
                    expr_type: ExpressionType::Identifier(IdentifierExpression {
                        name: "i32".to_string(),
                    }),
                }),
                default_value: None,
                is_variadic: false,
            },
            Parameter {
                name: "y".to_string(),
                param_type: Some(Expression {
                    expr_type: ExpressionType::Identifier(IdentifierExpression {
                        name: "string".to_string(),
                    }),
                }),
                default_value: None,
                is_variadic: false,
            },
        ],
        return_type: Some(Expression {
            expr_type: ExpressionType::Identifier(IdentifierExpression {
                name: "bool".to_string(),
            }),
        }),
        body: AstNode {
            node_type: AstNodeType::BlockStatement(cursed::ast::BlockStatement {
                statements: Vec::new(),
            }),
        },
        generic_params: Some(vec!["T".to_string(), "U".to_string()]),
        constraints: Some(vec![
            GenericConstraint {
                constraint_type: "Clone".to_string(),
                target_type: "T".to_string(),
                expression: "T: Clone".to_string(),
            },
        ]),
        is_async: true,
        is_public: true,
        location: SourceLocation {
            file: "test.csd".to_string(),
            line: 10,
            column: 1,
        },
    };

    // Create AST node for the function
    let ast_node = AstNode {
        node_type: AstNodeType::FunctionDeclaration(func_decl),
    };

    let source_code = r#"
    /// This is a test function that demonstrates enhanced extraction
    /// @param x The first parameter of type i32
    /// @param y The second parameter of type string
    /// @return Returns a boolean value
    /// @example
    /// ```cursed
    /// let result = test_function(42, "hello");
    /// spill(result);
    /// ```
    async slay test_function<T: Clone, U>(x: i32, y: string) -> bool {
        // Implementation here
        return lowkey true;
    }
    "#;

    let file_path = PathBuf::from("test.csd");
    let enhanced_items = extractor.extract_complete_documentation(&ast_node, &file_path, source_code)
        .await
        .expect("Failed to extract documentation");

    assert!(!enhanced_items.is_empty(), "Should extract at least one documentation item");
    
    let function_item = &enhanced_items[0];
    assert_eq!(function_item.base.name, "test_function");
    assert!(function_item.base.description.is_some(), "Should have extracted description from comments");
    assert!(function_item.type_info.is_some(), "Should have extracted type information");
    assert!(function_item.generic_info.is_some(), "Should have extracted generic information");
    
    if let Some(ref generic_info) = function_item.generic_info {
        assert_eq!(generic_info.parameters.len(), 2, "Should have extracted 2 generic parameters");
        assert!(generic_info.constraints.len() > 0, "Should have extracted generic constraints");
    }
}

#[tokio::test]
async fn test_enhanced_struct_extraction() {
    let config = ExtractionConfig::default();
    let extractor = AstExtractor::new(config).expect("Failed to create AST extractor");

    // Create a sample struct declaration
    let struct_decl = StructDeclaration {
        name: "TestStruct".to_string(),
        fields: vec![
            Field {
                name: "id".to_string(),
                field_type: Some(Expression {
                    expr_type: ExpressionType::Identifier(IdentifierExpression {
                        name: "u64".to_string(),
                    }),
                }),
                is_public: true,
                is_optional: false,
            },
            Field {
                name: "name".to_string(),
                field_type: Some(Expression {
                    expr_type: ExpressionType::Identifier(IdentifierExpression {
                        name: "string".to_string(),
                    }),
                }),
                is_public: true,
                is_optional: false,
            },
            Field {
                name: "metadata".to_string(),
                field_type: Some(Expression {
                    expr_type: ExpressionType::FunctionCall(FunctionCallExpression {
                        function: Box::new(Expression {
                            expr_type: ExpressionType::Identifier(IdentifierExpression {
                                name: "HashMap".to_string(),
                            }),
                        }),
                        arguments: vec![
                            Expression {
                                expr_type: ExpressionType::Identifier(IdentifierExpression {
                                    name: "string".to_string(),
                                }),
                            },
                            Expression {
                                expr_type: ExpressionType::Identifier(IdentifierExpression {
                                    name: "string".to_string(),
                                }),
                            },
                        ],
                    }),
                }),
                is_public: false,
                is_optional: true,
            },
        ],
        generic_params: Some(vec!["T".to_string()]),
        constraints: None,
        is_public: true,
        location: SourceLocation {
            file: "test.csd".to_string(),
            line: 5,
            column: 1,
        },
    };

    let ast_node = AstNode {
        node_type: AstNodeType::StructDeclaration(struct_decl),
    };

    let source_code = r#"
    /// A test struct for demonstrating enhanced extraction
    /// This struct contains various field types including generics
    squad TestStruct<T> {
        /// Unique identifier for the struct
        pub id: u64,
        /// Human-readable name
        pub name: string,
        /// Optional metadata storage
        metadata: HashMap<string, string>?,
    }
    "#;

    let file_path = PathBuf::from("test.csd");
    let enhanced_items = extractor.extract_complete_documentation(&ast_node, &file_path, source_code)
        .await
        .expect("Failed to extract documentation");

    assert!(!enhanced_items.is_empty(), "Should extract struct documentation");
    
    let struct_item = &enhanced_items[0];
    assert_eq!(struct_item.base.name, "TestStruct");
    assert!(struct_item.type_info.is_some(), "Should have extracted type information");
    
    if let Some(ref type_info) = struct_item.type_info {
        assert_eq!(type_info.type_name, "TestStruct");
        assert_eq!(type_info.type_parameters.len(), 1, "Should have 1 generic parameter");
        assert!(!type_info.nested_types.is_empty(), "Should have nested field types");
    }
}

#[tokio::test]
async fn test_enhanced_interface_extraction() {
    let config = ExtractionConfig::default();
    let extractor = AstExtractor::new(config).expect("Failed to create AST extractor");

    // Create a sample interface declaration with methods
    let interface_decl = InterfaceDeclaration {
        name: "TestInterface".to_string(),
        methods: vec![
            FunctionDeclaration {
                name: "get_name".to_string(),
                parameters: vec![],
                return_type: Some(Expression {
                    expr_type: ExpressionType::Identifier(IdentifierExpression {
                        name: "string".to_string(),
                    }),
                }),
                body: AstNode {
                    node_type: AstNodeType::BlockStatement(cursed::ast::BlockStatement {
                        statements: Vec::new(),
                    }),
                },
                generic_params: None,
                constraints: None,
                is_async: false,
                is_public: true,
                location: SourceLocation {
                    file: "test.csd".to_string(),
                    line: 12,
                    column: 5,
                },
            },
            FunctionDeclaration {
                name: "set_value".to_string(),
                parameters: vec![
                    Parameter {
                        name: "value".to_string(),
                        param_type: Some(Expression {
                            expr_type: ExpressionType::Identifier(IdentifierExpression {
                                name: "T".to_string(),
                            }),
                        }),
                        default_value: None,
                        is_variadic: false,
                    },
                ],
                return_type: None,
                body: AstNode {
                    node_type: AstNodeType::BlockStatement(cursed::ast::BlockStatement {
                        statements: Vec::new(),
                    }),
                },
                generic_params: None,
                constraints: None,
                is_async: false,
                is_public: true,
                location: SourceLocation {
                    file: "test.csd".to_string(),
                    line: 15,
                    column: 5,
                },
            },
        ],
        generic_params: Some(vec!["T".to_string()]),
        constraints: Some(vec![
            GenericConstraint {
                constraint_type: "Clone".to_string(),
                target_type: "T".to_string(),
                expression: "T: Clone".to_string(),
            },
        ]),
        is_public: true,
        location: SourceLocation {
            file: "test.csd".to_string(),
            line: 8,
            column: 1,
        },
    };

    let ast_node = AstNode {
        node_type: AstNodeType::InterfaceDeclaration(interface_decl),
    };

    let source_code = r#"
    /// A test interface demonstrating method signatures
    /// @example
    /// ```cursed
    /// impl TestInterface<string> for MyStruct {
    ///     slay get_name() -> string {
    ///         return "example";
    ///     }
    ///     slay set_value(value: string) {
    ///         // implementation
    ///     }
    /// }
    /// ```
    collab TestInterface<T: Clone> {
        /// Get the name of the object
        slay get_name() -> string;
        
        /// Set a value of type T
        slay set_value(value: T);
    }
    "#;

    let file_path = PathBuf::from("test.csd");
    let enhanced_items = extractor.extract_complete_documentation(&ast_node, &file_path, source_code)
        .await
        .expect("Failed to extract documentation");

    assert!(!enhanced_items.is_empty(), "Should extract interface documentation");
    
    let interface_item = &enhanced_items[0];
    assert_eq!(interface_item.base.name, "TestInterface");
    assert!(interface_item.type_info.is_some(), "Should have extracted type information");
    assert!(interface_item.generic_info.is_some(), "Should have extracted generic information");
    
    if let Some(ref type_info) = interface_item.type_info {
        assert_eq!(type_info.type_name, "TestInterface");
        assert_eq!(type_info.type_parameters.len(), 1, "Should have 1 generic parameter");
    }
}

#[tokio::test]
async fn test_enhanced_module_extraction() {
    let config = ExtractionConfig::default();
    let extractor = AstExtractor::new(config).expect("Failed to create AST extractor");

    // Create a sample module declaration
    let module_decl = ModuleDeclaration {
        name: "test_module".to_string(),
        body: Some(AstNode {
            node_type: AstNodeType::Program(Program {
                statements: vec![
                    AstNode {
                        node_type: AstNodeType::FunctionDeclaration(FunctionDeclaration {
                            name: "helper_function".to_string(),
                            parameters: vec![],
                            return_type: None,
                            body: AstNode {
                                node_type: AstNodeType::BlockStatement(cursed::ast::BlockStatement {
                                    statements: Vec::new(),
                                }),
                            },
                            generic_params: None,
                            constraints: None,
                            is_async: false,
                            is_public: false,
                            location: SourceLocation {
                                file: "test.csd".to_string(),
                                line: 20,
                                column: 5,
                            },
                        }),
                    },
                ],
            }),
        }),
        is_public: true,
        location: SourceLocation {
            file: "test.csd".to_string(),
            line: 18,
            column: 1,
        },
    };

    let ast_node = AstNode {
        node_type: AstNodeType::ModuleDeclaration(module_decl),
    };

    let source_code = r#"
    //! Test module for demonstrating enhanced module extraction
    //! 
    //! This module contains helper functions and utilities.
    //! @author Test Developer
    //! @version 1.0.0
    module test_module {
        slay helper_function() {
            // Helper implementation
        }
    }
    "#;

    let file_path = PathBuf::from("test.csd");
    let enhanced_items = extractor.extract_complete_documentation(&ast_node, &file_path, source_code)
        .await
        .expect("Failed to extract documentation");

    assert!(!enhanced_items.is_empty(), "Should extract module documentation");
    
    let module_item = &enhanced_items[0];
    assert_eq!(module_item.base.name, "test_module");
    assert!(module_item.base.description.is_some(), "Should have extracted module description");
    
    // Should also extract nested items
    let nested_items: Vec<_> = enhanced_items.iter()
        .filter(|item| item.base.name == "helper_function")
        .collect();
    assert!(!nested_items.is_empty(), "Should extract nested function from module");
}

#[tokio::test]
async fn test_enhanced_cross_references() {
    let config = ExtractionConfig {
        include_relationships: true,
        ..ExtractionConfig::default()
    };
    let extractor = AstExtractor::new(config).expect("Failed to create AST extractor");

    // Create a program with multiple related items
    let program = Program {
        statements: vec![
            AstNode {
                node_type: AstNodeType::StructDeclaration(StructDeclaration {
                    name: "User".to_string(),
                    fields: vec![
                        Field {
                            name: "id".to_string(),
                            field_type: Some(Expression {
                                expr_type: ExpressionType::Identifier(IdentifierExpression {
                                    name: "u64".to_string(),
                                }),
                            }),
                            is_public: true,
                            is_optional: false,
                        },
                    ],
                    generic_params: None,
                    constraints: None,
                    is_public: true,
                    location: SourceLocation {
                        file: "test.csd".to_string(),
                        line: 5,
                        column: 1,
                    },
                }),
            },
            AstNode {
                node_type: AstNodeType::FunctionDeclaration(FunctionDeclaration {
                    name: "create_user".to_string(),
                    parameters: vec![
                        Parameter {
                            name: "id".to_string(),
                            param_type: Some(Expression {
                                expr_type: ExpressionType::Identifier(IdentifierExpression {
                                    name: "u64".to_string(),
                                }),
                            }),
                            default_value: None,
                            is_variadic: false,
                        },
                    ],
                    return_type: Some(Expression {
                        expr_type: ExpressionType::Identifier(IdentifierExpression {
                            name: "User".to_string(),
                        }),
                    }),
                    body: AstNode {
                        node_type: AstNodeType::BlockStatement(cursed::ast::BlockStatement {
                            statements: Vec::new(),
                        }),
                    },
                    generic_params: None,
                    constraints: None,
                    is_async: false,
                    is_public: true,
                    location: SourceLocation {
                        file: "test.csd".to_string(),
                        line: 10,
                        column: 1,
                    },
                }),
            },
        ],
    };

    let ast_node = AstNode {
        node_type: AstNodeType::Program(program),
    };

    let source_code = r#"
    /// User struct for representing users
    squad User {
        pub id: u64,
    }

    /// Creates a new user with the given ID
    /// @param id The user ID
    /// @return A new User instance
    /// @see User
    slay create_user(id: u64) -> User {
        return User { id: id };
    }
    "#;

    let file_path = PathBuf::from("test.csd");
    let enhanced_items = extractor.extract_complete_documentation(&ast_node, &file_path, source_code)
        .await
        .expect("Failed to extract documentation");

    assert!(enhanced_items.len() >= 2, "Should extract at least 2 items (User struct and create_user function)");
    
    // Check for relationships
    let function_item = enhanced_items.iter()
        .find(|item| item.base.name == "create_user")
        .expect("Should find create_user function");
    
    assert!(!function_item.relationships.is_empty(), "Should have extracted relationships");
    
    // Should have a relationship to the User struct
    let user_relationship = function_item.relationships.iter()
        .find(|rel| rel.target == "User");
    assert!(user_relationship.is_some(), "Should have relationship to User struct");
}

#[tokio::test]
async fn test_documentation_system_integration() {
    let config = DocumentationConfig {
        source_dirs: vec![PathBuf::from("tests/fixtures")],
        output_dir: PathBuf::from("target/test_docs"),
        output_formats: vec![cursed::documentation::OutputFormat::Html],
        project: ProjectMetadata {
            name: "Enhanced AST Integration Test".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Testing enhanced AST integration".to_string()),
            authors: vec!["Test Developer".to_string()],
            homepage: None,
            repository: None,
            license: None,
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

    let mut doc_system = DocumentationSystem::new(config)
        .expect("Failed to create documentation system");

    // This test would require actual source files in tests/fixtures
    // For now, we'll just verify the system initializes correctly
    assert!(doc_system.extracted_docs().is_empty(), "Should start with no extracted docs");
    assert!(doc_system.cross_references().is_empty(), "Should start with no cross references");
    assert!(doc_system.search_index().is_empty(), "Should start with empty search index");
}

/// Test helper to create a basic AST node for testing
fn create_test_ast_node() -> AstNode {
    AstNode {
        node_type: AstNodeType::Program(Program {
            statements: vec![
                AstNode {
                    node_type: AstNodeType::FunctionDeclaration(FunctionDeclaration {
                        name: "test_function".to_string(),
                        parameters: vec![],
                        return_type: None,
                        body: AstNode {
                            node_type: AstNodeType::BlockStatement(cursed::ast::BlockStatement {
                                statements: Vec::new(),
                            }),
                        },
                        generic_params: None,
                        constraints: None,
                        is_async: false,
                        is_public: true,
                        location: SourceLocation {
                            file: "test.csd".to_string(),
                            line: 1,
                            column: 1,
                        },
                    }),
                },
            ],
        }),
    }
}

#[test]
fn test_extraction_config_defaults() {
    let config = ExtractionConfig::default();
    
    assert!(!config.include_private, "Should not include private items by default");
    assert!(config.include_source, "Should include source by default");
    assert!(config.include_generics, "Should include generics by default");
    assert!(config.include_relationships, "Should include relationships by default");
    assert_eq!(config.max_type_depth, 10, "Should have reasonable max depth");
    assert!(config.include_implementations, "Should include implementations by default");
    assert!(config.include_error_types, "Should include error types by default");
}

#[test]
fn test_enhanced_documentation_item_creation() {
    use cursed::documentation::extractors::ast_extractor::{EnhancedDocumentationItem, CompleteTypeInfo, TypeKind};
    use cursed::documentation::DocumentationItem;
    
    let base_item = DocumentationItem {
        name: "test_item".to_string(),
        kind: cursed::documentation::ItemKind::Function,
        description: Some("Test description".to_string()),
        location: SourceLocation {
            file: "test.csd".to_string(),
            line: 1,
            column: 1,
        },
        source_code: Some("slay test_item() {}".to_string()),
        visibility: "public".to_string(),
        metadata: HashMap::new(),
    };

    let enhanced_item = EnhancedDocumentationItem {
        base: base_item,
        type_info: Some(CompleteTypeInfo {
            type_name: "test_function".to_string(),
            type_signature: "slay test_function()".to_string(),
            type_kind: TypeKind::Function,
            type_parameters: Vec::new(),
            constraints: Vec::new(),
            nested_types: Vec::new(),
            size_info: None,
        }),
        generic_info: None,
        relationships: Vec::new(),
        implementations: Vec::new(),
        error_info: None,
    };

    assert_eq!(enhanced_item.base.name, "test_item");
    assert!(enhanced_item.type_info.is_some());
    assert_eq!(enhanced_item.type_info.as_ref().unwrap().type_name, "test_function");
}
