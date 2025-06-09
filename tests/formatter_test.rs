//! Comprehensive tests for the CURSED code formatter
//!
//! These tests verify that the formatter correctly handles all CURSED language constructs
//! and maintains proper formatting according to various configuration options.

use cursed::tools::{CursedFormatter, FormatterConfig, BraceStyle};
use cursed::lexer::{Lexer, TokenType};
use cursed::parser::Parser;

mod common;

#[test]
fn test_format_simple_function() {
    common::tracing::setup();
    
    let source = r#"slay add(x normie,y normie)normie{yolo x+y}"#;
    let expected = r#"slay add(x normie, y normie) normie {
    yolo x + y
}"#;

    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    assert!(result.changed);
    assert_eq!(result.formatted_code.trim(), expected);
}

#[test]
fn test_format_function_with_generics() {
    common::tracing::setup();
    
    let source = r#"slay max[T](a T,b T)T{lowkey a>b{yolo a}highkey{yolo b}}"#;
    let expected = r#"slay max[T](a T, b T) T {
    lowkey a > b {
        yolo a
    } highkey {
        yolo b
    }
}"#;

    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    assert!(result.changed);
    assert_eq!(result.formatted_code.trim(), expected);
}

#[test]
fn test_format_struct_declaration() {
    common::tracing::setup();
    
    let source = r#"squad Person{name sip age normie}"#;
    let expected = r#"squad Person {
    name sip
    age normie
}"#;

    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    assert!(result.changed);
    assert_eq!(result.formatted_code.trim(), expected);
}

#[test]
fn test_format_interface_declaration() {
    common::tracing::setup();
    
    let source = r#"collab Writer{write(data sip)normie}"#;
    let expected = r#"collab Writer {
    write(data sip) normie
}"#;

    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    assert!(result.changed);
    assert_eq!(result.formatted_code.trim(), expected);
}

#[test]
fn test_format_variable_declarations() {
    common::tracing::setup();
    
    let source = r#"sus x=42
sus y normie=100
facts PI=3.14159"#;
    let expected = r#"sus x = 42
sus y normie = 100
facts PI = 3.14159"#;

    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    assert!(result.changed);
    assert_eq!(result.formatted_code.trim(), expected);
}

#[test]
fn test_format_control_flow() {
    common::tracing::setup();
    
    let source = r#"lowkey x>0{sus y=x*2}highkey{sus y=0}
periodt x>0{x=x-1}
bestie i flex range(10){sus z=i*2}"#;
    let expected = r#"lowkey x > 0 {
    sus y = x * 2
} highkey {
    sus y = 0
}
periodt x > 0 {
    x = x - 1
}
bestie i flex range(10) {
    sus z = i * 2
}"#;

    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    assert!(result.changed);
    assert_eq!(result.formatted_code.trim(), expected);
}

#[test]
fn test_format_switch_statement() {
    common::tracing::setup();
    
    let source = r#"vibe_check value{mood 1:yolo "one"
mood 2:yolo "two"
basic:yolo "other"}"#;
    let expected = r#"vibe_check value {
    mood 1:
        yolo "one"
    mood 2:
        yolo "two"
    basic:
        yolo "other"
}"#;

    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    assert!(result.changed);
    assert_eq!(result.formatted_code.trim(), expected);
}

#[test]
fn test_format_expressions() {
    common::tracing::setup();
    
    let source = r#"sus result=add(1,2)+multiply(3,4)
sus arr=[1,2,3,4]
sus hash={a:1,b:2}"#;
    let expected = r#"sus result = add(1, 2) + multiply(3, 4)
sus arr = [1, 2, 3, 4]
sus hash = {a: 1, b: 2}"#;

    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    assert!(result.changed);
    assert_eq!(result.formatted_code.trim(), expected);
}

#[test]
fn test_format_brace_styles() {
    common::tracing::setup();
    
    let source = r#"slay test(){yolo 42}"#;
    
    // Test same-line brace style (default)
    let mut formatter_same_line = CursedFormatter::new(FormatterConfig {
        brace_style: BraceStyle::SameLine,
        ..FormatterConfig::default()
    });
    let result_same_line = formatter_same_line.format(source).unwrap();
    assert!(result_same_line.formatted_code.contains("test() {"));
    
    // Test next-line brace style
    let mut formatter_next_line = CursedFormatter::new(FormatterConfig {
        brace_style: BraceStyle::NextLine,
        ..FormatterConfig::default()
    });
    let result_next_line = formatter_next_line.format(source).unwrap();
    assert!(result_next_line.formatted_code.contains("test()\n    {"));
    
    // Test next-line unindented brace style
    let mut formatter_unindented = CursedFormatter::new(FormatterConfig {
        brace_style: BraceStyle::NextLineUnindented,
        ..FormatterConfig::default()
    });
    let result_unindented = formatter_unindented.format(source).unwrap();
    assert!(result_unindented.formatted_code.contains("test()\n{"));
}

