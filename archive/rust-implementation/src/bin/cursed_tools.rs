use std::path::PathBuf;
use std::env;
use clap::{Arg, Command, ArgMatches};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            handle_init(sub_matches).await?;
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
                        .value_parser(["html", "json", "flamegraph"])
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
}

async fn handle_init(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.get_one::<String>("name").unwrap();
    let path = PathBuf::from(matches.get_one::<String>("path").unwrap());

    let mut tools = cursed::tools::CursedTools::new();
    tools.init_project(name, &path).await?;

    Ok(())
}

async fn handle_profile(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let program = PathBuf::from(matches.get_one::<String>("program").unwrap());
    let output = matches.get_one::<String>("output").unwrap();
    let format = matches.get_one::<String>("format").unwrap();
    let sample_rate: u64 = matches.get_one::<String>("sample-rate").unwrap().parse()?;

    let mut config = cursed::tools::ProfilerConfig::default();
    config.sample_rate = sample_rate;
    config.output_format = format.clone();
    config.output_file = output.clone();

    if let Some(duration_str) = matches.get_one::<String>("duration") {
        let duration_secs: u64 = duration_str.parse()?;
        config.profile_duration = Some(std::time::Duration::from_secs(duration_secs));
    }

    let mut tools = cursed::tools::CursedTools::new();
    tools.profiler = cursed::tools::Profiler::new(config);
    
    let _report = tools.profile_application(&program).await?;

    Ok(())
}

async fn handle_package(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let mut tools = cursed::tools::CursedTools::new();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            let default_version = "*".to_string();
            let version = sub_matches.get_one::<String>("version").unwrap_or(&default_version);
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

    let mut tools = cursed::tools::CursedTools::new();
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
        assert!(subcommands.contains(&"profile"));
        assert!(subcommands.contains(&"pkg"));
        assert!(subcommands.contains(&"analyze"));
    }
}
