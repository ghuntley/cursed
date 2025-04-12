//! Tests for string-based switch statements in the LLVM codegen
//!
//! These tests verify string comparison capabilities needed for implementing
//! switch statements with string case values in the CURSED language.

use cursed::ast::{Expression, StringLiteral, Statement, Node};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::any::Any;
use std::path::PathBuf;

// StringLiteral needs to be reimplemented for our tests
#[derive(Debug)]
struct TestStringLiteral {
    pub value: String,
}

impl Expression for TestStringLiteral {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Implement Node trait for StringLiteral
impl Node for TestStringLiteral {
    fn token_literal(&self) -> String {
        "string_literal".to_string()
    }

    fn string(&self) -> String {
        format!("{}", self.value)
    }
}

#[test]
fn test_string_comparison() {
    // Create a new LLVM context and module for testing
    let context = inkwell::context::Context::create();
    let module_name = "string_comparison_test";
    let file_path = PathBuf::from("test_module.csd");
    let mut code_generator = LlvmCodeGenerator::new(&context, module_name, file_path);

    // Create a function to test string comparison
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let bool_type = context.bool_type();
    let fn_type = bool_type.fn_type(&[], false);
    let function = code_generator
        .module()
        .add_function("test_function", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    code_generator.builder().position_at_end(entry_block);

    // Create string constants
    let hello_str = code_generator.create_string_constant("hello").unwrap();
    let world_str = code_generator.create_string_constant("world").unwrap();
    let hello2_str = code_generator.create_string_constant("hello").unwrap();

    // Generate string comparisons
    let cmp1 = code_generator
        .generate_string_comparison(hello_str, world_str)
        .unwrap(); // should be false
    let cmp2 = code_generator
        .generate_string_comparison(hello_str, hello2_str)
        .unwrap(); // should be true

    // Build a return value that combines the results
    // If our string comparison works correctly, this will return false
    let and_result = code_generator
        .builder()
        .build_and(cmp1, cmp2, "and_result")
        .unwrap();
    code_generator
        .builder()
        .build_return(Some(&and_result))
        .unwrap();

    // Verify module - this ensures our LLVM IR is well-formed
    assert!(code_generator.module().verify().is_ok());

    // Get the generated IR code and make sure it contains the expected function calls
    let ir_code = code_generator.module().print_to_string().to_string();
    assert!(
        ir_code.contains("@strcmp"),
        "IR should contain strcmp function"
    );
    assert!(
        ir_code.contains("@string_0"),
        "IR should contain string constants"
    );
    assert!(
        ir_code.contains("@string_1"),
        "IR should contain string constants"
    );
    assert!(
        ir_code.contains("@string_2"),
        "IR should contain string constants"
    );
}

#[test]
fn test_string_literal_evaluation() {
    // Create a new LLVM context and module for testing
    let context = inkwell::context::Context::create();
    let module_name = "string_eval_test";
    let file_path = PathBuf::from("test_module.csd");
    let mut code_generator = LlvmCodeGenerator::new(&context, module_name, file_path);

    // Create a function to test string evaluation
    let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
    let fn_type = i8_ptr_type.fn_type(&[], false);
    let function = code_generator
        .module()
        .add_function("test_function", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    code_generator.builder().position_at_end(entry_block);

    // Create a string literal expression
    let string_literal = TestStringLiteral {
        value: "hello world".to_string(),
    };

    // Evaluate the string expression
    let str_ptr = code_generator
        .evaluate_string_expr(&string_literal)
        .unwrap();

    // Build a return value
    code_generator.builder().build_return(Some(&str_ptr)).unwrap();

    // Verify module - this ensures our LLVM IR is well-formed
    assert!(code_generator.module().verify().is_ok());

    // Get the generated IR code and make sure it contains the expected string content
    let ir_code = code_generator.module().print_to_string().to_string();
    assert!(
        ir_code.contains("hello world"),
        "IR should contain the string literal content"
    );
}

// Create a dummy SwitchStatement for testing
struct DummyBlockStatement {
    statements: Vec<Box<dyn Statement>>,
}

impl std::fmt::Debug for DummyBlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DummyBlockStatement with {} statements", self.statements.len())
    }
}

impl Statement for DummyBlockStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for DummyBlockStatement {
    fn token_literal(&self) -> String {
        "{".to_string()
    }
    
