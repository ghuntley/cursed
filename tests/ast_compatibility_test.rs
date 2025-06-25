/// Test for AST compatibility 
use cursed::ast::identifiers::Identifier;

#[test]
fn test_identifier_creation() {
    let id = Identifier::new("test_name".to_string(), "test_value".to_string());
    assert_eq!(id.name, "test_name");
    assert_eq!(id.value, "test_value");
}

#[test]
fn test_identifier_equality() {
    let id1 = Identifier::new("name".to_string(), "value".to_string());
    let id2 = Identifier::new("name".to_string(), "value".to_string());
    let id3 = Identifier::new("other".to_string(), "value".to_string());
    
    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_identifier_clone() {
    let id1 = Identifier::new("test".to_string(), "value".to_string());
    let id2 = id1.clone();
    assert_eq!(id1, id2);
}
