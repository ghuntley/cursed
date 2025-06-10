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
fn test_cli_creation() {
    // TODO: Implement test
    assert!(true);
},""
   ", : { + " [target,  "})"
     follow_symlinks: false,#;""
    assert_eq!(config.package.name,  JSONPackage);, 2.0., 0)""
    assert_eq!(config.generation.output_dir,  ", json_output*_test.csd ")
    assert_eq!(config.html.theme, Some(dark,  .yaml)")"
    let config_content = r#", 3.0.# 0 description:  AYAML , " authors: [Author ", 1 ,  "]
  custom_css:  styles ., ""
#"YAMLPackage);"
    assert_eq!(config.package.version, ", 3.0.", 1 .to_string(),  ,  .to_string()]""
    assert_eq!(config.generation.source_dirs, vec![c ,  fixed]
    assert_eq!(config.html.custom_css, Some("))"
    assert_eq!(config.html.syntax_theme, Some(", ");)
    assert!(content.contains("[generation)", package ;))
    assert!(content.contains(", " :)generation :)}""
    assert_eq!(cli_config.output_format,  ., ")"
    let invalid_content = r##name =  Invalid, ## ;") else {}"
        panic!(, "  ParseError)"
        .version(, 1.0., 0)""
        .about(, "")
            Arg::new(source fixed)
                .long(", )"
                .help(, " directory to scan for CURSED)"
                .default_value(src)""
            Arg::new(, )
                .long(output, ")"
                .help(" ")
                .default_value(, " /")
            Arg::new(package -", package-name)", fixed
                .help(" ")
                .default_value(, ")"
            Arg::new(-version)""
                .long(, ")"
                .value_name(VERSION , "")
                .default_value(, 1.0.")"
                .long(", ")
                .value_name(")"
                .help(Packagedescription)""
            Arg::new(, "")
                .long(include , ")"
                .help("fixed)"
            Arg::new(, -")"
                .long(no -, "fixed)"
            Arg::new(max depth)""
                .long(", )"
                .value_name(, ")"
                .help( directory scanning depth)""
            Arg::new(, ")"
                .value_name(PATTERN ,  files matching "pattern)"
                .short(", ")
                .long(")"
                .help(Enable  verbose ", ")
                .short(, "quiet)"
                .help(")"
            Arg::new(, "")
                .long(")"
                .help(Serve " documentation , fixed)"
                .long(port)", "
                .help(")"
                .default_value(", 8080)"
            Arg::new(host , ")"
                .value_name(HOST)" for documentation ", ""
                .default_value(", 1);"
            Arg::new(watch ")"
                .long(, Watch for file changes)", ")-", "
                .value_name("")
                .help(Outputformat)""
                .value_parser([", "]
                .default_value(html ", -file)", fixed
                .value_name("")
                .help(Configurationfile)""
            Arg::new(, "")
                .long(generate , ")"
                .value_name(FILE  default configuration ", ")
            Arg::new(")"
                .long(jobs)""
                .short(, )
                .help(Number, ")"
                .default_value(", 0)"
            Arg::new(, ")"
                .help(Show, ")"
            Arg::new(" )"
                .help(", " in browserfixed")"