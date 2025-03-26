use crate::prelude::*;
use crate::error::Error;
use crate::lexer::Lexer;
use crate::parser_impl::Parser;
use crate::compiler::Compiler;
use crate::vm::{VM, ErrorLocation};
use crate::object::Object;
use log::{debug, info};
use rustyline::error::ReadlineError;
use rustyline::{Editor};
use rustyline::history::DefaultHistory;
use std::borrow::Cow;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use crate::memory::MemoryManager;
use crate::vm::constants::{STACK_SIZE, MAX_FRAMES, DEFAULT_MEMORY_SIZE, HEAP_SIZE, GC_SIZE};
use crate::compiler::{CompilationScope, Instruction, Opcode};
use std::collections::HashMap;

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
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(&line);
                let input = line.as_str();
                let trimmed = input.trim();
                
                if trimmed.is_empty() {
                    continue;
                }
                
                match trimmed {
                    "help" => {
                        print_help();
                        return Ok(())
                    },
                    "exit" | "quit" => {
                        println!("Goodbye!");
                        break;
                    },
                    _ => {
                        if let Err(e) = evaluate_input(trimmed) {
                            println!("Error: {}", e);
                        }
                        return Ok(())
                    }
                }
            },
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

/// Evaluate a CURSED expression
fn evaluate_input(input: &str) -> Result<(), Error> {
    // Create lexer and parser
    let lexer = create_lexer(input);
    let mut parser = create_parser(lexer);
    
    // Parse the input
    let program = parser.parse_program()?;
    
    // Check for parsing errors
    if !parser.errors().is_empty() {
        for err in parser.errors() {
            eprintln!("Parser error: {}", err);
        }
        return Err(Error::Parser {
            location: SourceLocation::new(0, 0),
            message: format!("Failed to parse input"),
        });
    }
    
    // Compile the program
    let mut compiler = create_compiler();
    let bytecode = compiler.compile_program(&program)?;
    
    // Create VM and run the program
    let mut vm = create_vm(bytecode);
    let result = vm.run()?;
    
    // Print the result
    println!("{}", result);
    
    Ok(())
}

/// Create a lexer for the given input
fn create_lexer(input: &str) -> Lexer {
    Lexer {
        input,
        position: 0,
        read_position: 0,
        ch: None,
        line: 1,
        column: 1,
    }
}

/// Create a parser from a lexer
fn create_parser(lexer: Lexer) -> Parser {
    Parser::new(lexer)
}

/// Create a compiler
fn create_compiler() -> Compiler {
    Compiler::new()
}

/// Create a VM with the given bytecode
fn create_vm(bytecode: Bytecode) -> VM {
    VM::new(bytecode)
}

/// Extract an error object from an Object value if it's an error
fn get_error_object(obj: &Object) -> Option<(&String, &Option<String>, &Vec<ErrorLocation>)> {
    match obj {
        Object::Error { message, error_type, stack_trace } => Some((message, error_type, stack_trace)),
        _ => None,
    }
}

/// Print the error trace for better debugging
fn print_error_trace(error: (&String, &Option<String>, &Vec<ErrorLocation>)) {
    let (message, error_type, stack_trace) = error;
    eprintln!("Error: {}", message);
    
    if let Some(err_type) = error_type {
        eprintln!("Type: {}", err_type);
    }
    
    if !stack_trace.is_empty() {
        eprintln!("\nStack trace:");
        for (i, location) in stack_trace.iter().enumerate() {
            eprintln!("  {}. at line {}, column {}", i+1, location.line, location.column);
            eprintln!("     {}", location.source_line);
            
            // Use standard library split_whitespace method directly to avoid recursive calls
            let tokens: Vec<&str> = location.source_line.as_str().split_whitespace().collect();
            
            // Print a caret under the exact position
            eprintln!("     {}^", " ".repeat(location.column - 1));
        }
    }
}

// Implementation of VM::run is now handled in vm/mod.rs 