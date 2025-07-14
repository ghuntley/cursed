//! Tests for codegen support of spec keywords
//! Validates that all spec-required keywords have proper LLVM codegen support

use cursed::parser::Parser;
use cursed::codegen::CodeGenerator;

#[test]
fn test_periodt_codegen_support() {
    let test_cases = vec![
        "periodt x > 0 { x = x - 1 }",
        "periodt based { vibez.spill(\"loop\") }",
        "periodt condition() { doWork() }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let ast = parser.parse().expect(&format!("Failed to parse: {}", case));
        
        // Test that codegen can handle periodt statements
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate_llvm_ir(&ast);
        assert!(result.is_ok(), 
            "Codegen failed for periodt statement: {}", case);
        
        let ir = result.unwrap();
        // Verify LLVM IR contains loop structures
        assert!(ir.contains("br label") || ir.contains("loop"), 
            "Generated IR doesn't contain loop structures for: {}", case);
    }
}

#[test]
fn test_flex_range_for_codegen_support() {
    let test_cases = vec![
        "flex i in 0..10 { vibez.spill(i) }",
        "flex item in collection { process(item) }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let ast = parser.parse_flex_statement().expect(&format!("Failed to parse: {}", case));
        
        // Test that codegen can handle flex range-for statements
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate_for_statement(&ast);
        assert!(result.is_ok(), 
            "Codegen failed for flex range-for statement: {}", case);
        
        let ir = result.unwrap();
        // Verify LLVM IR contains range iteration structures
        assert!(ir.contains("range") || ir.contains("iterator") || ir.contains("for"), 
            "Generated IR doesn't contain range iteration for: {}", case);
    }
}

#[test]
fn test_stan_goroutine_codegen_support() {
    let test_cases = vec![
        "stan worker()",
        "stan process(data)",
        "stan { doAsyncWork() }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let ast = parser.parse_go_statement().expect(&format!("Failed to parse: {}", case));
        
        // Test that codegen can handle stan goroutine statements
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate_goroutine_statement(&ast);
        assert!(result.is_ok(), 
            "Codegen failed for stan goroutine statement: {}", case);
        
        let ir = result.unwrap();
        // Verify LLVM IR contains goroutine spawn code
        assert!(ir.contains("spawn") || ir.contains("goroutine") || ir.contains("thread"), 
            "Generated IR doesn't contain goroutine spawn for: {}", case);
    }
}

#[test]
fn test_dm_channel_codegen_support() {
    let test_cases = vec![
        "sus ch dm<normie>",
        "sus buffered dm<tea>[10]",
        "ch <- value",
        "value := <-ch",
        "close(ch)",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        
        // Parse based on statement type
        let ast = if case.contains("dm<") {
            parser.parse_variable_declaration().expect(&format!("Failed to parse: {}", case))
        } else if case.contains("<-") {
            parser.parse_channel_operation().expect(&format!("Failed to parse: {}", case))
        } else {
            parser.parse_statement().expect(&format!("Failed to parse: {}", case)).unwrap()
        };
        
        // Test that codegen can handle dm channel operations
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate_statement(&ast);
        assert!(result.is_ok(), 
            "Codegen failed for dm channel operation: {}", case);
        
        let ir = result.unwrap();
        // Verify LLVM IR contains channel operation code
        assert!(ir.contains("channel") || ir.contains("chan") || ir.contains("send") || ir.contains("recv"), 
            "Generated IR doesn't contain channel operations for: {}", case);
    }
}

#[test]
fn test_ready_select_codegen_support() {
    let test_cases = vec![
        "ready { mood ch <- value: vibez.spill(\"sent\"); basic: vibez.spill(\"default\") }",
        "ready { mood result := <-ch: handle(result); basic: timeout() }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let ast = parser.parse_select_statement().expect(&format!("Failed to parse: {}", case));
        
        // Test that codegen can handle ready select statements
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate_select_statement(&ast);
        assert!(result.is_ok(), 
            "Codegen failed for ready select statement: {}", case);
        
        let ir = result.unwrap();
        // Verify LLVM IR contains select operation code
        assert!(ir.contains("select") || ir.contains("case") || ir.contains("switch"), 
            "Generated IR doesn't contain select operations for: {}", case);
    }
}

#[test]
fn test_later_defer_codegen_support() {
    let test_cases = vec![
        "later cleanup()",
        "later file.close()",
        "later { resources.free() }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        let ast = parser.parse_defer_statement().expect(&format!("Failed to parse: {}", case));
        
        // Test that codegen can handle later defer statements
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate_defer_statement(&ast);
        assert!(result.is_ok(), 
            "Codegen failed for later defer statement: {}", case);
        
        let ir = result.unwrap();
        // Verify LLVM IR contains defer/cleanup code
        assert!(ir.contains("defer") || ir.contains("cleanup") || ir.contains("finally"), 
            "Generated IR doesn't contain defer operations for: {}", case);
    }
}

