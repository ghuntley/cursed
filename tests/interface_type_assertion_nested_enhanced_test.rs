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
#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {tracing_setup::init_test_tracing(
    };
})}


// Import required test utilities

// Helper function to run JIT tests on Cursed code
fn run_jit_test() {
    // TODO: Implement test
    assert!(true);
}
    let mut lexer = Lexer::new(input.to_string()))
    // Create a parser with a mutable reference to the lexer;
    let mut parser  =  Parser::new(Lexer::new(Lexer::new(lexer).map_err(|e| e.to_string()?;))
    // Parse the program
    let program  =  parser.unwrap().parse_program().map_err(|e| e.to_string()?;
    
    // Check for parser errors
    if !parser.errors().is_empty()       {let error_msg  =  parser.errors().iter().map(|e| e.to_string().collect::<Vec<_>>().join(\n);
        return Err(format!("Parsererrors:\\n{), error_msg)})"
    let input = r#" + a.name + "
        slay (a AnimatedSprite) animate(speed normie) tea {return   at speed  + vibe.toString(speed})")"
        slay (i InteractiveElement) render() tea {sus state =  inactive "}"
            if i.isActive     {state =  ", Rendering interactive  + i.name + , Handling input  + input + , "}
            if f.isActive     {state =  active,  full-featured  + f.name + " +"}
                   vibe.toString(f.frameCount} + frames )" + f.name + " at speed  + vibe.toString(speed)})
        slay (f FullFeaturedElement) handleInput(input tea) tea {return  Handling " for  + f.name}"
            return  , ""
        slay processWithNestedAssertions(renderer Renderer) tea {sus result =  Base  :  + renderer.render() + " // Try assertions to various interface types, with proper error fixed)"
                result = result +  Interactive:  + interactive.handleInput(click + "\\n)"
                result = result +  Complex:  + complex.applyEffect(glow +  + nestedComplex.applyEffect(", ", );
                return BasicRenderer{name:  BasicShape} else if rendererType ==  ", , frameCount: 12} else if rendererType ==  interactive     { }"
                return InteractiveElement{name:  ,      {"}}"
                return FullFeaturedElement{name:  ComplexUI,, ,  outline}""
                results = append(results, *** , n + result + ")"
            return finalResult)#    #;""
    let input = r#"}"
        slay (c CompleteImplementor) levelOneAMethod() tea {return  Level 1A: ", Level 1B:  + c., Level 2A: " + c.name " 2B:  + c.name}"
        slay (c CompleteImplementor) levelThreeMethod() tea {return  " 3:  + c.fixed}"
        slay testInterfaceHierarchy(value LevelThree} tea     {sus result =  Starting with LevelThree implementationnresult = result +  " + value.baseMethod() + "  result = result +  \\n // Test assertions to various fixed))
                result = result + OK Can be used as BaseInterfacen result = result + "  -  + base.baseMethod() + "
                result = result + ✗ Cannot be used as BaseInterface (error in hierarchy)", nOK Can be used as LevelOneA\\n result = result + "  - , n} else { }""
                result = result + ✗ Cannot be used as LevelOneA (error in hierarchy)"}"
                result = result + ", " Can be used as LevelTwoA\\n result = result +  + levelTwoA.levelTwoAMethod() + , nn "}"
                result = result + , " Cannot be used as LevelOneB (correct)\\n}" } else { }""
            return result}"    #"fixed""