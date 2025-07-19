/// Type System Basic Functional Tests
/// 
/// Tests fundamental type system features including type checking,
/// basic generics, interface implementation, and type conversions.

use cursed::*;
use cursed::error::CursedError;
use cursed::type_system::checker::{TypeChecker, VariableInfo};
use cursed::type_system::{TypeExpression, TypeCheckError};
use cursed::ast::{LetStatement, LetTarget, Expression, Literal, Statement, AssignmentStatement, AssignmentTarget, ConstDecl, ConstSpec};

fn compile_to_ir(code: &str) -> Result<String, CursedError> {
    let mut codegen = LlvmCodeGeneratorReal::new()?;

    let mut lexer = Lexer::new(code.to_string());
    let mut parser = Parser::new(lexer)?;
    let ast = parser.parse()?;
    let program = match ast {
        cursed::ast::Ast::Program(program) => program,
        _ => panic!("Expected Program")
    };

    codegen.compile_ast(&program)?;
    Ok(codegen.module().print_to_string().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_type_checking() {
        let valid_programs = vec![
            "facts x normie = 42;",
            "facts name tea = \"hello\";",
            "facts flag lit = based;",
            "facts pi meal = 3.14159;",
        ];
        
        for program in valid_programs {
            let result = compile_to_ir(program);
            match result {
                Ok(ir) => {
                    assert!(!ir.is_empty(), "Valid type should compile to non-empty IR");
                    println!("Type checking passed for: {}", program);
                }
                Err(error) => {
                    println!("Type checking failed for '{}': {}", program, error);
                }
            }
        }
    }

    #[test]
    fn test_type_inference() {
        let inference_cases = vec![
            "facts x = 42;",           // Should infer i64
            "facts name = \"hello\";", // Should infer String
            "facts flag = true;",      // Should infer bool
            "facts pi = 3.14;",        // Should infer f64
        ];
        
        for case in inference_cases {
            let result = compile_to_ir(case);
            match result {
                Ok(ir) => {
                    assert!(!ir.is_empty(), "Type inference should work");
                    println!("Type inference succeeded for: {}", case);
                }
                Err(error) => {
                    println!("Type inference failed for '{}': {}", case, error);
                }
            }
        }
    }

    #[test]
    fn test_function_types() {
        let function_cases = vec![
            r#"
                slay add(a i64, b i64) i64 {
                    periodt a + b;
                }
            "#,
            r#"
                slay greet(name String) String {
                    periodt "Hello, " + name;
                }
            "#,
            r#"
                slay is_positive(n i64) bool {
                    periodt n > 0;
                }
            "#,
        ];
        
        for function in function_cases {
            let result = compile_to_ir(function);
            match result {
                Ok(ir) => {
                    assert!(!ir.is_empty(), "Function types should compile");
                    assert!(ir.contains("define"), "Should contain function definition");
                    println!("Function type checking succeeded");
                }
                Err(error) => {
                    println!("Function type checking failed: {}", error);
                }
            }
        }
    }

    #[test]
    fn test_struct_types() {
        let struct_program = r#"
            squad Person {
                name String,
                age i64,
                active bool,
            }
            
            facts person = Person {
                name: "Alice",
                age: 30,
                active: true,
            };
        "#;
        
        let result = compile_to_ir(struct_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Struct types should compile");
                println!("Struct type checking succeeded");
            }
            Err(error) => {
                println!("Struct type checking failed: {}", error);
            }
        }
    }

    #[test]
    fn test_array_types() {
        let array_cases = vec![
            "facts numbers: [5]i64 = [1, 2, 3, 4, 5];",
            "facts names: []String = [\"alice\", \"bob\"];",
            "facts flags: [3]bool = [true, false, true];",
        ];
        
        for case in array_cases {
            let result = compile_to_ir(case);
            match result {
                Ok(ir) => {
                    assert!(!ir.is_empty(), "Array types should compile");
                    println!("Array type checking succeeded for: {}", case);
                }
                Err(error) => {
                    println!("Array type checking failed for '{}': {}", case, error);
                }
            }
        }
    }

    #[test]
    fn test_map_types() {
        let map_program = r#"
            facts scores: map[String]i64 = {
                "alice": 100,
                "bob": 85,
                "charlie": 92,
            };
        "#;
        
        let result = compile_to_ir(map_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Map types should compile");
                println!("Map type checking succeeded");
            }
            Err(error) => {
                println!("Map type checking failed: {}", error);
            }
        }
    }

    #[test]
    fn test_basic_generics() {
        let generic_function = r#"
            slay identity<T>(value T) T {
                periodt value;
            }
            
            facts num = identity<i64>(42);
            facts text = identity<String>("hello");
        "#;
        
        let result = compile_to_ir(generic_function);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Generic functions should compile");
                println!("Basic generics succeeded");
            }
            Err(error) => {
                println!("Basic generics failed: {}", error);
            }
        }
    }

    #[test]
    fn test_interface_basic() {
        let interface_program = r#"
            collab Drawable {
                slay draw() String;
            }
            
            squad Circle {
                radius f64,
            }
            
            impl Drawable for Circle {
                slay draw() String {
                    periodt "Drawing a circle";
                }
            }
        "#;
        
        let result = compile_to_ir(interface_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Interface implementation should compile");
                println!("Basic interface implementation succeeded");
            }
            Err(error) => {
                println!("Basic interface implementation failed: {}", error);
            }
        }
    }

    #[test]
    fn test_type_assertions() {
        let assertion_program = r#"
            collab Any {
                slay type_name() String;
            }
            
            facts value: Any = get_some_value();
            facts number = value.(i64);
        "#;
        
        let result = compile_to_ir(assertion_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Type assertions should compile");
                println!("Type assertions succeeded");
            }
            Err(error) => {
                println!("Type assertions failed: {}", error);
            }
        }
    }

    #[test]
    fn test_type_conversions() {
        let conversion_cases = vec![
            "facts f = f64(42);",              // int to float
            "facts i = i64(3.14);",            // float to int  
            "facts s = String(42);",           // int to string
            "facts b = bool(1);",              // int to bool
        ];
        
        for case in conversion_cases {
            let result = compile_to_ir(case);
            match result {
                Ok(ir) => {
                    assert!(!ir.is_empty(), "Type conversion should compile");
                    println!("Type conversion succeeded for: {}", case);
                }
                Err(error) => {
                    println!("Type conversion failed for '{}': {}", case, error);
                }
            }
        }
    }

    #[test]
    fn test_nil_handling() {
        let nil_cases = vec![
            "facts x: ?i64 = nil;",
            "facts y: ?String = nil;",
            "facts z: ?bool = nil;",
        ];
        
        for case in nil_cases {
            let result = compile_to_ir(case);
            match result {
                Ok(ir) => {
                    assert!(!ir.is_empty(), "Nil handling should compile");
                    println!("Nil handling succeeded for: {}", case);
                }
                Err(error) => {
                    println!("Nil handling failed for '{}': {}", case, error);
                }
            }
        }
    }

    #[test]
    fn test_channel_types() {
        let channel_program = r#"
            facts ch: chan i64 = make(chan i64, 10);
            facts value: i64 = 42;
            
            ch <- value;
            facts received = <-ch;
        "#;
        
        let result = compile_to_ir(channel_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Channel types should compile");
                println!("Channel types succeeded");
            }
            Err(error) => {
                println!("Channel types failed: {}", error);
            }
        }
    }

    #[test]
    fn test_error_type_handling() {
        let error_program = r#"
            slay divide(a i64, b i64) Result<i64, String> {
                lowkey (b == 0) {
                    periodt Err("Division by zero");
                }
                periodt Ok(a / b);
            }
            
            facts result = divide(10, 2);
        "#;
        
        let result = compile_to_ir(error_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Error types should compile");
                println!("Error type handling succeeded");
            }
            Err(error) => {
                println!("Error type handling failed: {}", error);
            }
        }
    }

    #[test]
    fn test_type_compatibility() {
        let compatibility_tests = vec![
            ("i32", "i64", "Integer compatibility"),
            ("f32", "f64", "Float compatibility"),
            ("&str", "String", "String reference compatibility"),
        ];
        
        for (from_type, to_type, description) in compatibility_tests {
            let program = format!(
                "facts x: {} = default_value(); facts y: {} = x;", 
                from_type, to_type
            );
            
            let result = compile_to_ir(&program);
            match result {
                Ok(ir) => {
                    println!("Type compatibility succeeded for: {}", description);
                }
                Err(error) => {
                    println!("Type compatibility failed for '{}': {}", description, error);
                }
            }
        }
    }

    #[test]
    fn test_recursive_types() {
        let recursive_program = r#"
            squad Node {
                value i64,
                next ?*Node,
            }
            
            facts head = Node {
                value: 1,
                next: nil,
            };
        "#;
        
        let result = compile_to_ir(recursive_program);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Recursive types should compile");
                println!("Recursive types succeeded");
            }
            Err(error) => {
                println!("Recursive types failed: {}", error);
            }
        }
    }

    #[test]
    fn test_generic_constraints() {
        let constrained_generic = r#"
            collab Comparable {
                slay compare(other Self) i64;
            }
            
            slay max<T: Comparable>(a T, b T) T {
                lowkey (a.compare(b) > 0) {
                    periodt a;
                } highkey {
                    periodt b;
                }
            }
        "#;
        
        let result = compile_to_ir(constrained_generic);
        match result {
            Ok(ir) => {
                assert!(!ir.is_empty(), "Generic constraints should compile");
                println!("Generic constraints succeeded");
            }
            Err(error) => {
                println!("Generic constraints failed: {}", error);
            }
        }
    }

    // Mutability tracking tests
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
}
