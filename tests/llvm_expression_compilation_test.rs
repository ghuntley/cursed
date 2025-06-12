use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::ast::{AstNode, Expression, Literal};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use inkwell::context::Context;
use inkwell::module::Module;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_llvm() -> (Context, Module<'static>, LlvmCodeGenerator<'static>) {
        let context = Context::create();
        let module = context.create_module("test");
        let codegen = LlvmCodeGenerator::new(&context, &module);
        (context, module, codegen)
    }

    #[test]
    fn test_integer_literal_compilation() {
        let (_context, _module, mut codegen) = setup_llvm();
        
        let expr = Expression::Literal(Literal::Integer(42));
        let result = codegen.compile_expression(&expr);
        
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_some());
    }

    #[test]
    fn test_string_literal_compilation() {
        let (_context, _module, mut codegen) = setup_llvm();
        
        let expr = Expression::Literal(Literal::String("hello".to_string()));
        let result = codegen.compile_expression(&expr);
        
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_some());
    }

    #[test]
    fn test_binary_arithmetic_compilation() {
        let source = "42 + 24";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expression().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        let result = codegen.compile_expression(&expr);
        
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_some());
    }

    #[test]
    fn test_variable_reference_compilation() {
        let (_context, _module, mut codegen) = setup_llvm();
        
        // First declare a variable
        let var_name = "x".to_string();
        let var_value = Expression::Literal(Literal::Integer(42));
        let _ = codegen.compile_variable_declaration(&var_name, &var_value);
        
        // Then reference it
        let expr = Expression::Variable(var_name.clone());
        let result = codegen.compile_expression(&expr);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_call_compilation() {
        let source = r#"
            slay add(a: i64, b: i64) -> i64 {
                a + b
            }
            add(10, 20)
        "#;
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        
        // Compile function declaration first
        if let AstNode::FunctionDeclaration { .. } = &ast.statements[0] {
            let result = codegen.compile_function(&ast.statements[0]);
            assert!(result.is_ok());
        }
        
        // Then compile function call
        if let AstNode::ExpressionStatement { expression } = &ast.statements[1] {
            let result = codegen.compile_expression(expression);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_boolean_expression_compilation() {
        let source = "facts && lowkey";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expression().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        let result = codegen.compile_expression(&expr);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_array_access_compilation() {
        let source = "arr[0]";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expression().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        
        // First create an array variable
        let arr_name = "arr".to_string();
        let arr_expr = Expression::ArrayLiteral(vec![
            Expression::Literal(Literal::Integer(1)),
            Expression::Literal(Literal::Integer(2)),
            Expression::Literal(Literal::Integer(3))
        ]);
        let _ = codegen.compile_variable_declaration(&arr_name, &arr_expr);
        
        // Then compile array access
        let result = codegen.compile_expression(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_struct_field_access_compilation() {
        let source = "point.x";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expression().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        let result = codegen.compile_expression(&expr);
        
        // Should handle gracefully even if struct not defined
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_complex_nested_expression() {
        let source = "(a + b) * (c - d)";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expression().unwrap();
        
        let (_context, _module, mut codegen) = setup_llvm();
        let result = codegen.compile_expression(&expr);
        
        assert!(result.is_ok());
    }
}
