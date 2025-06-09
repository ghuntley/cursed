//! Unit tests for documentation AST structures
//!
//! This module tests the creation, manipulation, and string representation
//! of documentation AST nodes in the CURSED language.

use cursed::ast::documentation::*;
use cursed::ast::{Node};

#[test]
fn test_doc_position_creation_and_display() {
    let position = DocPosition::new(42, 16, "test.csd".to_string());
    
    assert_eq!(position.line, 42);
    assert_eq!(position.column, 16);
    assert_eq!(position.file, "test.csd");
    assert_eq!(position.to_string(), "test.csd:42:16");
}

#[test]
fn test_doc_metadata_creation_and_manipulation() {
    let metadata = DocMetadata::new()
        .with_author("Test Author".to_string())
        .with_version("1.0.0".to_string())
        .with_since("1.0.0".to_string())
        .with_deprecated("Use new_function instead".to_string())
        .with_stability("stable".to_string())
        .add_tag("performance".to_string())
        .add_tag("utility".to_string())
        .add_see_also("related_function".to_string())
        .add_custom_field("example".to_string(), "fibonacci".to_string());

    assert_eq!(metadata.author.as_ref().unwrap(), "Test Author");
    assert_eq!(metadata.version.as_ref().unwrap(), "1.0.0");
    assert_eq!(metadata.since.as_ref().unwrap(), "1.0.0");
    assert_eq!(metadata.deprecated.as_ref().unwrap(), "Use new_function instead");
    assert_eq!(metadata.stability.as_ref().unwrap(), "stable");
    assert!(metadata.is_deprecated());
    assert_eq!(metadata.tags.len(), 2);
    assert_eq!(metadata.see_also.len(), 1);
    assert_eq!(metadata.custom_fields.get("example").unwrap(), "fibonacci");

    // Test display format
    let display_str = metadata.to_string();
    assert!(display_str.contains("@author Test Author"));
    assert!(display_str.contains("@version 1.0.0"));
    assert!(display_str.contains("@deprecated Use new_function instead"));
    assert!(display_str.contains("@tag performance"));
    assert!(display_str.contains("@example fibonacci"));
}

#[test]
fn test_doc_comment_creation_and_node_interface() {
    let position = DocPosition::new(10, 5, "example.csd".to_string());
    let metadata = DocMetadata::new().with_author("Developer".to_string());
    
    let doc_comment = DocComment::new("This is a test function".to_string(), position)
        .with_symbol("test_function".to_string())
        .with_metadata(metadata)
        .multiline();

    assert_eq!(doc_comment.content, "This is a test function");
    assert_eq!(doc_comment.associated_symbol.as_ref().unwrap(), "test_function");
    assert!(doc_comment.is_multiline);
    assert_eq!(doc_comment.metadata.author.as_ref().unwrap(), "Developer");

    // Test Node interface
    assert_eq!(doc_comment.token_literal(), "/**");
    assert!(doc_comment.string().contains("/**"));
    assert!(doc_comment.string().contains("This is a test function"));
    assert!(doc_comment.string().contains("*/"));

    // Test summary generation
    let summary = doc_comment.get_summary();
    assert_eq!(summary, "This is a test function");
}

#[test]
fn test_doc_comment_single_line() {
    let position = DocPosition::new(5, 0, "test.csd".to_string());
    let doc_comment = DocComment::new("Single line comment".to_string(), position);

    assert!(!doc_comment.is_multiline);
    assert_eq!(doc_comment.token_literal(), "///");
    assert_eq!(doc_comment.string(), "/// Single line comment");
}

#[test]
fn test_doc_parameter_creation_and_display() {
    let param = DocParameter::new("count".to_string(), "Number of iterations".to_string())
        .with_type("normie".to_string())
        .optional()
        .with_default("10".to_string());

    assert_eq!(param.name, "count");
    assert_eq!(param.type_name.as_ref().unwrap(), "normie");
    assert_eq!(param.description, "Number of iterations");
    assert!(param.is_optional);
    assert_eq!(param.default_value.as_ref().unwrap(), "10");

    let display_str = param.to_string();
    assert!(display_str.contains("count"));
    assert!(display_str.contains("normie"));
    assert!(display_str.contains("optional"));
    assert!(display_str.contains("= 10"));
    assert!(display_str.contains("Number of iterations"));
}

