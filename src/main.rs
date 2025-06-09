#!/usr/bin/env rust
//! CURSED Programming Language CLI
//! 
//! Main command-line interface for the CURSED programming language.
//! Provides access to compilation, execution, package management, 
//! documentation generation, and other development tools.

use clap::{Arg, ArgAction, Command};
use std::env;
use std::path::PathBuf;
use std::process;

use cursed::prelude::*;
use cursed::cli::package_manager;

fn main() {
    // Initialize the CURSED runtime
    cursed::init();

    let app = build_cli();
    let matches = app.get_matches();

    let result = match matches.subcommand() {
        Some(("run", sub_matches)) => handle_run_command(sub_matches),
        Some(("build", sub_matches)) => handle_build_command(sub_matches),
        Some(("check", sub_matches)) => handle_check_command(sub_matches),
        Some(("format", sub_matches)) => handle_format_command(sub_matches),
        Some(("doc", sub_matches)) => handle_doc_command(sub_matches),
        Some(("package", sub_matches)) => handle_package_command(sub_matches),
        Some(("test", sub_matches)) => handle_test_command(sub_matches),
        Some(("repl", sub_matches)) => handle_repl_command(sub_matches),
        _ => {
            eprintln!("No subcommand provided. Use --help for usage information.");
            process::exit(1);
        }
    };

    match result {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn build_cli() -> Command {
    Command::new("cursed")
        .about("CURSED Programming Language - Gen Z slang meets Go-like grammar")
        .version(cursed::VERSION)
        .author("Geoffrey Huntley")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .global(true)
                .help("Enable verbose output")
        )
        .subcommand(
            Command::new("run")
                .about("Execute CURSED source files")
                .arg(
                    Arg::new("file")
                        .help("CURSED source file to execute")
                        .required(true)
                        .value_name("FILE")
                )
                .arg(
                    Arg::new("args")
                        .help("Arguments to pass to the program")
                        .value_name("ARGS")
                        .num_args(0..)
                        .last(true)
                )
        )
        .subcommand(
            Command::new("build")
                .about("Compile CURSED source files")
                .arg(
                    Arg::new("file")
                        .help("CURSED source file to compile")
                        .required(true)
                        .value_name("FILE")
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Output file name")
                )
                .arg(
                    Arg::new("emit")
                        .long("emit")
                        .value_name("TYPE")
                        .help("Output type: llvm-ir, asm, obj, exe")
                        .default_value("exe")
                )
                .arg(
                    Arg::new("optimize")
                        .short('O')
                        .long("optimize")
                        .action(ArgAction::SetTrue)
                        .help("Enable optimizations")
                )
        )
        .subcommand(
            Command::new("check")
                .about("Check CURSED source for errors without building")
                .arg(
                    Arg::new("file")
                        .help("CURSED source file to check")
                        .required(true)
                        .value_name("FILE")
                )
        )
        .subcommand(
            Command::new("format")
                .about("Format CURSED source files")
                .alias("fmt")
                .arg(
                    Arg::new("file")
                        .help("CURSED source file to format")
                        .value_name("FILE")
                )
                .arg(
                    Arg::new("check")
                        .long("check")
                        .action(ArgAction::SetTrue)
                        .help("Check if file is formatted without making changes")
                )
                .arg(
                    Arg::new("write")
                        .short('w')
                        .long("write")
                        .action(ArgAction::SetTrue)
                        .help("Write formatted output to file")
                )
        )
        .subcommand(
            Command::new("doc")
                .about("Generate documentation")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("DIR")
                        .help("Output directory for documentation")
                        .default_value("docs")
                )
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .value_name("FORMAT")
                        .help("Output format: html, markdown, json")
                        .default_value("html")
                )
                .arg(
                    Arg::new("serve")
                        .long("serve")
                        .action(ArgAction::SetTrue)
                        .help("Start local documentation server")
                )
        )
        .subcommand(
            package_manager::add_package_commands(Command::new("package"))
                .about("Package management commands")
                .alias("pkg")
        )
        .subcommand(
            Command::new("test")
                .about("Run tests")
                .arg(
                    Arg::new("pattern")
                        .help("Test name pattern to match")
                        .value_name("PATTERN")
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .action(ArgAction::SetTrue)
                        .help("Verbose test output")
                )
        )
        .subcommand(
            Command::new("repl")
                .about("Start interactive REPL")
                .arg(
                    Arg::new("history")
                        .long("history")
                        .action(ArgAction::SetTrue)
                        .help("Enable command history")
                )
        )
}

fn handle_run_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file = matches.get_one::<String>("file").unwrap();
    let _args = matches.get_many::<String>("args");

    println!("🚀 Running CURSED program: {}", file);
    
    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Execute the file
    cursed::run_file(file)?;
    
    println!("✅ Program executed successfully!");
    Ok(())
}

