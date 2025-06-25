use crate::error::CursedError;
#!/usr/bin/env rust
// CURSED Programming Language CLI (Truly Minimal Build)
// 
// Minimal command-line interface for core CURSED language functionality.
// Only provides basic parsing and syntax checking.

use clap::{Arg, ArgAction, Command};
use std::process;

use cursed::prelude::*;

fn main() {
        // TODO: implement
    }
    // Initialize the minimal CURSED runtime
    cursed::init();

    let app = build_minimal_cli();
    let matches = app.get_matches();

    let result = match matches.subcommand() {
        _ => {
            eprintln!("No subcommand provided. Use --help for usage information.");
            process::exit(1);
        }

    match result {
        Err(e) => {
            eprintln!("CursedError: {}", e);
            process::exit(1);
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
        .subcommand(
            Command::new("test")
                .about("Test minimal CURSED functionality")
        )
        .subcommand(
            Command::new("compile")
                .about("Compile CURSED source to LLVM IR or object file")
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
                    Arg::new("emit-llvm")
                        .long("emit-llvm")
                        .action(ArgAction::SetTrue)
                        .help("Emit LLVM IR instead of object file")
                )
                .arg(
                    Arg::new("executable")
                        .short('e')
                        .long("executable")
                        .action(ArgAction::SetTrue)
                        .help("Create executable instead of object file")
                )
        )
fn handle_run_command(matches: &clap::ArgMatches) -> crate::error::Result<()> {
    let file = matches.get_one::<String>("file").unwrap();
    
    println!("🚀 Parsing CURSED program (minimal): {}", file);
    
    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    // Parse the file
    cursed::run_file(file)?;
    
    println!("✅ Program parsed successfully!");
    Ok(())
fn handle_check_command(matches: &clap::ArgMatches) -> crate::error::Result<()> {
    let file = matches.get_one::<String>("file").unwrap();
    
    println!("🔍 Checking CURSED program (minimal): {}", file);

    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    // Read and check source
    let source = std::fs::read_to_string(file)?;
    cursed::check(&source)?;
    
    println!("✅ Check completed successfully!");
    Ok(())
fn handle_format_command(matches: &clap::ArgMatches) -> crate::error::Result<()> {
    let file_opt = matches.get_one::<String>("file");
    let check_only = matches.get_flag("check");
    
    if let Some(file) = file_opt {
        println!("📝 Formatting CURSED file (minimal): {}", file);

        // Check if file exists
        if !std::path::Path::new(file).exists() {
            return Err(format!("File not found: {}", file).into());
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
    Ok(())
fn handle_tokenize_command(matches: &clap::ArgMatches) -> crate::error::Result<()> {
    let file = matches.get_one::<String>("file").unwrap();
    
    println!("🔤 Tokenizing CURSED program: {}", file);

    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    // Read and tokenize source
    let source = std::fs::read_to_string(file)?;
    let tokens = cursed::tokenize(&source)?;
    
    println!("Found {} tokens:", tokens.len());
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?} '{}'", i + 1, token.token_type, token.literal);
    Ok(())

fn handle_test_command() -> crate::error::Result<()> {
    let source = r#"
facts x = 42;
facts name = "CURSED";
slay greet(name) {
    facts greeting = "Hello";
}
"#;

    println!("🔤 Testing Minimal CURSED Functionality");
    println!("Source code:");
    println!("{}", source);
    println!();

    // Test tokenization
    println!("🔍 Tokenizing...");
    match cursed::tokenize(source) {
        Ok(tokens) => {
            println!("✅ Found {} tokens:", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!("  {}: {:?} '{}'", i + 1, token.token_type, token.literal);
            }
        }
        Err(e) => {
            println!("❌ Tokenization failed: {}", e);
            return Err(e);
        }
    }
    println!();

    // Test parsing
    println!("🔍 Parsing...");
    match cursed::parse(source) {
        Ok(program) => {
            println!("✅ Parsed successfully!");
            println!("Program has {} statements:", program.statements.len());
            for (i, stmt) in program.statements.iter().enumerate() {
                println!("  {}: {:?}", i + 1, stmt);
            }
        }
        Err(e) => {
            println!("❌ Parsing failed: {}", e);
            return Err(e);
        }
    }
    println!();

    // Test basic functionality
    println!("🎯 Testing basic run functionality...");
    match cursed::run(source) {
        Ok(_) => {
            println!("✅ Run completed successfully!");
        }
        Err(e) => {
            println!("❌ Run failed: {}", e);
            return Err(e);
        }
    }
    println!();

    println!("🎉 Minimal CURSED functionality test completed!");
    Ok(())
fn handle_compile_command(matches: &clap::ArgMatches) -> crate::error::Result<()> {
    let file = matches.get_one::<String>("file").unwrap();
    let emit_llvm = matches.get_flag("emit-llvm");
    let executable = matches.get_flag("executable");
    
    println!("🔥 Compiling CURSED program: {}", file);
    
    // Check if file exists
    if !std::path::Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    // Parse the file first
    let program = cursed::parse_file(file)?;
    
    if emit_llvm {
        // Generate LLVM IR
        let output = matches.get_one::<String>("output")
            .map(|s| s.as_str())
            .unwrap_or("output.ll");
            
        println!("📄 Generating LLVM IR: {}", output);
        cursed::compile_to_llvm_ir(&program, file, output)?;
        println!("✅ LLVM IR generated: {}", output);
    } else if executable {
        // Generate executable
        let output = matches.get_one::<String>("output")
            .map(|s| s.as_str())
            .unwrap_or("output");
            
        println!("🚀 Compiling to executable: {}", output);
        cursed::compile_to_executable(&program, file, output)?;
        println!("✅ Executable generated: {}", output);
    } else {
        // Generate object file
        let output = matches.get_one::<String>("output")
            .map(|s| s.as_str())
            .unwrap_or("output.o");
            
        println!("🔧 Compiling to object file: {}", output);
        cursed::compile_to_object(&program, file, output)?;
        println!("✅ Object file generated: {}", output);
    Ok(())
}
