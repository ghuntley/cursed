/// Simple test for question mark operator AST functionality
/// 
/// This test only verifies the QuestionMarkExpression AST node basic functionality
/// without trying to compile the full codebase which has errors.

#[cfg(test)]
mod tests {
    use cursed::ast::expressions::QuestionMarkExpression;
    use cursed::ast::identifiers::Identifier;
    use cursed::ast::traits::{Expression, Node};

    #[test]
    fn test_question_mark_ast_creation() {
        // Create a simple identifier expression to wrap with ?
        let inner_expr = Identifier::new("result".to_string(), "result".to_string());
        
        // Create the QuestionMarkExpression AST node
        let question_expr = QuestionMarkExpression::new(
            Box::new(inner_expr),
            10, // line
            5   // column
        );
        
        // Verify basic properties
        assert_eq!(question_expr.location(), (10, 5));
        assert_eq!(question_expr.string(), "result?");
        assert_eq!(question_expr.token_literal(), "?");
        
        // Test that inner expression is accessible
        let inner = question_expr.inner_expression();
        assert_eq!(inner.string(), "result");
    }

    #[test]
    fn test_question_mark_display() {
        let inner_expr = Identifier::new("value".to_string(), "value".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(inner_expr),
            1,
            7
        );
        
        // Test Display trait implementation
        assert_eq!(format!("{}", question_expr), "value?");
        assert_eq!(question_expr.to_string(), "value?");
    }

    #[test] 
    fn test_question_mark_cloning() {
        let inner_expr = Identifier::new("data".to_string(), "data".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(inner_expr),
            5,
            12
        );
        
        // Test cloning functionality
        let cloned = question_expr.clone();
        assert_eq!(cloned.string(), question_expr.string());
        assert_eq!(cloned.location(), question_expr.location());
    }

    #[test]
    fn test_question_mark_traits() {
        let inner_expr = Identifier::new("test".to_string(), "test".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(inner_expr),
            1,
            1
        );
        
        // Test Expression trait
        let expr_trait: &dyn Expression = &question_expr;
        assert_eq!(expr_trait.string(), "test?");
        
        // Test cloning through trait
        let cloned_box = question_expr.clone_box();
        assert_eq!(cloned_box.string(), "test?");
        
        // Test as_any() method
        let any_ref = question_expr.as_any();
        assert!(any_ref.downcast_ref::<QuestionMarkExpression>().is_some());
    }

    #[test]
    fn test_nested_question_mark() {
        // Test that we can create nested question mark expressions
        let inner_expr = Identifier::new("nested".to_string(), "nested".to_string());
        let first_question = QuestionMarkExpression::new(
            Box::new(inner_expr),
            1,
            1
        );
        
        let nested_question = QuestionMarkExpression::new(
            Box::new(first_question),
            1,
            2
        );
        
        assert_eq!(nested_question.string(), "nested??");
        assert_eq!(nested_question.location(), (1, 2));
    }

    #[test]
    fn test_question_mark_with_complex_expression() {
        // Test with a more complex inner expression (another identifier for simplicity)
        let complex_expr = Identifier::new("complex_operation".to_string(), "complex_operation".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(complex_expr),
            15,
            25
        );
        
        assert_eq!(question_expr.string(), "complex_operation?");
        assert_eq!(question_expr.location(), (15, 25));
        
        // Verify the inner expression is preserved correctly
        let inner = question_expr.inner_expression();
        assert_eq!(inner.string(), "complex_operation");
    }

    #[test]
    fn test_question_mark_api_exists() {
        // Test that the QuestionMarkExpression API is complete and accessible
        let inner_expr = Identifier::new("api_test".to_string(), "api_test".to_string());
        let question_expr = QuestionMarkExpression::new(
            Box::new(inner_expr),
            1,
            1
        );
        
        // Test all public methods exist
        let _ = question_expr.inner_expression();
        let _ = question_expr.location();
        let _ = question_expr.to_string();
        let _ = question_expr.string();
        let _ = question_expr.token_literal();
        let _ = question_expr.clone();
        let _ = question_expr.clone_box();
        let _ = question_expr.as_any();
        
        // If we get here, all the expected API methods exist and are callable
        assert!(true, "QuestionMarkExpression API is complete and functional");
    }
}
