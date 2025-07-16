use cursed::lexer::{Lexer, TokenKind};
use cursed::parser_main::Parser;
use cursed::ast::{InterfaceStatement, Statement};

#[test]
fn test_interface_inheritance_colon_syntax() {
    let source = r#"
collab Writer {
    slay write(data tea) normie
}

collab ReadWriter : Writer {
    slay read() tea
}
"#.to_string();

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization should succeed");
    
    let mut parser = Parser::new(lexer).expect("Parser creation should succeed");
    let ast = parser.parse().expect("Parsing should succeed");
    
    // Find the ReadWriter interface
    let read_writer = ast.statements.iter()
        .find_map(|stmt| match stmt {
            Statement::Interface(interface) if interface.name == "ReadWriter" => Some(interface),
            _ => None
        })
        .expect("ReadWriter interface should be found");
    
    // Check that it extends Writer
    assert_eq!(read_writer.extends, vec!["Writer"]);
    assert_eq!(read_writer.methods.len(), 1);
    assert_eq!(read_writer.methods[0].name, "read");
}

#[test]
fn test_multiple_interface_inheritance() {
    let source = r#"
collab Writer {
    slay write(data tea) normie
}

collab Reader {
    slay read() tea
}

collab ReadWriter : Reader, Writer {
    slay size() normie
}
"#.to_string();

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization should succeed");
    
    let mut parser = Parser::new(lexer).expect("Parser creation should succeed");
    let ast = parser.parse().expect("Parsing should succeed");
    
    // Find the ReadWriter interface
    let read_writer = ast.statements.iter()
        .find_map(|stmt| match stmt {
            Statement::Interface(interface) if interface.name == "ReadWriter" => Some(interface),
            _ => None
        })
        .expect("ReadWriter interface should be found");
    
    // Check that it extends both Reader and Writer
    assert_eq!(read_writer.extends, vec!["Reader", "Writer"]);
    assert_eq!(read_writer.methods.len(), 1);
    assert_eq!(read_writer.methods[0].name, "size");
}

#[test]
fn test_generic_interface_inheritance() {
    let source = r#"
collab GenericWriter<T> {
    slay write_generic(data T) normie
}

collab GenericBuffered<T> : GenericWriter<T> {
    slay flush() normie
}
"#.to_string();

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization should succeed");
    
    let mut parser = Parser::new(lexer).expect("Parser creation should succeed");
    let ast = parser.parse().expect("Parsing should succeed");
    
    // Find the GenericBuffered interface
    let generic_buffered = ast.statements.iter()
        .find_map(|stmt| match stmt {
            Statement::Interface(interface) if interface.name == "GenericBuffered" => Some(interface),
            _ => None
        })
        .expect("GenericBuffered interface should be found");
    
    // Check that it extends GenericWriter<T>
    assert_eq!(generic_buffered.extends, vec!["GenericWriter<T>"]);
    assert_eq!(generic_buffered.type_parameters.len(), 1);
    assert_eq!(generic_buffered.type_parameters[0].name, "T");
    assert_eq!(generic_buffered.methods.len(), 1);
    assert_eq!(generic_buffered.methods[0].name, "flush");
}

#[test]
fn test_backward_compatibility_extends_keyword() {
    let source = r#"
collab Writer {
    slay write(data tea) normie
}

collab ReadWriter extends Writer {
    slay read() tea
}
"#.to_string();

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization should succeed");
    
    let mut parser = Parser::new(lexer).expect("Parser creation should succeed");
    let ast = parser.parse().expect("Parsing should succeed");
    
    // Find the ReadWriter interface
    let read_writer = ast.statements.iter()
        .find_map(|stmt| match stmt {
            Statement::Interface(interface) if interface.name == "ReadWriter" => Some(interface),
            _ => None
        })
        .expect("ReadWriter interface should be found");
    
    // Check that extends keyword still works
    assert_eq!(read_writer.extends, vec!["Writer"]);
    assert_eq!(read_writer.methods.len(), 1);
    assert_eq!(read_writer.methods[0].name, "read");
}

#[test]
fn test_nested_interface_inheritance() {
    let source = r#"
collab Writer {
    slay write(data tea) normie
}

collab ReadWriter : Writer {
    slay read() tea
}

collab BufferedReadWriter : ReadWriter {
    slay flush() normie
    slay buffer_size() normie
}
"#.to_string();

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenization should succeed");
    
    let mut parser = Parser::new(lexer).expect("Parser creation should succeed");
    let ast = parser.parse().expect("Parsing should succeed");
    
    // Find the BufferedReadWriter interface
    let buffered_read_writer = ast.statements.iter()
        .find_map(|stmt| match stmt {
            Statement::Interface(interface) if interface.name == "BufferedReadWriter" => Some(interface),
            _ => None
        })
        .expect("BufferedReadWriter interface should be found");
    
    // Check nested inheritance
    assert_eq!(buffered_read_writer.extends, vec!["ReadWriter"]);
    assert_eq!(buffered_read_writer.methods.len(), 2);
    assert_eq!(buffered_read_writer.methods[0].name, "flush");
    assert_eq!(buffered_read_writer.methods[1].name, "buffer_size");
}
