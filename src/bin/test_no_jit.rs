fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.csd>", args[0]);
        std::process::exit(1);
    }
    
    let result = cursed::run_file_no_jit(&args[1]);
    match result {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
