use std::sync::Once;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::codegen::llvm::interface_type_assertion_nested::NestedInterfaceTypeAssertion;


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
fn run_jit_test(input: &str) -> Result<i32, String> {
    // Create a lexer
    let mut lexer = Lexer::new(input);
    // Create a parser with a mutable reference to the lexer
    let mut parser = Parser::new(&mut lexer).map_err(|e| e.to_string()?;
    // Parse the program
    let program = parser.parse_program().map_err(|e| e.to_string()?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msg = parser.errors().iter().map(|e| e.to_string().collect::<Vec<_>>().join("\n");
        return Err(format!("Parser errors:\n{}", error_msg));
    }
    
    // Create LLVM context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_program.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "main", file_path.clone());
    
    // Compile the program
    code_gen.compile(&program).map_err(|e| e.to_string()?;
    
    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
        .map_err(|e| e.to_string()?;
    
    // Initialize the goroutine manager
    cursed::codegen::jit::init_goroutine_manager();
    
    // Create JIT compiler
    let mut jit_compiler = JitCompiler::new(&context, execution_engine, "_main_main", file_path.clone());
    
    // Use existing code_gen to avoid recompilation
    *jit_compiler.code_generator_mut() = Some(code_gen);
    
    // Execute the program
    let result = jit_compiler.execute().map_err(|e| e.to_string()?;
    
    // Wait for any goroutines to complete (10ms timeout)
    let _remaining = cursed::codegen::jit::wait_for_goroutines(10);
    
    Ok(result)
}

#[test]
fn test_nested_interface_type_assertion() {
    init_tracing!();
    
    // This test verifies the nested interface type assertion functionality
    let input = r#"
        // Define a base interface
        collab BaseRenderer {
            render() tea;
        }
        
        // Define interfaces that extend the base interface
        collab AnimatedRenderer {
            render() tea;
            animate(speed normie) tea;
        }
        
        collab InteractiveRenderer {
            render() tea;
            handleInput(input tea) tea;
        }
        
        // Define a combined interface that extends both
        collab AdvancedRenderer {
            render() tea;
            animate(speed normie) tea;
            handleInput(input tea) tea;
            getDetails() tea;
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
        
        squad AdvancedElement {
            name tea,
            frameCount normie,
            isActive lit,
            details tea
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
        
        slay (a AdvancedElement) render() tea {
            sus state = "inactive"
            if a.isActive {
                state = "active"
            }
            return "Rendering advanced " + a.name + " (" + state + ") with " + 
                   vibe.toString(a.frameCount) + " frames"
        }
        
        slay (a AdvancedElement) animate(speed normie) tea {
            return "Animating advanced " + a.name + " at speed " + vibe.toString(speed)
        }
        
        slay (a AdvancedElement) handleInput(input tea) tea {
            return "Handling input '" + input + "' for advanced " + a.name
        }
        
        slay (a AdvancedElement) getDetails() tea {
            return "Details: " + a.details
        }
        
        // Function to test nested interface assertions
        slay testNestedAssertions(renderer BaseRenderer) tea {
            sus result = "Base: " + renderer.render() + "\n"
            
            // Try assertions to extended interfaces
            sus animated, isAnimated = renderer.(AnimatedRenderer)
            if isAnimated {
                result = result + "Animated: " + animated.animate(5) + "\n"
            }
            
            sus interactive, isInteractive = renderer.(InteractiveRenderer)
            if isInteractive {
                result = result + "Interactive: " + interactive.handleInput("click") + "\n"
            }
            
            sus advanced, isAdvanced = renderer.(AdvancedRenderer)
            if isAdvanced {
                result = result + "Advanced: " + advanced.getDetails() + "\n"
            }
            
            return result
        }
        
        // Main function to test all scenarios
        slay main() normie {
            // Create instances of all renderer types
            sus basic = BasicRenderer{name: "BasicElement"}
            sus animated = AnimatedSprite{name: "AnimatedElement", frameCount: 10}
            sus interactive = InteractiveElement{name: "InteractiveElement", isActive: true}
            sus advanced = AdvancedElement{
                name: "AdvancedElement", 
                frameCount: 24, 
                isActive: true,
                details: "High-performance renderer"
            }
            
            // Test with each renderer type
            sus result1 = testNestedAssertions(basic)
            sus result2 = testNestedAssertions(animated)
            sus result3 = testNestedAssertions(interactive)
            sus result4 = testNestedAssertions(advanced)
            
            // Print results
            vibe.println("--- Basic Renderer ---")
            vibe.println(result1)
            vibe.println("--- Animated Renderer ---")
            vibe.println(result2)
            vibe.println("--- Interactive Renderer ---")
            vibe.println(result3)
            vibe.println("--- Advanced Renderer ---")
            vibe.println(result4)
            
            return 0
        }
    "#;
    
    // Run the test and verify nested interface assertions work correctly
    match run_jit_test(input) {
        Ok(result) => {
            // The test is expected to succeed with proper error propagation
            assert_eq!(result, 0, "Program should complete successfully");
        },
        Err(e) => panic!("Failed to run nested interface assertion test: {}", e),
    }
}

#[test]
fn test_multiple_interface_inheritance() {
    init_tracing!();
    
    // This test checks assertions with complex interface inheritance
    let input = r#"
        // Define a hierarchy of interfaces
        collab Entity {
            getId() normie;
        }
        
        collab Named {
            getName() tea;
        }
        
        collab Positioned {
            getPosition() tea;
        }
        
        collab Visible : Named, Positioned {
            render() tea;
        }
        
        collab Interactive : Visible {
            handleInput(action tea) tea;
        }
        
        collab GameObject : Entity, Interactive {
            update(deltaTime snack) tea;
        }
        
        // Define a concrete implementation
        squad Player {
            id normie,
            name tea,
            x normie,
            y normie,
            isActive lit
        }
        
        // Implement all required methods
        slay (p Player) getId() normie {
            return p.id
        }
        
        slay (p Player) getName() tea {
            return p.name
        }
        
        slay (p Player) getPosition() tea {
            return "(" + vibe.toString(p.x) + ", " + vibe.toString(p.y) + ")"
        }
        
        slay (p Player) render() tea {
            return "Rendering player " + p.name + " at " + p.getPosition()
        }
        
        slay (p Player) handleInput(action tea) tea {
            return "Player " + p.name + " handling action: " + action
        }
        
        slay (p Player) update(deltaTime snack) tea {
            return "Updating player " + p.name + " with delta: " + vibe.toString(deltaTime)
        }
        
        // Test function that tries different interface assertions
        slay testMultipleInheritance(obj Entity) tea {
            sus result = "Entity ID: " + vibe.toString(obj.getId()) + "\n"
            
            // Try assertions to different interfaces in the hierarchy
            sus named, isNamed = obj.(Named)
            if isNamed {
                result = result + "Named: " + named.getName() + "\n"
            }
            
            sus positioned, isPositioned = obj.(Positioned)
            if isPositioned {
                result = result + "Positioned: " + positioned.getPosition() + "\n"
            }
            
            sus visible, isVisible = obj.(Visible)
            if isVisible {
                result = result + "Visible: " + visible.render() + "\n"
            }
            
            sus interactive, isInteractive = obj.(Interactive)
            if isInteractive {
                result = result + "Interactive: " + interactive.handleInput("jump") + "\n"
            }
            
            sus gameObject, isGameObject = obj.(GameObject)
            if isGameObject {
                result = result + "GameObject: " + gameObject.update(0.16) + "\n"
            }
            
            return result
        }
        
        // Main function to run the test
        slay main() normie {
            // Create a player that implements multiple interfaces
            sus player = Player{
                id: 42,
                name: "Hero",
                x: 100,
                y: 200,
                isActive: true
            }
            
            // Test with the player object
            sus result = testMultipleInheritance(player)
            
            // Print the result
            vibe.println("--- Multiple Interface Inheritance Test ---")
            vibe.println(result)
            
            return 0
        }
    "#;
    
    // Run the test and verify complex interface inheritance works correctly
    match run_jit_test(input) {
        Ok(result) => {
            // The test is expected to succeed with proper error propagation
            assert_eq!(result, 0, "Program should complete successfully");
        },
        Err(e) => panic!("Failed to run multiple interface inheritance test: {}", e),
    }
}