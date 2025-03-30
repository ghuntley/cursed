use crate::error::Error;
use crate::lexer;
use crate::parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::history::DefaultHistory;
use log::info;
use inkwell::context::Context;
use crate::codegen::llvm::LlvmCodeGenerator;

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
    
    // Create LLVM context for the REPL session
    let context = Context::create();
    
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
                let mut lexer = lexer::Lexer::new(&line);
                
                // Create a parser for the lexer
                let mut parser = match parser::Parser::new(&mut lexer) {
                    Ok(parser) => parser,
                    Err(e) => {
                        eprintln!("Parser initialization error: {}", e);
                        continue;
                    }
                };
                
                // Parse the program
                let program = match parser.parse_program() {
                    Ok(prog) => prog,
                    Err(e) => {
                        eprintln!("Parser error: {}", e);
                        continue;
                    }
                };
                
                // Check for parser errors
                if !parser.errors().is_empty() {
                    for err in parser.errors() {
                        eprintln!("Parser error: {}", err);
                    }
                    continue;
                }
                
                // Create LLVM code generator for this expression
                // Provide a dummy path for REPL context
                let repl_dummy_path = std::path::PathBuf::from("./repl_line.csd");
                let mut code_gen = LlvmCodeGenerator::new(&context, "repl", repl_dummy_path);
                
                // Generate LLVM IR
                match code_gen.compile(&program) {
                    Ok(()) => {
                        // Print the generated LLVM IR (for now, eventually we would JIT execute)
                        println!("Generated LLVM IR:");
                        println!("{}", code_gen.module().print_to_string().to_string());
                        
                        // TODO: Add JIT execution once ready
                    },
                    Err(e) => {
                        eprintln!("Code generation error: {}", e);
                        continue;
                    }
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
    println!("  sus x = 5;          - Define a variable");
    println!("  sus y = 10;         - Define another variable");
    println!("  x + y;               - Evaluate an expression");
    println!("  yolo x + y;          - Return a value");
    println!();
    println!("Running Programs Outside REPL:");
    println!("  ./cursed file.csd           - Run a CURSED program from a file");
    println!("  ./cursed -e \"sus x = 5;\"   - Execute code from command line");
    println!("  cat file.csd | ./cursed -   - Run code from standard input");
    println!();
    println!("For more help, run: ./cursed --help");
}

/// Repeat a string n times
fn repeat_str(s: &str, n: usize) -> String {
    s.repeat(n)
} 