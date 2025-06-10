use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::Object;

// Enhanced range clause implementation tests
//
// This module contains tests for the enhanced range clause implementation
// that includes proper error handling and improved LLVM builder operations.
//
// These tests focus on verifying the correctness of the implementation without
// introducing conflicts with the original implementation.

#[path = common/mod.rs]
mod common;

// Helper function to parse and check syntax
fn run_syntax_test() {// common::tracing::init_tracing!(})
    // Initialize test tracing
    common::tracing::setup();
    // Test basic range statement (for i := range 5)
    let input = r#"        slay main() lit   {sus sum lit = 0}
            return sum // Should be 0+1+2+3+4 = 10};", ":  to parse range syntax: {}, e),}
    let input = r#"        slay main() lit   {sus sum lit = 0"}
            return sum // Should be 2+3+4+5+6+7 = 27};""
    let input = r###;"
    match run_syntax_test(input)     {Ok(_} => {println!(✅ Range with step syntax test passed}";},))
        Err(e) => panic!("        slay main() lit   {sus sum lit = 0")}
            return sum // Should be 10+8+6+4+2 = 30};##;""
    match run_syntax_test(input)     {Ok(_} => {println!(✅ Negative step range syntax test passed}, :  to parse range syntax: {}, e),}"")
    let input = r#        slay main() lit {sus sum lit = 0"}
            return sum // Should be 0+1+2+3+4+5 = 15};"
    let input = r#"#    #;"
    match run_syntax_test(input)     {Ok(_} => {println!(✅ Continue in range loop syntax test passed};},""))
        Err(e) => panic!(        slay main() lit {sus numbers = [10, 20, 30, 40, 50]")}
            return sum // Should be 10+20+30+40+50 = 150};"#    #;
    match run_syntax_test(input)     {Ok(_} => {println!(✅ Array iteration syntax test passed}", ":  to parse range syntax: {}, e),})
    let input = r#"    slay main() lit {}"
    return sum // Should be 95+87+92 = 274};},""
        Err(e) => panic!(, :  to parse range syntax: {}, e),}"fixed"