    fn string(&self) -> String {
        "{ ... }".to_string()
    }
}

struct DummyCaseStatement {
    expressions: Vec<Box<dyn Expression>>,
    body: DummyBlockStatement,
}

impl std::fmt::Debug for DummyCaseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DummyCaseStatement with {} expressions", self.expressions.len())
    }
}

#[derive(Debug)]
struct DummySwitchStatement {
    cases: Vec<DummyCaseStatement>,
    default: Option<DummyBlockStatement>,
}

impl Statement for DummySwitchStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for DummySwitchStatement {
    fn token_literal(&self) -> String {
        "vibe_check".to_string()
    }
    
    fn string(&self) -> String {
        "vibe_check {...}".to_string()
    }
}

#[derive(Debug)]
struct DummyBreakStatement {}

impl Statement for DummyBreakStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Node for DummyBreakStatement {
    fn token_literal(&self) -> String {
        "ghosted".to_string()
    }
    
    fn string(&self) -> String {
        "ghosted;".to_string()
    }
}

#[test]
fn test_string_switch_compilation() {
    // Create a function for testing our string switch compilation
    let context = inkwell::context::Context::create();
    let module_name = "test_string_switch";
    let file_path = PathBuf::from("test_module.csd");
    let mut code_generator = LlvmCodeGenerator::new(&context, module_name, file_path);
    
    // Create a function to hold our switch statement
    let fn_type = context.void_type().fn_type(&[], false);
    let function = code_generator.module().add_function("test_switch", fn_type, None);
    let entry_block = context.append_basic_block(function, "entry");
    code_generator.builder().position_at_end(entry_block);
    
    // Create a string parameter to switch on
    let day_str = code_generator.create_string_constant("Monday").unwrap();
    
    // Create dummy case statements
    let monday_lit = TestStringLiteral { value: "Monday".to_string() };
    let monday_expr: Box<dyn Expression> = Box::new(monday_lit);
    
    let tuesday_lit = TestStringLiteral { value: "Tuesday".to_string() };
    let tuesday_expr: Box<dyn Expression> = Box::new(tuesday_lit);
    
    let break_stmt: Box<dyn Statement> = Box::new(DummyBreakStatement {});
    
    // Create a simple switch statement with one case and a default
    let case1 = DummyCaseStatement {
        expressions: vec![monday_expr],
        body: DummyBlockStatement {
            statements: vec![Box::new(DummyBreakStatement {})],
        },
    };
    
    let case2 = DummyCaseStatement {
        expressions: vec![tuesday_expr],
        body: DummyBlockStatement {
            statements: vec![break_stmt],
        },
    };
    
    let default_block = DummyBlockStatement {
        statements: vec![],
    };
    
    let switch_stmt = DummySwitchStatement {
        cases: vec![case1, case2],
        default: Some(default_block),
    };
    
    // Attempt to compile the string switch
    let result = code_generator.compile_string_switch_statement(&switch_stmt, day_str);
    assert!(result.is_ok(), "Failed to compile string switch: {:?}", result.err());
    
    // Generate a proper return to satisfy the function
    code_generator.builder().build_return(None).unwrap();
    
    // Verify the module to ensure the IR is valid
    assert!(code_generator.module().verify().is_ok(), "Invalid LLVM module");
    
    // Get the generated IR code
    let ir_code = code_generator.module().print_to_string().to_string();
    
    // Verify that strcmp is used in the IR
    assert!(ir_code.contains("@strcmp"), "strcmp not found in IR");
    
    // Verify that we have basic blocks for cases and default
    assert!(ir_code.contains("switch.case"), "Case blocks not found in IR");
    assert!(ir_code.contains("switch.default"), "Default block not found in IR");
}
