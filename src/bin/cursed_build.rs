//! CURSED Build binary

use std::env;
use std::process;
use cursed;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <project> [--version] [--help]", args[0]);
        process::exit(1);
    }
    
    match args[1].as_str() {
        "--version" => {
            println!("CURSED Build {} - Project Build System", cursed::VERSION);
        }
        "--help" => {
            println!("CURSED Build - Project Build and Package System");
            println!("Usage:");
            println!("  {} <project>     Build a CURSED project", args[0]);
            println!("  {} --version     Show version", args[0]);
            println!("  {} --help        Show this help", args[0]);
        }
        project => {
            println!("Building project: {}", project);
            // For now, just try to compile the project file
            let project_file = format!("{}/main.csd", project);
            if let Err(e) = cursed::run_file(&project_file) {
                eprintln!("Build Error: {}", e);
                process::exit(1);
            }
            println!("Build successful!");
        }
    }
}
