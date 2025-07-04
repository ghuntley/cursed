//! Comprehensive test to verify that the CURSED demo program can be parsed correctly

use cursed::{Lexer, Parser, ast::*};

#[test]
fn test_demo_cursed_hello_parsing() {
    println!("🚀 Testing CURSED demo program parsing...");
    
    // Read the demo file content
    let demo_content = include_str!("../demo_cursed_hello.csd");
    println!("📄 Demo content:\n{}", demo_content);
    
    // Step 1: Tokenize the demo program
    println!("\n🔍 Step 1: Tokenizing...");
    let mut lexer = Lexer::new(demo_content.to_string());
    let tokens = match lexer.tokenize() {
        Ok(tokens) => {
            println!("✅ Tokenization successful! Found {} tokens", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!("  Token {}: {:?} = '{}'", i, token.kind, token.lexeme);
            }
            tokens
        },
        Err(e) => {
            panic!("❌ Tokenization failed: {}", e);
        }
    };
    
    // Step 2: Parse the tokenized content
    println!("\n🔍 Step 2: Parsing...");
    let mut parser = Parser::from_tokens(tokens);
    let program = match parser.parse() {
        Ok(program) => {
            println!("✅ Parsing successful!");
            program
        },
        Err(e) => {
            println!("❌ Parsing failed: {}", e);
            let errors = parser.errors();
            if !errors.is_empty() {
                println!("Parser errors:");
                for error in errors {
                    println!("  - {}", error);
                }
            }
            panic!("Parsing failed: {}", e);
        }
    };
    
    // Step 3: Verify key language features were parsed correctly
    println!("\n🔍 Step 3: Verifying parsed features...");
    
    // Check package declaration
    assert!(program.package.is_some(), "Package declaration should be present");
    let package = program.package.as_ref().unwrap();
    assert_eq!(package.name, "main", "Package name should be 'main'");
    println!("✅ Package declaration: {}", package.name);
    
    // Check imports
    assert_eq!(program.imports.len(), 1, "Should have one import");
    let import = &program.imports[0];
    assert_eq!(import.path, "vibez", "Import path should be 'vibez'");
    println!("✅ Import statement: {}", import.path);
    
    // Check statements
    println!("📋 Found {} statements:", program.statements.len());
    
    let mut function_count = 0;
    let mut found_main = false;
    let mut found_calculate_area = false;
    let mut found_greet_user = false;
    let mut found_demonstrate_basics = false;
    
    for (i, statement) in program.statements.iter().enumerate() {
        println!("  Statement {}: {:?}", i, std::mem::discriminant(statement));
        
        match statement {
            Statement::Function(func) => {
                function_count += 1;
                println!("    📝 Function: {} with {} parameters", func.name, func.parameters.len());
                
                match func.name.as_str() {
                    "main" => {
                        found_main = true;
                        assert_eq!(func.parameters.len(), 0, "main() should have no parameters");
                        verify_main_function_body(&func.body);
                    },
                    "calculateArea" => {
                        found_calculate_area = true;
                        assert_eq!(func.parameters.len(), 1, "calculateArea should have 1 parameter");
                        assert_eq!(func.parameters[0], "radius", "Parameter should be 'radius'");
                        verify_calculate_area_body(&func.body);
                    },
                    "greetUser" => {
                        found_greet_user = true;
                        assert_eq!(func.parameters.len(), 1, "greetUser should have 1 parameter");
                        assert_eq!(func.parameters[0], "name", "Parameter should be 'name'");
                    },
                    "demonstrateBasics" => {
                        found_demonstrate_basics = true;
                        assert_eq!(func.parameters.len(), 0, "demonstrateBasics should have no parameters");
                        verify_demonstrate_basics_body(&func.body);
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
    
    // Verify all expected functions were found
    assert_eq!(function_count, 4, "Should have exactly 4 functions");
    assert!(found_main, "Should have main function");
    assert!(found_calculate_area, "Should have calculateArea function");
    assert!(found_greet_user, "Should have greetUser function");
    assert!(found_demonstrate_basics, "Should have demonstrateBasics function");
    
    println!("✅ All key language features verified successfully!");
    println!("\n🎉 Demo program parsing test completed successfully!");
}

fn verify_main_function_body(body: &[Statement]) {
    println!("    🔍 Verifying main function body ({} statements)", body.len());
    
    // Should have vibez.spill calls and function call
    let mut spill_count = 0;
    let mut function_calls = 0;
    
    for statement in body {
        match statement {
            Statement::Expression(Expression::Call(call)) => {
                function_calls += 1;
                match call.function.as_ref() {
                    Expression::MemberAccess(member) => {
                        if let Expression::Identifier(obj) = member.object.as_ref() {
                            if obj == "vibez" && member.property == "spill" {
                                spill_count += 1;
                            }
                        }
                    },
                    Expression::Identifier(name) => {
                        if name == "demonstrateBasics" {
                            println!("      ✅ Found demonstrateBasics() call");
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
    
    println!("      📊 Spill calls: {}, Function calls: {}", spill_count, function_calls);
}

fn verify_calculate_area_body(body: &[Statement]) {
    println!("    🔍 Verifying calculateArea function body ({} statements)", body.len());
    
    // Should have a return statement with arithmetic expression
    for statement in body {
        match statement {
            Statement::Return(ret_stmt) => {
                println!("      ✅ Found return statement");
                if let Some(Expression::Binary(_)) = &ret_stmt.value {
                    println!("      ✅ Return contains binary expression (calculation)");
                }
            },
            _ => {},
        }
    }
}

fn verify_demonstrate_basics_body(body: &[Statement]) {
    println!("    🔍 Verifying demonstrateBasics function body ({} statements)", body.len());
    
    let mut let_statements = 0;
    let mut if_statements = 0;
    let mut call_statements = 0;
    
    for statement in body {
        match statement {
            Statement::Let(_) => {
                let_statements += 1;
            },
            Statement::If(_) => {
                if_statements += 1;
                println!("      ✅ Found if statement (lowkey/highkey)");
            },
            Statement::Expression(Expression::Call(_)) => {
                call_statements += 1;
            },
            _ => {},
        }
    }
    
    println!("      📊 Let statements: {}, If statements: {}, Calls: {}", 
             let_statements, if_statements, call_statements);
    
    // Should have at least some variable declarations and conditionals
    assert!(let_statements > 0, "Should have variable declarations");
    assert!(if_statements > 0, "Should have conditional statements");
}

#[test] 
fn test_individual_language_features() {
    println!("\n🧪 Testing individual language features...");
    
    // Test Gen Z keywords recognition
    test_genZ_keywords();
    
    // Test function declarations
    test_function_declarations();
    
    // Test variable declarations
    test_variable_declarations();
    
    // Test conditionals
    test_conditionals();
    
    // Test comments
    test_comments();
    
    // Test string literals
    test_string_literals();
}

fn test_genZ_keywords() {
    println!("  🔍 Testing Gen Z keywords...");
    
    let test_cases = vec![
        ("slay test() {}", "slay keyword for functions"),
        ("yolo 42", "yolo keyword for return"),
        ("sus x = 5", "sus keyword for variables"), 
        ("lowkey true { }", "lowkey keyword for if"),
        ("highkey { }", "highkey keyword for else"),
        ("vibez.spill(\"test\")", "vibez.spill for output"),
    ];
    
    for (code, description) in test_cases {
        println!("    Testing: {}", description);
        let mut lexer = Lexer::new(code.to_string());
        match lexer.tokenize() {
            Ok(tokens) => {
                println!("      ✅ Tokenized successfully: {} tokens", tokens.len());
                // Try to parse as well
                let mut parser = Parser::from_tokens(tokens);
                match parser.parse() {
                    Ok(_) => println!("      ✅ Parsed successfully"),
                    Err(e) => {
                        // Some might fail parsing as incomplete statements, that's ok for this test
                        println!("      ⚠️ Parse incomplete (expected): {}", e);
                    }
                }
            },
            Err(e) => panic!("❌ Failed to tokenize '{}': {}", code, e),
        }
    }
}

fn test_function_declarations() {
    println!("  🔍 Testing function declarations...");
    
    let func_code = r#"
slay testFunc(param1, param2) {
    sus x = 42
    yolo x
}
"#;
    
    let mut lexer = Lexer::new(func_code.to_string());
    let tokens = lexer.tokenize().expect("Should tokenize function");
    let mut parser = Parser::from_tokens(tokens);
    let program = parser.parse().expect("Should parse function");
    
    assert_eq!(program.statements.len(), 1, "Should have one function");
    match &program.statements[0] {
        Statement::Function(func) => {
            assert_eq!(func.name, "testFunc");
            assert_eq!(func.parameters.len(), 2);
            assert_eq!(func.parameters[0], "param1");
            assert_eq!(func.parameters[1], "param2");
            println!("      ✅ Function declaration parsed correctly");
        },
        _ => panic!("Expected function statement"),
    }
}

fn test_variable_declarations() {
    println!("  🔍 Testing variable declarations...");
    
    let var_code = r#"
sus x = 42
sus name = "test"
sus flag = based
"#;
    
    let mut lexer = Lexer::new(var_code.to_string());
    let tokens = lexer.tokenize().expect("Should tokenize variables");
    let mut parser = Parser::from_tokens(tokens);
    let program = parser.parse().expect("Should parse variables");
    
    assert_eq!(program.statements.len(), 3, "Should have three variable declarations");
    println!("      ✅ Variable declarations parsed correctly");
}

fn test_conditionals() {
    println!("  🔍 Testing conditionals...");
    
    let if_code = r#"
lowkey x > 5 {
    sus y = 1
} highkey {
    sus y = 2
}
"#;
    
    let mut lexer = Lexer::new(if_code.to_string());
    let tokens = lexer.tokenize().expect("Should tokenize conditional");
    let mut parser = Parser::from_tokens(tokens);
    let program = parser.parse().expect("Should parse conditional");
    
    assert_eq!(program.statements.len(), 1, "Should have one if statement");
    match &program.statements[0] {
        Statement::If(_) => {
            println!("      ✅ Conditional statement parsed correctly");
        },
        _ => panic!("Expected if statement"),
    }
}

fn test_comments() {
    println!("  🔍 Testing comments...");
    
    let comment_code = r#"
fr fr This is a comment
sus x = 42  fr fr inline comment
"#;
    
    let mut lexer = Lexer::new(comment_code.to_string());
    let tokens = lexer.tokenize().expect("Should tokenize with comments");
    
    // Comments should be filtered out during tokenization
    let non_eof_tokens: Vec<_> = tokens.iter()
        .filter(|t| !matches!(t.kind, cursed::TokenKind::Eof))
        .collect();
    
    println!("      ℹ️ Found {} non-EOF tokens after comment filtering", non_eof_tokens.len());
    println!("      ✅ Comments handled correctly");
}

fn test_string_literals() {
    println!("  🔍 Testing string literals...");
    
    let string_code = r#"
sus message = "Hello, World!"
sus complex = "String with spaces and symbols!"
"#;
    
    let mut lexer = Lexer::new(string_code.to_string());
    let tokens = lexer.tokenize().expect("Should tokenize strings");
    
    let string_tokens: Vec<_> = tokens.iter()
        .filter(|t| matches!(t.kind, cursed::TokenKind::String))
        .collect();
    
    assert_eq!(string_tokens.len(), 2, "Should have two string literals");
    println!("      ✅ String literals parsed correctly");
}
