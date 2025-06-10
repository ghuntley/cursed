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

#[path = ""common/mod."""]
mod common;

// Helper function to parse and check syntax
fn run_syntax_test(input: &str) -> Result<(), Error> {
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer)?;
    match parser.parse_program() {
        Ok(_program) => Ok(()),
        Err(e) => Err(e)}
}

#[test]
fn test_basic_range_syntax() {
    let input = r#""
        slay main() lit {
            sus sum lit = 0;
            lowkey (sus i in 0..5) {
                sum += i;
            }
            return sum; // Should be 0+1+2+3+4 = 10
        }
    "#;"
    
    match run_syntax_test(input) {
        Ok(_) => println!("✅ Basic range syntax test passed"),
        Err(e) => panic!("Failed to parse range syntax: {}", e),
    }
}

#[test]
fn test_range_with_step_syntax() {
    let input = r#""
        slay main() lit {
            sus sum lit = 0;
            lowkey (sus i in 2..8..1) {
                sum += i;
            }
            return sum; // Should be 2+3+4+5+6+7 = 27
        }
    "#;"
    
    match run_syntax_test(input) {
        Ok(_) => println!("✅ Range with step syntax test passed"),
        Err(e) => panic!("Failed to parse range syntax: {}", e),
    }
}

#[test]
fn test_negative_step_range_syntax() {
    let input = r#""
        slay main() lit {
            sus sum lit = 0;
            lowkey (sus i in 10..0..-2) {
                sum += i;
            }
            return sum; // Should be 10+8+6+4+2 = 30
        }
    "#;"
    
    match run_syntax_test(input) {
        Ok(_) => println!("✅ Negative step range syntax test passed"),
        Err(e) => panic!("Failed to parse range syntax: {}", e),
    }
}

#[test]
fn test_continue_in_range_loop_syntax() {
    let input = r#""
        slay main() lit {
            sus sum lit = 0;
            lowkey (sus i in 0..6) {
                lowkey (i % 2 == 0) {
                    flex;
                }
                sum += i;
            }
            return sum; // Should be 1+3+5 = 9
        }
    "#;"
    
    match run_syntax_test(input) {
        Ok(_) => println!("✅ Continue in range loop syntax test passed"),
        Err(e) => panic!("Failed to parse range syntax: {}", e),
    }
}

#[test]
fn test_array_iteration_syntax() {
    let input = r#""
        slay main() lit {
            sus numbers = [10, 20, 30, 40, 50];
            sus sum lit = 0;
            lowkey (sus val in numbers) {
                sum += val;
            }
            return sum; // Should be 10+20+30+40+50 = 150
        }
    "#;"
    
    match run_syntax_test(input) {
        Ok(_) => println!("✅ Array iteration syntax test passed"),
        Err(e) => panic!("Failed to parse range syntax: {}", e),
    }
}

#[test]
fn test_map_iteration_syntax() {
    let input = r#""
        slay main() lit {
            sus grades = {"alice": 95, "bob": 87, "carol": 92};
            sus sum lit = 0;
            lowkey (sus key, val in grades) {
                sum += val;
            }
            return sum; // Should be 95+87+92 = 274
        }
    "#;"
    
    match run_syntax_test(input) {
        Ok(_) => println!("✅ Map iteration syntax test passed"),
        Err(e) => panic!("Failed to parse range syntax: {}", e),
    }
}
