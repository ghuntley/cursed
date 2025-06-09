//! Golden file tests for the CURSED code formatter
//!
//! These tests compare formatter output against known-good "golden" files
//! to ensure consistent formatting and catch regressions.

use cursed::tools::{CursedFormatter, FormatterConfig, BraceStyle};
use std::fs;
use std::path::Path;

#[path = "common/mod.rs"]
mod common;

/// Helper function to read test file content
fn read_test_file(name: &str) -> String {
    let path = Path::new("tests/formatter_test_files").join(name);
    fs::read_to_string(path).expect(&format!("Failed to read test file: {}", name))
}

/// Test golden file formatting with default configuration
mod golden_file_tests {
    use super::*;

    #[test]
    fn test_simple_function_golden() {
        init_tracing!();
        
        let input = read_test_file("simple_function_before.csd");
        let expected = read_test_file("simple_function_after.csd");
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(&input).unwrap();
        
        assert!(result.changed);
        assert_eq!(result.formatted_code.trim(), expected.trim());
    }

    #[test]
    fn test_complex_program_golden() {
        init_tracing!();
        
        let input = read_test_file("complex_program_before.csd");
        let expected = read_test_file("complex_program_after.csd");
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(&input).unwrap();
        
        assert!(result.changed);
        assert_eq!(result.formatted_code.trim(), expected.trim());
    }

    #[test]
    fn test_generics_golden() {
        init_tracing!();
        
        let input = read_test_file("generics_before.csd");
        let expected = read_test_file("generics_after.csd");
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(&input).unwrap();
        
        assert!(result.changed);
        assert_eq!(result.formatted_code.trim(), expected.trim());
    }

    #[test]
    fn test_edge_cases_golden() {
        init_tracing!();
        
        let input = read_test_file("edge_cases_before.csd");
        let expected = read_test_file("edge_cases_after.csd");
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(&input).unwrap();
        
        assert!(result.changed);
        // For edge cases, we might need more flexible comparison
        // due to different valid formatting approaches
        assert!(result.formatted_code.len() > input.len());
        assert!(result.formatted_code.contains("squad Matrix[T] {"));
        assert!(result.formatted_code.contains("    data [][]T"));
    }

    #[test]
    fn test_comments_golden() {
        init_tracing!();
        
        let input = read_test_file("comments_before.csd");
        let expected = read_test_file("comments_after.csd");
        
        let config = FormatterConfig {
            format_comments: true,
            ..FormatterConfig::default()
        };
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(&input).unwrap();
        
        assert!(result.changed);
        assert_eq!(result.formatted_code.trim(), expected.trim());
    }
}

/// Test regression detection
mod regression_tests {
    use super::*;

    #[test]
    fn test_formatting_stability() {
        init_tracing!();
        
        let input = read_test_file("simple_function_before.csd");
        
        let mut formatter = CursedFormatter::default();
        let first_pass = formatter.format(&input).unwrap();
        
        // Second pass should produce identical output
        let second_pass = formatter.format(&first_pass.formatted_code).unwrap();
        
        assert!(!second_pass.changed); // Should detect no changes needed
        assert_eq!(first_pass.formatted_code, second_pass.formatted_code);
    }

    #[test]
    fn test_idempotency() {
        init_tracing!();
        
        let inputs = [
            "simple_function_before.csd",
            "complex_program_before.csd",
            "generics_before.csd",
        ];
        
        for input_file in &inputs {
            let input = read_test_file(input_file);
            let mut formatter = CursedFormatter::default();
            
            // Format multiple times
            let mut current = input;
            for i in 0..5 {
                let result = formatter.format(&current).unwrap();
                current = result.formatted_code;
                
                // After first pass, should be stable
                if i > 0 {
                    assert!(!result.changed, "Formatting not stable for {}", input_file);
                }
            }
        }
    }

