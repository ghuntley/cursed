/// CURSED Language Linter CLI
/// 
/// Command-line interface for the CURSED linter with comprehensive
/// configuration options and output formats.

use clap::{Parser, ValueEnum};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use colored::*;
use cursed::tools::linter::{CursedLinter, LinterConfig, LintSeverity, LintCategory};

#[derive(Parser)]
#[command(name = "cursed-lint")]
#[command(about = "CURSED language linter - enforces style and detects issues")]
#[command(version = "1.0.0")]
struct Cli {
    /// Files or directories to lint
    #[arg(value_name = "FILES")]
    files: Vec<PathBuf>,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,

    /// Only show errors and warnings (suppress suggestions and info)
    #[arg(short, long)]
    quiet: bool,

    /// Show all issues including suggestions and info
    #[arg(short, long)]
    verbose: bool,

    /// Use strict linting rules
    #[arg(long)]
    strict: bool,

    /// Use relaxed linting rules  
    #[arg(long)]
    relaxed: bool,

    /// Maximum line length
    #[arg(long)]
    max_line_length: Option<usize>,

    /// Maximum function length
    #[arg(long)]
    max_function_length: Option<usize>,

    /// Maximum function parameters
    #[arg(long)]
    max_function_parameters: Option<usize>,

    /// Disable specific rules (comma-separated)
    #[arg(long)]
    disable: Option<String>,

    /// Only check specific categories (comma-separated)
    #[arg(long)]
    categories: Option<String>,

    /// Exit with non-zero code only on errors (not warnings)
    #[arg(long)]
    error_only: bool,

    /// Show statistics summary
    #[arg(long)]
    stats: bool,

    /// Show rule documentation
    #[arg(long)]
    show_rules: bool,
}

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Human,     // Human-readable colored output
    Json,      // JSON format for tool integration
    Compact,   // Compact one-line format
    Checkstyle, // Checkstyle XML format
}

fn main() {
    let cli = Cli::parse();

    if cli.show_rules {
        show_rule_documentation();
        return;
    }

    if cli.files.is_empty() {
        eprintln!("{}", "Error: No files specified".red());
        eprintln!("Use --help for usage information");
        process::exit(1);
    }

    // Build configuration
    let mut config = build_config(&cli);

    // Create linter
    let mut linter = CursedLinter::new(config);

    // Process files
    let mut total_issues = 0;
    let mut total_errors = 0;
    let mut total_warnings = 0;
    let mut total_suggestions = 0;
    let mut total_files = 0;

    for file_path in &cli.files {
        match process_path(&mut linter, file_path, &cli) {
            Ok((issues, errors, warnings, suggestions, files)) => {
                total_issues += issues;
                total_errors += errors;
                total_warnings += warnings;
                total_suggestions += suggestions;
                total_files += files;
            }
            Err(e) => {
                eprintln!("{}: {}", "Error".red(), e);
                process::exit(1);
            }
        }
    }

    // Show statistics if requested
    if cli.stats {
        show_statistics(total_files, total_issues, total_errors, total_warnings, total_suggestions);
    }

    // Exit with appropriate code
    let exit_code = if cli.error_only {
        if total_errors > 0 { 1 } else { 0 }
    } else {
        if total_errors > 0 || total_warnings > 0 { 1 } else { 0 }
    };

    process::exit(exit_code);
}

fn build_config(cli: &Cli) -> LinterConfig {
    let mut config = if cli.strict {
        LinterConfig::strict()
    } else if cli.relaxed {
        LinterConfig::relaxed()
    } else {
        LinterConfig::default()
    };

    // Apply CLI overrides
    if let Some(max_line_length) = cli.max_line_length {
        config.max_line_length = max_line_length;
    }

    if let Some(max_function_length) = cli.max_function_length {
        config.max_function_length = max_function_length;
    }

    if let Some(max_function_parameters) = cli.max_function_parameters {
        config.max_function_parameters = max_function_parameters;
    }

    // Disable specified rules
    if let Some(disabled_rules) = &cli.disable {
        for rule in disabled_rules.split(',') {
            config.disable_rule(rule.trim());
        }
    }

    config
}

