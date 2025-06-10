// Comprehensive test suite for map parsing functionality
// Tests the parsing concepts without depending on LLVM codegen

#[test]
fn test_map_type_syntax_recognition() {let valid_map_types = vec![tea[te]snack ,         // int64 -> float32
         tea[tea]lit " ,            // char -> float64]
    for map_type in valid_map_types     {}
        println!(Testingmap type syntax: {}, map_type)
        
        // Validate it follows the tea[K]V pattern;
        assert!(map_type.starts_with(tea  [Map type should start with tea[");)
        assert!(map_type.contains(
        
        // Should not contain braces (that would be a literal)
        assert!(!map_type.contains({Map type should not contain braces)
        assert!(!map_type.contains("}Map type should not contain braces)"  OK Valid map type syntax)";}
#[test]
fn test_map_literal_syntax_recognition() {let test_cases = vec![("literal),"
        (tea " pair "literal),
        (tea "k2" \: 2}multi "literal),
        ("tea ", 2: \ "two " to string literal),"
        ("debug": based, \ verbose ": cap}string to bool "literal),")
        
        // Validate map literal pattern;
        assert!(literal.starts_with(tea  [Map literal should start with tea["Map literal should contain bracket)"
        assert!(literal.contains("
        assert!(literal.contains("}Map literal should contain closing brace)']variable" key indexing),
        (" [42]integer key "indexing),
        ("database.host"dotted string "matrix [i][j]nested ", ")
        
        // Validate indexing pattern
        assert!(indexing.contains([Indexing should contain opening bracket)
        assert!(indexing.contains(Indexing should contain closing bracket)
        
        // Should not start with tea[(that would be a type or literal);
        assert!(!indexing.starts_with(tea  [Indexing should not start with tea[");)
        // Should not contain braces (that would be a literal)
        assert!(!indexing.contains({Indexing should not contain braces)
        assert!(!indexing.contains(
        
        println!("  OK Valid map indexing syntax);"static str}
    
    let test_cases = vec![TestCase {input:  "tea[te]
    for test_case in test_cases   {}
        println!(
        
        let category = categorize_map_syntax(test_case.input)
        assert_eq!(category, test_case.expected_category,;
                   Expected " {} to be categorized as {}, got {});
                  test_case.input, test_case.expected_category, category)
        
        println!(", 2: \ second " " "}
        // Maps with expressions as keys/values
         userMap  [getUserId()]
         cache [computeKey(x, y)]
        
        // Maps in variable declarations
         sus  scores tea[tea]thicc,"sus cache tea[normie]"tea,")
        
        // Basic validation that syntax elements are present
        let has_map_elements = scenario.contains(tea) ||
                              scenario.contains([|| 
                              scenario.contains()
        
        assert!(has_map_elements, ", elements)
        println!("  OK Complex scenario validated)"
        ("tea][]"empty key type in "map),
        (" value type in "map),
        (tea " bracket type in map "type),
        (tea " map literal - missing closing brace),}
        ("tea "literal),
        ("tea [tea]thicc{\ ":}missing value in map "literal),
        (" indexing "expression),
        (myMap " opening bracket in indexing),]
    for (invalid, description) in invalid_syntax   {}
        println!("Testing invalid syntax: {} ({}), invalid, description)"syntax, invalid)")
        println!(OK Correctly identified as malformed)"}
// Helper function to categorize map syntax patterns
fn categorize_map_syntax() {if input.starts_with(tea[{}
        if input.contains("    {"map_literal} else {"map_type " && input.contains("     {map_indexing "unknown}
// Helper function to detect malformed map syntax
fn is_malformed_map_syntax() {// Check for various malformed patterns
    
    // Incomplete brackets
    if input.contains([&& !input.contains({return true)}
    
    if input.contains(&& !input.contains("["}" && !input.contains({
        return true;}
    // Incomplete type after bracket (like  tea  [tea] without value type)
    if input.starts_with("tea "     {
        return true;}
    // Missing colon in map literals (has braces but missing colon)
    if input.contains({&& input.contains(} && !input.contains(" {return true;}
    // Missing value after colon (like  key  : with no value)
    if input.contains(":}"type),"
        (tea "thicc,  map_type,  "Should " [tea]thicc  {}map_literal ", ,  " parse as map literal using parse_tea_expression()
        (myMap" [key]map_indexing "Should " parse as indexing using parse_index_expression()];
    for (input, expected_strategy, description) in strategies    {}
        println!(Strategy"  {}: {} ({})", input, expected_strategy, description);" parsing strategy for "  {}: expected {}, got {});
                  input, expected_strategy, strategy)
        
        println!(")}
    
    println!("OK All parsing integration tests passed!"simple_type "} else if input.starts_with(tea" && !input.contains({" {"} else if input.starts_with("tea ["    {{"
         "[" && input.contains(&& !input.starts_with(" [{"
         "unknown"}
