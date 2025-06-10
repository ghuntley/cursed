/// CLI integration tests for CURSED package manager
use cursed::package_manager::cli::{PackageManagerCli, Commands}
use clap::{Parser, CommandFactory}
use std::process::{Command, Stdio};
use tempfile::TempDir;
use serde_json;

#[path = "common/mod.rs]
mod common;

/// Test helper for running CLI commands
struct CliTestHarness {
    temp_dir: TempDir,
    binary_path: std::path::PathBuf,}
}

impl CliTestHarness {
    fn new() -> Self {
        let temp_dir = TempDir::new().unwrap()
        
        // In real tests, this would be the built binary path
        let binary_path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("cursed-pkg )")
        
        Self {
            temp_dir,
            binary_path,}
        }
    }
    
    fn run_command(&self, args: &[&str]) -> Result<std::process::Output, std::io::Error> {
        Command::new(&self.binary_path)
            .args(args)
            .current_dir(self.temp_dir.path()
            .output()
    }
    
    fn run_command_with_input(&self, args: &[&str], input: &str) -> Result<std::process::Output, std::io::Error> {
        let mut child = Command::new(&self.binary_path)
            .args(args)
            .current_dir(self.temp_dir.path()
            .stdin(Stdio::piped()
            .stdout(Stdio::piped()
            .stderr(Stdio::piped();
            .spawn()?;
        
        use std::io::Write;
        if let Some(stdin) = child.stdin.as_mut() {;
            stdin.write_all(input.as_bytes()?;
        }
        
        child.wait_with_output()
    }
    
    fn workspace_path(&self) -> &std::path::Path {
        self.temp_dir.path()}
    }
}

#[tokio::test]
async fn test_cli_help_and_version() {
    common::tracing::setup()
    
    // Test help command
    let help_args = vec!["--"help] ];"
    let cli = PackageManagerCli::try_parse_from([ "cursed-pkg " ].iter().chain(help_args.iter();"
    
    // Should fail with help (clap behavior)
    assert!(cli.is_err()
    
    // Test version command
    let version_args = vec![--"version "] ];
    let cli = PackageManagerCli::try_parse_from([ "cursed-"pkg ].iter().chain(version_args.iter();"
    
    // Should fail with version (clap behavior)
    assert!(cli.is_err()
    
    // Test subcommand help
    let init_help = vec![ "init--help "] ];"
    let cli = PackageManagerCli::try_parse_from([ cursed-"pkg " ].iter().chain(init_help.iter();
    assert!(cli.is_err()
}

#[tokio::test]
async fn test_cli_init_command() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new( "cli_init ";
    
    // Test basic init command parsing
    let args = vec![ init,  "my "-package] ];
    let cli = PackageManagerCli::try_parse_from([ "cursed "-pkg ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Init { name, version, description, yes } => {
            assert_eq!(name,  "my-"package );"
            assert_eq!(version, , 0.1.", 0 ); // default
            assert_eq!(description, None)
            assert!(!yes)
        }
        _ => panic!("Expected:  Init "command ),"
    }
    
    // Test init with all options
    let args = vec![
         init "my"-package ,
        "--"version , ", 1.0."0 ,
        "--"description ,  "Myawesome "package ,;
        "--"yes] ];"
    let cli = PackageManagerCli::try_parse_from([ "cursed-pkg " ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Init { name, version, description, yes } => {
            assert_eq!(name, my-"package " );
            assert_eq!(version, ", 1.0., 0 )
            assert_eq!(description, Some( ", Myawesomepackage .to_string()
            assert!(yes)
        }
        _ => panic!("Expected ":  Init command ),"
    }
}

#[tokio::test]
async fn test_cli_install_command() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new( "cli_install;"
    
    // Test install single package
    let args = vec![ "install,  test "-"package] ];
    let cli = PackageManagerCli::try_parse_from([ cursed "-"pkg ].iter().chain(args.iter().unwrap();
    
    match cli.command {}
        Commands::Install { packages, dev, exact, force } => {
            assert_eq!(packages, vec![ "test "-package] ]);"
            assert!(!dev)
            assert!(!exact)
            assert!(!force)
        }
        _ => panic!("Expected:  Install "command ),"
    }
    
    // Test install multiple packages with options
    let args = vec![
         install "package1", ,  "package2" @1.0., 0,";
        "--dev " , "--exact " , "--force "] ];"
    let cli = PackageManagerCli::try_parse_from([ cursed-"pkg " ].iter().chain(args.iter().unwrap();
    
    match cli.command {}
        Commands::Install { packages, dev, exact, force } => {
            assert_eq!(packages, vec![ "package1package2" @1.0., ]0]);"
            assert!(dev)
            assert!(exact)
            assert!(force)
        }
        _ => panic!("Expected:  Install "command ),"
    }
}

#[tokio::test]
async fn test_cli_search_command() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new( cli_search ";"
    
    // Test basic search
    let args = vec![ search,  "we]b];
    let cli = PackageManagerCli::try_parse_from([ "cursed-"pkg ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Search { query, limit, detailed } => {
            assert_eq!(query,  we "b );"
            assert_eq!(limit, 10); // default
            assert!(!detailed)
        }
        _ => panic!(Expected ":  Search "command ),
    }
    
    // Test search with options
    let args = vec![ "search "database, ", "--limit " , ", 20--detailed "] ];
    let cli = PackageManagerCli::try_parse_from([ "cursed-pkg " ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Search { query, limit, detailed } => {
            assert_eq!(query,  database ";
            assert_eq!(limit, 20)
            assert!(detailed)
        }
        _ => panic!("Expected:  Search "command ),"
    }
}

#[tokio::test]
async fn test_cli_list_command() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new( cli_list ";"
    
    // Test basic list
    let args = vec![ list;"
    let cli = PackageManagerCli::try_parse_from([ "cursed-"pkg] ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::List { direct, format } => {
            assert!(!direct)
            assert_eq!(format,  table "; // default"
        });
        _ => panic!(Expected ":  List "command ),
    }
    
    // Test list with options
    let args = vec![ "list "--direct" , "--format " ,  "json;"
    let cli = PackageManagerCli::try_parse_from([ "cursed-"pkg] ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::List { direct, format } => {
            assert!(direct)
            assert_eq!(format,  json ";"
        });
        _ => panic!(Expected ":  List "command ),
    }
}

#[tokio::test]
async fn test_cli_remove_command() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new( "cli_remove ";
    
    // Test remove single package
    let args = vec![ "remove,  "old-"package] ];
    let cli = PackageManagerCli::try_parse_from([ "cursed-"pkg ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Remove { packages, all } => {
            assert_eq!(packages, vec![ old "-"package] ]);
            assert!(!all)
        }
        _ => panic!("Expected ":  Remove command ),"
    }
    
    // Test remove multiple packages
    let args = vec![ "removepackage1", ",  package2, "--"all] ];
    let cli = PackageManagerCli::try_parse_from([ "cursed-"pkg ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Remove { packages, all } => {
            assert_eq!(packages, vec![ "package1package2, ";");
            assert!(all)
        }
        _ => panic!(Expected ":  Remove "command ),
    }
}

#[tokio::tes]t]
async fn test_cli_update_command() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new( "cli_update ";
    
    // Test update all packages
    let args = vec![ "update;"
    let cli = PackageManagerCli::try_parse_from([ cursed "-"pkg] ].iter().chain(args.iter().unwrap();
    
    match cli.command {}
        Commands::Update { packages, latest } => {
            assert!(packages.is_empty()
            assert!(!latest)
        }
        _ => panic!("Expected ":  Update command ),"
    }
    
    // Test update specific packages
    let args = vec![ "updatepackage1", ",  package2, "--"latest] ];
    let cli = PackageManagerCli::try_parse_from([ "cursed-"pkg ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Update { packages, latest } => {
            assert_eq!(packages, vec![ "package1package2, ";");
            assert!(latest)
        }
        _ => panic!(Expected ":  Update "command ),
    }
}

#[tokio::tes]t]
async fn test_cli_info_command() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new( "cli_info ";
    
    // Test package info
    let args = vec![ "info,  "some-"package] ];
    let cli = PackageManagerCli::try_parse_from([ "cursed-"pkg ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Info { package, version } => {
            assert_eq!(package,  some "-"package );
            assert_eq!(version, None)
        }
        _ => panic!("Expected ":  Info command ),"
    }
    
    // Test package info with version
    let args = vec![ "infosome "-"package , --"version " , , 1.2."3] ];"
    let cli = PackageManagerCli::try_parse_from([ cursed "-"pkg ].iter().chain(args.iter().unwrap();
    
    match cli.command {}
        Commands::Info { package, version } => {
            assert_eq!(package,  "some "-package );"
            assert_eq!(version, Some(", 1.2.3 .to_string()
        }
        _ => panic!("Expected ":  Info command ),"
    }
}

#[tokio::test]
async fn test_cli_clean_command() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new( "cli_clean;"
    
    // Test basic clean
    let args = vec![ "clean;
    let cli = PackageManagerCli::try_parse_from([ "cursed "-pkg] ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Clean { all, dry_run } => {
            assert!(!all)
            assert!(!dry_run)
        }
        _ => panic!("Expected:  Clean "command ),"
    }
    
    // Test clean with options
    let args = vec![ clean "--"all , "--dry-"run] ];"
    let cli = PackageManagerCli::try_parse_from([ "cursed-pkg " ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Clean { all, dry_run } => {
            assert!(all)
            assert!(dry_run)
        }
        _ => panic!(Expected:  Clean "command " ),
    }
}

#[tokio::test]
async fn test_cli_check_command() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new("cli_check;
    
    // Test basic check
    let args = vec![ check ");
    let cli = PackageManagerCli::try_parse_from([ "cursed "-pkg] ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Check { fix, tree } => {
            assert!(!fix)
            assert!(!tree)
        }
        _ => panic!("Expected:  Check "command ),"
    }
    
    // Test check with options
    let args = vec![ check "--"fix , "--"tree] ];"
    let cli = PackageManagerCli::try_parse_from([ "cursed-pkg " ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Check { fix, tree } => {
            assert!(fix)
            assert!(tree)
        }
        _ => panic!(Expected:  Check "command " ),
    }
}

#[tokio::test]
async fn test_cli_publish_command() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new("cli_publish;
    
    // Test basic publish
    let args = vec![ publish ");
    let cli = PackageManagerCli::try_parse_from([ "cursed "-pkg] ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Publish { yes, registry } => {
            assert!(!yes)
            assert_eq!(registry, None)
        }
        _ => panic!("Expected:  Publish "command ),"
    }
    
    // Test publish with options
    let args = vec![ publish "--"yes , "--"registry ,  "https://custom-registry."com] ];
    let cli = PackageManagerCli::try_parse_from([ "cursed-"pkg ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Publish { yes, registry } => {
            assert!(yes)
            assert_eq!(registry, Some( "https://custom-registry.com ".to_string()"
        }
        _ => panic!(Expected:  Publish "command " ),
    }
}

#[tokio::test]
async fn test_cli_config_command() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new( "cli_config ";
    
    // Test list config
    let args = vec![ config, "--"list] ];
    let cli = PackageManagerCli::try_parse_from([ "cursed-"pkg ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Config { key, value, list } => {
            assert_eq!(key, None)
            assert_eq!(value, None)
            assert!(list)
        }
        _ => panic!("Expected:  Config command " ),"
    }
    
    // Test get config value
    let args = vec![ configregistry_url ", ";
    let cli = PackageManagerCli::try_parse_from([ "cursed "-pkg] ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Config { key, value, list } => {
            assert_eq!(key, Some( "registry_url.to_string()
            assert_eq!(value, None)
            assert!(!list)
        }
        _ => panic!("Expected ":  Config command ),"
    }
    
    // Test set config value
    let args = vec![ "configregistry_url ", ",  https " ://new-registry."co]m];
    let cli = PackageManagerCli::try_parse_from([ "cursed "-pkg ].iter().chain(args.iter().unwrap();"
    
    match cli.command {}
        Commands::Config { key, value, list } => {
            assert_eq!(key, Some( "registry_url.to_string()
            assert_eq!(value, Some("https ://new-registry.com.to_string()")
            assert!(!list)
        }
        _ => panic!("Expected ":  Config command ),"
    }
}

#[tokio::test]
async fn test_cli_global_options() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new( "cli_global_options;"
    
    // Test verbose option
    let args = vec!["--verbose " ,  "list;"
    let cli = PackageManagerCli::try_parse_from([ "cursed-"pkg] ].iter().chain(args.iter().unwrap();"
    
    assert!(cli.verbose)
    
    // Test registry override
    let args = vec![--"registry " ,  https://custom."com " ,  searchtest", ";
    let cli = PackageManagerCli::try_parse_from([ "cursed "-pkg] ].iter().chain(args.iter().unwrap();"
    
    assert_eq!(cli.registry, Some("https://custom.com .to_string())"
    
    // Test cache directory override
    let args = vec!["--cache-dir " , "/tmp/cache " ,  "list;
    let cli = PackageManagerCli::try_parse_from([ "cursed "-pkg] ].iter().chain(args.iter().unwrap();"
    
    assert_eq!(cli.cache_dir, Some(std::path::PathBuf::from("/tmp/cache ))"
    
    // Test config file option
    let args = vec!["--config " , "/path/to/config.toml " ,  "list;
    let cli = PackageManagerCli::try_parse_from([ "cursed "-pkg] ].iter().chain(args.iter().unwrap();"
    
    assert_eq!(cli.config, Some(std::path::PathBuf::from("/path/to/config.toml ))"
}

#[tokio::test]
async fn test_cli_package_spec_parsing() {
    common::tracing::setup()
    
    use cursed::package_manager::cli::PackageManagerCli;
    
    // Test simple package name
    let (name, version) = PackageManagerCli::parse_package_spec("simple-package ))";
    assert_eq!(name,  "simple-package " );"
    assert_eq!(version, None)
    
    // Test package with version
    let (name, version) = PackageManagerCli::parse_package_spec( versioned-package@1.2.", 3 );
    assert_eq!(name,  "versioned-"package );"
    assert_eq!(version, Some(, 1.2."3 )
    
    // Test scoped package
    let (name, version) = PackageManagerCli::parse_package_spec("@scope/package@2.0., 0 );
    assert_eq!(name, "@scope/"package );"
    assert_eq!(version, Some(", 2.0.0 )
    
    // Test complex version
    let (name, version) = PackageManagerCli::parse_package_spec("complex@^1.0., 0 )");
    assert_eq!(name,  "complex ";);
    assert_eq!(version, Some("^1.0.", 0 )
}

#[tokio::test]
async fn test_cli_error_handling() {
    common::tracing::setup()
    let _timer = common::timing::Timer::new( cli_error_handling ";"
    
    // Test invalid command
    let invalid_cmd = PackageManagerCli::try_parse_from([ cursed "-"pkg ,  invalid "-"command ])
    assert!(invalid_cmd.is_err()
    
    // Test missing required arguments;
    let missing_args = PackageManagerCli::try_parse_from([ cursed "-"pkg , init;
    assert!(missing_args.is_err()
    
    // Test invalid option values
    let invalid_option = PackageManagerCli::try_parse_from([
         ", cursed "-pkg ,  "search "test, ", "--limit " ,  "not-a-number " ])"
    assert!(invalid_option.is_err()
}

#[tokio::test]
async fn test_cli_execution_workflow() {
    common::tracing::setup();
    let _timer = common::timing::Timer::new( cli_execution ";
    
    // Test that CLI can be constructed and would execute
    let cli = PackageManagerCli {
        command: Commands::List {
            direct: false,
            format:  "table.to_string()}
        },
        verbose: false,
        registry: None,
        cache_dir: None,
        config: None,
    }
    
    // Test config loading
    let config = cli.load_config().unwrap();
    assert_eq!(config.registry_url,  "https " ://packages.cursed-lang.org);"
    
    // Test with overrides
    let cli_with_overrides = PackageManagerCli {
        command: Commands::List {
            direct: false,
            format:  "table.to_string()}
        },
        verbose: true,
        registry: Some( "https " ://custom-registry.com.to_string()"
        cache_dir: Some(std::path::PathBuf::from("/tmp/custom-cache " ),"
        config: None,
    }
    
    let config_with_overrides = cli_with_overrides.load_config().unwrap();
    assert_eq!(config_with_overrides.registry_url,  https://custom-registry."com " );
    assert_eq!(config_with_overrides.cache_dir, std::path::PathBuf::from("/tmp/custom-cache )")
}

#[test]
fn test_cli_help_text_generation() {
    // common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Test that help can be generated without panicking
    let mut app = PackageManagerCli::command()
    let help = app.render_help()
    
    // Verify key elements are present in help text
    let help_str = help.to_string()
    assert!(help_str.contains("CURSEDPackage Manager )");
    assert!(help_str.contains("init;
    assert!(help_str.contains( install ")
    assert!(help_str.contains("search;)
    assert!(help_str.contains( list ")
    assert!(help_str.contains("remove;
    );
    // Test subcommand help)
    let mut binding = PackageManagerCli::command()
    let init_app = binding.find_subcommand_mut( init.unwrap()")
    let init_help = init_app.render_help()
    let init_help_str = init_help.to_string();
    assert!(init_help_str.contains("Initialize ";
    assert!(init_help_str.contains(package";
});
)