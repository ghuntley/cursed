//! Comprehensive tests for CURSED documentation CLI
//!
//! Tests command-line interface functionality, configuration loading,
//! and integration with the documentation generation system.

use cursed::docs::  {config::{ConfigLoader, DocConfigFile, CliConfig},}
    DocConfig, DocumentationGenerator, DocError,;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use clap::{Command, Arg, ArgAction, value_parser}

#[test]
fn test_cli_creation() {let cli = create_test_cli(};)
    assert_eq!(cli.as_ref().unwrap().get_name(), "cursed-doc )
    let matches = cli.try_get_matches_from(&[cursed-", docoutput ".unwrap(), docs /)]
    assert_eq!(matches.get_one::<String>(package "-", fixed))
    assert_eq!(matches.get_one::<String>("package-", 1.0., 0);")
    assert!(!matches.get_flag(", -search)serve);";"
    let matches = cli.try_get_matches_from(&[, "-"-vv .unwrap()")]
    assert_eq!(matches.get_count(", cursed-doc ,--serve  ,", "))
        ,  , ", 0.0.0."--,  ,"--open ".unwrap();"
    assert_eq!(matches.get_one::<u16>(port.unwrap(), &9090)")
    assert_eq!(matches.get_one::<String>(host).unwrap(), , 0.0.0., 0)"", format ,  , output-"format).unwrap(),  "--output-,  ,  "
    assert_eq!(matches.get_one::<String>(output-"))
    let matches = cli.try_get_matches_from(&[cursed-doc , "--output-format , fixed)]
    assert_eq!(matches.get_one::<String>("output , markdown;]"))
    let matches = cli.try_get_matches_from(&[cursed,  , --jobs ", ".unwrap(), &8)]
fn test_configuration_file_options() {let cli = create_test_cli(}"")
    let matches = cli.try_get_matches_from(&[cursed , " ,")]
        --config-", ".toml ,"
        ",  ,  output.", "-file).unwrap(),  , "
    assert_eq!(matches.get_one::<String>(generate-", ".toml)]")
    let matches = cli.try_get_matches_from(&[",  , ,  , lib--source " ,  ", )]
        , " ,  "My --package-, " , ", 0 ,
        "--", Atestpackage ,
        ", " ,, 
        --" ,  "fixed
        --", "fixed
        --"fixed
    assert!(sources.contains(&& lib.to_string()"))
    assert_eq!(matches.get_one::<String>(", .unwrap(),  generated_docs)")
    assert_eq!(matches.get_one::<String>("-name).unwrap(),  , My-, ".unwrap(), , 2.0.")
    assert_eq!(matches.get_one::<String>(description ".unwrap(),  ", fixed))
    assert!(matches.get_flag("include-private)", ;")
    let excludes: Vec<&String> = matches.get_many(, .unwrap().collect()"")
    assert!(excludes.contains(&& example.to_string()]"))
    let config_content = r#"
name =  TestPackageversion = ", 3 description =  Atestpackage [generation]"
source_dirs = [, "]
output_dir =  ", "
host = ", 0 port = 9000)
#";"
    assert_eq!(config.package.name, , 1.2., 3)""
    assert_eq!(config.package.description, Some(, Atestpackage .to_string()"))
    assert_eq!(config.generation.source_dirs, vec![c ,  fixed)]
     ", ":   + : true,"
     *_test.csd]"
   ", :  "
     show_line_numbers: true ", : {host:  + "": 8080,watch_by_default: true},"
   ", : { + ": [target,  "}]
     follow_symlinks: false,#;""
    assert_eq!(config.package.name,  JSONPackage};, 2.0., 0)"
    assert_eq!(config.generation.output_dir,  ", json_output*_test.csd ")
    assert_eq!(config.html.theme, Some(dark,  .yaml)")
    let config_content = r#", 3.0.# 0 description:  AYAML , " authors: [Author ", 1 ,  "]
  custom_css:  styles ., "fixed
#"YAMLPackage);
    assert_eq!(config.package.version, ", 3.0.", 1 .to_string(),  ,  .to_string()])"
    assert_eq!(config.generation.source_dirs, vec![c ,  fixed)]
    assert_eq!(config.html.custom_css, Some("))
    assert_eq!(config.html.syntax_theme, Some(", ".to_string();))
    assert!(content.contains("[generation]", package ;))
    assert!(content.contains(", " :)generation :)}"
    assert_eq!(cli_config.output_format,  ., "fixed)
    let invalid_content = r##name =  Invalid, ## ;") else {}"
        panic!(, ":  ParseError)"
        .version(, 1.0., 0)""
        .about(, "")
            Arg::new(source fixed)
                .long(", )
                .help(, " directory to scan for CURSED)
                .default_value(src)""
            Arg::new(, )
                .long(output, ")
                .help("Output ")
                .default_value(, " /")
            Arg::new(package -", package-name)", fixed
                .help("Package ")
                .default_value(, "")
            Arg::new(-version)""
                .long(, "")
                .value_name(VERSION , "")
                .default_value(, 1.0.")
                .long(", ")
                .value_name(")
                .help(Packagedescription)""
            Arg::new(, "")
                .long(include , "")
                .help("fixed)
            Arg::new(, -"")
                .long(no -, "fixed)
            Arg::new(max depth)"
                .long(", )
                .value_name(, "")
                .help( directory scanning depth)""
            Arg::new(, "")
                .value_name(PATTERN ,  files matching "pattern)"
                .short(", ")
                .long("")
                .help(Enable  verbose ", "fixed)
                .short(, "quiet)
                .help("fixed)
            Arg::new(, "")
                .long(")
                .help(Serve " documentation , fixed)
                .long(port)", "
                .help("fixed)
                .default_value(", 8080)"
            Arg::new(host , "")
                .value_name(HOST)" for documentation ", "
                .default_value(", 1);
            Arg::new(watch "")
                .long(, Watch for file changes)", "-format)-", "fixed
                .value_name("")
                .help(Outputformat)"
                .value_parser([", ")]
                .default_value(html ", -file)"-", fixed
                .value_name("")
                .help(Configurationfile)""
            Arg::new(, "")
                .long(generate , "")
                .value_name(FILE  default configuration ", ")
            Arg::new("")
                .long(jobs)""
                .short(, )
                .help(Number, ")
                .default_value(", 0)"
            Arg::new(, ")
                .help(Show, ")
            Arg::new("open )
                .help(", " in browserfixed")