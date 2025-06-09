//! CLI commands for LLVM IR and bitcode generation.
//!
//! This module provides command-line interface functionality for generating
//! LLVM IR (.ll) and bitcode (.bc) files from CURSED programs. It integrates
//! with the existing CLI infrastructure to add new compiler modes.

use crate::ast::Program;
use crate::codegen::llvm::{IrOutputGenerator, IrOutputConfig, IrOutputFormat};
use crate::error::Error;
use crate::lexer::Lexer;
use crate::parser::Parser;

use inkwell::context::Context;
use std::path::{Path, PathBuf};
use std::fs;

use tracing::{debug, error, info, instrument};

/// CLI arguments for IR generation commands
#[derive(Debug, Clone)]
pub struct IrCompileArgs {
    /// Input file path
    pub input_file: PathBuf,
    /// Output directory
    pub output_dir: Option<PathBuf>,
    /// Output format
    pub format: IrOutputFormat,
    /// Whether to preserve directory structure
    pub preserve_structure: bool,
    /// Whether to optimize before output
    pub optimize: bool,
    /// Base name for output files
    pub base_name: Option<String>,
    /// Whether to include debug comments
    pub include_debug_comments: bool,
}

impl Default for IrCompileArgs {
    fn default() -> Self {
        Self {
            input_file: PathBuf::new(),
            output_dir: None,
            format: IrOutputFormat::LlvmIr,
            preserve_structure: true,
            optimize: false,
            base_name: None,
            include_debug_comments: true,
        }
    }
}

/// Execute IR compilation from command line arguments
#[instrument(skip(args), fields(
    input_file = ?args.input_file,
    format = ?args.format,
    output_dir = ?args.output_dir
))]
pub fn execute_ir_compile(args: IrCompileArgs) -> Result<(), Error> {
    info!("Starting IR compilation");

    // Validate input file exists
    if !args.input_file.exists() {
        return Err(Error::IO(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Input file does not exist: {:?}", args.input_file)
        )));
    }

    // Read and parse the input file
    let source_code = fs::read_to_string(&args.input_file)
        .map_err(Error::IO)?;

    debug!("Parsing CURSED source code");
    let mut lexer = Lexer::new(&source_code);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;

    if !parser.errors().is_empty() {
        let error_msgs: Vec<String> = parser.errors().iter().map(|e| e.to_string()).collect();
        return Err(Error::Parsing(format!("Parser errors: {}", error_msgs.join(", "))));
    }

    // Set up IR output configuration
    let output_dir = args.output_dir.unwrap_or_else(|| {
        args.input_file
            .parent()
            .map(|p| p.join("output"))
            .unwrap_or_else(|| PathBuf::from("output"))
    });

    let config = IrOutputConfig {
        format: args.format,
        output_dir,
        preserve_structure: args.preserve_structure,
        optimize: args.optimize,
        optimization_level: "O0".to_string(),
        base_name: args.base_name,
        include_debug_comments: args.include_debug_comments,
        show_optimization_stats: false,
    };

    // Generate IR output
    let context = Context::create();
    let generator = IrOutputGenerator::new(&context, config);
    let generated_files = generator.generate_from_program(&program, &args.input_file)?;

    // Report generated files
    if generated_files.has_files() {
        println!("Generated files:");
        for file in generated_files.all_files() {
            println!("  {:?}", file);
        }
        info!("IR compilation completed successfully");
    } else {
        error!("No files were generated");
        return Err(Error::CodeGenError("No output files generated".to_string()));
    }

    Ok(())
}

/// Parse format string into IrOutputFormat
pub fn parse_format(format_str: &str) -> Result<IrOutputFormat, Error> {
    match format_str.to_lowercase().as_str() {
        "ir" | "ll" | "llvm" => Ok(IrOutputFormat::LlvmIr),
        "bc" | "bitcode" => Ok(IrOutputFormat::Bitcode),
        "both" | "all" => Ok(IrOutputFormat::Both),
        _ => Err(Error::InvalidArguments(format!(
            "Invalid output format: {}. Valid formats: ir, bc, both", 
            format_str
        ))),
    }
}

