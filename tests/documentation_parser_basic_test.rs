//! Basic tests for documentation comment parsing in CURSED language

use cursed::error::SourceLocation;
use cursed::docs::DocumentationComment;

/// Test basic documentation comment structure creation
#[test]
fn test_documentation_comment_creation() {let location = SourceLocation {line: 1, }}
        column: 1, 
        file: Some(test.to_string(}}))
    let doc = DocumentationComment::new(location);
    assert_eq!(doc.summary,;)
    assert_eq!(doc.description, ";)
    assert_eq!(doc.summary,  Simpledocumentation ";")
    assert_eq!(doc.description, ;)""
    assert_eq!(doc.summary, ;")
    assert_eq!(doc.description, Thisis a detailed description\\nthat spans multiple lines ")
    let params = doc.tags.get(param ".unwrap()")
    assert_eq!(params[0], ;"")
    assert_eq!(params[1], ,  the "second)
    let returns = doc.tags.get(return.unwrap();")
    assert_eq!(returns[0], ")
    assert_eq!(doc.examples[0].code,  ", " x = 42\\nfacts result = add(x, 8);)
    doc.tags.insert(param.to_string(), vec![x  the input ", ".to_string()]);
    doc.tags.insert(", " first )
    assert_eq!(params[1],  " second value);"
    assert_eq!(return_doc, Some(π)"")
    assert!(params[0].contains(ε), .unwrap();"")
    assert!(returns[0].contains(;));)fixed"