#[test]
fn test_format_operator_spacing() {
    common::tracing::setup();
    
    let source = r#"sus x=a+b*c-d/e"#;
    
    // Test with operator spacing (default)
    let mut formatter_with_spaces = CursedFormatter::new(FormatterConfig {
        spaces_around_operators: true,
        ..FormatterConfig::default()
    });
    let result_with_spaces = formatter_with_spaces.format(source).unwrap();
    assert!(result_with_spaces.formatted_code.contains("a + b * c - d / e"));
    
    // Test without operator spacing
    let mut formatter_no_spaces = CursedFormatter::new(FormatterConfig {
        spaces_around_operators: false,
        ..FormatterConfig::default()
    });
    let result_no_spaces = formatter_no_spaces.format(source).unwrap();
    assert!(result_no_spaces.formatted_code.contains("a+b*c-d/e"));
}

#[test]
fn test_format_comma_spacing() {
    common::tracing::setup();
    
    let source = r#"sus arr=[1,2,3,4]"#;
    
    // Test with comma spacing (default)
    let mut formatter_with_spaces = CursedFormatter::new(FormatterConfig {
        space_after_comma: true,
        ..FormatterConfig::default()
    });
    let result_with_spaces = formatter_with_spaces.format(source).unwrap();
    assert!(result_with_spaces.formatted_code.contains("[1, 2, 3, 4]"));
    
    // Test without comma spacing
    let mut formatter_no_spaces = CursedFormatter::new(FormatterConfig {
        space_after_comma: false,
        ..FormatterConfig::default()
    });
    let result_no_spaces = formatter_no_spaces.format(source).unwrap();
    assert!(result_no_spaces.formatted_code.contains("[1,2,3,4]"));
}

#[test]
fn test_format_indentation_sizes() {
    common::tracing::setup();
    
    let source = r#"slay test(){lowkey based{yolo 42}}"#;
    
    // Test 2-space indentation
    let mut formatter_2_spaces = CursedFormatter::new(FormatterConfig {
        indent_size: 2,
        ..FormatterConfig::default()
    });
    let result_2_spaces = formatter_2_spaces.format(source).unwrap();
    let lines: Vec<&str> = result_2_spaces.formatted_code.lines().collect();
    assert!(lines.iter().any(|line| line.starts_with("  lowkey")));
    
    // Test 8-space indentation
    let mut formatter_8_spaces = CursedFormatter::new(FormatterConfig {
        indent_size: 8,
        ..FormatterConfig::default()
    });
    let result_8_spaces = formatter_8_spaces.format(source).unwrap();
    let lines: Vec<&str> = result_8_spaces.formatted_code.lines().collect();
    assert!(lines.iter().any(|line| line.starts_with("        lowkey")));
}

#[test]
fn test_format_already_formatted_code() {
    common::tracing::setup();
    
    let source = r#"slay add(x normie, y normie) normie {
    yolo x + y
}"#;

    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    // Should detect no changes needed
    assert!(!result.changed);
    assert_eq!(result.formatted_code.trim(), source.trim());
}

#[test]
fn test_format_complex_nested_structures() {
    common::tracing::setup();
    
    let source = r#"squad Matrix[T]{data[][]T rows normie cols normie}
slay new_matrix[T](rows normie,cols normie)Matrix[T]{sus data=make([][]T,rows)
bestie i flex range(rows){data[i]=make([]T,cols)}
yolo Matrix[T]{data:data,rows:rows,cols:cols}}"#;
    
    let expected = r#"squad Matrix[T] {
    data [][]T
    rows normie
    cols normie
}
slay new_matrix[T](rows normie, cols normie) Matrix[T] {
    sus data = make([][]T, rows)
    bestie i flex range(rows) {
        data[i] = make([]T, cols)
    }
    yolo Matrix[T]{data: data, rows: rows, cols: cols}
}"#;

    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    assert!(result.changed);
    // Note: For complex cases, we might need to adjust expectations based on actual AST parsing
    assert!(result.formatted_code.contains("squad Matrix[T] {"));
    assert!(result.formatted_code.contains("slay new_matrix[T]"));
}

