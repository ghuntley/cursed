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
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::codegen::llvm::TypeAssertionErrorPropagation;

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
fn test_improved_error_propagation() {
    init_tracing!();
    
    // This test verifies enhanced error propagation in interface type assertions
    let input = r#"
        // Define an interface
        collab ErrorHandler {
            handle(msg tea) tea;
        }
        
        // Define two struct types that implement the interface
        squad SafeHandler {
            prefix tea
        }
        
        squad DetailedHandler {
            level normie,
            verbose lit
        }
        
        // Implement the interface method for SafeHandler
        slay (h SafeHandler) handle(msg tea) tea {
            return h.prefix + ": " + msg
        }
        
        // Implement the interface method for DetailedHandler
        slay (h DetailedHandler) handle(msg tea) tea {
            sus result = "Level " + vibe.toString(h.level) + ": " + msg
            if h.verbose {
                result = result + " (verbose mode)"
            }
            return result
        }
        
        // Function with multiple levels of type assertions
        slay processMessage(handler ErrorHandler, msg tea) tea {
            // Try multiple type assertions with proper error propagation
            sus safeHandler, isSafe = handler.(SafeHandler)
            sus detailedHandler, isDetailed = handler.(DetailedHandler)
            
            // First approach: concatenate results from both handlers if available
            if isSafe && isDetailed {
                // This would never happen in a normal program (same value can't be both types)
                // but tests our error handling in complex scenarios
                return safeHandler.handle(msg) + " AND " + detailedHandler.handle(msg)
            }
            
            // Second approach: use the safe handler
            if isSafe {
                return safeHandler.handle(msg) + " (safe mode)"
            }
            
            // Third approach: use the detailed handler
            if isDetailed {
                return detailedHandler.handle(msg)
            }
            
            // Fallback: just use the interface method
            return "Generic: " + handler.handle(msg)
        }
        
        // Nested type assertions with proper error propagation
        slay processWithWrapper(wrapper lit) tea {
            // Call a generic function that returns an interface value
            sus handler = getHandler(wrapper)
            
            // Process a message with this handler
            return processMessage(handler, "Test message")
        }
        
        // Return different handlers based on the wrapper value
        slay getHandler(wrapper lit) ErrorHandler {
            if wrapper {
                return DetailedHandler{level: 5, verbose: true}
            } else {
                return SafeHandler{prefix: "Safe"}
            }
        }
        
        // Main function to test error propagation
        slay main() tea {
            // Create both handler types
            sus safe = SafeHandler{prefix: "Safe"}
            sus detailed = DetailedHandler{level: 3, verbose: false}
            
            // Process messages with both direct handlers
            sus result1 = processMessage(safe, "Direct safe message")
            sus result2 = processMessage(detailed, "Direct detailed message")
            
            // Process messages with handlers from wrappers
            sus result3 = processWithWrapper(false) // Gets SafeHandler
            sus result4 = processWithWrapper(true)  // Gets DetailedHandler
            
            // Return all results
            return result1 + "\n" + result2 + "\n" + result3 + "\n" + result4
        }
    "#;
    
    // Run the test and verify enhanced error propagation works correctly
    match run_jit_test(input) {
        Ok(result) => {
            // The test is expected to succeed with proper error propagation
            assert_eq!(result, 0, "Program should complete successfully");
        },
        Err(e) => panic!("Failed to run test with enhanced error propagation: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_complex_error_propagation() {
    init_tracing!();
    
    // This test checks error propagation in complex nested type assertions
    let input = r#"
        // Define a hierarchy of interfaces
        collab Renderer {
            render() tea;
        }
        
        collab AnimatedRenderer {
            render() tea;
            animate(speed normie) tea;
        }
        
        collab InteractiveRenderer {
            render() tea;
            handleInput(input tea) tea;
        }
        
        // Define concrete types implementing these interfaces
        squad BasicRenderer {
            name tea
        }
        
        squad AnimatedSprite {
            name tea,
            frameCount normie
        }
        
        squad InteractiveElement {
            name tea,
            isActive lit
        }
        
        squad FullFeaturedElement {
            name tea,
            frameCount normie,
            isActive lit
        }
        
        // Implement interface methods
        slay (r BasicRenderer) render() tea {
            return "Rendering " + r.name
        }
        
        slay (a AnimatedSprite) render() tea {
            return "Rendering animated " + a.name + " with " + vibe.toString(a.frameCount) + " frames"
        }
        
        slay (a AnimatedSprite) animate(speed normie) tea {
            return "Animating " + a.name + " at speed " + vibe.toString(speed)
        }
        
        slay (i InteractiveElement) render() tea {
            sus state = "inactive"
            if i.isActive {
                state = "active"
            }
            return "Rendering interactive " + i.name + " (" + state + ")"
        }
        
        slay (i InteractiveElement) handleInput(input tea) tea {
            return "Handling input '" + input + "' for " + i.name
        }
        
        slay (f FullFeaturedElement) render() tea {
            sus state = "inactive"
            if f.isActive {
                state = "active"
            }
            return "Rendering full-featured " + f.name + " (" + state + ") with " + 
                   vibe.toString(f.frameCount) + " frames"
        }
        
        slay (f FullFeaturedElement) animate(speed normie) tea {
            return "Animating " + f.name + " at speed " + vibe.toString(speed)
        }
        
        slay (f FullFeaturedElement) handleInput(input tea) tea {
            return "Handling input '" + input + "' for " + f.name
        }
        
        // Complex function using nested type assertions with error propagation
        slay processRenderer(renderer Renderer, input tea, animationSpeed normie) tea {
            sus result = "Base: " + renderer.render() + "\n"
            
            // Try assertion to AnimatedRenderer
            sus animated, isAnimated = renderer.(AnimatedRenderer)
            if isAnimated {
                result = result + "Animated: " + animated.animate(animationSpeed) + "\n"
            }
            
            // Try assertion to InteractiveRenderer
            sus interactive, isInteractive = renderer.(InteractiveRenderer)
            if isInteractive {
                result = result + "Interactive: " + interactive.handleInput(input) + "\n"
            }
            
            // Try assertion to specific concrete types for specialized handling
            sus fullFeatured, isFullFeatured = renderer.(FullFeaturedElement)
            if isFullFeatured {
                result = result + "FullFeatured: Special processing for " + fullFeatured.name + "\n"
            }
            
            return result
        }
        
        // Main function to test all scenarios
        slay main() tea {
            // Create instances of all renderer types
            sus basic = BasicRenderer{name: "BasicElement"}
            sus animated = AnimatedSprite{name: "AnimatedElement", frameCount: 10}
            sus interactive = InteractiveElement{name: "InteractiveElement", isActive: true}
            sus fullFeatured = FullFeaturedElement{name: "FullFeaturedElement", frameCount: 24, isActive: true}
            
            // Process all renderers
            sus result1 = processRenderer(basic, "click", 5)
            sus result2 = processRenderer(animated, "hover", 8)
            sus result3 = processRenderer(interactive, "keypress", 3)
            sus result4 = processRenderer(fullFeatured, "drag", 12)
            
            // Return combined result
            return "Results:\n" + 
                   "--- Basic ---\n" + result1 + "\n" + 
                   "--- Animated ---\n" + result2 + "\n" + 
                   "--- Interactive ---\n" + result3 + "\n" + 
                   "--- Full Featured ---\n" + result4
        }
    "#;
    
    // Run the test and verify complex nested assertions work correctly
    match run_jit_test(input) {
        Ok(result) => {
            // The test is expected to succeed with proper error propagation
            assert_eq!(result, 0, "Program should complete successfully");
        },
        Err(e) => panic!("Failed to run complex type assertion test: {}", e),
    }
}