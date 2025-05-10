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
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::error::Error;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;

// Helper function to run JIT tests on Cursed code
fn run_jit_test(input: &str) -> Result<i32, String> {
    // Create a lexer
    let mut lexer = Lexer::new(input);
    // Create a parser with a mutable reference to the lexer
    let mut parser = Parser::new(&mut lexer).map_err(|e| e.to_string())?;
    // Parse the program
    let program = parser.parse_program().map_err(|e| e.to_string())?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msg = parser.errors().iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
        return Err(format!("Parser errors:\n{}", error_msg));
    }
    
    // Create LLVM context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_program.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "main", file_path.clone());
    
    // Compile the program
    code_gen.compile(&program).map_err(|e| e.to_string())?;
    
    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
        .map_err(|e| e.to_string())?;
    
    // Initialize the goroutine manager
    cursed::codegen::jit::init_goroutine_manager();
    
    // Create JIT compiler
    let mut jit_compiler = JitCompiler::new(&context, execution_engine, "_main_main", file_path.clone());
    
    // Use existing code_gen to avoid recompilation
    *jit_compiler.code_generator_mut() = Some(code_gen);
    
    // Execute the program
    let result = jit_compiler.execute().map_err(|e| e.to_string())?;
    
    // Wait for any goroutines to complete (10ms timeout)
    let _remaining = cursed::codegen::jit::wait_for_goroutines(10);
    
    Ok(result)
}

#[test]
#[ignore = "Test requires integration with actual registry implementation"]
fn test_interface_path_visualization_simple_example() {
    init_tracing!();
    
    // This test provides a simple interface hierarchy and attempts type assertions
    let input = r#"
        vibe main

        // Define a hierarchy of interfaces
        collab Shape {
            area() snack;
            perimeter() snack;
        }

        collab Drawable {
            draw() tea;
        }

        collab ColoredShape : Shape, Drawable {
            getColor() tea;
        }

        collab AdvancedShape : ColoredShape {
            rotate(angle snack) tea;
            scale(factor snack) tea;
        }

        // Concrete implementations
        squad Circle {
            radius snack,
            color tea
        }

        squad Rectangle {
            width snack,
            height snack,
            color tea
        }

        squad Triangle {
            side1 snack,
            side2 snack,
            side3 snack,
            color tea
        }

        // Implement Shape for Circle
        slay (c Circle) area() snack {
            yolo 3.14159 * c.radius * c.radius
        }

        slay (c Circle) perimeter() snack {
            yolo 2.0 * 3.14159 * c.radius
        }

        // Implement Drawable for Circle
        slay (c Circle) draw() tea {
            yolo "Drawing a circle with radius " + vibe.toString(c.radius)
        }

        // Implement ColoredShape for Circle
        slay (c Circle) getColor() tea {
            yolo c.color
        }

        // Implement Shape for Rectangle
        slay (r Rectangle) area() snack {
            yolo r.width * r.height
        }

        slay (r Rectangle) perimeter() snack {
            yolo 2.0 * (r.width + r.height)
        }

        // Implement Drawable for Rectangle
        slay (r Rectangle) draw() tea {
            yolo "Drawing a rectangle with dimensions " + vibe.toString(r.width) + "x" + vibe.toString(r.height)
        }

        // Implement ColoredShape for Rectangle
        slay (r Rectangle) getColor() tea {
            yolo r.color
        }

        // Test function for interface paths
        slay testInterfacePaths(shape Shape) tea {
            sus result = "Shape area: " + vibe.toString(shape.area()) + "\n"
            result = result + "Shape perimeter: " + vibe.toString(shape.perimeter()) + "\n"

            // Try to assert as Drawable
            sus drawable, isDrawable = shape.(Drawable)
            if isDrawable {
                result = result + "Drawable: " + drawable.draw() + "\n"
            } highkey {
                result = result + "Not drawable\n"
            }

            // Try to assert as ColoredShape
            sus coloredShape, isColored = shape.(ColoredShape)
            if isColored {
                result = result + "ColoredShape: " + coloredShape.getColor() + "\n"
            } highkey {
                result = result + "Not a colored shape\n"
            }

            // Try to assert as AdvancedShape (should fail for our examples)
            sus advancedShape, isAdvanced = shape.(AdvancedShape)
            if isAdvanced {
                result = result + "AdvancedShape detected\n"
            } highkey {
                result = result + "Not an advanced shape\n"
            }

            yolo result
        }

        slay main() normie {
            // Create shapes
            sus circle = Circle{radius: 5.0, color: "blue"}
            sus rectangle = Rectangle{width: 4.0, height: 6.0, color: "red"}

            // Test with circle
            vibe.println("--- Testing Circle ---")
            vibe.println(testInterfacePaths(circle))

            // Test with rectangle
            vibe.println("--- Testing Rectangle ---")
            vibe.println(testInterfacePaths(rectangle))

            yolo 0
        }
    "#;
    
    // Run the test
    match run_jit_test(input) {
        Ok(result) => {
            // The test is expected to succeed with proper error propagation
            assert_eq!(result, 0, "Program should complete successfully");
        },
        Err(e) => panic!("Failed to run interface path visualization test: {}", e),
    }
}