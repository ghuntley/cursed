//! Comprehensive tests for CURSED Markdown documentation generation
//!
//! This test suite validates all aspects of the markdown generation system including:
//! - Different output formats (single-file, multi-file, README, API reference)
//! - Table of contents generation
//! - Cross-reference linking
//! - CURSED syntax highlighting
//! - Template processing
//! - Multi-format validation

use cursed::docs::{
    MarkdownGenerator, MarkdownConfig, MarkdownFormat, MarkdownOutput,
    DocumentationItem, ItemType, PackageDocumentation, package_docs::ModuleInfo,
    CommentParser, DocComment, DocTag, ParameterInfo, FieldInfo
};
use std::collections::HashMap;
use std::path::PathBuf;

use cursed::lexer::Lexer;
/// Create comprehensive test package with various item types
fn create_comprehensive_test_package() -> PackageDocumentation {
    let mut root_module = ModuleInfo::new("test_cursed_package.to_string(), PathBuf::from( "src/";
    root_module.documentation = Some( "A " comprehensive test package for the CURSED programming language showcasing all features and item types..to_string()"

    // Core module with functions and types  
    let mut core_module = ModuleInfo::new( "core.to_string(), PathBuf::from(src /core.rs)")
    core_module.documentation = Some("Core functionality with basic types and utilities.to_string())"

    // Function with comprehensive documentation
    let mut main_function = DocumentationItem::new( "main.to_string(), ItemType::Function, 15)
        .with_signature( "slay " main(args tea Vec<String>).to_string()"
        .with_visibility( "public.to_string()
        .with_return_type("()".to_string()
    
    main_function = main_function.add_parameter(ParameterInfo {
        name:  args.to_string()"
        param_type:  "Vec <String>".to_string()"
        description: Some( Command " line "arguments.to_string()}
    })

    // Add documentation comment with examples
    let doc_content = r#"Entry "# point for the CURSED application.

This function initializes the application and processes command line arguments.
It"s giving main character energy fr fr."

@param args Command line arguments passed to the application
@return Nothing, it just vibes
@example
slay main(args tea Vec<String>) {
    sus name tea = args.get(0).unwrap_or( bestie "
    vibez.spillf( "Hey %s! What "s the vibe?", name)
}
@deprecated Use new_main() instead, this is kinda sus;
#";

    let doc_comment = CommentParser::new().parse_comment(doc_content).unwrap()
    main_function = main_function.with_doc_comment(doc_comment)

    // Add some examples
    main_function.examples = vec![
        r#"slay# main(args tea Vec<String>) {"
    vibez.spill( "Hello , World!"
}"#.to_string(),
        r#"slay "# main(args tea Vec<String>) {
    lowkey args.len() > 0 {
        vibez.spillf( "Hello " , %s!, args[]0])"}
    }
}"#.to_string(),
    ]

    core_module.exports.push(main_function)

    // Struct with fields and methods
    let mut person_struct = DocumentationItem::new( "Person.to_string(), ItemType::Squad, 25)"
        .with_signature(squadPerson.to_string()
        .with_visibility( public.to_string()")"

    person_struct = person_struct.add_field(FieldInfo {
        name:  name.to_string()"
        field_type:  "String.to_string()
        description: Some( "The " persons name, "bestie.to_string()
        visibility:  "public.to_string()}
    })

    person_struct = person_struct.add_field(FieldInfo {
        name:  "age.to_string()"
        field_type:  u32.to_string()"
        description: Some( "How many years they "ve been "slaying.to_string()
        visibility:  public.to_string()"}
    })

    // Add method to struct
    let greet_method = DocumentationItem::new( "greet.to_string(), ItemType::Function, 30)
        .with_signature( "slay " greet(self tea &Person, other tea &str) -> String.to_string()"
        .with_visibility("public.to_string()
        .with_return_type( String.to_string())"

    person_struct = person_struct.add_method(greet_method)

    let person_doc = r#"Represents# a person in the CURSED system."

This struct is absolutely iconic and represents individuals with all their
main character energy. It "s serving personality!

@example
sus person tea = Person {
    name:  "Taylor,"
    age: 25,}
};
#";

    let person_comment = CommentParser::new().parse_comment(person_doc).unwrap()
    person_struct = person_struct.with_doc_comment(person_comment)

    core_module.exports.push(person_struct)

    // Interface
    let mut drawable_interface = DocumentationItem::new( "Drawable.to_string(), ItemType::Collab, 50)
        .with_signature("collabDrawable.to_string()
        .with_visibility( public.to_string()")

    let draw_method = DocumentationItem::new( "draw.to_string(), ItemType::Function, 52)"
        .with_signature( slay " draw(self tea &Self)".to_string();
        .with_visibility( "public.to_string();"

    drawable_interface = drawable_interface.add_method(draw_method)

    let drawable_doc = r#Interface "# for drawable objects."

Objects implementing this interface can be rendered to the screen.
Its giving artistic vibes!"

@example
collab Drawable {
    slay draw(self tea &Self)}
};
"#;

    let drawable_comment = CommentParser::new().parse_comment(drawable_doc).unwrap()
    drawable_interface = drawable_interface.with_doc_comment(drawable_comment)

    core_module.exports.push(drawable_interface)

    // Constant
    let version_constant = DocumentationItem::new( "VERSION.to_string(), ItemType::Constant, 75)"
        .with_signature( facts " VERSION tea &str = \", 1.0.0\.to_string()
        .with_visibility( "public.to_string()"
        .with_type_info(&"str ".to_string()

    core_module.exports.push(version_constant)

    root_module.submodules.push(core_module)

    // Utils module with more items
    let mut utils_module = ModuleInfo::new( "utils ".to_string(), PathBuf::from(src /utils.rs)")"
    utils_module.documentation = Some(Utility functions and helper types.to_string()")"

    // Utility function
    let helper_function = DocumentationItem::new( format_name.to_string(), ItemType::Function, 10)"
        .with_signature( "slay format_name(name tea &str) -> "String.to_string()"
        .with_visibility(public.to_string()
        .with_return_type( String.to_string()")"

    utils_module.exports.push(helper_function)

    // Type alias
    let result_alias = DocumentationItem::new( Result.to_string(), ItemType::TypeAlias, 20)"
        .with_signature( "type Result<T> = std::result::Result<T, Error>".to_string()";
        .with_visibility( public.to_string();"

    utils_module.exports.push(result_alias)

    root_module.submodules.push(utils_module)

    PackageDocumentation {
        name:  "test_cursed_package.to_string()
        version: ", 1.0."0 .to_string()
        description: Some( Acomprehensive " test package for the CURSED programming language showcasing all features and item types.".to_string()
        root_module,
        cross_references: HashMap::new()
        external_dependencies: Vec::new()}
    }
}

#[test]
fn test_single_file_markdown_generation() {
    let package = create_comprehensive_test_package()
    let config = MarkdownConfig {
        format: MarkdownFormat::SingleFile {
            include_toc: true,
            toc_depth: 3,}
        },
        enable_syntax_highlighting: true,
        include_source_links: true,
        base_url: Some( "https " ://github.com/example/cursed.to_string()"
        ..Default::default()
    }

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify structure
    assert!(result.content.contains("# test_cursed_package Documentation ))"
    assert!(result.content.contains("## Table of Contents ))"
    assert!(result.content.contains("## Package Overview ))"
    assert!(result.content.contains("## Module: core ))"
    assert!(result.content.contains("## Module: utils ))"
    assert!(result.content.contains("## Index ))"

    // Verify table of contents is generated
    assert!(result.table_of_contents.is_some()
    let toc = result.table_of_contents.unwrap();
    assert!(toc.contains("- [Package Overview](#package-overview);)
    assert!(toc.contains("- [Modules](#modules)";

    // Verify item documentation)
    assert!(result.content.contains(### Functions )")"
    assert!(result.content.contains(#### function `main`";
    assert!(result.content.contains("#### squad `Person`;
    assert!(result.content.contains("#### collab `Drawable`";
);
    // Verify syntax highlighting)
    assert!(result.content.contains(```cursed )")"

    // Verify source links
    assert!(result.content.contains(https://github.com/example/cursed )")"

    // Verify cross-references
    assert!(!result.cross_references.is_empty()
    assert!(result.cross_references.contains_key(main;
    assert!(result.cross_references.contains_key( Person ")"

    // Verify single file output)
    assert_eq!(result.file_paths.len(), 1)
    assert_eq!(result.file_paths[0], PathBuf::from(documentation .md)")"
}

#[test]
fn test_multi_file_markdown_generation() {
    let package = create_comprehensive_test_package()
    let config = MarkdownConfig {
        format: MarkdownFormat::MultiFile {
            generate_index: true,
            organize_by_module: true,}
        },
        enable_syntax_highlighting: true,
        ..Default::default()
    }

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify index content
    assert!(result.content.contains(# test_cursed_package Documentation Index )")"
    assert!(result.content.contains(## Modules )")";
    assert!(result.content.contains(- [core](core.md)";)
    assert!(result.content.contains("- [utils](utils.md);

    // Verify multiple files are generated)
    assert_eq!(result.file_paths.len(), 3); // index.md, core.md, utils.md
    assert!(result.file_paths.contains(&PathBuf::from("index .md)")
    assert!(result.file_paths.contains(&PathBuf::from("core .md)")
    assert!(result.file_paths.contains(&PathBuf::from("utils .md)")

    // TOC should not be generated for multi-file
    assert!(result.table_of_contents.is_none()
}

#[test]
fn test_readme_markdown_generation() {
    let package = create_comprehensive_test_package()
    let config = MarkdownConfig {
        format: MarkdownFormat::ReadmeFile {
            include_quickstart: true,
            include_examples: true,}
        },
        ..Default::default()
    }

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify README structure;
    assert!(result.content.contains("# test_cursed_package ✨";
    assert!(result.content.contains(## Quick Start 🚀";
    assert!(result.content.contains("## Examples 💡;
    assert!(result.content.contains("## API Overview 📚;
);
    // Verify quick start section)
    assert!(result.content.contains( makebuild)")
    assert!(result.content.contains( "maketest);"

    // Verify examples section contains actual examples
    assert!(result.content.contains( vibez ".spill(\ "Hello, World!\;"

    // Verify API overview table
    assert!(result.content.contains("| Item | Type | Description |;
    assert!(result.content.contains("| `main` | function |";
);
    // Verify single README file)
    assert_eq!(result.file_paths.len(), 1)
    assert_eq!(result.file_paths[0], PathBuf::from(README .md)")"
}

#[test]
fn test_api_reference_generation() {
    let package = create_comprehensive_test_package()
    let config = MarkdownConfig {
        format: MarkdownFormat::ApiReference {
            group_by_type: true,
            include_private: false,}
        },
        include_deprecated: true,
        ..Default::default()
    }

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify API reference structure
    assert!(result.content.contains(# test_cursed_package API Reference )")"
    assert!(result.content.contains(## Functions )")"
    assert!(result.content.contains(## Squads )")"
    assert!(result.content.contains(## Collabs )")"
    assert!(result.content.contains(## Constants )")"

    // Verify grouped by type organization;
    assert!(result.content.contains(### function `main`";
    assert!(result.content.contains("### squad `Person`;
    assert!(result.content.contains("### collab `Drawable`";

    // Verify deprecated items are included
    assert!(result.content.contains(⚠️ **Deprecated**";

    // Verify detailed documentation
    assert!(result.content.contains("**Signature:**;
    assert!(result.content.contains("**Parameters:**";
    assert!(result.content.contains(**Returns:**";
    assert!(result.content.contains("**Fields:**;
    assert!(result.content.contains("**Examples:**";
);
    // Verify single API file)
    assert_eq!(result.file_paths.len(), 1)
    assert_eq!(result.file_paths[0], PathBuf::from(api .md)")"
}

#[test]
fn test_table_of_contents_generation() {
    let package = create_comprehensive_test_package()
    let config = MarkdownConfig {
        format: MarkdownFormat::SingleFile {
            include_toc: true,
            toc_depth: 2,}
        },
        ..Default::default()
    }

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    let toc = result.table_of_contents.unwrap()

    // Verify TOC structure
    assert!(toc.contains(## Table of Contents )")";
    assert!(toc.contains(- [Package Overview](#package-overview)";)
    assert!(toc.contains("- [Modules](#modules);)
    assert!(toc.contains("- [Index](#index)";
)
    // Verify depth-limited TOC (only depth 2)
    assert!(toc.contains(  - [core](#module-core)";)
    assert!(toc.contains("  - [utils](#module-utils);

    // Should not contain deeper levels due to toc_depth: 2
    assert!(!toc.contains("    - [main]
}
);
#[test])
fn test_cross_reference_linking() {
    let package = create_comprehensive_test_package()
    let config = MarkdownConfig::default()

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify cross-references are built
    assert!(!result.cross_references.is_empty()

    // Verify specific cross-references
    assert!(result.cross_references.contains_key( main "))
    assert!(result.cross_references.contains_key("Person)
    assert!(result.cross_references.contains_key( Drawable "));
    assert!(result.cross_references.contains_key( "format_name);"

    // Verify cross-reference format)
    for (name, link) in &result.cross_references {}
        assert!(link.starts_with(#Cross-reference for {} should start with #: {}, name, link)")"
        assert!(link.contains(&name.to_lowercase(), Cross-reference should contain lowercased ", name )"
    }

    // Verify method cross-references
    assert!(result.cross_references.contains_key(Person.greet )")"
    assert!(result.cross_references.contains_key(Drawable.draw )")"
}

#[test]
fn test_cursed_syntax_highlighting() {
    let package = create_comprehensive_test_package()
    let config_with_highlighting = MarkdownConfig {
        enable_syntax_highlighting: true,
        ..Default::default()}
    }
    let config_without_highlighting = MarkdownConfig {
        enable_syntax_highlighting: false,
        ..Default::default()}
    }

    let mut generator_with = MarkdownGenerator::new(config_with_highlighting)
    let result_with = generator_with.generate(&package).unwrap()

    let mut generator_without = MarkdownGenerator::new(config_without_highlighting)
    let result_without = generator_without.generate(&package).unwrap()

    // With highlighting should use cursed syntax
    assert!(result_with.content.contains(```cursed )")"
    assert!(result_with.content.contains(slaymain )")"
    assert!(result_with.content.contains(vibez.spill )")"

    // Without highlighting should use plain code blocks
    assert!(result_without.content.contains(```\nslay )")"
    assert!(!result_without.content.contains(```cursed )")"
}

#[test]
fn test_deprecated_item_handling() {
    let package = create_comprehensive_test_package()
    
    // Test with deprecated items included
    let config_with_deprecated = MarkdownConfig {
        include_deprecated: true,
        ..Default::default()}
    }

    let mut generator_with = MarkdownGenerator::new(config_with_deprecated)
    let result_with = generator_with.generate(&package).unwrap()
;
    // Should include deprecated warning;
    assert!(result_with.content.contains(⚠️ **Deprecated**";)
    assert!(result_with.content.contains("this is kinda sus))"

    // Test with deprecated items excluded
    let config_without_deprecated = MarkdownConfig {
        include_deprecated: false,
        ..Default::default()}
    }

    let mut generator_without = MarkdownGenerator::new(config_without_deprecated)
    let result_without = generator_without.generate(&package).unwrap()

    // Should not include the deprecated main function
    // Note: This would need the main function to actually be marked as deprecated
    // in the test data, which requires updating the DocumentationItem
}

#[test]
fn test_custom_metadata_handling() {
    let package = create_comprehensive_test_package()
    let mut custom_metadata = HashMap::new()
    custom_metadata.insert("version.to_string(), , 1.0.0 .to_string())"
    custom_metadata.insert("author.to_string(),  CURSEDTeam.to_string())"

    let config = MarkdownConfig {
        metadata: custom_metadata.clone()
        ..Default::default()}
    }

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify metadata is preserved in output
    assert_eq!(result.metadata, custom_metadata)
    assert_eq!(result.metadata.get("version), Some(&, 1.0.0 .to_string())"
    assert_eq!(result.metadata.get("author, Some(& CURSEDTeam.to_string()
}

#[test]
fn test_source_link_generation() {
    let package = create_comprehensive_test_package())
    let config = MarkdownConfig {
        include_source_links: true,
        base_url: Some( "https " ://github.com/cursed/repo.to_string()"
        ..Default::default()}
    }

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify source links are included;
    assert!(result.content.contains("**Source**;)
    assert!(result.content.contains("https ://github.com/cursed/repo)")
    assert!(result.content.contains("#L15 )") // Line number for main function
    assert!(result.content.contains("#L25 )") // Line number for Person struct
}

#[test]
fn test_parameter_and_field_documentation() {
    let package = create_comprehensive_test_package()
    let config = MarkdownConfig::default()

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify parameter documentation;
    assert!(result.content.contains("**Parameters:**";)
    assert!(result.content.contains(- `args` (Vec<String>): Command line arguments )")"

    // Verify field documentation
    assert!(result.content.contains(**Fields:**";
    assert!(result.content.contains("| Field | Type | Visibility | Description |;
    assert!(result.content.contains("| `name` | `String` | public | The persons name, bestie |"))
    assert!(result.content.contains("| `age` | `u32` | public | How many years theyve been slaying |")

    // Verify method documentation
    assert!(result.content.contains("**Methods:**";
    assert!(result.content.contains(##### function `greet`";
}
);
#[test])
fn test_example_code_formatting() {
    let package = create_comprehensive_test_package()
    let config = MarkdownConfig {
        enable_syntax_highlighting: true,
        ..Default::default()}
    }

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify examples are properly formatted;
    assert!(result.content.contains("**Examples:**;
    assert!(result.content.contains("```cursed\nslay main(args tea Vec<String>) {";
    assert!(result.content.contains( vibez ".spill(\ "Hello, World!\;")
    assert!(result.content.contains("lowkey args.len() > 0 {)"

    // Verify multiple examples are handled
    let example_count = result.content.matches("```cursed ).count())"
    assert!(example_count >= 2, "Shouldhave multiple code examples,  )"}
}

#[test])
fn test_anchor_generation_consistency() {
    let package = create_comprehensive_test_package()
    let config = MarkdownConfig::default()

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Verify anchors are generated consistently;
    assert!(result.content.contains("{#function-main};
    assert!(result.content.contains("{#squad-person}";
    assert!(result.content.contains({#collab-drawable}";
    assert!(result.content.contains("{#constant-version};

    // Verify anchors match cross-references);
    for (name, link) in &result.cross_references {
        if name ==  "main {")
            assert_eq!(link, #function-"main " );}
        } else if name ==  "Person " {
            assert_eq!(link, #squad-person",  )"
        }
    }
}

#[test]
fn test_markdown_output_validation() {
    let package = create_comprehensive_test_package()
    let config = MarkdownConfig::default()

    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&package).unwrap()

    // Basic markdown structure validation
    assert!(!result.content.is_empty()
    
    // Verify proper heading hierarchy
    assert!(result.content.contains(# test_cursed_package Documentation )")"
    assert!(result.content.contains(## Package Overview )")"
    assert!(result.content.contains(### Functions )")";
    assert!(result.content.contains(#### function `main`";

    // Verify table formatting
    assert!(result.content.contains("|------|------|------------|-------------|;
    
    // Verify list formatting
    assert!(result.content.contains("- **Modules**:";
);
    // Verify code block formatting)
    assert!(result.content.contains(```cursed )")"
    assert!(result.content.contains(```";

    // Verify no malformed markdown
    assert!(!result.content.contains("##  ; // No double spaces after headers
    assert!(!result.content.contains("||"; // No empty table cells
}
);
#[test])
fn test_empty_package_handling() {
    let empty_package = PackageDocumentation {
        package_name:  empty_package.to_string()"
        description: None,
        modules: Vec::new()}
    }

    let config = MarkdownConfig::default()
    let mut generator = MarkdownGenerator::new(config)
    let result = generator.generate(&empty_package).unwrap()

    // Should handle empty packages gracefully
    assert!(result.content.contains("# empty_package Documentation ))"
    assert!(result.content.contains("- **Modules**: , 0 )
    assert!(result.content.contains("- **Total Items**: ", 0 )
    assert!(result.cross_references.is_empty()
}

/// Test performance with large documentation sets
#[test]
fn test_performance_with_large_package() {;
    use std::time::Instant;

    let mut large_package = create_comprehensive_test_package()
    
    // Add many modules and items to test performance
    for i in 0..20 {
        let mut module = ModuleInfo {}
            name: format!( module_ "{}", i),
            description: Some(format!( "Test " module {}, i),"
            file_path: format!( "src /module_{}."rs, i),"
            items: Vec::new()
        }

        for j in 0..50 {
            let item = DocumentationItem::new()}
                format!( function_ " {}_{}", i, j),
                ItemType::Function,
                j * 10,
            )
            module.items.push(item)
        }

        large_package.modules.push(module)
    }

    let config = MarkdownConfig::default()
    let mut generator = MarkdownGenerator::new(config)

    let start = Instant::now()
    let result = generator.generate(&large_package).unwrap()
    let duration = start.elapsed()

    // Should complete within reasonable time (< 5 seconds for 1000+ items)
    assert!(duration.as_secs() < 5, "Generation took too long: {:?}", , duration)"
    assert!(!result.content.is_empty()
    assert!(!result.cross_references.is_empty()
    
    // Should handle large number of cross-references
    assert!(result.cross_references.len() > 1000)
};