fn process_path(
    linter: &mut CursedLinter, 
    path: &Path, 
    cli: &Cli
) -> Result<(), Error>> {
    let mut total_issues = 0;
    let mut total_errors = 0;
    let mut total_warnings = 0;
    let mut total_suggestions = 0;
    let mut total_files = 0;

    if path.is_file() {
        if is_cursed_file(path) {
            let (issues, errors, warnings, suggestions) = process_file(linter, path, cli)?;
            total_issues += issues;
            total_errors += errors;
            total_warnings += warnings;
            total_suggestions += suggestions;
            total_files += 1;
        }
    } else if path.is_dir() {
        // Recursively process directory
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && is_cursed_file(&path) {
                let (issues, errors, warnings, suggestions) = process_file(linter, &path, cli)?;
                total_issues += issues;
                total_errors += errors;
                total_warnings += warnings;
                total_suggestions += suggestions;
                total_files += 1;
            } else if path.is_dir() {
                let (sub_issues, sub_errors, sub_warnings, sub_suggestions, sub_files) = 
                    process_path(linter, &path, cli)?;
                total_issues += sub_issues;
                total_errors += sub_errors;
                total_warnings += sub_warnings;
                total_suggestions += sub_suggestions;
                total_files += sub_files;
            }
        }
    }

    Ok((total_issues, total_errors, total_warnings, total_suggestions, total_files))
}

fn process_file(
    linter: &mut CursedLinter,
    file_path: &Path,
    cli: &Cli,
) -> Result<(), Error>> {
    let content = fs::read_to_string(file_path)?;
    let results = linter.lint(&content)?;

    // Filter results based on CLI options
    let filtered_results: Vec<_> = results.iter()
        .filter(|result| {
            // Filter by categories if specified
            if let Some(categories) = &cli.categories {
                let allowed_categories: Vec<_> = categories
                    .split(',')
                    .map(|s| s.trim().to_lowercase())
                    .collect();
                
                let category_name = format!("{:?}", result.category).to_lowercase();
                if !allowed_categories.contains(&category_name) {
                    return false;
                }
            }

            // Filter by severity based on quiet/verbose flags
            if cli.quiet {
                matches!(result.severity, LintSeverity::Error | LintSeverity::Warning)
            } else {
                true
            }
        })
        .collect();

    // Count by severity
    let errors = filtered_results.iter().filter(|r| r.severity == LintSeverity::Error).count();
    let warnings = filtered_results.iter().filter(|r| r.severity == LintSeverity::Warning).count();
    let suggestions = filtered_results.iter().filter(|r| r.severity == LintSeverity::Suggestion).count();

    // Output results
    if !filtered_results.is_empty() {
        output_results(file_path, &filtered_results, &cli.format);
    }

    Ok((filtered_results.len(), errors, warnings, suggestions))
}

fn output_results(
    file_path: &Path,
    results: &[&cursed::tools::linter::LintResult],
    format: &OutputFormat,
) {
    match format {
        OutputFormat::Human => output_human(file_path, results),
        OutputFormat::Json => output_json(file_path, results),
        OutputFormat::Compact => output_compact(file_path, results),
        OutputFormat::Checkstyle => output_checkstyle(file_path, results),
    }
}

fn output_human(file_path: &Path, results: &[&cursed::tools::linter::LintResult]) {
    println!("\n{}", file_path.display().to_string().bold());
    
    for result in results {
        let severity_color = match result.severity {
            LintSeverity::Error => "red",
            LintSeverity::Warning => "yellow", 
            LintSeverity::Suggestion => "blue",
            LintSeverity::Info => "cyan",
        };

        let severity_text = format!("{:?}", result.severity).to_lowercase();
        let category_text = format!("{:?}", result.category).to_lowercase();

        println!(
            "  {}:{} {} [{}] {} ({})",
            result.line.to_string().bright_black(),
            result.column.to_string().bright_black(),
            severity_text.color(severity_color),
            result.rule_id.bright_black(),
            result.message,
            category_text.bright_black()
        );

        if let Some(suggestion) = &result.suggestion {
            println!("    {}: {}", "suggestion".green(), suggestion);
        }

        if let Some(help) = &result.help_text {
            println!("    {}: {}", "help".blue(), help);
        }
    }
}

