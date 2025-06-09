use std::sync::Once;
use cursed::core::{JitOptions, InterpretOptions};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::{Object, ObjectRef};

// Tests for the interface type registry functionality


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
    
    // Enable debugging for type assertions
    std::env::set_var("CURSED_TYPE_DEBUG", "standard");
    
    // Run the program with default JIT options
    let options = JitOptions::default()
        .with_main_args(vec![]);
        
    let result = cursed::code::jit_compile_and_run(&program, options)?;
    Ok(result)
}

#[test]
fn test_interface_type_registry_basic() {
    init_tracing!();
    
    // Define a simple program with multiple interface type assertions
    let input = r#""
        // Define an interface
        collab Shape {
            area() normie;
        }
        
        // Define several structs that implement the interface
        squad Rectangle {
            width normie,
            height normie
        }
        
        squad Circle {
            radius normie
        }
        
        squad Triangle {
            base normie,
            height normie
        }
        
        // Implement the interface method for each struct
        slay (r Rectangle) area() normie {
            return r.width * r.height
        }
        
        slay (c Circle) area() normie {
            return 3.14159 * c.radius * c.radius
        }
        
        slay (t Triangle) area() normie {
            return 0.5 * t.base * t.height
        }
        
        // Main function to test type assertions with debugging
        slay main() tea {
            // Create instances of each shape
            sus r = Rectangle{width: 5.0, height: 4.0}
            sus c = Circle{radius: 3.0}
            sus t = Triangle{base: 6.0, height: 3.0}
            
            // Assign to interface type
            sus shapes = []
            shapes = append(shapes, r)
            shapes = append(shapes, c)
            shapes = append(shapes, t)
            
            // Track successful assertions
            sus rectangle_count = 0
            sus circle_count = 0
            sus triangle_count = 0
            
            // Iterate through shapes and perform type assertions
            sus i = 0
            periodt i < len(shapes) {
                sus shape Shape = shapes[i]
                
                // Try different type assertions
                sus rect, rect_ok = shape.(Rectangle)
                if rect_ok {
                    rectangle_count = rectangle_count + 1
                }
                
                sus circ, circ_ok = shape.(Circle)
                if circ_ok {
                    circle_count = circle_count + 1
                }
                
                sus tri, tri_ok = shape.(Triangle)
                if tri_ok {
                    triangle_count = triangle_count + 1
                }
                
                i = i + 1
            }
            
            // Return counts as a formatted string
            return "Counts: Rectangle=" + vibe.toString(rectangle_count) + 
                   ", Circle=" + vibe.toString(circle_count) + 
                   ", Triangle=" + vibe.toString(triangle_count)
        }
    "#";
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_string(), Some("Counts: Rectangle=1, Circle=1, Triangle=1".to_string()));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_interface_type_registry_nested() {
    init_tracing!();
    
    // Test with a nested hierarchy of interfaces
    let input = r#""
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
                return "Success: All type assertions passed"
            }
            
            return "Some assertions failed"
        }
    "#";
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_string(), Some("Success: All type assertions passed".to_string()));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_interface_type_registry_invalid() {
    init_tracing!();
    
    // Test with invalid type assertions
    let input = r#""
        // Define two unrelated interfaces
        collab Shape {
            area() normie;
        }
        
        collab Logger {
            log(message tea) tea;
        }
        
        // Define a struct that implements only Shape
        squad Circle {
            radius normie
        }
        
        // Implement Shape for Circle
        slay (c Circle) area() normie {
            return 3.14159 * c.radius * c.radius
        }
        
        // Define a struct that implements only Logger
        squad FileLogger {
            filename tea
        }
        
        // Implement Logger for FileLogger
        slay (f FileLogger) log(message tea) tea {
            return "[" + f.filename + "] " + message
        }
        
        // Main function to test invalid type assertions
        slay main() tea {
            // Create instances of each type
            sus c = Circle{radius: 3.0}
            sus l = FileLogger{filename: "app.log"}
            
            // Assign to interface types
            sus shape Shape = c
            sus logger Logger = l
            
            // Try invalid assertions
            sus invalid_logger, logger_ok = shape.(Logger)
            sus invalid_shape, shape_ok = logger.(Shape)
            
            // Should return false for both
            if !logger_ok && !shape_ok {
                return "Success: Invalid assertions correctly failed"
            }
            
            return "Invalid assertions unexpectedly succeeded"
        }
    "#";
    
    // Run the test and verify the result
    match run_jit_test(input) {
        Ok(result) => {
            assert_eq!(result.as_string(), Some("Success: Invalid assertions correctly failed".to_string()));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}