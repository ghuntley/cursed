/// CURSED Code Formatter CLI Tool
/// 
/// A command-line interface for formatting CURSED source code files with
/// configurable formatting rules and comprehensive options.

use cursed::tools::formatter::{CursedFormatter, FormatterConfig, BraceStyle, OperatorSpacing, CommaSpacing};
use cursed::error::Error;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug)]
struct CliConfig {
    files: Vec<PathBuf>,
    check_only: bool,
    show_diff: bool,
    write_in_place: bool,
    recursive: bool,
    config_file: Option<PathBuf>,
    formatter_config: FormatterConfig,
    verbose: bool,
    quiet: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            check_only: false,
            show_diff: false,
            write_in_place: false,
            recursive: false,
            config_file: None,
            formatter_config: FormatterConfig::default(),
            verbose: false,
            quiet: false,
        }
    }
}

fn main() {
    let config = match parse_args() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    };

    let result = run_formatter(config);
    match result {
        Ok(exit_code) => process::exit(exit_code),
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    }
}

fn parse_args() -> Result<CliConfig, String> {
    let args: Vec<String> = env::args().collect();
    let mut config = CliConfig::default();
    let mut i = 1;

    if args.len() == 1 {
        print_usage();
        process::exit(0);
    }

    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_usage();
                process::exit(0);
            }
            "--version" | "-V" => {
                println!("cursed-fmt {}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            }
            "--check" | "-c" => {
                config.check_only = true;
            }
            "--diff" | "-d" => {
                config.show_diff = true;
            }
            "--write" | "-w" => {
                config.write_in_place = true;
            }
            "--recursive" | "-r" => {
                config.recursive = true;
            }
            "--verbose" | "-v" => {
                config.verbose = true;
            }
            "--quiet" | "-q" => {
                config.quiet = true;
            }
            "--config" => {
                i += 1;
                if i >= args.len() {
                    return Err("--config requires a file path".to_string());
                }
                config.config_file = Some(PathBuf::from(&args[i]));
            }
            "--indent-size" => {
                i += 1;
                if i >= args.len() {
                    return Err("--indent-size requires a number".to_string());
                }
                config.formatter_config.indent_size = args[i].parse()
                    .map_err(|_| "Invalid indent size")?;
            }
            "--line-width" => {
                i += 1;
                if i >= args.len() {
                    return Err("--line-width requires a number".to_string());
                }
                config.formatter_config.line_width = args[i].parse()
                    .map_err(|_| "Invalid line width")?;
            }
            "--brace-style" => {
                i += 1;
                if i >= args.len() {
                    return Err("--brace-style requires a style".to_string());
                }
                config.formatter_config.brace_style = match args[i].as_str() {
                    "same-line" => BraceStyle::SameLine,
                    "next-line" => BraceStyle::NextLine,
                    "next-line-unindented" => BraceStyle::NextLineUnindented,
                    _ => return Err("Invalid brace style. Use: same-line, next-line, next-line-unindented".to_string()),
                };
            }
            "--operator-spacing" => {
                i += 1;
                if i >= args.len() {
                    return Err("--operator-spacing requires with-spaces or without-spaces".to_string());
                }
                config.formatter_config.operator_spacing = match args[i].as_str() {
                    "with-spaces" => OperatorSpacing::WithSpaces,
                    "without-spaces" => OperatorSpacing::WithoutSpaces,
                    _ => return Err("Invalid operator spacing. Use: with-spaces, without-spaces".to_string()),
                };
            }
            "--comma-spacing" => {
                i += 1;
                if i >= args.len() {
                    return Err("--comma-spacing requires with-spaces or without-spaces".to_string());
                }
                config.formatter_config.comma_spacing = match args[i].as_str() {
                    "with-spaces" => CommaSpacing::WithSpaces,
                    "without-spaces" => CommaSpacing::WithoutSpaces,
                    _ => return Err("Invalid comma spacing. Use: with-spaces, without-spaces".to_string()),
                };
            }
            arg if arg.starts_with('-') => {
                return Err(format!("Unknown option: {}", arg));
            }
            _ => {
                config.files.push(PathBuf::from(&args[i]));
            }
        }
        i += 1;
    }

    // Load config file if specified
    if let Some(config_path) = &config.config_file {
        load_config_file(&mut config, config_path)?;
    }

    // If no files specified and not reading from stdin
    if config.files.is_empty() && atty::is(atty::Stream::Stdin) {
        return Err("No input files specified. Use --help for usage information.".to_string());
    }

    Ok(config)
}

