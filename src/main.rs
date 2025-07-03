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
        println!("Usage: {} <file.csd> | --compile <file.csd> [-o <output>] | --version | --help", args[0]);
        process::exit(1);
    }
    
    match args[1].as_str() {
        "--version" => {
            println!("CURSED {} - Minimal Build", cursed::VERSION);
        }
        "--help" => {
            println!("CURSED Programming Language - Minimal Build");
            println!("Usage:");
            println!("  {} <file.csd>                Run a CURSED program", args[0]);
            println!("  {} --compile <file.csd>      Compile to executable", args[0]);
            println!("  {} --compile <file.csd> -o <output>  Compile with custom output name", args[0]);
            println!("  {} --version                 Show version", args[0]);
            println!("  {} --help                    Show this help", args[0]);
        }
        "--compile" => {
            if args.len() < 3 {
                eprintln!("Error: --compile requires a source file");
                process::exit(1);
            }
            
            let source_file = &args[2];
            let output_file = if args.len() > 4 && args[3] == "-o" {
                args[4].clone()
            } else {
                // Default output name: remove .csd extension and use as executable name
                source_file.strip_suffix(".csd").unwrap_or(source_file).to_string()
            };
            
            if let Err(e) = compile_file(source_file, &output_file) {
                eprintln!("Compilation error: {}", e);
                process::exit(1);
            }
        }
        filename => {
            if let Err(e) = run_file(filename) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn run_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(filename)?;
    
    cursed::run(&source).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn compile_file(source_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Compiling {} to {}", source_file, output_file);
    
    cursed::compile(source_file, output_file).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
