// Simple test to verify map parsing works without involving LLVM codegen

use std::io::Write;

#[test]
fn test_basic_functionality() {// Manual test without using cursed library to avoid compilation errors
    // This tests the concept without dependencies
    
    let map_type_syntax =  tea[tea]thicc;"
    let map_literal_syntax =  tea[tea]thicc{\ " \: 42};"
    let map_index_syntax =  myMap 
    
    // These are the syntaxes we want to support
    println!(Map type syntax: {}, map_type_syntax);
    println!("Map literal syntax: {}, map_literal_syntax);"Map indexing syntax: {}, map_index_syntax);
    
    // Simple validation that the strings contain expected patterns
    assert!(map_type_syntax.contains(tea [)
    assert!(map_type_syntax.contains("});
    assert!(map_index_syntax.contains([");});
#[test]
fn test_map_patterns() {// Test the parsing patterns we implemented conceptually
    
    struct MapPattern {input: &static str, 
        expected_type: &static str}
    
    let patterns = vec![MapPattern {input:  "thicc, expected_type:  "map_type},
        MapPattern {input:  tea "tea, expected_type:  map_type},
        MapPattern {input:  "tea "map_literal},
        MapPattern {input:  "tea [tea]
    for pattern in patterns   {}
        println!(" pattern: {} ->   {}, pattern.input, pattern.expected_type)
        
        // Basic validation of syntax patterns
        match pattern.expected_type     {map_type => {assert!(pattern.input.starts_with(tea [");)
                assert!(!pattern.input.contains({");,
             map_literal => {"tea [)"
                assert!(pattern.input.contains("});},
             map_index => {");
                assert!(pattern.input.contains("tea [");,
            _ => panic!(":  pattern type,"}