fn load_config_file(config: &mut CliConfig, path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    
    // Basic TOML-like parsing for configuration
    for line in content.split("\n") {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim();
            let value = line[eq_pos + 1..].trim().trim_matches('"');
            
            match key {
                "indent_size" => {
                    config.formatter_config.indent_size = value.parse()
                        .map_err(|_| format!("Invalid indent_size in config: {}", value))?;
                }
                "line_width" => {
                    config.formatter_config.line_width = value.parse()
                        .map_err(|_| format!("Invalid line_width in config: {}", value))?;
                }
                "brace_style" => {
                    config.formatter_config.brace_style = match value {
                        "same-line" => BraceStyle::SameLine,
                        "next-line" => BraceStyle::NextLine,
                        "next-line-unindented" => BraceStyle::NextLineUnindented,
                        _ => return Err(format!("Invalid brace_style in config: {}", value)),
                    };
                }
                _ => {} // Ignore unknown keys
            }
        }
    }
    
    Ok(())
}

fn run_formatter(config: CliConfig) -> Result<(), Error> {
    let mut formatter = CursedFormatter::new(config.formatter_config.clone());
    let mut total_files = 0;
    let mut changed_files = 0;
    let mut error_files = 0;

    if config.files.is_empty() {
        // Read from stdin
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)
            .map_err(|e| Error::General(format!("Failed to read from stdin: {}", e)))?;

        let result = formatter.format(&input)?;
        
        if config.check_only {
            if result.changes_made {
                if !config.quiet {
                    eprintln!("stdin: would be reformatted");
                }
                return Ok(1);
            }
        } else if config.show_diff {
            print_diff("<stdin>", &input, &result.formatted_code);
        } else {
            print!("{}", result.formatted_code);
        }
        
        return Ok(0);
    }

    let mut files_to_process = Vec::new();
    for file_path in &config.files {
        if file_path.is_dir() && config.recursive {
            collect_cursed_files(file_path, &mut files_to_process)?;
        } else if file_path.extension().map_or(false, |ext| ext == "csd") {
            files_to_process.push(file_path.clone());
        } else if file_path.is_file() {
            files_to_process.push(file_path.clone());
        }
    }

    for file_path in files_to_process {
        total_files += 1;
        
        if config.verbose {
            eprintln!("Processing: {}", file_path.display());
        }

        match process_file(&mut formatter, &file_path, &config) {
            Ok(changed) => {
                if changed {
                    changed_files += 1;
                }
            }
            Err(e) => {
                if !config.quiet {
                    eprintln!("Error processing {}: {}", file_path.display(), e);
                }
                error_files += 1;
            }
        }
    }

    if !config.quiet {
        if config.check_only {
            if changed_files > 0 {
                eprintln!("{} file(s) would be reformatted", changed_files);
            } else {
                eprintln!("All files are properly formatted");
            }
        } else {
            eprintln!("Processed {} file(s), {} changed, {} error(s)", 
                     total_files, changed_files, error_files);
        }
    }

    // Exit code: 0 = success, 1 = files need formatting (check mode), 2 = errors
    if error_files > 0 {
        Ok(2)
    } else if config.check_only && changed_files > 0 {
        Ok(1)
    } else {
        Ok(0)
    }
}

fn collect_cursed_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in fs::read_dir(dir)
        .map_err(|e| Error::General(format!("Failed to read directory {}: {}", dir.display(), e)))? {
        let entry = entry
            .map_err(|e| Error::General(format!("Failed to read directory entry: {}", e)))?;
        let path = entry.path();
        
        if path.is_dir() {
            collect_cursed_files(&path, files)?;
        } else if path.extension().map_or(false, |ext| ext == "csd") {
            files.push(path);
        }
    }
    Ok(())
}

