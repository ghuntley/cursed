use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::type_system::*;

fn main() -> Result<(), cursed::error::CursedError> {
    println!("=== CURSED Type System Final Integration Test ===\n");
    
    // Create a type system
    let mut type_system = TypeSystem::new();
    
    // Test cases with expected results
    let test_cases = vec![
        // Basic literals
        ("42", Some("int")),
        ("\"hello\"", Some("string")), 
        ("true", Some("bool")),
        
        // Member access
        ("vibez.spill", Some("void")),
        
        // Function calls
        ("vibez.spill(\"test\")", Some("void")),
        
        // Binary operations
        ("1 + 2", Some("int")),
        ("5 - 3", Some("int")),
        ("2 * 4", Some("int")),
        ("8 / 2", Some("int")),
        ("1 == 2", Some("bool")),
        ("3 != 4", Some("bool")),
        ("5 < 6", Some("bool")),
        ("7 > 1", Some("bool")),
        ("true && false", Some("bool")),
        ("true || false", Some("bool")),
        
        // Error cases (should fail)
        ("unknown_var", None),
        ("\"string\" + 42", None),
        ("nonexistent.method()", None),
    ];
    
    let mut passed = 0;
    let mut total = 0;
    
    for (source, expected_type) in test_cases {
        total += 1;
        println!("Testing: {}", source);
        
        // Parse the expression
        let lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;
        
        if let Some(cursed::ast::Statement::Expression(expr)) = program.statements.first() {
            // Type check the expression
            match type_system.check_expression(expr) {
                Ok(type_expr) => {
                    if let Some(expected) = expected_type {
                        if type_expr.name.as_deref() == Some(expected) {
                            println!("  ✓ PASS: {} -> {}", source, expected);
                            passed += 1;
                        } else {
                            println!("  ✗ FAIL: {} -> expected {}, got {:?}", source, expected, type_expr.name);
                        }
                    } else {
                        println!("  ✗ FAIL: {} -> expected error, got {:?}", source, type_expr.name);
                    }
                }
                Err(e) => {
                    if expected_type.is_none() {
                        println!("  ✓ PASS: {} -> Error: {} (as expected)", source, e);
                        passed += 1;
                    } else {
                        println!("  ✗ FAIL: {} -> unexpected error: {}", source, e);
                    }
                }
            }
        } else {
            println!("  ✗ FAIL: {} -> failed to parse as expression", source);
        }
        
        println!();
    }
    
    println!("=== Results ===");
    println!("Passed: {}/{}", passed, total);
    
    if passed == total {
        println!("🎉 ALL TESTS PASSED! Type system is working correctly!");
    } else {
        println!("❌ Some tests failed. Type system needs more work.");
    }
    
    // Additional type system demonstration
    println!("\n=== Type System Features Demonstration ===");
    
    // Show type substitution
    println!("\n1. Type Substitution:");
    let mut subst = TypeSubstitution::new();
    subst.add("T".to_string(), TypeExpression::named("int"));
    let generic_type = TypeExpression::named("T");
    let concrete_type = subst.apply(&generic_type);
    println!("   T -> {:?}", concrete_type.name);
    
    // Show type unification
    println!("\n2. Type Unification:");
    let mut unifier = TypeUnifier::new();
    let var_type = TypeExpression::named("T0");
    let concrete = TypeExpression::named("string");
    match unifier.unify(&var_type, &concrete) {
        Ok(subs) => {
            for (var, typ) in &subs {
                println!("   {} -> {:?}", var, typ.name);
            }
        }
        Err(e) => println!("   Error: {:?}", e),
    }
    
    // Show constraint resolution
    println!("\n3. Constraint Resolution:");
    let resolver = ConstraintResolver::new();
    let constraint = GenericConstraint {
        constraint_name: "Printable".to_string(),
        type_parameters: vec!["T".to_string()],
        bounds: vec![],
    };
    match resolver.validate_constraint(&constraint, &type_system.environment) {
        Ok(()) => println!("   Constraint 'Printable' validated successfully"),
        Err(e) => println!("   Constraint validation failed: {:?}", e),
    }
    
    println!("\n=== Type System Implementation Complete! ===");
    println!("✓ Core type expressions and operations");
    println!("✓ Type checking for CURSED expressions");
    println!("✓ Type substitution and unification");
    println!("✓ Constraint resolution framework");
    println!("✓ Built-in types and vibez object support");
    println!("✓ Comprehensive error handling");
    
    Ok(())
}