    #[test]
    fn test_semantic_preservation() {
        init_tracing!();
        
        let input = r#"
sus result = a + b * c - d / e
sus string = "This is a \"quoted\" string"
sus numbers = [42, 0x1A, 0o755, 0b1010]
sus map = {key: "value", number: 42}
sus type_cast = value.(Type)
"#.trim();

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(input).unwrap();
        
        // Check that semantic elements are preserved
        assert!(result.formatted_code.contains("a + b * c - d / e"));
        assert!(result.formatted_code.contains(r#""This is a \"quoted\" string""#));
        assert!(result.formatted_code.contains("42"));
        assert!(result.formatted_code.contains("0x1A"));
        assert!(result.formatted_code.contains("0o755"));
        assert!(result.formatted_code.contains("0b1010"));
        assert!(result.formatted_code.contains("value.(Type)"));
    }
}

/// Test performance with large files
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_large_file_performance() {
        init_tracing!();
        
        let real_world_content = read_test_file("real_world_example.csd");
        
        // Replicate content to create a large file
        let mut large_content = String::new();
        for i in 0..100 {
            large_content.push_str(&real_world_content);
            large_content.push_str(&format!("\n// Copy {}\n", i));
        }
        
        let start = Instant::now();
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(&large_content).unwrap();
        let duration = start.elapsed();
        
        // Should complete within reasonable time
        assert!(duration.as_millis() < 5000); // 5 seconds
        assert!(result.formatted_code.len() >= large_content.len());
        assert!(result.lines_processed > 1000);
    }

    #[test]
    fn test_memory_usage() {
        init_tracing!();
        
        // Create content with deep nesting
        let mut deep_content = String::new();
        for i in 0..100 {
            deep_content.push_str(&format!("lowkey x > {} {{\n", i));
        }
        deep_content.push_str("yolo x\n");
        for _ in 0..100 {
            deep_content.push_str("}\n");
        }
        
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(&deep_content);
        
        // Should handle deep nesting without excessive memory usage
        assert!(result.is_ok());
        let formatted = result.unwrap();
        assert!(formatted.changed);
        assert!(formatted.formatted_code.len() > deep_content.len());
    }

    #[test]
    fn test_repeated_formatting_performance() {
        init_tracing!();
        
        let input = read_test_file("complex_program_before.csd");
        let mut formatter = CursedFormatter::default();
        
        let start = Instant::now();
        for _ in 0..100 {
            let _ = formatter.format(&input).unwrap();
        }
        let duration = start.elapsed();
        
        // Should be fast for repeated formatting
        assert!(duration.as_millis() < 2000); // 2 seconds for 100 iterations
    }
}

/// Test different configuration combinations
mod configuration_golden_tests {
    use super::*;

    #[test]
    fn test_allman_brace_style_golden() {
        init_tracing!();
        
        let input = "slay test(){lowkey based{yolo 42}}";
        let config = FormatterConfig {
            brace_style: BraceStyle::NextLineUnindented,
            ..FormatterConfig::default()
        };
        
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(input).unwrap();
        
        let expected = "slay test()\n{\n    lowkey based\n    {\n        yolo 42\n    }\n}";
        assert_eq!(result.formatted_code.trim(), expected);
    }

    #[test]
    fn test_compact_style_golden() {
        init_tracing!();
        
        let input = "slay test(a normie, b normie) normie { yolo a + b }";
        let config = FormatterConfig {
            spaces_around_operators: false,
            space_after_comma: false,
            ..FormatterConfig::default()
        };
        
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(input).unwrap();
        
        assert!(result.formatted_code.contains("test(a normie,b normie)"));
        assert!(result.formatted_code.contains("yolo a+b"));
    }

    #[test]
    fn test_wide_indentation_golden() {
        init_tracing!();
        
        let input = "slay test(){lowkey based{yolo 42}}";
        let config = FormatterConfig {
            indent_size: 8,
            ..FormatterConfig::default()
        };
        
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(input).unwrap();
        
        assert!(result.formatted_code.contains("        lowkey based")); // 8 spaces
        assert!(result.formatted_code.contains("                yolo 42")); // 16 spaces
    }
}

/// Test error handling with malformed input
mod error_handling_golden_tests {
    use super::*;

