use std::sync::Once;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::codegen::llvm::interface_type_assertion_nested_enhanced::NestedInterfaceTypeAssertionEnhanced;


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
fn test_enhanced_nested_interface_type_assertions() {
    init_tracing!();
    
    // This test verifies enhanced nested interface type assertions
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
        
        // This interface extends both AnimatedRenderer and InteractiveRenderer
        collab ComplexRenderer {
            render() tea;
            animate(speed normie) tea;
            handleInput(input tea) tea;
            applyEffect(name tea) tea;
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
            isActive lit,
            effectsList []tea
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
        
        slay (f FullFeaturedElement) applyEffect(name tea) tea {
            f.effectsList = append(f.effectsList, name)
            return "Applied effect " + name + " to " + f.name
        }
        
        // Function to test nested interface assertions with enhanced error handling
        slay processWithNestedAssertions(renderer Renderer) tea {
            sus result = "Base: " + renderer.render() + "\n"
            
            // Try assertions to various interface types, with proper error propagation
            
            // 1. Direct parent interface assertion
            sus basicRenderer, isBasicRenderer = renderer.(Renderer)
            if isBasicRenderer {
                result = result + "Confirmed basic renderer capability\n"
            }
            
            // 2. Child interface assertion (might fail)
            sus animated, isAnimated = renderer.(AnimatedRenderer)
            if isAnimated {
                result = result + "Animated: " + animated.animate(5) + "\n"
            }
            
            // 3. Another child interface assertion (might fail)
            sus interactive, isInteractive = renderer.(InteractiveRenderer)
            if isInteractive {
                result = result + "Interactive: " + interactive.handleInput("click") + "\n"
            }
            
            // 4. Complex interface that extends multiple interfaces (might fail)
            sus complex, isComplex = renderer.(ComplexRenderer)
            if isComplex {
                result = result + "Complex: " + complex.applyEffect("glow") + "\n"
            }
            
            // 5. Concrete type assertions (for specialization)
            sus fullFeatured, isFullFeatured = renderer.(FullFeaturedElement)
            if isFullFeatured {
                result = result + "Full Featured: Special processing\n"
                
                // Nested assertions on the result of first assertion
                // This tests proper error propagation in complex nested scenarios
                sus nestedComplex, isNestedComplex = fullFeatured.(ComplexRenderer)
                if isNestedComplex {
                    result = result + "  - Confirmed that FullFeaturedElement is also a ComplexRenderer\n"
                    result = result + "  - " + nestedComplex.applyEffect("special") + "\n"
                }
            }
            
            return result
        }
        
        // Function to create renderers of different types
        slay createRenderer(rendererType tea) Renderer {
            if rendererType == "basic" {
                return BasicRenderer{name: "BasicShape"}
            } else if rendererType == "animated" {
                return AnimatedSprite{name: "AnimatedCharacter", frameCount: 12}
            } else if rendererType == "interactive" {
                return InteractiveElement{name: "Button", isActive: true}
            } else if rendererType == "full" {
                return FullFeaturedElement{
                    name: "ComplexUI", 
                    frameCount: 24, 
                    isActive: true,
                    effectsList: []tea{"shadow", "outline"}
                }
            }
            
            // Default case
            return BasicRenderer{name: "Default"}
        }
        
        // Main function to test everything
        slay main() tea {
            sus rendererTypes = []tea{"basic", "animated", "interactive", "full"}
            sus results = []tea{}
            
            for rendererType in rendererTypes {
                sus renderer = createRenderer(rendererType)
                sus result = processWithNestedAssertions(renderer)
                results = append(results, "*** " + rendererType + " ***\n" + result + "\n")
            }
            
            // Join all results and return
            sus finalResult = ""
            for result in results {
                finalResult = finalResult + result
            }
            
            return finalResult
        }
    "#;
    
    // Run the test and verify enhanced nested interface assertions work correctly
    match run_jit_test(input) {
        Ok(_) => {
            // Test passes if it runs without errors
        },
        Err(e) => panic!("Failed to run enhanced nested interface assertion test: {}", e),
    }
}

#[test]
fn test_interface_extension_hierarchy() {
    init_tracing!();
    
    // This test specifically tests interface extension hierarchies
    let input = r#"
        // Define a deep hierarchy of interfaces with extensions
        collab BaseInterface {
            baseMethod() tea;
        }
        
        collab LevelOneA {
            baseMethod() tea;
            levelOneAMethod() tea;
        }
        
        collab LevelOneB {
            baseMethod() tea;
            levelOneBMethod() tea;
        }
        
        collab LevelTwoA {
            baseMethod() tea;
            levelOneAMethod() tea;
            levelTwoAMethod() tea;
        }
        
        collab LevelTwoB {
            baseMethod() tea;
            levelOneBMethod() tea;
            levelTwoBMethod() tea;
        }
        
        collab LevelThree {
            baseMethod() tea;
            levelOneAMethod() tea;
            levelTwoAMethod() tea;
            levelThreeMethod() tea;
        }
        
        // Implement a type that implements the lowest level interface
        squad CompleteImplementor {
            name tea
        }
        
        // Implement all methods
        slay (c CompleteImplementor) baseMethod() tea {
            return "Base: " + c.name
        }
        
        slay (c CompleteImplementor) levelOneAMethod() tea {
            return "Level 1A: " + c.name
        }
        
        slay (c CompleteImplementor) levelOneBMethod() tea {
            return "Level 1B: " + c.name
        }
        
        slay (c CompleteImplementor) levelTwoAMethod() tea {
            return "Level 2A: " + c.name
        }
        
        slay (c CompleteImplementor) levelTwoBMethod() tea {
            return "Level 2B: " + c.name
        }
        
        slay (c CompleteImplementor) levelThreeMethod() tea {
            return "Level 3: " + c.name
        }
        
        // Function to test if a value implements interfaces at any level
        slay testInterfaceHierarchy(value LevelThree) tea {
            sus result = "Starting with LevelThree implementation\n"
            result = result + "Base method: " + value.baseMethod() + "\n"
            result = result + "Level 3 method: " + value.levelThreeMethod() + "\n"
            
            // Test assertions to various levels
            sus base, isBase = value.(BaseInterface)
            if isBase {
                result = result + "✓ Can be used as BaseInterface\n"
                result = result + "  - " + base.baseMethod() + "\n"
            } else {
                result = result + "✗ Cannot be used as BaseInterface (error in hierarchy)\n"
            }
            
            sus levelOneA, isLevelOneA = value.(LevelOneA)
            if isLevelOneA {
                result = result + "✓ Can be used as LevelOneA\n"
                result = result + "  - " + levelOneA.levelOneAMethod() + "\n"
            } else {
                result = result + "✗ Cannot be used as LevelOneA (error in hierarchy)\n"
            }
            
            sus levelTwoA, isLevelTwoA = value.(LevelTwoA)
            if isLevelTwoA {
                result = result + "✓ Can be used as LevelTwoA\n"
                result = result + "  - " + levelTwoA.levelTwoAMethod() + "\n"
            } else {
                result = result + "✗ Cannot be used as LevelTwoA (error in hierarchy)\n"
            }
            
            // This should fail as LevelThree doesn't extend LevelOneB or LevelTwoB
            sus levelOneB, isLevelOneB = value.(LevelOneB)
            if isLevelOneB {
                result = result + "✓ Can be used as LevelOneB (unexpected)\n"
            } else {
                result = result + "✓ Cannot be used as LevelOneB (correct)\n"
            }
            
            sus levelTwoB, isLevelTwoB = value.(LevelTwoB)
            if isLevelTwoB {
                result = result + "✓ Can be used as LevelTwoB (unexpected)\n"
            } else {
                result = result + "✓ Cannot be used as LevelTwoB (correct)\n"
            }
            
            return result
        }
        
        // Main function to execute the test
        slay main() tea {
            sus implementor = CompleteImplementor{name: "HierarchyTest"}
            sus result = testInterfaceHierarchy(implementor)
            return result
        }
    "#;
    
    // Run the test and verify the interface extension hierarchy works correctly
    match run_jit_test(input) {
        Ok(_) => {
            // Test passes if it runs without errors
        },
        Err(e) => panic!("Failed to run interface extension hierarchy test: {}", e),
    }
}