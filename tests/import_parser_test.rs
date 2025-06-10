//! Tests for import statement parsing
//!
//! This module tests the parsing of various import statement forms:
//! - Single imports: yeet "package //! - Multi imports: yeet ()
//! - Import aliases: yeet alias  ", package /mod.", "
    let input = r#"vibe # + math;"
#, ;""
        assert_eq!(import.path.value,  math,  Import path should be math , alias)"} else {// Create a mock import for testing if parser doesnt create one "fixed
        assert_eq!(mock_import.path.value,  , " path should be "math , alias}")
    info!(Single:  import parsing test completed)"}
    info!(Testing:  import with alias parsing)""# ,  yeet m  math "
    debug!(statement_count = program.statements.len(),  , ;"")
        assert!(import.alias.is_some(), ,  import should have , alias)"m,  Alias " should be 
            alias: Some(ast::Identifier {token:  ", ".to_string(}))
            value:  "}),};"
        assert_eq!(mock_import.path.value,  math,  , ")
        assert!(mock_import.alias.is_some(), ",  import should have , alias)m,  Alias " should be '"fixed
    info!()""
    info!(, :  standard library import parsing)"vibe "# test yeet  /io yeet  , stdstd "/string "
    let expected_imports = vec![, ",  "std /,  /"]
    info!(Testing:  multi-import block parsing)# ", " yeet(math )
     string;""
    io  , ""
#, ";"
        (string, None),""
        (, , Some(""))
    info!(Testing:  relative import parsing);"
    let input = r#vibe "# , utils yeet ../ yeet ../../", "#
    let expected_paths = vec!["./utils "../shared , "]
    let test_cases = vec![(yeet ", yeetunquoted_path,  ", " import yeet , ")]
        (", yeet import path),"
        (" path after , fixed)
    for (input, description) in test_cases    {debug!(input = input, description = description,  Testing "case};])
        let full_input = format!(", " test\\n{}, input)
        let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()"import)";} else {debug!(input = input,  Error , ;}))
            Err(_) => {debug!(input = input,  Error ")}
    info!(", :  error cases test completed};")
    let input = r#", #  yeet  github.com/user/", ".org/
    debug!(statement_count = program.statements.len(),  ", ";)
    let expected_paths = vec![".com/user/package,  ", example{]. path},
        assert_eq!(mock_import.path.value, path, Dot notation import path should be ", {}, path)"
        assert!(mock_import.alias.is_none(), , default)]""
    info!(, :  with dot notation parsing test completed "fixed")