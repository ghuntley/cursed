//! Basic tests for documentation comment parsing in CURSED language

use cursed::error::SourceLocation;
use cursed::docs::DocumentationComment;

/// Test basic documentation comment structure creation
#[test]
fn test_documentation_comment_creation() {
    let location = SourceLocation {
        line: 1, 
        column: 1, 
        file: Some("test".to_string()),
    };
    let doc = DocumentationComment::new(location);
    
    assert_eq!(doc.summary, "");
    assert_eq!(doc.description, "");
    assert!(doc.tags.is_empty());
    assert!(doc.examples.is_empty());
    assert_eq!(doc.location.line, 1);
    assert_eq!(doc.location.column, 1);
}

/// Test parsing simple single-line documentation
#[test]
fn test_parse_simple_documentation() {
    let location = SourceLocation { 
        line: 1, 
        column: 1, 
        file: Some("test".to_string()),
    };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "/// Simple documentation comment".to_string();
    
    doc.parse_content();
    
    assert_eq!(doc.summary, "Simple documentation comment");
    assert_eq!(doc.description, "");
}

/// Test parsing documentation with summary and description
#[test]
fn test_parse_summary_and_description() {
    let location = SourceLocation { 
        line: 1, 
        column: 1, 
        file: Some("test".to_string()),
    };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "/// Brief summary of the function\n///\n/// This is a detailed description\n/// that spans multiple lines".to_string();
    
    doc.parse_content();
    
    assert_eq!(doc.summary, "Brief summary of the function");
    assert_eq!(doc.description, "This is a detailed description\nthat spans multiple lines");
}

/// Test parsing structured tags (@param, @return, etc.)
#[test]
fn test_parse_structured_tags() {
    let location = SourceLocation { 
        line: 1, 
        column: 1, 
        file: Some("test".to_string()),
    };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "/// Calculate the sum of two numbers\n/// @param x the first number\n/// @param y the second number\n/// @return the sum of x and y".to_string();
    
    doc.parse_tags();
    
    let params = doc.tags.get("param").unwrap();
    assert_eq!(params.len(), 2);
    assert_eq!(params[0], "x the first number");
    assert_eq!(params[1], "y the second number");
    
    let returns = doc.tags.get("return").unwrap();
    assert_eq!(returns[0], "the sum of x and y");
}

/// Test parsing code examples with fenced code blocks
#[test]
fn test_parse_code_examples() {
    let location = SourceLocation { 
        line: 1, 
        column: 1, 
        file: Some("test".to_string()),
    };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "/// Example usage:\n/// ```cursed\n/// sus x = 42\n/// facts result = add(x, 8)\n/// ```".to_string();
    
    doc.parse_examples();
    
    assert_eq!(doc.examples.len(), 1);
    assert_eq!(doc.examples[0].language, "cursed");
    assert_eq!(doc.examples[0].code, "sus x = 42\nfacts result = add(x, 8)");
}

/// Test documentation validation functionality
#[test]
fn test_documentation_validation() {
    let location = SourceLocation { 
        line: 1, 
        column: 1, 
        file: Some("test".to_string()),
    };
    let mut doc = DocumentationComment::new(location);
    
    // Valid documentation
    doc.tags.insert("param".to_string(), vec!["x the input value".to_string()]);
    doc.tags.insert("return".to_string(), vec!["the processed result".to_string()]);
    
    assert!(doc.validate().is_ok());
    
    // Test validation with empty param
    doc.tags.insert("param".to_string(), vec!["".to_string()]);
    let result = doc.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains(&"Empty @param tag found".to_string()));
}

/// Test documentation helper methods
#[test]
fn test_documentation_helper_methods() {
    let location = SourceLocation { 
        line: 1, 
        column: 1, 
        file: Some("test".to_string()),
    };
    let mut doc = DocumentationComment::new(location);
    
    doc.tags.insert("param".to_string(), vec!["x first value".to_string(), "y second value".to_string()]);
    doc.tags.insert("return".to_string(), vec!["computed result".to_string()]);
    
    let params = doc.get_parameters();
    assert_eq!(params.len(), 2);
    assert_eq!(params[0], "x first value");
    assert_eq!(params[1], "y second value");
    
    let return_doc = doc.get_return_documentation();
    assert_eq!(return_doc, Some("computed result".to_string()));
    
    let examples = doc.get_examples();
    assert_eq!(examples.len(), 0);
}

/// Test unicode support in documentation
#[test] 
fn test_documentation_with_unicode() {
    let location = SourceLocation { 
        line: 1, 
        column: 1, 
        file: Some("test".to_string()),
    };
    let mut doc = DocumentationComment::new(location);
    doc.raw_content = "/// Calculate π approximation using 🌟 method\n/// @param ε precision tolerance (ε > 0)\n/// @return approximated π value".to_string();
    
    doc.parse_content();
    doc.parse_tags();
    
    assert!(doc.summary.contains("π"));
    assert!(doc.summary.contains("🌟"));
    
    let params = doc.tags.get("param").unwrap();
    assert!(params[0].contains("ε"));
    
    let returns = doc.tags.get("return").unwrap();
    assert!(returns[0].contains("π"));
}
