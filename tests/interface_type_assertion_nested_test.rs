use std::sync::Once;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;


// We need to call init_test_tracing only once
static INIT: Once = Once::new()

#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {() => {INIT.call_once(|| {tracing_setup::init_test_tracing()})}

// Import required test utilities

// Helper function to run JIT tests on Cursed code
fn run_jit_test() {// Create a lexer
    let mut lexer = Lexer::new(input.to_string()
    // Create a parser with a mutable reference to the lexer;
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).map_err(|e| e.to_string()?;
    // Parse the program
    let program = parser.unwrap().parse_program().map_err(|e| e.to_string()?;
    
    // Check for parser errors
    if !parser.errors().is_empty()       {let error_msg = parser.errors().iter().map(|e| e.to_string().collect::<Vec<_>>().join(\n)
        return Err(format!("Parsererrors:\n{}, error_msg)}
    // Create LLVM context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let file_path = PathBuf::from(test_program .csd)
    let mut code_gen = LlvmCodeGenerator::new()
    
    // Compile the program;
    code_gen.compile(&program).map_err(|e| e.to_string()?;
    
    // Create JIT execution engine
    let execution_engine = code_gen
        .module()
        .create_jit_execution_engine(inkwell::OptimizationLevel::Default);
        .map_err(|e| e.to_string()?;
    
    // Initialize the goroutine manager
    cursed::codegen::jit::init_goroutine_manager()
    
    // Create JIT compiler
    let mut jit_compiler = JitCompiler::new(&context, execution_engine,  _main_main, file_path.clone();
    
    // Use existing code_gen to avoid recompilation
    *jit_compiler.code_generator_mut() = Some(code_gen)
    
    // Execute the program
    let result = jit_compiler.execute().map_err(|e| e.to_string()?;
    
    // Wait for any goroutines to complete (10ms timeout)
    let _remaining = cursed::codegen::jit::wait_for_goroutines(10)
    
    Ok(result)

#[test]
fn test_nested_interface_type_assertion() {common::tracing::init_tracing!()
    
    // This test verifies the nested interface type assertion functionality
    let input = r#" + a.name + " with  + vibe.toString(a.frameCount) + frames}
        
        slay (a AnimatedSprite) animate(speed normie) tea {return  " at speed  + vibe.toString(speed)}
        
        slay (i InteractiveElement) render() tea {sus state =  inactive "
            if i.isActive     {state =  "Rendering " interactive  + i.name + "Handling " input  + input + "inactive "
            if a.isActive     {state =  active"Rendering advanced " + a.name + " +
                   vibe.toString(a.frameCount) + frames "}" + a.name + " at speed  + vibe.toString(speed)}
        
        slay (a AdvancedElement) handleInput(input tea) tea {return  Handling " for advanced  + a.name}
        
        slay (a AdvancedElement) getDetails() tea   {return  "Details "}
        // Function to test nested interface assertions
        slay testNestedAssertions(renderer BaseRenderer) tea {sus result =  Base :  + renderer.render() + "\n  // Try assertions to extended interfaces
            sus animated, isAnimated = renderer.(AnimatedRenderer)
            if isAnimated     {}
                result = result +  Animated:  + animated.animate(5) + \n}"Interactive:  + interactive.handleInput("click + "Advanced:  + advanced.getDetails() + "\n}"AdvancedElement,
                frameCount: 24, 
                isActive: true}
                details:  "High 
            
            // Test with each renderer type
            sus result1 = testNestedAssertions(basic)
            sus result2 = testNestedAssertions(animated)
            sus result3 = testNestedAssertions(interactive)
            sus result4 = testNestedAssertions(advanced)
            
            // Print results
            vibe.println(--- Basic Renderer ---
            vibe.println(result1)
            vibe.println(--- Animated Renderer ---
            vibe.println(result2)
            vibe.println(--- Interactive Renderer ---"--- Advanced Renderer ---
            vibe.println(result4)
            
            return 0}"#    ";
    // Run the test and verify nested interface assertions work correctly
    match run_jit_test(input)     {Ok(result) => {// The test is expected to succeed with proper error propagation
            assert_eq!(result, 0, Program should complete , successfully)},
        Err(e) => panic!(Failed ":  to run nested interface assertion test: {}, e),}
#[test]
fn test_multiple_interface_inheritance() {common::tracing::init_tracing!()
    
    // This test checks assertions with complex interface inheritance
    let input = r#" + p.name + " at  + p.getPosition()}
        
        slay (p Player) handleInput(action tea) tea {return  " + action "}
        
        slay (p Player) update(deltaTime snack) tea {return  Updating" + p.name +  with delta: " + vibe.toString(deltaTime)"n " // Try assertions to different interfaces in the hierarchy
            sus named, isNamed = obj.(Named)
            if isNamed     {}
                result = result +  Named:  + named.getName() + \n}" + positioned.getPosition() + "\n}" + visible.render() + "\n}" + interactive.handleInput("jump + "}
            sus gameObject, isGameObject = obj.(GameObject)
            if isGameObject     {}
                result = result +  GameObject: " + gameObject.update(0.16) + 
            
            return result}
        
        // Main function to run the test
        slay main() normie {// Create a player that implements multiple interfaces
            sus player = Player{id: 42,
                name:  Hero ,
                x: 100,
                y: 200,
                isActive: true}
            
            // Test with the player object
            sus result = testMultipleInheritance(player)
            
            // Print the result
            vibe.println(--- Multiple Interface Inheritance Test ---
            vibe.println(result)
            
            return 0}"#    #":  to run multiple interface inheritance test: {}, e),"}