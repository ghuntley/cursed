// Simple test to debug the identifier issue
use cursed::type_system::checker::TypeChecker;

fn main() {
    let mut checker = TypeChecker::new();
    
    // Test looking up variable 'a' 
    match checker.check_identifier("a") {
        Ok(type_expr) => {
            println!("Found type for 'a': {:?}", type_expr);
        }
        Err(error) => {
            println!("Error looking up 'a': {}", error.message);
        }
    }
    
    // Test looking up variable 'b'
    match checker.check_identifier("b") {
        Ok(type_expr) => {
            println!("Found type for 'b': {:?}", type_expr);
        }
        Err(error) => {
            println!("Error looking up 'b': {}", error.message);
        }
    }
    
    // Test looking up variable 'c'
    match checker.check_identifier("c") {
        Ok(type_expr) => {
            println!("Found type for 'c': {:?}", type_expr);
        }
        Err(error) => {
            println!("Error looking up 'c': {}", error.message);
        }
    }
}
