//! Tests for lexer vs parser consistency
//! Identifies discrepancies between what lexer recognizes and parser expects

use cursed::lexer::{Lexer, TokenKind};
use cursed::parser::Parser;

#[test]
fn test_periodt_consistency() {
    // Test that lexer recognizes 'periodt' and parser can handle it
    let input = "periodt x > 0 { x-- }";
    
    // Test lexer
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize().expect("Failed to tokenize periodt statement");
    
    // First token should be Periodt
    assert_eq!(tokens[0].kind, TokenKind::Periodt, 
        "Lexer does not recognize 'periodt' as Periodt token");
    assert_eq!(tokens[0].lexeme, "periodt");
    
    // Test parser
    let mut parser = Parser::new(input);
    let result = parser.parse_while_statement();
    assert!(result.is_ok(), 
        "Parser failed to handle 'periodt' while statement: {:?}", result.err());
}

#[test]
fn test_flex_consistency() {
    // Test that lexer recognizes 'flex' and parser can handle range-for loops
    let input = "flex i in 0..10 { vibez.spill(i) }";
    
    // Test lexer
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize().expect("Failed to tokenize flex statement");
    
    // First token should be Flex
    assert_eq!(tokens[0].kind, TokenKind::Flex, 
        "Lexer does not recognize 'flex' as Flex token");
    assert_eq!(tokens[0].lexeme, "flex");
    
    // Test parser
    let mut parser = Parser::new(input);
    let result = parser.parse_flex_statement();
    assert!(result.is_ok(), 
        "Parser failed to handle 'flex' range-for statement: {:?}", result.err());
}

#[test]
fn test_bestie_vs_flex_disambiguation() {
    // Test that parser correctly distinguishes between 'bestie' (regular for) and 'flex' (range-for)
    let bestie_cases = vec![
        "bestie i := 0; i < 10; i++ { vibez.spill(i) }",  // C-style for
        "bestie condition { doSomething() }",              // While-style for
        "bestie { infiniteLoop() }",                       // Infinite for
    ];
    
    let flex_cases = vec![
        "flex i in 0..10 { vibez.spill(i) }",             // Range for
        "flex item in items { process(item) }",           // Collection iteration
    ];
    
    // Test 'bestie' cases
    for case in bestie_cases {
        let mut lexer = Lexer::new(case.to_string());
        let tokens = lexer.tokenize().expect(&format!("Failed to tokenize bestie case: {}", case));
        assert_eq!(tokens[0].kind, TokenKind::Bestie, 
            "Lexer does not recognize 'bestie' in: {}", case);
        
        let mut parser = Parser::new(case);
        let result = parser.parse_for_statement();
        assert!(result.is_ok(), 
            "Parser failed to handle 'bestie' for statement: {}", case);
    }
    
    // Test 'flex' cases
    for case in flex_cases {
        let mut lexer = Lexer::new(case.to_string());
        let tokens = lexer.tokenize().expect(&format!("Failed to tokenize flex case: {}", case));
        assert_eq!(tokens[0].kind, TokenKind::Flex, 
            "Lexer does not recognize 'flex' in: {}", case);
        
        let mut parser = Parser::new(case);
        let result = parser.parse_flex_statement();
        assert!(result.is_ok(), 
            "Parser failed to handle 'flex' range-for statement: {}", case);
    }
}

#[test]
fn test_comment_keyword_conflicts() {
    // Test that comment keywords don't conflict with regular identifiers
    let test_cases = vec![
        ("fr", TokenKind::Identifier),        // 'fr' alone should be identifier
        ("fr fr comment", TokenKind::Eof),    // 'fr fr' should be consumed as comment
        ("no", TokenKind::Identifier),        // 'no' alone should be identifier  
        ("cap", TokenKind::Lies),             // 'cap' should be boolean literal
        ("nocap", TokenKind::NoCap),          // 'nocap' should be separate keyword
    ];
    
    for (input, expected_first_token) in test_cases {
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().expect(&format!("Failed to tokenize: {}", input));
        
        if expected_first_token == TokenKind::Eof {
            // Comment case - should only have EOF
            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].kind, TokenKind::Eof);
        } else {
            // Regular token case
            assert!(tokens.len() >= 2); // token + EOF
            assert_eq!(tokens[0].kind, expected_first_token,
                "Input '{}' produced {:?}, expected {:?}", 
                input, tokens[0].kind, expected_first_token);
        }
    }
}

#[test]
fn test_basic_keyword_consistency() {
    // Test all basic keywords for lexer/parser consistency
    let keyword_tests = vec![
        ("vibe", TokenKind::Vibe),
        ("yeet", TokenKind::Yeet),
        ("slay", TokenKind::Slay),
        ("yolo", TokenKind::Yolo),
        ("sus", TokenKind::Sus),
        ("facts", TokenKind::Facts),
        ("lowkey", TokenKind::Lowkey),
        ("highkey", TokenKind::Highkey),
        ("bestie", TokenKind::Bestie),
        ("periodt", TokenKind::Periodt),
        ("vibe_check", TokenKind::VibeCheck),
        ("mood", TokenKind::Mood),
        ("basic", TokenKind::Basic),
        ("ghosted", TokenKind::Ghosted),
        ("simp", TokenKind::Simp),
        ("be_like", TokenKind::BeLike),
        ("squad", TokenKind::Squad),
        ("collab", TokenKind::Collab),
        ("tea", TokenKind::Tea),
        ("dm", TokenKind::Dm),
        ("stan", TokenKind::Stan),
        ("flex", TokenKind::Flex),
        ("later", TokenKind::Later),
        ("based", TokenKind::Truth),
        ("cap", TokenKind::Lies),
        ("cringe", TokenKind::Cap),
        ("ready", TokenKind::Ready),
    ];
    
    for (keyword, expected_token) in keyword_tests {
        // Test lexer recognition
        let mut lexer = Lexer::new(keyword.to_string());
        let tokens = lexer.tokenize().expect(&format!("Failed to tokenize keyword: {}", keyword));
        
        assert_eq!(tokens.len(), 2); // keyword + EOF
        assert_eq!(tokens[0].kind, expected_token,
            "Lexer maps '{}' to {:?}, expected {:?}", 
            keyword, tokens[0].kind, expected_token);
        assert_eq!(tokens[0].lexeme, keyword);
        
        // Test that parser doesn't reject the keyword
        // (This is a basic check - specific parsing would need individual test functions)
        let mut parser = Parser::new(keyword);
        let _ = parser.parse_primary_expression(); // Should not panic
    }
}

