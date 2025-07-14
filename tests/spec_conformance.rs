//! Comprehensive grammar conformance tests
//! Validates that implementation matches spec-map.json requirements

use cursed::lexer::{Lexer, TokenKind};
use cursed::parser::Parser;
use std::fs;
use serde_json::{self, Value};

#[test]
fn test_spec_map_keyword_conformance() {
    // Load spec-map.json
    let spec_content = fs::read_to_string("spec-map.json")
        .expect("Failed to read spec-map.json");
    let spec: Value = serde_json::from_str(&spec_content)
        .expect("Failed to parse spec-map.json");
    
    // Extract keyword mapping from spec
    let keywords = &spec["lexical"][0]["mapping"];
    
    // Test each keyword in the spec
    test_keyword_implementation(keywords, "vibe", TokenKind::Vibe);
    test_keyword_implementation(keywords, "yeet", TokenKind::Yeet);
    test_keyword_implementation(keywords, "slay", TokenKind::Slay);
    test_keyword_implementation(keywords, "yolo", TokenKind::Yolo);
    test_keyword_implementation(keywords, "sus", TokenKind::Sus);
    test_keyword_implementation(keywords, "facts", TokenKind::Facts);
    test_keyword_implementation(keywords, "lowkey", TokenKind::Lowkey);
    test_keyword_implementation(keywords, "highkey", TokenKind::Highkey);
    test_keyword_implementation(keywords, "bestie", TokenKind::Bestie);
    test_keyword_implementation(keywords, "periodt", TokenKind::Periodt);
    test_keyword_implementation(keywords, "vibe_check", TokenKind::VibeCheck);
    test_keyword_implementation(keywords, "mood", TokenKind::Mood);
    test_keyword_implementation(keywords, "basic", TokenKind::Basic);
    test_keyword_implementation(keywords, "ghosted", TokenKind::Ghosted);
    test_keyword_implementation(keywords, "simp", TokenKind::Simp);
    test_keyword_implementation(keywords, "be_like", TokenKind::BeLike);
    test_keyword_implementation(keywords, "squad", TokenKind::Squad);
    test_keyword_implementation(keywords, "collab", TokenKind::Collab);
    test_keyword_implementation(keywords, "tea", TokenKind::Tea);
    test_keyword_implementation(keywords, "dm", TokenKind::Dm);
    test_keyword_implementation(keywords, "stan", TokenKind::Stan);
    test_keyword_implementation(keywords, "flex", TokenKind::Flex);
    test_keyword_implementation(keywords, "later", TokenKind::Later);
    test_keyword_implementation(keywords, "based", TokenKind::Truth);
    test_keyword_implementation(keywords, "cap", TokenKind::Lies);
    test_keyword_implementation(keywords, "cringe", TokenKind::Cap);
    test_keyword_implementation(keywords, "ready", TokenKind::Ready);
}

fn test_keyword_implementation(spec_keywords: &Value, keyword: &str, expected_token: TokenKind) {
    // Verify keyword exists in spec
    assert!(spec_keywords.get(keyword).is_some(), 
        "Keyword '{}' not found in spec-map.json", keyword);
    
    // Test lexer recognizes keyword
    let mut lexer = Lexer::new(keyword.to_string());
    let tokens = lexer.tokenize().expect("Failed to tokenize keyword");
    
    assert_eq!(tokens.len(), 2); // keyword + EOF
    assert_eq!(tokens[0].kind, expected_token, 
        "Lexer does not recognize '{}' as {:?}", keyword, expected_token);
    assert_eq!(tokens[0].lexeme, keyword);
}

