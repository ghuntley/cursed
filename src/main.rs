// Minimal implementation for the main function
fn main() {
    println!("CURSED Programming Language v{}", cursed::VERSION);
    println!("Authors: {}", cursed::AUTHORS);
    println!("Description: {}", cursed::DESCRIPTION);
    
    if let Err(e) = cursed::run_repl() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
