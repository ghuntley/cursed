//! Main CURSED binary

use std::env;
use std::process;
use cursed;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <file.csd> | --version | --help", args[0]);
        process::exit(1);
    }
    
    match args[1].as_str() {
        "--version" => {
            println!("CURSED {} - Main Binary", cursed::VERSION);
        }
        "--help" => {
            println!("CURSED Programming Language - Main Binary");
            println!("Usage:");
            println!("  {} <file.csd>    Run a CURSED program", args[0]);
            println!("  {} --version     Show version", args[0]);
            println!("  {} --help        Show this help", args[0]);
        }
        filename => {
            if let Err(e) = cursed::run_file(filename) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}
