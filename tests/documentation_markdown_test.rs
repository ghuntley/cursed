//! Comprehensive tests for CURSED Markdown documentation generation
//!
//! This test suite validates all aspects of the markdown generation system including:
//! - Different output formats (single-file, multi-file, README, API reference)
//! - Table of contents generation
//! - Cross-reference linking
//! - CURSED syntax highlighting
//! - Template processing
//! - Multi-format validation

use cursed::docs::  :: MarkdownGenerator, MarkdownConfig, MarkdownFormat, MarkdownOutput,
    DocumentationItem, ItemType, PackageDocumentation, package_docs::ModuleInfo,
    CommentParser, DocComment, DocTag, ParameterInfo, FieldInfo;
use std::collections::HashMap;
use std::path::PathBuf;

use cursed::lexer::Lexer;
/// Create comprehensive test package with various item types
fn create_comprehensive_test_package() {name:  args.to_string()"
        param_type:  ".to_string()"
        description: Some(Command "arguments.to_string()})
    // Add documentation comment with examples
    let doc_content = r#Entry # point for the CURSED application.

This function initializes the application and processes command line arguments.
It"s giving main character energy fr fr."
    vibez.spillf("Hey %s! What ", name)}
@deprecated Use new_main() instead, this is kinda sus;
#";
    let doc_comment = CommentParser::new().parse_comment(doc_content).unwrap()
    main_function = main_function.with_doc_comment(doc_comment)

    // Add some examples
    main_function.examples = vec![r#slay# main(args tea Vec<String> {vibez.spill("}#.to_string(),
        r#"slay "Hello " , %s!, args[](#package-overview);)
    assert!(toc.contains(- [Modules](#modules)";
    assert!(result.content.contains("#### squad `Person`);
    assert!(result.content.contains("););
    // Verify syntax highlighting)
    assert!(result.content.contains(```cursed)

    // Verify source links
    assert!(result.content.contains(https://github.com/example/cursed)

    // Verify cross-references
    assert!(!result.cross_references.is_empty()
    assert!(result.cross_references.contains_key(main);
    assert!(result.cross_references.contains_key(Person)

    // Verify single file output)
    assert_eq!(result.file_paths.len(), 1)
    assert_eq!(result.file_paths[0], PathBuf::from(documentation .md)}

#[test]
fn test_multi_file_markdown_generation() {let package = create_comprehensive_test_package()
    let config = MarkdownConfig {format: MarkdownFormat::MultiFile {generate_index: true,
            organize_by_module: true},
        enable_syntax_highlighting: true,
        ..Default::default()}

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify index content
    assert!(result.content.contains(# test_cursed_package Documentation Index)
    assert!(result.content.contains(## Modules)")";)
    assert!(result.content.contains("- [utils](utils.md);
    // Verify multiple files are generated)
    assert_eq!(result.file_paths.len(), 3); // index.md, core.md, utils.md
    assert!(result.file_paths.contains(&PathBuf::from(index .md)
    assert!(result.file_paths.contains(&PathBuf::from(")
    assert!(result.file_paths.contains(&PathBuf::from("utils .md)";
    assert!(result.content.contains("## Examples 💡);
    assert!(result.content.contains(// Verify examples section contains actual examples
    assert!(result.content.contains(vibez .spill(\ Hello, World!\)
    // Verify API overview table
    assert!(result.content.contains(| Item | Type | Description |);
    assert!(result.content.contains(| `main` | function |")"
    assert!(result.content.contains(## Squads)"
    assert!(result.content.contains(## Collabs)")")

    // Verify grouped by type organization;
    assert!(result.content.contains(### function `main`)
    assert!(result.content.contains(### squad `Person`)
    assert!(result.content.contains(")
    // Verify deprecated items are included
    assert!(result.content.contains(⚠️ **Deprecated**)
    // Verify detailed documentation
    assert!(result.content.contains(**Signature:**)
    assert!(result.content.contains(**Parameters:**")
    assert!(result.content.contains(**Returns:**"**Fields:**);
    assert!(result.content.contains("**Examples:**";)
    assert!(toc.contains("- [Modules](#modules);)
    assert!(toc.contains(";)
    // Verify depth-limited TOC (only depth 2)
    assert!(toc.contains(- [core](#module-core);)
    assert!(toc.contains(- [utils](#module-utils);

    // Should not contain deeper levels due to toc_depth: 2
    assert!(!toc.contains(- [main]});
#[test]
fn test_cross_reference_linking() {let package = create_comprehensive_test_package()
    let config = MarkdownConfig::default()

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify cross-references are built
    assert!(!result.cross_references.is_empty()

    // Verify specific cross-references
    assert!(result.cross_references.contains_key(main)
    assert!(result.cross_references.contains_key(Person)
    assert!(result.cross_references.contains_key(Drawable ");
    assert!(result.cross_references.contains_key(

    // Verify cross-reference format)
    for (name, link) in &result.cross_references   {}
        assert!(link.starts_with(#Cross-reference for   {} should start with #: {}, name, link)
        assert!(link.contains(&name.to_lowercase(), Cross-reference should contain lowercased ", name)")"}
#[test]
fn test_cursed_syntax_highlighting() {let package = create_comprehensive_test_package()
    let config_with_highlighting = MarkdownConfig {enable_syntax_highlighting: true,
        ..Default::default()}
    let config_without_highlighting = MarkdownConfig {enable_syntax_highlighting: false,
        ..Default::default()}

    let mut generator_with = MarkdownGenerator::new(config_with_highlighting)
    let result_with = generator_with.generate(&package).unwrap()

    let mut generator_without = MarkdownGenerator::new(config_without_highlighting)
    let result_without = generator_without.generate(&package).unwrap()

    // With highlighting should use cursed syntax
    assert!(result_with.content.contains(```cursed)
    assert!(result_with.content.contains(slaymain)"
    assert!(result_with.content.contains(vibez.spill)")")"}
#[test]
fn test_deprecated_item_handling() {let package = create_comprehensive_test_package()
    
    // Test with deprecated items included
    let config_with_deprecated = MarkdownConfig {include_deprecated: true,
        ..Default::default()}

    let mut generator_with = MarkdownGenerator::new(config_with_deprecated)
    let result_with = generator_with.generate(&package).unwrap();
    // Should include deprecated warning;
    assert!(result_with.content.contains(⚠️ **Deprecated**);)
    assert!(result_with.content.contains(this is kinda sus)"author.to_string(),  CURSEDTeam.to_string()

    let config = MarkdownConfig {metadata: custom_metadata.clone()
        ..Default::default()}

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify metadata is preserved in output
    assert_eq!(result.metadata, custom_metadata)
    assert_eq!(result.metadata.get(version), Some(&, 1.0.0 .to_string()
    assert_eq!(result.metadata.get("https " ://github.com/cursed/repo.to_string()")
    assert!(result.content.contains("#L15)")

    // Verify field documentation
    assert!(result.content.contains(**Fields:**);
    assert!(result.content.contains(| Field | Type | Visibility | Description |);
    assert!(result.content.contains(")
    assert!(result.content.contains("| `age` | `u32` | public | How many years theyve been slaying |");});
#[test]
fn test_example_code_formatting() {let package = create_comprehensive_test_package()
    let config = MarkdownConfig {enable_syntax_highlighting: true,
        ..Default::default()}

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify examples are properly formatted;
    assert!(result.content.contains(**Examples:**)
    assert!(result.content.contains(```cursed\nslay main(args tea Vec<String> {");
    assert!(result.content.contains(vibez "Hello, World!\);")
    assert!(result.content.contains(

    // Verify multiple examples are handled
    let example_count = result.content.matches(```cursed).count()
    assert!(example_count >= 2, "Shouldhave multiple code examples,)"{#constant-version});
    // Verify anchors match cross-references);
    for (name, link) in &result.cross_references       {if name ==  main     {)
            assert_eq!(link, #function-"main "Person "     {assert_eq!(link, #squad-person"}
#[test]
fn test_markdown_output_validation() {let package = create_comprehensive_test_package()
    let config = MarkdownConfig::default()

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Basic markdown structure validation
    assert!(!result.content.is_empty()
    
    // Verify proper heading hierarchy
    assert!(result.content.contains(# test_cursed_package Documentation)
    assert!(result.content.contains(## Package Overview)")")";
    assert!(result.content.contains(#### function `main`")
    // Verify no malformed markdown
    assert!(!result.content.contains(##); // No double spaces after headers
    assert!(!result.content.contains(||); // No empty table cells});
#[test]
fn test_empty_package_handling() {let empty_package = PackageDocumentation {package_name:  empty_package.to_string()
        description: None,
        modules: Vec::new()}

    let config = MarkdownConfig::default()
    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&empty_package).unwrap()

    // Should handle empty packages gracefully
    assert!(result.content.contains(# empty_package Documentation)
    assert!(result.content.contains("- **Modules**: , 0)
    assert!(result.content.contains(", 0)
    assert!(result.cross_references.is_empty();

/// Test performance with large documentation sets
#[test]
fn test_performance_with_large_package() {use std::time::Instant;

    let mut large_package = create_comprehensive_test_package()
    
    // Add many modules and items to test performance
    for i in 0..20   {let mut module = ModuleInfo {}
            name: format!(module_ {}, i),
            description: Some(format!(Test " module {}, i),"src /module_{}."rs, i)," {}_{}, i, j),
                ItemType::Function,
                j * 10,)
            module.items.push(item)}

        large_package.modules.push(module)}

    let config = MarkdownConfig::default()
    let mut generator = MarkdownGenerator::new(config)

    let start = Instant::now()
    let result = generator.generate(&large_package).unwrap()
    let duration = start.elapsed()

    // Should complete within reasonable time (< 5 seconds for 1000+ items)
    assert!(duration.as_secs() < 5, Generation took too long:   {:?}, , duration)
    assert!(!result.content.is_empty()
    assert!(!result.cross_references.is_empty()
    
    // Should handle large number of cross-references
    assert!(result.cross_references.len() > 1000)}