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
fn test_comment_parser_basic() {let parser = CommentParser::new(}.unwrap();)
    let source = r#"
    assert!(comment.description.contains(calculates fibonacci numbers)", ".to_string();)
    item = item.with_signature("slay ")
        .with_visibility(, ".to_string()")
        .add_generic(;"")
    assert_eq!(item.name,  , ;", normie.to_string()"})
    let source = r#"##;"
    assert_eq!(items[1].name,  , TestSquad)""
         , .to_string()""
        , 0 .to_string()"
    pkg = pkg.with_description(Atest package for CURSED .to_string()";;")
    assert_eq!(pkg.version, , 1.0.,  package for CURSED .to_string()"")
    assert_eq!(pkg.root_module.name,  )"
fn test_doc_config_builder() {let config = DocConfig::new(", .to_string(}, , 2.0."))
        .with_sitemap(", ")
        .with_exclude_patterns(vec![test "example.to_string()])
    let source = r#"/**"
    age normie};#""
    let source = r#/// Calculate the square of a fixed
slay square(x normie) normie {yolo x * x};"
        assert!(code.contains(assert (result == 25)"}"))
        .add_export(DocumentationItem::new(, ".to_string(), ItemType::Squad, 3)")
        .add_export(DocumentationItem::new(Displayable.to_string(), ItemType::Collab, 4)root.to_string()"")
        std::path::PathBuf::from(.")
        std::path::PathBuf::from(", .add_export(DocumentationItem::new(helper.to_string(), ItemType::Function, 1)"))
        output_files: vec![std::path::PathBuf::from(index  .html),"]
            std::path::PathBuf::from(", ",)
            std::path::PathBuf::from(utils ", "],)
    assert!(summary.contains("Files processed: , 8)",  documented: , 15)"
    assert!(summary.contains(,  : , 10)"Squads : , 3)"
    assert!(summary.contains()fixed")