#[test]
fn test_doc_return_creation_and_display() {
    let doc_return = DocReturn::new("The computed result".to_string())
        .with_type("normie".to_string())
        .add_example("42".to_string())
        .add_example("100".to_string());

    assert_eq!(doc_return.description, "The computed result");
    assert_eq!(doc_return.type_name.as_ref().unwrap(), "normie");
    assert_eq!(doc_return.examples.len(), 2);
    assert_eq!(doc_return.examples[0], "42");

    let display_str = doc_return.to_string();
    assert!(display_str.contains("normie"));
    assert!(display_str.contains("The computed result"));
}

#[test]
fn test_doc_example_creation_and_display() {
    let example = DocExample::new("slay fibonacci(n normie) normie { /* ... */ }".to_string())
        .with_title("Basic Usage".to_string())
        .with_description("Simple fibonacci implementation".to_string())
        .with_language("cursed".to_string());

    assert_eq!(example.title.as_ref().unwrap(), "Basic Usage");
    assert_eq!(example.description.as_ref().unwrap(), "Simple fibonacci implementation");
    assert_eq!(example.language, "cursed");
    assert!(example.is_runnable);

    let display_str = example.to_string();
    assert!(display_str.contains("# Basic Usage"));
    assert!(display_str.contains("Simple fibonacci implementation"));
    assert!(display_str.contains("```cursed"));
    assert!(display_str.contains("slay fibonacci"));
}

#[test]
fn test_doc_example_not_runnable() {
    let example = DocExample::new("pseudo code".to_string())
        .with_language("text".to_string())
        .not_runnable();

    assert!(!example.is_runnable);
    assert_eq!(example.language, "text");
}

#[test]
fn test_doc_module_creation_and_node_interface() {
    let position = DocPosition::new(1, 1, "module.csd".to_string());
    let metadata = DocMetadata::new().with_version("2.0.0".to_string());
    let example = DocExample::new("facts x = 42".to_string()).with_title("Example".to_string());

    let doc_module = DocModule::new("utils".to_string(), "Utility functions".to_string(), position)
        .with_metadata(metadata)
        .add_example(example)
        .add_section("Installation".to_string(), "Run `make install`".to_string());

    assert_eq!(doc_module.name, "utils");
    assert_eq!(doc_module.description, "Utility functions");
    assert_eq!(doc_module.examples.len(), 1);
    assert_eq!(doc_module.sections.len(), 1);
    assert_eq!(doc_module.sections.get("Installation").unwrap(), "Run `make install`");

    // Test Node interface
    assert_eq!(doc_module.token_literal(), "module");
    assert!(doc_module.string().contains("Module: utils"));
    assert!(doc_module.string().contains("Utility functions"));

    // Test display
    let display_str = doc_module.to_string();
    assert!(display_str.contains("# Module: utils"));
    assert!(display_str.contains("Utility functions"));
    assert!(display_str.contains("## Installation"));
    assert!(display_str.contains("# Example"));
}

#[test]
fn test_doc_function_creation_and_node_interface() {
    let position = DocPosition::new(15, 0, "functions.csd".to_string());
    let metadata = DocMetadata::new().with_since("1.0.0".to_string());
    
    let param = DocParameter::new("x".to_string(), "Input value".to_string())
        .with_type("normie".to_string());
    let returns = DocReturn::new("Doubled value".to_string())
        .with_type("normie".to_string());
    let example = DocExample::new("facts result = double(5)  // result = 10".to_string());

    let doc_function = DocFunction::new("double".to_string(), "Doubles a number".to_string(), position)
        .with_metadata(metadata)
        .add_parameter(param)
        .with_returns(returns)
        .add_example(example)
        .add_throws("InvalidInputError".to_string())
        .with_complexity("O(1)".to_string());

    assert_eq!(doc_function.name, "double");
    assert_eq!(doc_function.description, "Doubles a number");
    assert_eq!(doc_function.parameters.len(), 1);
    assert!(doc_function.returns.is_some());
    assert_eq!(doc_function.examples.len(), 1);
    assert_eq!(doc_function.throws.len(), 1);
    assert_eq!(doc_function.complexity.as_ref().unwrap(), "O(1)");

    // Test Node interface
    assert_eq!(doc_function.token_literal(), "function");
    assert!(doc_function.string().contains("Function: double"));

    // Test display
    let display_str = doc_function.to_string();
    assert!(display_str.contains("## Function: double"));
    assert!(display_str.contains("Doubles a number"));
    assert!(display_str.contains("### Parameters"));
    assert!(display_str.contains("### Returns"));
    assert!(display_str.contains("### Throws"));
    assert!(display_str.contains("### Complexity"));
    assert!(display_str.contains("### Example"));
}

