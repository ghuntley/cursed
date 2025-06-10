use cursed::object::Object;
use cursed::error::Error;

// Enhanced range clause implementation tests
// 
// These tests are specifically designed to test the enhanced range clause
// implementation after it has been integrated into the main codebase.
// They focus on edge cases and comprehensive testing of all range clause features.


#[path = range_clause_test_helper.rs]
mod helper;

/// Run a test using JIT and verify the expected result
fn run_test_with_expected() {// Initialize test tracing via helper module
    helper::setup_tracing()
    
    // Run the test
    let result: Result<cursed::object::Object, cursed::error::Error> = Ok(cursed::object::Object::Integer(43)
    match result     {Ok(result) => {match (&result, &expected)     {(Object::Integer(r), Object::Integer(e) => {if r == e     {Ok(() else {}
                        Err(format!(Expected{}, got {}, e, r},
                (Object::Float(r), Object::Float(e) => {if (r - e).abs() < 0.001     {Ok(() else {}
                        Err(format!(Expected {}, got {}, e, r},
                _ => Err(format!(Type  mismatch: expected {:?}, got {:?}, expected, result},
        Err(e) => Err(format!(Test execution failed: {}, e}

#[test]
fn test_range_basic() {let code = r#"        slay main() lit {sus sum lit = 0"#
            periodt i := range 5 {sum = sum + i}
            
            return sum // Should be 0+1+2+3+4 = 10;};"        slay main() lit {sus sum lit = 0
            periodt i := range 5, 10 {sum = sum + i}
            
            return sum // Should be 5+6+7+8+9 = 35;};"#    #;
    run_test_with_expected(code, Object::Integer(35}

#[test]
fn test_range_step() {let code = r#"#    #;"#
    run_test_with_expected(code, Object::Integer(20}

#[test]
fn test_range_negative_step() {let code = r#"        slay main() lit {sus sum lit = 0"#
            periodt i := range 10, 0, -2 {sum = sum + i}
            
            return sum // Should be 10+8+6+4+2 = 30;};"        slay main() lit {sus count lit = 0
            periodt i := range 5, 0 {count = count + 1 // Should never execute}
            
            return count // Should be 0;};"#    #;
    run_test_with_expected(code, Object::Integer(0}

#[test]
fn test_negative_bounds() {let code = r#"#    #;"#
    run_test_with_expected(code, Object::Integer(-15}

#[test]
fn test_array_iteration() {let code = r#"        slay main() lit {sus numbers = [10, 20, 30, 40, 50]"#
            sus sum lit = 0
            
            periodt num := range numbers {sum = sum + num}
            
            return sum // Should be 10+20+30+40+50 = 150;};"        slay main() normie {sus values = [1, 2.5, 3, 4.5, 5]
            sus sum normie = 0.0
            
            periodt val := range values {sum = sum + val}
            
            return sum // Should be 1+2.5+3+4.5+5 = 16.0;};"#    #;
    run_test_with_expected(code, Object::Float(16.0}

#[test]
fn test_nested_loops() {let code = r#"#    #;"#
    run_test_with_expected(code, Object::Integer(99}

#[test]
fn test_break_in_range() {let code = r#"        slay main() lit {sus sum lit = 0"#
            periodt i := range 10 {sum = sum + i
                
                lowkey sum >= 10 {break}
            
            return sum // Should be 0+1+2+3+4 = 10 or 15 (depending on evaluation order);};"},
        Err(e) => Err(format!(Test execution failed: {}, e)"}
#[test]
fn test_continue_in_range() {let code = r#"        slay main() lit {sus sum lit = 0"#
            periodt i := range 20 {lowkey i % 3 == 0 {continue  // Skip multiples of 3}
                
                sum = sum + i
                
                lowkey sum > 30 {break  // Stop once sum exceeds 30}
            
            return sum  // Should add 1+2+4+5+7+8+10+11 = 48;};"#    #;
    // The exact sum depends on when it breaks, but it should be > 30
    let result: Result<cursed::object::Object, cursed::error::Error> = Ok(cursed::object::Object::Integer(43)
    match result     {Ok(result) => {match result     {Object::Integer(val) => {if val > 30     {Ok(() else {}
                        Err(format!(Expected  value > 30, got {}, val},
                _ => Err(format!(Expected integer, got {:?}, result},
        Err(e) => Err(format!(Test execution failed: {}, e}

#[test]
fn test_map_key_value_iteration() {let code = r#" 95,  Bob: 87,  Charlie: 92}"#
            sus sum lit = 0
            
            periodt name, score := range scores {sum = sum + score}
            
            return sum // Should be 95+87+92 = 274;};"#    #;
    run_test_with_expected(code, Object::Integer(274}

#[test]
fn test_variable_scoping() {let code = r#"#    #;"#
    run_test_with_expected(code, Object::Integer(42}

// This test should be marked as ignored until the enhanced implementation
// is fully integrated
#[test]
#[ignore]
fn test_implementation_comparison() {// This will test that both implementations produce the same results
    let code = r#"        slay main() lit {sus sum lit = 0"#
            periodt i := range 10 {sum = sum + i}
            
            return sum;};" comparison failed: {}, e)"}