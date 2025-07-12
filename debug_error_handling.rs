use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <cursed_file>", args[0]);
        return;
    }

    let file_path = &args[1];
    
    match cursed::main::run_file(file_path) {
        Ok(result) => {
            println!("Execution completed successfully");
            println!("Result: {:?}", result);
        }
        Err(e) => {
            eprintln!("Error during execution: {:?}", e);
            std::process::exit(1);
        }
    }
}