#[test]
fn test_type_keyword_consistency() {
    // Test type keywords specifically
    let type_keywords = vec![
        ("normie", TokenKind::Normie),
        ("tea", TokenKind::Tea),
        ("lit", TokenKind::Lit),
        ("smol", TokenKind::Smol),
        ("mid", TokenKind::Mid),
        ("thicc", TokenKind::Thicc),
        ("snack", TokenKind::Snack),
        ("meal", TokenKind::Meal),
        ("byte", TokenKind::Byte),
        ("rune", TokenKind::Rune),
        ("extra", TokenKind::Extra),
        ("sip", TokenKind::Sip),
    ];
    
    for (type_keyword, expected_token) in type_keywords {
        // Test lexer recognition
        let mut lexer = Lexer::new(type_keyword.to_string());
        let tokens = lexer.tokenize().expect(&format!("Failed to tokenize type: {}", type_keyword));
        
        assert_eq!(tokens[0].kind, expected_token,
            "Type keyword '{}' not recognized correctly", type_keyword);
        
        // Test in variable declaration context
        let var_decl = format!("sus x {} = defaultValue", type_keyword);
        let mut parser = Parser::new(&var_decl);
        let result = parser.parse_variable_declaration();
        assert!(result.is_ok(), 
            "Parser failed to handle type keyword '{}' in variable declaration", type_keyword);
    }
}

#[test]
fn test_operator_consistency() {
    // Test operators that might have lexer/parser mismatches
    let operator_tests = vec![
        (":=", TokenKind::ColonEqual),
        ("<-", TokenKind::LeftArrow),
        ("->", TokenKind::Arrow),
        ("..", TokenKind::DotDot),
        ("==", TokenKind::EqualEqual),
        ("!=", TokenKind::NotEqual),
        ("<=", TokenKind::LessEqual),
        (">=", TokenKind::GreaterEqual),
        ("&&", TokenKind::LogicalAnd),
        ("||", TokenKind::LogicalOr),
        ("++", TokenKind::Increment),
        ("--", TokenKind::Decrement),
    ];
    
    for (operator, expected_token) in operator_tests {
        let mut lexer = Lexer::new(operator.to_string());
        let tokens = lexer.tokenize().expect(&format!("Failed to tokenize operator: {}", operator));
        
        assert_eq!(tokens[0].kind, expected_token,
            "Operator '{}' not recognized as {:?}", operator, expected_token);
    }
}

#[test]
fn test_range_for_syntax_edge_cases() {
    // Test edge cases in range-for syntax that might cause lexer/parser mismatches
    let edge_cases = vec![
        // Valid cases
        ("flex i in 0..10 { }", true),
        ("flex item in items { }", true),
        ("flex _, value := flex collection { }", true),
        
        // Invalid cases that should be caught
        ("flex in 0..10 { }", false),        // Missing variable
        ("flex i 0..10 { }", false),         // Missing 'in'
        ("flex i in 0 10 { }", false),       // Missing '..'
        ("flex i in 0.. { }", false),        // Incomplete range
    ];
    
    for (test_case, should_parse) in edge_cases {
        let mut parser = Parser::new(test_case);
        let result = parser.parse_flex_statement();
        
        if should_parse {
            assert!(result.is_ok(), 
                "Valid range-for syntax should parse: {}", test_case);
        } else {
            assert!(result.is_err(), 
                "Invalid range-for syntax should not parse: {}", test_case);
        }
    }
}

#[test]
fn test_composite_for_range_syntax() {
    // Test composite for-range expressions that combine multiple elements
    let composite_cases = vec![
        "bestie i, val := flex items { process(i, val) }",
        "bestie _, value := flex collection { handle(value) }",
        "bestie key, val := flex map { store(key, val) }",
    ];
    
    for case in composite_cases {
        // Test lexer tokenization
        let mut lexer = Lexer::new(case.to_string());
        let tokens = lexer.tokenize().expect(&format!("Failed to tokenize composite for-range: {}", case));
        
        // Verify key tokens are present
        let token_kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert!(token_kinds.contains(&TokenKind::Bestie), 
            "Missing 'bestie' token in: {}", case);
        assert!(token_kinds.contains(&TokenKind::ColonEqual), 
            "Missing ':=' token in: {}", case);
        assert!(token_kinds.contains(&TokenKind::Flex), 
            "Missing 'flex' token in: {}", case);
        
        // Test parser handling
        let mut parser = Parser::new(case);
        let result = parser.parse_for_statement();
        assert!(result.is_ok(), 
            "Parser failed to handle composite for-range: {}", case);
    }
}