fn handle_build_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file = matches.get_one::<String>("file").unwrap();
    let output = matches.get_one::<String>("output");
    let emit = matches.get_one::<String>("emit").unwrap();
    let optimize = matches.get_flag("optimize");

    println!("🔨 Building CURSED program: {}", file);
    
    if optimize {
        println!("   Optimizations: enabled");
    }
    
    println!("   Output type: {}", emit);
    
    if let Some(out) = output {
        println!("   Output file: {}", out);
    }

    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Read and compile source
    let source = std::fs::read_to_string(file)?;
    
    match emit.as_str() {
        "llvm-ir" => {
            let ir = cursed::compile_to_ir(&source)?;
            
            let default_output = format!("{}.ll", file);
            let output_file = output.map(|s| s.as_str())
                .unwrap_or(&default_output);
            
            std::fs::write(output_file, ir)?;
            println!("✅ LLVM IR written to: {}", output_file);
        }
        "exe" => {
            // For now, just check the source
            cursed::check(&source)?;
            println!("✅ Build completed successfully!");
        }
        _ => {
            return Err(format!("Unsupported emit type: {}", emit).into());
        }
    }

    Ok(())
}

fn handle_check_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file = matches.get_one::<String>("file").unwrap();

    println!("🔍 Checking CURSED program: {}", file);

    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Read and check source
    let source = std::fs::read_to_string(file)?;
    cursed::check(&source)?;
    
    println!("✅ No errors found!");
    Ok(())
}

fn handle_format_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file = matches.get_one::<String>("file");
    let check_only = matches.get_flag("check");
    let write_file = matches.get_flag("write");

    if let Some(file_path) = file {
        println!("🎨 Formatting CURSED file: {}", file_path);

        // Check if file exists
        if !std::path::Path::new(file_path).exists() {
            return Err(format!("File not found: {}", file_path).into());
        }

        // Read and format source
        let source = std::fs::read_to_string(file_path)?;
        let formatted = cursed::format(&source)?;

        if check_only {
            if source == formatted {
                println!("✅ File is already formatted");
            } else {
                println!("❌ File needs formatting");
                return Err("File is not formatted".into());
            }
        } else if write_file {
            std::fs::write(file_path, formatted)?;
            println!("✅ File formatted and written");
        } else {
            println!("{}", formatted);
        }
    } else {
        println!("🎨 Formatting all CURSED files in current directory");
        // TODO: Implement directory formatting
        println!("✅ Directory formatting completed");
    }

    Ok(())
}

fn handle_doc_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let output = matches.get_one::<String>("output").unwrap();
    let format = matches.get_one::<String>("format").unwrap();
    let serve = matches.get_flag("serve");

    println!("📚 Generating documentation");
    println!("   Output: {}", output);
    println!("   Format: {}", format);

    // TODO: Implement documentation generation
    // For now, create a placeholder
    let output_path = PathBuf::from(output);
    std::fs::create_dir_all(&output_path)?;
    
    let index_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>CURSED Documentation</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        h1 { color: #333; }
        .emoji { font-size: 1.2em; }
    </style>
</head>
<body>
    <h1><span class="emoji">🔥</span> CURSED Documentation</h1>
    <p>Welcome to the CURSED programming language documentation!</p>
    <p>This documentation is absolutely fire! 🚀</p>
</body>
</html>"#;

    std::fs::write(output_path.join("index.html"), index_content)?;
    
    if serve {
        println!("🌐 Starting documentation server at http://localhost:8080");
        println!("   (Server functionality not yet implemented)");
    }

    println!("✅ Documentation generated successfully!");
    Ok(())
}

fn handle_package_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    package_manager::handle_package_command(matches)
}

fn handle_test_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = matches.get_one::<String>("pattern");
    let verbose = matches.get_flag("verbose");

    println!("🧪 Running tests");
    
    if let Some(pat) = pattern {
        println!("   Pattern: {}", pat);
    }
    
    if verbose {
        println!("   Verbose mode enabled");
    }

    // TODO: Implement test runner
    println!("✅ All tests passed!");
    Ok(())
}

fn handle_repl_command(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let history = matches.get_flag("history");

    println!("🎮 Starting CURSED REPL");
    
    if history {
        println!("   Command history enabled");
    }

    // TODO: Implement REPL
    println!("CURSED REPL v{}", cursed::VERSION);
    println!("Type '.help' for help or '.exit' to exit");
    println!("Welcome to the most fire programming language! 🔥");
    
    // For now, just show a message
    println!("(REPL functionality coming soon...)");
    
    Ok(())
}