#[test]
fn test_doc_field_creation_and_display() {
    let field = DocField::new("name".to_string(), "User's full name".to_string())
        .with_type("normie".to_string())
        .optional()
        .with_default("Anonymous".to_string());

    assert_eq!(field.name, "name");
    assert_eq!(field.type_name.as_ref().unwrap(), "normie");
    assert_eq!(field.description, "User's full name");
    assert!(field.is_public);
    assert!(field.is_optional);
    assert_eq!(field.default_value.as_ref().unwrap(), "Anonymous");

    let display_str = field.to_string();
    assert!(display_str.contains("name: normie"));
    assert!(display_str.contains("optional"));
    assert!(display_str.contains("= Anonymous"));
    assert!(display_str.contains("User's full name"));
}

#[test]
fn test_doc_field_private() {
    let field = DocField::new("secret".to_string(), "Internal field".to_string())
        .private();

    assert!(!field.is_public);
}

#[test]
fn test_doc_type_creation_and_node_interface() {
    let position = DocPosition::new(20, 0, "types.csd".to_string());
    let metadata = DocMetadata::new().with_stability("experimental".to_string());
    
    let field = DocField::new("value".to_string(), "The stored value".to_string())
        .with_type("T".to_string());
    let method = DocMethod::new("get".to_string(), "Gets the value".to_string(), position.clone());
    let example = DocExample::new("facts container = Container[normie]::new(42)".to_string());

    let doc_type = DocType::new("Container".to_string(), "struct".to_string(), "Generic container".to_string(), position)
        .with_metadata(metadata)
        .add_field(field)
        .add_method(method)
        .add_example(example)
        .add_generic_parameter("T".to_string());

    assert_eq!(doc_type.name, "Container");
    assert_eq!(doc_type.kind, "struct");
    assert_eq!(doc_type.description, "Generic container");
    assert_eq!(doc_type.fields.len(), 1);
    assert_eq!(doc_type.methods.len(), 1);
    assert_eq!(doc_type.examples.len(), 1);
    assert_eq!(doc_type.generic_parameters.len(), 1);
    assert_eq!(doc_type.generic_parameters[0], "T");

    // Test Node interface
    assert_eq!(doc_type.token_literal(), "struct");
    assert!(doc_type.string().contains("struct: Container"));

    // Test display
    let display_str = doc_type.to_string();
    assert!(display_str.contains("## struct Container<T>"));
    assert!(display_str.contains("Generic container"));
    assert!(display_str.contains("### Fields"));
    assert!(display_str.contains("### Methods"));
    assert!(display_str.contains("### Example"));
}

#[test]
fn test_doc_method_creation_and_node_interface() {
    let position = DocPosition::new(30, 4, "methods.csd".to_string());
    let metadata = DocMetadata::new().with_deprecated("Use new_method instead".to_string());
    
    let param = DocParameter::new("index".to_string(), "Array index".to_string())
        .with_type("normie".to_string());
    let returns = DocReturn::new("Element at index".to_string())
        .with_type("T".to_string());
    let example = DocExample::new("facts item = array.get(0)".to_string());

    let doc_method = DocMethod::new("get".to_string(), "Gets element by index".to_string(), position)
        .with_metadata(metadata)
        .add_parameter(param)
        .with_returns(returns)
        .add_example(example)
        .add_throws("IndexOutOfBoundsError".to_string())
        .static_method()
        .with_visibility("private".to_string());

    assert_eq!(doc_method.name, "get");
    assert_eq!(doc_method.description, "Gets element by index");
    assert_eq!(doc_method.parameters.len(), 1);
    assert!(doc_method.returns.is_some());
    assert_eq!(doc_method.examples.len(), 1);
    assert_eq!(doc_method.throws.len(), 1);
    assert!(doc_method.is_static);
    assert_eq!(doc_method.visibility, "private");

    // Test Node interface
    assert_eq!(doc_method.token_literal(), "method");
    assert!(doc_method.string().contains("Method: get"));

    // Test display
    let display_str = doc_method.to_string();
    assert!(display_str.contains("#### static private get"));
    assert!(display_str.contains("Gets element by index"));
    assert!(display_str.contains("##### Parameters"));
    assert!(display_str.contains("##### Returns"));
    assert!(display_str.contains("##### Throws"));
    assert!(display_str.contains("##### Example"));
}

