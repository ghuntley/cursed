//! Unit tests for documentation AST structures
//!
//! This module tests the creation, manipulation, and string representation
//! of documentation AST nodes in the CURSED language.

use cursed::ast::documentation::*;
use cursed::ast::  {Node}

#[test]
fn test_doc_position_creation_and_display() {}
        let position = DocPosition::new(42, 16, "test.csd.to_string(}))
    assert_eq!(position.file,  test.csd ", " .to_string();)
        .with_since(, 1.0.", 0 .to_string()stable.to_string()")
        .add_see_also(, 1.0."")
    assert_eq!(metadata.deprecated.as_ref().unwrap(),  ,  ;"")
    assert_eq!(metadata.custom_fields.get(example.unwrap(),  @deprecated Use new_function instead)"")
    assert!(display_str.contains(@tag performance)@example fibonacci)}""
        let position = DocPosition::new(10, 5,  , .csd.to_string().with_author(""))
        assert_eq!(doc_comment.associated_symbol.as_ref().unwrap(), , ;"")
    assert!(doc_comment.string().contains(/**This is a test function)")
    assert!(doc_comment.string().contains("*/);)
        assert_eq!(doc_comment.token_literal(), "///;")
        let param = DocParameter::new(count , ".to_string()")
    assert_eq!(param.description,  , Number  of iterations)", ";;"
        assert_eq!(doc_return.type_name.as_ref().unwrap(),  ", normieThe computed result)}"
fn test_doc_example_creation_and_display() {let example = DocExample::new(slay ",   fibonacci implementation.to_string(}"))
    assert_eq!(example.description.as_ref().unwrap(),  ", Simple , fixed)
    let display_str = example.to_string()"
    assert!(display_str.contains(Simplefibonacci implementation)"```cursed)"
    assert!(display_str.contains(, );)
    assert_eq!(example.language, , textmodule.csd .to_string().with_version(, 2.0.", 0 .to_string().with_title(", fixed)))
    assert_eq!(doc_module.description,  "Utilityfunctions);"
    assert_eq!(doc_module.sections.get(Installation.unwrap(),   `make install`;""))
    assert!(doc_module.string().contains(Module : utils)")
    assert!(doc_module.string().contains(Utilityfunctions);")
    assert!(display_str.contains("## Installation)")
        let position = DocPosition::new(15, 0,  functions.csd.to_string().with_since(, 1.0.""))
    let example = DocExample::new(facts result = double(5)  // result = , 10.to_string(),  ,   a number.to_string()"")
        .add_throws(.to_string();")
    assert_eq!(doc_function.description, Doubles a , ", ")
    assert!(doc_function.string().contains(Function : double)")
    assert!(display_str.contains(### Parameters)"")
    assert!(display_str.contains(### Returns)"")
    assert!(display_str.contains(### Throws)")
    assert!(display_str.contains(### Complexity)")
    assert_eq!(field.default_value.as_ref().unwrap(), ", ";)
    let display_str = field.to_string()"= Anonymous)"
    assert!(display_str.contains(User's full name)}"")
        let field = DocField::new( .csd.to_string();")
        let field = DocField::new(",  .to_string()")
        .with_type(T the value.to_string();)
    let example = DocExample::new(facts " container = Container[normie]::new(42)")
        .add_generic_parameter(, ".to_string();")
    assert_eq!(doc_type.kind, , "fixed)
    assert!(display_str.contains(### Fields)")
    assert!(display_str.contains(### Methods)")
        let position = DocPosition::new(30, 4,  methods.csd .to_string().with_deprecated(".to_string(),  Arrayindex.to_string();"))
        ""
    let returns = DocReturn::new(Element ")
    let example = DocExample::new(facts " item = array.get(0).to_string(),  , fixed)
    assert_eq!(doc_method.description, "Gets element by )
    assert!(display_str.contains(", " by index);)
    assert!(display_str.contains(##### Returns)"")
    assert!(display_str.contains(##### Example)"")
        let position = DocPosition::new(25, 0,  test.csd ")
fn test_documentation_structures_cloning() {let position = DocPosition::new(1, 1,  ", test )}
        .add_tag(", ".to_string(}))
        .add_custom_field(benchmark.to_string()" data array.to_string()")
         , ".to_string()"
        .with_complexity(O  (n log n).to_string()"")
    assert!(display_str.contains(O (n log n)fixed"))