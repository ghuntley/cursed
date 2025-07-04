/// Clap API Fixes Demo
/// 
/// Demonstrates the fixed clap API usage, showing the migration from
/// deprecated methods to the new clap 4 API.

use clap::{Command, Arg, ArgMatches};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = create_cli_app().get_matches();
    
    match matches.subcommand() {
        Some(("build", sub_matches)) => run_build_command(sub_matches),
        Some(("test", sub_matches)) => run_test_command(sub_matches),
        _ => {
            println!("Use --help to see available commands");
            Ok(())
        }
    }
}

fn create_cli_app() -> Command {
    Command::new("clap-fix-demo")
        .version("1.0.0")
        .about("Demo showing fixed clap API usage")
        .subcommand(
            Command::new("build")
                .about("Build with various options")
                .arg(
                    Arg::new("source")
                        .help("Source files to compile")
                        .num_args(1..)
                        .required(true)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("DIR")
                        .help("Output directory")
                )
                .arg(
                    Arg::new("release")
                        .long("release")
                        .help("Build in release mode")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Enable verbose output")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("test")
                .about("Run tests")
                .arg(
                    Arg::new("test-name")
                        .help("Test name pattern")
                        .default_value(".*")
                )
                .arg(
                    Arg::new("threads")
                        .short('j')
                        .long("threads")
                        .value_name("N")
                        .help("Number of test threads")
                )
        )
}

fn run_build_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Running Build Command");
    
    // Fixed: Use get_many instead of values_of
    let source_files: Vec<PathBuf> = matches.get_many::<String>("source")
        .unwrap()
        .map(|s| PathBuf::from(s))
        .collect();
    
    // Fixed: Use get_one instead of value_of
    let output_dir = matches.get_one::<String>("output").map(PathBuf::from);
    
    // Fixed: Use get_flag instead of is_present
    let release = matches.get_flag("release");
    let verbose = matches.get_flag("verbose");
    
    println!("📁 Source files: {} files", source_files.len());
    for file in &source_files {
        println!("  - {}", file.display());
    }
    
    if let Some(output) = &output_dir {
        println!("📂 Output directory: {}", output.display());
    }
    
    println!("🔧 Build mode: {}", if release { "Release" } else { "Debug" });
    println!("📢 Verbose: {}", verbose);
    
    println!("✅ Build completed successfully!");
    Ok(())
}

fn run_test_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Running Test Command");
    
    // Fixed: Use get_one instead of value_of
    let test_name = matches.get_one::<String>("test-name").unwrap();
    let threads = matches.get_one::<String>("threads");
    
    println!("🔍 Test pattern: {}", test_name);
    
    if let Some(thread_count) = threads {
        println!("🔀 Test threads: {}", thread_count);
    }
    
    println!("✅ Tests completed successfully!");
    Ok(())
}
