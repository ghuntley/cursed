use std::path::PathBuf;
use std::env;
use clap::{Arg, Command, ArgMatches};
use cursed::tools::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            handle_init(sub_matches).await?;
        }
        Some(("fmt", sub_matches)) => {
            handle_format(sub_matches)?;
        }
        Some(("doc", sub_matches)) => {
            handle_docs(sub_matches)?;
        }
        Some(("profile", sub_matches)) => {
            handle_profile(sub_matches).await?;
        }
        Some(("pkg", sub_matches)) => {
            handle_package(sub_matches).await?;
        }
        Some(("analyze", sub_matches)) => {
            handle_analyze(sub_matches).await?;
        }
        Some(("debug", sub_matches)) => {
            handle_debug(sub_matches)?;
        }
        _ => {
            println!("Use --help for usage information");
        }
    }

    Ok(())
}

fn build_cli() -> Command {
    Command::new("cursed-tools")
        .version("1.0.0")
        .author("CURSED Development Team")
        .about("Comprehensive development tools for CURSED language")
        .subcommand(
            Command::new("init")
                .about("Initialize new CURSED project with tooling")
                .arg(
                    Arg::new("name")
                        .help("Project name")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("path")
                        .help("Project directory")
                        .long("path")
                        .value_name("DIR")
                        .default_value(".")
                )
        )
        .subcommand(
            Command::new("fmt")
                .about("Format CURSED source code")
                .arg(
                    Arg::new("files")
                        .help("Files or directories to format")
                        .multiple_values(true)
                        .index(1)
                )
                .arg(
                    Arg::new("check")
                        .help("Check if files are formatted without modifying")
                        .long("check")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("recursive")
                        .help("Format directories recursively")
                        .short('r')
                        .long("recursive")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("config")
                        .help("Configuration file")
                        .long("config")
                        .value_name("FILE")
                )
        )
        .subcommand(
            Command::new("doc")
                .about("Generate documentation")
                .arg(
                    Arg::new("source")
                        .help("Source directory")
                        .long("source")
                        .value_name("DIR")
                        .default_value("src")
                )
                .arg(
                    Arg::new("output")
                        .help("Output directory")
                        .long("output")
                        .value_name("DIR")
                        .default_value("docs")
                )
                .arg(
                    Arg::new("format")
                        .help("Output format")
                        .long("format")
                        .value_name("FORMAT")
                        .possible_values(["html", "markdown", "json", "pdf"])
                        .default_value("html")
                )
                .arg(
                    Arg::new("private")
                        .help("Include private items")
                        .long("private")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("profile")
                .about("Profile application performance")
                .arg(
                    Arg::new("program")
                        .help("CURSED program to profile")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("output")
                        .help("Output file for profile report")
                        .long("output")
                        .value_name("FILE")
                        .default_value("profile.html")
                )
                .arg(
                    Arg::new("format")
                        .help("Report format")
                        .long("format")
                        .value_name("FORMAT")
                        .possible_values(["html", "json", "flamegraph"])
                        .default_value("html")
                )
                .arg(
                    Arg::new("sample-rate")
                        .help("Sampling rate (Hz)")
                        .long("sample-rate")
                        .value_name("RATE")
                        .default_value("100")
                )
                .arg(
                    Arg::new("duration")
                        .help("Profile duration (seconds)")
                        .long("duration")
                        .value_name("SECONDS")
                )
        )
        .subcommand(
            Command::new("pkg")
                .about("Package management")
                .subcommand(
                    Command::new("add")
                        .about("Add dependency")
                        .arg(
                            Arg::new("package")
                                .help("Package name")
                                .required(true)
                                .index(1)
                        )
                        .arg(
                            Arg::new("version")
                                .help("Package version")
                                .long("version")
                                .value_name("VERSION")
                        )
                        .arg(
                            Arg::new("dev")
                                .help("Add as development dependency")
                                .long("dev")
                                .action(clap::ArgAction::SetTrue)
                        )
                )
                .subcommand(
                    Command::new("install")
                        .about("Install dependencies")
                )
                .subcommand(
                    Command::new("update")
                        .about("Update dependencies")
                )
                .subcommand(
                    Command::new("outdated")
                        .about("Check for outdated dependencies")
                )
                .subcommand(
                    Command::new("publish")
                        .about("Publish package")
                        .arg(
                            Arg::new("token")
                                .help("Registry token")
                                .long("token")
                                .value_name("TOKEN")
                                .required(true)
                        )
                )
        )
        .subcommand(
            Command::new("analyze")
                .about("Comprehensive project analysis")
                .arg(
                    Arg::new("project")
                        .help("Project directory")
                        .long("project")
                        .value_name("DIR")
                        .default_value(".")
                )
                .arg(
                    Arg::new("report")
                        .help("Output report file")
                        .long("report")
                        .value_name("FILE")
                        .default_value("analysis.json")
                )
        )
        .subcommand(
            Command::new("debug")
                .about("Debug information tools")
                .arg(
                    Arg::new("source")
                        .help("Source file")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("output")
                        .help("Debug output directory")
                        .long("output")
                        .value_name("DIR")
                        .default_value("debug")
                )
                .arg(
                    Arg::new("level")
                        .help("Debug level")
                        .long("level")
                        .value_name("LEVEL")
                        .possible_values(["none", "line-numbers", "full", "optimized"])
                        .default_value("full")
                )
        )
}

async fn handle_init(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.get_one::<String>("name").unwrap();
    let path = PathBuf::from(matches.get_one::<String>("path").unwrap());

    let mut tools = CursedTools::new();
    tools.init_project(name, &path).await?;

    Ok(())
}

fn handle_format(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let files: Vec<_> = matches.get_many::<String>("files")
        .unwrap_or_default()
        .collect();
    let check_only = matches.get_flag("check");
    let recursive = matches.get_flag("recursive");

    let config = if let Some(config_file) = matches.get_one::<String>("config") {
        load_formatter_config(config_file)?
    } else {
        FormatterConfig::default()
    };

    let mut formatter = CursedFormatter::new(config);

    if files.is_empty() {
        // Format current directory
        let current_dir = env::current_dir()?;
        if check_only {
            check_formatting(&mut formatter, &current_dir, recursive)?;
        } else {
            formatter.format_directory(&current_dir, recursive)?;
        }
    } else {
        for file_str in files {
            let path = PathBuf::from(file_str);
            if path.is_dir() {
                if check_only {
                    check_formatting(&mut formatter, &path, recursive)?;
                } else {
                    formatter.format_directory(&path, recursive)?;
                }
            } else if path.extension().map_or(false, |ext| ext == "csd") {
                if check_only {
                    let original = std::fs::read_to_string(&path)?;
                    let formatted = formatter.format_source(&original)?;
                    if original != formatted {
                        println!("❌ {} needs formatting", path.display());
                    } else {
                        println!("✅ {} is properly formatted", path.display());
                    }
                } else {
                    let formatted = formatter.format_file(&path)?;
                    std::fs::write(&path, formatted)?;
                    println!("✅ Formatted {}", path.display());
                }
            }
        }
    }

    Ok(())
}

fn handle_docs(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let source_dir = PathBuf::from(matches.get_one::<String>("source").unwrap());
    let output_dir = PathBuf::from(matches.get_one::<String>("output").unwrap());
    let format = matches.get_one::<String>("format").unwrap();
    let include_private = matches.get_flag("private");

    let mut config = DocConfig::default();
    config.include_private = include_private;
    config.output_formats = vec![format.clone()];

    let mut doc_generator = DocGenerator::new(output_dir, config);
    doc_generator.generate_docs(&source_dir)?;

    Ok(())
}

async fn handle_profile(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let program = PathBuf::from(matches.get_one::<String>("program").unwrap());
    let output = matches.get_one::<String>("output").unwrap();
    let format = matches.get_one::<String>("format").unwrap();
    let sample_rate: u64 = matches.get_one::<String>("sample-rate").unwrap().parse()?;

    let mut config = ProfilerConfig::default();
    config.sample_rate = sample_rate;
    config.output_format = format.clone();
    config.output_file = output.clone();

    if let Some(duration_str) = matches.get_one::<String>("duration") {
        let duration_secs: u64 = duration_str.parse()?;
        config.profile_duration = Some(std::time::Duration::from_secs(duration_secs));
    }

    let mut tools = CursedTools::new();
    tools.profiler = Profiler::new(config);
    
    let _report = tools.profile_application(&program).await?;

    Ok(())
}

async fn handle_package(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let mut tools = CursedTools::new();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            let version = sub_matches.get_one::<String>("version").unwrap_or(&"*".to_string());
            let dev = sub_matches.get_flag("dev");

            tools.package_manager.add_dependency(package, version, dev)?;
        }
        Some(("install", _)) => {
            tools.package_manager.install_dependencies().await?;
        }
        Some(("update", _)) => {
            tools.package_manager.update_dependencies().await?;
        }
        Some(("outdated", _)) => {
            tools.package_manager.check_outdated().await?;
        }
        Some(("publish", sub_matches)) => {
            let token = sub_matches.get_one::<String>("token").unwrap();
            tools.package_manager.publish_package(token).await?;
        }
        _ => {
            println!("Use 'cursed-tools pkg --help' for package management commands");
        }
    }

    Ok(())
}

