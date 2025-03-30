#[cfg(test)]
mod integration_tests {
    use crate::ast::{Program, ExpressionStatement, CallExpression, IntegerLiteral, Identifier};
    use crate::codegen::llvm::LlvmCodeGenerator;
    use crate::lexer::Token;
    use inkwell::context::Context;

    #[test]
    fn test_compile_puts_call() {
        let context = Context::create();
        let mut codegen = LlvmCodeGenerator::new(&context, "test_puts");
        
        // Create a puts call with the argument 42
        let mut program = Program::default();
        
        // Create the call expression with enum tokens
        let call_expr = CallExpression {
            token: Token::LParen,
            function: Box::new(Identifier {
                token: "IDENT".to_string(),
                value: "puts".to_string(),
            }),
            arguments: vec![
                Box::new(IntegerLiteral {
                    token: "INT".to_string(),
                    value: 42,
                })
            ],
        };
        
        // Add the call as an expression statement
        let expr_stmt = ExpressionStatement {
            token: ";".to_string(), 
            expression: Some(Box::new(call_expr)),
        };
        
        program.statements.push(Box::new(expr_stmt));
        
        // Compile the program
        let result = codegen.compile_program(&program);
        assert!(result.is_ok(), "Failed to compile program");
        
        // Convert the LLVM IR to a string for verification
        let ir = codegen.module().print_to_string().to_string();
        
        // Verify the IR contains what we expect
        assert!(ir.contains("puts"), "IR should contain a call to puts");
        assert!(ir.contains("printf"), "IR should contain a call to printf");
        assert!(ir.contains("%lld"), "IR should contain the integer format specifier");
        
        // Verify the module is valid
        assert!(codegen.module().verify().is_ok(), "Module verification failed");
    }
} 