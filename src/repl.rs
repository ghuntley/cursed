use crate::prelude::*;
use crate::error::Error;
use crate::lexer::Lexer;
use crate::parser_impl::Parser;
use crate::compiler::Compiler;
use crate::vm::VM;
use log::{debug, info};
use rustyline::error::ReadlineError;
use rustyline::{Editor};
use rustyline::history::DefaultHistory;
use std::borrow::Cow;
use std::path::PathBuf;

/// REPL header to display when starting interactive mode
const REPL_HEADER: &str = r#"
  ____ _   _ ____  ____  _____ ____  
 / ___| | | |  _ \/ ___|| ____|  _ \ 
| |   | | | | |_) \___ \|  _| | | | |
| |___| |_| |  _ < ___) | |___| |_| |
 \____|\___/|_| \_\____/|_____|____/ 
                                     
CURSED Programming Language - Gen Z slang meets Go-like grammar
Type 'exit' or 'quit' to exit the REPL, 'help' for commands.
"#;

/// Start the REPL interactive mode
pub fn start_repl() -> std::result::Result<(), Error> {
    info!("Starting REPL interactive mode");
    println!("{}", REPL_HEADER);

    // Initialize the REPL history
    let mut rl = Editor::<(), DefaultHistory>::new().map_err(|e| Error::Unknown(e.to_string()))?;
    let history_path = dirs::home_dir()
        .map(|mut path| {
            path.push(".cursed_history");
            path
        })
        .unwrap_or_default();

    // Load REPL history if it exists
    if rl.load_history(&history_path).is_err() {
        debug!("No previous history found");
    }

    // REPL loop
    loop {
        match rl.readline("cursed> ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                
                // Exit the REPL if the user enters "exit" or "quit"
                if line.is_empty() {
                    continue;
                }
                
                // Remove leading/trailing whitespace
                let input = line.as_str();
                let trimmed = input.trim();
                
                if trimmed == "exit" || trimmed == "quit" {
                    println!("Goodbye!");
                    break;
                }

                match trimmed {
                    "help" => {
                        print_help();
                    }
                    _ => {
                        if let Err(err) = eval(trimmed) {
                            eprintln!("Error: {}", err);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C pressed, type 'exit' to quit");
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D pressed, exiting");
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }

    // Save REPL history
    if let Err(err) = rl.save_history(&history_path) {
        debug!("Failed to save history: {}", err);
    }

    Ok(())
}

/// Print the help information
fn print_help() {
    println!("Available commands:");
    println!("  help       - Show this help message");
    println!("  exit, quit - Exit the REPL");
    println!("");
    println!("Enter CURSED code to evaluate it immediately.");
}

/// Evaluate a line of code
fn eval(input: &str) -> std::result::Result<(), Error> {
    debug!("Evaluating input: {}", input);

    // Create a lexer for the input
    let lexer = Lexer::new(input);
    
    // Create a parser with the lexer
    let mut parser = Parser::new(lexer);
    
    // Parse the program
    let program = parser.parse_program()?;
    
    // Create a compiler
    let mut compiler = Compiler::new();
    
    // Compile the program
    let bytecode = compiler.compile(program)?;
    
    // Create a VM with the bytecode
    let mut vm = VM::new(bytecode);
    
    // Run the VM
    let result = vm.run()?;
    
    // Print the result if there is one
    if let Some(obj) = result {
        println!("{}", obj);
    }
    
    Ok(())
} 