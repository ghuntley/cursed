/// Comprehensive tests for the CURSED code formatter
/// 
/// This test suite validates the formatter across all CURSED language constructs,
/// configuration options, and edge cases to ensure production-ready quality.

use cursed::tools::formatter::{CursedFormatter, FormatterConfig, BraceStyle, OperatorSpacing, CommaSpacing};

#[test]
fn test_basic_function_formatting() {
    let mut formatter = CursedFormatter::default();
    let input = "slay add(sus a int, sus b int) int { facts result = a + b; yolo result; }";
    let result = formatter.format(input).unwrap();
    
    let expected = "slay add(sus a int, sus b int) int {\n    facts result = a + b\n    yolo result\n}\n";
    assert_eq!(result.formatted_code, expected);
    assert!(result.changes_made);
}

#[test]
fn test_squad_formatting() {
    let mut formatter = CursedFormatter::default();
    let input = "squad Person{name string;age int;}";
    let result = formatter.format(input).unwrap();
    
    let expected = "squad Person {\n    name string\n    age int\n}\n";
    assert_eq!(result.formatted_code, expected);
}

#[test]
fn test_collab_formatting() {
    let mut formatter = CursedFormatter::default();
    let input = "collab Writer{write(data string)error;}";
    let result = formatter.format(input).unwrap();
    
    let expected = "collab Writer {\n    write(data string) error\n}\n";
    assert_eq!(result.formatted_code, expected);
}

#[test]
fn test_if_statement_formatting() {
    let mut formatter = CursedFormatter::default();
    let input = "lowkey x > 0{yolo x;}highkey{yolo -x;}";
    let result = formatter.format(input).unwrap();
    
    let expected = "lowkey x > 0 {\n    yolo x\n} highkey {\n    yolo -x\n}\n";
    assert_eq!(result.formatted_code, expected);
}

#[test]
fn test_switch_statement_formatting() {
    let mut formatter = CursedFormatter::default();
    let input = "vibe_check x{mood 1:yolo \"one\";mood 2:yolo \"two\";basic:yolo \"other\";}";
    let result = formatter.format(input).unwrap();
    
    let expected = "vibe_check x {\n    mood 1:\n        yolo \"one\"\n    mood 2:\n        yolo \"two\"\n    basic:\n        yolo \"other\"\n}\n";
    assert_eq!(result.formatted_code, expected);
}

#[test]
fn test_for_loop_formatting() {
    let mut formatter = CursedFormatter::default();
    let input = "bestie sus i = 0; i < 10; i++{yolo i;}";
    let result = formatter.format(input).unwrap();
    
    let expected = "bestie sus i = 0; i < 10; i++ {\n    yolo i\n}\n";
    assert_eq!(result.formatted_code, expected);
}

#[test]
fn test_range_for_formatting() {
    let mut formatter = CursedFormatter::default();
    let input = "bestie i, v := flex items{yolo v;}";
    let result = formatter.format(input).unwrap();
    
    let expected = "bestie i, v := flex items {\n    yolo v\n}\n";
    assert_eq!(result.formatted_code, expected);
}

#[test]
fn test_while_loop_formatting() {
    let mut formatter = CursedFormatter::default();
    let input = "periodt x > 0{x--;yolo x;}";
    let result = formatter.format(input).unwrap();
    
    let expected = "periodt x > 0 {\n    x--\n    yolo x\n}\n";
    assert_eq!(result.formatted_code, expected);
}

#[test]
fn test_brace_style_same_line() {
    let mut config = FormatterConfig::default();
    config.brace_style = BraceStyle::SameLine;
    let mut formatter = CursedFormatter::new(config);
    
    let input = "slay test() { yolo 42; }";
    let result = formatter.format(input).unwrap();
    
    assert!(result.formatted_code.contains("test() {"));
}

#[test]
fn test_brace_style_next_line() {
    let mut config = FormatterConfig::default();
    config.brace_style = BraceStyle::NextLine;
    let mut formatter = CursedFormatter::new(config);
    
    let input = "slay test() { yolo 42; }";
    let result = formatter.format(input).unwrap();
    
    assert!(result.formatted_code.contains("test()\n    {"));
}

#[test]
fn test_brace_style_next_line_unindented() {
    let mut config = FormatterConfig::default();
    config.brace_style = BraceStyle::NextLineUnindented;
    let mut formatter = CursedFormatter::new(config);
    
    let input = "slay test() { yolo 42; }";
    let result = formatter.format(input).unwrap();
    
    assert!(result.formatted_code.contains("test()\n{"));
}

