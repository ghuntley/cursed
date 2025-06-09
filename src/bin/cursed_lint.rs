//! CURSED Language Linter CLI
//!
//! Command-line interface for the CURSED language linter.
//! Provides comprehensive linting capabilities with configurable rules and output formats.

use clap::{Arg, Command, ArgMatches};
use cursed::tools::{CursedLinter, LinterConfig, LintSeverity};
use cursed::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
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
    Command::new("cursed_lint")
        .version("1.0.0")
        .about("CURSED Language Linter - Analyze CURSED code for style and quality issues")
        .author("CURSED Dev Team")
        .arg(
            Arg::new("files")
                .help("CURSED source files to lint")
                .required(true)
                .num_args(1..)
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
            Arg::new("max-line-length")
                .long("max-line-length")
                .help("Maximum line length")
                .value_name("LENGTH")
                .default_value("100"),
        )
        .arg(
            Arg::new("max-complexity")
                .long("max-complexity")
                .help("Maximum function complexity")
                .value_name("COMPLEXITY")
                .default_value("10"),
        )
        .arg(
            Arg::new("no-genz-naming")
                .long("no-genz-naming")
                .help("Disable Gen Z naming convention checks")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-unused")
                .long("no-unused")
                .help("Disable unused variable/function checks")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-unreachable")
                .long("no-unreachable")
                .help("Disable unreachable code checks")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-style")
                .long("no-style")
                .help("Disable style consistency checks")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-dead-code")
                .long("no-dead-code")
                .help("Disable dead code detection")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("severity")
                .long("severity")
                .short('s')
                .help("Minimum severity level to report")
                .value_name("LEVEL")
                .value_parser(["info", "warning", "error"])
                .default_value("info"),
        )
        .arg(
            Arg::new("format")
                .long("format")
                .short('f')
                .help("Output format")
                .value_name("FORMAT")
                .value_parser(["human", "json", "checkstyle"])
                .default_value("human"),
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .help("Only show summary")
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
            Arg::new("recursive")
                .long("recursive")
                .short('r')
                .help("Recursively search directories for .csd files")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("fix")
                .long("fix")
                .help("Automatically fix issues where possible")
                .action(clap::ArgAction::SetTrue),
        )
}

fn run(matches: &ArgMatches) -> Result<(), Error> {
    // Parse configuration
    let config = build_config(matches)?;
    
    // Get files to lint
    let files = collect_files(matches)?;
    
    if files.is_empty() {
        info!("No CURSED files found to lint");
        return Ok(());
    }

    info!("Linting {} files", files.len());

    // Create linter
    let mut linter = CursedLinter::with_config(config);
    
    // Lint all files
    let mut total_issues = 0;
    let mut error_count = 0;
    let format = matches.get_one::<String>("format").unwrap();
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");

    for file_path in &files {
        if verbose {
            info!("Linting file: {}", file_path.display());
        }

        match linter.lint_file(file_path) {
            Ok(issues) => {
                total_issues += issues.len();
                
                // Count errors
                error_count += issues.iter()
                    .filter(|issue| issue.severity == LintSeverity::Error)
                    .count();

                if !quiet {
                    print_issues(&issues, file_path, format);
                }
            }
            Err(e) => {
                error!("Failed to lint {}: {}", file_path.display(), e);
                error_count += 1;
            }
        }
    }

    // Print summary
    if !quiet || verbose {
        let summary = linter.summary();
        println!("\n{}", summary);
        println!("Total files processed: {}", files.len());
        println!("Total issues found: {}", total_issues);
    }

    // Exit with non-zero code if there are errors
    if error_count > 0 {
        process::exit(1);
    }

    Ok(())
}

fn build_config(matches: &ArgMatches) -> Result<LinterConfig, Error> {
    let max_line_length = matches.get_one::<String>("max-line-length")
        .unwrap()
        .parse::<usize>()
        .map_err(|_| Error::Parser {
            location: cursed::error::SourceLocation::new(0, 0),
            message: "Invalid max-line-length value".to_string(),
        })?;

    let max_complexity = matches.get_one::<String>("max-complexity")
        .unwrap()
        .parse::<usize>()
        .map_err(|_| Error::Parser {
            location: cursed::error::SourceLocation::new(0, 0),
            message: "Invalid max-complexity value".to_string(),
        })?;

    let min_severity = match matches.get_one::<String>("severity").unwrap().as_str() {
        "info" => LintSeverity::Info,
        "warning" => LintSeverity::Warning,
        "error" => LintSeverity::Error,
        _ => LintSeverity::Info,
    };

    Ok(LinterConfig {
        max_function_complexity: max_complexity,
        max_line_length,
        enforce_genz_naming: !matches.get_flag("no-genz-naming"),
        check_unused_variables: !matches.get_flag("no-unused"),
        check_unreachable_code: !matches.get_flag("no-unreachable"),
        check_style_consistency: !matches.get_flag("no-style"),
        check_dead_code: !matches.get_flag("no-dead-code"),
        min_severity,
    })
}

fn collect_files(matches: &ArgMatches) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    let recursive = matches.get_flag("recursive");
    
    let file_args: Vec<&String> = matches.get_many::<String>("files").unwrap().collect();
    
    for file_arg in file_args {
        let path = Path::new(file_arg);
        
        if path.is_file() {
            if is_cursed_file(path) {
                files.push(path.to_path_buf());
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

fn print_issues(issues: &[cursed::tools::LintIssue], file_path: &Path, format: &str) {
    if issues.is_empty() {
        return;
    }

    match format {
        "json" => print_issues_json(issues, file_path),
        "checkstyle" => print_issues_checkstyle(issues, file_path),
        _ => print_issues_human(issues, file_path),
    }
}

fn print_issues_human(issues: &[cursed::tools::LintIssue], file_path: &Path) {
    println!("\n{}", file_path.display());
    for issue in issues {
        println!("  {}", issue);
    }
}

fn print_issues_json(issues: &[cursed::tools::LintIssue], file_path: &Path) {
    for issue in issues {
        let location = &issue.location;
        println!(
            r#"{{"file":"{}","line":{},"column":{},"severity":"{}","rule":"{}","message":"{}"}}"#,
            file_path.display(),
            location.line + 1, // Convert to 1-based
            location.column + 1, // Convert to 1-based
            issue.severity,
            issue.rule_name,
            issue.message.replace('"', "\\\"")
        );
    }
}

fn print_issues_checkstyle(issues: &[cursed::tools::LintIssue], file_path: &Path) {
    if issues.is_empty() {
        return;
    }

    println!(r#"<file name="{}">"#, file_path.display());
    for issue in issues {
        let location = &issue.location;
        println!(
            r#"  <error line="{}" column="{}" severity="{}" message="{}" source="{}"/>"#,
            location.line + 1, // Convert to 1-based
            location.column + 1, // Convert to 1-based
            issue.severity,
            html_escape(&issue.message),
            issue.rule_name
        );
    }
    println!("</file>");
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
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
    fn test_html_escape() {
        assert_eq!(html_escape("test"), "test");
        assert_eq!(html_escape("a < b"), "a &lt; b");
        assert_eq!(html_escape("a & b"), "a &amp; b");
        assert_eq!(html_escape("\"quoted\""), "&quot;quoted&quot;");
    }
}
