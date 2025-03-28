use crate::error::{Error, ErrorReporter, SourceLocation};
use crate::lexer::Lexer;
use crate::parser_impl::Parser;
use crate::compiler::Compiler;
use crate::compiler::Bytecode;
use crate::vm::VM;
use crate::object::Object;
use crate::vm::ErrorLocation;
use std::io::{self, Write};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::history::DefaultHistory;
use log::{debug, info};

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
pub fn start_repl() -> Result<(), Error> {
    info!("Starting REPL interactive mode");
    println!("{}", REPL_HEADER);

    // Initialize the REPL history
    let mut rl = Editor::<(), DefaultHistory>::new().unwrap();
    
    // Load history from ~/.cursed_history
    if let Err(e) = rl.load_history("~/.cursed_history") {
        // This is okay, it might not exist
        println!("No previous history: {}", e);
    }
    
    // Welcome message
    println!("Welcome to CURSED v{}", crate::VERSION);
    println!("Type 'exit' or 'quit' to exit, 'help' for help.");
    
    // Main REPL loop
    loop {
        // Read
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }
                
                rl.add_history_entry(line.as_str());
                
                // Handle special commands
                match line.trim() {
                    "exit" | "quit" => {
                        println!("Goodbye! 👋");
                        break;
                    },
                    "help" => {
                        print_help();
                        continue;
                    },
                    _ => {}
                }
                
                // Create a lexer for the input
                let mut lexer = Lexer::new(&line);
                
                // Create a parser for the lexer
                let mut parser = Parser::new(&mut lexer)?;
                
                // Parse the program
                let program = parser.parse_program()?;
                
                // Check for parser errors
                if !parser.errors().is_empty() {
                    for err in parser.errors() {
                        eprintln!("Parser error: {}", err);
                    }
                    continue;
                }
                
                // Create a compiler
                let mut compiler = Compiler::new();
                
                // Compile the program
                let bytecode = compiler.compile_program(&program)?;
                
                // Create a VM
                let mut vm = VM::with_bytecode(bytecode);
                
                // Run the VM
                match vm.run() {
                    Ok(obj) => println!("{}", obj),
                    Err(e) => eprintln!("VM error: {}", e)
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }
    
    // Save history
    if let Err(e) = rl.save_history("~/.cursed_history") {
        eprintln!("Error saving history: {}", e);
    }
    
    Ok(())
}

/// Print the help information
fn print_help() {
    println!("CURSED Language Help");
    println!("====================");
    println!("exit/quit - Exit the REPL");
    println!("help      - Display this help message");
    println!();
    println!("Examples:");
    println!("  sus x = 5;");
    println!("  sus y = 10;");
    println!("  x + y;");
}

/// Print an error message
fn print_error(error: &Error) {
    eprintln!("Error: {}", error);
    
    if let Some(location) = error.location() {
        print_error_location(&location);
    }
}

/// Print error location details
fn print_error_location(location: &SourceLocation) {
    eprintln!("  at line {}, column {}", location.line, location.column);
    
    if !location.source_line.is_empty() {
        eprintln!("     {}", location.source_line);
        
        // Highlight the error position with a caret
        let tokens: Vec<&str> = location.source_line.split_whitespace().collect();
        if !tokens.is_empty() {
            // Create spacing and point to the error with ^
            eprintln!("     {}^", repeat_str(" ", location.column - 1));
        }
    }
}

/// Repeat a string n times
fn repeat_str(s: &str, n: usize) -> String {
    s.repeat(n)
} 