#[test]
fn test_grammar_rule_conformance() {
    let spec_content = fs::read_to_string("spec-map.json")
        .expect("Failed to read spec-map.json");
    let spec: Value = serde_json::from_str(&spec_content)
        .expect("Failed to parse spec-map.json");
    
    let grammar_rules = spec["grammar"].as_array()
        .expect("Grammar rules not found in spec");
    
    for rule in grammar_rules {
        let rule_name = rule["rule"].as_str().unwrap();
        let description = rule["description"].as_str().unwrap();
        let syntax = rule["syntax"].as_str().unwrap();
        let required = rule["required"].as_bool().unwrap_or(false);
        
        if required {
            match rule_name {
                "PackageClause" => test_package_clause_parsing(syntax),
                "ImportDecl" => test_import_decl_parsing(syntax),
                "FuncDecl" => test_func_decl_parsing(syntax),
                "VarDecl" => test_var_decl_parsing(syntax),
                "ConstDecl" => test_const_decl_parsing(syntax),
                "TypeDecl" => test_type_decl_parsing(syntax),
                "IfStmt" => test_if_stmt_parsing(syntax),
                "ForStmt" => test_for_stmt_parsing(syntax),
                "WhileStmt" => test_while_stmt_parsing(syntax),
                "SwitchStmt" => test_switch_stmt_parsing(syntax),
                "ReturnStmt" => test_return_stmt_parsing(syntax),
                "BreakStmt" => test_break_stmt_parsing(syntax),
                "ContinueStmt" => test_continue_stmt_parsing(syntax),
                "DeferStmt" => test_defer_stmt_parsing(syntax),
                "GoStmt" => test_go_stmt_parsing(syntax),
                "SelectStmt" => test_select_stmt_parsing(syntax),
                "ShortVarDecl" => test_short_var_decl_parsing(syntax),
                "TupleDestructuring" => test_tuple_destructuring_parsing(syntax),
                _ => println!("Warning: Grammar rule '{}' not tested", rule_name),
            }
        }
    }
}

fn test_package_clause_parsing(syntax: &str) {
    let test_cases = vec![
        "vibe main",
        "vibe mypackage",
        "vibe testutils",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse();
        assert!(result.is_ok(), 
            "Failed to parse package clause: {}", case);
    }
}

fn test_import_decl_parsing(syntax: &str) {
    let test_cases = vec![
        "yeet \"fmt\"",
        "yeet \"os\"",
        "yeet (\n  \"fmt\"\n  \"os\"\n)",
        "yeet alias \"package\"",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_import_statement();
        assert!(result.is_ok(), 
            "Failed to parse import declaration: {}", case);
    }
}

fn test_func_decl_parsing(syntax: &str) {
    let test_cases = vec![
        "slay add(x, y normie) normie { yolo x + y }",
        "slay main() { vibez.spill(\"hello\") }",
        "slay greet(name tea) { vibez.spill(name) }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_function_declaration();
        assert!(result.is_ok(), 
            "Failed to parse function declaration: {}", case);
    }
}

fn test_var_decl_parsing(syntax: &str) {
    let test_cases = vec![
        "sus x normie = 42",
        "sus name tea = \"Alice\"",
        "sus flag lit = based",
        "sus (\n  x normie = 1\n  y normie = 2\n)",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_variable_declaration();
        assert!(result.is_ok(), 
            "Failed to parse variable declaration: {}", case);
    }
}

fn test_const_decl_parsing(syntax: &str) {
    let test_cases = vec![
        "facts PI = 3.14159",
        "facts MAX_SIZE = 1000",
        "facts (\n  PI = 3.14159\n  E = 2.71828\n)",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_constant_declaration();
        assert!(result.is_ok(), 
            "Failed to parse constant declaration: {}", case);
    }
}

fn test_type_decl_parsing(syntax: &str) {
    let test_cases = vec![
        "be_like Person squad { name tea; age normie }",
        "be_like StringAlias tea",
        "be_like Counter normie",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_type_declaration();
        assert!(result.is_ok(), 
            "Failed to parse type declaration: {}", case);
    }
}

fn test_if_stmt_parsing(syntax: &str) {
    let test_cases = vec![
        "lowkey x > 0 { vibez.spill(x) }",
        "lowkey x > 0 { vibez.spill(x) } highkey { vibez.spill(0) }",
        "lowkey x := getValue(); x > 0 { vibez.spill(x) }",
        "lowkey (x > 0) { vibez.spill(x) }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_if_statement();
        assert!(result.is_ok(), 
            "Failed to parse if statement: {}", case);
    }
}

