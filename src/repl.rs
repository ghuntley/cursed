use crate::error::Error;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vm::VM;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::history::DefaultHistory;
use log::info;

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
                
                // Add the line to history if it's not empty
                if !line.trim().is_empty() {
                    let _ = rl.add_history_entry(line.as_str());
                }
                
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
    println!("REPL Commands:");
    println!("  exit/quit  - Exit the REPL");
    println!("  help       - Display this help message");
    println!();
    println!("Basic Examples:");
    println!("  vibe x = 5;          - Define a variable");
    println!("  vibe y = 10;         - Define another variable");
    println!("  x + y;               - Evaluate an expression");
    println!("  yolo x + y;          - Print a value");
    println!();
    println!("Running Programs Outside REPL:");
    println!("  ./cursed file.csd           - Run a CURSED program from a file");
    println!("  ./cursed -e \"vibe x = 5;\"   - Execute code from command line");
    println!("  cat file.csd | ./cursed -   - Run code from standard input");
    println!();
    println!("For more help, run: ./cursed --help");
}

/// Repeat a string n times
fn repeat_str(s: &str, n: usize) -> String {
    s.repeat(n)
} 