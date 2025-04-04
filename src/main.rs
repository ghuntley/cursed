use std::env;
use std::process;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let program_name = args.get(0).unwrap_or(&String::from("cursed")).clone();
    
    // Welcome message (only for interactive mode)
    if args.len() <= 1 {
        println!("CURSED Programming Language v{}", cursed::VERSION);
        println!("Authors: {}", cursed::AUTHORS);
        println!("Description: {}", cursed::DESCRIPTION);
    }
    
    // Parse command line arguments
    let result = match args.len() {
        // No arguments - start interactive REPL
        1 => cursed::run_repl(),
        
        // Single argument could be a file path or a flag
        2 => {
            match args[1].as_str() {
                // Check for flags
                "-h" | "--help" => {
                    print_usage(&program_name);
                    Ok(())
                },
                "-v" | "--version" => {
                    println!("CURSED v{}", cursed::VERSION);
                    Ok(())
                },
                "--debug-tokens" => {
                    eprintln!("Error: The --debug-tokens option requires a file path");
                    process::exit(1);
                },
                "-e" | "--eval" => {
                    eprintln!("Error: The --eval option requires a code string");
                    print_usage(&program_name);
                    process::exit(1);
                },
                "-" => {
                    // Read from stdin
                    cursed::run_stdin()
                },
                // Otherwise, treat as a file path
                _ => cursed::run_file(&args[1]),
            }
        },
        
        // Two or more arguments - check for options
        _ => {
            match args[1].as_str() {
                "--debug-tokens" => {
                    // Debug token stream for the specified file
                    if let Some(file_path) = args.get(2) {
                        match std::fs::read_to_string(file_path) {
                            Ok(input) => {
                                if let Err(err) = cursed::lexer::debug_tokens(&input) {
                                    eprintln!("Error debugging tokens: {}", err);
                                    process::exit(1);
                                }
                                Ok(())
                            }
                            Err(err) => {
                                eprintln!("Error reading file: {}", err);
                                process::exit(1);
                            }
                        }
                    } else {
                        eprintln!("Error: The --debug-tokens option requires a file path");
                        process::exit(1);
                    }
                },
                "-e" | "--eval" => {
                    // Execute code from -e argument
                    if let Some(code) = args.get(2) {
                        // Provide a dummy path for code from -e
                        let execute_path = std::path::PathBuf::from("./execute_arg.csd");
                        // Return the result directly
                        cursed::run_program(code, false, execute_path)
                    } else {
                        eprintln!("Error: The --eval option requires a code string");
                        process::exit(1);
                    }
                },
                _ => {
                    // If no recognized options, error
                    eprintln!("Error: Unrecognized arguments");
                    print_usage(&program_name);
                    process::exit(1);
                }
            }
        }
    };
    
    // Handle errors
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn print_usage(program_name: &str) {
    println!("Usage: {} [OPTIONS] [FILE]", program_name);
    println!("Options:");
    println!("  -h, --help         Display this help message");
    println!("  -v, --version      Display version information");
    println!("  -e, --eval CODE    Execute CODE");
    println!("  -                  Read from standard input");
    println!("  --debug-tokens FILE Debug token stream for FILE");
    println!("");
    println!("If no arguments are provided, the REPL will start in interactive mode.");
    println!("If a file path is provided, the file will be executed.");
}
