use std::io::Write;

fn main() {
    // Simulate what the test does
    let source = r#"nah x > 0 {
vibez.spill("positive")
}"#;
    
    println!("Source:");
    println!("{}", source);
    println!("=================");
    
    // Simple string processing like the formatter does
    let lines: Vec<String> = source.lines().map(|l| l.to_string()).collect();
    println!("Lines: {:?}", lines);
    
    // Check if it matches compact expectations
    let expected_line1 = "nah x>0{";
    let expected_line2 = "  vibez.spill(\"positive\")";
    
    println!("Expected line 1: '{}'", expected_line1);
    println!("Expected line 2: '{}'", expected_line2);
    
    // Check if the compact logic works
    let mut test_line = "nah x > 0 {".to_string();
    println!("Original test line: '{}'", test_line);
    
    // Remove spaces around operators
    test_line = test_line.replace(" > ", ">");
    println!("After removing operator spaces: '{}'", test_line);
    
    // Remove spaces before braces
    test_line = test_line.replace(" {", "{");
    println!("After removing brace spaces: '{}'", test_line);
    
    println!("Does it match expected? {}", test_line == expected_line1);
}
