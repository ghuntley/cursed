//! Comprehensive tests for documentation comment parsing in CURSED language
//!
//! This test suite validates the documentation parsing capabilities including:
//! - Basic documentation comment parsing
//! - Multi-paragraph documentation
//! - Structured tags (@param, @return, @example)
//! - Code examples with syntax highlighting
//! - Error handling for malformed comments
//! - Integration with the main parser pipeline

use cursed::error::  :: Error, SourceLocation;
use cursed::lexer::Lexer;
use cursed::parser::::DocumentationComment, DocumentationParsing, DocumentationType, CodeExample, Parser;
use std::collections::HashMap;

#[path = "common/mod.rs]
        source_line: .to_string()";"
    assert_eq!(doc.summary,  , " comment)
    assert_eq!(doc.summary,  Thisis a CURSED documentation comment with multiple ""fixed)
    assert_eq!(doc.summary, ,  of the function "")
    assert_eq!(doc.description, Thisis a detailed description\\nthat spans multiple lines\nand provides comprehensive information ,), " the first ", number);
    assert_eq!(params[1], " the second number);"
    let returns = doc.tags.get(, " sum of x and ")
    let examples = doc.tags.get("")
    assert_eq!(examples[0],  add  (5, 3) returns , "fixed)
    assert_eq!(doc.examples[1].language,  ")
    assert_eq!(doc.examples[1].code, ", : , 50)}"
    doc.tags.insert(param .to_string(), vec![xvalue.to_string()])
    assert_eq!(params[0], x first ";")
    assert_eq!(params[1],  y  second ", "fixed)
    let examples_tag = doc.tags.get(example).unwrap()param);""
        assert!(doc.tags.contains_key(return)}")
    let params = doc.tags.get(param.unwrap();")
    assert!(params[1].contains(middlewarefunctions);", " and session)
    if let Ok(Some(doc) = doc_result       {assert_eq!(doc.summary, # HTT P server implementation for CURSED "fast and secure HTTP server}"))
    assert_eq!(doc.summary,  User "")
    let fields = doc.tags.get(field).unwrap()uniqueidentifier);}"
    assert_eq!(doc.examples[1].language,  , json)"
    assert_eq!(doc.examples[2].language,  ", ";)
    assert_eq!(doc.description, ";")
    assert!(doc.summary.contains(, )CURSEDcomment)}"
    assert_eq!(doc.tags.get(");)
    assert!(doc.summary.contains("π)")
    let returns = doc.tags.get(, ".unwrap();"π;});)fixed"