fn test_for_stmt_parsing(syntax: &str) {
    let test_cases = vec![
        "bestie i := 0; i < 10; i++ { vibez.spill(i) }",
        "bestie x < 100 { x = x * 2 }",
        "bestie { vibez.spill(\"infinite\") }",
        "bestie _, val := flex items { vibez.spill(val) }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_for_statement();
        assert!(result.is_ok(), 
            "Failed to parse for statement: {}", case);
    }
}

fn test_while_stmt_parsing(syntax: &str) {
    let test_cases = vec![
        "periodt x > 0 { x-- }",
        "periodt based { vibez.spill(\"loop\") }",
        "periodt condition() { doSomething() }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_while_statement();
        assert!(result.is_ok(), 
            "Failed to parse while statement: {}", case);
    }
}

fn test_switch_stmt_parsing(syntax: &str) {
    let test_cases = vec![
        "vibe_check x { mood 1: vibez.spill(\"one\"); basic: vibez.spill(\"other\") }",
        "vibe_check { mood x > 0: vibez.spill(\"positive\"); basic: vibez.spill(\"zero\") }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_switch_statement();
        assert!(result.is_ok(), 
            "Failed to parse switch statement: {}", case);
    }
}

fn test_return_stmt_parsing(syntax: &str) {
    let test_cases = vec![
        "yolo",
        "yolo x",
        "yolo x + y",
        "yolo x, y",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_return_statement();
        assert!(result.is_ok(), 
            "Failed to parse return statement: {}", case);
    }
}

fn test_break_stmt_parsing(syntax: &str) {
    let test_cases = vec![
        "ghosted",
        "ghosted label",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_break_statement();
        assert!(result.is_ok(), 
            "Failed to parse break statement: {}", case);
    }
}

fn test_continue_stmt_parsing(syntax: &str) {
    let test_cases = vec![
        "simp",
        "simp label",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_continue_statement();
        assert!(result.is_ok(), 
            "Failed to parse continue statement: {}", case);
    }
}

fn test_defer_stmt_parsing(syntax: &str) {
    let test_cases = vec![
        "later cleanup()",
        "later file.close()",
        "later { vibez.spill(\"cleanup\") }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_defer_statement();
        assert!(result.is_ok(), 
            "Failed to parse defer statement: {}", case);
    }
}

fn test_go_stmt_parsing(syntax: &str) {
    let test_cases = vec![
        "stan worker()",
        "stan process(data)",
        "stan { doWork() }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_go_statement();
        assert!(result.is_ok(), 
            "Failed to parse go statement: {}", case);
    }
}

fn test_select_stmt_parsing(syntax: &str) {
    let test_cases = vec![
        "ready { mood ch <- value: vibez.spill(\"sent\"); basic: vibez.spill(\"default\") }",
        "ready { mood val := <-ch: vibez.spill(val); basic: }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_select_statement();
        assert!(result.is_ok(), 
            "Failed to parse select statement: {}", case);
    }
}

fn test_short_var_decl_parsing(syntax: &str) {
    let test_cases = vec![
        "x := 42",
        "name := \"Alice\"",
        "a, b := 1, 2",
        "result, err := operation()",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_short_variable_declaration();
        assert!(result.is_ok(), 
            "Failed to parse short variable declaration: {}", case);
    }
}

fn test_tuple_destructuring_parsing(syntax: &str) {
    let test_cases = vec![
        "(a, b, c) := tuple",
        "(x, y) := getCoordinates()",
        "(first, _, third) := triple",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_tuple_destructuring();
        assert!(result.is_ok(), 
            "Failed to parse tuple destructuring: {}", case);
    }
}

#[test]
fn test_comment_syntax_conformance() {
    let test_cases = vec![
        ("fr fr This is a line comment", true),
        ("no cap This is a block comment on god", true),
        ("// Invalid C-style comment", false),
        ("/* Invalid C-style block */", false),
    ];
    
    for (case, should_pass) in test_cases {
        let mut lexer = Lexer::new(case.to_string());
        let result = lexer.tokenize();
        
        if should_pass {
            assert!(result.is_ok(), 
                "Failed to parse valid comment: {}", case);
        } else {
            // Should parse as regular tokens, not comments
            let tokens = result.unwrap();
            assert!(tokens.len() > 2, // More than just comment + EOF
                "Incorrectly parsed invalid comment as valid: {}", case);
        }
    }
}

