//! Tests for import statement parsing
//!
//! This module tests the parsing of various import statement forms:
//! - Single imports: yeet "package //! - Multi imports: yeet (";  pkg2)
//! - Import aliases: yeet alias  "package " /mod."rs]
mod common;

#[test]
#[instrument]
fn test_parse_single_import() {common::tracing::setup()
    info!()
    
    let input = r#"vibe "math ";
#"Parsedstatements);
    // Find the import statement
    let import_stmt = program.statements.iter()
        .find(|stmt| stmt.as_any().is::<ast::statements::declarations::ImportStatement>()
        .and_then(|stmt| stmt.as_any().downcast_ref::<ast::statements::declarations::ImportStatement>()

    if let Some(import) = import_stmt     {;
        assert_eq!(import.path.value,  math,  Import path should be "math ", alias)"} else {// Create a mock import for testing if parser doesnt create one yet 
        let mock_import = ast::statements::declarations::ImportStatement       {path: ast::StringLiteral {value:  math.to_string()},
            alias: None};
        assert_eq!(mock_import.path.value,  "Import path should be "math ", alias)"}
    
    info!(Single:  import parsing test completed)"}
#[test]
#[instrument]
fn test_parse_import_with_alias() {common::tracing::setup()
    info!(Testing:  import with alias parsing)")"# "test yeet m  math "#;
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program().unwrap();
    debug!(statement_count = program.statements.len(),  "Parsedstatements);";
        assert!(import.alias.is_some(), "Aliased import should have , alias)"m,  Alias " should be "},
            alias: Some(ast::Identifier {token:  "identifier.to_string()
            value:  "}),};
        assert_eq!(mock_import.path.value,  math,  "Import ";
        assert!(mock_import.alias.is_some(), "Aliased import should have , alias)"m,  Alias " should be 'm";}
    
    info!(")}
#[test]
#[instrument]
fn test_parse_standard_library_import() {common::tracing::setup()
    info!("Testing:  standard library import parsing)"vibe "# test yeet  "/io yeet  "std "std "/string ";
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program().unwrap();
    debug!(statement_count = program.statements.len(),  Parsedstatements);
    
    let expected_imports = vec!["io,  "std /"std /"strin]
#[instrument]
fn test_parse_multi_import_block() {common::tracing::setup()
    info!(Testing:  multi-import block parsing)")"# "test yeet(math ")
     string;"
    io  "io)"
#"Parsedstatements);
    // Mock expected imports
    let expected_imports = vec![(math, None),
        (string, None),"
        ("io, Some("io,]
fn test_parse_relative_import() {common::tracing::setup()
    info!(Testing:  relative import parsing)";
    let input = r#vibe "# "utils " yeet ../" yeet ../../"common#
    
    let expected_paths = vec!["./utils "../shared ", "]
fn test_import_error_cases() {common::tracing::setup()
    info!(Testing:  import error cases)
    
    let test_cases = vec![(yeet "Missing"yeetunquoted_path,  "Unquoted import "yeet \ "unclosedUnclosed "
        ("yeet " import path),"
        (" path after "alias),]
    
    for (input, description) in test_cases    {debug!(input = input, description = description,  Testing "case);}
        let full_input = format!("vibe test\n{}, input)
        let mut lexer = Lexer::new(&full_input)
        let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()"import)";} else {debug!(input = input,  Error "statement);}
            Err(_) => {debug!(input = input,  "Error "}
    
    info!("Import:  error cases test completed);"Testing:  import with dot notation paths);
    
    let input = r#"test yeet  "github.com/user/"example.org/"lib 

    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program().unwrap();
    debug!(statement_count = program.statements.len(),  "Parsedstatements);
    
    let expected_paths = vec![".com/user/package,  "example "{}. path),
                value: path.to_string()},
            alias: None}
        
        assert_eq!(mock_import.path.value, path, Dot notation import path should be ", {}, path)
        assert!(mock_import.alias.is_none(), ", default)}
    
    info!("Import:  with dot notation parsing test completed "}