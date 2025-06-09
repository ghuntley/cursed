//! CURSED Language Formatter CLI
//!
//! Command-line interface for the CURSED language formatter.
//! Provides comprehensive code formatting with configurable style options.

use clap::{Arg, Command, ArgMatches};
use cursed::tools::formatter::{CursedFormatter, FormatterConfig, BraceStyle};
use cursed::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::io::{self, Write, Read};
use tracing::{debug, error, info};
use tracing_subscriber;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let matches = build_cli().get_matches();
    
    if let Err(e) = run(&matches) {
        error!("Error: {}", e);
        process::exit(1);
    }
}

fn build_cli() -> Command {
    Command::new("cursed-fmt")
        .version("1.0.0")
        .about("CURSED Language Formatter - Format CURSED code according to style guidelines")
        .author("CURSED Dev Team")
        .arg(
            Arg::new("files")
                .help("CURSED source files to format")
                .num_args(0..)
                .value_name("FILE"),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .help("Path to configuration file")
                .value_name("CONFIG_FILE"),
        )
        .arg(
            Arg::new("check")
                .long("check")
                .help("Check if files are formatted without making changes")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("diff")
                .long("diff")
                .help("Show formatting differences without making changes")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("write")
                .long("write")
                .short('w')
                .help("Write formatting changes back to files")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("indent-size")
                .long("indent-size")
                .help("Number of spaces for each indentation level")
                .value_name("SIZE")
                .default_value("4"),
        )
        .arg(
            Arg::new("line-width")
                .long("line-width")
                .help("Maximum line width before wrapping")
                .value_name("WIDTH")
                .default_value("100"),
        )
        .arg(
            Arg::new("brace-style")
                .long("brace-style")
                .help("Brace placement style")
                .value_name("STYLE")
                .value_parser(["same-line", "next-line", "next-line-unindented"])
                .default_value("same-line"),
        )
        .arg(
            Arg::new("no-spaces-around-operators")
                .long("no-spaces-around-operators")
                .help("Don't insert spaces around operators")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-space-after-comma")
                .long("no-space-after-comma")
                .help("Don't insert space after commas")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-format-comments")
                .long("no-format-comments")
                .help("Don't format comments")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("max-empty-lines")
                .long("max-empty-lines")
                .help("Maximum number of consecutive empty lines")
                .value_name("COUNT")
                .default_value("2"),
        )
        .arg(
            Arg::new("recursive")
                .long("recursive")
                .short('r')
                .help("Recursively search directories for .csd files")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Show detailed output")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .help("Only show errors and summary")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("jobs")
                .long("jobs")
                .short('j')
                .help("Number of parallel jobs (0 = auto)")
                .value_name("JOBS")
                .default_value("0"),
        )
        .arg(
            Arg::new("summary")
                .long("summary")
                .help("Show formatting summary")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Output format")
                .value_name("FORMAT")
                .value_parser(["human", "json", "summary"])
                .default_value("human"),
        )
}

