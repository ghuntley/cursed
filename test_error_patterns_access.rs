// Test if error_patterns module is accessible
use cursed::types::result::error_patterns;

fn main() {
    // Test basic error pattern access
    let parse_err = error_patterns::parse_error::<i32>("syntax error", 10, 5);
    println!("Parse error created successfully: {:?}", parse_err);

    let runtime_err = error_patterns::runtime_error::<i32>("division by zero");
    println!("Runtime error created successfully: {:?}", runtime_err);

    let type_err = error_patterns::type_error::<i32>("type mismatch");  
    println!("Type error created successfully: {:?}", type_err);

    println!("ERROR_PATTERNS module is accessible!");
}