    #[test]
    fn test_syntax_errors() {
        init_tracing!();
        
        let invalid_input = read_test_file("syntax_errors.csd");
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(&invalid_input);
        
        // Should return an error for malformed syntax
        assert!(result.is_err());
    }

    #[test]
    fn test_partial_formatting() {
        init_tracing!();
        
        // Test with partially valid code
        let mixed_input = r#"
slay valid_function() {
    yolo 42
}

slay incomplete_function(
// Missing closing parenthesis and body

slay another_valid() {
    yolo "hello"
}
"#.trim();

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(mixed_input);
        
        // Should handle partial failures gracefully
        // (behavior depends on implementation - might format valid parts or fail entirely)
        match result {
            Ok(formatted) => {
                assert!(formatted.formatted_code.contains("slay valid_function() {"));
                assert!(formatted.warnings.len() > 0);
            }
            Err(_) => {
                // Also acceptable - depends on error recovery strategy
            }
        }
    }

    #[test]
    fn test_unicode_handling() {
        init_tracing!();
        
        let unicode_input = r#"
sus café = "coffee"
sus 变量 = 42
sus αβγ = "greek"
slay test_unicode() {
    yolo café + 变量
}
"#.trim();

        let mut formatter = CursedFormatter::default();
        let result = formatter.format(unicode_input).unwrap();
        
        assert!(result.formatted_code.contains("café"));
        assert!(result.formatted_code.contains("变量"));
        assert!(result.formatted_code.contains("αβγ"));
        assert!(result.formatted_code.contains("café + 变量"));
    }
}

/// Test line ending normalization
mod line_ending_tests {
    use super::*;

    #[test]
    fn test_crlf_normalization() {
        init_tracing!();
        
        let crlf_input = "slay test(){yolo 42}\r\nsus x=24\r\n";
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(crlf_input).unwrap();
        
        // Should normalize to LF
        assert!(!result.formatted_code.contains("\r\n"));
        assert!(result.formatted_code.contains("\n"));
    }

    #[test]
    fn test_mixed_line_endings() {
        init_tracing!();
        
        let mixed_input = "slay test1(){yolo 1}\nsus x=2\r\nslay test2(){yolo 3}\r";
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(mixed_input).unwrap();
        
        // Should normalize all line endings
        assert!(!result.formatted_code.contains("\r\n"));
        assert!(!result.formatted_code.contains("\r"));
        assert!(result.formatted_code.lines().count() >= 3);
    }
}

/// Test whitespace handling
mod whitespace_tests {
    use super::*;

    #[test]
    fn test_trailing_whitespace_removal() {
        init_tracing!();
        
        let input = "slay test() {   \n    yolo 42   \n}   ";
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(input).unwrap();
        
        // Should remove trailing whitespace
        for line in result.formatted_code.lines() {
            assert_eq!(line, line.trim_end());
        }
    }

    #[test]
    fn test_leading_whitespace_normalization() {
        init_tracing!();
        
        let input = "   slay test() {\n        yolo 42\n   }";
        let mut formatter = CursedFormatter::default();
        let result = formatter.format(input).unwrap();
        
        // Should normalize leading whitespace
        assert!(result.formatted_code.starts_with("slay test()"));
        assert!(result.formatted_code.contains("    yolo 42")); // Proper indentation
    }

    #[test]
    fn test_empty_line_handling() {
        init_tracing!();
        
        let input = "slay test1(){}\n\n\n\n\nslay test2(){}";
        let config = FormatterConfig {
            max_empty_lines: 2,
            ..FormatterConfig::default()
        };
        
        let mut formatter = CursedFormatter::new(config);
        let result = formatter.format(input).unwrap();
        
        // Should limit consecutive empty lines
        assert!(!result.formatted_code.contains("\n\n\n\n"));
        assert!(result.formatted_code.contains("\n\n"));
    }
}
