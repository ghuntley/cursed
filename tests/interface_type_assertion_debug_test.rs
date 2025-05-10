use std::sync::Once;

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
use cursed::core::{JitOptions, InterpretOptions};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::{Object, ObjectRef};

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
    
    // Enable debugging for type assertions
    std::env::set_var("CURSED_TYPE_DEBUG", "standard");
    
    // Run the program with default JIT options
    let options = JitOptions::default()
        .with_main_args(vec![]);
        
    let result = cursed::code::jit_compile_and_run(&program, options)?;
    Ok(result)
}

#[test]
fn test_interface_type_assertion_debug_success() {
    init_tracing!();
    
    // Define a simple program with interface type assertions
    let input = r#"
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
        
        // Main function to test type assertions with debugging
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
                return "Success: " + person.name
            }
            
            return "Type assertion failed"
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_string(), Some("Success: Alice".to_string()));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_debug_failure() {
    init_tracing!();
    
    // Define a program with a failed type assertion
    let input = r#"
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
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_string(), Some("Assertion failed as expected".to_string()));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_debug_verbose() {
    init_tracing!();
    
    // Set verbose debugging
    std::env::set_var("CURSED_TYPE_DEBUG", "verbose");
    
    // Test with multiple interface implementations
    let input = r#"
        // Define interfaces
        collab Shape {
            area() normie;
        }
        
        collab Printable {
            print() tea;
        }
        
        // Define a struct that implements both interfaces
        squad Rectangle {
            width normie,
            height normie
        }
        
        // Implement Shape for Rectangle
        slay (r Rectangle) area() normie {
            return r.width * r.height
        }
        
        // Implement Printable for Rectangle
        slay (r Rectangle) print() tea {
            return "Rectangle: " + vibe.toString(r.width) + " × " + vibe.toString(r.height)
        }
        
        // Main function to test with verbose debugging
        slay main() tea {
            // Create a Rectangle
            sus r = Rectangle{width: 10.0, height: 5.0}
            
            // Assign to Shape interface
            sus s Shape = r
            
            // Assign to Printable interface
            sus p Printable = r
            
            // Type assertion from Shape back to Rectangle
            sus rect1, ok1 = s.(Rectangle)
            
            // Type assertion from Printable back to Rectangle
            sus rect2, ok2 = p.(Rectangle)
            
            if ok1 && ok2 {
                return rect1.print() + " | Area: " + vibe.toString(rect2.area())
            }
            
            return "Assertion failed"
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_string(), Some("Rectangle: 10 × 5 | Area: 50".to_string()));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_debug_nested() {
    init_tracing!();
    
    // Test with a nested hierarchy of interfaces
    let input = r#"
        // Define a nested interface hierarchy
        collab Object {
            id() lit;
        }
        
        collab Drawable : Object {
            draw() tea;
        }
        
        collab Animated : Drawable {
            animate() tea;
        }
        
        // Implement a class that satisfies the interfaces
        squad AnimatedSprite {
            spriteId lit,
            name tea,
            frameCount lit
        }
        
        // Implement Object interface
        slay (s AnimatedSprite) id() lit {
            return s.spriteId
        }
        
        // Implement Drawable interface
        slay (s AnimatedSprite) draw() tea {
            return "Drawing sprite: " + s.name
        }
        
        // Implement Animated interface
        slay (s AnimatedSprite) animate() tea {
            return "Animating " + s.name + " with " + vibe.toString(s.frameCount) + " frames"
        }
        
        // Main function to test nested interfaces
        slay main() tea {
            // Create an AnimatedSprite
            sus sprite = AnimatedSprite{spriteId: 42, name: "Hero", frameCount: 8}
            
            // Assign to the most specific interface
            sus animated Animated = sprite
            
            // Try assertions at different interface levels
            sus obj, isObj = animated.(Object)
            sus drawable, isDrawable = animated.(Drawable)
            sus original, isOriginal = animated.(AnimatedSprite)
            
            if isObj && isDrawable && isOriginal {
                return obj.id() + ": " + drawable.draw() + " | " + original.animate()
            }
            
            return "Some assertions failed"
        }
    "#;
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_string(), Some("42: Drawing sprite: Hero | Animating Hero with 8 frames".to_string()));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}