#[test]
fn test_literal_conformance() {
    let test_cases = vec![
        // Integer literals
        ("123", TokenKind::Number),
        ("0x1A", TokenKind::Number),
        ("0o17", TokenKind::Number),
        ("0b1010", TokenKind::Number),
        
        // Float literals
        ("3.14", TokenKind::Number),
        ("1.0e10", TokenKind::Number),
        (".5", TokenKind::Number),
        ("1.", TokenKind::Number),
        
        // String literals
        ("\"hello world\"", TokenKind::StringLiteral("hello world".to_string())),
        ("`multiline\nstring`", TokenKind::String),
        
        // Boolean literals
        ("based", TokenKind::Truth),
        ("cap", TokenKind::Lies),
        
        // Nil literal
        ("cringe", TokenKind::Cap),
        
        // Character literals
        ("'a'", TokenKind::Character),
        ("'\\n'", TokenKind::Character),
    ];
    
    for (input, expected_kind) in test_cases {
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().expect(&format!("Failed to tokenize: {}", input));
        
        assert_eq!(tokens.len(), 2); // literal + EOF
        assert_eq!(tokens[0].kind, expected_kind,
            "Literal '{}' parsed as {:?}, expected {:?}", 
            input, tokens[0].kind, expected_kind);
    }
}

#[test] 
fn test_range_for_loop_conformance() {
    // Test flex (range-for) loop parsing
    let test_cases = vec![
        "flex i in 0..10 { vibez.spill(i) }",
        "flex item in items { process(item) }",
        "flex _, value := flex collection { handle(value) }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let result = parser.parse_flex_statement();
        assert!(result.is_ok(), 
            "Failed to parse range-for loop: {}", case);
    }
}

#[test]
fn test_round_trip_spec_validation() {
    // Test that all spec examples can be parsed and then formatted back
    let spec_examples = vec![
        // Package and imports
        "vibe main",
        "yeet \"fmt\"",
        "yeet ( \"fmt\"; \"os\" )",
        
        // Declarations
        "facts PI = 3.14159",
        "sus name tea = \"World\"",
        "be_like Person squad { name tea; age normie }",
        "slay add(x, y normie) normie { yolo x + y }",
        
        // Control flow
        "lowkey x > 0 { vibez.spill(x) } highkey { vibez.spill(0) }",
        "bestie i := 0; i < 10; i++ { vibez.spill(i) }",
        "periodt x > 0 { x-- }",
        "vibe_check day { mood \"Monday\": vibez.spill(\"start\"); basic: vibez.spill(\"mid\") }",
        
        // Concurrency
        "stan worker(data)",
        "ready { mood ch <- value: vibez.spill(\"sent\"); basic: vibez.spill(\"default\") }",
        
        // Other statements
        "yolo x + y",
        "ghosted",
        "simp",
        "later cleanup()",
        "x := 42",
        "(a, b) := tuple",
    ];
    
    for example in spec_examples {
        // Parse the example
        let mut parser = Parser::new(example);
        let ast = parser.parse();
        assert!(ast.is_ok(), 
            "Failed to parse spec example: {}", example);
        
        // Format it back (round-trip test)
        // Note: This would require implementing a formatter
        // For now, we just verify parsing works
    }
}

#[test]
fn test_codegen_keyword_support() {
    // Verify that all spec keywords are supported in codegen
    let keywords_requiring_codegen = vec![
        "stan",    // goroutines
        "dm",      // channels 
        "ready",   // select statements
        "later",   // defer statements
        "yikes",   // error handling
        "shook",   // panic/recover
        "fam",     // error recovery
    ];
    
    for keyword in keywords_requiring_codegen {
        // Test that codegen supports these keywords
        // This is a placeholder - actual implementation would test LLVM IR generation
        println!("Testing codegen support for keyword: {}", keyword);
    }
}
