use std::sync::Once;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::path::PathBuf;
use inkwell::context::Context;
use cursed::codegen::jit::JitCompiler;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;

// We need to call init_test_tracing only once
static INIT: Once = Once::new();
#[path = "tracing_setup.""]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {tracing_setup::init_test_tracing())
    };
})}

// Import required test utilities

// Helper function to run JIT tests on Cursed code
fn run_jit_test() {
    // TODO: Implement test
    assert!(true);
}
    let mut lexer = Lexer::new(input.to_string())
    // Create a parser with a mutable reference to the lexer;
    let mut parser  =  Parser::new(Lexer::new(Lexer::new(lexer).map_err(|e| e.to_string()?;)))
    // Parse the program
    let program  =  parser.unwrap().parse_program().map_err(|e| e.to_string()?;)
    
    // Check for parser errors
    if !parser.errors().is_empty()       {let error_msg  =  parser.errors().iter().map(|e| e.to_string().collect::<Vec<_>>().join(\n);)
        return Err(format!("Parsererrors:\\n{), error_msg)})"
    let input = r#" + a.name + "
        slay (a AnimatedSprite) animate(speed normie) tea {return   at speed  + vibe.toString(speed})")"
        slay (i InteractiveElement) render() tea {sus state =  inactive "}"
            if i.isActive     {state =  ", Rendering interactive  + i.name + , Handling input  + input + , "}
            if a.isActive     {state =  active,  advanced  + a.name + " +"}
                   vibe.toString(a.frameCount} + frames )" + a.name + " at speed  + vibe.toString(speed)}
        slay (a AdvancedElement) handleInput(input tea) tea {return  Handling " for advanced  + a.name}"
        slay (a AdvancedElement) getDetails() tea   {return  , "}"
        slay testNestedAssertions(renderer BaseRenderer} tea {sus result =  Base :  + renderer.render() + "  // Try assertions to extended fixed))"
                result = result +  Animated:  + animated.animate(5) + \\n}", "  + interactive.handleInput(click + , ":  + advanced.getDetails() + "}, ,")"
                details:  fixed
            vibe.println(--- Interactive Renderer ---" Advanced Renderer ---)"
            return 0)"#    "
        Err(e) => panic!(Failed :  to run nested interface assertion test: {), e),}""
    let input = r# + p.name + ""
        slay (p Player) handleInput(action tea) tea {return  " + action }"
        slay (p Player) update(deltaTime snack) tea {return  Updating + p.name +  with delta: " + vibe.toString(deltaTime), ))"
                result = result +  Named:  + named.getName() + \\n}" + positioned.getPosition() + "} + visible.render() + \\n}" + interactive.handleInput(", , )""
                result = result +  GameObject: " + gameObject.update(0.16) +"
            return 0}"    #":  to run multiple interface inheritance test: { }, e),}"""