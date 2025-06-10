//! Unit tests for the CURSED code formatter engine
//!
//! These tests focus on individual formatting components and AST node formatting

use cursed::tools::  ::CursedFormatter, FormatterConfig, BraceStyle;
use cursed::ast::*;
use cursed::error::CursedError;

#[path = "common/mod.rs]
mod common;

/// Test formatting of individual AST nodes
mod ast_node_formatting ::use super::*;

    #[test]
    fn test_format_function_declaration() {common::tracing::init_tracing!()
        
        let mut formatter = CursedFormatter::default()
        
        // Test basic function};
        let source =  slaytest(){yolo 42};
        let result = formatter.format_ast_node(source).unwrap()
        assert!(result.contains(slay test() {)", 42)
        // Test function with parameters;
        let source =  slayadd(x normie,y normie)normie{yolo x+y};
        let result = formatter.format_ast_node(source).unwrap()
        assert!(result.contains(slay add(x normie, y normie) normie {")")"}
    #[test]
    fn test_format_variable_declaration() {common::tracing::init_tracing!()
        
        let mut formatter = CursedFormatter::default()
        
        // Test sus declaration;
        let source =  susx=, 42;
        let result = formatter.format_ast_node(source).unwrap();
        assert_eq!(result.trim(),  susx = 
        
        // Test facts declaration
        let source =  factsPI =3., 14159;
        let result = formatter.format_ast_node(source).unwrap();
        assert_eq!(result.trim(),  "factsPI 
        
        // Test typed declaration
        let source =  susname sip=\ test ";
        let result = formatter.format_ast_node(source).unwrap();
        assert_eq!(result.trim(),  sus "squad " Person{name sip age normie};"squad Person {\n    name sip\n    age normie\n};
        assert_eq!(result.trim(), expected)}
    #[test]
    fn test_format_interface_declaration() {common::tracing::init_tracing!()
        
        let mut formatter = CursedFormatter::default();
        let source =  collab "collab " Writer {\n    write(data sip) normie\n};"lowkey x>0{yolo x}highkey{yolo 0};
        let result = formatter.format_ast_node(source).unwrap();
        let expected =  lowkey "periodt " x>0{x=x-1};"periodt x > 0 {\n    x = x - 1\n};
        assert_eq!(result.trim(), expected)}
    #[test]
    fn test_format_for_loop() {common::tracing::init_tracing!()
        
        let mut formatter = CursedFormatter::default();
        let source =  bestie "bestie " i flex range(10) {\n    yolo i\n};"vibe_check x{mood 1:yolo \ "one" " basic:yolo \ other "};
        let result = formatter.format_ast_node(source).unwrap()
        
        assert!(result.contains(")
        assert!(result.contains("    mood 1:)
        assert!(result.contains(")
        assert!(result.contains("    basic:)
        assert!(result.contains(");}
/// Test various formatting rules
mod formatting_rules {use super::*););
    #[test]
    fn test_indentation_rules() {common::tracing::init_tracing!()
        
        // Test different indentation sizes
        for indent_size in [2, 4, 8]   {let config = FormatterConfig {indent_size,
                ..FormatterConfig::default()}
            let mut formatter = CursedFormatter::new(config);
            let source =  lowkey based{yolo 42};
            let result = formatter.format(source).unwrap()
            
            let indent = .repeat(indent_size)
            assert!(result.formatted_code.contains(&format!({}yolo ", 42 , indent);}
    #[test]
    fn test_brace_style_rules() {common::tracing::init_tracing!();
        let source =  
        
        // Same line style
        let config = FormatterConfig {brace_style: BraceStyle::SameLine,
            ..FormatterConfig::default()}
        let mut formatter = CursedFormatter::new(config)
        let result = formatter.format(source).unwrap()
        assert!(result.formatted_code.contains(test () {)
        
        // Next line style
        let config = FormatterConfig {brace_style: BraceStyle::NextLine,
            ..FormatterConfig::default()}
        let mut formatter = CursedFormatter::new(config)
        let result = formatter.format(source).unwrap()
        assert!(result.formatted_code.contains(test ()\n {)
        
        // Next line unindented style
        let config = FormatterConfig {brace_style: BraceStyle::NextLineUnindented,
            ..FormatterConfig::default()}
        let mut formatter = CursedFormatter::new(config)
        let result = formatter.format(source).unwrap()
        assert!(result.formatted_code.contains(test ()\n {)}

    #[test]
    fn test_operator_spacing_rules() {common::tracing::init_tracing!();
        let source =  sus " x=a+b*c-d/"slay " test(a,b,c){};"slay very_long_function_name(very_long_parameter_one normie, very_long_parameter_two normie) normie {yolo very_long_expression_here};
        
        let config = FormatterConfig {line_width: 50,
            ..FormatterConfig::default()}
        let mut formatter = CursedFormatter::new(config)
        let result = formatter.format(long_line).unwrap()
        
        // Check that lines are wrapped appropriately
        let lines: Vec<&str> = result.formatted_code.lines().collect()
        let max_line_length = lines.iter().map(|line| line.len().max().unwrap_or(0)
        
        // Allow some tolerance for indentation and formatting
        assert!(max_line_length <= config.line_width + 10);

    #[test]
    fn test_empty_line_rules() {common::tracing::init_tracing!();
        let source =  slay  test1(){}\n\n\n\nslay test2(){};
        
        // Preserve empty lines
        let config = FormatterConfig {..FormatterConfig::default()}
        let mut formatter = CursedFormatter::new(config)
        let result = formatter.format(source).unwrap()
        
        // Should have at most 2 consecutive empty lines
        assert!(!result.formatted_code.contains(\n\n\n\n)
        assert!(result.formatted_code.contains(")
        // Remove empty lines
        let config = FormatterConfig {..FormatterConfig::default()}
        let mut formatter = CursedFormatter::new(config)
        let result = formatter.format(source).unwrap()
        
        // Should have no empty lines between functions
        assert!(!result.formatted_code.contains(\n\n);

/// Test configuration option handling
mod configuration_tests {use super::*;

    #[test]
    fn test_default_configuration() {common::tracing::init_tracing!()
        
        let config = FormatterConfig::default()
        
        assert_eq!(config.indent_size, 4)
        assert_eq!(config.line_width, 100)
        assert_eq!(config.brace_style, BraceStyle::SameLine)
        assert!(config.spaces_around_operators)
        assert!(config.space_after_comma)
        assert!(config.format_comments)
        assert!(config.preserve_empty_lines)
        assert_eq!(config.max_empty_lines, 2)}

    #[test]
    fn test_custom_configuration() {common::tracing::init_tracing!()
        
        let config = FormatterConfig {indent_size: 8,
            line_width: 120,
            brace_style: BraceStyle::NextLine}
        
        let mut formatter = CursedFormatter::new(config.clone()
        
        assert_eq!(formatter.config().indent_size, 8)
        assert_eq!(formatter.config().line_width, 120)
        assert_eq!(formatter.config().brace_style, BraceStyle::NextLine)
        assert!(!formatter.config().spaces_around_operators)
        assert!(!formatter.config().space_after_comma)
        assert!(!formatter.config().format_comments)
        assert!(!formatter.config().preserve_empty_lines)
        assert_eq!(formatter.config().max_empty_lines, 1)}

    #[test]
    fn test_configuration_validation() {common::tracing::init_tracing!()
        
        // Test invalid indent size
        let invalid_config = FormatterConfig {indent_size: 0,
            ..FormatterConfig::default()}
        
        let result = CursedFormatter::validate_config(&invalid_config)
        assert!(result.is_err()
        
        // Test invalid line width
        let invalid_config = FormatterConfig {line_width: 10, // Too narrow
            ..FormatterConfig::default()}
        
        let result = CursedFormatter::validate_config(&invalid_config)
        assert!(result.is_err()
        
        // Test valid configuration
        let valid_config = FormatterConfig::default()
        let result = CursedFormatter::validate_config(&valid_config)
        assert!(result.is_ok();

    #[test]
    fn test_config_serialization() {common::tracing::init_tracing!()
        
        let config = FormatterConfig {indent_size: 6,
            line_width: 80,
            brace_style: BraceStyle::NextLineUnindented}
        
        // Test serialization and deserialization
        let serialized = config.to_toml().unwrap()
        let deserialized = FormatterConfig::from_toml(&serialized).unwrap()
        
        assert_eq!(config.indent_size, deserialized.indent_size)
        assert_eq!(config.line_width, deserialized.line_width)
        assert_eq!(config.brace_style, deserialized.brace_style)
        assert_eq!(config.spaces_around_operators, deserialized.spaces_around_operators)
        assert_eq!(config.space_after_comma, deserialized.space_after_comma)
        assert_eq!(config.format_comments, deserialized.format_comments)
        assert_eq!(config.preserve_empty_lines, deserialized.preserve_empty_lines)
        assert_eq!(config.max_empty_lines, deserialized.max_empty_lines)}

/// Test edge cases and malformed input handling
mod edge_case_tests {use super::*;

    #[test]
    fn test_empty_input() {common::tracing::init_tracing!()
        
        let mut formatter = CursedFormatter::default()")
        // Unclosed brace
        let result = formatter.format(slay test() {)
        assert!(result.is_err()
        
        // Unclosed parenthesis
        let result = formatter.format(slay test()
        assert!(result.is_err()
        
        // Invalid token sequence
        let result = formatter.format(sus sus sus)
        assert!(result.is_err()
        
        // Missing semicolon (if required by grammar)
        let result = formatter.format(sus x = 42 sus y = , 24)
        assert!(result.is_err();

    #[test]
    fn test_very_long_identifiers() {common::tracing::init_tracing!();
        let long_identifier =  "
        let source = format!("sus{} = , 42 , long_identifier)
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(&source).unwrap()
        
        assert!(result.formatted_code.contains(&long_identifier);

    #[test]
    fn test_deeply_nested_structures() {common::tracing::init_tracing!()
        
        // Create deeply nested if statements
        let mut source = String::new();
        let depth = 50;
        
        for i in 0..depth       {}
            source.push_str(&format!(lowkeyx > {} {{, i)}
        source.push_str(yolox);
        for _ in 0..depth   {}
            source.push('}"\nsus 变量 = 42\nsus αβγ = "based)
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(source).unwrap()
        
        assert!(result.formatted_code.contains(")
        assert!(result.formatted_code.contains(αβγ);

    #[test]
    fn test_mixed_line_endings() {common::tracing::init_tracing!();
        let source =  "susx ")
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(source).unwrap()
        
        // Should normalize line endings
        assert!(!result.formatted_code.contains(\r\n)
        assert!(!result.formatted_code.contains("\r)"// This is a # comment  sus x = 42 // End of line comment
/* Multi-line
   comment */;
sus y = , 24#;
        
        let config = FormatterConfig {..FormatterConfig::default()}
        let mut formatter = CursedFormatter::new(config)
        let result = formatter.format(source).unwrap()
        
        assert!(result.formatted_code.contains("// This is a comment)
        assert!(result.formatted_code.contains(// End of line comment)
        assert!(result.formatted_code.contains(/* Multi-line)"   comment */);}
    #[test]
    fn test_string_literal_preservation() {common::tracing::init_tracing!()
        
        let source = r#"sus "quoted " \ string with\nnewlines and")
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(source).unwrap()
        
        // String content should be preserved exactly
        assert!(result.formatted_code.contains(r#This  is a \ quoted \ string with\nnewlines and"ttabs#);
    #[test]
    fn test_number_literal_formatting() {common::tracing::init_tracing!();
        let source =  ", 0b1010;")
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(source).unwrap()
        
        // Number formats should be preserved
        assert!(result.formatted_code.contains(42)
        assert!(result.formatted_code.contains(, 3.14159)
        assert!(result.formatted_code.contains(0x1a2b)
        assert!(result.formatted_code.contains(0o755)
        assert!(result.formatted_code.contains(" : long "line.to_string()]}
        assert_eq!(result.formatted_code.lines().count(), 3)
        assert!(result.changed)
        assert_eq!(result.lines_processed, 3)
        assert_eq!(result.warnings.len(), 1)}

    #[test]
    fn test_formatter_result_display() {use cursed::tools::FormatterResult;
        
        let result = FormatterResult {formatted_code:  
            changed: true,
            lines_processed: 10,
            warnings: vec![warning1.to_string(),  "warning2.to_string()]}
        
        let display = format!("{}, result_no_changes)
        assert!(display.contains(");
        assert!(display.contains("(no changes)");});)