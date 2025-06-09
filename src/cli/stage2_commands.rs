// CLI commands for Stage 2 (CURSED) compiler management

use crate::bootstrap::{BootstrapManager, CompilerStage};
use crate::error::Error;
use std::env;

/// Stage 2 compiler command arguments
#[derive(Debug, Clone)]
pub struct Stage2Args {
    pub command: Stage2Command,
    pub source_file: Option<String>,
    pub output_file: Option<String>,
    pub verbose: bool,
    pub debug: bool,
    pub optimization_level: u8,
}

/// Stage 2 compiler commands
#[derive(Debug, Clone, PartialEq)]
pub enum Stage2Command {
    /// Check if Stage 2 compiler is available
    Status,
    /// Build Stage 2 compiler from CURSED source
    Build,
    /// Compile using Stage 2 compiler
    Compile,
    /// Enable/disable self-hosting mode
    SelfHost(bool),
    /// Show Stage 2 compiler version
    Version,
    /// Test Stage 2 compiler functionality
    Test,
}

impl Stage2Args {
    pub fn new() -> Self {
        Self {
            command: Stage2Command::Status,
            source_file: None,
            output_file: None,
            verbose: false,
            debug: false,
            optimization_level: 0,
        }
    }
}

impl Default for Stage2Args {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse Stage 2 command line arguments
pub fn parse_stage2_args(args: &[String]) -> Result<Stage2Args, Error> {
    let mut parsed = Stage2Args::new();
    let mut i = 0;
    
    while i < args.len() {
        match args[i].as_str() {
            "status" => {
                parsed.command = Stage2Command::Status;
            }
            "build" => {
                parsed.command = Stage2Command::Build;
            }
            "compile" => {
                parsed.command = Stage2Command::Compile;
                
                // Next argument should be source file
                if i + 1 < args.len() {
                    i += 1;
                    parsed.source_file = Some(args[i].clone());
                } else {
                    return Err(crate::error::compilation_error("Source file required for compile command"));
                }
            }
            "self-host" => {
                if i + 1 < args.len() && (args[i + 1] == "on" || args[i + 1] == "off") {
                    i += 1;
                    parsed.command = Stage2Command::SelfHost(args[i] == "on");
                } else {
                    return Err(crate::error::compilation_error("self-host requires 'on' or 'off'"));
                }
            }
            "version" => {
                parsed.command = Stage2Command::Version;
            }
            "test" => {
                parsed.command = Stage2Command::Test;
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    i += 1;
                    parsed.output_file = Some(args[i].clone());
                } else {
                    return Err(crate::error::compilation_error("Output file required after -o"));
                }
            }
            "-v" | "--verbose" => {
                parsed.verbose = true;
            }
            "-g" | "--debug" => {
                parsed.debug = true;
            }
            "-O" | "--optimize" => {
                if i + 1 < args.len() {
                    i += 1;
                    parsed.optimization_level = args[i].parse()
                        .map_err(|_| crate::error::compilation_error("Invalid optimization level"))?;
                } else {
                    parsed.optimization_level = 1;
                }
            }
            arg if arg.starts_with('-') => {
                return Err(crate::error::compilation_error(&format!("Unknown flag: {}", arg)));
            }
            _ => {
                // Treat as source file if no source file set yet
                if parsed.source_file.is_none() {
                    parsed.source_file = Some(args[i].clone());
                } else {
                    return Err(crate::error::compilation_error(&format!("Unexpected argument: {}", args[i])));
                }
            }
        }
        i += 1;
    }
    
    Ok(parsed)
}

