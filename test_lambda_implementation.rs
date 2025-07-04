// Test our lambda implementation directly
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cursed::codegen::llvm::expression_compiler::ExpressionCompiler;
    use cursed::ast::{Expression, LambdaExpression};
    
    // Create a simple lambda expression: || { 42 }
    let lambda_expr = LambdaExpression {
        parameters: vec![],
        body: Box::new(Expression::Integer(42)),
    };
    
    let mut compiler = ExpressionCompiler::new();
    
    // Compile the lambda expression
    let result = compiler.compile_expression(&Expression::Lambda(lambda_expr))?;
    
    println!("Lambda compiled successfully! Result register: {}", result);
    println!("Generated IR:");
    println!("{}", compiler.get_ir());
    
    println!("Lambda functions:");
    for lambda_func in compiler.get_lambda_functions() {
        println!("{}", lambda_func);
    }
    
    Ok(())
}
