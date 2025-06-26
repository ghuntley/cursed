//! CURSED Debug binary

use std::env;
use std::process;
use cursed;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <file.csd> [--version] [--help]", args[0]);
        process::exit(1);
    }
    
    match args[1].as_str() {
        "--version" => {
            println!("CURSED Debug {} - Debug Mode Compiler", cursed::VERSION);
        }
        "--help" => {
            println!("CURSED Debug - Debug Information Compiler");
            println!("Usage:");
            println!("  {} <file.csd>    Compile with debug information", args[0]);
            println!("  {} --version     Show version", args[0]);
            println!("  {} --help        Show this help", args[0]);
        }
        filename => {
            println!("Debug compiling: {}", filename);
            if let Err(e) = cursed::compile_to_ir_with_optimization(
                &std::fs::read_to_string(filename).unwrap_or_else(|e| {
                    eprintln!("Error reading file: {}", e);
                    process::exit(1);
                }),
                Some("debug")
            ) {
                eprintln!("Debug Compilation Error: {}", e);
                process::exit(1);
            }
            println!("Debug compilation successful!");
        }
    }
}
