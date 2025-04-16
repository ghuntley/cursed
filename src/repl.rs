use crate::ast::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;
use crate::lexer;
use crate::parser;
use inkwell::context::Context;
use log::info;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;

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
#[tracing::instrument(level = "info")]
pub fn start_repl() -> Result<(), Error> {
    tracing::info!("Starting REPL interactive mode");
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
    println!("Note: Parser implementation is incomplete - only token display is functional.");

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
                    }
                    "help" => {
                        print_help();
                        continue;
                    }
                    _ => {}
                }

                // Create a lexer for the input
                println!("🔍 Lexical Analysis...");
                let mut lexer = lexer::Lexer::new(&line);

                // Create a parser for the lexer
                println!("🔨 Parsing...");
                let mut parser = match parser::Parser::new(&mut lexer) {
                    Ok(parser) => parser,
                    Err(e) => {
                        eprintln!("❌ Parser initialization error: {}", e);
                        continue;
                    }
                };

                // Parse the program
                let program = match parser.parse_program() {
                    Ok(prog) => prog,
                    Err(e) => {
                        eprintln!("❌ Parser error: {}", e);
                        continue;
                    }
                };

                // Check for parser errors
                if !parser.errors().is_empty() {
                    println!("❌ Parser found {} errors", parser.errors().len());
                    for err in parser.errors() {
                        eprintln!("Parser error: {}", err);
                    }
                    continue;
                }

                println!("✅ Successfully parsed program");
                println!("📊 Program structure:\n{}", program.string());

                // Create LLVM code generator for this expression
                // Provide a dummy path for REPL context
                println!("🏗️ Setting up LLVM code generation...");
                let repl_dummy_path = std::path::PathBuf::from("./repl_line.csd");
                let mut code_gen = LlvmCodeGenerator::new(&context, "repl", repl_dummy_path);

                // Generate LLVM IR
                println!("🔧 Compiling to LLVM IR...");
                match code_gen.compile(&program) {
                    Ok(()) => {
                        println!("✅ Compilation successful");
                        // Print the generated LLVM IR
                        println!("📄 Generated LLVM IR:");
                        println!("{}", code_gen.module().print_to_string().to_string());

                        // Execute the code using JIT
                        println!("🚀 Executing code using JIT...");
                        match code_gen
                            .module()
                            .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
                        {
                            Ok(execution_engine) => {
                                // Get main function
                                unsafe {
                                    match execution_engine
                                        .get_function::<unsafe extern "C" fn()>("main")
                                    {
                                        Ok(main_fn) => {
                                            tracing::info!("REPL executing main function");
                                            println!("📌 Function 'main' found, executing...");
                                            println!("--- Execution Output ---");
                                            main_fn.call();
                                            println!("------------------------");
                                            tracing::info!("REPL execution completed successfully");
                                            println!("✅ Execution completed successfully");
                                        }
                                        Err(e) => {
                                            println!(
                                                "⚠️ Function 'main' not found in the module: {}",
                                                e
                                            );
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to create execution engine: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!(error = ?e, "REPL compilation failed");
                        eprintln!("❌ Compilation failed: {}", e);
                        continue;
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
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
