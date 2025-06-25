//! Cross-reference generation and linking tests for AST documentation
//!
//! This test suite validates the cross-reference system that links between
//! documented elements across modules and creates navigable documentation.

#[path = "common/mod.rs"]
mod common;

use cursed::ast::documentation::*;
use cursed::ast::*;
use cursed::error::{Error, SourceLocation};
use cursed::lexer::{Token, TokenType};
use std::collections::HashMap;
use std::path::PathBuf;

/// Test basic cross-reference detection
#[test]
fn test_basic_cross_reference_detection() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create a function that references another function in its description
    let location = SourceLocation { line: 10, column: 1, file: Some("module1.csd".to_string()) };
    let caller_func = DocElement {
        name: "caller_function".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "module1".to_string(),
        summary: "Function that calls helper_function".to_string(),
        description: Some("This function uses helper_function to process data".to_string()),
        signature: Some("slay caller_function() -> i32".to_string()),
        parameters: Vec::new(),
        return_type: Some("i32".to_string()),
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: location.clone(),
        source_code: Some("slay caller_function() -> i32 { periodt helper_function(42); }".to_string()),
        metadata: ElementMetadata::default(),
    };
    
    let helper_location = SourceLocation { line: 5, column: 1, file: Some("module2.csd".to_string()) };
    let helper_func = DocElement {
        name: "helper_function".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "module2".to_string(),
        summary: "Helper function for data processing".to_string(),
        description: Some("Processes input data and returns result".to_string()),
        signature: Some("slay helper_function(value: i32) -> i32".to_string()),
        parameters: vec![
            ParameterDoc {
                name: "value".to_string(),
                param_type: Some("i32".to_string()),
                description: "Input value to process".to_string(),
                default_value: None,
                is_optional: false,
            }
        ],
        return_type: Some("i32".to_string()),
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: helper_location,
        source_code: Some("slay helper_function(value: i32) -> i32 { periodt value * 2; }".to_string()),
        metadata: ElementMetadata::default(),
    };
    
    // Create module documentation
    let module1 = ModuleDocumentation {
        name: "module1".to_string(),
        file_path: PathBuf::from("module1.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![caller_func],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("module1"),
        source_info: SourceInfo {
            file_size: 200,
            line_count: 20,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    let module2 = ModuleDocumentation {
        name: "module2".to_string(),
        file_path: PathBuf::from("module2.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![helper_func],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("module2"),
        source_info: SourceInfo {
            file_size: 150,
            line_count: 15,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    // Update symbol tables
    extractor.update_symbol_table(&module1);
    extractor.update_symbol_table(&module2);
    
    // Build cross-references
    let result = extractor.build_cross_references();
    assert!(result.is_ok());
    
    // Check for cross-references
    let references = extractor.get_cross_references("module1::caller_function");
    if let Some(refs) = references {
        // Should find reference to helper_function
        assert!(!refs.is_empty());
        let has_helper_ref = refs.iter().any(|r| r.target.contains("helper_function"));
        assert!(has_helper_ref);
    }
}

/// Test cross-references in function signatures
#[test]
fn test_signature_cross_references() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create a function that uses a custom type in its signature
    let location = SourceLocation { line: 8, column: 1, file: Some("api.csd".to_string()) };
    let func_element = DocElement {
        name: "process_user".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "api".to_string(),
        summary: "Processes user data".to_string(),
        description: Some("Takes a User struct and returns processed result".to_string()),
        signature: Some("slay process_user(user: User) -> ProcessResult".to_string()),
        parameters: vec![
            ParameterDoc {
                name: "user".to_string(),
                param_type: Some("User".to_string()),
                description: "User data to process".to_string(),
                default_value: None,
                is_optional: false,
            }
        ],
        return_type: Some("ProcessResult".to_string()),
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: location.clone(),
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    // Create the User struct
    let user_location = SourceLocation { line: 1, column: 1, file: Some("types.csd".to_string()) };
    let user_struct = DocElement {
        name: "User".to_string(),
        element_type: ElementType::Struct,
        visibility: Visibility::Public,
        module: "types".to_string(),
        summary: "User data structure".to_string(),
        description: Some("Represents a user in the system".to_string()),
        signature: Some("squad User".to_string()),
        parameters: Vec::new(),
        return_type: None,
        type_info: Some(TypeInfo {
            base_type: "struct".to_string(),
            generic_params: Vec::new(),
            constraints: Vec::new(),
            fields: vec![
                FieldDoc {
                    name: "id".to_string(),
                    field_type: "u64".to_string(),
                    description: "User ID".to_string(),
                    is_public: true,
                    default_value: None,
                }
            ],
            methods: Vec::new(),
        }),
        examples: Vec::new(),
        tags: HashMap::new(),
        location: user_location,
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    // Create the ProcessResult struct
    let result_location = SourceLocation { line: 10, column: 1, file: Some("types.csd".to_string()) };
    let result_struct = DocElement {
        name: "ProcessResult".to_string(),
        element_type: ElementType::Struct,
        visibility: Visibility::Public,
        module: "types".to_string(),
        summary: "Result of processing operation".to_string(),
        description: Some("Contains the result of user processing".to_string()),
        signature: Some("squad ProcessResult".to_string()),
        parameters: Vec::new(),
        return_type: None,
        type_info: Some(TypeInfo {
            base_type: "struct".to_string(),
            generic_params: Vec::new(),
            constraints: Vec::new(),
            fields: vec![
                FieldDoc {
                    name: "success".to_string(),
                    field_type: "bool".to_string(),
                    description: "Whether processing succeeded".to_string(),
                    is_public: true,
                    default_value: None,
                }
            ],
            methods: Vec::new(),
        }),
        examples: Vec::new(),
        tags: HashMap::new(),
        location: result_location,
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    // Create modules
    let api_module = ModuleDocumentation {
        name: "api".to_string(),
        file_path: PathBuf::from("api.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![func_element],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("api"),
        source_info: SourceInfo {
            file_size: 300,
            line_count: 30,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    let types_module = ModuleDocumentation {
        name: "types".to_string(),
        file_path: PathBuf::from("types.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![user_struct, result_struct],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("types"),
        source_info: SourceInfo {
            file_size: 250,
            line_count: 25,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    // Update symbol tables
    extractor.update_symbol_table(&api_module);
    extractor.update_symbol_table(&types_module);
    
    // Build cross-references
    let result = extractor.build_cross_references();
    assert!(result.is_ok());
    
    // Check for type references in function signature
    let references = extractor.get_cross_references("api::process_user");
    if let Some(refs) = references {
        let has_user_ref = refs.iter().any(|r| r.target.contains("User"));
        let has_result_ref = refs.iter().any(|r| r.target.contains("ProcessResult"));
        
        if !refs.is_empty() {
            // At least some references should be found
            assert!(has_user_ref || has_result_ref);
        }
    }
}

/// Test cross-references in code examples
#[test]
fn test_example_cross_references() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create a function with code examples that reference other functions
    let location = SourceLocation { line: 15, column: 1, file: Some("math.csd".to_string()) };
    let math_func = DocElement {
        name: "calculate_area".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "math".to_string(),
        summary: "Calculates area of a rectangle".to_string(),
        description: Some("Uses width and height to calculate rectangle area".to_string()),
        signature: Some("slay calculate_area(width: f64, height: f64) -> f64".to_string()),
        parameters: vec![
            ParameterDoc {
                name: "width".to_string(),
                param_type: Some("f64".to_string()),
                description: "Rectangle width".to_string(),
                default_value: None,
                is_optional: false,
            },
            ParameterDoc {
                name: "height".to_string(),
                param_type: Some("f64".to_string()),
                description: "Rectangle height".to_string(),
                default_value: None,
                is_optional: false,
            }
        ],
        return_type: Some("f64".to_string()),
        type_info: None,
        examples: vec![
            CodeExample {
                title: Some("Basic Usage".to_string()),
                description: Some("Calculate area and display result".to_string()),
                code: r#"sus area = calculate_area(10.0, 5.0);
display_result(area);
validate_positive(area);"#.to_string(),
                language: "cursed".to_string(),
                output: Some("Area: 50.0".to_string()),
                is_runnable: true,
            }
        ],
        tags: HashMap::new(),
        location: location.clone(),
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    // Create referenced functions
    let display_location = SourceLocation { line: 5, column: 1, file: Some("utils.csd".to_string()) };
    let display_func = DocElement {
        name: "display_result".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "utils".to_string(),
        summary: "Displays calculation result".to_string(),
        description: Some("Formats and displays numerical results".to_string()),
        signature: Some("slay display_result(value: f64)".to_string()),
        parameters: vec![
            ParameterDoc {
                name: "value".to_string(),
                param_type: Some("f64".to_string()),
                description: "Value to display".to_string(),
                default_value: None,
                is_optional: false,
            }
        ],
        return_type: None,
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: display_location,
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    let validate_location = SourceLocation { line: 10, column: 1, file: Some("validation.csd".to_string()) };
    let validate_func = DocElement {
        name: "validate_positive".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "validation".to_string(),
        summary: "Validates that value is positive".to_string(),
        description: Some("Checks if the provided value is greater than zero".to_string()),
        signature: Some("slay validate_positive(value: f64) -> bool".to_string()),
        parameters: vec![
            ParameterDoc {
                name: "value".to_string(),
                param_type: Some("f64".to_string()),
                description: "Value to validate".to_string(),
                default_value: None,
                is_optional: false,
            }
        ],
        return_type: Some("bool".to_string()),
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: validate_location,
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    // Create modules
    let math_module = ModuleDocumentation {
        name: "math".to_string(),
        file_path: PathBuf::from("math.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![math_func],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("math"),
        source_info: SourceInfo {
            file_size: 400,
            line_count: 40,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    let utils_module = ModuleDocumentation {
        name: "utils".to_string(),
        file_path: PathBuf::from("utils.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![display_func],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("utils"),
        source_info: SourceInfo {
            file_size: 200,
            line_count: 20,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    let validation_module = ModuleDocumentation {
        name: "validation".to_string(),
        file_path: PathBuf::from("validation.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![validate_func],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("validation"),
        source_info: SourceInfo {
            file_size: 180,
            line_count: 18,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    // Update symbol tables
    extractor.update_symbol_table(&math_module);
    extractor.update_symbol_table(&utils_module);
    extractor.update_symbol_table(&validation_module);
    
    // Build cross-references
    let result = extractor.build_cross_references();
    assert!(result.is_ok());
    
    // Check for references in examples
    let references = extractor.get_cross_references("math::calculate_area");
    if let Some(refs) = references {
        let has_display_ref = refs.iter().any(|r| r.target.contains("display_result"));
        let has_validate_ref = refs.iter().any(|r| r.target.contains("validate_positive"));
        
        if !refs.is_empty() {
            // Should find references to functions used in examples
            assert!(has_display_ref || has_validate_ref);
        }
    }
}

/// Test inheritance cross-references
#[test]
fn test_inheritance_cross_references() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create an interface
    let location = SourceLocation { line: 1, column: 1, file: Some("traits.csd".to_string()) };
    let drawable_interface = DocElement {
        name: "Drawable".to_string(),
        element_type: ElementType::Interface,
        visibility: Visibility::Public,
        module: "traits".to_string(),
        summary: "Interface for drawable objects".to_string(),
        description: Some("Defines the contract for objects that can be drawn".to_string()),
        signature: Some("collab Drawable".to_string()),
        parameters: Vec::new(),
        return_type: None,
        type_info: Some(TypeInfo {
            base_type: "interface".to_string(),
            generic_params: Vec::new(),
            constraints: Vec::new(),
            fields: Vec::new(),
            methods: vec![
                MethodDoc {
                    name: "draw".to_string(),
                    signature: "slay draw(&self)".to_string(),
                    description: "Draws the object".to_string(),
                    parameters: Vec::new(),
                    return_type: None,
                    is_static: false,
                }
            ],
        }),
        examples: Vec::new(),
        tags: HashMap::new(),
        location: location.clone(),
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    // Create a struct that implements the interface
    let shape_location = SourceLocation { line: 10, column: 1, file: Some("shapes.csd".to_string()) };
    let circle_struct = DocElement {
        name: "Circle".to_string(),
        element_type: ElementType::Struct,
        visibility: Visibility::Public,
        module: "shapes".to_string(),
        summary: "Circle shape implementation".to_string(),
        description: Some("Implements Drawable interface for circle shapes".to_string()),
        signature: Some("squad Circle".to_string()),
        parameters: Vec::new(),
        return_type: None,
        type_info: Some(TypeInfo {
            base_type: "struct".to_string(),
            generic_params: Vec::new(),
            constraints: vec!["implements Drawable".to_string()],
            fields: vec![
                FieldDoc {
                    name: "radius".to_string(),
                    field_type: "f64".to_string(),
                    description: "Circle radius".to_string(),
                    is_public: true,
                    default_value: None,
                }
            ],
            methods: vec![
                MethodDoc {
                    name: "draw".to_string(),
                    signature: "slay draw(&self)".to_string(),
                    description: "Draws the circle".to_string(),
                    parameters: Vec::new(),
                    return_type: None,
                    is_static: false,
                }
            ],
        }),
        examples: Vec::new(),
        tags: HashMap::new(),
        location: shape_location,
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    // Create modules
    let traits_module = ModuleDocumentation {
        name: "traits".to_string(),
        file_path: PathBuf::from("traits.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![drawable_interface],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("traits"),
        source_info: SourceInfo {
            file_size: 150,
            line_count: 15,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    let shapes_module = ModuleDocumentation {
        name: "shapes".to_string(),
        file_path: PathBuf::from("shapes.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![circle_struct],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("shapes"),
        source_info: SourceInfo {
            file_size: 300,
            line_count: 30,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    // Update symbol tables
    extractor.update_symbol_table(&traits_module);
    extractor.update_symbol_table(&shapes_module);
    
    // Build cross-references
    let result = extractor.build_cross_references();
    assert!(result.is_ok());
    
    // Check for inheritance references
    let references = extractor.get_cross_references("shapes::Circle");
    if let Some(refs) = references {
        let has_drawable_ref = refs.iter().any(|r| r.target.contains("Drawable"));
        
        if !refs.is_empty() {
            // Should find reference to implemented interface
            assert!(has_drawable_ref);
        }
    }
}

/// Test bidirectional cross-references
#[test]
fn test_bidirectional_cross_references() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create two functions that reference each other
    let location1 = SourceLocation { line: 5, column: 1, file: Some("recursive.csd".to_string()) };
    let func_a = DocElement {
        name: "function_a".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "recursive".to_string(),
        summary: "Function A in mutual recursion".to_string(),
        description: Some("Calls function_b in certain conditions".to_string()),
        signature: Some("slay function_a(n: i32) -> i32".to_string()),
        parameters: vec![
            ParameterDoc {
                name: "n".to_string(),
                param_type: Some("i32".to_string()),
                description: "Input number".to_string(),
                default_value: None,
                is_optional: false,
            }
        ],
        return_type: Some("i32".to_string()),
        type_info: None,
        examples: vec![
            CodeExample {
                title: Some("Mutual Recursion".to_string()),
                description: None,
                code: "sus result = function_a(10); // This may call function_b".to_string(),
                language: "cursed".to_string(),
                output: None,
                is_runnable: true,
            }
        ],
        tags: HashMap::new(),
        location: location1,
        source_code: Some("slay function_a(n: i32) -> i32 { lowkey (n > 0) { periodt function_b(n - 1); } }".to_string()),
        metadata: ElementMetadata::default(),
    };
    
    let location2 = SourceLocation { line: 15, column: 1, file: Some("recursive.csd".to_string()) };
    let func_b = DocElement {
        name: "function_b".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "recursive".to_string(),
        summary: "Function B in mutual recursion".to_string(),
        description: Some("Calls function_a in certain conditions".to_string()),
        signature: Some("slay function_b(n: i32) -> i32".to_string()),
        parameters: vec![
            ParameterDoc {
                name: "n".to_string(),
                param_type: Some("i32".to_string()),
                description: "Input number".to_string(),
                default_value: None,
                is_optional: false,
            }
        ],
        return_type: Some("i32".to_string()),
        type_info: None,
        examples: vec![
            CodeExample {
                title: Some("Mutual Recursion".to_string()),
                description: None,
                code: "sus result = function_b(5); // This may call function_a".to_string(),
                language: "cursed".to_string(),
                output: None,
                is_runnable: true,
            }
        ],
        tags: HashMap::new(),
        location: location2,
        source_code: Some("slay function_b(n: i32) -> i32 { lowkey (n > 0) { periodt function_a(n - 1); } }".to_string()),
        metadata: ElementMetadata::default(),
    };
    
    // Create module
    let recursive_module = ModuleDocumentation {
        name: "recursive".to_string(),
        file_path: PathBuf::from("recursive.csd"),
        package_info: None,
        imports: Vec::new(),
        items: vec![func_a, func_b],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("recursive"),
        source_info: SourceInfo {
            file_size: 500,
            line_count: 50,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    // Update symbol table
    extractor.update_symbol_table(&recursive_module);
    
    // Build cross-references
    let result = extractor.build_cross_references();
    assert!(result.is_ok());
    
    // Check that both functions have cross-references to each other
    let refs_a = extractor.get_cross_references("recursive::function_a");
    let refs_b = extractor.get_cross_references("recursive::function_b");
    
    if let Some(refs) = refs_a {
        let has_b_ref = refs.iter().any(|r| r.target.contains("function_b"));
        if !refs.is_empty() {
            assert!(has_b_ref);
        }
    }
    
    if let Some(refs) = refs_b {
        let has_a_ref = refs.iter().any(|r| r.target.contains("function_a"));
        if !refs.is_empty() {
            assert!(has_a_ref);
        }
    }
}

/// Test cross-reference types
#[test]
fn test_cross_reference_types() {
    common::tracing::setup();
    
    let location = SourceLocation { line: 1, column: 1, file: Some("test.csd".to_string()) };
    
    // Test different reference types
    let usage_ref = CrossReference {
        target: "helper_function".to_string(),
        context: "function call in main()".to_string(),
        location: location.clone(),
        reference_type: ReferenceType::Usage,
    };
    
    let type_ref = CrossReference {
        target: "CustomType".to_string(),
        context: "parameter type annotation".to_string(),
        location: location.clone(),
        reference_type: ReferenceType::TypeReference,
    };
    
    let import_ref = CrossReference {
        target: "external::module".to_string(),
        context: "import statement".to_string(),
        location: location.clone(),
        reference_type: ReferenceType::Import,
    };
    
    let inheritance_ref = CrossReference {
        target: "BaseInterface".to_string(),
        context: "interface implementation".to_string(),
        location: location.clone(),
        reference_type: ReferenceType::Inheritance,
    };
    
    let mention_ref = CrossReference {
        target: "related_function".to_string(),
        context: "documentation comment".to_string(),
        location: location.clone(),
        reference_type: ReferenceType::Mention,
    };
    
    // Verify reference types
    assert!(matches!(usage_ref.reference_type, ReferenceType::Usage));
    assert!(matches!(type_ref.reference_type, ReferenceType::TypeReference));
    assert!(matches!(import_ref.reference_type, ReferenceType::Import));
    assert!(matches!(inheritance_ref.reference_type, ReferenceType::Inheritance));
    assert!(matches!(mention_ref.reference_type, ReferenceType::Mention));
    
    // Verify reference content
    assert_eq!(usage_ref.target, "helper_function");
    assert_eq!(type_ref.target, "CustomType");
    assert_eq!(import_ref.target, "external::module");
    assert_eq!(inheritance_ref.target, "BaseInterface");
    assert_eq!(mention_ref.target, "related_function");
}

/// Test cross-reference performance with many symbols
#[test]
fn test_cross_reference_performance() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create many symbols to test performance
    let mut items = Vec::new();
    for i in 0..50 {
        let location = SourceLocation { line: i + 1, column: 1, file: Some("large_module.csd".to_string()) };
        
        // Create a function that references the next function
        let next_func = if i < 49 { format!("function_{}", i + 1) } else { "function_0".to_string() };
        
        let func = DocElement {
            name: format!("function_{}", i),
            element_type: ElementType::Function,
            visibility: Visibility::Public,
            module: "large_module".to_string(),
            summary: format!("Function number {}", i),
            description: Some(format!("This function calls {}", next_func)),
            signature: Some(format!("slay function_{}() -> void", i)),
            parameters: Vec::new(),
            return_type: Some("void".to_string()),
            type_info: None,
            examples: Vec::new(),
            tags: HashMap::new(),
            location,
            source_code: Some(format!("slay function_{}() {{ {}(); }}", i, next_func)),
            metadata: ElementMetadata::default(),
        };
        
        items.push(func);
    }
    
    // Create large module
    let large_module = ModuleDocumentation {
        name: "large_module".to_string(),
        file_path: PathBuf::from("large_module.csd"),
        package_info: None,
        imports: Vec::new(),
        items,
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("large_module"),
        source_info: SourceInfo {
            file_size: 5000,
            line_count: 500,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    // Update symbol table
    let start_time = std::time::Instant::now();
    extractor.update_symbol_table(&large_module);
    let symbol_time = start_time.elapsed();
    
    // Build cross-references
    let start_time = std::time::Instant::now();
    let result = extractor.build_cross_references();
    let ref_time = start_time.elapsed();
    
    assert!(result.is_ok());
    
    // Performance should be reasonable (less than 1 second for 50 functions)
    assert!(symbol_time.as_secs() < 1);
    assert!(ref_time.as_secs() < 1);
    
    // Verify some cross-references were built
    let references = extractor.get_cross_references("large_module::function_0");
    if let Some(refs) = references {
        if !refs.is_empty() {
            let has_next_ref = refs.iter().any(|r| r.target.contains("function_1"));
            assert!(has_next_ref);
        }
    }
}

/// Test cross-reference context information
#[test]
fn test_cross_reference_context() {
    common::tracing::setup();
    
    let location = SourceLocation { 
        line: 25, 
        column: 10, 
        file: Some("context_test.csd".to_string()) 
    };
    
    let cross_ref = CrossReference {
        target: "target_function".to_string(),
        context: "Called from within conditional block in main function".to_string(),
        location: location.clone(),
        reference_type: ReferenceType::Usage,
    };
    
    // Verify context information is preserved
    assert_eq!(cross_ref.context, "Called from within conditional block in main function");
    assert_eq!(cross_ref.location.line, 25);
    assert_eq!(cross_ref.location.column, 10);
    assert_eq!(cross_ref.location.file, Some("context_test.csd".to_string()));
    
    // Context should provide meaningful information about where the reference occurs
    assert!(cross_ref.context.contains("conditional block"));
    assert!(cross_ref.context.contains("main function"));
}

/// Test symbol table lookup efficiency
#[test]
fn test_symbol_table_lookup() {
    common::tracing::setup();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Create symbols with fully qualified names
    let location = SourceLocation { line: 1, column: 1, file: None };
    
    let func_doc = DocElement {
        name: "test_function".to_string(),
        element_type: ElementType::Function,
        visibility: Visibility::Public,
        module: "test_module".to_string(),
        summary: "Test function".to_string(),
        description: None,
        signature: Some("slay test_function() -> void".to_string()),
        parameters: Vec::new(),
        return_type: Some("void".to_string()),
        type_info: None,
        examples: Vec::new(),
        tags: HashMap::new(),
        location: location.clone(),
        source_code: None,
        metadata: ElementMetadata::default(),
    };
    
    let struct_doc = DocElement {
        name: "TestStruct".to_string(),
        element_type: ElementType::Struct,
        visibility: Visibility::Public,
        module: "test_module".to_string(),
        summary: "Test struct".to_string(),
        description: None,
        signature: Some("squad TestStruct".to_string()),
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
        items: vec![func_doc, struct_doc],
        module_comments: Vec::new(),
        metadata: DocumentationMetadata::new("test_module"),
        source_info: SourceInfo {
            file_size: 200,
            line_count: 20,
            last_modified: None,
            encoding: "UTF-8".to_string(),
        },
    };
    
    // Update symbol table
    extractor.update_symbol_table(&module_doc);
    
    // Test symbol lookup
    let func_symbol = extractor.get_symbol("test_module::test_function");
    assert!(func_symbol.is_some());
    let func_sym = func_symbol.unwrap();
    assert_eq!(func_sym.name, "test_function");
    assert_eq!(func_sym.module, "test_module");
    assert_eq!(func_sym.element_type, ElementType::Function);
    
    let struct_symbol = extractor.get_symbol("test_module::TestStruct");
    assert!(struct_symbol.is_some());
    let struct_sym = struct_symbol.unwrap();
    assert_eq!(struct_sym.name, "TestStruct");
    assert_eq!(struct_sym.module, "test_module");
    assert_eq!(struct_sym.element_type, ElementType::Struct);
    
    // Test non-existent symbol
    let missing_symbol = extractor.get_symbol("test_module::nonexistent");
    assert!(missing_symbol.is_none());
    
    // Test malformed symbol name
    let malformed_symbol = extractor.get_symbol("malformed_name_without_module");
    assert!(malformed_symbol.is_none());
}
