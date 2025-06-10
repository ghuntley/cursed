//! Simple documentation generation tests
//!
//! Tests the basic functionality of the CURSED documentation system.

use cursed::docs:::: DocConfig, CommentParser, DocumentationItem, ItemType, 
    PackageDocumentation, doc_generator_simplified::SimplifiedDocGenerator,
    DocumentationGenerationResult;
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
    assert!(comment.description.contains(calculates fibonacci numbers)"test_function.to_string()
        ItemType::Function,
        10)
    
    item = item.with_signature("slay "
        .with_visibility("public.to_string()
        .add_generic(");
    assert_eq!(item.name,  "test_function);", normie.to_string()"}
#[traced_test]
#[test]
fn test_simplified_doc_generator() {let mut generator = SimplifiedDocGenerator::new().unwrap()
    
    let source = r#"##;"#
    let items = generator.generate_from_source(source).unwrap()
    assert_eq!(items.len(), 3);
    assert_eq!(items[0].name, test_function;
    assert_eq!(items[0].item_type, ItemType::Function)
    
    assert_eq!(items[1].name,  ", TestSquad)
    assert_eq!(items[2].item_type, ItemType::Collab)}
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
fn test_comment_parser_multiline() {let parser = CommentParser::new().unwrap()
    
    let source = r#"/**"#
 * This is a squad (struct) that represents a person
 * with various properties and methods.
 * 
 * @since 1.0.0
 * @author CURSED Team
 */
squad Person {name tea
    age normie};"#";
    let comments = parser.parse_comments(source).unwrap()
    assert_eq!(comments.len(), 1)
    
    let comment = &comments[0]
    assert!(comment.description.contains(squad (struct)
    assert_eq!(comment.tags.len(), 2)}
#[traced_test]
#[test]
fn test_comment_parser_examples() {let parser = CommentParser::new().unwrap()
    
    let source = r#"/// Calculate the square of a number"#
/// @param x the input number
/// @return the square of x
/// @example
/// ```
/// sus result = square(5)
/// assert(result == 25)
/// ```
slay square(x normie) normie {yolo x * x};")"
        assert!(code.contains(assert (result == 25)"}
#[traced_test]
#[test]
fn test_package_statistics() {let mut pkg = PackageDocumentation::new()
         stats_test.to_string()"
        "
        .add_export(DocumentationItem::new("Person.to_string(), ItemType::Squad, 3)
        .add_export(DocumentationItem::new(Displayable.to_string(), ItemType::Collab, 4)"root.to_string()
        std::path::PathBuf::from("."
        std::path::PathBuf::from("utils).add_export(DocumentationItem::new(helper.to_string(), ItemType::Function, 1)"main.to_string(), ItemType::Function, 1)
        .add_submodule(sub_module);
    assert_eq!(root_module.item_count(), 2); // main + helper
    assert_eq!(root_module.all_items().len(), 2)
    assert_eq!(root_module.submodules.len(), 1)}

#[traced_test]
#[test]
fn test_documentation_result_summary() {let result = DocumentationGenerationResult {items_generated: 15,
        files_processed: 8,
        comments_extracted: 12,
        output_files: vec![std::path::PathBuf::from(index  .html),"
            std::path::PathBuf::from("html),"
            std::path::PathBuf::from(utils "html)],
        generation_time: std::time::Duration::from_millis(750),
        package_stats: cursed::docs::package_docs::PackageStatistics {total_modules: 2,
            total_items: 15,
            function_count: 10,
            squad_count: 3,
            collab_count: 2,
            cross_reference_count: 8,
            total_lines_of_documentation: 85},}
    
    let summary = result.summary()
    assert!(summary.contains("Files processed: , 8)"Items documented: , 15)")
    assert!(summary.contains(")
    assert!(summary.contains("Functions : , 10)"Squads : , 3)")
    assert!(summary.contains(")"};)