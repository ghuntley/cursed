//! Enhanced CURSED Language Linter CLI
//!
//! Production-ready command-line interface for the CURSED language linter.
//! Provides comprehensive linting capabilities with configurable rules and output formats.

use clap::{Arg, Command, ArgMatches, value_parser};
use cursed::linter::{
    config::{LinterConfig, ConfigLoader, CliConfig},
    engine::{LintEngine, LintSeverity},
    reporter::OutputFormat,
};
use cursed::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use tracing::{debug, error, info, warn};
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
    Command::new("cursed-lint")
        .version("2.0.0")
        .about("CURSED Language Linter - Production-ready code analysis tool")
        .author("CURSED Dev Team")
        .arg(
            Arg::new("files")
                .help("CURSED source files or directories to lint")
                .required(true)
                .num_args(1..)
                .value_name("FILE_OR_DIR"),
        )
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .help("Path to configuration file")
                .value_name("CONFIG_FILE"),
        )
        .arg(
            Arg::new("generate-config")
                .long("generate-config")
                .help("Generate default configuration file")
                .value_name("OUTPUT_FILE"),
        )
        .arg(
            Arg::new("max-line-length")
                .long("max-line-length")
                .help("Maximum line length")
                .value_name("LENGTH")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("indent-size")
                .long("indent-size")
                .help("Number of spaces per indent")
                .value_name("SIZE")
                .value_parser(value_parser!(usize)),
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
                .value_parser(["human", "json", "checkstyle", "sarif"])
                .default_value("human"),
        )
        .arg(
            Arg::new("disable")
                .long("disable")
                .short('d')
                .help("Disable specific rules (comma-separated)")
                .value_name("RULES"),
        )
        .arg(
            Arg::new("enable-only")
                .long("enable-only")
                .help("Enable only specific rule categories")
                .value_name("CATEGORIES")
                .value_parser(["style", "correctness", "performance", "complexity", "cursed"]),
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
        .arg(
            Arg::new("check")
                .long("check")
                .help("Check mode: exit with non-zero if issues found")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("diff")
                .long("diff")
                .help("Show diff for auto-fixes without applying them")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .help("Only show summary and errors")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Show detailed output and progress")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("stats")
                .long("stats")
                .help("Show detailed statistics")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("fail-on")
                .long("fail-on")
                .help("Exit with non-zero code on specified severity level")
                .value_name("LEVEL")
                .value_parser(["info", "warning", "error"])
                .default_value("error"),
        )
        .arg(
            Arg::new("jobs")
                .long("jobs")
                .short('j')
                .help("Number of parallel jobs (0 = auto)")
                .value_name("COUNT")
                .value_parser(value_parser!(usize))
                .default_value("0"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Write output to file instead of stdout")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("list-rules")
                .long("list-rules")
                .help("List all available rules and exit")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("explain")
                .long("explain")
                .help("Explain a specific rule")
                .value_name("RULE_NAME"),
        )
        .arg(
            Arg::new("no-ignore")
                .long("no-ignore")
                .help("Ignore .cursedlintignore files")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("exclude")
                .long("exclude")
                .help("Exclude files matching pattern (glob)")
                .value_name("PATTERN")
                .action(clap::ArgAction::Append),
        )
}

fn run(matches: &ArgMatches) -> Result<(), Error> {
    // Handle special commands first
    if matches.get_flag("list-rules") {
        return list_rules();
    }

    if let Some(rule_name) = matches.get_one::<String>("explain") {
        return explain_rule(rule_name);
    }

    if let Some(config_path) = matches.get_one::<String>("generate-config") {
        return generate_config(config_path);
    }

    // Load configuration
    let config = load_configuration(matches)?;
    
    // Create linter engine
    let mut engine = LintEngine::with_config(config);
    
    // Get files to lint
    let files = collect_files(matches)?;
    
    if files.is_empty() {
        if !matches.get_flag("quiet") {
            println!("No CURSED files found to lint");
        }
        return Ok(());
    }

    let verbose = matches.get_flag("verbose");
    let quiet = matches.get_flag("quiet");
    let stats = matches.get_flag("stats");

    if verbose {
        info!("Linting {} files", files.len());
    }

    // Run linting
    let results = if matches.get_one::<usize>("jobs").copied().unwrap_or(0) > 1 {
        lint_parallel(&mut engine, &files, matches)?
    } else {
        lint_sequential(&mut engine, &files, matches)?
    };

    // Generate and output report
    handle_output(&engine, &results, matches)?;

    // Check exit conditions
    let statistics = engine.statistics();
    let fail_on = parse_severity(matches.get_one::<String>("fail-on").unwrap())?;
    let should_fail = match fail_on {
        LintSeverity::Error => statistics.has_errors(),
        LintSeverity::Warning => {
            statistics.issues_by_severity.get(&LintSeverity::Error).unwrap_or(&0) > &0 ||
            statistics.issues_by_severity.get(&LintSeverity::Warning).unwrap_or(&0) > &0
        }
        LintSeverity::Info => statistics.total_issues > 0,
    };

    if !quiet || verbose || stats {
        println!("\n{}", statistics.summary());
    }

    if should_fail || matches.get_flag("check") && statistics.total_issues > 0 {
        process::exit(1);
    }

    Ok(())
}

fn load_configuration(matches: &ArgMatches) -> Result<LinterConfig, Error> {
    let config_file = matches.get_one::<String>("config").map(|s| Path::new(s));
    
    // Build CLI overrides
    let cli_config = CliConfig {
        max_line_length: matches.get_one::<usize>("max-line-length").copied(),
        indent_size: matches.get_one::<usize>("indent-size").copied(),
        min_severity: matches.get_one::<String>("severity").map(|s| parse_severity(s)).transpose()?,
        output_format: matches.get_one::<String>("format").map(|s| parse_format(s)).transpose()?,
        auto_fix: matches.get_flag("fix"),
        disabled_rules: matches.get_one::<String>("disable").map(|s| {
            s.split(',').map(|r| r.trim().to_string()).collect()
        }),
    };
    
    ConfigLoader::load_with_precedence(config_file, true, Some(&cli_config))
}

fn collect_files(matches: &ArgMatches) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    let recursive = matches.get_flag("recursive");
    let exclude_patterns: Vec<&String> = matches.get_many("exclude").unwrap_or_default().collect();
    
    let file_args: Vec<&String> = matches.get_many::<String>("files").unwrap().collect();
    
    for file_arg in file_args {
        let path = Path::new(file_arg);
        
        if path.is_file() {
            if is_cursed_file(path) && !is_excluded(path, &exclude_patterns) {
                files.push(path.to_path_buf());
            }
        } else if path.is_dir() {
            if recursive {
                collect_files_recursive(path, &mut files, &exclude_patterns)?;
            } else {
                // Only process .csd files in the directory
                for entry in fs::read_dir(path).map_err(Error::IoError)? {
                    let entry = entry.map_err(Error::IoError)?;
                    let entry_path = entry.path();
                    if entry_path.is_file() && is_cursed_file(&entry_path) && !is_excluded(&entry_path, &exclude_patterns) {
                        files.push(entry_path);
                    }
                }
            }
        } else {
            return Err(Error::Configuration(
                format!("File or directory not found: {}", file_arg)
            ));
        }
    }
    
    Ok(files)
}

