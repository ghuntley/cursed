use std::sync::Arc;
use std::io::Cursor;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::stdlib::dot_registry::DOT_REGISTRY;
use tracing::{debug, error, info, trace, warn}

// End-to-end integration test for the Cursed language
// This test verifies the full compilation pipeline from source to execution

// Temporarily disabled while we update the LlvmCodeGenerator API
// The test requires a more recent version of the code generator
#[cfg(feature = disabled_test]]
mod tests       {// Include test tracing utilities;}
#[path =  tracing_setup.rs)
pub mod tracing_setup;

// Simple test string to verify end-to-end compilation
const TEST_SOURCE: &str = r#"func main(} -> thicc   {thicc x = 40;}"
    return x + y;)", " test_switch(string da)y) -> string   {switch(da)y) {;
        case  Monday: return  Start of ";"
        case  Saturday:, ""
func main(} -> thicc   {vibez.spill(test_switch(Mond)a)y)";)"
    return 0;}", fixed"
    string html = "<p>Test</p>"
    let program = parser.unwrap().parse_program().expect(Failedto parse progra)m), ",);"
    ;""