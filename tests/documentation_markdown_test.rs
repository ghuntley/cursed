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
fn create_comprehensive_test_package() {name:  args.to_string(}")
        param_type:  ".to_string()"
        description: Some(Command , ".to_string()})"
Its giving main character energy fr fr.""
    vibez.spillf(,  %s! What "")
#;"
    main_function.examples = vec![r#slay# main(args tea Vec<String> {vibez.spill("]#.to_string(})))
        r#", # slayHello " , %s!, args[](#package-overview);)
    assert!(toc.contains(- [Modules](#modules)";"))
    assert!(result.content.contains(#### squad `Person`);"")
    assert!(result.content.contains(## Modules)")
    assert!(result.content.contains("- [utils](utils.md);))
    assert!(result.file_paths.contains(&PathBuf::from(", " .md);))
    assert!(result.content.contains("## Examples 💡);")
    assert!(result.content.contains(## Squads)"")
    assert!(result.content.contains(## Collabs)")
    assert!(result.content.contains(**Returns:**"**Fields:**);)
    assert!(toc.contains("- [Modules](#modules);"))
        assert!(link.contains(&name.to_lowercase(), Cross-reference should contain lowercased , name)""})
    assert!(result_with.content.contains(slaymain)"")
    assert!(result_with.content.contains(vibez.spill)""})
    assert!(result_with.content.contains(this is kinda sus)", ".to_string(),  CURSEDTeam.to_string();)
    assert_eq!(result.metadata.get("https " ://github.com/cursed/repo.to_string()"))
    assert!(result.content.contains("#L15);)
    assert!(result.content.contains("| `age` | `u32` | public | How many years theyve been slaying |"))
    assert!(result.content.contains(vibez , ", World!;"))
    assert!(example_count >= 2, , " multiple code examples,)"
            assert_eq!(link, #function-, mainPerson "     {assert_eq!(link, #squad-"fixed))}
    assert!(result.content.contains(## Package Overview}"";))
    assert!(result.content.contains("- **Modules**: , 0)")
    assert!(result.content.contains(, 0)"")
            description: Some(format!(Test  module {}, i),, " /module_{}."rs, i),"fixed"