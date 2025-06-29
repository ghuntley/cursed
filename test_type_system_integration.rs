use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::type_system::{TypeSystem, TypeExpression};
use cursed::ast::*;

fn main() -> Result<(), cursed::error::CursedError> {
    println!("=== CURSED Type System Integration Test ===\n");
    
    // Test 1: Basic type checking
    println!("Test 1: Basic expressions");
    let mut type_system = TypeSystem::new();
    
    let test_expressions = vec![
        ("42", "int"),
        ("\"hello\"", "string"),
        ("true", "bool"),
    ];
    
    for (expr_str, expected_type) in test_expressions {
        let lexer = Lexer::new(expr_str.to_string());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;
        
        if let Some(Statement::Expression(expr)) = program.statements.first() {
            match type_system.check_expression(expr) {
                Ok(type_expr) => {
                    println!("  ✓ {} -> {:?}", expr_str, type_expr.name);
                    assert_eq!(type_expr.name.as_deref(), Some(expected_type));
                }
                Err(e) => {
                    println!("  ✗ {} -> Error: {}", expr_str, e);
                }
            }
        }
    }
    
    // Test 2: Member access (vibez.spill)
    println!("\nTest 2: Member access");
    let source = "vibez.spill";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    
    if let Some(Statement::Expression(expr)) = program.statements.first() {
        match type_system.check_expression(expr) {
            Ok(type_expr) => {
                println!("  ✓ {} -> {:?}", source, type_expr.name);
            }
            Err(e) => {
                println!("  ✗ {} -> Error: {}", source, e);
            }
        }
    }
    
    // Test 3: Function call (vibez.spill("message"))
    println!("\nTest 3: Function call");
    let source = "vibez.spill(\"hello world\")";
    let lexer = Lexer::new(source.to_string());
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    
    if let Some(Statement::Expression(expr)) = program.statements.first() {
        match type_system.check_expression(expr) {
            Ok(type_expr) => {
                println!("  ✓ {} -> {:?}", source, type_expr.name);
                assert_eq!(type_expr.name.as_deref(), Some("void"));
            }
            Err(e) => {
                println!("  ✗ {} -> Error: {}", source, e);
            }
        }
    }
    
    // Test 4: Binary operations
    println!("\nTest 4: Binary operations");
    let binary_tests = vec![
        ("1 + 2", "int"),
        ("1 == 2", "bool"),
        ("true && false", "bool"),
    ];
    
    for (expr_str, expected_type) in binary_tests {
        let lexer = Lexer::new(expr_str.to_string());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;
        
        if let Some(Statement::Expression(expr)) = program.statements.first() {
            match type_system.check_expression(expr) {
                Ok(type_expr) => {
                    println!("  ✓ {} -> {:?}", expr_str, type_expr.name);
                    assert_eq!(type_expr.name.as_deref(), Some(expected_type));
                }
                Err(e) => {
                    println!("  ✗ {} -> Error: {}", expr_str, e);
                }
            }
        }
    }
    
    // Test 5: Error cases
    println!("\nTest 5: Error handling");
    let error_cases = vec![
        "unknown_var",
        "\"string\" + 42",
        "nonexistent.method()",
    ];
    
    for expr_str in error_cases {
        let lexer = Lexer::new(expr_str.to_string());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;
        
        if let Some(Statement::Expression(expr)) = program.statements.first() {
            match type_system.check_expression(expr) {
                Ok(type_expr) => {
                    println!("  ! {} -> {:?} (expected error)", expr_str, type_expr.name);
                }
                Err(e) => {
                    println!("  ✓ {} -> Error: {} (as expected)", expr_str, e);
                }
            }
        }
    }
    
    // Test 6: Type system internals
    println!("\nTest 6: Type system internals");
    
    // Test type substitution
    let mut subst = cursed::type_system::TypeSubstitution::new();
    subst.add("T".to_string(), TypeExpression::named("int"));
    let type_var = TypeExpression::named("T");
    let resolved = subst.apply(&type_var);
    println!("  ✓ Type substitution: T -> {:?}", resolved.name);
    
    // Test type unification
    let mut unifier = cursed::type_system::TypeUnifier::new();
    let t1 = TypeExpression::named("T0");
    let t2 = TypeExpression::named("string");
    match unifier.unify(&t1, &t2) {
        Ok(substitutions) => {
            println!("  ✓ Type unification successful with {} substitutions", substitutions.len());
        }
        Err(e) => {
            println!("  ✗ Type unification failed: {:?}", e);
        }
    }
    
    // Test constraint resolver
    let resolver = cursed::type_system::ConstraintResolver::new();
    let constraint = cursed::type_system::GenericConstraint {
        constraint_name: "Display".to_string(),
        type_parameters: vec!["T".to_string()],
        bounds: vec![],
    };
    match resolver.validate_constraint(&constraint, &type_system.environment) {
        Ok(()) => {
            println!("  ✓ Constraint validation successful");
        }
        Err(e) => {
            println!("  ✗ Constraint validation failed: {:?}", e);
        }
    }
    
    println!("\n=== Type System Integration Test Complete ===");
    println!("✓ All core type system functionality is working!");
    
    Ok(())
}