fn run(matches: &ArgMatches) -> Result<(), Error> {
    // Parse configuration
    let config = build_config(matches)?;
    
    // Get files to format
    let files = collect_files(matches)?;
    
    // Read from stdin if no files provided
    let files = if files.is_empty() {
        if atty::is(atty::Stream::Stdin) {
            // Terminal input, show help
            println!("No files provided. Use --help for usage information.");
            return Ok(());
        } else {
            // Stdin input
            return format_stdin(&config, matches);
        }
    } else {
        files
    };

    info!("Formatting {} files", files.len());

    // Determine operation mode
    let check_mode = matches.get_flag("check");
    let diff_mode = matches.get_flag("diff");
    let write_mode = matches.get_flag("write");
    let verbose = matches.get_flag("verbose");
    let quiet = matches.get_flag("quiet");
    let show_summary = matches.get_flag("summary");
    let output_format = matches.get_one::<String>("output").unwrap();

    // Default to check mode if no explicit mode specified
    let mode = if write_mode {
        FormatMode::Write
    } else if diff_mode {
        FormatMode::Diff
    } else {
        FormatMode::Check
    };

    // Format all files
    let mut total_files = 0;
    let mut changed_files = 0;
    let mut error_count = 0;
    let mut total_lines = 0;

    for file_path in &files {
        if verbose && !quiet {
            info!("Processing file: {}", file_path.display());
        }

        match format_file(file_path, &config, &mode) {
            Ok(result) => {
                total_files += 1;
                total_lines += result.lines_processed;
                
                if result.changed {
                    changed_files += 1;
                    
                    match mode {
                        FormatMode::Check => {
                            if !quiet {
                                match output_format.as_str() {
                                    "json" => {
                                        println!(r#"{{"file":"{}","formatted":false,"changed":true}}"#, file_path.display());
                                    }
                                    _ => {
                                        println!("Would format: {}", file_path.display());
                                    }
                                }
                            }
                        }
                        FormatMode::Diff => {
                            show_diff(file_path, &result.formatted_code)?;
                        }
                        FormatMode::Write => {
                            if !quiet {
                                match output_format.as_str() {
                                    "json" => {
                                        println!(r#"{{"file":"{}","formatted":true,"changed":true}}"#, file_path.display());
                                    }
                                    _ => {
                                        println!("Formatted: {}", file_path.display());
                                    }
                                }
                            }
                        }
                    }
                } else if verbose && !quiet {
                    match output_format.as_str() {
                        "json" => {
                            println!(r#"{{"file":"{}","formatted":true,"changed":false}}"#, file_path.display());
                        }
                        _ => {
                            println!("Already formatted: {}", file_path.display());
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to format {}: {}", file_path.display(), e);
                error_count += 1;
            }
        }
    }

    // Print summary
    if show_summary || (!quiet && output_format == "human") {
        match output_format.as_str() {
            "json" => {
                println!(
                    r#"{{"summary":{{"total_files":{},"changed_files":{},"error_count":{},"total_lines":{}}}}}"#,
                    total_files, changed_files, error_count, total_lines
                );
            }
            _ => {
                println!("\nFormatting Summary:");
                println!("  Total files processed: {}", total_files);
                println!("  Files that needed formatting: {}", changed_files);
                println!("  Total lines processed: {}", total_lines);
                if error_count > 0 {
                    println!("  Errors encountered: {}", error_count);
                }
                
                match mode {
                    FormatMode::Check if changed_files > 0 => {
                        println!("  Some files need formatting. Run with --write to apply changes.");
                    }
                    FormatMode::Write if changed_files > 0 => {
                        println!("  Successfully formatted {} files.", changed_files);
                    }
                    _ => {
                        println!("  All files are properly formatted.");
                    }
                }
            }
        }
    }

    // Exit with non-zero code if there are errors or unformatted files in check mode
    if error_count > 0 || (mode == FormatMode::Check && changed_files > 0) {
        process::exit(1);
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
enum FormatMode {
    Check,
    Diff,
    Write,
}

fn build_config(matches: &ArgMatches) -> Result<FormatterConfig, Error> {
    let indent_size = matches.get_one::<String>("indent-size")
        .unwrap()
        .parse::<usize>()
        .map_err(|_| Error::Parser {
            location: cursed::error::SourceLocation::new(0, 0),
            message: "Invalid indent-size value".to_string(),
        })?;

    let line_width = matches.get_one::<String>("line-width")
        .unwrap()
        .parse::<usize>()
        .map_err(|_| Error::Parser {
            location: cursed::error::SourceLocation::new(0, 0),
            message: "Invalid line-width value".to_string(),
        })?;

    let max_empty_lines = matches.get_one::<String>("max-empty-lines")
        .unwrap()
        .parse::<usize>()
        .map_err(|_| Error::Parser {
            location: cursed::error::SourceLocation::new(0, 0),
            message: "Invalid max-empty-lines value".to_string(),
        })?;

    let brace_style = match matches.get_one::<String>("brace-style").unwrap().as_str() {
        "same-line" => BraceStyle::SameLine,
        "next-line" => BraceStyle::NextLine,
        "next-line-unindented" => BraceStyle::NextLineUnindented,
        _ => BraceStyle::SameLine,
    };

    Ok(FormatterConfig {
        indent_size,
        line_width,
        brace_style,
        spaces_around_operators: !matches.get_flag("no-spaces-around-operators"),
        space_after_comma: !matches.get_flag("no-space-after-comma"),
        format_comments: !matches.get_flag("no-format-comments"),
        preserve_empty_lines: true,
        max_empty_lines,
    })
}

fn collect_files(matches: &ArgMatches) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    let recursive = matches.get_flag("recursive");
    
    if let Some(file_args) = matches.get_many::<String>("files") {
        let file_args: Vec<&String> = file_args.collect();
        
        for file_arg in file_args {
            let path = Path::new(file_arg);
            
            if path.is_file() {
                if is_cursed_file(path) {
                    files.push(path.to_path_buf());
                } else {
                    return Err(Error::Parser {
                        location: cursed::error::SourceLocation::new(0, 0),
                        message: format!("Not a CURSED file: {}", file_arg),
                    });
                }
            } else if path.is_dir() {
                if recursive {
                    collect_files_recursive(path, &mut files)?;
                } else {
                    // Only process .csd files in the directory
                    for entry in fs::read_dir(path).map_err(Error::IoError)? {
                        let entry = entry.map_err(Error::IoError)?;
                        let entry_path = entry.path();
                        if entry_path.is_file() && is_cursed_file(&entry_path) {
                            files.push(entry_path);
                        }
                    }
                }
            } else {
                return Err(Error::Parser {
                    location: cursed::error::SourceLocation::new(0, 0),
                    message: format!("File or directory not found: {}", file_arg),
                });
            }
        }
    }
    
    Ok(files)
}

fn collect_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in fs::read_dir(dir).map_err(Error::IoError)? {
        let entry = entry.map_err(Error::IoError)?;
        let path = entry.path();
        
        if path.is_dir() {
            collect_files_recursive(&path, files)?;
        } else if is_cursed_file(&path) {
            files.push(path);
        }
    }
    Ok(())
}

fn is_cursed_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("csd"))
        .unwrap_or(false)
}

fn format_stdin(config: &FormatterConfig, matches: &ArgMatches) -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).map_err(Error::IoError)?;
    
    let mut formatter = CursedFormatter::new(config.clone());
    let result = formatter.format(&input)?;
    
    let output_format = matches.get_one::<String>("output").unwrap();
    let check_mode = matches.get_flag("check");
    
    if check_mode {
        if result.changed {
            match output_format.as_str() {
                "json" => {
                    println!(r#"{{"stdin":true,"formatted":false,"changed":true}}"#);
                }
                _ => {
                    eprintln!("stdin: needs formatting");
                }
            }
            process::exit(1);
        } else {
            match output_format.as_str() {
                "json" => {
                    println!(r#"{{"stdin":true,"formatted":true,"changed":false}}"#);
                }
                _ => {
                    eprintln!("stdin: already formatted");
                }
            }
        }
    } else {
        print!("{}", result.formatted_code);
    }
    
    Ok(())
}

fn format_file(file_path: &Path, config: &FormatterConfig, mode: &FormatMode) -> Result<cursed::tools::formatter::FormatterResult, Error> {
    let source = fs::read_to_string(file_path).map_err(Error::IoError)?;
    
    let mut formatter = CursedFormatter::new(config.clone());
    let result = formatter.format(&source)?;
    
    if *mode == FormatMode::Write && result.changed {
        fs::write(file_path, &result.formatted_code).map_err(Error::IoError)?;
    }
    
    Ok(result)
}

fn show_diff(file_path: &Path, formatted_code: &str) -> Result<(), Error> {
    let original = fs::read_to_string(file_path).map_err(Error::IoError)?;
    
    println!("--- {}", file_path.display());
    println!("+++ {} (formatted)", file_path.display());
    
    let original_lines: Vec<&str> = original.lines().collect();
    let formatted_lines: Vec<&str> = formatted_code.lines().collect();
    
    // Simple diff implementation
    for (i, (orig, fmt)) in original_lines.iter().zip(formatted_lines.iter()).enumerate() {
        if orig != fmt {
            println!("@@ -{},{} +{},{} @@", i + 1, 1, i + 1, 1);
            println!("-{}", orig);
            println!("+{}", fmt);
        }
    }
    
    // Handle different line counts
    if original_lines.len() != formatted_lines.len() {
        let start = original_lines.len().min(formatted_lines.len());
        
        if original_lines.len() > formatted_lines.len() {
            for (i, line) in original_lines[start..].iter().enumerate() {
                println!("@@ -{},{} +{},{} @@", start + i + 1, 1, start + i + 1, 0);
                println!("-{}", line);
            }
        } else {
            for (i, line) in formatted_lines[start..].iter().enumerate() {
                println!("@@ -{},{} +{},{} @@", start + i + 1, 0, start + i + 1, 1);
                println!("+{}", line);
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cursed_file() {
        assert!(is_cursed_file(Path::new("test.csd")));
        assert!(is_cursed_file(Path::new("TEST.CSD")));
        assert!(!is_cursed_file(Path::new("test.rs")));
        assert!(!is_cursed_file(Path::new("test")));
    }

    #[test]
    fn test_build_config_defaults() {
        let app = build_cli();
        let matches = app.try_get_matches_from(vec!["cursed-fmt"]).unwrap();
        let config = build_config(&matches).unwrap();
        
        assert_eq!(config.indent_size, 4);
        assert_eq!(config.line_width, 100);
        assert_eq!(config.brace_style, BraceStyle::SameLine);
        assert!(config.spaces_around_operators);
        assert!(config.space_after_comma);
    }

    #[test]
    fn test_brace_style_parsing() {
        let app = build_cli();
        
        let matches = app.clone().try_get_matches_from(vec!["cursed-fmt", "--brace-style", "next-line"]).unwrap();
        let config = build_config(&matches).unwrap();
        assert_eq!(config.brace_style, BraceStyle::NextLine);
        
        let matches = app.try_get_matches_from(vec!["cursed-fmt", "--brace-style", "next-line-unindented"]).unwrap();
        let config = build_config(&matches).unwrap();
        assert_eq!(config.brace_style, BraceStyle::NextLineUnindented);
    }

    #[test]
    fn test_format_mode_detection() {
        let app = build_cli();
        
        let matches = app.clone().try_get_matches_from(vec!["cursed-fmt", "--check"]).unwrap();
        assert!(matches.get_flag("check"));
        assert!(!matches.get_flag("write"));
        
        let matches = app.try_get_matches_from(vec!["cursed-fmt", "--write"]).unwrap();
        assert!(matches.get_flag("write"));
        assert!(!matches.get_flag("check"));
    }
}
