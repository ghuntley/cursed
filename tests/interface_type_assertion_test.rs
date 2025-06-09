use std::sync::Once;
use cursed::core::{JitOptions, InterpretOptions};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::{Object, ObjectRef};


// We need to call init_test_tracing only once
static INIT: Once = Once::new();

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {
            tracing_setup::init_test_tracing();
        });
    };
}

// Import required test utilities

// Helper function to run JIT tests on Cursed code
fn run_jit_test(input: &str) -> Result<ObjectRef, String> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msg = parser.errors().join("\n");
        return Err(format!("Parser errors:\n{}", error_msg));
    }
    
    // Run the program with default JIT options
    let options = JitOptions::default()
        .with_main_args(vec![]);
        
    let result = cursed::code::jit_compile_and_run(&program, options)?;
    Ok(result)
}

#[test]
fn test_interface_type_assertion_basic() {
    init_tracing!();
    
    // Define a simple program with interface type assertions
    let input = r#""
        // Define an interface
        collab Stringer {
            toString() tea;
        }
        
        // Define a struct that implements the interface
        squad Person {
            name tea,
            age lit
        }
        
        // Implement the interface method
        slay (p Person) toString() tea {
            return p.name
        }
        
        // Main function to test type assertions
        slay main() tea {
            // Create a Person instance
            sus p = Person{name: "Alice", age: 30}
            
            // Assign to interface type
            sus s Stringer = p
            
            // Type assertion back to Person
            sus person, ok = s.(Person)
            
            // Check if assertion succeeded
            if ok {
                // Access the concrete type's field
                return person.name
            }
            
            return "Type assertion failed"
        }
    "#";
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_string(), Some("Alice".to_string()));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_failed() {
    init_tracing!();
    
    // Define a program with a failed type assertion
    let input = r#""
        // Define two interfaces
        collab Stringer {
            toString() tea;
        }
        
        collab Numberer {
            getValue() lit;
        }
        
        // Define a struct that implements Stringer
        squad Person {
            name tea,
            age lit
        }
        
        // Define a struct that implements Numberer
        squad Counter {
            value lit
        }
        
        // Implement Stringer for Person
        slay (p Person) toString() tea {
            return p.name
        }
        
        // Implement Numberer for Counter
        slay (c Counter) getValue() lit {
            return c.value
        }
        
        // Main function to test failed type assertion
        slay main() tea {
            // Create a Person instance
            sus p = Person{name: "Alice", age: 30}
            
            // Assign to Stringer interface
            sus s Stringer = p
            
            // Try to assert to Counter (should fail)
            sus counter, ok = s.(Counter)
            
            if ok {
                return "Should not succeed"
            } else {
                return "Assertion failed as expected"
            }
        }
    "#";
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_string(), Some("Assertion failed as expected".to_string()));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_multiple() {
    init_tracing!();
    
    // Test with multiple interface implementations
    let input = r#""
        // Define interfaces
        collab Shape {
            area() normie;
        }
        
        collab Printable {
            print() tea;
        }
        
        // Define a struct that implements both interfaces
        squad Circle {
            radius normie
        }
        
        // Implement Shape for Circle
        slay (c Circle) area() normie {
            return 3.14159 * c.radius * c.radius
        }
        
        // Implement Printable for Circle
        slay (c Circle) print() tea {
            return "Circle with radius: " + vibe.toString(c.radius)
        }
        
        // Main function to test multiple interfaces
        slay main() tea {
            // Create a Circle
            sus c = Circle{radius: 5.0}
            
            // Assign to Shape interface
            sus s Shape = c
            
            // Assign to Printable interface
            sus p Printable = c
            
            // Type assertion from Shape back to Circle
            sus circle1, ok1 = s.(Circle)
            
            // Type assertion from Printable back to Circle
            sus circle2, ok2 = p.(Circle)
            
            // Verify both assertions succeeded
            if ok1 && ok2 {
                return "Both assertions succeeded"
            } else if ok1 {
                return "Only Shape assertion succeeded"
            } else if ok2 {
                return "Only Printable assertion succeeded"
            } else {
                return "Both assertions failed"
            }
        }
    "#";
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_string(), Some("Both assertions succeeded".to_string()));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_error_handling() {
    init_tracing!();
    
    // Test proper error handling with type assertions
    let input = r#""
        // Define an interface
        collab Handler {
            handle(msg tea) tea;
        }
        
        // Define structs implementing the interface
        squad StringHandler {
            prefix tea
        }
        
        squad NumberHandler {
            multiplier lit
        }
        
        // Implement Handler for StringHandler
        slay (sh StringHandler) handle(msg tea) tea {
            return sh.prefix + ": " + msg
        }
        
        // Implement Handler for NumberHandler
        slay (nh NumberHandler) handle(msg tea) tea {
            // Try to parse the message as a number and multiply
            sus n = vibe.parseInt(msg)
            return vibe.toString(n * nh.multiplier)
        }
        
        // Function that uses type assertions for dispatching
        slay processMessage(h Handler, msg tea) tea {
            // Try to assert as StringHandler
            sus sh, isString = h.(StringHandler)
            if isString {
                return sh.handle(msg)
            }
            
            // Try to assert as NumberHandler
            sus nh, isNumber = h.(NumberHandler)
            if isNumber {
                return nh.handle(msg)
            }
            
            // Unknown handler type
            return "Error: Unknown handler type"
        }
        
        // Main function to test error handling
        slay main() tea {
            // Create handlers
            sus strHandler = StringHandler{prefix: "String"}
            sus numHandler = NumberHandler{multiplier: 10}
            
            // Process messages with different handlers
            sus result1 = processMessage(strHandler, "hello")
            sus result2 = processMessage(numHandler, "5")
            
            // Return combined results
            return result1 + " | " + result2
        }
    "#";
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_string(), Some("String: hello | 50".to_string()));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}