fn output_json(file_path: &Path, results: &[&cursed::tools::linter::LintResult]) {
    use serde_json::{json, Map, Value};

    let file_results: Vec<Value> = results.iter().map(|result| {
        let mut obj = Map::new();
        obj.insert("rule_id".to_string(), json!(result.rule_id));
        obj.insert("severity".to_string(), json!(format!("{:?}", result.severity)));
        obj.insert("category".to_string(), json!(format!("{:?}", result.category)));
        obj.insert("message".to_string(), json!(result.message));
        obj.insert("line".to_string(), json!(result.line));
        obj.insert("column".to_string(), json!(result.column));
        
        if let Some(suggestion) = &result.suggestion {
            obj.insert("suggestion".to_string(), json!(suggestion));
        }
        
        if let Some(help) = &result.help_text {
            obj.insert("help".to_string(), json!(help));
        }

        Value::Object(obj)
    }).collect();

    let output = json!({
        "file": file_path.display().to_string(),
        "issues": file_results
    });

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

fn output_compact(file_path: &Path, results: &[&cursed::tools::linter::LintResult]) {
    for result in results {
        println!(
            "{}:{}:{}: {} [{}] {}",
            file_path.display(),
            result.line,
            result.column,
            format!("{:?}", result.severity).to_lowercase(),
            result.rule_id,
            result.message
        );
    }
}

fn output_checkstyle(file_path: &Path, results: &[&cursed::tools::linter::LintResult]) {
    println!("<file name=\"{}\">", file_path.display());
    
    for result in results {
        let severity = match result.severity {
            LintSeverity::Error => "error",
            LintSeverity::Warning => "warning",
            LintSeverity::Suggestion => "info",
            LintSeverity::Info => "info",
        };

        println!(
            "  <error line=\"{}\" column=\"{}\" severity=\"{}\" message=\"{}\" source=\"{}\"/>",
            result.line,
            result.column,
            severity,
            html_escape(&result.message),
            result.rule_id
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

fn is_cursed_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext == "csd")
        .unwrap_or(false)
}

fn show_statistics(files: usize, issues: usize, errors: usize, warnings: usize, suggestions: usize) {
    println!("\n{}", "Linting Statistics".bold());
    println!("  Files processed: {}", files);
    println!("  Total issues: {}", issues);
    println!("  Errors: {}", errors.to_string().red());
    println!("  Warnings: {}", warnings.to_string().yellow());
    println!("  Suggestions: {}", suggestions.to_string().blue());
    
    if files > 0 {
        let avg_issues = issues as f64 / files as f64;
        println!("  Average issues per file: {:.1}", avg_issues);
    }
}

fn show_rule_documentation() {
    println!("{}", "CURSED Linter Rules".bold());
    println!();
    
    let rules = vec![
        ("line_too_long", "Style", "Lines that exceed the maximum length"),
        ("trailing_whitespace", "Style", "Lines with trailing whitespace"),
        ("mixed_indentation", "Style", "Mixed tabs and spaces for indentation"),
        ("go_style_comment", "GenZSlang", "Go-style comments instead of CURSED syntax"),
        ("go_style_block_comment", "GenZSlang", "Go-style block comments"),
        ("go_style_keyword", "GenZSlang", "Go keywords instead of CURSED equivalents"),
        ("non_ascii_identifier", "Style", "Identifiers with non-ASCII characters"),
        ("identifier_too_long", "Style", "Overly long identifiers"),
        ("single_letter_variable", "Naming", "Non-descriptive single-letter variables"),
        ("too_many_parameters", "Complexity", "Functions with too many parameters"),
        ("mixed_naming_style", "Naming", "Mixed camelCase and snake_case"),
        ("generic_function_name", "Naming", "Overly generic function names"),
        ("deep_nesting", "Complexity", "Deeply nested code blocks"),
        ("unused_variable", "Correctness", "Variables that are declared but not used"),
        ("unused_import", "Correctness", "Imports that are not used"),
        ("empty_package_name", "Naming", "Empty package names"),
        ("invalid_package_name", "Naming", "Package names with invalid characters"),
        ("long_comment", "Style", "Overly long comments"),
        ("unnecessary_escape", "Style", "Unnecessary escape sequences in strings"),
        ("long_string_literal", "Style", "Very long string literals"),
    ];

    for (rule_id, category, description) in rules {
        println!("  {} ({})", rule_id.cyan(), category.yellow());
        println!("    {}", description);
        println!();
    }

    println!("Use --disable <rule1,rule2> to disable specific rules");
    println!("Use --categories <cat1,cat2> to check only specific categories");
}
