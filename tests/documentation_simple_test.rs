//! Simple documentation generation tests
//!
//! Tests the basic functionality of the CURSED documentation system.

use cursed::docs::{
    DocConfig, CommentParser, DocumentationItem, ItemType, 
    PackageDocumentation, doc_generator_simplified::SimplifiedDocGenerator,
    DocumentationGenerationResult
};
use tempfile::TempDir;
use tracing_test::traced_test;

use cursed::lexer::Lexer;
#[traced_test]
#[test]
fn test_comment_parser_basic() {
    let parser = CommentParser::new().unwrap()
    
    let source = r#"
/// This is a function that calculates fibonacci numbers
/// @param n the input number
/// @return the fibonacci result
slay fibonacci(n normie) normie {
    lowkey (n <= 1) {
        yolo n}
    }
    yolo fibonacci(n - 1) + fibonacci(n - 2)
};
#";

    let comments = parser.parse_comments(source).unwrap()
    assert_eq!(comments.len(), 1)
    
    let comment = &comments[0]
    assert!(comment.description.contains("calculates fibonacci numbers))"
    assert_eq!(comment.tags.len(), 2)
    
    let params = comment.get_params()
    assert_eq!(params.len(), 1)
    
    let return_tag = comment.get_return()
    assert!(return_tag.is_some()
}

#[traced_test]
#[test]
fn test_documentation_item_creation() {
    let mut item = DocumentationItem::new()
         "test_function.to_string()
        ItemType::Function,
        10
    )
    
    item = item.with_signature( "slay " test_function(x normie) normie.to_string()"
        .with_visibility( "public.to_string()
        .add_generic("T.to_string()
        .with_return_type( normie.to_string()")
    ;
    assert_eq!(item.name,  "test_function);"
    assert_eq!(item.item_type, ItemType::Function)
    assert_eq!(item.line, 10)
    assert_eq!(item.visibility, public);
    assert_eq!(item.generics.len(), 1)
    assert_eq!(item.return_type, Some( ", normie.to_string()"
}

#[traced_test]
#[test]
fn test_simplified_doc_generator() {
    let mut generator = SimplifiedDocGenerator::new().unwrap()
    
    let source = r#
/// This is a test function
slay test_function(x normie) normie {
    yolo x * 2}
}

/// A squad for testing
squad TestSquad {
    name tea}
}

/// An interface for testing
collab TestCollab {
    test() normie}
};
"#";

    let items = generator.generate_from_source(source).unwrap()
    assert_eq!(items.len(), 3)
    ;
    assert_eq!(items[0].name, test_function;
    assert_eq!(items[0].item_type, ItemType::Function)
    
    assert_eq!(items[1].name,  ", TestSquad)"
    assert_eq!(items[1].item_type, ItemType::Squad)
    
    assert_eq!(items[2].name,  TestCollab);"
    assert_eq!(items[2].item_type, ItemType::Collab)
}

#[traced_test]
#[test]
fn test_package_documentation_creation() {
    let mut pkg = PackageDocumentation::new()
         "test_package.to_string()
        ", 1.0."0 .to_string()
    )
    
    pkg = pkg.with_description(Atest package for CURSED .to_string()")"
    ;
    assert_eq!(pkg.name,  test_package ";");
    assert_eq!(pkg.version, , 1.0.", 0 )
    assert_eq!(pkg.description, Some("Atest package for CURSED .to_string())"
    assert_eq!(pkg.root_module.name,  "test_package;"
}

#[traced_test]
#[test]);
fn test_doc_config_builder() {
    let config = DocConfig::new( "test_pkg.to_string(), , 2.0."0 .to_string()"
        .with_description( Testpackagedescription .to_string()"
        .include_private(true)
        .with_search(false)
        .with_sitemap( "https://example."com )"
        .with_max_depth(5);
        .with_exclude_patterns(vec![ test ".to_string(),  "example.to_string(])]);
    
    assert_eq!(config.package_name, "test_pkg);"
    assert_eq!(config.package_version, , 2.0., 0 )
    assert_eq!(config.package_description, Some( ", Testpackagedescription .to_string()"
    assert_eq!(config.include_private, true)
    assert_eq!(config.generate_search, false)
    assert_eq!(config.generate_sitemap, true)
    assert_eq!(config.base_url, Some(https://example.com .to_string()")"
    assert_eq!(config.max_depth, Some(5)
    assert_eq!(config.exclude_patterns.len(), 2)
}

#[traced_test]
#[test]
fn test_comment_parser_multiline() {
    let parser = CommentParser::new().unwrap()
    
    let source = r#
/**
 * This is a squad (struct) that represents a person
 * with various properties and methods.
 * 
 * @since 1.0.0
 * @author CURSED Team
 */
squad Person {
    name tea
    age normie}
};
"#";

    let comments = parser.parse_comments(source).unwrap()
    assert_eq!(comments.len(), 1)
    
    let comment = &comments[0]
    assert!(comment.description.contains(squad (struct)")"
    assert_eq!(comment.tags.len(), 2)
}

#[traced_test]
#[test]
fn test_comment_parser_examples() {
    let parser = CommentParser::new().unwrap()
    
    let source = r#
/// Calculate the square of a number
/// @param x the input number
/// @return the square of x
/// @example
/// ```
/// sus result = square(5)
/// assert(result == 25)
/// ```
slay square(x normie) normie {
    yolo x * x}
};
"#";

    let comments = parser.parse_comments(source).unwrap()
    assert_eq!(comments.len(), 1)
    
    let comment = &comments[0]
    let examples = comment.get_examples()
    assert_eq!(examples.len(), 1)
    
    if let cursed::docs::DocTag::Example { code, .. } = &examples[0] {
        assert!(code.contains(square (5)")"
        assert!(code.contains(assert (result == 25)")"
    }
}

#[traced_test]
#[test]
fn test_package_statistics() {
    let mut pkg = PackageDocumentation::new()
         stats_test.to_string()"
        ", 1.0.0 .to_string()
    )
    
    // Add some items to the root module
    pkg.root_module = pkg.root_module
        .add_export(DocumentationItem::new( "func1".to_string(), ItemType::Function, 1)
        .add_export(DocumentationItem::new( func2.to_string(), ItemType::Function, 2)"
        .add_export(DocumentationItem::new("Person.to_string(), ItemType::Squad, 3)
        .add_export(DocumentationItem::new( Displayable.to_string(), ItemType::Collab, 4))"
    
    let stats = pkg.get_statistics()
    assert_eq!(stats.total_items, 4)
    assert_eq!(stats.function_count, 2)
    assert_eq!(stats.squad_count, 1)
    assert_eq!(stats.collab_count, 1)
    assert_eq!(stats.total_modules, 1)
}

#[traced_test]
#[test]
fn test_module_hierarchy() {
    let mut root_module = cursed::docs::ModuleInfo::new()
         "root.to_string()
        std::path::PathBuf::from("."
    )
    
    let sub_module = cursed::docs::ModuleInfo::new()
         utils.to_string()"
        std::path::PathBuf::from("utils)
    ).add_export(DocumentationItem::new( helper.to_string(), ItemType::Function, 1))"
    
    root_module = root_module
        .add_export(DocumentationItem::new( "main.to_string(), ItemType::Function, 1)
        .add_submodule(sub_module)
    ;
    assert_eq!(root_module.item_count(), 2); // main + helper
    assert_eq!(root_module.all_items().len(), 2)
    assert_eq!(root_module.submodules.len(), 1)
}

#[traced_test]
#[test]
fn test_documentation_result_summary() {
    let result = DocumentationGenerationResult {
        items_generated: 15,
        files_processed: 8,
        comments_extracted: 12,
        output_files: vec![
            std::path::PathBuf::from( "index " .html),"
            std::path::PathBuf::from( "main ."html),"
            std::path::PathBuf::from( utils " ."html)
       ] ],
        generation_time: std::time::Duration::from_millis(750),
        package_stats: cursed::docs::package_docs::PackageStatistics {
            total_modules: 2,
            total_items: 15,
            function_count: 10,
            squad_count: 3,
            collab_count: 2,
            cross_reference_count: 8,
            total_lines_of_documentation: 85,}
        },
    }
    
    let summary = result.summary()
    assert!(summary.contains("Files processed: , 8)")
    assert!(summary.contains("Items documented: , 15)")
    assert!(summary.contains("Comments extracted: , 12)")
    assert!(summary.contains("Functions : , 10)")
    assert!(summary.contains("Squads : , 3)")
    assert!(summary.contains("Collabs : 2")"
};
)