// Test sync execution without tokio
fn main() {
    let filename = "test_stack_overflow.csd";
    println!("Running {} synchronously without tokio", filename);
    
    match cursed::run_file(filename) {
        Ok(_) => println!("Execution completed successfully"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
