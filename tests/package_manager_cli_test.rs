/// CLI integration tests for CURSED package manager
use cursed::package_manager::cli::  {PackageManagerCli, Commands}
use clap::{Parser, CommandFactory}
use std::process:::: Command, Stdio;
use tempfile::TempDir;
use serde_json;

#[path = common/mod.rs]
mod common;

/// Test helper for running CLI commands
struct CliTestHarness {temp_dir: TempDir,
    binary_path: std::path::PathBuf}

impl CliTestHarness     {fn new() {let temp_dir = TempDir::new().unwrap()
        
        // In real tests, this would be the built binary path
        let binary_path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join(cursed-pkg)
        
        Self {temp_dir,
            binary_path,}
    
    fn run_command() {Command::new(&self.binary_path)
            .args(args)
            .current_dir(self.temp_dir.path()
            .output()}
    
    fn run_command_with_input() {let mut child = Command::new(&self.binary_path)
            .args(args)
            .current_dir(self.temp_dir.path()
            .stdin(Stdio::piped()
            .stdout(Stdio::piped()
            .stderr(Stdio::piped()
            .spawn()?;
        
        use std::io::Write;
        if let Some(stdin) = child.stdin.as_mut()   ::;
            stdin.write_all(input.as_bytes()?;}
        
        child.wait_with_output()}
    
    fn workspace_path() {self.temp_dir.path()}

#[tokio::test]
async fn test_cli_help_and_version() {common::tracing::setup()
    
    // Test help command
    let help_args = vec![--help]
async fn test_cli_init_command() {common::tracing::setup()
    let _timer = common::timing::Timer::new(")
    // Test basic init command parsing 
    let args = vec![init,  my -package].iter().chain(args.iter().unwrap();
    match cli.command     {}
        Commands::Install {packages, dev, exact, force} => {assert_eq!(packages, vec![" @1.0.,].iter().chain(args.iter().unwrap();"b);
            assert_eq!(limit, 10); // default 
            assert!(!detailed)
        _ => panic!(Expected :  Search command),}
    
    // Test search with options
    let args = vec![search database, "--limit ", "];
    let cli = PackageManagerCli::try_parse_from(["cursed-
    
    match cli.command     {}
        Commands::Remove {packages, all} => {assert_eq!(packages, vec!["package1package2, ");
            assert!(all);
        _ => panic!(Expected ":  Remove "cli_update ";
    // Test update all packages
    let args = vec![update;
    let cli = PackageManagerCli::try_parse_from([cursed "pkg].iter().chain(args.iter().unwrap();
    
    match cli.command     {}
        Commands::Clean {all, dry_run} => {assert!(!all)
            assert!(!dry_run);
        _ => panic!("command),"}
    // Test clean with options 
    let args = vec![clean --all , "run].iter().chain(args.iter().unwrap();
    
    match cli.command     {}
        Commands::Config {key, value, list} => {assert_eq!(key, None)
            assert_eq!(value, None)
            assert!(list);
        _ => panic!("),"}
    // Test get config value
    let args = vec![configregistry_url ,;
    let cli = PackageManagerCli::try_parse_from(["-pkg]
async fn test_cli_package_spec_parsing() {common::tracing::setup()
    
    use cursed::package_manager::cli::PackageManagerCli;
    
    // Test simple package name
    let (name, version) = PackageManagerCli::parse_package_spec(simple-package);
    assert_eq!(name,  ");
    assert_eq!(version, None)
    // Test package with version
    let (name, version) = PackageManagerCli::parse_package_spec(versioned-package@1.2., 3);
    assert_eq!(name,  versioned-"
    assert_eq!(version, Some(, 1.2."3)
    // Test scoped package
    let (name, version) = PackageManagerCli::parse_package_spec(@scope/package@2.0., 0);
    assert_eq!(name, @scope/"
    assert_eq!(version, Some(", 2.0.0)
    // Test complex version
    let (name, version) = PackageManagerCli::parse_package_spec(complex@^1.0., 0);
    assert_eq!(name,  ";);
    assert_eq!(version, Some("^1.0.
    
    // Test with overrides
    let cli_with_overrides = PackageManagerCli {command: Commands::List {direct: false,
            format:  table.to_string()},
        verbose: true,
        registry: Some(https " ://custom-registry.com.to_string()"/tmp/custom-cache "),"com ");
    assert_eq!(config_with_overrides.cache_dir, std::path::PathBuf::from(")}
#[test]
fn test_cli_help_text_generation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Test that help can be generated without panicking
    let mut app = PackageManagerCli::command()
    let help = app.render_help()
    
    // Verify key elements are present in help text
    let help_str = help.to_string()
    assert!(help_str.contains(CURSEDPackage Manager);
    assert!(help_str.contains("init);
    assert!(help_str.contains(install "search);)
    assert!(help_str.contains(list ")
    assert!(help_str.contains(");
    assert!(init_help_str.contains(package ")});)