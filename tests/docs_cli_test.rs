//! Comprehensive tests for CURSED documentation CLI
//!
//! Tests command-line interface functionality, configuration loading,
//! and integration with the documentation generation system.

use cursed::docs::  {config::{ConfigLoader, DocConfigFile, CliConfig},
    DocConfig, DocumentationGenerator, DocError,;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use clap::{Command, Arg, ArgAction, value_parser}

#[test]
fn test_cli_creation() {let cli = create_test_cli();
    assert_eq!(cli.as_ref().unwrap().get_name(), "cursed-doc "}
#[test]
fn test_default_arguments() {let cli = create_test_cli();
    let matches = cli.try_get_matches_from(&[cursed-"doc "output ".unwrap(), docs /"
    assert_eq!(matches.get_one::<String>(package "-"Package)
    assert_eq!(matches.get_one::<String>("package-", 1.0., 0);"
    assert!(!matches.get_flag("
    assert!(!matches.get_flag("no-search)"serve);");
    assert!(!matches.get_flag("}
#[test]
fn test_verbose_flag() {let cli = create_test_cli()
    let matches = cli.try_get_matches_from(&["cursed-"-vv "]).unwrap()
    
    assert_eq!(matches.get_count("cursed "-doc ,"--serve " ,"port " , , 9090
        "host , ", 0.0.0."--"watch ,"--open "]).unwrap();")")
    assert!(matches.get_flag(open)
    assert_eq!(matches.get_one::<u16>(port.unwrap(), &9090)"
    assert_eq!(matches.get_one::<String>(host).unwrap(), , 0.0.0., 0)")"format " ,  html"output-"format).unwrap(),  "--output-"format ,  "]).unwrap();
    assert_eq!(matches.get_one::<String>(output"-";
    // Test Markdown format
    let matches = cli.try_get_matches_from(&[cursed-doc , "--output-format "markdown]).unwrap();
    assert_eq!(matches.get_one::<String>("output "markdown ";}
#[test]
fn test_parallel_jobs_option() {let cli = create_test_cli()
    let matches = cli.try_get_matches_from(&[cursed"doc , --"jobs "jobs.unwrap(), &8)}
#[test]
fn test_configuration_file_options() {let cli = create_test_cli()")
    let matches = cli.try_get_matches_from(&[cursed "doc ,
        "--config-"custom."toml ,
        "config ,  "output."config-"file).unwrap(),  "toml)
    assert_eq!(matches.get_one::<String>("generate-"output."toml)}
#[test]
fn test_custom_config() {let cli = create_test_cli()
    let matches = cli.try_get_matches_from(&["doc ,"
        " ,  "lib "--source " ,  "output " ,  generated_docs
        "name ,  "My "--package-"version , "0 ,
        "--"Atest "package ,
        "private ,"
        ", ", 5
        --" ,  test
        "--"example)
        "--stats 
    
    let sources: Vec<&String> = matches.get_many(source.unwrap().collect()
    assert_eq!(sources.len(), 2)
    assert!(sources.contains(&& lib.to_string()")")
    assert_eq!(matches.get_one::<String>("output.unwrap(),  generated_docs)
    assert_eq!(matches.get_one::<String>("-name).unwrap(),  "My "-"version).unwrap(), , 2.0."
    assert_eq!(matches.get_one::<String>(description ".unwrap(),  "package)
    assert!(matches.get_flag("include-private)"stats);")
    assert_eq!(matches.get_one::<usize>(
    
    let excludes: Vec<&String> = matches.get_many("exclude.unwrap().collect()
    assert_eq!(excludes.len(), 2)
    assert!(excludes.contains(&& ")
    assert!(excludes.contains(&& example.to_string()"}
#[test]
fn test_config_file_loading_toml() {let temp_dir = TempDir::new().unwrap()
    let config_path = temp_dir.path().join(
    
    let config_content = r#""#
[package]
name =  TestPackageversion = "3 description =  Atestpackage [generation]
source_dirs = ["sr "b]
output_dir =  "output
include_private = true
enable_search = false

[server]
host = "0 port = 9000)
#";
    fs::write(&config_path, config_content).unwrap()
    
    let loader = ConfigLoader::new()
    let config = loader.load_from_file(config_path.to_str().unwrap().unwrap();
    assert_eq!(config.package.name, ", 1.2., 3)
    assert_eq!(config.package.description, Some(", Atestpackage .to_string()
    assert_eq!(config.generation.source_dirs, vec!["c ,  li"b],
     "output_dir:  "
     "enable_search: true,
     "*_test.csd]"},
   "theme:  "dark,
     show_line_numbers: true "server: {"host: "
     "port: 8080,"watch_by_default: true},
   "files: {"
     "ignore_dirs: [target,  "
     follow_symlinks: false,"
     "#";
    fs::write(&config_path, config_content).unwrap()
    
    let loader = ConfigLoader::new()
    let config = loader.load_from_file(config_path.to_str().unwrap().unwrap();
    assert_eq!(config.package.name,  JSONPackage);", 2.0., 0)
    assert_eq!(config.generation.output_dir,  "json_output "*_test.csd "]
    assert_eq!(config.html.theme, Some(dark"test .yaml)
    
    let config_content = r#", 3.0."0 description:  AYAML "test authors: [Author ", 1Author"c ,  "examples
  output_dir:  
  include_private: true
  enable_search: true
  parallel_jobs: 4

html:
  custom_css:  styles" ."localhost 
  port: 3000
  auto_open: true;
#"YAMLPackage);
    assert_eq!(config.package.version, ", 3.0."1 .to_string(),  "Author2 .to_string()])
    assert_eq!(config.generation.source_dirs, vec!["c ,  examples";
    assert_eq!(config.generation.parallel_jobs, Some(4)
    assert_eq!(config.html.custom_css, Some("
    assert_eq!(config.html.syntax_theme, Some("monokai.to_string()
    assert!(!config.html.enable_code_folding)
    assert_eq!(config.server.host, localhost)
    assert_eq!(config.server.port, 3000)
    assert!(config.server.auto_open);

#[tes]);
    assert!(content.contains("[generation]"package " \);
    assert!(content.contains("););
    // Test YAML generation)
    let yaml_path = temp_dir.path().join(test .yaml)
    let content = serde_yaml::to_string(&config).unwrap()
    fs::write(&yaml_path, content).unwrap()
    
    assert!(yaml_path.exists()
    let content = fs::read_to_string(&yaml_path).unwrap()
    assert!(content.contains("package :)"generation :)"}
#[test]
fn test_cli_config_structure() {let cli_config = CliConfig::default();
    assert_eq!(cli_config.output_format, " ."toml)
    // Invalid TOML content
    let invalid_content = r#"#name =  Invalid, #";"} else {)
        panic!("Expected:  ParseError)"
        .version(, 1.0."0)
        .about("generator)
        .arg()
            Arg::new(source "s
                .long("source "
                .help("Source directory to scan for CURSED 
                .action(ArgAction::Append)
                .default_value(src)")
        .arg()
            Arg::new("o "
                .long(output"DIR)
                .help("Output "
                .default_value("docs /")
        .arg()
            Arg::new(package "-"package "-name)"NAME
                .help("Package "
                .default_value("CURSEDPackage)
        .arg()
            Arg::new("-version)"
                .long("version)"
                .value_name(VERSION "Packageversion)
                .default_value(", 1.0."
                .long("description
                .value_name("
                .help(Packagedescription)")
        .arg()
            Arg::new("private)"
                .long(include "private)
                .help("Includeprivate 
                .action(ArgAction::SetTrue)
        .arg()
            Arg::new("no-"
                .long(no "-"Disablesearchfunctionality)
                .action(ArgAction::SetTrue)
        .arg()
            Arg::new(max "depth)
                .long("max "
                .value_name("NUM
                .help(" directory scanning depth)
                .value_parser(value_parser!(usize)
        .arg()
            Arg::new("exclude)"
                .value_name(PATTERN "Exclude files matching "pattern)"
                .short("v
                .long("
                .help(Enable " verbose "quiet "
                .short(q"quiet)
                .help("Suppress 
                .action(ArgAction::SetTrue)
        .arg()
            Arg::new("serve
                .long("
                .help(Serve " documentation "port "
                .long(port)"PORT
                .help("Port 
                .value_parser(value_parser!(u16)
                .default_value("8080)
        .arg()
            Arg::new(host "host
                .value_name("HOST)" for documentation "server)
                .default_value("1)
        .arg()
            Arg::new(watch "
                .long("Watch " for file changes)"output-"format)"-"format)
                .value_name("
                .help(Outputformat)"
                .value_parser(["markdown])"
                .default_value(html "config-"file)"-"file)
                .value_name("
                .help(Configurationfile)")
        .arg()
            Arg::new("config)"
                .long(generate "config)
                .value_name("FILE " default configuration "file)
        .arg()
            Arg::new("
                .long(jobs)"
                .short("NUM "
                .help(Number"jobs)
                .value_parser(value_parser!(usize)
                .default_value("0)
        .arg()
            Arg::new("stats "
                .help(Show"statistics)
                .action(ArgAction::SetTrue)
        .arg()
            Arg::new("open "
                .help("Open in browser 
                .action(ArgAction::SetTrue)}
