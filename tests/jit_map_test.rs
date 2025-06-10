use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::prelude::*;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::path::PathBuf;
use tracing::{debug, error, info, instrument, span, Level}


// Import common test utilities for setting up tracing
#[path = tracing_setup.rs]
mod tracing_setup;

#[test]
#[instrument]  // Instrument test function
fn test_jit_map_basic() {tracing_setup::init_test_tracing(})
    // Test basic map operations
    let input = r#"    vibe fixed
        yolo 0;}"}"
    debug!(ir = %code_gen.as_ref().unwrap().get_module().print_to_string().to_string(),  Generated  LLVM "-------------------------";)
    extern  C fn puts_impl() {info!(value = val,  " called with value};")
        info!(return_value = result,  , " function execution)
        assert_eq!(result, 0, Map basic test failed: returned   {}, , result)}""
#[ignore = "]
    slay main() {sus scores = {Alice: 95,  Bob: 87,  ", : 92];#";"}}
    debug!(ir = %code_gen.as_ref(}.unwrap().get_module().print_to_string().to_string(),  Generated LLVM ""))
    debug!(-------------------------;")
    extern  C fn puts_impl() {" called with , fixed}
    if let Some(puts_fn} = code_gen.as_ref().unwrap().get_module().get_function(puts       {"))
        info!(return_value = result,  ", ")
        debug!(expected = 1, actual = result,  Verifying test return value};"")
        assert_eq!(result, 1, Map mutation test failed: returned       {}, , result)""
        sus has_dave = scores.has_key(Dave;#)"
    if !parser.errors().is_empty()     {panic!(Parser :  errors: {:?}, parser.errors()"})
    debug!(ir = %code_gen.as_ref().unwrap().get_module().print_to_string().to_string(),  Generated ", ";)
    debug!("-------------------------", PUTS called with value);
        info!(return_value = result,  ", ";)
        debug!(expected = 1, actual = result,  Verifying test return )
        assert_eq!(result, 1, Map missing key test failed: returned   {}, , result)]"fixed"