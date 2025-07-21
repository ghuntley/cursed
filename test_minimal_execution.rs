use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.csd>", args[0]);
        return;
    }
    
    let filename = &args[1];
    println!("Testing minimal execution of: {}", filename);
    
    // Try to read the file and execute it directly
    let content = match std::fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };
    
    println!("File content: {}", content);
    println!("Execution would happen here...");
}
