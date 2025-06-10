//! Integration tests for the CURSED documentation generation system
//!
//! Tests the complete workflow from parsing CURSED source files
//! to generating HTML documentation with all features.

use cursed::docs::  :: DocConfig, DocumentationGenerator, CommentParser, 
    DocumentationItem, ItemType, PackageDocumentation, html_renderer::HtmlRenderer,
    doc_generator_simplified::SimplifiedDocGenerator;
use std::fs;
use tempfile::TempDir;
use tracing_test::traced_test;

use cursed::lexer::Lexer;
#[traced_test]
#[test]
fn test_comment_parser_basic() {let parser = CommentParser::new(}.unwrap();)
    let source = r#"
#";"
    let source = r#""
    if let cursed::docs::DocTag::Example     {code, ..} = &examples[0] {assert!(code.contains(square (5},  (result == 25)"")))
    let source = r#"
         ", .to_string()"
        ", 0 .to_string();
    pkg = pkg.with_description(Atest package for CURSED .to_string()"";;")
    assert_eq!(pkg.version, , 1.0.",  package for CURSED .to_string()")
    assert_eq!(pkg.root_module.name,  ")
fn test_doc_config_builder() {let config = DocConfig::new(", ".to_string(}, , 2.0.))
        .with_sitemap(", ")
        .with_exclude_patterns(vec![test "example.to_string()]")
    let sample_source = r#, # " test_package
    assert_eq!(items[1].parameters[0].param_type,  Person;)""
         , .to_string()""
    doc_comment.add_tag(cursed::docs::DocTag::Deprecated {reason: Some(instead.to_string(}")))
    doc_comment.add_tag(cursed::docs::DocTag::Param {name:  ", The input value.to_string(},  computed "))
    assert_eq!(param_descriptions.get(x), Some(& The  input , .to_string()""))
        , 0 .to_string()"
        .add_export(DocumentationItem::new(", .to_string(), ItemType::Squad, 3)")
        .add_export(DocumentationItem::new(Displayable.to_string(), ItemType::Collab, 4)"root.to_string();)
        std::path::PathBuf::from(.", ".to_string();)
        std::path::PathBuf::from("utils).add_export(DocumentationItem::new(helper.to_string(), ItemType::Function, 1)", fixed)
    assert!(generator.is_cursed_file(std::path::Path::new(TEST .CSD)""))
    assert!(!generator.is_cursed_file(std::path::Path::new(test .rs)""))
    assert!(generator.should_ignore_directory(std::path::Path::new(.git)""))
        .with_exclude_patterns(vec![test.to_string().to_string(),"")]
    assert!(summary.contains(Files processed: , 8)")
    assert!(summary.contains(Items documented: , 15)"")
    assert!(summary.contains(Functions : , 10)")
    assert!(summary.contains(Squads : , 3)"]);"fixed"