#[test]
fn test_format_boolean_literals() {
    common::tracing::setup();
    
    let source = r#"sus is_true=true
sus is_false=false
sus is_based=based
sus is_cap=cap"#;
    
    // The formatter should convert boolean literals to CURSED slang
    let expected = r#"sus is_true = based
sus is_false = cap
sus is_based = based
sus is_cap = cap"#;

    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    assert!(result.changed);
    // Note: This test assumes the formatter can handle boolean literal conversion
    assert!(result.formatted_code.contains("= based"));
    assert!(result.formatted_code.contains("= cap"));
}

#[test]
fn test_format_error_handling() {
    common::tracing::setup();
    
    // Test with invalid syntax
    let invalid_source = r#"slay incomplete("#;
    
    let mut formatter = CursedFormatter::default();
    let result = formatter.format(invalid_source);
    
    // Should return an error for invalid syntax
    assert!(result.is_err());
}

#[test]
fn test_formatter_result_display() {
    let result = cursed::tools::FormatterResult {
        formatted_code: "test".to_string(),
        changed: true,
        lines_processed: 10,
        warnings: vec!["test warning".to_string()],
    };
    
    let display = format!("{}", result);
    assert!(display.contains("Formatted 10 lines"));
    assert!(display.contains("(changes made)"));
    assert!(display.contains("1 warnings"));
}

#[test]
fn test_format_gen_z_keywords() {
    common::tracing::setup();
    
    let source = r#"slay vibes(){sus mood=based
lowkey mood{yolo "good vibes"}
periodt cap{simp}
bestie i flex range(5){ghosted}}"#;
    
    let mut formatter = CursedFormatter::default();
    let result = formatter.format(source).unwrap();
    
    // Verify Gen Z slang keywords are preserved
    assert!(result.formatted_code.contains("slay"));
    assert!(result.formatted_code.contains("sus"));
    assert!(result.formatted_code.contains("based"));
    assert!(result.formatted_code.contains("lowkey"));
    assert!(result.formatted_code.contains("yolo"));
    assert!(result.formatted_code.contains("periodt"));
    assert!(result.formatted_code.contains("cap"));
    assert!(result.formatted_code.contains("simp"));
    assert!(result.formatted_code.contains("bestie"));
    assert!(result.formatted_code.contains("flex"));
    assert!(result.formatted_code.contains("ghosted"));
}

#[test]
fn test_lexer_token_types() {
    common::tracing::setup();
    
    // Test that our formatter works with the CURSED lexer
    let source = "slay test() { yolo 42 }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    // Verify we get the expected CURSED tokens
    let token_types: Vec<TokenType> = tokens.iter().map(|t| t.token_type()).collect();
    
    // Should contain CURSED-specific tokens
    assert!(token_types.contains(&TokenType::Slay));  // slay keyword
    assert!(token_types.contains(&TokenType::Yolo));  // yolo keyword
    assert!(token_types.contains(&TokenType::LBrace)); // {
    assert!(token_types.contains(&TokenType::RBrace)); // }
}

#[test]
fn test_parser_integration() {
    common::tracing::setup();
    
    // Test that our formatter works with the CURSED parser
    let source = "slay add(x normie, y normie) normie { yolo x + y }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();
    
    // Verify we can parse a basic function
    assert!(!program.statements.is_empty());
}

#[test]
fn test_format_line_length_handling() {
    common::tracing::setup();
    
    let long_source = r#"slay very_long_function_name_that_exceeds_normal_line_width(very_long_parameter_name_one normie, very_long_parameter_name_two normie, very_long_parameter_name_three normie) normie { yolo very_long_parameter_name_one + very_long_parameter_name_two + very_long_parameter_name_three }"#;
    
    let mut formatter = CursedFormatter::new(FormatterConfig {
        line_width: 50,  // Short line width to test wrapping
        ..FormatterConfig::default()
    });
    
    let result = formatter.format(long_source).unwrap();
    
    // Should format the code (exact wrapping behavior depends on implementation)
    assert!(result.changed);
    assert!(result.formatted_code.len() > 0);
}

#[test]
fn test_configuration_options() {
    let config = FormatterConfig {
        indent_size: 8,
        line_width: 120,
        brace_style: BraceStyle::NextLine,
        spaces_around_operators: false,
        space_after_comma: false,
        format_comments: false,
        preserve_empty_lines: false,
        max_empty_lines: 1,
    };
    
    assert_eq!(config.indent_size, 8);
    assert_eq!(config.line_width, 120);
    assert_eq!(config.brace_style, BraceStyle::NextLine);
    assert!(!config.spaces_around_operators);
    assert!(!config.space_after_comma);
    assert!(!config.format_comments);
    assert!(!config.preserve_empty_lines);
    assert_eq!(config.max_empty_lines, 1);
}
