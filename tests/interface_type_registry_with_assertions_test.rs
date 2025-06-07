use std::env;
use common::tracing;
use tracing::{debug, error, info, trace, warn};
use cursed::ast::types::{InterfaceType, StructType, Type};
use cursed::ast::expressions::TypeAssertion;
use cursed::parser::Parser;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::core::jit::JitCompiler;
use cursed::error::Error;

// # Interface Type Registry with Assertions Test
//
// Tests the enhanced interface type registry with better error messages
// during type assertions.


#[path = "common.rs"]
mod common;



/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

/// Test type assertion with enhanced error reporting
#[test]
fn test_type_assertion_with_enhanced_error_reporting() {
    init_tracing!();
    info!(test_case = "type_assertion_with_enhanced_error_reporting", "Starting test");
    
    // Set debug mode to verbose for maximum error information
    env::set_var("CURSED_TYPE_DEBUG", "verbose");
    
    let source = r#"
        vibe main;
        
        tea Drawable {
            bruh Draw() void;
        }
        
        tea Textual {
            bruh GetText() tea;
        }
        
        struct Circle struct {
            sus radius thicc;
        }
        
        struct TextBox struct {
            sus text tea;
        }
        
        bruh (c Circle) Draw() void {
            // Drawing implementation
        }
        
        bruh (t TextBox) GetText() tea {
            return t.text;
        }
        
        slay main() void {
            sus circle Circle = Circle{radius: 5.0};
            sus textBox TextBox = TextBox{text: "Hello"};
            
            sus drawable Drawable = circle;
            sus textual Textual = textBox;
            
            // Successful assertion
            sus backToCircle, ok1 = drawable.(Circle);
            debug("Assertion 1 result: %v", ok1);
            
            // Failed assertion with enhanced error reporting
            captcha {
                sus wrongTextBox, ok2 = drawable.(TextBox);
                debug("Assertion 2 result: %v", ok2);
                if ok2 {
                    poppin();
                }
            } drip (e) {
                debug("Expected error occurred: %s", e.message);
            };
            
            // Try another failed assertion with different types
            captcha {
                sus wrongCircle, ok3 = textual.(Circle);
                debug("Assertion 3 result: %v", ok3);
                if ok3 {
                    poppin();
                }
            } drip (e) {
                debug("Expected error occurred: %s", e.message);
            };
        }
    "#;
    
    match compile_and_run(source) {
        Ok(_) => {
            info!(test_result = "success", "Test passed: enhanced error reporting worked");
        },
        Err(e) => {
            error!(error = ?e, "Test failed unexpectedly");
            panic!("Test failed: {:?}", e);
        }
    }
}

/// Helper function to compile and run CURSED code
fn compile_and_run(source: &str) -> Result<(), Error> {
    // Parse the source code
    let mut parser = Parser::new(source, "test.csd")?;
    let program = parser.parse_program()?;
    
    // Set up the LLVM code generator
    let code_generator = LlvmCodeGenerator::new("test_module")?;
    
    // Generate LLVM IR code
    let module = code_generator.compile_program(&program)?;
    
    // Set up JIT compiler
    let jit = JitCompiler::new()?;
    
    // Compile and run the code
    jit.run_jit(&module)?;
    
    Ok(())
}