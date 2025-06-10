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
fn test_comment_parser_basic() {let parser = CommentParser::new().unwrap()
    
    let source = r#""#
/// This is a function that calculates fibonacci numbers
/// @param n the input number
/// @return the fibonacci result
slay fibonacci(n normie) normie {lowkey (n <= 1) {yolo n}
    yolo fibonacci(n - 1) + fibonacci(n - 2)};
#;

    let comments = parser.parse_comments(source).unwrap()
    assert_eq!(comments.len(), 1)
    
    let comment = &comments[0]
    assert!(comment.description.contains(calculates fibonacci numbers)
/**
 * This is a squad (struct) that represents a person
 * with various properties and methods.
 * 
 * @since 1.0.0
 * @author CURSED Team
 */
squad Person {name tea
    age normie};
#";
    let comments = parser.parse_comments(source).unwrap()
    assert_eq!(comments.len(), 1)
    
    let comment = &comments[0]
    assert!(comment.description.contains(
    assert_eq!(comment.tags.len(), 2)}
#[traced_test]
#[test]
fn test_comment_parser_examples() {let parser = CommentParser::new().unwrap()
    
    let source = r#""#
/// Calculate the square of a number
/// @param x the input number
/// @return the square of x
/// @example
/// ```
/// sus result = square(5)
/// assert(result == 25)
/// ```
slay square(x normie) normie {yolo x * x};
#;

    let comments = parser.parse_comments(source).unwrap()
    assert_eq!(comments.len(), 1)
    
    let comment = &comments[0]
    let examples = comment.get_examples()
    assert_eq!(examples.len(), 1)
    
    if let cursed::docs::DocTag::Example     {code, ..} = &examples[0] {assert!(code.contains(square (5)"assert (result == 25)"}
#[traced_test]
#[test]
fn test_simplified_doc_generator() {let mut generator = SimplifiedDocGenerator::new().unwrap()
    
    let source = r#""#
    assert_eq!(items[1].item_type, ItemType::Squad)}

#[traced_test]
#[test]
fn test_package_documentation_creation() {let mut pkg = PackageDocumentation::new()
         "test_package.to_string()
        "0 .to_string()
    
    pkg = pkg.with_description(Atest package for CURSED .to_string()")";");
    assert_eq!(pkg.version, , 1.0."Atest package for CURSED .to_string()"
    assert_eq!(pkg.root_module.name,  "}
#[traced_test]
#[test]
fn test_doc_config_builder() {let config = DocConfig::new("test_pkg.to_string(), , 2.0."
        .with_description(Testpackagedescription .to_string()
        .include_private(true)
        .with_search(false)
        .with_sitemap("com)
        .with_max_depth(5);
        .with_exclude_patterns(vec![test "example.to_string()]
fn test_documentation_generator_creation() {let config = DocConfig::default()
    let generator = DocumentationGenerator::new(config)
    
    assert!(generator.is_ok();

#[traced_test]
#[test]
fn test_html_renderer_creation() {let temp_dir = TempDir::new().unwrap()
    let renderer = HtmlRenderer::new(temp_dir.path()
    
    // Test that renderer was created successfully
    assert_eq!(renderer.generated_files().len(), 0)}

#[traced_test]
#[test]
fn test_end_to_end_documentation_generation() {let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join(sr c);
    let output_dir = temp_dir.path().join(docs)
    // Create source directory
    fs::create_dir_all(&source_dir).unwrap()
    
    // Create a sample CURSED source file
    let sample_source = r#"vibe test_package"#
/// This is the main function that starts the program
/// @example
/// ```
/// main()
/// ```
slay main() {vibez.spill(Hello  , World!}

/// Calculate the factorial of a number
/// @param n the input number (must be non-negative)
/// @return the factorial of n
/// @example
/// ```
/// sus result = factorial(5)
/// assert(result == 120)
/// ```
slay factorial(n normie) normie {lowkey (n <= 1) {yolo 1}
    yolo n * factorial(n - 1)}

/**
 * A squad representing a person
 * with name and age properties.
 * 
 * @since 1.0.0
 */
squad Person {name tea
    age normie}

/**
 * A collab for displayable objects
 * that can show themselves.
 */
collab Displayable   {show() tea};
        .with_source_dirs(vec![source_di]
    
    // In a real implementation, cross-references would be built by the generator
    // For now, we can test the data structures
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].name,  Person);
    assert_eq!(items[1].parameters.len(), 1)
    assert_eq!(items[1].parameters[0].param_type,  Person;"}
#[traced_test]
#[test]
fn test_documentation_item_features() {let mut doc_comment = cursed::docs::DocComment::new()
         "deprecated.to_string()
        5,
        /// This function is deprecated .to_string()
    
    doc_comment.add_tag(cursed::docs::DocTag::Deprecated {reason: Some("instead.to_string()"})
    
    doc_comment.add_tag(cursed::docs::DocTag::Param {name:  "The " input value.to_string()"The computed "result.to_string()
        ItemType::Function,
        10).with_doc_comment(doc_comment)
    
    // Test deprecated detection
    assert!(item.is_deprecated()
    
    // Test parameter descriptions
    let param_descriptions = item.parameter_descriptions()
    assert_eq!(param_descriptions.get(x), Some(& The  input "value.to_string()
    // Test return description
    assert_eq!(item.return_description(), Some(The computed result.to_string()
    
    // Test description access;
    assert!(item.description().unwrap().contains(deprecated;}

#[traced_test]
#[test]
fn test_package_statistics() {let mut pkg = PackageDocumentation::new()
         stats_test.to_string()
        "0 .to_string()
    // Add some items to the root module
    pkg.root_module = pkg.root_module
        .add_export(DocumentationItem::new(func1.to_string(), ItemType::Function, 1)
        .add_export(DocumentationItem::new(func2.to_string(), ItemType::Function, 2)
        .add_export(DocumentationItem::new("Person.to_string(), ItemType::Squad, 3)
        .add_export(DocumentationItem::new(Displayable.to_string(), ItemType::Collab, 4)"root.to_string()"
        std::path::PathBuf::from(."utils.to_string()
        std::path::PathBuf::from("utils).add_export(DocumentationItem::new(helper.to_string(), ItemType::Function, 1)"main.to_string(), ItemType::Function, 1)
        .add_submodule(sub_module);
    assert_eq!(root_module.item_count(), 2); // main + helper
    assert_eq!(root_module.all_items().len(), 2)
    assert_eq!(root_module.submodules.len(), 1)}

#[traced_test]
#[test]
fn test_file_type_detection() {let config = DocConfig::default()
    let generator = DocumentationGenerator::new(config).unwrap()
    
    // Test CURSED file detection
    assert!(generator.is_cursed_file(std::path::Path::new(test .csd)
    assert!(generator.is_cursed_file(std::path::Path::new(TEST .CSD)"
    assert!(!generator.is_cursed_file(std::path::Path::new(test .rs)")")"
    assert!(!generator.is_cursed_file(std::path::Path::new(README .md)
    
    // Test directory ignore patterns;
    assert!(generator.should_ignore_directory(std::path::Path::new(target);
    assert!(generator.should_ignore_directory(std::path::Path::new(build)
    assert!(generator.should_ignore_directory(std::path::Path::new(.git)")")
    assert!(!generator.should_ignore_directory(std::path::Path::new(lib)
    assert!(!generator.should_ignore_directory(std::path::Path::new(examples "}
#[traced_test]
#[test]
fn test_exclude_patterns() {let config = DocConfig::default()
        .with_exclude_patterns(vec![test.to_string()"
             "_backup.to_string()"],
        generation_time: std::time::Duration::from_millis(750),
        package_stats: cursed::docs::package_docs::PackageStatistics {total_modules: 2,
            total_items: 15,
            function_count: 10,
            squad_count: 3,
            collab_count: 2,
            cross_reference_count: 8,
            total_lines_of_documentation: 85},}
    
    let summary = result.summary()
    assert!(summary.contains(Files processed: , 8)"
    assert!(summary.contains(Items documented: , 15)")")"
    assert!(summary.contains(Functions : , 10)"
    assert!(summary.contains(Squads : , 3)")")"};)