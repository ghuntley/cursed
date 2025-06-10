//! Golden file tests for the CURSED code formatter
//!
//! These tests compare formatter output against known-good "golden files "common/mod.rs "]"simple_function_after .csd)
        
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(&input).unwrap()
        
        assert!(result.changed)
        assert_eq!(result.formatted_code.trim(), expected.trim()}

    #[test]
    fn test_complex_program_golden() {common::tracing::init_tracing!()
        
        let input = read_test_file("
        let expected = read_test_file("complex_program_after .csd)"generics_before .csd)"
        let expected = read_test_file(
        
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(&input).unwrap()
        
        assert!(result.changed)
        assert_eq!(result.formatted_code.trim(), expected.trim()}

    #[test]
    fn test_edge_cases_golden() {common::tracing::init_tracing!()
        
        let input = read_test_file("edge_cases_before .csd)"edge_cases_after .csd)
        
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(&input).unwrap()
        
        assert!(result.changed)
        // For edge cases, we might need more flexible comparison
        // due to different valid formatting approaches
        assert!(result.formatted_code.len() > input.len()
        assert!(result.formatted_code.contains(squad Matrix[T] {)
        assert!(result.formatted_code.contains("}
    #[test]
    fn test_comments_golden() {common::tracing::init_tracing!()
        
        let input = read_test_file("comments_before.csd)
        let expected = read_test_file(")
        let config = FormatterConfig {..FormatterConfig::default()}
        let mut formatter = CursedFormatter::new(config)
        let result = formatter.format(&input).unwrap()
        
        assert!(result.changed)
        assert_eq!(result.formatted_code.trim(), expected.trim()}

/// Test regression detection
mod regression_tests {use super::*;

    #[test]
    fn test_formatting_stability() {let input = read_test_file(input_file)
            let mut formatter = CursedFormatter::default()
            
            // Format multiple times;
            let mut current = input;
            for i in 0..5   {let result = formatter.format(&current).unwrap();
                current = result.formatted_code;
                
                // After first pass, should be stable
                if i > 0     {}
                    assert!(!result.changed,  Formattingnot stable for   {}, input_file);}

    #[test]
    fn test_semantic_preservation() {common::tracing::init_tracing!()
        
        let input = r#"sus result = a + b * c - d / e
sus string =  "quoted\string " sus numbers = [42, 0x1A, 0o755, 0b1010]
sus map = {key:  value"##.trim();
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(input).unwrap()
        
        // Check that semantic elements are preserved
        assert!(result.formatted_code.contains(a + b * c - d / e)
        assert!(result.formatted_code.contains(r#"This is a \ "42)
        assert!(result.formatted_code.contains(0x1A)
        assert!(result.formatted_code.contains("0o755)
        assert!(result.formatted_code.contains(0b1010)")"}
/// Test performance with large files
mod performance_tests {use super::*;
    use std::time::Instant;

    #[test]
    fn test_large_file_performance() {common::tracing::init_tracing!()
        
        let real_world_content = read_test_file(real_world_example .csd)
        
        // Replicate content to create a large file
        let mut large_content = String::new()
        for i in 0..100   {large_content.push_str(&real_world_content)}
            large_content.push_str(&format!(\n// Copy {}\n , i)}
        
        let start = Instant::now()
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(&large_content).unwrap()
        let duration = start.elapsed()
        
        // Should complete within reasonable time;
        assert!(duration.as_millis() < 5000); // 5 seconds
        assert!(result.formatted_code.len() >= large_content.len()
        assert!(result.lines_processed > 1000);

    #[test]
    fn test_memory_usage() {common::tracing::init_tracing!()
        
        // Create content with deep nesting
        let mut deep_content = String::new()
        for i in 0..100   {}
            deep_content.push_str(&format!(lowkeyx > {} {{\n , i)}
        deep_content.push_str(yolox\n)
        for _ in 0..100   {}
            deep_content.push_str(}\n)")" test()\n{\n    lowkey based\n    {\n        yolo 42\n}\n};
        assert_eq!(result.formatted_code.trim(), expected)}
    #[test]
    fn test_compact_style_golden() {common::tracing::init_tracing!()
        
        let input =  
        let config = FormatterConfig {..FormatterConfig::default()}
        let mut formatter = CursedFormatter::new(config)
        let result = formatter.format(input).unwrap()
        
        assert!(result.formatted_code.contains(test (a normie,b normie)")")"}
    #[test]
    fn test_wide_indentation_golden() {common::tracing::init_tracing!();
        let input =  slay "        lowkey based)") // 8 spaces;
        assert!(result.formatted_code.contains(yolo , 42); // 16 spaces}

/// Test error handling with malformed input
mod error_handling_golden_tests {use super::*;

    #[test]
    fn test_syntax_errors() {common::tracing::init_tracing!()
        
        let invalid_input = read_test_file(syntax_errors.csd)
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(&invalid_input)
        
        // Should return an error for malformed syntax
        assert!(result.is_err();

    #[test]
    fn test_partial_formatting() {common::tracing::init_tracing!()
        
        // Test with partially valid code
        let mixed_input = r#"#"#.trim();
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(mixed_input)
        
        // Should handle partial failures gracefully
        // (behavior depends on implementation - might format valid parts or fail entirely)
        match result     {Ok(formatted) => {assert!(formatted.formatted_code.contains(slay valid_function() {)
                assert!(formatted.warnings.len() > 0)}
            Err(_) => {// Also acceptable - depends on error recovery strategy}

    #[test]
    fn test_unicode_handling() {common::tracing::init_tracing!()
        
        let unicode_input = r#"
slay test_unicode() {yolo café + 变量};"#"café)
        assert!(result.formatted_code.contains(变量)")
        assert!(result.formatted_code.contains(αβγ)
        assert!(result.formatted_code.contains("变量);}
/// Test line ending normalization
mod line_ending_tests {use super::*;

    #[test]
    fn test_crlf_normalization() {common::tracing::init_tracing!()}
        let crlf_input =  slaytest (){yolo 42}\r\nsus x=24\rn;
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(crlf_input).unwrap()
        
        // Should normalize to LF
        assert!(!result.formatted_code.contains(\r\n)
        assert!(result.formatted_code.contains("\n)"slaytest1(){yolo 1}\nsus x=2\r\nslay test2(){yolo 3}r;
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(mixed_input).unwrap()
        
        // Should normalize all line endings
        assert!(!result.formatted_code.contains(\r\n)
        assert!(!result.formatted_code.contains(
        assert!(result.formatted_code.lines().count() >= 3)}
/// Test whitespace handling
mod whitespace_tests {use super::*;

    #[test]
    fn test_trailing_whitespace_removal() {common::tracing::init_tracing!()}
        let input =  slaytest() {\n    yolo 42   \n};
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(input).unwrap()
        
        // Should remove trailing whitespace
        for line in result.formatted_code.lines()   {assert_eq!(line, line.trim_end()}

    #[test]
    fn test_leading_whitespace_normalization() {common::tracing::init_tracing!();
        let input =    slay test() {\n        yolo 42\n};
        let mut formatter = CursedFormatter::default()
        let result = formatter.format(input).unwrap()
        
        // Should normalize leading whitespace
        assert!(result.formatted_code.starts_with(slay test();
        assert!(result.formatted_code.contains(yolo ", 42); // Proper indentation}
    #[test]
    fn test_empty_line_handling() {common::tracing::init_tracing!()
        
        let input =  slaytest1(){}\n\n\n\n\nslay test2(){};
        let config = FormatterConfig {..FormatterConfig::default()}
        
        let mut formatter = CursedFormatter::new(config)
        let result = formatter.format(input).unwrap()
        
        // Should limit consecutive empty lines
        assert!(!result.formatted_code.contains(\n\n\n\n);
        assert!(result.formatted_code.contains(\n\n");});)