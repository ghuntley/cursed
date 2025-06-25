/// CURSED Programming Language CLI (Truly Minimal Build)
/// 
/// Minimal command-line interface for core CURSED language functionality.

use std::env;
use std::fs;
use std::process;
use cursed;

fn main() {
    // Initialize the minimal CURSED runtime
    cursed::init();

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <file.csd> | --version | --help", args[0]);
        process::exit(1);
    match args[1].as_str() {
        "--version" => {
            println!("CURSED {} - Minimal Build", cursed::VERSION);
        }
        "--help" => {
            println!("CURSED Programming Language - Minimal Build");
            println!("Usage:");
            println!("  {} <file.csd>    Run a CURSED program", args[0]);
            println!("  {} --version     Show version", args[0]);
            println!("  {} --help        Show this help", args[0]);
        }
        filename => {
            if let Err(e) = run_file(filename) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn run_file(filename: &str) -> cursed::Result<()> {
    let source = fs::read_to_string(filename)
        .map_err(|e| cursed::CursedError {
        })?;
    
    cursed::run(&source)
}
