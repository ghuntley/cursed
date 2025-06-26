//! CURSED REPL binary

use std::env;
use std::process;
use std::io::{self, Write};
use cursed;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--version" => {
                println!("CURSED REPL {} - Interactive Mode", cursed::VERSION);
                return;
            }
            "--help" => {
                println!("CURSED REPL - Interactive Programming Environment");
                println!("Usage:");
                println!("  {} [--version] [--help]", args[0]);
                return;
            }
            _ => {
                eprintln!("Unknown option: {}", args[1]);
                process::exit(1);
            }
        }
    }
    
    // Start a simple REPL
    println!("CURSED REPL - Interactive Programming Environment");
    println!("Type 'exit' to quit");
    
    let mut session_manager = match cursed::repl::SessionManager::new() {
        Ok(sm) => sm,
        Err(e) => {
            eprintln!("REPL Initialization Error: {}", e);
            process::exit(1);
        }
    };
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        
        let input = input.trim();
        if input == "exit" || input == "quit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        match cursed::execute_repl_code(input, &mut session_manager) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