fn collect_files_recursive(dir: &Path, files: &mut Vec<PathBuf>, exclude_patterns: &[&String]) -> Result<(), Error> {
    for entry in fs::read_dir(dir).map_err(Error::IoError)? {
        let entry = entry.map_err(Error::IoError)?;
        let path = entry.path();
        
        if path.is_dir() {
            // Skip hidden directories and common ignore directories
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                if dir_name.starts_with('.') || dir_name == "target" || dir_name == "node_modules" {
                    continue;
                }
            }
            collect_files_recursive(&path, files, exclude_patterns)?;
        } else if is_cursed_file(&path) && !is_excluded(&path, exclude_patterns) {
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

fn is_excluded(path: &Path, exclude_patterns: &[&String]) -> bool {
    for pattern in exclude_patterns {
        // Simple glob-like matching (could be enhanced with a proper glob library)
        if path.to_string_lossy().contains(pattern.as_str()) {
            return true;
        }
    }
    false
}

fn lint_sequential(
    engine: &mut LintEngine,
    files: &[PathBuf],
    matches: &ArgMatches
) -> Result<Vec<(PathBuf, Vec<cursed::linter::engine::LintIssue>)>, Error> {
    let verbose = matches.get_flag("verbose");
    let mut results = Vec::new();
    
    for (i, file_path) in files.iter().enumerate() {
        if verbose {
            println!("Linting [{}/{}]: {}", i + 1, files.len(), file_path.display());
        }
        
        match engine.lint_file(file_path) {
            Ok(issues) => {
                results.push((file_path.clone(), issues));
            }
            Err(e) => {
                error!("Failed to lint {}: {}", file_path.display(), e);
                // Continue with other files
            }
        }
    }
    
    Ok(results)
}

fn lint_parallel(
    engine: &mut LintEngine,
    files: &[PathBuf],
    matches: &ArgMatches
) -> Result<Vec<(PathBuf, Vec<cursed::linter::engine::LintIssue>)>, Error> {
    // For now, just use sequential linting
    // In a real implementation, you'd use rayon or similar for parallel processing
    lint_sequential(engine, files, matches)
}

fn handle_output(
    engine: &LintEngine,
    results: &[(PathBuf, Vec<cursed::linter::engine::LintIssue>)],
    matches: &ArgMatches
) -> Result<(), Error> {
    let quiet = matches.get_flag("quiet");
    
    if let Some(output_file) = matches.get_one::<String>("output") {
        // Write to file
        let report = engine.generate_report(results)?;
        fs::write(output_file, report).map_err(Error::IoError)?;
    } else if !quiet {
        // Print to stdout
        engine.print_issues(results);
    }
    
    Ok(())
}

fn parse_severity(severity: &str) -> Result<LintSeverity, Error> {
    match severity.to_lowercase().as_str() {
        "info" => Ok(LintSeverity::Info),
        "warning" => Ok(LintSeverity::Warning),
        "error" => Ok(LintSeverity::Error),
        _ => Err(Error::Configuration(format!("Invalid severity: {}", severity))),
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, Error> {
    match format.to_lowercase().as_str() {
        "human" => Ok(OutputFormat::Human),
        "json" => Ok(OutputFormat::Json),
        "checkstyle" => Ok(OutputFormat::Checkstyle),
        "sarif" => Ok(OutputFormat::Sarif),
        _ => Err(Error::Configuration(format!("Invalid format: {}", format))),
    }
}

fn list_rules() -> Result<(), Error> {
    use cursed::linter::rules::{LintRuleSet, RuleCategory};
    use cursed::linter::config::LinterConfig;
    
    let config = LinterConfig::default();
    let rule_set = LintRuleSet::from_config(&config);
    
    println!("Available linting rules:\n");
    
    for category in [
        RuleCategory::Style,
        RuleCategory::Correctness,
        RuleCategory::Performance,
        RuleCategory::Complexity,
        RuleCategory::CursedSpecific,
    ] {
        let rules = rule_set.rules_by_category(category.clone());
        if !rules.is_empty() {
            println!("{}:", category);
            for rule in rules {
                println!("  {} - {} ({})", rule.name(), rule.description(), rule.default_severity());
            }
            println!();
        }
    }
    
    Ok(())
}

fn explain_rule(rule_name: &str) -> Result<(), Error> {
    use cursed::linter::rules::LintRuleSet;
    use cursed::linter::config::LinterConfig;
    
    let config = LinterConfig::default();
    let rule_set = LintRuleSet::from_config(&config);
    
    if let Some(rule) = rule_set.rule_by_name(rule_name) {
        println!("Rule: {}", rule.name());
        println!("Category: {}", rule.category());
        println!("Default Severity: {}", rule.default_severity());
        println!("Description: {}", rule.description());
        
        if rule.supports_auto_fix() {
            println!("Auto-fix: Supported");
        } else {
            println!("Auto-fix: Not supported");
        }
    } else {
        println!("Rule '{}' not found. Use --list-rules to see available rules.", rule_name);
        process::exit(1);
    }
    
    Ok(())
}

fn generate_config(output_path: &str) -> Result<(), Error> {
    use cursed::linter::config::ConfigLoader;
    
    ConfigLoader::generate_default_config(output_path)?;
    println!("Generated default configuration at: {}", output_path);
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
    fn test_parse_severity() {
        assert_eq!(parse_severity("info").unwrap(), LintSeverity::Info);
        assert_eq!(parse_severity("INFO").unwrap(), LintSeverity::Info);
        assert_eq!(parse_severity("warning").unwrap(), LintSeverity::Warning);
        assert_eq!(parse_severity("error").unwrap(), LintSeverity::Error);
        assert!(parse_severity("invalid").is_err());
    }

    #[test]
    fn test_parse_format() {
        assert_eq!(parse_format("human").unwrap(), OutputFormat::Human);
        assert_eq!(parse_format("JSON").unwrap(), OutputFormat::Json);
        assert_eq!(parse_format("checkstyle").unwrap(), OutputFormat::Checkstyle);
        assert!(parse_format("invalid").is_err());
    }

    #[test]
    fn test_exclude_patterns() {
        let patterns = vec![&"test".to_string(), &"generated".to_string()];
        assert!(is_excluded(Path::new("src/test/file.csd"), &patterns));
        assert!(is_excluded(Path::new("generated/output.csd"), &patterns));
        assert!(!is_excluded(Path::new("src/main.csd"), &patterns));
    }
}
