#!/usr/bin/env rust
//! CURSED REPL - Interactive Read-Eval-Print Loop
//! 
//! Enhanced interactive shell for the CURSED programming language with:
//! - Syntax highlighting for keywords, operators, literals, and comments
//! - Multi-line input support with automatic indentation detection
//! - Built-in command system for development tools
//! - Tab completion for keywords, variables, and functions
//! - Session management with variable persistence
//! - Build system integration for project-aware features
//! - Comprehensive error handling and recovery

use clap::{Arg, ArgAction, Command};
use std::process;

use cursed::repl::CursedRepl;

fn main() {
    // Initialize the CURSED runtime
    cursed::init();

    let app = build_cli();
    let matches = app.get_matches();

    let result = run_repl(&matches);

    match result {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn build_cli() -> Command {
    Command::new("cursed-repl")
        .about("CURSED Interactive REPL - Enhanced shell for development")
        .version(cursed::VERSION)
        .author("Geoffrey Huntley")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Enable verbose output and debug mode")
        )
        .arg(
            Arg::new("no-history")
                .long("no-history")
                .action(ArgAction::SetTrue)
                .help("Disable command history persistence")
        )
        .arg(
            Arg::new("no-syntax-highlighting")
                .long("no-syntax-highlighting")
                .action(ArgAction::SetTrue)
                .help("Disable syntax highlighting")
        )
        .arg(
            Arg::new("load")
                .short('l')
                .long("load")
                .value_name("FILE")
                .help("Load and execute a CURSED file at startup")
        )
        .arg(
            Arg::new("working-dir")
                .short('w')
                .long("working-dir")
                .value_name("DIR")
                .help("Set working directory for project context")
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .value_name("SECONDS")
                .help("Set timeout for command execution (default: 30)")
                .default_value("30")
        )
}

fn run_repl(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let verbose = matches.get_flag("verbose");
    let enable_history = !matches.get_flag("no-history");
    let enable_syntax_highlighting = !matches.get_flag("no-syntax-highlighting");
    let load_file = matches.get_one::<String>("load");
    let working_dir = matches.get_one::<String>("working-dir");
    let timeout: u64 = matches.get_one::<String>("timeout")
        .unwrap()
        .parse()
        .unwrap_or(30);

    // Create and configure REPL
    let mut repl = CursedRepl::new()
        .with_verbose(verbose)
        .with_history(enable_history)
        .with_syntax_highlighting(enable_syntax_highlighting)
        .with_timeout(std::time::Duration::from_secs(timeout));

    // Set working directory if provided
    if let Some(dir) = working_dir {
        repl = repl.with_working_directory(dir)?;
    }

    // Load file if provided
    if let Some(file) = load_file {
        repl.load_file(file)?;
    }

    // Start the interactive REPL
    Ok(repl.run()?)
}