#[test]
fn test_error_handling_codegen_support() {
    let test_cases = vec![
        "yikes custom_error(\"message\")",
        "result := operation() shook",
        "fam { risky_operation() } sus error { handle_error(error) }",
    ];
    
    for case in test_cases {
        let mut parser = Parser::new(case);
        
        // Parse based on error handling type
        let ast = if case.contains("yikes") {
            parser.parse_yikes_statement().expect(&format!("Failed to parse: {}", case))
        } else if case.contains("shook") {
            parser.parse_shook_expression().expect(&format!("Failed to parse: {}", case))
        } else if case.contains("fam") {
            parser.parse_fam_statement().expect(&format!("Failed to parse: {}", case))
        } else {
            parser.parse_statement().expect(&format!("Failed to parse: {}", case)).unwrap()
        };
        
        // Test that codegen can handle error handling
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate_error_handling(&ast);
        assert!(result.is_ok(), 
            "Codegen failed for error handling: {}", case);
        
        let ir = result.unwrap();
        // Verify LLVM IR contains error handling code
        assert!(ir.contains("error") || ir.contains("exception") || ir.contains("panic"), 
            "Generated IR doesn't contain error handling for: {}", case);
    }
}

#[test]
fn test_type_keyword_codegen_support() {
    let type_tests = vec![
        ("sus x normie = 42", "normie"),
        ("sus name tea = \"Alice\"", "tea"),
        ("sus flag lit = based", "lit"),
        ("sus small smol = 10", "smol"),
        ("sus medium mid = 1000", "mid"),
        ("sus large thicc = 1000000", "thicc"),
        ("sus floating snack = 3.14", "snack"),
        ("sus precise meal = 3.14159265", "meal"),
        ("sus character sip = 'A'", "sip"),
        ("sus data byte = 255", "byte"),
        ("sus unicode rune = 'π'", "rune"),
        ("sus complex extra = 1+2i", "extra"),
    ];
    
    for (declaration, type_name) in type_tests {
        let mut parser = Parser::new(declaration);
        let ast = parser.parse_variable_declaration().expect(&format!("Failed to parse: {}", declaration));
        
        // Test that codegen can handle type declarations
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate_variable_declaration(&ast);
        assert!(result.is_ok(), 
            "Codegen failed for type {}: {}", type_name, declaration);
        
        let ir = result.unwrap();
        // Verify LLVM IR contains appropriate type information
        let expected_llvm_types = match type_name {
            "normie" => vec!["i32"],
            "tea" => vec!["i8*", "ptr"],
            "lit" => vec!["i1", "bool"],
            "smol" => vec!["i8"],
            "mid" => vec!["i16"],
            "thicc" => vec!["i64"],
            "snack" => vec!["float"],
            "meal" => vec!["double"],
            "sip" => vec!["i8", "i32"],
            "byte" => vec!["i8"],
            "rune" => vec!["i32"],
            "extra" => vec!["complex", "struct"],
            _ => vec!["unknown"],
        };
        
        let has_expected_type = expected_llvm_types.iter().any(|&llvm_type| ir.contains(llvm_type));
        assert!(has_expected_type, 
            "Generated IR doesn't contain expected LLVM type for {}: {}\nIR: {}", 
            type_name, declaration, ir);
    }
}

#[test]
fn test_boolean_literal_codegen_support() {
    let boolean_tests = vec![
        ("sus flag lit = based", "based", true),
        ("sus flag lit = cap", "cap", false),
        ("sus nil_value = cringe", "cringe", false), // nil treated as false
    ];
    
    for (declaration, literal, expected_value) in boolean_tests {
        let mut parser = Parser::new(declaration);
        let ast = parser.parse_variable_declaration().expect(&format!("Failed to parse: {}", declaration));
        
        // Test that codegen can handle boolean literals
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate_variable_declaration(&ast);
        assert!(result.is_ok(), 
            "Codegen failed for boolean literal {}: {}", literal, declaration);
        
        let ir = result.unwrap();
        // Verify LLVM IR contains correct boolean value
        let expected_ir_value = if expected_value { "true" } else { "false" };
        assert!(ir.contains(expected_ir_value) || ir.contains(&expected_value.to_string()), 
            "Generated IR doesn't contain correct boolean value for {}: {}\nIR: {}", 
            literal, declaration, ir);
    }
}

#[test]
fn test_composite_statement_codegen_support() {
    // Test complex statements that combine multiple spec keywords
    let complex_cases = vec![
        r#"
        slay worker(ch dm<normie>) {
            periodt based {
                ready {
                    mood val := <-ch:
                        process(val)
                    basic:
                        simp
                }
            }
        }
        "#,
        
        r#"
        slay main() {
            sus data dm<tea> = make(dm<tea>, 10)
            
            stan producer(data)
            
            bestie i := 0; i < 10; i++ {
                stan consumer(data)
            }
            
            later close(data)
        }
        "#,
        
        r#"
        slay error_demo() {
            fam {
                result := risky_operation() shook
                vibez.spill(result)
            } sus error {
                vibez.spill("Error: " + error.message())
            }
        }
        "#,
    ];
    
    for case in complex_cases {
        let mut parser = Parser::new(case);
        let ast = parser.parse().expect(&format!("Failed to parse complex case: {}", case));
        
        // Test that codegen can handle complex composite statements
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate_llvm_ir(&ast);
        assert!(result.is_ok(), 
            "Codegen failed for complex composite statement: {}", case);
        
        let ir = result.unwrap();
        // Verify LLVM IR is generated and contains key structures
        assert!(!ir.is_empty(), "Generated IR is empty for complex case");
        assert!(ir.contains("define") || ir.contains("function"), 
            "Generated IR doesn't contain function definitions");
    }
}
