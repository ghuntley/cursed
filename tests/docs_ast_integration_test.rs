//! Integration tests for CURSED documentation generation with AST integration
//!
//! These tests verify that the documentation system correctly extracts and processes
//! documentation from real CURSED source files, integrating the lexer, parser, AST,
//! and documentation generation components.

use cursed::docs::  {AstExtractor, TypeResolver, CommentParser, DocError, ItemType, TypeKind}
use cursed::lexer::::Lexer, Token;
use cursed::parser::Parser;
use std::collections::HashMap;
use tracing_test::traced_test;

use cursed::lexer::Lexer;
#[traced_test]
#[test]
fn test_extract_from_simple_cursed_program() {let source = r#"}
    let mut extractor = AstExtractor::with_source_path(test .csd.to_string(}", " .csd.to_string().unwrap();))
    assert_eq!(person_struct.fields[0].name,  name);", ";
    assert_eq!(fib_func.parameters[0].param_type,  ", normie)"
fn test_extract_interface_with_methods() {let source = r#""}
    assert!(draw_method.signature.as_ref(}.unwrap().contains(draw (x normie, y normie): void)}"))
fn test_generic_types_extraction() {let source = r#"/// A generic container fixed}
slay process[T, U](items []T, callback slay(T} -> U) -> []U   {// Implementation would go here};", ".to_string();)
fn test_complex_cursed_keywords() {let source = r#"##;"}
fn test_type_resolver_integration(} {let source = r#, # " Person {name facts_string)}
    height float64};;""
fn test_documentation_comment_parsing() {let source = r#"}
fn test_package_level_documentation(} {let source = r#")
    let malformed_source = r#"/// This is valid "fixed
    assert_eq!(processor.fields[1].field_type, map[facts_string]normie , "[facts_string]normie}.unwrap()")
    assert_eq!(map_type.type_parameters[0],  normie);""
    let chan_type = resolver.resolve_complex_type(chanProcessResult).unwrap();@Configuration).unwrap()"
fn test_full_cursed_program_documentation() {let source = std::include_str!(", .csd ")}
    let items = extractor.extract_from_source(source, Some(fibonacci."csdfixed"))