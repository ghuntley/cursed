#[cfg(test)]
mod tests {
    use inkwell::context::Context;
    use std::path::PathBuf;
    use crate::ast::{Program, Statement, Expression};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use super::llvm::LlvmCodeGenerator;

    fn parse_program(input: &str) -> Program {
        let lexer = Lexer::new(input).unwrap();
        let mut parser = Parser::new(lexer).unwrap();
        parser.parse_program().unwrap()
    }

    #[test]
    fn test_basic_integer_literal() {
        let input = "42;";
        let program = parse_program(input);
        
        let context = Context::create();
        let mut codegen = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));
        
        let result = codegen.compile(&program);
        assert!(result.is_ok(), "Failed to compile integer literal: {:?}", result.err());
        
        let ir = codegen.module().print_to_string().to_string();
        assert!(ir.contains("i64 42"), "Generated IR doesn't contain expected integer constant");
    }

    #[test]
    fn test_basic_arithmetic() {
        let input = "5 + 3 * 2;";
        let program = parse_program(input);
        
        let context = Context::create();
        let mut codegen = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));
        
        let result = codegen.compile(&program);
        assert!(result.is_ok(), "Failed to compile arithmetic expression: {:?}", result.err());
        
        let ir = codegen.module().print_to_string().to_string();
        assert!(ir.contains("add"), "Generated IR doesn't contain addition operation");
        assert!(ir.contains("mul"), "Generated IR doesn't contain multiplication operation");
    }

    #[test]
    fn test_variable_declaration() {
        let input = "sus x = 42;";
        let program = parse_program(input);
        
        let context = Context::create();
        let mut codegen = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));
        
        let result = codegen.compile(&program);
        assert!(result.is_ok(), "Failed to compile variable declaration: {:?}", result.err());
        
        let ir = codegen.module().print_to_string().to_string();
        assert!(ir.contains("alloca"), "Generated IR doesn't contain stack allocation");
        assert!(ir.contains("store"), "Generated IR doesn't contain store operation");
    }

    #[test]
    fn test_if_statement() {
        let input = "lowkey 5 > 3 { sus x = 10; } highkey { sus x = 20; }";
        let program = parse_program(input);
        
        let context = Context::create();
        let mut codegen = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));
        
        let result = codegen.compile(&program);
        assert!(result.is_ok(), "Failed to compile if statement: {:?}", result.err());
        
        let ir = codegen.module().print_to_string().to_string();
        assert!(ir.contains("icmp"), "Generated IR doesn't contain comparison operation");
        assert!(ir.contains("br"), "Generated IR doesn't contain branch operation");
    }

    #[test]
    fn test_function_definition() {
        let input = "slay add(a, b) { yolo a + b; }";
        let program = parse_program(input);
        
        let context = Context::create();
        let mut codegen = LlvmCodeGenerator::new(&context, "test", PathBuf::from("test.csd"));
        
        let result = codegen.compile(&program);
        assert!(result.is_ok(), "Failed to compile function definition: {:?}", result.err());
        
        let ir = codegen.module().print_to_string().to_string();
        assert!(ir.contains("define"), "Generated IR doesn't contain function definition");
        assert!(ir.contains("ret"), "Generated IR doesn't contain return instruction");
    }
} 