async fn handle_analyze(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = PathBuf::from(matches.get_one::<String>("project").unwrap());
    let report_file = matches.get_one::<String>("report").unwrap();

    let mut tools = CursedTools::new();
    let analysis = tools.analyze_project(&project_dir).await?;

    // Generate analysis report
    let report = serde_json::to_string_pretty(&analysis)?;
    std::fs::write(report_file, report)?;

    println!("📊 Project Analysis Results:");
    println!("   Format Issues: {}", analysis.format_issues);
    println!("   Documentation Coverage: {:.1}%", analysis.doc_coverage);
    println!("   Outdated Dependencies: {}", analysis.outdated_dependencies);
    println!("   Code Quality Score: {:.1}/10", analysis.code_quality_score);

    Ok(())
}

fn handle_debug(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let source_file = matches.get_one::<String>("source").unwrap();
    let output_dir = PathBuf::from(matches.get_one::<String>("output").unwrap());
    let level_str = matches.get_one::<String>("level").unwrap();

    let debug_level = match level_str.as_str() {
        "none" => DebugLevel::None,
        "line-numbers" => DebugLevel::LineNumbers,
        "full" => DebugLevel::Full,
        "optimized" => DebugLevel::Optimized,
        _ => DebugLevel::Full,
    };

    let mut config = DebugConfig::default();
    config.debug_level = debug_level;
    config.output_path = output_dir;

    let mut debug_generator = DebugInfoGenerator::new(config);
    
    // This would integrate with the LLVM module from compilation
    println!("🔧 Debug information generation for: {}", source_file);
    println!("📁 Output directory: {}", debug_generator.config.output_path.display());
    println!("🎯 Debug level: {:?}", debug_generator.config.debug_level);

    Ok(())
}