#[test]
fn test_doc_method_non_static_public() {
    let position = DocPosition::new(25, 0, "test.csd".to_string());
    let doc_method = DocMethod::new("process".to_string(), "Processes data".to_string(), position);

    assert!(!doc_method.is_static);
    assert_eq!(doc_method.visibility, "public");
}

#[test]
fn test_documentation_metadata_empty() {
    let metadata = DocMetadata::new();
    
    assert!(metadata.author.is_none());
    assert!(metadata.version.is_none());
    assert!(metadata.since.is_none());
    assert!(metadata.deprecated.is_none());
    assert!(metadata.stability.is_none());
    assert!(metadata.tags.is_empty());
    assert!(metadata.see_also.is_empty());
    assert!(metadata.custom_fields.is_empty());
    assert!(!metadata.is_deprecated());
}

#[test]
fn test_documentation_structures_cloning() {
    let position = DocPosition::new(1, 1, "test.csd".to_string());
    let metadata = DocMetadata::new().with_author("Test".to_string());
    
    // Test cloning
    let position_clone = position.clone();
    assert_eq!(position.line, position_clone.line);
    assert_eq!(position.file, position_clone.file);
    
    let metadata_clone = metadata.clone();
    assert_eq!(metadata.author, metadata_clone.author);
    
    let doc_comment = DocComment::new("Test".to_string(), position).with_metadata(metadata);
    let doc_comment_clone = doc_comment.clone();
    assert_eq!(doc_comment.content, doc_comment_clone.content);
    assert_eq!(doc_comment.associated_symbol, doc_comment_clone.associated_symbol);
}

#[test]
fn test_complex_documentation_structure() {
    // Test creating a complex documentation structure with all elements
    let position = DocPosition::new(100, 1, "complex.csd".to_string());
    let metadata = DocMetadata::new()
        .with_author("Complex Author".to_string())
        .with_version("3.0.0".to_string())
        .with_since("2.0.0".to_string())
        .add_tag("performance".to_string())
        .add_tag("algorithm".to_string())
        .add_see_also("related_function".to_string())
        .add_custom_field("benchmark".to_string(), "O(n log n)".to_string());

    let param1 = DocParameter::new("data".to_string(), "Input data array".to_string())
        .with_type("Array[normie]".to_string());
    let param2 = DocParameter::new("compare".to_string(), "Comparison function".to_string())
        .with_type("Function".to_string())
        .optional()
        .with_default("default_compare".to_string());
    
    let returns = DocReturn::new("Sorted array".to_string())
        .with_type("Array[normie]".to_string())
        .add_example("[1, 2, 3, 4, 5]".to_string());
    
    let example = DocExample::new(
        "facts sorted = sort([5, 2, 8, 1], ascending_compare)\n// sorted = [1, 2, 5, 8]".to_string()
    ).with_title("Basic sorting".to_string())
     .with_description("Sort numbers in ascending order".to_string());

    let doc_function = DocFunction::new("sort".to_string(), "Advanced sorting algorithm".to_string(), position)
        .with_metadata(metadata)
        .add_parameter(param1)
        .add_parameter(param2)
        .with_returns(returns)
        .add_example(example)
        .add_throws("EmptyArrayError".to_string())
        .add_throws("InvalidComparatorError".to_string())
        .with_complexity("O(n log n)".to_string());

    // Verify the complete structure
    assert_eq!(doc_function.name, "sort");
    assert_eq!(doc_function.parameters.len(), 2);
    assert_eq!(doc_function.throws.len(), 2);
    assert!(doc_function.returns.is_some());
    assert_eq!(doc_function.examples.len(), 1);
    assert_eq!(doc_function.metadata.tags.len(), 2);
    assert_eq!(doc_function.metadata.custom_fields.len(), 1);

    // Test that the display contains all expected elements
    let display_str = doc_function.to_string();
    assert!(display_str.contains("sort"));
    assert!(display_str.contains("Advanced sorting algorithm"));
    assert!(display_str.contains("data Array[normie]"));
    assert!(display_str.contains("compare Function"));
    assert!(display_str.contains("optional"));
    assert!(display_str.contains("Sorted array"));
    assert!(display_str.contains("EmptyArrayError"));
    assert!(display_str.contains("InvalidComparatorError"));
    assert!(display_str.contains("O(n log n)"));
    assert!(display_str.contains("Basic sorting"));
}
