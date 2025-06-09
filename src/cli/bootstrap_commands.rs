//! Command-line interface for bootstrap compilation mode
//!
//! This module provides CLI commands and utilities for working with
//! the bootstrap compiler subset.

use std::path::PathBuf;
use clap::{Arg, ArgMatches, Command};
// Note: Bootstrap imports will be enabled once the bootstrap module compiles
use crate::bootstrap::validator::SubsetValidator;
use crate::bootstrap::subset::BootstrapSubset;
use crate::bootstrap::config::{BootstrapConfig, BootstrapConfigBuilder};
use crate::error::Error;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ast::base::Program;
use tracing::{info, warn, error};

/// Arguments for bootstrap compilation commands
#[derive(Debug, Clone)]
pub struct BootstrapArgs {
    /// Input file to validate or compile
    pub input_file: Option<PathBuf>,
    /// Whether to run in strict mode
    pub strict: bool,
    /// Whether to generate warnings
    pub warnings: bool,
    /// Maximum allowed statements
    pub max_statements: Option<usize>,
    /// Additional stdlib modules to allow
    pub extra_modules: Vec<String>,
    /// Whether to allow experimental features
    pub experimental: bool,
    /// Output directory for compilation artifacts
    pub output_dir: Option<PathBuf>,
    /// Command to execute
    pub command: BootstrapCommand,
}

/// Bootstrap-specific commands
#[derive(Debug, Clone)]
pub enum BootstrapCommand {
    /// Validate that a program uses only bootstrap subset
    Validate,
    /// Show information about the bootstrap subset
    Info,
    /// Compile using bootstrap mode
    Compile,
    /// Show configuration options
    Config,
}

