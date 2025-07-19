use cursed::type_system::checker::{TypeChecker, VariableInfo};
use cursed::type_system::{TypeExpression, TypeCheckError};
use cursed::ast::{LetStatement, LetTarget, Expression, Literal, Statement, AssignmentStatement, AssignmentTarget, ConstDecl, ConstSpec};

#[test]
fn test_mutability_tracking_let_statement() {
    let mut checker = TypeChecker::new();
    
    // Create a let statement (sus variable - mutable)
    let let_stmt = LetStatement {
        target: LetTarget::Single("test_var".to_string()),
        value: Expression::Literal(Literal::Integer(42)),
        var_type: None,
        visibility: cursed::ast::Visibility::Private,
    };
    
    // Check the let statement
    let result = checker.check_let_statement(&let_stmt);
    assert!(result.is_ok());
    
    // Verify the variable was added with correct mutability
    let var_info = checker.get_variable("test_var");
    assert!(var_info.is_some());
    assert!(var_info.unwrap().is_mutable, "Let statement variables should be mutable");
}

#[test]
fn test_mutability_tracking_const_statement() {
    let mut checker = TypeChecker::new();
    
    // Create a const statement (facts constant - immutable)
    let const_decl = ConstDecl {
        specs: vec![ConstSpec {
            names: vec!["test_const".to_string()],
            const_type: None,
            values: vec![Expression::Literal(Literal::Integer(100))],
        }],
    };
    
    // Check the const statement
    let result = checker.check_const_statement(&const_decl);
    assert!(result.is_ok());
    
    // Verify the constant was added with correct mutability
    let var_info = checker.get_variable("test_const");
    assert!(var_info.is_some());
    assert!(!var_info.unwrap().is_mutable, "Const statement variables should be immutable");
}

#[test]
fn test_assignment_to_mutable_variable() {
    let mut checker = TypeChecker::new();
    
    // Add a mutable variable manually
    checker.add_variable_with_mutability(
        "mutable_var".to_string(),
        TypeExpression::named("normie"),
        true
    );
    
    // Create an assignment statement
    let assignment = AssignmentStatement {
        target: AssignmentTarget::Single("mutable_var".to_string()),
        value: Expression::Literal(Literal::Integer(84)),
    };
    
    // Check the assignment - should succeed
    let result = checker.check_assignment_statement(&assignment);
    assert!(result.is_ok(), "Assignment to mutable variable should succeed");
}

#[test]
fn test_assignment_to_immutable_variable() {
    let mut checker = TypeChecker::new();
    
    // Add an immutable variable manually
    checker.add_variable_with_mutability(
        "immutable_var".to_string(),
        TypeExpression::named("normie"),
        false
    );
    
    // Create an assignment statement
    let assignment = AssignmentStatement {
        target: AssignmentTarget::Single("immutable_var".to_string()),
        value: Expression::Literal(Literal::Integer(200)),
    };
    
    // Check the assignment - should fail with mutability error
    let result = checker.check_assignment_statement(&assignment);
    assert!(result.is_err(), "Assignment to immutable variable should fail");
    
    let error = result.unwrap_err();
    assert!(error.message.contains("Cannot assign to immutable variable"), 
            "Error should mention immutability violation");
}
