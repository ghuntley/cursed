#[cfg(test)]
mod tests {
    use crate::object::Object;
    use crate::compiler::Compiler;
    use crate::vm::VM;
    use crate::ast::*;
    use crate::lexer::*;
    use crate::parser::*;
    use std::rc::Rc;

    fn test_vm(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        
        let mut compiler = Compiler::new();
        let bytecode = compiler.compile(&program).unwrap();
        
        let mut vm = VM::new(bytecode);
        vm.run().unwrap();
        
        match vm.last_popped_stack_elem() {
            Some(obj) => (*obj).clone(),
            None => Object::Null,
        }
    }

    #[test]
    fn test_variadic_function_creation() {
        // Create a variadic function that accepts a fixed first parameter and a rest parameter
        let input = r#"
        let sum = fn(first, ...rest) {
            let total = first;
            let i = 0;
            while (i < len(rest)) {
                total = total + rest[i];
                i = i + 1;
            }
            return total;
        };
        sum(1, 2, 3, 4, 5);
        "#;

        let result = test_vm(input);
        match result {
            Object::Integer(value) => assert_eq!(value, 15), // 1 + 2 + 3 + 4 + 5 = 15
            _ => panic!("Expected integer, got: {:?}", result),
        }
    }

    #[test]
    fn test_variadic_function_with_no_variadic_args() {
        // Test a variadic function with just the fixed parameter and no variadic arguments
        let input = r#"
        let greet = fn(name, ...titles) {
            if (len(titles) > 0) {
                return "Hello, " + name + " the " + titles[0];
            }
            return "Hello, " + name;
        };
        greet("John");
        "#;

        let result = test_vm(input);
        match result {
            Object::String(value) => assert_eq!(value, "Hello, John"),
            _ => panic!("Expected string, got: {:?}", result),
        }
    }

    #[test]
    fn test_variadic_function_with_variadic_args() {
        // Test a variadic function with both fixed and variadic arguments
        let input = r#"
        let greet = fn(name, ...titles) {
            if (len(titles) > 0) {
                return "Hello, " + name + " the " + titles[0];
            }
            return "Hello, " + name;
        };
        greet("John", "Magnificent");
        "#;

        let result = test_vm(input);
        match result {
            Object::String(value) => assert_eq!(value, "Hello, John the Magnificent"),
            _ => panic!("Expected string, got: {:?}", result),
        }
    }

    #[test]
    fn test_multiple_variadic_function_calls() {
        // Test multiple calls to a variadic function with different numbers of arguments
        let input = r#"
        let join = fn(separator, ...words) {
            let result = "";
            let i = 0;
            while (i < len(words)) {
                result = result + words[i];
                if (i < len(words) - 1) {
                    result = result + separator;
                }
                i = i + 1;
            }
            return result;
        };
        let first = join(", ", "one", "two", "three");
        let second = join("-", "a", "b", "c", "d", "e");
        first + " and " + second;
        "#;

        let result = test_vm(input);
        match result {
            Object::String(value) => assert_eq!(value, "one, two, three and a-b-c-d-e"),
            _ => panic!("Expected string, got: {:?}", result),
        }
    }
} 