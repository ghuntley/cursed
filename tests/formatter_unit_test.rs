//! Unit tests for the CURSED code formatter engine
//!
//! These tests focus on individual formatting components and AST node formatting

use cursed::tools::{CursedFormatter, FormatterConfig, BraceStyle};
use cursed::ast::*;
use cursed::error::CursedError;

#[path = "common/mod.rs"]
mod common;

/// Test formatting of individual AST nodes
mod ast_node_formatting {
    use super::*;

    #[test]
    fn test_format_function_declaration() {
        init_tracing!();
        
        let mut formatter = CursedFormatter::default();
        
        // Test basic function
        let source = "slay test(){yolo 42}";
        let result = formatter.format_ast_node(source).unwrap();
        assert!(result.contains("slay test() {"));
        assert!(result.contains("    yolo 42"));
        
        // Test function with parameters
        let source = "slay add(x normie,y normie)normie{yolo x+y}";
        let result = formatter.format_ast_node(source).unwrap();
        assert!(result.contains("slay add(x normie, y normie) normie {"));
        assert!(result.contains("    yolo x + y"));
    }

    #[test]
    fn test_format_variable_declaration() {
        init_tracing!();
        
        let mut formatter = CursedFormatter::default();
        
        // Test sus declaration
        let source = "sus x=42";
        let result = formatter.format_ast_node(source).unwrap();
        assert_eq!(result.trim(), "sus x = 42");
        
        // Test facts declaration
        let source = "facts PI=3.14159";
        let result = formatter.format_ast_node(source).unwrap();
        assert_eq!(result.trim(), "facts PI = 3.14159");
        
        // Test typed declaration
        let source = "sus name sip=\"test\"";
        let result = formatter.format_ast_node(source).unwrap();
        assert_eq!(result.trim(), "sus name sip = \"test\"");
    }

    #[test]
    fn test_format_struct_declaration() {
        init_tracing!();
        
        let mut formatter = CursedFormatter::default();
        
        let source = "squad Person{name sip age normie}";
        let result = formatter.format_ast_node(source).unwrap();
        
        let expected = "squad Person {\n    name sip\n    age normie\n}";
        assert_eq!(result.trim(), expected);
    }

    #[test]
    fn test_format_interface_declaration() {
        init_tracing!();
        
        let mut formatter = CursedFormatter::default();
        
        let source = "collab Writer{write(data sip)normie}";
        let result = formatter.format_ast_node(source).unwrap();
        
        let expected = "collab Writer {\n    write(data sip) normie\n}";
        assert_eq!(result.trim(), expected);
    }

    #[test]
    fn test_format_if_statement() {
        init_tracing!();
        
        let mut formatter = CursedFormatter::default();
        
        let source = "lowkey x>0{yolo x}highkey{yolo 0}";
        let result = formatter.format_ast_node(source).unwrap();
        
        let expected = "lowkey x > 0 {\n    yolo x\n} highkey {\n    yolo 0\n}";
        assert_eq!(result.trim(), expected);
    }

    #[test]
    fn test_format_while_loop() {
        init_tracing!();
        
        let mut formatter = CursedFormatter::default();
        
        let source = "periodt x>0{x=x-1}";
        let result = formatter.format_ast_node(source).unwrap();
        
        let expected = "periodt x > 0 {\n    x = x - 1\n}";
        assert_eq!(result.trim(), expected);
    }

    #[test]
    fn test_format_for_loop() {
        init_tracing!();
        
        let mut formatter = CursedFormatter::default();
        
        let source = "bestie i flex range(10){yolo i}";
        let result = formatter.format_ast_node(source).unwrap();
        
        let expected = "bestie i flex range(10) {\n    yolo i\n}";
        assert_eq!(result.trim(), expected);
    }

    #[test]
    fn test_format_switch_statement() {
        init_tracing!();
        
        let mut formatter = CursedFormatter::default();
        
        let source = "vibe_check x{mood 1:yolo \"one\" mood 2:yolo \"two\" basic:yolo \"other\"}";
        let result = formatter.format_ast_node(source).unwrap();
        
        assert!(result.contains("vibe_check x {"));
        assert!(result.contains("    mood 1:"));
        assert!(result.contains("        yolo \"one\""));
        assert!(result.contains("    basic:"));
        assert!(result.contains("        yolo \"other\""));
    }
}