fn load_formatter_config(config_file: &str) -> Result<FormatterConfig, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(config_file)?;
    let config: FormatterConfig = toml::from_str(&content)?;
    Ok(config)
}

fn check_formatting(formatter: &mut CursedFormatter, path: &std::path::Path, recursive: bool) -> Result<(), Box<dyn std::error::Error>> {
    let results = formatter.format_directory(path, recursive)?;
    let mut needs_formatting = false;

    for result in results {
        match result.status {
            formatter::FormatStatus::Formatted => {
                println!("❌ {} needs formatting", result.file_path.display());
                needs_formatting = true;
            }
            formatter::FormatStatus::NoChanges => {
                println!("✅ {} is properly formatted", result.file_path.display());
            }
            formatter::FormatStatus::Error(ref error) => {
                eprintln!("💥 Error checking {}: {}", result.file_path.display(), error);
            }
        }
    }

    if needs_formatting {
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_construction() {
        let app = build_cli();
        assert_eq!(app.get_name(), "cursed-tools");
        
        // Test that all subcommands are present
        let subcommands: Vec<_> = app.get_subcommands().map(|s| s.get_name()).collect();
        assert!(subcommands.contains(&"init"));
        assert!(subcommands.contains(&"fmt"));
        assert!(subcommands.contains(&"doc"));
        assert!(subcommands.contains(&"profile"));
        assert!(subcommands.contains(&"pkg"));
        assert!(subcommands.contains(&"analyze"));
        assert!(subcommands.contains(&"debug"));
    }

    #[test]
    fn test_debug_level_parsing() {
        assert_eq!(format!("{:?}", DebugLevel::None), "None");
        assert_eq!(format!("{:?}", DebugLevel::Full), "Full");
    }
}
