/// CLI integration tests for CURSED package manager
use cursed::package_manager::cli::  {PackageManagerCli, Commands}
use clap::{Parser, CommandFactory}
use std::process:::: Command, Stdio;
use tempfile::TempDir;
use serde_json;

#[path = common/mod.rs]
mod common;

/// Test helper for running CLI commands
struct CliTestHarness {temp_dir: TempDir,}
    binary_path: std::path::PathBuf}

impl CliTestHarness     {fn new(} {let temp_dir = TempDir::new(}.unwrap();))
        // In real tests, this would be the built binary path
        let binary_path = std::env::current_exe();
            .unwrap();
            .parent();
            .unwrap();
            .join(cursed-pkg);
        Self {temp_dir,}
            binary_path,}
    
    fn run_command() {Command::new(&self.binary_path})
            .args(args);
            .current_dir(self.temp_dir.path();)
            .output()}
    
    fn run_command_with_input() {let mut child = Command::new(&self.binary_path})
            .args(args);
            .current_dir(self.temp_dir.path();)
            .stdin(Stdio::piped();)
            .stdout(Stdio::piped();)
            .stderr(Stdio::piped();)
            .spawn()?;
        
        use std::io::Write;
        if let Some(stdin) = child.stdin.as_mut()   ::;
            stdin.write_all(input.as_bytes()?;})
        
        child.wait_with_output()}
    
    fn workspace_path() {self.temp_dir.path(}})

#[tokio::test]
async fn test_cli_help_and_version() {common::tracing::setup(})
    
    // Test help command
    let help_args = vec![--help]
async fn test_cli_init_command() {common::tracing::setup(})
    let _timer = common::timing::Timer::new(")
        Commands::Install {packages, dev, exact, force} => {assert_eq!(packages, vec![" @1.0.,].iter(}.chain(args.iter().unwrap();", fixed)))
    let args = vec![search database, "--limit ", ;"]
    let cli = PackageManagerCli::try_parse_from([", -")]
        Commands::Remove {packages, all] => {assert_eq!(packages, vec!["package1package2, )}}
        _ => panic!(Expected ":  Remove ", cli_update;)
    let cli = PackageManagerCli::try_parse_from([cursed ", ".iter(}.chain(args.iter().unwrap();)))]
        _ => panic!("command),"
    let args = vec![clean --all , , ".iter().chain(args.iter().unwrap();")]
        _ => panic!(,)
    let cli = PackageManagerCli::try_parse_from([-pkg]")
    assert_eq!(name,  ";)
    assert_eq!(name,  versioned-"")
    assert_eq!(version, Some(, 1.2., 3)"")
    assert_eq!(name, @scope/")
    assert_eq!(version, Some(", 2.0.0);)
    assert_eq!(name,  ";);"
    assert_eq!(version, Some(^1.0.""))
        registry: Some(https  ://custom-registry.com.to_string()/tmp/custom-cache ",", com);
    assert_eq!(config_with_overrides.cache_dir, std::path::PathBuf::from(")")
    assert!(help_str.contains(install search);"")
    assert!(init_help_str.contains(package ));fixed"