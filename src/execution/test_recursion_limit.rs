#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::execution::execution_context::ExecutionContext;
    
    #[test]
    fn test_recursion_depth_limit() {
        let mut engine = CursedExecutionEngine::new_no_jit().expect("Failed to create engine");
        let mut context = ExecutionContext::new();
        
        // Create a deeply nested binary expression (200 levels of nesting)
        let mut expr = Expression::Integer(1);
        for _ in 0..200 {
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: "+".to_string(),
                right: Box::new(Expression::Integer(1)),
            });
        }
        
        // This should hit our recursion limit of 100 and return an error
        let result = engine.evaluate_expression(&expr, &mut context);
        
        match result {
            Err(e) => {
                let error_msg = e.to_string();
                assert!(error_msg.contains("Maximum recursion depth exceeded"));
                println!("SUCCESS: Recursion limit triggered as expected: {}", error_msg);
            }
            Ok(_) => {
                panic!("Expected recursion limit error, but evaluation succeeded");
            }
        }
    }

    #[test]
    fn test_simple_expression_still_works() {
        let mut engine = CursedExecutionEngine::new_no_jit().expect("Failed to create engine");
        let mut context = ExecutionContext::new();
        
        // Create a simple expression that should work
        let expr = Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Integer(1)),
            operator: "+".to_string(),
            right: Box::new(Expression::Integer(2)),
        });
        
        // This should work fine
        let result = engine.evaluate_expression(&expr, &mut context);
        
        match result {
            Ok(CursedValue::Integer(3)) => {
                println!("SUCCESS: Simple expression works correctly");
            }
            Ok(other) => {
                panic!("Expected integer 3, got: {:?}", other);
            }
            Err(e) => {
                panic!("Expected success, got error: {}", e);
            }
        }
    }
}
