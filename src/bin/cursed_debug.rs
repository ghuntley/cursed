/// CURSED Debug Information Generator
/// 
/// Command-line tool for generating and inspecting debug information
/// for CURSED programs.

use clap::{Arg, Command};
use cursed::debug::{DebugConfig, DebugInfoManager, DebugUtils, DebugConfigBuilder};
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{info, error, debug};

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("cursed_debug=info,cursed=debug")
        .init();

    let matches = Command::new("cursed-debug")
        .about("CURSED Debug Information Generator")
        .version("1.0.0")
        .author("CURSED Development Team")
        .arg(
            Arg::new("input")
                .help("Input CURSED source file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("Output directory for debug files")
                .short('o')
                .long("output")
                .value_name("DIR")
                .default_value("."),
        )
        .arg(
            Arg::new("debug-level")
                .help("Debug information level (0-3)")
                .short('g')
                .long("debug-level")
                .value_name("LEVEL")
                .default_value("2"),
        )
        .arg(
            Arg::new("format")
                .help("Output format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .value_parser(["llvm-ir", "dwarf", "gdb-script", "lldb-script", "vscode-config", "report"])
                .default_value("llvm-ir"),
        )
        .arg(
            Arg::new("optimized")
                .help("Generate optimized debug information")
                .long("optimized")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("include-source")
                .help("Include source code in debug information")
                .long("include-source")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("compress")
                .help("Compress debug sections")
                .long("compress")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("split-debug")
                .help("Split debug information into separate file")
                .long("split-debug")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dwarf-version")
                .help("DWARF version to generate (2, 3, 4, or 5)")
                .long("dwarf-version")
                .value_name("VERSION")
                .default_value("4"),
        )
        .arg(
            Arg::new("validate")
                .help("Validate debug information consistency")
                .long("validate")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("statistics")
                .help("Print debug information statistics")
                .long("stats")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .help("Enable verbose output")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if let Err(e) = run_debug_tool(&matches) {
        error!("Debug tool failed: {}", e);
        std::process::exit(1);
    }
}

fn run_debug_tool(matches: &clap::ArgMatches) -> Result<(), Error> {
    let input_file = matches.get_one::<String>("input").unwrap();
    let output_dir = matches.get_one::<String>("output").unwrap();
    let debug_level: u8 = matches.get_one::<String>("debug-level")
        .unwrap()
        .parse()
        .map_err(|_| Error::Compile("Invalid debug level".to_string()))?;
    let format = matches.get_one::<String>("format").unwrap();
    let optimized = matches.get_flag("optimized");
    let include_source = matches.get_flag("include-source");
    let compress = matches.get_flag("compress");
    let split_debug = matches.get_flag("split-debug");
    let dwarf_version: u8 = matches.get_one::<String>("dwarf-version")
        .unwrap()
        .parse()
        .map_err(|_| Error::Compile("Invalid DWARF version".to_string()))?;
    let validate = matches.get_flag("validate");
    let show_stats = matches.get_flag("statistics");
    let verbose = matches.get_flag("verbose");

    if verbose {
        info!("CURSED Debug Information Generator v1.0.0");
        info!("Input file: {}", input_file);
        info!("Output directory: {}", output_dir);
        info!("Debug level: {}", debug_level);
        info!("Format: {}", format);
    }

    // Check input file exists
    let input_path = Path::new(input_file);
    if !input_path.exists() {
        return Err(Error::Compile(format!("Input file '{}' does not exist", input_file)));
    }

    // Read source code
    let source = fs::read_to_string(input_path)
        .map_err(|e| Error::Io(e.into()))?;

    // Create debug configuration
    let debug_config = DebugConfigBuilder::new()
        .debug_info(true)
        .level(debug_level)
        .optimized(optimized)
        .include_source(include_source)
        .compressed(compress)
        .split_debug(split_debug)
        .dwarf_version(dwarf_version)
        .output_dir(PathBuf::from(output_dir))
        .with_lines()
        .with_variables()
        .with_types()
        .build_validated()
        .map_err(|errors| Error::Compile(format!("Invalid debug configuration: {:?}", errors)))?;

    if verbose {
        debug!("Debug configuration: {:?}", debug_config);
    }

    // Create LLVM code generator with debug support
    let mut generator = LlvmCodeGenerator::new_with_debug(debug_config)?;

    // Generate debug information based on format
    match format.as_str() {
        "llvm-ir" => generate_llvm_ir(&mut generator, input_path, &source, output_dir)?,
        "dwarf" => generate_dwarf_info(&mut generator, input_path, &source, output_dir)?,
        "gdb-script" => generate_gdb_script(&mut generator, input_path, &source, output_dir)?,
        "lldb-script" => generate_lldb_script(&mut generator, input_path, &source, output_dir)?,
        "vscode-config" => generate_vscode_config(&mut generator, input_path, output_dir)?,
        "report" => generate_debug_report(&mut generator, input_path, &source, output_dir)?,
        _ => return Err(Error::Compile(format!("Unknown format: {}", format))),
    }

    // Validate debug information if requested
    if validate {
        info!("Validating debug information...");
        match generator.validate_debug() {
            Ok(()) => info!("Debug information validation passed"),
            Err(errors) => {
                error!("Debug information validation failed:");
                for error in errors {
                    error!("  - {}", error);
                }
                return Err(Error::Compile("Debug validation failed".to_string()));
            }
        }
    }

    // Show statistics if requested
    if show_stats {
        info!("Debug Information Statistics:");
        info!("{}", generator.debug_statistics());
        
        let line_table = generator.line_table();
        if !line_table.is_empty() {
            info!("Line table entries: {}", line_table.len());
            if verbose {
                for (line, info) in line_table.iter().take(10) {
                    debug!("  Line {}: {}", line, info);
                }
                if line_table.len() > 10 {
                    debug!("  ... and {} more entries", line_table.len() - 10);
                }
            }
        }
    }

    info!("Debug information generation completed successfully");
    Ok(())
}

fn generate_llvm_ir(
    generator: &mut LlvmCodeGenerator,
    input_path: &Path,
    source: &str,
    output_dir: &str,
) -> Result<(), Error> {
    info!("Generating LLVM IR with debug information");
    
    let ir = generator.generate_ir_with_debug(input_path.to_path_buf(), source)?;
    
    let output_file = Path::new(output_dir).join(
        input_path.file_stem().unwrap_or_default()
    ).with_extension("ll");
    
    fs::write(&output_file, ir)
        .map_err(|e| Error::Io(e.into()))?;
    
    info!("LLVM IR written to: {}", output_file.display());
    Ok(())
}

fn generate_dwarf_info(
    generator: &mut LlvmCodeGenerator,
    input_path: &Path,
    source: &str,
    output_dir: &str,
) -> Result<(), Error> {
    info!("Generating DWARF debug information");
    
    // Generate IR first to populate debug info
    let _ir = generator.generate_ir_with_debug(input_path.to_path_buf(), source)?;
    
    // Create a simplified DWARF dump
    let dwarf_info = format!(
        "# DWARF Debug Information for {}\n\
         # Generated by CURSED Debug Tool\n\
         \n\
         Debug Statistics:\n\
         {}\n\
         \n\
         Line Table:\n",
        input_path.display(),
        generator.debug_statistics()
    );
    
    let mut content = dwarf_info;
    for (line, info) in generator.line_table() {
        content.push_str(&format!("Line {}: {}\n", line, info));
    }
    
    let output_file = Path::new(output_dir).join(
        input_path.file_stem().unwrap_or_default()
    ).with_extension("dwarf");
    
    fs::write(&output_file, content)
        .map_err(|e| Error::Io(e.into()))?;
    
    info!("DWARF information written to: {}", output_file.display());
    Ok(())
}

fn generate_gdb_script(
    generator: &mut LlvmCodeGenerator,
    input_path: &Path,
    source: &str,
    output_dir: &str,
) -> Result<(), Error> {
    info!("Generating GDB debugging script");
    
    // Generate IR to populate debug info
    let _ir = generator.generate_ir_with_debug(input_path.to_path_buf(), source)?;
    
    // Create a mock debug manager to generate GDB commands
    let debug_manager = cursed::debug::DebugInfoManager::new(cursed::debug::DebugConfig::default());
    
    let executable_path = Path::new(output_dir).join(
        input_path.file_stem().unwrap_or_default()
    );
    
    let commands = DebugUtils::generate_gdb_commands(&executable_path, &debug_manager);
    
    let mut script = String::new();
    script.push_str("# GDB debugging script for CURSED program\n");
    script.push_str(&format!("# Generated for: {}\n\n", input_path.display()));
    
    for command in commands {
        script.push_str(&command);
        script.push('\n');
    }
    
    script.push_str("\n# Additional CURSED-specific debugging commands\n");
    script.push_str("define cursed-info\n");
    script.push_str("  info functions\n");
    script.push_str("  info variables\n");
    script.push_str("  info sources\n");
    script.push_str("end\n");
    
    let output_file = Path::new(output_dir).join(
        input_path.file_stem().unwrap_or_default()
    ).with_extension("gdb");
    
    fs::write(&output_file, script)
        .map_err(|e| Error::Io(e.into()))?;
    
    info!("GDB script written to: {}", output_file.display());
    Ok(())
}

fn generate_lldb_script(
    generator: &mut LlvmCodeGenerator,
    input_path: &Path,
    source: &str,
    output_dir: &str,
) -> Result<(), Error> {
    info!("Generating LLDB debugging script");
    
    // Generate IR to populate debug info
    let _ir = generator.generate_ir_with_debug(input_path.to_path_buf(), source)?;
    
    // Create a mock debug manager to generate LLDB commands
    let debug_manager = cursed::debug::DebugInfoManager::new(cursed::debug::DebugConfig::default());
    
    let executable_path = Path::new(output_dir).join(
        input_path.file_stem().unwrap_or_default()
    );
    
    let commands = DebugUtils::generate_lldb_commands(&executable_path, &debug_manager);
    
    let mut script = String::new();
    script.push_str("# LLDB debugging script for CURSED program\n");
    script.push_str(&format!("# Generated for: {}\n\n", input_path.display()));
    
    for command in commands {
        script.push_str(&command);
        script.push('\n');
    }
    
    script.push_str("\n# Additional CURSED-specific debugging commands\n");
    script.push_str("command alias cursed-info frame info\n");
    script.push_str("command alias cursed-vars frame variable\n");
    
    let output_file = Path::new(output_dir).join(
        input_path.file_stem().unwrap_or_default()
    ).with_extension("lldb");
    
    fs::write(&output_file, script)
        .map_err(|e| Error::Io(e.into()))?;
    
    info!("LLDB script written to: {}", output_file.display());
    Ok(())
}

fn generate_vscode_config(
    generator: &mut LlvmCodeGenerator,
    input_path: &Path,
    output_dir: &str,
) -> Result<(), Error> {
    info!("Generating VS Code debugging configuration");
    
    let executable_path = Path::new(output_dir).join(
        input_path.file_stem().unwrap_or_default()
    );
    
    let source_root = input_path.parent().unwrap_or(Path::new("."));
    let config = DebugUtils::generate_vscode_launch_config(&executable_path, source_root);
    
    let config_str = serde_json::to_string_pretty(&config)
        .map_err(|e| Error::Compile(format!("Failed to serialize VS Code config: {}", e)))?;
    
    let output_file = Path::new(output_dir).join("launch.json");
    
    fs::write(&output_file, config_str)
        .map_err(|e| Error::Io(e.into()))?;
    
    info!("VS Code configuration written to: {}", output_file.display());
    Ok(())
}

fn generate_debug_report(
    generator: &mut LlvmCodeGenerator,
    input_path: &Path,
    source: &str,
    output_dir: &str,
) -> Result<(), Error> {
    info!("Generating comprehensive debug report");
    
    // Generate IR to populate debug info
    let _ir = generator.generate_ir_with_debug(input_path.to_path_buf(), source)?;
    
    let mut report = String::new();
    report.push_str("# CURSED Debug Information Report\n\n");
    report.push_str(&format!("## Source File: {}\n", input_path.display()));
    report.push_str(&format!("## Generated: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    
    report.push_str("## Debug Statistics\n\n");
    report.push_str(&format!("```\n{}\n```\n\n", generator.debug_statistics()));
    
    report.push_str("## Line Table\n\n");
    let line_table = generator.line_table();
    if line_table.is_empty() {
        report.push_str("No line table entries generated.\n\n");
    } else {
        report.push_str("| Line | Location |\n");
        report.push_str("|------|----------|\n");
        for (line, info) in line_table {
            report.push_str(&format!("| {} | {} |\n", line, info));
        }
        report.push_str("\n");
    }
    
    report.push_str("## Configuration\n\n");
    report.push_str("```json\n");
    let config_json = serde_json::to_string_pretty(generator.debug_config())
        .unwrap_or_else(|_| "Configuration serialization failed".to_string());
    report.push_str(&config_json);
    report.push_str("\n```\n\n");
    
    report.push_str("## Validation Results\n\n");
    match generator.validate_debug() {
        Ok(()) => report.push_str("✅ Debug information validation passed\n\n"),
        Err(errors) => {
            report.push_str("❌ Debug information validation failed:\n\n");
            for error in errors {
                report.push_str(&format!("- {}\n", error));
            }
            report.push_str("\n");
        }
    }
    
    let output_file = Path::new(output_dir).join(
        input_path.file_stem().unwrap_or_default()
    ).with_extension("debug_report.md");
    
    fs::write(&output_file, report)
        .map_err(|e| Error::Io(e.into()))?;
    
    info!("Debug report written to: {}", output_file.display());
    Ok(())
}
