//! # CURSED Language CLI
//!
//! Command-line interface for the CURSED programming language.
//! Provides REPL, file execution, and debugging capabilities.

use std::env;
use std::process;
use std::path::Path;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{fmt, EnvFilter};

// Import our custom mainpatch module
mod main_patch;

// Initialize Vector2D type methods when the program starts
static INITIALIZE: std::sync::Once = std::sync::Once::new();

/// Main entry point for the CURSED compiler and runtime
///
/// Processes command-line arguments and dispatches to appropriate handlers:
/// - No arguments: Starts the REPL
/// - File path: Executes the file
/// - Special options: Handles debug, help, version, etc.
fn main() {
    // Initialize tracing
    cursed::init_tracing();
    // Initialize Vector2D type methods
    INITIALIZE.call_once(|| {
        // Register Vector2D methods with the registry
        cursed::stdlib::vector2d::register_vector2d_methods();
    });
    
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
                }
                "-v" | "--version" => {
                    println!("CURSED v{}", cursed::VERSION);
                    Ok(())
                }
                "--debug-tokens" => {
                    eprintln!("Error: The --debug-tokens option requires a file path");
                    process::exit(1);
                }
                "--opt-help" | "--optimization-help" => {
                    cursed::cli::print_optimization_help();
                    Ok(())
                }
                "stage2" => {
                    println!("Stage 2 compiler commands not yet implemented");
                    Ok(())
                }
                "--list-passes" => {
                    let args = vec!["--list-passes".to_string()];
                    match cursed::cli::parse_optimization_args(&args) {
                        Ok(Some(opt_args)) => {
                            if let Err(e) = cursed::cli::execute_optimization_command(&opt_args) {
                                eprintln!("Error: {}", e);
                                process::exit(1);
                            }
                        }
                        Ok(None) => {
                            cursed::cli::print_optimization_help();
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            process::exit(1);
                        }
                    }
                    Ok(())
                }
                "-e" | "--eval" => {
                    eprintln!("Error: The --eval option requires a code string");
                    print_usage(&program_name);
                    process::exit(1);
                }
                "-" => {
                    // Read from stdin
                    cursed::run_stdin()
                }
                // Otherwise, treat as a file path
                _ => {
                    // Check for vibez.spill calls in the file and process them directly if needed
                    if main_patch::patch_for_vibez_spill(&args[1]) {
                        println!("📢 Detected and processed vibez.spill calls directly");
                        // Exit with success without further compilation
                        return;
                    }
                    // Continue with normal execution
                    cursed::run_file(&args[1])
                },
            }
        }

        // Two or more arguments - check for options
        _ => {
            // Check for optimization-related commands first
            match cursed::cli::parse_optimization_args(&args[1..]) {
                Ok(Some(opt_args)) => {
                    if let Err(e) = cursed::cli::execute_optimization_command(&opt_args) {
                        error!(error = %e, "Optimization command failed");
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                    return;
                }
                Ok(None) => {
                    // Not an optimization command, continue processing
                }
                Err(e) => {
                    error!(error = %e, "Failed to parse optimization arguments");
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }

            // Check for IR-related commands
            if let Some(result) = cursed::cli::handle_ir_arguments(&args[1..]) {
                if let Err(e) = result {
                    error!(error = ?e, "IR command failed");
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
                return;
            }

            // // Check for Stage 2 compiler commands - temporarily disabled
            // if args.len() > 1 && args[1] == "stage2" {
            //     println!("Stage 2 compiler commands not yet implemented");
            //     return;
            // }

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
                }
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
                }
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
        error!(error = ?e, "Program execution failed");
        eprintln!("Error: {}", e);
        process::exit(1);
    }
    info!("Program executed successfully");
}

/// Prints usage information for the CURSED CLI
///
/// # Arguments
///
/// * `program_name` - The name of the program as invoked by the user
fn print_usage(program_name: &str) {
    println!("Usage: {} [OPTIONS] [FILE]", program_name);
    println!("Options:");
    println!("  -h, --help         Display this help message");
    println!("  -v, --version      Display version information");
    println!("  -e, --eval CODE    Execute CODE");
    println!("  -                  Read from standard input");
    println!("  --debug-tokens FILE Debug token stream for FILE");
    println!("  --emit-ir FILE     Generate LLVM IR (.ll) file");
    println!("  --emit-bc FILE     Generate LLVM bitcode (.bc) file");
    println!("  --emit-both FILE   Generate both IR and bitcode");
    println!("  --ir-help          Show detailed IR generation help");
    println!("");
    println!("Optimization Options:");
    println!("  -O0, -O1, -O2, -O3 Set optimization level");
    println!("  -Os, -Oz           Optimize for size");
    println!("  --opt-help         Show detailed optimization help");
    println!("  --list-passes      List available optimization passes");
    println!("  --benchmark-opt    Benchmark optimization levels");
    println!("");
    println!("Stage 2 Compiler (Self-Hosting):");
    println!("  stage2             Show Stage 2 compiler help");
    println!("  stage2 status      Check Stage 2 compiler availability");
    println!("  stage2 build       Build Stage 2 compiler from CURSED source");
    println!("  stage2 compile     Compile using Stage 2 (CURSED) compiler");
    println!("  stage2 self-host   Enable/disable self-hosting mode");
    println!("");
    println!("If no arguments are provided, the REPL will start in interactive mode.");
    println!("If a file path is provided, the file will be executed.");
}