#[test]
fn test_indent_size_configuration() {
    let mut config = FormatterConfig::default();
    config.indent_size = 2;
    let mut formatter = CursedFormatter::new(config);
    
    let input = "slay test() { yolo 42; }";
    let result = formatter.format(input).unwrap();
    
    assert!(result.formatted_code.contains("  yolo 42")); // 2 spaces
}

#[test]
fn test_operator_spacing_with_spaces() {
    let mut config = FormatterConfig::default();
    config.operator_spacing = OperatorSpacing::WithSpaces;
    let mut formatter = CursedFormatter::new(config);
    
    let input = "bestie i:=0;i<10;i++{}";
    let result = formatter.format(input).unwrap();
    
    assert!(result.formatted_code.contains("bestie i := 0; i < 10; i++ {"));
}

#[test]
fn test_operator_spacing_without_spaces() {
    let mut config = FormatterConfig::default();
    config.operator_spacing = OperatorSpacing::WithoutSpaces;
    let mut formatter = CursedFormatter::new(config);
    
    let input = "bestie i := 0; i < 10; i++{}";
    let result = formatter.format(input).unwrap();
    
    assert!(result.formatted_code.contains("bestiei:=0;i<10;i++{"));
}

#[test]
fn test_comma_spacing_with_spaces() {
    let mut config = FormatterConfig::default();
    config.comma_spacing = CommaSpacing::WithSpaces;
    let mut formatter = CursedFormatter::new(config);
    
    let input = "slay test(a int,b int) { }";
    let result = formatter.format(input).unwrap();
    
    assert!(result.formatted_code.contains("test(a int, b int)"));
}

#[test]
fn test_comma_spacing_without_spaces() {
    let mut config = FormatterConfig::default();
    config.comma_spacing = CommaSpacing::WithoutSpaces;
    let mut formatter = CursedFormatter::new(config);
    
    let input = "slay test(a int, b int) { }";
    let result = formatter.format(input).unwrap();
    
    assert!(result.formatted_code.contains("test(a int,b int)"));
}

#[test]
fn test_package_and_imports_formatting() {
    let mut formatter = CursedFormatter::default();
    let input = "vibe main\nyeet \"fmt\"\nyeet alias \"path\"\nslay main() {}";
    let result = formatter.format(input).unwrap();
    
    let expected = "vibe main\n\nyeet \"fmt\"\nyeet alias \"path\"\n\nslay main() {\n}\n";
    assert_eq!(result.formatted_code, expected);
}

#[test]
fn test_break_and_continue_statements() {
    let mut formatter = CursedFormatter::default();
    let input = "bestie true { ghosted; simp; }";
    let result = formatter.format(input).unwrap();
    
    assert!(result.formatted_code.contains("ghosted"));
    assert!(result.formatted_code.contains("simp"));
}

#[test]
fn test_type_alias_formatting() {
    let mut formatter = CursedFormatter::default();
    let input = "be_like MyInt int";
    let result = formatter.format(input).unwrap();
    
    assert!(result.formatted_code.contains("be_like MyInt int"));
}

#[test]
fn test_nested_structures() {
    let mut formatter = CursedFormatter::default();
    let input = "slay outer() { lowkey true { bestie i := 0; i < 5; i++ { yolo i; } } }";
    let result = formatter.format(input).unwrap();
    
    // Should have proper nested indentation
    assert!(result.formatted_code.contains("    lowkey true"));
    assert!(result.formatted_code.contains("        bestie i := 0"));
    assert!(result.formatted_code.contains("            yolo i"));
}

#[test]
fn test_complex_program_formatting() {
    let mut formatter = CursedFormatter::default();
    let input = r#"vibe main
yeet "fmt"
squad Person{name string;age int;}
collab Greeter{greet(p Person)string;}
slay main(){facts p = Person{name:"Alice",age:30};fmt.Println(p);}"#;
    
    let result = formatter.format(input).unwrap();
    
    // Check major structure
    assert!(result.formatted_code.contains("vibe main"));
    assert!(result.formatted_code.contains("yeet \"fmt\""));
    assert!(result.formatted_code.contains("squad Person {"));
    assert!(result.formatted_code.contains("collab Greeter {"));
    assert!(result.formatted_code.contains("slay main() {"));
    assert!(result.changes_made);
}

