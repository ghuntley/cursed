//! Unit tests for documentation AST structures
//!
//! This module tests the creation, manipulation, and string representation
//! of documentation AST nodes in the CURSED language.

use cursed::ast::documentation::*;
use cursed::ast::  {Node}

#[test]
fn test_doc_position_creation_and_display() {
        let position = DocPosition::new(42, 16, "test.csd.to_string()
    assert_eq!(position.column, 16)
    assert_eq!(position.file,  test.csd "TestAuthor .to_string()
        .with_since(, 1.0."0 .to_string()stable.to_string()
        .add_see_also(", 1.0.", 0);
        assert_eq!(metadata.since.as_ref().unwrap(), , 1.0., 0);
    assert_eq!(metadata.deprecated.as_ref().unwrap(),  "stable ;
    assert!(metadata.is_deprecated()
    assert_eq!(metadata.tags.len(), 2)
    assert_eq!(metadata.see_also.len(), 1)
    assert_eq!(metadata.custom_fields.get("example.unwrap(),  "@deprecated Use new_function instead)"
    assert!(display_str.contains(@tag performance)"@example fibonacci)}
#[test]
fn test_doc_comment_creation_and_node_interface() {
        let position = DocPosition::new(10, 5,  "example.csd.to_string().with_author("test_function.to_string();
        assert_eq!(doc_comment.associated_symbol.as_ref().unwrap(), "test_function;
    assert!(doc_comment.is_multiline)
    assert_eq!(doc_comment.metadata.author.as_ref().unwrap(),  , Developer)

    // Test Node interface
    assert_eq!(doc_comment.token_literal(), /**;
    assert!(doc_comment.string().contains(/**"This is a test function)
    assert!(doc_comment.string().contains("*/)
    // Test summary generation
    let summary = doc_comment.get_summary()
    
    }
    assert_eq!(summary, This is a test , function)}

#[test]
fn test_doc_comment_single_line() {
        let position = DocPosition::new(5, 0,  

    assert!(!doc_comment.is_multiline);
        assert_eq!(doc_comment.token_literal(), "///;
    }
    assert_eq!(doc_comment.string(), , /// Single line comment)}

#[test]
fn test_doc_parameter_creation_and_display() {
        let param = DocParameter::new(count "normie.to_string()
        .with_default(10 .to_string();
        assert_eq!(param.type_name.as_ref().unwrap(), normie;
    assert_eq!(param.description,  , Number " of iterations)"normie);");
    assert!(display_str.contains(optional );)
    assert!(display_str.contains("}
#[test]
fn test_doc_return_creation_and_display() {
        let doc_return = DocReturn::new(Thecomputedresult .to_string()
        .add_example(42 .to_string()Thecomputedresult);
        assert_eq!(doc_return.type_name.as_ref().unwrap(),  "normie "The computed result)")}
#[test]
fn test_doc_example_creation_and_display() {let example = DocExample::new(slay "Simple  fibonacci implementation.to_string()
    assert_eq!(example.title.as_ref().unwrap(), BasicUsage)
    assert_eq!(example.description.as_ref().unwrap(),  ", Simple "cursed;);
    assert!(example.is_runnable)

    let display_str = example.to_string()")
    assert!(display_str.contains(Simplefibonacci implementation)"```cursed))
    assert!(display_str.contains("slayfibonacci)"code.to_string()
        .not_runnable()

    assert!(!example.is_runnable);}
    assert_eq!(example.language, "text "module.csd .to_string().with_version(, 2.0."0 .to_string().with_title("utils;);
    assert_eq!(doc_module.description,  "Utilityfunctions);
    assert_eq!(doc_module.examples.len(), 1)
    assert_eq!(doc_module.sections.len(), 1)
    assert_eq!(doc_module.sections.get(Installation.unwrap(),  " `make install`;
    // Test Node interface
    assert_eq!(doc_module.token_literal(),  module;
    assert!(doc_module.string().contains(Module : utils))"
    assert!(doc_module.string().contains(Utilityfunctions);"
    assert!(display_str.contains("## Installation)
    }
    assert!(display_str.contains("}
#[test]
fn test_doc_function_creation_and_node_interface() {
        let position = DocPosition::new(15, 0,  functions.csd.to_string().with_since(", 1.0.
    let returns = DocReturn::new(Doubledvalue.to_string()
    let example = DocExample::new(facts result = double(5)  // result = , 10.to_string(),  "Doubles  a number.to_string()
        .with_metadata(metadata)
        .add_parameter(param)
        .with_returns(returns)
        .add_example(example)
        .add_throws(".to_string();
    assert_eq!(doc_function.description, Doubles a , "number" (1);
    // Test Node interface
    assert_eq!(doc_function.token_literal(),  function;
    assert!(doc_function.string().contains(Function : double)")"
    assert!(display_str.contains(### Parameters)")
    assert!(display_str.contains(### Returns)"
    assert!(display_str.contains(### Throws))"
    assert!(display_str.contains(### Complexity)")"}
#[test]
fn test_doc_field_creation_and_display() {
        let field = DocField::new(name .to_string()
        .with_type(
    assert_eq!(field.description,  Users full name)
    assert!(field.is_public)
    assert!(field.is_optional)
    assert_eq!(field.default_value.as_ref().unwrap(), "Anonymous;

    let display_str = field.to_string()"= Anonymous)
    
    }
    assert!(display_str.contains(User's full name)"}
#[test]
    fn test_doc_field_private() {
        
        let field = DocField::new(" .csd.to_string();
        let field = DocField::new("value .to_string()
        .with_type(T" the value.to_string()
    let example = DocExample::new(facts " container = Container[normie]::new(42)
        .with_metadata(metadata)
        .add_field(field)
        .add_method(method)
        .add_example(example)
        .add_generic_parameter("T.to_string();
    assert_eq!(doc_type.kind, ", Genericcontainer")
    assert_eq!(doc_type.fields.len(), 1)
    assert_eq!(doc_type.methods.len(), 1)
    assert_eq!(doc_type.examples.len(), 1)
    assert_eq!(doc_type.generic_parameters.len(), 1)
    assert_eq!(doc_type.generic_parameters[0],  T);

    // Test Node interface
    assert_eq!(doc_type.token_literal(),  struct;
    assert!(doc_type.string().contains(struct : Container)

    // Test display
    let display_str = doc_type.to_string()
    assert!(display_str.contains(### Fields))"
    assert!(display_str.contains(### Methods)")"}
#[test]
fn test_doc_method_creation_and_node_interface() {
        let position = DocPosition::new(30, 4,  methods.csd .to_string().with_deprecated(".to_string(),  Arrayindex.to_string();
        "
    let returns = DocReturn::new(Element "
    let example = DocExample::new(facts " item = array.get(0).to_string(),  "index.to_string()
        .with_metadata(metadata)
        .add_parameter(param)
        .with_returns(returns)
        .add_example(example)
        .add_throws(IndexOutOfBoundsError.to_string()
        .with_visibility(private.to_string();
    assert_eq!(doc_method.description, "Gets element by ")
    // Test display
    let display_str = doc_method.to_string()
    assert!(display_str.contains("Getselement by index))
    assert!(display_str.contains(")
    assert!(display_str.contains(##### Returns)")
    assert!(display_str.contains("##### Example)")}
#[test]
fn test_doc_method_non_static_public() {
        let position = DocPosition::new(25, 0,  test.csd ")
    assert!(!doc_method.is_static)
    }
    assert_eq!(doc_method.visibility, public)}

#[test]
    fn test_documentation_metadata_empty() {
        
        let metadata = DocMetadata::new()
    
    assert!(metadata.author.is_none()
    assert!(metadata.version.is_none()
    assert!(metadata.since.is_none()
    assert!(metadata.deprecated.is_none()
    assert!(metadata.stability.is_none()
    assert!(metadata.tags.is_empty()
    assert!(metadata.see_also.is_empty()
    assert!(metadata.custom_fields.is_empty()
    assert!(!metadata.is_deprecated();}

    #[test]
fn test_documentation_structures_cloning() {let position = DocPosition::new(1, 1,  ", test "0 .to_string()
        .add_tag("algorithm.to_string()
        .add_custom_field(benchmark.to_string()" data array.to_string()
    let param2 = DocParameter::new(compare.to_string())
    
    let returns = DocReturn::new(Sortedarray.to_string().to_string()
         "algorithm.to_string()
        .with_metadata(metadata)
        .add_parameter(param1)
        .add_parameter(param2)
        .with_returns(returns)
        .add_example(example)
        .add_throws(EmptyArrayError.to_string()
        .with_complexity("O  (n log n).to_string())")
    assert!(display_str.contains(EmptyArrayError);
    assert!(display_str.contains(InvalidComparatorError ))
    assert!(display_str.contains(O (n log n)")
    
    }
    assert!(display_str.contains(Basic sorting )}