/// Print help for IR compilation commands
pub fn print_ir_help() {
    println!("CURSED IR/Bitcode Generation:");
    println!();
    println!("USAGE:");
    println!("    cursed --emit-ir [OPTIONS] <INPUT_FILE>");
    println!("    cursed --emit-bc [OPTIONS] <INPUT_FILE>");
    println!("    cursed --emit-both [OPTIONS] <INPUT_FILE>");
    println!();
    println!("OPTIONS:");
    println!("    --output-dir <DIR>         Output directory (default: ./output)");
    println!("    --format <FORMAT>          Output format: ir, bc, both (default: ir)");
    println!("    --base-name <NAME>         Base name for output files");
    println!("    --no-preserve-structure    Don't preserve input directory structure");
    println!("    --optimize                 Optimize module before output");
    println!("    --no-debug-comments        Don't include debug comments in IR");
    println!();
    println!("EXAMPLES:");
    println!("    cursed --emit-ir example.csd");
    println!("    cursed --emit-bc --output-dir build example.csd");
    println!("    cursed --emit-both --optimize examples/*.csd");
    println!();
    println!("OUTPUT:");
    println!("    .ll files    Human-readable LLVM IR for debugging");
    println!("    .bc files    LLVM bitcode for distribution and linking");
}

/// Handle IR-related command line arguments
pub fn handle_ir_arguments(args: &[String]) -> Option<Result<(), Error>> {
    if args.is_empty() {
        return None;
    }

    // Check for IR-related flags
    match args[0].as_str() {
        "--emit-ir" | "--emit-bc" | "--emit-both" => {
            Some(handle_emit_command(args))
        }
        "--ir-help" => {
            print_ir_help();
            Some(Ok(()))
        }
        _ => None,
    }
}

/// Handle emit commands (--emit-ir, --emit-bc, --emit-both)
fn handle_emit_command(args: &[String]) -> Result<(), Error> {
    let mut compile_args = IrCompileArgs::default();

    // Determine format from command
    compile_args.format = match args[0].as_str() {
        "--emit-ir" => IrOutputFormat::LlvmIr,
        "--emit-bc" => IrOutputFormat::Bitcode,
        "--emit-both" => IrOutputFormat::Both,
        _ => return Err(Error::InvalidArguments(format!("Unknown command: {}", args[0]))),
    };

    // Parse remaining arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--output-dir" => {
                if i + 1 < args.len() {
                    compile_args.output_dir = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    return Err(Error::InvalidArguments("--output-dir requires a directory path".to_string()));
                }
            }
            "--format" => {
                if i + 1 < args.len() {
                    compile_args.format = parse_format(&args[i + 1])?;
                    i += 2;
                } else {
                    return Err(Error::InvalidArguments("--format requires a format specification".to_string()));
                }
            }
            "--base-name" => {
                if i + 1 < args.len() {
                    compile_args.base_name = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    return Err(Error::InvalidArguments("--base-name requires a name".to_string()));
                }
            }
            "--no-preserve-structure" => {
                compile_args.preserve_structure = false;
                i += 1;
            }
            "--optimize" => {
                compile_args.optimize = true;
                i += 1;
            }
            "--no-debug-comments" => {
                compile_args.include_debug_comments = false;
                i += 1;
            }
            _ => {
                // Assume it's the input file
                if compile_args.input_file.as_os_str().is_empty() {
                    compile_args.input_file = PathBuf::from(&args[i]);
                } else {
                    return Err(Error::InvalidArguments(format!("Unexpected argument: {}", args[i])));
                }
                i += 1;
            }
        }
    }

    // Validate required arguments
    if compile_args.input_file.as_os_str().is_empty() {
        return Err(Error::InvalidArguments("Input file is required".to_string()));
    }

    execute_ir_compile(compile_args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_format() {
        assert_eq!(parse_format("ir").unwrap(), IrOutputFormat::LlvmIr);
        assert_eq!(parse_format("ll").unwrap(), IrOutputFormat::LlvmIr);
        assert_eq!(parse_format("bc").unwrap(), IrOutputFormat::Bitcode);
        assert_eq!(parse_format("both").unwrap(), IrOutputFormat::Both);
        
        assert!(parse_format("invalid").is_err());
    }

    #[test]
    fn test_handle_ir_arguments() {
        let args = vec!["--emit-ir".to_string(), "test.csd".to_string()];
        assert!(handle_ir_arguments(&args).is_some());

        let args = vec!["--other-command".to_string()];
        assert!(handle_ir_arguments(&args).is_none());

        let empty_args: Vec<String> = vec![];
        assert!(handle_ir_arguments(&empty_args).is_none());
    }
}
