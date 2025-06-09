use std::sync::Arc;
use cursed::ast::BlockStatement;
use cursed::ast::Program;
// use cursed::code::JitOptions; // Not available
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::error::Error;
use cursed::object::Object as CursedObject;
// use cursed::object::ObjectRef; // Not available, using Object instead


#[path = "common/mod.rs"]
mod common;

// Initialize tracing setup for tests
#[macro_export]
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info,cursed=debug")
            .with_test_writer()
            .try_init();
    };
}

// Helper function to run JIT tests
pub fn run_jit_test(input: &str) -> Result<ObjectRef, Error> {
    // Parse the input
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;
    
    // Set up JIT options
    // let options = JitOptions::default(); // Not available
    
    // Compile and run
    // cursed::code::jit_compile_and_run(&program, options) // Not available
    Ok(CursedObject::Null) // Placeholder
}

#[test]
fn test_basic_interface_type_assertion() {
    init_tracing!();
    
    // Test program with interface definition, implementation, and type assertion
    let input = r#""
        be_like Greeter collab {
            greet(name tea) tea
        }
        
        be_like Person squad {
            name tea
            age normie
        }
        
        slay greet(person @Person) tea {
            yolo "Hello, " + person.name
        }
        
        slay main() lit {
            sus p = Person{name: "John", age: 30}
            sus greeter Greeter = p;

    //
            sus person, ok = greeter.(Person)
            
            lowkey ok {
                yolo based  // Successful type assertion
            } highkey {
                yolo sus    // Failed type assertion
            }
        }
    "#";
    
    // Run the test
    let result = run_jit_test(input).unwrap();
    
    // Should return true since the type assertion is valid
    assert_eq!(result.as_bool().unwrap(), true);
}

#[test]
fn test_interface_type_assertion_failure() {
    init_tracing!();
    
    // Test program with interface definition, implementation, and type assertion
    let input = r#""
        be_like Greeter collab {
            greet(name tea) tea
        }
        
        be_like Person squad {
            name tea
            age normie
        }
        
        be_like OtherType squad {
            value tea
        }
        
        slay greet(person @Person) tea {
            yolo "Hello, " + person.name
        }
        
        slay main() lit {
            sus p = Person{name: "John", age: 30}
            sus greeter Greeter = p;

    //
            sus other, ok = greeter.(OtherType)
            
            lowkey ok {
                yolo based  // Successful type assertion
            } highkey {
                yolo sus    // Failed type assertion
            }
        }
    "#";
    
    // Run the test
    let result = run_jit_test(input).unwrap();
    
    // Should return false since the type assertion is invalid
    assert_eq!(result.as_bool().unwrap(), false);
}

#[test]
fn test_interface_type_assertion_null_check() {
    init_tracing!();
    
    // Test program with interface definition, implementation, and type assertion
    let input = r#""
        be_like Greeter collab {
            greet(name tea) tea
        }
        
        be_like Person squad {
            name tea
            age normie
        }
        
        slay main() lit {
            sus greeter Greeter = cap  // Null interface value;

    //
            sus person, ok = greeter.(Person)
            
            lowkey ok {
                yolo based  // Successful type assertion
            } highkey {
                yolo sus    // Failed type assertion
            }
        }
    "#";
    
    // Run the test
    let result = run_jit_test(input).unwrap();
    
    // Should return false since the interface value is null
    assert_eq!(result.as_bool().unwrap(), false);
}