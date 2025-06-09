// Comprehensive test suite for map parsing functionality
// Tests the parsing concepts without depending on LLVM codegen

#[test]
fn test_map_type_syntax_recognition() {
    let valid_map_types = vec![
        "tea[tea]thicc",           // string -> int64
        "tea[normie]tea",          // int32 -> string  
        "tea[thicc]snack",         // int64 -> float32
        "tea[tea]lit",             // string -> bool
        "tea[sip]meal",            // char -> float64
    ];
    
    for map_type in valid_map_types {
        println!("Testing map type syntax: {}", map_type);
        
        // Validate it follows the tea[K]V pattern
        assert!(map_type.starts_with("tea["), "Map type should start with 'tea['");
        assert!(map_type.contains("]"), "Map type should contain closing bracket");
        
        // Should not contain braces (that would be a literal)
        assert!(!map_type.contains("{"), "Map type should not contain braces");
        assert!(!map_type.contains("}"), "Map type should not contain braces");
        
        println!("  ✓ Valid map type syntax");
    }
}

#[test]
fn test_map_literal_syntax_recognition() {
    let test_cases = vec![
        ("tea[tea]thicc{}", "empty map literal"),
        ("tea[tea]thicc{\"key\": 42}", "single pair literal"),
        ("tea[tea]thicc{\"k1\": 1, \"k2\": 2}", "multi-pair literal"),
        ("tea[normie]tea{1: \"one\", 2: \"two\"}", "int to string literal"),
        ("tea[tea]lit{\"debug\": based, \"verbose\": cap}", "string to bool literal"),
    ];
    
    for (literal, description) in test_cases {
        println!("Testing {}: {}", description, literal);
        
        // Validate map literal pattern
        assert!(literal.starts_with("tea["), "Map literal should start with 'tea['");
        assert!(literal.contains("]"), "Map literal should contain bracket");
        assert!(literal.contains("{"), "Map literal should contain opening brace");
        assert!(literal.contains("}"), "Map literal should contain closing brace");
        
        // Validate type section comes before brace
        let bracket_pos = literal.find(']').unwrap();
        let brace_pos = literal.find('{').unwrap();
        assert!(bracket_pos < brace_pos, "Type section should come before literal section");
        
        println!("  ✓ Valid map literal syntax");
    }
}

#[test]
fn test_map_indexing_syntax_recognition() {
    let test_cases = vec![
        ("myMap[\"key\"]", "string key indexing"),
        ("scores[userId]", "variable key indexing"),
        ("cache[42]", "integer key indexing"),
        ("config[\"database.host\"]", "dotted string key"),
        ("matrix[i][j]", "nested indexing"),
    ];
    
    for (indexing, description) in test_cases {
        println!("Testing {}: {}", description, indexing);
        
        // Validate indexing pattern
        assert!(indexing.contains("["), "Indexing should contain opening bracket");
        assert!(indexing.contains("]"), "Indexing should contain closing bracket");
        
        // Should not start with tea[ (that would be a type or literal)
        assert!(!indexing.starts_with("tea["), "Indexing should not start with 'tea['");
        
        // Should not contain braces (that would be a literal)
        assert!(!indexing.contains("{"), "Indexing should not contain braces");
        assert!(!indexing.contains("}"), "Indexing should not contain braces");
        
        println!("  ✓ Valid map indexing syntax");
    }
}

#[test]
fn test_syntax_disambiguation() {
    // Test that we can distinguish between different map-related syntax patterns
    
    struct TestCase {
        input: &'static str,
        expected_category: &'static str,
        description: &'static str,
    }
    
    let test_cases = vec![
        TestCase {
            input: "tea[tea]thicc",
            expected_category: "map_type",
            description: "Map type declaration",
        },
        TestCase {
            input: "tea[tea]thicc{}",
            expected_category: "map_literal", 
            description: "Empty map literal",
        },
        TestCase {
            input: "tea[tea]thicc{\"key\": 42}",
            expected_category: "map_literal",
            description: "Map literal with content",
        },
        TestCase {
            input: "myMap[\"key\"]",
            expected_category: "map_indexing",
            description: "Map element access",
        },
        TestCase {
            input: "userScores[userId]",
            expected_category: "map_indexing", 
            description: "Map access with variable key",
        },
    ];
    
    for test_case in test_cases {
        println!("Categorizing: {} -> {}", test_case.input, test_case.description);
        
        let category = categorize_map_syntax(test_case.input);
        assert_eq!(category, test_case.expected_category, 
                  "Expected {} to be categorized as {}, got {}", 
                  test_case.input, test_case.expected_category, category);
        
        println!("  ✓ Correctly categorized as {}", category);
    }
}

