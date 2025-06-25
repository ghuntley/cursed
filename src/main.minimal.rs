use crate::error_types::Error;
#!/usr/bin/env rust
// CURSED Programming Language CLI (Minimal Build)
// 
// Minimal command-line interface for core CURSED language functionality.
// Provides basic compilation, execution, and checking capabilities.

use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;
use std::process;

use cursed::prelude::*;

fn main() {
    // Initialize the minimal CURSED runtime
    cursed::init();

    let app = build_minimal_cli();
    let matches = app.get_matches();

    let result = match matches.subcommand() {
        Some(("run", sub_matches)) => handle_run_command(sub_matches),
        Some(("build", sub_matches)) => handle_build_command(sub_matches),
        Some(("check", sub_matches)) => handle_check_command(sub_matches),
        Some(("format", sub_matches)) => handle_format_command(sub_matches),
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
        .about("CURSED Programming Language - Minimal Build")
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
                        .help("Output type: llvm-ir, check")
                        .default_value("check")
                )
                .arg(
                    Arg::new("opt-level")
                    .short('O')
                    .long("opt-level")
                    .value_name("LEVEL")
                    .help("Optimization level (O0, O1, O2, O3)")
                    .default_value("O0")
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
        )
}

fn handle_run_command(matches: &clap::ArgMatches) -> Result<(), Error> {
    let file = matches.get_one::<String>("file").unwrap();
    
    println!("🚀 Running CURSED program (minimal): {}", file);
    
    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Execute the file
    cursed::run_file(file)?;
    
    println!("✅ Program executed successfully!");
    Ok(())
}

fn handle_build_command(matches: &clap::ArgMatches) -> Result<(), Error> {
    let file = matches.get_one::<String>("file").unwrap();
    let output = matches.get_one::<String>("output");
    let emit = matches.get_one::<String>("emit").unwrap();
    let opt_level = matches.get_one::<String>("opt-level").unwrap();
    
    println!("🔨 Building CURSED program (minimal): {}", file);
    println!("   Output type: {}", emit);
    println!("   Optimization: {}", opt_level);
    
    if let Some(out) = output {
        println!("   Output file: {}", out);
    }

    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Read and compile source
    let source = std::fs::read_to_string(file)?;
    
    match emit.as_ref() {
        "llvm-ir" => {
            let ir = cursed::compile_to_ir_with_optimization(&source, Some(opt_level))?;
            
            let default_output = format!("{}.ll", file);
            let output_file = output.map(|s| s.as_str())
                .unwrap_or(&default_output);
            
            std::fs::write(output_file, ir)?;
            println!("✅ LLVM IR written to: {} ({})", output_file, opt_level);
        }
        "check" => {
            cursed::check(&source)?;
            println!("✅ Build check completed successfully!");
        }
        _ => {
            return Err(format!("Unsupported emit type: {} (minimal build supports: llvm-ir, check)", emit).into());
        }
    }

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