fn process_file(formatter: &mut CursedFormatter, file_path: &Path, config: &CliConfig) -> Result<(), Error> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| Error::General(format!("Failed to read file: {}", e)))?;

    let result = formatter.format(&content)?;

    if !result.formatting_errors.is_empty() && config.verbose {
        for error in &result.formatting_errors {
            eprintln!("Warning in {}: {}", file_path.display(), error);
        }
    }

    if config.check_only {
        if result.changes_made {
            if !config.quiet {
                eprintln!("{}: would be reformatted", file_path.display());
            }
            return Ok(true);
        }
    } else if config.show_diff {
        if result.changes_made {
            print_diff(&file_path.to_string_lossy(), &content, &result.formatted_code);
            return Ok(true);
        }
    } else if config.write_in_place {
        if result.changes_made {
            fs::write(file_path, &result.formatted_code)
                .map_err(|e| Error::General(format!("Failed to write file: {}", e)))?;
            if !config.quiet {
                eprintln!("Formatted: {}", file_path.display());
            }
            return Ok(true);
        }
    } else {
        // Output to stdout
        print!("{}", result.formatted_code);
    }

    Ok(false)
}

fn print_diff(filename: &str, original: &str, formatted: &str) {
    println!("--- {}", filename);
    println!("+++ {}", filename);
    
    let original_lines: Vec<&str> = original.split("\n").collect();
    let formatted_lines: Vec<&str> = formatted.split("\n").collect();
    
    // Simple diff - just show different lines
    let max_lines = original_lines.len().max(formatted_lines.len());
    
    for i in 0..max_lines {
        let orig_line = original_lines.get(i).unwrap_or(&"");
        let fmt_line = formatted_lines.get(i).unwrap_or(&"");
        
        if orig_line != fmt_line {
            if !orig_line.is_empty() {
                println!("-{}", orig_line);
            }
            if !fmt_line.is_empty() {
                println!("+{}", fmt_line);
            }
        }
    }
}

fn print_usage() {
    println!("cursed-fmt {}", env!("CARGO_PKG_VERSION"));
    println!("A code formatter for the CURSED programming language");
    println!();
    println!("USAGE:");
    println!("    cursed-fmt [OPTIONS] [FILES]...");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help                   Show this help message");
    println!("    -V, --version                Show version information");
    println!("    -c, --check                  Check if files are formatted (exit 1 if not)");
    println!("    -d, --diff                   Show formatting differences");
    println!("    -w, --write                  Write formatted output in-place");
    println!("    -r, --recursive              Process directories recursively");
    println!("    -v, --verbose                Verbose output");
    println!("    -q, --quiet                  Suppress non-error output");
    println!();
    println!("FORMATTING OPTIONS:");
    println!("    --config FILE                Load configuration from file");
    println!("    --indent-size N              Set indentation size (default: 4)");
    println!("    --line-width N               Set maximum line width (default: 100)");
    println!("    --brace-style STYLE          Set brace style:");
    println!("                                   same-line (default)");
    println!("                                   next-line");
    println!("                                   next-line-unindented");
    println!("    --operator-spacing MODE      Set operator spacing:");
    println!("                                   with-spaces (default)");
    println!("                                   without-spaces");
    println!("    --comma-spacing MODE         Set comma spacing:");
    println!("                                   with-spaces (default)");
    println!("                                   without-spaces");
    println!();
    println!("EXAMPLES:");
    println!("    cursed-fmt main.csd                    Format file to stdout");
    println!("    cursed-fmt -w main.csd                 Format file in-place");
    println!("    cursed-fmt -c src/                     Check if files need formatting");
    println!("    cursed-fmt -d main.csd                 Show diff of changes needed");
    println!("    cursed-fmt --indent-size 2 main.csd   Use 2-space indentation");
    println!();
    println!("If no files are specified, input is read from stdin.");
}