/// Execute Stage 2 compiler command
pub fn execute_stage2_command(args: Stage2Args) -> Result<(), Error> {
    let manager = BootstrapManager::new();
    
    match args.command {
        Stage2Command::Status => {
            let status = manager.status();
            println!("{}", status);
            Ok(())
        }
        
        Stage2Command::Build => {
            println!("Building Stage 2 compiler...");
            
            // Set environment variable to trigger build
            env::set_var("CURSED_BUILD_STAGE2", "1");
            
            println!("To build Stage 2 compiler:");
            println!("  1. Run: cargo build");
            println!("  2. The Stage 2 compiler will be built from CURSED source files");
            println!("  3. Binary will be available at: target/debug/cursed-stage2");
            
            Ok(())
        }
        
        Stage2Command::Compile => {
            let source = args.source_file
                .ok_or_else(|| crate::error::compilation_error("Source file required for compilation"))?;
            
            let output = args.output_file
                .unwrap_or_else(|| "a.out".to_string());
            
            if args.verbose {
                println!("Compiling {} -> {}", source, output);
            }
            
            manager.compile(&source, &output)
        }
        
        Stage2Command::SelfHost(enable) => {
            if enable {
                env::set_var("CURSED_USE_STAGE2", "1");
                println!("Self-hosting enabled. Stage 2 compiler will be used when available.");
            } else {
                env::remove_var("CURSED_USE_STAGE2");
                println!("Self-hosting disabled. Stage 1 compiler will be used.");
            }
            Ok(())
        }
        
        Stage2Command::Version => {
            if manager.stage2_available {
                let stage2 = crate::bootstrap::stage2::create_stage2_compiler();
                match stage2.version() {
                    Ok(version) => println!("{}", version),
                    Err(_) => println!("CURSED Stage 2 Compiler v0.1.0"),
                }
            } else {
                println!("Stage 2 compiler not available");
                println!("Build it with: cursed stage2 build && cargo build");
            }
            Ok(())
        }
        
        Stage2Command::Test => {
            if !manager.stage2_available {
                return Err(crate::error::compilation_error("Stage 2 compiler not available"));
            }
            
            println!("Testing Stage 2 compiler functionality...");
            
            // Create a simple test program
            let test_source = r#"
func main() int {
    let x: int = 42
    let y: int = 8
    return x + y
}
"#;
            
            // Write test file
            std::fs::write("test_stage2.csd", test_source)
                .map_err(|e| crate::error::compilation_error(&format!("Failed to write test file: {}", e)))?;
            
            // Compile with Stage 2
            match manager.compile_stage2("test_stage2.csd", "test_stage2") {
                Ok(_) => {
                    println!("✓ Stage 2 compilation successful");
                    
                    // Clean up
                    std::fs::remove_file("test_stage2.csd").ok();
                    std::fs::remove_file("test_stage2").ok();
                    std::fs::remove_file("test_stage2.ll").ok();
                    
                    Ok(())
                }
                Err(e) => {
                    // Clean up
                    std::fs::remove_file("test_stage2.csd").ok();
                    
                    Err(crate::error::compilation_error(&format!("Stage 2 test failed: {}", e)))
                }
            }
        }
    }
}

/// Print Stage 2 command help
pub fn print_stage2_help() {
    println!("CURSED Stage 2 Compiler Commands:");
    println!();
    println!("USAGE:");
    println!("    cursed stage2 <COMMAND> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    status              Show Stage 2 compiler availability and status");
    println!("    build               Build Stage 2 compiler from CURSED source");
    println!("    compile <file>      Compile CURSED source using Stage 2 compiler");
    println!("    self-host <on|off>  Enable or disable self-hosting mode");
    println!("    version             Show Stage 2 compiler version");
    println!("    test                Test Stage 2 compiler functionality");
    println!();
    println!("OPTIONS:");
    println!("    -o, --output <file>     Specify output file");
    println!("    -v, --verbose           Enable verbose output");
    println!("    -g, --debug             Include debug information");
    println!("    -O, --optimize [level]  Set optimization level (0-3)");
    println!();
    println!("EXAMPLES:");
    println!("    cursed stage2 status");
    println!("    cursed stage2 build");
    println!("    cursed stage2 compile hello.csd -o hello");
    println!("    cursed stage2 self-host on");
    println!("    cursed stage2 test");
    println!();
    println!("ENVIRONMENT VARIABLES:");
    println!("    CURSED_USE_STAGE2       Use Stage 2 compiler when available");
    println!("    CURSED_BUILD_STAGE2     Build Stage 2 compiler during cargo build");
    println!("    CURSED_SELF_HOSTING     Enable self-hosting mode");
}

/// Check if current compiler stage matches expected
pub fn verify_compiler_stage(expected: CompilerStage) -> Result<(), Error> {
    let manager = BootstrapManager::new();
    let actual = manager.preferred_stage();
    
    if actual != expected {
        return Err(crate::error::compilation_error(&format!(
            "Expected compiler stage {}, but {} is being used",
            expected, actual
        )));
    }
    
    Ok(())
}

/// Get current compiler stage information
pub fn get_compiler_info() -> String {
    let manager = BootstrapManager::new();
    let status = manager.status();
    
    format!(
        "Current compiler: {} (Stage 1: {}, Stage 2: {})",
        status.preferred_stage,
        if status.stage1_available { "Available" } else { "Not Available" },
        if status.stage2_available { "Available" } else { "Not Available" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_stage2_args_status() {
        let args = vec!["status".to_string()];
        let parsed = parse_stage2_args(&args).unwrap();
        assert_eq!(parsed.command, Stage2Command::Status);
    }
    
    #[test]
    fn test_parse_stage2_args_compile() {
        let args = vec!["compile".to_string(), "test.csd".to_string()];
        let parsed = parse_stage2_args(&args).unwrap();
        assert_eq!(parsed.command, Stage2Command::Compile);
        assert_eq!(parsed.source_file, Some("test.csd".to_string()));
    }
    
    #[test]
    fn test_parse_stage2_args_self_host() {
        let args = vec!["self-host".to_string(), "on".to_string()];
        let parsed = parse_stage2_args(&args).unwrap();
        assert_eq!(parsed.command, Stage2Command::SelfHost(true));
    }
    
    #[test]
    fn test_parse_stage2_args_verbose() {
        let args = vec!["status".to_string(), "--verbose".to_string()];
        let parsed = parse_stage2_args(&args).unwrap();
        assert!(parsed.verbose);
    }
    
    #[test]
    fn test_get_compiler_info() {
        let info = get_compiler_info();
        assert!(info.contains("Current compiler:"));
    }
}
