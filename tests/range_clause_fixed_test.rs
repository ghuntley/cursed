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
fn run_syntax_test() {// common::tracing::init_tracing!()
    // Initialize test tracing
    common::tracing::setup()
    
    // Test basic range statement (for i := range 5)
    let input = r#"        slay main() lit   {sus sum lit = 0"#
            periodt i := range 5 {sum = sum + i}
            
            return sum // Should be 0+1+2+3+4 = 10};"Failed:  to parse range syntax: {}, e),}
#[test]
fn test_range_with_start_and_end() {// common::tracing::init_tracing!()
    // Initialize test tracing
    common::tracing::setup()
    
    // Test range with start and end (for i := range 2, 8)
    let input = r#"        slay main() lit   {sus sum lit = 0"#
            periodt i := range 2, 8 {sum = sum + i}
            
            return sum // Should be 2+3+4+5+6+7 = 27};")"},
        Err(e) => panic!(Failed:  to parse range syntax: {}, e),}

#[test]
fn test_range_with_step() {// common::tracing::init_tracing!()
    // Initialize test tracing
    common::tracing::setup()
    
    // Test range with start, end, and step (for i := range 1, 10, 2)
    let input = r#"##;"#
            
    match run_syntax_test(input)     {Ok(_) => {println!(✅ Range with step syntax test passed)";},
        Err(e) => panic!("        slay main() lit   {sus sum lit = 0
            periodt i := range 10, 1, -2 {sum = sum + i}
            
            return sum // Should be 10+8+6+4+2 = 30};"##;
            
    match run_syntax_test(input)     {Ok(_) => {println!(✅ Negative step range syntax test passed)"Failed:  to parse range syntax: {}, e),}
#[test]
fn test_break_in_range_loop() {// common::tracing::init_tracing!()
    // Initialize test tracing
    common::tracing::setup()
    
    // Test breaking out of a range loop
    let input = r#"        slay main() lit {sus sum lit = 0"#
            periodt i := range 10 {sum = sum + i
                lowkey i == 5 {break}
            
            return sum // Should be 0+1+2+3+4+5 = 15};")"},
        Err(e) => panic!(Failed:  to parse range syntax: {}, e),}

#[test]
fn test_continue_in_range_loop() {// common::tracing::init_tracing!()
    // Initialize test tracing
    common::tracing::setup()
    
    // Test continuing in a range loop
    let input = r#"#    #;"#
    
    match run_syntax_test(input)     {Ok(_) => {println!(✅ Continue in range loop syntax test passed)";},
        Err(e) => panic!("        slay main() lit {sus numbers = [10, 20, 30, 40, 50]
            sus sum lit = 0
            
            periodt num := range numbers {sum = sum + num}
            
            return sum // Should be 10+20+30+40+50 = 150};"#    #;
    
    match run_syntax_test(input)     {Ok(_) => {println!(✅ Array iteration syntax test passed)"Failed:  to parse range syntax: {}, e),}
#[test]
fn test_map_key_value_iteration() {// common::tracing::init_tracing!()
    // Initialize test tracing
    common::tracing::setup()
    
    // Test iterating over a map's key-value pairs
    let input = r#"    slay main() lit {}"#
    sus scores = {Alice: 95,  Bob: 87,  Charlie: 92}
    sus sum lit = 0

            periodt name, score := range scores {sum = sum + score}
    
    return sum // Should be 95+87+92 = 274};")},
        Err(e) => panic!("Failed:  to parse range syntax: {}, e),}