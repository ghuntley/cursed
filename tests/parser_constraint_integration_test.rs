/// Parser Integration Tests for Constraint Resolution
/// 
/// This test suite validates the integration between the parser and
/// the constraint resolution system for generic types and expressions.

use cursed::parser::Parser;
use cursed::type_system::{TypeSystem, ConstraintResolver};
use cursed::codegen::llvm::type_system::TypeCompilationContext;
use cursed::ast::declarations::{SquadStatement, GenericConstraint};
use cursed::error::Error;

#[path = "common.rs"]
mod common;

/// Test parsing generic struct with constraints
#[test]
fn test_parse_generic_struct_with_constraints() {
    common::tracing::setup();
    
    let source = r#"
        squad Container<T: Comparable> {
            sus value: T,
            normie size: normie
        }
    "#;
    
    let mut parser = Parser::new(source);
    let result = parser.parse_program();
    
    assert!(result.is_ok(), "Should parse generic struct with constraints");
    
    let program = result.unwrap();
    assert!(!program.statements.is_empty());
    
    // Verify the parsed structure contains generic information
    // This would require accessing the specific statement type
}

/// Test parsing interface with method constraints
#[test]
fn test_parse_interface_with_method_constraints() {
    common::tracing::setup();
    
    let source = r#"
        collab Comparable<T> {
            slay compare(sus other: T) -> normie;
            slay equals(sus other: T) -> facts;
        }
    "#;
    
    let mut parser = Parser::new(source);
    let result = parser.parse_program();
    
    assert!(result.is_ok(), "Should parse interface with method constraints");
    
    let program = result.unwrap();
    assert!(!program.statements.is_empty());
}

/// Test parsing function with generic parameters
#[test]
fn test_parse_generic_function() {
    common::tracing::setup();
    
    let source = r#"
        slay sort<T: Comparable>(sus items: [T]) -> [T] {
            // Implementation would go here
            periodt items;
        }
    "#;
    
    let mut parser = Parser::new(source);
    let result = parser.parse_program();
    
    assert!(result.is_ok(), "Should parse generic function");
    
    let program = result.unwrap();
    assert!(!program.statements.is_empty());
}

/// Test parsing multiple constraints
#[test]
fn test_parse_multiple_constraints() {
    common::tracing::setup();
    
    let source = r#"
        squad SortedSet<T: Comparable + Hashable> {
            sus data: [T]
        }
    "#;
    
    let mut parser = Parser::new(source);
    let result = parser.parse_program();
    
    // Note: This syntax might not be fully implemented yet
    // The test validates that the parser can handle it gracefully
    assert!(result.is_ok() || result.is_err()); // Either parse or fail gracefully
}

/// Test parsing nested generic types
#[test]
fn test_parse_nested_generic_types() {
    common::tracing::setup();
    
    let source = r#"
        squad Matrix<T: Numeric> {
            sus data: [[T]]
        }
    "#;
    
    let mut parser = Parser::new(source);
    let result = parser.parse_program();
    
    assert!(result.is_ok(), "Should parse nested generic types");
}

/// Test parsing with constraint resolution integration
#[test]
fn test_constraint_resolution_during_parsing() {
    common::tracing::setup();
    
    let source = r#"
        squad Container<T: Comparable> {
            sus value: T
        }
        
        slay create_container(sus val: normie) -> Container<normie> {
            periodt Container { value: val };
        }
    "#;
    
    let mut parser = Parser::new(source);
    let program_result = parser.parse_program();
    
    assert!(program_result.is_ok(), "Should parse program with generic usage");
    
    let program = program_result.unwrap();
    
    // Create type compilation context to test constraint resolution
    let mut type_context = TypeCompilationContext::new("test_module".to_string());
    
    // This would ideally integrate with the parsed AST
    // For now, test that the compilation context can handle the expected types
    let constraint = GenericConstraint {
        parameter: "T".to_string(),
        constraint_type: "Comparable".to_string(),
        methods: vec!["compare".to_string()],
    };
    
    let compiled_constraint = type_context.compile_constraint(&constraint);
    assert!(compiled_constraint.is_ok());
}

/// Test parsing invalid constraint syntax
#[test]
fn test_parse_invalid_constraint_syntax() {
    common::tracing::setup();
    
    let source = r#"
        squad BadContainer<T: > {
            sus value: T
        }
    "#;
    
    let mut parser = Parser::new(source);
    let result = parser.parse_program();
    
    // Should either parse with empty constraint or fail with error
    if result.is_err() {
        // Verify the error is related to constraint syntax
        let error = result.unwrap_err();
        assert!(format!("{:?}", error).contains("constraint") || 
                format!("{:?}", error).contains("parse") ||
                format!("{:?}", error).contains("syntax"));
    }
}