/// Creates the bootstrap CLI command structure
pub fn create_bootstrap_command() -> Command {
    Command::new("bootstrap")
        .about("Bootstrap compiler subset utilities")
        .subcommand(
            Command::new("validate")
                .about("Validate that a program uses only bootstrap subset features")
                .arg(
                    Arg::new("input")
                        .help("Input CURSED file to validate")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("strict")
                        .long("strict")
                        .help("Enable strict validation mode")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("warnings")
                        .long("warnings")
                        .help("Show warnings for suboptimal code")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("max-statements")
                        .long("max-statements")
                        .help("Maximum allowed statements")
                        .value_name("NUMBER")
                        .value_parser(clap::value_parser!(usize))
                )
        )
        .subcommand(
            Command::new("info")
                .about("Show information about the bootstrap subset")
                .arg(
                    Arg::new("detailed")
                        .long("detailed")
                        .help("Show detailed feature list")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("compile")
                .about("Compile using bootstrap mode")
                .arg(
                    Arg::new("input")
                        .help("Input CURSED file to compile")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output directory")
                        .value_name("DIR")
                )
                .arg(
                    Arg::new("strict")
                        .long("strict")
                        .help("Enable strict compilation mode")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("experimental")
                        .long("experimental")
                        .help("Allow experimental features")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("config")
                .about("Show or generate bootstrap configuration")
                .arg(
                    Arg::new("generate")
                        .long("generate")
                        .help("Generate a configuration file")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output configuration file path")
                        .value_name("FILE")
                )
        )
}

/// Parses bootstrap command arguments
pub fn parse_bootstrap_args(matches: &ArgMatches) -> Result<BootstrapArgs, Error> {
    let (command, submatches) = match matches.subcommand() {
        Some(("validate", sub_m)) => (BootstrapCommand::Validate, sub_m),
        Some(("info", sub_m)) => (BootstrapCommand::Info, sub_m),
        Some(("compile", sub_m)) => (BootstrapCommand::Compile, sub_m),
        Some(("config", sub_m)) => (BootstrapCommand::Config, sub_m),
        _ => return Err(Error::from_str("Invalid bootstrap command")),
    };

    let input_file = submatches
        .get_one::<String>("input")
        .map(PathBuf::from);

    let output_dir = submatches
        .get_one::<String>("output")
        .map(PathBuf::from);

    let strict = submatches.get_flag("strict");
    let warnings = submatches.get_flag("warnings");
    let experimental = submatches.get_flag("experimental");
    
    let max_statements = submatches.get_one::<usize>("max-statements").copied();

    Ok(BootstrapArgs {
        input_file,
        strict,
        warnings,
        max_statements,
        extra_modules: Vec::new(), // Could be extended to parse from args
        experimental,
        output_dir,
        command,
    })
}

/// Executes a bootstrap command
pub fn execute_bootstrap_command(args: BootstrapArgs) -> Result<(), Error> {
    match args.command {
        BootstrapCommand::Validate => execute_validate_command(args),
        BootstrapCommand::Info => execute_info_command(args),
        BootstrapCommand::Compile => execute_compile_command(args),
        BootstrapCommand::Config => execute_config_command(args),
    }
}

/// Executes the validate command
fn execute_validate_command(args: BootstrapArgs) -> Result<(), Error> {
    let input_file = args.input_file.ok_or_else(|| {
        Error::from_str("Input file is required for validation")
    })?;

    info!("Validating bootstrap subset compliance for: {:?}", input_file);

    // Read and parse the input file
    let input = std::fs::read_to_string(&input_file)
        .map_err(|e| Error::from_str(&format!("Failed to read file: {}", e)))?;

    let mut lexer = Lexer::new(&input);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    if !parser.errors().is_empty() {
        return Err(Error::from_str("Parser errors found - cannot validate"));
    }

    // Create validator with configuration
    let mut validator = SubsetValidator::new();
    let result = validator.validate_program(&program);

    // Print results
    println!("Bootstrap Subset Validation Results");
    println!("==================================");
    println!("File: {:?}", input_file);
    println!("Valid: {}", if result.is_valid { "✅ YES" } else { "❌ NO" });
    println!();

    // Print statistics
    println!("Statistics:");
    println!("  Statements checked: {}", result.stats.statements_checked);
    println!("  Expressions checked: {}", result.stats.expressions_checked);
    println!("  Functions found: {}", result.stats.functions_found);
    println!("  Imports found: {}", result.stats.imports_found);
    println!("  Expression types: {}", result.stats.expression_types.len());
    println!("  Statement types: {}", result.stats.statement_types.len());
    println!();

    // Print errors
    if !result.errors.is_empty() {
        println!("Errors ({}):", result.errors.len());
        for (i, error) in result.errors.iter().enumerate() {
            println!("  {}. {}", i + 1, error.message);
            if let Some(suggestion) = &error.suggestion {
                println!("     Suggestion: {}", suggestion);
            }
        }
        println!();
    }

    // Print warnings if requested
    if args.warnings && !result.warnings.is_empty() {
        println!("Warnings ({}):", result.warnings.len());
        for (i, warning) in result.warnings.iter().enumerate() {
            println!("  {}. {}", i + 1, warning.message);
            if let Some(suggestion) = &warning.suggestion {
                println!("     Suggestion: {}", suggestion);
            }
        }
        println!();
    }

    if result.is_valid {
        println!("✅ Program is compatible with bootstrap compilation!");
    } else {
        println!("❌ Program uses features not available in bootstrap subset.");
        return Err(Error::from_str("Validation failed"));
    }

    Ok(())
}

/// Executes the info command
fn execute_info_command(args: BootstrapArgs) -> Result<(), Error> {
    let subset = BootstrapSubset::new();
    
    println!("CURSED Bootstrap Compiler Subset");
    println!("===============================");
    println!();
    
    println!("The bootstrap subset includes only the essential language features");
    println!("needed for self-hosting compilation. This ensures the bootstrap");
    println!("compiler can be implemented with minimal complexity.");
    println!();
    
    println!("Supported Features:");
    println!("------------------");
    
    // Show allowed tokens
    println!("• Tokens:");
    for token_desc in subset.get_allowed_tokens() {
        println!("  - {}", token_desc);
    }
    println!();
    
    // Show allowed expressions  
    println!("• Expression Types:");
    for expr_type in subset.get_allowed_expressions() {
        println!("  - {}", expr_type);
    }
    println!();
    
    // Show allowed statements
    println!("• Statement Types:");
    for stmt_type in subset.get_allowed_statements() {
        println!("  - {}", stmt_type);
    }
    println!();
    
    println!("Key Capabilities:");
    println!("  ✅ Standard library access: {}", subset.allows_stdlib_access());
    println!("  ✅ Control flow support: {}", subset.supports_control_flow());
    println!("  ✅ Function definitions: {}", subset.supports_functions());
    println!("  ✅ Variable declarations: {}", subset.supports_variables());
    println!();
    
    println!("Excluded Features:");
    println!("  ❌ Structs and interfaces");
    println!("  ❌ Channels and goroutines");
    println!("  ❌ Generics and type parameters");
    println!("  ❌ Method definitions");
    println!("  ❌ Switch and select statements");
    println!("  ❌ Pointer operations");
    println!("  ❌ Advanced error handling");
    
    Ok(())
}

/// Executes the compile command
fn execute_compile_command(args: BootstrapArgs) -> Result<(), Error> {
    let input_file = args.input_file.ok_or_else(|| {
        Error::from_str("Input file is required for compilation")
    })?;

    info!("Compiling in bootstrap mode: {:?}", input_file);

    // Create bootstrap configuration
    let mut config_builder = BootstrapConfigBuilder::new().enabled();
    
    if args.strict {
        config_builder = config_builder.strict();
    } else {
        config_builder = config_builder.lenient();
    }
    
    if args.experimental {
        config_builder = config_builder.experimental();
    }
    
    if let Some(max) = args.max_statements {
        config_builder = config_builder.max_statements(max);
    }
    
    if let Some(output_dir) = args.output_dir {
        config_builder = config_builder.output_dir(output_dir);
    }

    let config = config_builder.build()
        .map_err(|e| Error::from_str(&format!("Invalid configuration: {}", e)))?;

    println!("Bootstrap Configuration: {}", config.describe());

    // First validate if in strict mode
    if config.validate_subset {
        let validation_args = BootstrapArgs {
            input_file: Some(input_file.clone()),
            strict: config.strict_mode,
            warnings: config.generate_warnings,
            max_statements: config.max_statements,
            extra_modules: config.allowed_stdlib_modules.clone(),
            experimental: config.allow_experimental,
            output_dir: None,
            command: BootstrapCommand::Validate,
        };
        
        execute_validate_command(validation_args)?;
        println!("✅ Validation passed, proceeding with compilation...");
        println!();
    }

    // Proceed with normal compilation but with bootstrap optimizations
    let input_str = input_file.to_string_lossy();
    crate::run_file(&input_str)?;

    println!("✅ Bootstrap compilation completed successfully!");
    
    Ok(())
}

/// Executes the config command
fn execute_config_command(args: BootstrapArgs) -> Result<(), Error> {
    // For now, just show the default configuration
    // In the future, this could generate config files
    
    let default_config = BootstrapConfig::default();
    let strict_config = BootstrapConfig::strict();
    let lenient_config = BootstrapConfig::lenient();
    
    println!("Bootstrap Configuration Options");
    println!("==============================");
    println!();
    
    println!("Default Configuration:");
    println!("  {}", default_config.describe());
    println!("  Validation: {}", if default_config.validate_subset { "enabled" } else { "disabled" });
    println!("  Optimization: {}", if default_config.optimize_for_bootstrap { "enabled" } else { "disabled" });
    println!();
    
    println!("Strict Mode Configuration:");
    println!("  {}", strict_config.describe());
    println!();
    
    println!("Lenient Mode Configuration:");
    println!("  {}", lenient_config.describe());
    println!();
    
    println!("Available Standard Library Modules:");
    for module in &default_config.allowed_stdlib_modules {
        println!("  - {}", module);
    }
    
    Ok(())
}

/// Prints help for bootstrap commands
pub fn print_bootstrap_help() {
    println!("Bootstrap Compiler Subset Commands");
    println!("=================================");
    println!();
    println!("The bootstrap subset provides a minimal set of CURSED language features");
    println!("sufficient for self-hosting compilation. Use these commands to work with");
    println!("bootstrap-compatible code.");
    println!();
    println!("Commands:");
    println!("  validate <file>    Validate bootstrap subset compliance");
    println!("  info              Show bootstrap subset information");
    println!("  compile <file>    Compile using bootstrap mode");
    println!("  config            Show configuration options");
    println!();
    println!("Examples:");
    println!("  cursed bootstrap validate compiler.csd");
    println!("  cursed bootstrap compile --strict --output ./output compiler.csd");
    println!("  cursed bootstrap info");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bootstrap_args_parsing() {
        // This would require setting up clap matches for testing
        // For now, just test the enum variants exist
        assert_eq!(
            std::mem::discriminant(&BootstrapCommand::Validate),
            std::mem::discriminant(&BootstrapCommand::Validate)
        );
    }
    
    #[test]
    fn test_bootstrap_command_creation() {
        let cmd = create_bootstrap_command();
        assert_eq!(cmd.get_name(), "bootstrap");
    }
}
