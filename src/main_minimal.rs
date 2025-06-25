use crate::error_types::Error;
#!/usr/bin/env rust
// CURSED Programming Language CLI (Truly Minimal Build)
// 
// Minimal command-line interface for core CURSED language functionality.
// Only provides basic parsing and syntax checking.

use clap::{Arg, ArgAction, Command};
use std::process;

use cursed::prelude::*;

fn main() {
    // Initialize the minimal CURSED runtime
    cursed::init();

    let app = build_minimal_cli();
    let matches = app.get_matches();

    let result = match matches.subcommand() {
        Some(("run", sub_matches)) => handle_run_command(sub_matches),
        Some(("check", sub_matches)) => handle_check_command(sub_matches),
        Some(("format", sub_matches)) => handle_format_command(sub_matches),
        Some(("tokenize", sub_matches)) => handle_tokenize_command(sub_matches),
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

fn build_minimal_cli() -> Command {
    Command::new("cursed")
        .about("CURSED Programming Language - Truly Minimal Build")
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
                .about("Parse and display CURSED source files")
                .arg(
                    Arg::new("file")
                        .help("CURSED source file to parse")
                        .required(true)
                        .value_name("FILE")
                )
        )
        .subcommand(
            Command::new("check")
                .about("Check CURSED source for syntax errors")
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
        )
        .subcommand(
            Command::new("tokenize")
                .about("Tokenize CURSED source files")
                .arg(
                    Arg::new("file")
                        .help("CURSED source file to tokenize")
                        .required(true)
                        .value_name("FILE")
                )
        )
}

fn handle_run_command(matches: &clap::ArgMatches) -> Result<(), Error> {
    let file = matches.get_one::<String>("file").unwrap();
    
    println!("🚀 Parsing CURSED program (minimal): {}", file);
    
    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Parse the file
    cursed::run_file(file)?;
    
    println!("✅ Program parsed successfully!");
    Ok(())
}

fn handle_check_command(matches: &clap::ArgMatches) -> Result<(), Error> {
    let file = matches.get_one::<String>("file").unwrap();
    
    println!("🔍 Checking CURSED program (minimal): {}", file);

    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Read and check source
    let source = std::fs::read_to_string(file)?;
    cursed::check(&source)?;
    
    println!("✅ Check completed successfully!");
    Ok(())
}

fn handle_format_command(matches: &clap::ArgMatches) -> Result<(), Error> {
    let file_opt = matches.get_one::<String>("file");
    let check_only = matches.get_flag("check");
    
    if let Some(file) = file_opt {
        println!("📝 Formatting CURSED file (minimal): {}", file);

        // Check if file exists
        if !std::path::Path::new(file).exists() {
            return Err(format!("File not found: {}", file).into());
        }

        // Read and format source
        let source = std::fs::read_to_string(file)?;
        let formatted = cursed::format(&source)?;
        
        if check_only {
            if source == formatted {
                println!("✅ File is properly formatted");
            } else {
                println!("❌ File needs formatting");
                return Err("File is not properly formatted".into());
            }
        } else {
            println!("{}", formatted);
        }
    } else {
        // Format from stdin
        println!("📝 Formatting CURSED source from stdin (minimal):");
        
        use std::io::{self, Read};
        let mut source = String::new();
        io::stdin().read_to_string(&mut source)?;
        
        let formatted = cursed::format(&source)?;
        println!("{}", formatted);
    }
    
    Ok(())
}

fn handle_tokenize_command(matches: &clap::ArgMatches) -> Result<(), Error> {
    let file = matches.get_one::<String>("file").unwrap();
    
    println!("🔤 Tokenizing CURSED program: {}", file);

    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Read and tokenize source
    let source = std::fs::read_to_string(file)?;
    let tokens = cursed::tokenize(&source)?;
    
    println!("Found {} tokens:", tokens.len());
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?} '{}'", i + 1, token.token_type, token.literal);
    }
    
    Ok(())
}