/// Test various formatting rules
mod formatting_rules {
    use super::*;

    #[test]
    fn test_indentation_rules() {
        init_tracing!();
        
        // Test different indentation sizes
        for indent_size in [2, 4, 8] {
            let config = FormatterConfig {
                indent_size,
                ..FormatterConfig::default()
            };
            let mut formatter = CursedFormatter::new(config);
            
            let source = "lowkey based{yolo 42}";
            let result = formatter.format(source).unwrap();
            
            let indent = " ".repeat(indent_size);
            assert!(result.formatted_code.contains(&format!("{}yolo 42", indent)));
        }
    }

    #[test]
    fn test_brace_style_rules() {
        init_tracing!();
        
        let source = "slay test(){yolo 42}";
        
        // Same line style
        let config = FormatterConfig {
            brace_style: BraceStyle::SameLine,
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        assert!(result.formatted_code.contains("test() {"));
        
        // Next line style
        let config = FormatterConfig {
            brace_style: BraceStyle::NextLine,
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        assert!(result.formatted_code.contains("test()\n    {"));
        
        // Next line unindented style
        let config = FormatterConfig {
            brace_style: BraceStyle::NextLineUnindented,
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        assert!(result.formatted_code.contains("test()\n{"));
    }

    #[test]
    fn test_operator_spacing_rules() {
        init_tracing!();
        
        let source = "sus x=a+b*c-d/e";
        
        // With spacing
        let config = FormatterConfig {
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        assert!(result.formatted_code.contains("x = a + b * c - d / e"));
        
        // Without spacing
        let config = FormatterConfig {
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        assert!(result.formatted_code.contains("x=a+b*c-d/e"));
    }

    #[test]
    fn test_comma_spacing_rules() {
        init_tracing!();
        
        let source = "slay test(a,b,c){}";
        
        // With spacing
        let config = FormatterConfig {
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        assert!(result.formatted_code.contains("test(a, b, c)"));
        
        // Without spacing
        let config = FormatterConfig {
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        assert!(result.formatted_code.contains("test(a,b,c)"));
    }

    #[test]
    fn test_line_width_rules() {
        init_tracing!();
        
        let long_line = "slay very_long_function_name(very_long_parameter_one normie, very_long_parameter_two normie) normie { yolo very_long_expression_here }";
        
        let config = FormatterConfig {
            line_width: 50,
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(long_line).unwrap();
        
        // Check that lines are wrapped appropriately
        let lines: Vec<&str> = result.formatted_code.lines().collect();
        let max_line_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        
        // Allow some tolerance for indentation and formatting
        assert!(max_line_length <= config.line_width + 10);
    }

    #[test]
    fn test_empty_line_rules() {
        init_tracing!();
        
        let source = "slay test1(){}\n\n\n\nslay test2(){}";
        
        // Preserve empty lines
        let config = FormatterConfig {
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        
        // Should have at most 2 consecutive empty lines
        assert!(!result.formatted_code.contains("\n\n\n\n"));
        assert!(result.formatted_code.contains("\n\n"));
        
        // Remove empty lines
        let config = FormatterConfig {
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        
        // Should have no empty lines between functions
        assert!(!result.formatted_code.contains("\n\n"));
    }
}

/// Test configuration option handling
mod configuration_tests {
    use super::*;

    #[test]
    fn test_default_configuration() {
        init_tracing!();
        
        let config = FormatterConfig::default();
        
        assert_eq!(config.indent_size, 4);
        assert_eq!(config.line_width, 100);
        assert_eq!(config.brace_style, BraceStyle::SameLine);
        assert!(config.spaces_around_operators);
        assert!(config.space_after_comma);
        assert!(config.format_comments);
        assert!(config.preserve_empty_lines);
        assert_eq!(config.max_empty_lines, 2);
    }

    #[test]
    fn test_custom_configuration() {
        init_tracing!();
        
        let config = FormatterConfig {
            indent_size: 8,
            line_width: 120,
            brace_style: BraceStyle::NextLine,
        };
        
        let mut formatter = CursedFormatter::new(config.clone());
        
        assert_eq!(formatter.config().indent_size, 8);
        assert_eq!(formatter.config().line_width, 120);
        assert_eq!(formatter.config().brace_style, BraceStyle::NextLine);
        assert!(!formatter.config().spaces_around_operators);
        assert!(!formatter.config().space_after_comma);
        assert!(!formatter.config().format_comments);
        assert!(!formatter.config().preserve_empty_lines);
        assert_eq!(formatter.config().max_empty_lines, 1);
    }

    #[test]
    fn test_configuration_validation() {
        init_tracing!();
        
        // Test invalid indent size
        let invalid_config = FormatterConfig {
            indent_size: 0,
            ..FormatterConfig::default()
        };
        
        let result = CursedFormatter::validate_config(&invalid_config);
        assert!(result.is_err());
        
        // Test invalid line width
        let invalid_config = FormatterConfig {
            line_width: 10, // Too narrow
            ..FormatterConfig::default()
        };
        
        let result = CursedFormatter::validate_config(&invalid_config);
        assert!(result.is_err());
        
        // Test valid configuration
        let valid_config = FormatterConfig::default();
        let result = CursedFormatter::validate_config(&valid_config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_serialization() {
        init_tracing!();
        
        let config = FormatterConfig {
            indent_size: 6,
            line_width: 80,
            brace_style: BraceStyle::NextLineUnindented,
        };
        
        // Test serialization and deserialization
        let serialized = config.to_toml().unwrap();
        let deserialized = FormatterConfig::from_toml(&serialized).unwrap();
        
        assert_eq!(config.indent_size, deserialized.indent_size);
        assert_eq!(config.line_width, deserialized.line_width);
        assert_eq!(config.brace_style, deserialized.brace_style);
        assert_eq!(config.spaces_around_operators, deserialized.spaces_around_operators);
        assert_eq!(config.space_after_comma, deserialized.space_after_comma);
        assert_eq!(config.format_comments, deserialized.format_comments);
        assert_eq!(config.preserve_empty_lines, deserialized.preserve_empty_lines);
        assert_eq!(config.max_empty_lines, deserialized.max_empty_lines);
    }
}

/// Test edge cases and malformed input handling
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        init_tracing!();
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format("").unwrap();
        
        assert!(!result.changed);
        assert_eq!(result.formatted_code, "");
        assert_eq!(result.lines_processed, 0);
    }

    #[test]
    fn test_whitespace_only_input() {
        init_tracing!();
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format("   \n\n\t\t\n   ").unwrap();
        
        assert!(result.changed);
        assert_eq!(result.formatted_code.trim(), "");
    }

    #[test]
    fn test_malformed_syntax() {
        init_tracing!();
        
        let mut formatter = CursedFormatter::default();
        
        // Unclosed brace
        let result = formatter.format("slay test() {");
        assert!(result.is_err());
        
        // Unclosed parenthesis
        let result = formatter.format("slay test(");
        assert!(result.is_err());
        
        // Invalid token sequence
        let result = formatter.format("sus sus sus");
        assert!(result.is_err());
        
        // Missing semicolon (if required by grammar)
        let result = formatter.format("sus x = 42 sus y = 24");
        assert!(result.is_err());
    }

    #[test]
    fn test_very_long_identifiers() {
        init_tracing!();
        
        let long_identifier = "a".repeat(1000);
        let source = format!("sus {} = 42", long_identifier);
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(&source).unwrap();
        
        assert!(result.formatted_code.contains(&long_identifier));
    }

    #[test]
    fn test_deeply_nested_structures() {
        init_tracing!();
        
        // Create deeply nested if statements
        let mut source = String::new();
        let depth = 50;
        
        for i in 0..depth {
            source.push_str(&format!("lowkey x > {} {{", i));
        }
        source.push_str("yolo x");
        for _ in 0..depth {
            source.push('}');
        }
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(&source);
        
        // Should handle deep nesting without stack overflow
        assert!(result.is_ok());
        
        let formatted = result.unwrap();
        assert!(formatted.changed);
        
        // Check proper indentation at various levels
        let lines: Vec<&str> = formatted.formatted_code.lines().collect();
        let max_indent = lines.iter()
            .map(|line| line.len() - line.trim_start().len())
            .max()
            .unwrap_or(0);
        
        // Should have proper indentation depth
        assert!(max_indent > 0);
        assert!(max_indent <= depth * 4); // 4 spaces per level
    }

    #[test]
    fn test_unicode_identifiers() {
        init_tracing!();
        
        let source = "sus café = \"coffee\"\nsus 变量 = 42\nsus αβγ = based";
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        assert!(result.formatted_code.contains("café"));
        assert!(result.formatted_code.contains("变量"));
        assert!(result.formatted_code.contains("αβγ"));
    }

    #[test]
    fn test_mixed_line_endings() {
        init_tracing!();
        
        let source = "sus x = 42\r\nsus y = 24\nsus z = 12\r";
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        // Should normalize line endings
        assert!(!result.formatted_code.contains("\r\n"));
        assert!(!result.formatted_code.contains("\r"));
        assert!(result.formatted_code.lines().count() >= 3);
    }

    #[test]
    fn test_comment_preservation() {
        init_tracing!();
        
        let source = r#"// This is a comment
sus x = 42 // End of line comment
/* Multi-line
   comment */
sus y = 24"#;
        
        let config = FormatterConfig {
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(source).unwrap();
        
        assert!(result.formatted_code.contains("// This is a comment"));
        assert!(result.formatted_code.contains("// End of line comment"));
        assert!(result.formatted_code.contains("/* Multi-line"));
        assert!(result.formatted_code.contains("   comment */"));
    }

    #[test]
    fn test_string_literal_preservation() {
        init_tracing!();
        
        let source = r#"sus message = "This is a \"quoted\" string with\nnewlines and\ttabs""#;
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        // String content should be preserved exactly
        assert!(result.formatted_code.contains(r#""This is a \"quoted\" string with\nnewlines and\ttabs""#));
    }

    #[test]
    fn test_number_literal_formatting() {
        init_tracing!();
        
        let source = "sus x=42 sus y=3.14159 sus z=0x1a2b sus w=0o755 sus v=0b1010";
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(source).unwrap();
        
        // Number formats should be preserved
        assert!(result.formatted_code.contains("42"));
        assert!(result.formatted_code.contains("3.14159"));
        assert!(result.formatted_code.contains("0x1a2b"));
        assert!(result.formatted_code.contains("0o755"));
        assert!(result.formatted_code.contains("0b1010"));
    }
}

/// Test formatter result structure
mod result_tests {
    use super::*;

    #[test]
    fn test_formatter_result_creation() {
        use cursed::tools::FormatterResult;
        
        let result = FormatterResult {
            formatted_code: "slay test() {\n    yolo 42\n}".to_string(),
            changed: true,
            lines_processed: 3,
            warnings: vec!["Warning: long line".to_string()],
        };
        
        assert_eq!(result.formatted_code.lines().count(), 3);
        assert!(result.changed);
        assert_eq!(result.lines_processed, 3);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_formatter_result_display() {
        use cursed::tools::FormatterResult;
        
        let result = FormatterResult {
            formatted_code: "test".to_string(),
            changed: true,
            lines_processed: 10,
            warnings: vec!["warning1".to_string(), "warning2".to_string()],
        };
        
        let display = format!("{}", result);
        assert!(display.contains("Formatted 10 lines"));
        assert!(display.contains("(changes made)"));
        assert!(display.contains("2 warnings"));
        
        let result_no_changes = FormatterResult {
            formatted_code: "test".to_string(),
            changed: false,
            lines_processed: 5,
            warnings: vec![],
        };
        
        let display = format!("{}", result_no_changes);
        assert!(display.contains("Formatted 5 lines"));
        assert!(display.contains("(no changes)"));
        assert!(display.contains("0 warnings"));
    }
}
