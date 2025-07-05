/// Type Inference Binary Expression Tests
/// 
/// Tests type inference for binary expressions with untyped function parameters

use cursed::*;
use cursed::error::CursedError;

fn compile_to_ir(code: &str) -> Result<String, CursedError> {
    let mut codegen = LlvmCodeGeneratorReal::new()?;

    let mut lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer)?;
    let ast = parser.parse()?;

    codegen.compile_ast(&ast)?;
    Ok(codegen.module().print_to_string().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_arithmetic_with_unknown_types() {
        let test_program = r#"
            slay add(a, b) {
                yolo a + b;
            }
            
            facts result = add(5, 3);
        "#;
        
        let result = compile_to_ir(test_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Binary arithmetic with unknown types should compile");
                println!("Binary arithmetic with unknown types succeeded");
                println!("Generated IR: {}", ir);
            }
            Err(error) => {
                eprintln!("Binary arithmetic with unknown types failed: {}", error);
                println!("Error details: {:?}", error);
                panic!("Binary arithmetic with unknown types failed: {}", error);
            }
        }
    }

    #[test]
    fn test_multiple_arithmetic_operations() {
        let test_program = r#"
            slay calculate(x, y, z) {
                facts sum = x + y;
                facts product = sum * z;
                yolo product - 10;
            }
            
            facts result = calculate(2, 3, 4);
        "#;
        
        let result = compile_to_ir(test_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Multiple arithmetic operations should compile");
                println!("Multiple arithmetic operations succeeded");
            }
            Err(error) => {
                panic!("Multiple arithmetic operations failed: {}", error);
            }
        }
    }

    #[test]
    fn test_mixed_typed_untyped_parameters() {
        let test_program = r#"
            slay mixed_add(a int, b) {
                yolo a + b;
            }
            
            facts result = mixed_add(10, 5);
        "#;
        
        let result = compile_to_ir(test_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Mixed typed/untyped parameters should compile");
                println!("Mixed typed/untyped parameters succeeded");
            }
            Err(error) => {
                panic!("Mixed typed/untyped parameters failed: {}", error);
            }
        }
    }

    #[test]
    fn test_arithmetic_with_literals() {
        let test_program = r#"
            slay add_literal(a) {
                yolo a + 42;
            }
            
            facts result = add_literal(8);
        "#;
        
        let result = compile_to_ir(test_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Arithmetic with literals should compile");
                println!("Arithmetic with literals succeeded");
            }
            Err(error) => {
                panic!("Arithmetic with literals failed: {}", error);
            }
        }
    }

    #[test]
    fn test_subtraction_multiplication_division() {
        let test_program = r#"
            slay math_operations(a, b) {
                facts sub = a - b;
                facts mul = a * b;
                facts div = a / b;
                yolo sub + mul + div;
            }
            
            facts result = math_operations(10, 2);
        "#;
        
        let result = compile_to_ir(test_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "All math operations should compile");
                println!("All math operations succeeded");
            }
            Err(error) => {
                panic!("All math operations failed: {}", error);
            }
        }
    }

    #[test]
    fn test_nested_arithmetic_expressions() {
        let test_program = r#"
            slay nested_calc(a, b, c) {
                yolo (a + b) * (c - a);
            }
            
            facts result = nested_calc(3, 4, 10);
        "#;
        
        let result = compile_to_ir(test_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Nested arithmetic expressions should compile");
                println!("Nested arithmetic expressions succeeded");
            }
            Err(error) => {
                panic!("Nested arithmetic expressions failed: {}", error);
            }
        }
    }

    #[test]
    fn test_comparison_with_unknown_types() {
        let test_program = r#"
            slay compare(a, b) {
                yolo a < b;
            }
            
            facts result = compare(5, 10);
        "#;
        
        let result = compile_to_ir(test_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Comparison with unknown types should compile");
                println!("Comparison with unknown types succeeded");
            }
            Err(error) => {
                panic!("Comparison with unknown types failed: {}", error);
            }
        }
    }

    #[test]
    fn test_variable_arithmetic() {
        let test_program = r#"
            slay variable_math(x, y) {
                facts a = x + 1;
                facts b = y * 2;
                yolo a + b;
            }
            
            facts result = variable_math(5, 3);
        "#;
        
        let result = compile_to_ir(test_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Variable arithmetic should compile");
                println!("Variable arithmetic succeeded");
            }
            Err(error) => {
                panic!("Variable arithmetic failed: {}", error);
            }
        }
    }
}