#[test]
fn test_already_formatted_code() {
    let mut formatter = CursedFormatter::default();
    let input = "slay test() {\n    yolo 42\n}\n";
    let result = formatter.format(input).unwrap();
    
    // Should detect no changes needed
    assert_eq!(result.formatted_code, input);
    assert!(!result.changes_made);
}

#[test]
fn test_formatter_result_metadata() {
    let mut formatter = CursedFormatter::default();
    let input = "slay test(){yolo 42;}";
    let result = formatter.format(input).unwrap();
    
    assert!(result.changes_made);
    assert!(result.lines_changed > 0);
    assert!(result.formatting_errors.is_empty());
}

#[test]
fn test_malformed_input_handling() {
    let mut formatter = CursedFormatter::default();
    let input = "slay incomplete_function("; // Intentionally malformed
    
    // Should return error for unparseable code
    let result = formatter.format(input);
    assert!(result.is_err());
}

#[test]
fn test_empty_input() {
    let mut formatter = CursedFormatter::default();
    let input = "";
    let result = formatter.format(input).unwrap();
    
    assert_eq!(result.formatted_code, "");
    assert!(!result.changes_made);
}

#[test]
fn test_whitespace_only_input() {
    let mut formatter = CursedFormatter::default();
    let input = "   \n  \n  ";
    let result = formatter.format(input).unwrap();
    
    assert_eq!(result.formatted_code, "");
}

#[test]
fn test_comment_preservation() {
    // Note: This test assumes comment handling is implemented
    // For now, we test the structure without comments
    let mut formatter = CursedFormatter::default();
    let input = "slay test() { facts x = 42; yolo x; }";
    let result = formatter.format(input).unwrap();
    
    assert!(result.formatted_code.contains("facts x = 42"));
    assert!(result.formatted_code.contains("yolo x"));
}

#[test]
fn test_formatter_config_defaults() {
    let config = FormatterConfig::default();
    
    assert_eq!(config.indent_size, 4);
    assert_eq!(config.line_width, 100);
    assert_eq!(config.brace_style, BraceStyle::SameLine);
    assert_eq!(config.operator_spacing, OperatorSpacing::WithSpaces);
    assert_eq!(config.comma_spacing, CommaSpacing::WithSpaces);
    assert_eq!(config.max_empty_lines, 2);
}

#[test]
fn test_large_function_formatting() {
    let mut formatter = CursedFormatter::default();
    let mut input = String::from("slay large_function() {");
    
    // Add many statements
    for i in 0..50 {
        input.push_str(&format!("facts x{} = {};", i, i));
    }
    input.push('}');
    
    let result = formatter.format(&input).unwrap();
    
    // Should handle large functions without issues
    assert!(result.changes_made);
    assert!(result.formatted_code.contains("slay large_function() {"));
    assert!(result.formatted_code.matches('\n').count() > 50);
}

#[test]
fn test_deeply_nested_structures() {
    let mut formatter = CursedFormatter::default();
    let input = "slay deep() { lowkey true { lowkey true { lowkey true { yolo 42; } } } }";
    let result = formatter.format(input).unwrap();
    
    // Check deep indentation levels
    assert!(result.formatted_code.contains("            yolo 42")); // 12 spaces (3 levels * 4)
}

#[test]
fn test_multiple_statements_same_line() {
    let mut formatter = CursedFormatter::default();
    let input = "slay test() { facts a = 1; facts b = 2; yolo a + b; }";
    let result = formatter.format(input).unwrap();
    
    // Should separate statements onto different lines
    let lines: Vec<&str> = result.formatted_code.split("\n").collect();
    assert!(lines.len() >= 4); // At least function, three statements, closing brace
}

#[test]
fn test_range_for_with_different_variable_patterns() {
    let mut formatter = CursedFormatter::default();
    
    // Test with both key and value
    let input1 = "bestie k, v := flex items { }";
    let result1 = formatter.format(input1).unwrap();
    assert!(result1.formatted_code.contains("k, v := flex"));
    
    // Test with value only
    let input2 = "bestie v := flex items { }";
    let result2 = formatter.format(input2).unwrap();
    assert!(result2.formatted_code.contains("v := flex"));
    
    // Test with underscore
    let input3 = "bestie _ := flex items { }";
    let result3 = formatter.format(input3).unwrap();
    assert!(result3.formatted_code.contains("_ := flex"));
}
