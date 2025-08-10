//! Enhanced CURSED REPL binary with all advanced features
//! 
//! This binary provides a fully-featured interactive CURSED shell with:
//! - Syntax highlighting
//! - Tab completion 
//! - Multi-line input support
//! - Interactive debugging
//! - Command history
//! - Variable inspection
//! - And much more!

use std::env;
use std::process;
use colored::*;

use cursed::repl::{EnhancedCursedRepl, ReplConfig};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let mut config = ReplConfig::default();
    let mut startup_file: Option<String> = None;
    let mut show_help = false;
    let mut show_version = false;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => show_help = true,
            "--version" | "-v" => show_version = true,
            "--no-syntax" => config.enable_syntax_highlighting = false,
            "--no-completion" => config.enable_tab_completion = false,
            "--no-multiline" => config.enable_multi_line = false,
            "--no-history" => config.enable_history = false,
            "--debug" => config.enable_debugging = true,
            "--light-theme" => config.color_theme = "light".to_string(),
            "--startup" => {
                if i + 1 < args.len() {
                    startup_file = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --startup requires a filename");
                    process::exit(1);
                }
            }
            arg if arg.starts_with('-') => {
                eprintln!("Error: Unknown option: {}", arg);
                show_help = true;
            }
            _ => {
                eprintln!("Error: Unexpected argument: {}", args[i]);
                show_help = true;
            }
        }
        i += 1;
    }
    
    if show_version {
        print_version();
        return;
    }
    
    if show_help {
        print_help();
        return;
    }
    
    // Initialize and run the enhanced REPL
    match run_enhanced_repl(config, startup_file) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            process::exit(1);
        }
    }
}

fn run_enhanced_repl(config: ReplConfig, startup_file: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Create enhanced REPL with configuration
    let mut repl = EnhancedCursedRepl::with_config(config)?;
    
    // Load startup file if specified
    if let Some(path) = startup_file {
        if let Err(e) = repl.load_startup_file(&path) {
            eprintln!("{} Failed to load startup file '{}': {}", 
                "Warning".yellow(), path, e);
        }
    }
    
    // Run the REPL
    repl.run_enhanced_repl()?;
    
    Ok(())
}

fn print_version() {
    println!("{}", "🔥 Enhanced CURSED REPL".cyan().bold());
    println!("Version: {}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
    println!("Build: {}", option_env!("BUILD_DATE").unwrap_or("unknown"));
    println!();
    println!("Advanced interactive shell for the CURSED programming language");
    println!("Features: syntax highlighting, tab completion, multi-line input, debugging");
}

fn print_help() {
    println!("{}", "🔥 Enhanced CURSED REPL".cyan().bold());
    println!("Advanced interactive shell for the CURSED programming language");
    println!();
    println!("{}", "USAGE:".green().bold());
    println!("    cursed_enhanced_repl [OPTIONS]");
    println!();
    println!("{}", "OPTIONS:".green().bold());
    println!("    -h, --help              Show this help message");
    println!("    -v, --version           Show version information");
    println!("    --startup <FILE>        Load and execute startup file");
    println!("    --debug                 Enable debugging features");
    println!("    --light-theme           Use light color theme (default: dark)");
    println!();
    println!("{}", "FEATURE TOGGLES:".yellow().bold());
    println!("    --no-syntax             Disable syntax highlighting");
    println!("    --no-completion         Disable tab completion");
    println!("    --no-multiline          Disable multi-line input");
    println!("    --no-history            Disable command history");
    println!();
    println!("{}", "EXAMPLES:".blue().bold());
    println!("    cursed_enhanced_repl                    # Start with default settings");
    println!("    cursed_enhanced_repl --debug            # Start with debugging enabled");
    println!("    cursed_enhanced_repl --startup init.csd # Load startup file");
    println!("    cursed_enhanced_repl --light-theme      # Use light color theme");
    println!();
    println!("{}", "INTERACTIVE COMMANDS:".cyan().bold());
    println!("    :help                   Show REPL help");
    println!("    :config                 Show/modify configuration");
    println!("    :debug                  Enter debug mode");
    println!("    :syntax check <code>    Check syntax");
    println!("    :theme <dark|light>     Change theme");
    println!("    :quit                   Exit REPL");
    println!();
    println!("{}", "CURSED LANGUAGE EXAMPLES:".magenta().bold());
    println!("    sus x drip = 42                         # Variable declaration");
    println!("    slay add(a drip, b drip) drip {{ damn a + b }}  # Function definition");
    println!("    vibez.spill(\"Hello, world!\")            # Print statement");
    println!("    yeet \"mathz\"                            # Import module");
    println!("    ready (x > 0) {{ vibez.spill(\"positive\") }}   # Conditional");
    println!();
    println!("For more information, visit: https://github.com/ghuntley/cursed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ReplConfig::default();
        assert!(config.enable_syntax_highlighting);
        assert!(config.enable_tab_completion);
        assert!(config.enable_multi_line);
        assert!(config.enable_history);
        assert!(!config.enable_debugging); // Disabled by default
    }
}