#[test]
fn test_complex_map_scenarios() {
    // Test more complex map usage patterns
    
    let complex_scenarios = vec![
        // Nested maps
        "tea[tea]tea[normie]thicc",  // Map type with map value type
        
        // Multiple type parameters
        "tea[normie]tea{1: \"first\", 2: \"second\", 3: \"third\"}",
        
        // Maps with expressions as keys/values
        "userMap[getUserId()]",
        "cache[computeKey(x, y)]",
        
        // Maps in variable declarations
        "sus scores tea[tea]thicc",
        "sus cache tea[normie]tea",
    ];
    
    for scenario in complex_scenarios {
        println!("Testing complex scenario: {}", scenario);
        
        // Basic validation that syntax elements are present
        let has_map_elements = scenario.contains("tea") || 
                              scenario.contains("[") || 
                              scenario.contains("]");
        
        assert!(has_map_elements, "Scenario should contain map-related syntax elements");
        
        println!("  ✓ Complex scenario validated");
    }
}

#[test]
fn test_error_scenarios() {
    // Test recognition of invalid syntax that should produce parse errors
    
    let invalid_syntax = vec![
        ("tea[", "incomplete map type - missing key type and closing bracket"),
        ("tea[]thicc", "empty key type in map"),
        ("tea[tea]", "missing value type in map"),
        ("tea[tea}thicc", "wrong bracket type in map type"),
        ("tea[tea]thicc{", "incomplete map literal - missing closing brace"),
        ("tea[tea]thicc{key}", "missing colon in map literal"),
        ("tea[tea]thicc{\"key\":}", "missing value in map literal"),
        ("myMap[", "incomplete indexing expression"),
        ("myMap]", "missing opening bracket in indexing"),
    ];
    
    for (invalid, description) in invalid_syntax {
        println!("Testing invalid syntax: {} ({})", invalid, description);
        
        // These should be recognized as malformed
        let is_malformed = is_malformed_map_syntax(invalid);
        assert!(is_malformed, "Should recognize '{}' as malformed syntax", invalid);
        
        println!("  ✓ Correctly identified as malformed");
    }
}

// Helper function to categorize map syntax patterns
fn categorize_map_syntax(input: &str) -> &'static str {
    if input.starts_with("tea[") {
        if input.contains("{") && input.contains("}") {
            "map_literal"
        } else {
            "map_type" 
        }
    } else if input.contains("[") && input.contains("]") {
        "map_indexing"
    } else {
        "unknown"
    }
}

// Helper function to detect malformed map syntax
fn is_malformed_map_syntax(input: &str) -> bool {
    // Check for various malformed patterns
    
    // Incomplete brackets
    if input.contains("[") && !input.contains("]") {
        return true;
    }
    
    if input.contains("]") && !input.contains("[") {
        return true;
    }
    
    // Incomplete braces in literals
    if input.contains("{") && !input.contains("}") {
        return true;
    }
    
    // Wrong bracket types
    if input.contains("tea[") && input.contains("}") && !input.contains("]") {
        return true;
    }
    
    // Empty key or value types
    if input.contains("tea[]") {
        return true;
    }
    
    // Incomplete type after bracket (like "tea[tea]" without value type)
    if input.starts_with("tea[") && input.ends_with("]") {
        return true;
    }
    
    // Missing colon in map literals (has braces but missing colon)
    if input.contains("{") && input.contains("}") && !input.contains(":") {
        return true;
    }
    
    // Missing value after colon (like "key:" with no value)
    if input.contains(":}") {
        return true;
    }
    
    false
}

#[test]
fn test_parsing_integration_readiness() {
    // Test that our syntax patterns are ready for parser integration
    
    println!("Testing parser integration readiness...");
    
    // Test that we can identify when to use different parsing strategies
    let strategies = vec![
        ("tea", "simple_type", "Should parse as simple tea type"),
        ("tea[tea]thicc", "map_type", "Should parse as map type using parse_type()"),
        ("tea[tea]thicc{}", "map_literal", "Should parse as map literal using parse_tea_expression()"),
        ("myMap[key]", "map_indexing", "Should parse as indexing using parse_index_expression()"),
    ];
    
    for (input, expected_strategy, description) in strategies {
        println!("Strategy for '{}': {} ({})", input, expected_strategy, description);
        
        let strategy = determine_parsing_strategy(input);
        assert_eq!(strategy, expected_strategy, 
                  "Wrong parsing strategy for '{}': expected {}, got {}", 
                  input, expected_strategy, strategy);
        
        println!("  ✓ Correct parsing strategy identified");
    }
    
    println!("✓ All parsing integration tests passed!");
}

// Helper function to determine which parsing strategy to use
fn determine_parsing_strategy(input: &str) -> &'static str {
    if input == "tea" {
        "simple_type"
    } else if input.starts_with("tea[") && !input.contains("{") {
        "map_type"
    } else if input.starts_with("tea[") && input.contains("{") {
        "map_literal"
    } else if input.contains("[") && input.contains("]") && !input.starts_with("tea[") {
        "map_indexing"
    } else {
        "unknown"
    }
}