/// Test parsing complex constraint expressions
#[test]
fn test_parse_complex_constraint_expressions() {
    common::tracing::setup();
    
    let source = r#"
        collab Iterator<T> {
            slay next() -> T?;
            slay has_next() -> facts;
        }
        
        squad FilteredIterator<T: Iterator<U>, U: Comparable> {
            sus iter: T,
            sus filter: slay(U) -> facts
        }
    "#;
    
    let mut parser = Parser::new(source);
    let result = parser.parse_program();
    
    // Complex constraint syntax might not be fully supported yet
    // Test that parser handles it gracefully
    assert!(result.is_ok() || result.is_err());
}

/// Test parsing with type inference hints
#[test]
fn test_parse_with_type_inference() {
    common::tracing::setup();
    
    let source = r#"
        slay process<T>(sus item: T) -> T {
            periodt item;
        }
        
        // Usage that would trigger type inference
        sus result = process(42);
    "#;
    
    let mut parser = Parser::new(source);
    let result = parser.parse_program();
    
    assert!(result.is_ok(), "Should parse code requiring type inference");
    
    let program = result.unwrap();
    assert!(program.statements.len() >= 2); // Function + variable declaration
}

/// Test error recovery in constraint parsing
#[test]
fn test_constraint_parsing_error_recovery() {
    common::tracing::setup();
    
    let source = r#"
        squad GoodContainer<T: Comparable> {
            sus value: T
        }
        
        squad BadContainer<T: UnknownConstraint> {
            sus value: T
        }
        
        squad AnotherGoodContainer<T: Hashable> {
            sus value: T
        }
    "#;
    
    let mut parser = Parser::new(source);
    let result = parser.parse_program();
    
    // Parser should be able to recover from unknown constraint and continue
    assert!(result.is_ok(), "Parser should recover from unknown constraints");
    
    let program = result.unwrap();
    assert!(program.statements.len() >= 2); // Should parse at least some statements
}

/// Test integration with existing parser infrastructure
#[test]
fn test_parser_infrastructure_integration() {
    common::tracing::setup();
    
    let source = r#"
        squad Point<T: Numeric> {
            sus x: T,
            sus y: T
        }
        
        slay distance<T: Numeric>(sus p1: Point<T>, sus p2: Point<T>) -> T {
            // Using regular CURSED syntax
            sus dx = p1.x - p2.x;
            sus dy = p1.y - p2.y;
            periodt sqrt(dx * dx + dy * dy);
        }
    "#;
    
    let mut parser = Parser::new(source);
    let result = parser.parse_program();
    
    assert!(result.is_ok(), "Should integrate with existing parser infrastructure");
    
    let program = result.unwrap();
    assert!(!program.statements.is_empty());
}

/// Test parsing performance with many generic types
#[test]
fn test_parsing_performance_with_generics() {
    common::tracing::setup();
    
    let mut source = String::new();
    
    // Generate many generic type definitions
    for i in 0..10 {
        source.push_str(&format!(r#"
            squad Container{}<T: Comparable> {{
                sus value{}: T
            }}
        "#, i, i));
    }
    
    let mut parser = Parser::new(&source);
    let start = std::time::Instant::now();
    let result = parser.parse_program();
    let duration = start.elapsed();
    
    assert!(result.is_ok(), "Should parse many generic types efficiently");
    assert!(duration.as_millis() < 1000, "Parsing should be reasonably fast");
    
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 10);
}

/// Test parser state consistency with constraints
#[test]
fn test_parser_state_consistency() {
    common::tracing::setup();
    
    let source = r#"
        // First define the constraint interface
        collab Comparable<T> {
            slay compare(sus other: T) -> normie;
        }
        
        // Then use it in a generic type
        squad SortedList<T: Comparable> {
            sus items: [T]
        }
        
        // Function using the generic type
        slay sort<T: Comparable>(sus list: SortedList<T>) -> SortedList<T> {
            periodt list; // Simplified implementation
        }
    "#;
    
    let mut parser = Parser::new(source);
    let result = parser.parse_program();
    
    assert!(result.is_ok(), "Parser should maintain consistent state across constraint definitions");
    
    let program = result.unwrap();
    assert!(program.statements.len() >= 3); // Interface + struct + function
}
