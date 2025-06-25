//! Basic Documentation Parser Tests
//! 
//! Tests the comment parsing system that extracts JSDoc-style documentation
//! from CURSED source code comments. This is critical for ensuring accurate
//! documentation generation from properly formatted comments.

use cursed::documentation::comment_parser::{CommentParser, ParsedComment, DocTag};
use cursed::error::Error;
use std::collections::HashMap;
use tracing::{debug, info};

#[path = "common.rs"]
mod common;

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_parser_creation() {
        init_tracing!();
        info!("Testing comment parser creation");
        
        let parser = CommentParser::new();
        assert!(parser.is_ok());
        debug!("Comment parser created successfully");
    }

    #[test]
    fn test_basic_comment_parsing() {
        init_tracing!();
        info!("Testing basic comment parsing");
        
        let parser = CommentParser::new().unwrap();
        let comment = "/// Basic function comment\n/// Does something useful";
        
        let result = parser.parse_comment(comment);
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        assert_eq!(parsed.summary, "Basic function comment");
        assert_eq!(parsed.description, Some("Does something useful".to_string()));
        debug!("Basic comment parsed successfully: {:?}", parsed);
    }

    #[test]
    fn test_jsdoc_style_tags() {
        init_tracing!();
        info!("Testing JSDoc-style tag parsing");
        
        let parser = CommentParser::new().unwrap();
        let comment = r#"/// Calculate the fibonacci number
/// 
/// @param n The number to calculate fibonacci for
/// @return The fibonacci result
/// @example
/// let result = fibonacci(10);
/// assert_eq!(result, 55);
/// @since 1.0.0
/// @author John Doe"#;
        
        let result = parser.parse_comment(comment);
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        assert_eq!(parsed.summary, "Calculate the fibonacci number");
        
        // Check for param tag
        let param_tags: Vec<_> = parsed.tags.iter()
            .filter(|tag| matches!(tag, DocTag::Param { .. }))
            .collect();
        assert_eq!(param_tags.len(), 1);
        
        // Check for return tag
        let return_tags: Vec<_> = parsed.tags.iter()
            .filter(|tag| matches!(tag, DocTag::Return { .. }))
            .collect();
        assert_eq!(return_tags.len(), 1);
        
        // Check for example tag
        let example_tags: Vec<_> = parsed.tags.iter()
            .filter(|tag| matches!(tag, DocTag::Example { .. }))
            .collect();
        assert_eq!(example_tags.len(), 1);
        
        debug!("JSDoc tags parsed successfully: {} tags total", parsed.tags.len());
    }

    #[test]
    fn test_code_example_parsing() {
        init_tracing!();
        info!("Testing code example parsing");
        
        let parser = CommentParser::new().unwrap();
        let comment = r#"/// Function with code example
/// 
/// @example
/// ```cursed
/// slay calculate_sum(a: f64, b: f64) -> f64 {
///     facts result = a + b;
///     return result;
/// }
/// ```"#;
        
        let result = parser.parse_comment(comment);
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        let example_tags: Vec<_> = parsed.tags.iter()
            .filter_map(|tag| match tag {
                DocTag::Example { code, .. } => Some(code),
                _ => None,
            })
            .collect();
        
        assert_eq!(example_tags.len(), 1);
        assert!(example_tags[0].contains("slay calculate_sum"));
        assert!(example_tags[0].contains("facts result"));
        debug!("Code example parsed successfully");
    }

    #[test]
    fn test_parameter_parsing() {
        init_tracing!();
        info!("Testing parameter parsing");
        
        let parser = CommentParser::new().unwrap();
        let comment = r#"/// Function with multiple parameters
/// 
/// @param name The name of the person
/// @param age The age as a number
/// @param active Whether the person is active
/// @return A formatted string"#;
        
        let result = parser.parse_comment(comment);
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        let param_tags: Vec<_> = parsed.tags.iter()
            .filter_map(|tag| match tag {
                DocTag::Param { name, description, .. } => Some((name, description)),
                _ => None,
            })
            .collect();
        
        assert_eq!(param_tags.len(), 3);
        
        // Check parameter names and descriptions
        let param_map: HashMap<&str, &str> = param_tags.iter()
            .map(|(name, desc)| (name.as_str(), desc.as_str()))
            .collect();
        
        assert_eq!(param_map.get("name"), Some(&"The name of the person"));
        assert_eq!(param_map.get("age"), Some(&"The age as a number"));
        assert_eq!(param_map.get("active"), Some(&"Whether the person is active"));
        
        debug!("Parameters parsed successfully: {:?}", param_map);
    }

    #[test]
    fn test_multiline_descriptions() {
        init_tracing!();
        info!("Testing multiline description parsing");
        
        let parser = CommentParser::new().unwrap();
        let comment = r#"/// Complex function with detailed description
/// 
/// This function performs a complex calculation that involves
/// multiple steps and considerations. It is important to
/// understand the algorithm before using this function.
/// 
/// The implementation uses an optimized approach that
/// reduces time complexity from O(n²) to O(n log n).
/// 
/// @param data The input data array
/// @return The processed result"#;
        
        let result = parser.parse_comment(comment);
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        assert_eq!(parsed.summary, "Complex function with detailed description");
        
        let description = parsed.description.unwrap();
        assert!(description.contains("multiple steps"));
        assert!(description.contains("O(n log n)"));
        assert!(description.len() > 100); // Should be a substantial description
        
        debug!("Multiline description parsed: {} characters", description.len());
    }

    #[test]
    fn test_malformed_comment_handling() {
        init_tracing!();
        info!("Testing malformed comment handling");
        
        let parser = CommentParser::new().unwrap();
        
        // Test empty comment
        let result = parser.parse_comment("");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert!(parsed.summary.is_empty());
        
        // Test comment with only whitespace
        let result = parser.parse_comment("///   \n///  \n/// ");
        assert!(result.is_ok());
        
        // Test malformed tag
        let comment = "/// Function comment\n/// @malformed-tag without proper format";
        let result = parser.parse_comment(comment);
        assert!(result.is_ok()); // Should not fail, just ignore malformed tags
        
        debug!("Malformed comments handled gracefully");
    }

    #[test]
    fn test_special_tag_types() {
        init_tracing!();
        info!("Testing special tag types");
        
        let parser = CommentParser::new().unwrap();
        let comment = r#"/// Special function
/// 
/// @deprecated Use new_function() instead
/// @throws InvalidArgumentError When input is invalid
/// @see related_function() for similar functionality
/// @author Jane Smith <jane@example.com>
/// @version 2.1.0
/// @since 1.5.0"#;
        
        let result = parser.parse_comment(comment);
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        
        // Check for deprecated tag
        let has_deprecated = parsed.tags.iter().any(|tag| {
            matches!(tag, DocTag::Deprecated { .. })
        });
        assert!(has_deprecated);
        
        // Check for throws tag
        let has_throws = parsed.tags.iter().any(|tag| {
            matches!(tag, DocTag::Throws { .. })
        });
        assert!(has_throws);
        
        // Check for see tag
        let has_see = parsed.tags.iter().any(|tag| {
            matches!(tag, DocTag::See { .. })
        });
        assert!(has_see);
        
        debug!("Special tag types parsed successfully: {} tags", parsed.tags.len());
    }

    #[test]
    fn test_inline_code_and_markdown() {
        init_tracing!();
        info!("Testing inline code and markdown parsing");
        
        let parser = CommentParser::new().unwrap();
        let comment = r#"/// Function with `inline code` and **bold** text
/// 
/// This function uses the `fibonacci` algorithm to calculate
/// values. The **time complexity** is O(n) and the *space
/// complexity* is O(1).
/// 
/// See: https://en.wikipedia.org/wiki/Fibonacci_number
/// 
/// @param n The input value (must be `>= 0`)
/// @return The calculated result"#;
        
        let result = parser.parse_comment(comment);
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        assert!(parsed.summary.contains("`inline code`"));
        assert!(parsed.summary.contains("**bold**"));
        
        let description = parsed.description.unwrap();
        assert!(description.contains("`fibonacci`"));
        assert!(description.contains("**time complexity**"));
        assert!(description.contains("https://"));
        
        debug!("Inline code and markdown preserved in parsing");
    }

    #[test]
    fn test_comment_extraction_from_source() {
        init_tracing!();
        info!("Testing comment extraction from source code");
        
        let parser = CommentParser::new().unwrap();
        let source_code = r#"
/// Calculate the factorial of a number
/// 
/// @param n The number to calculate factorial for
/// @return The factorial result
slay factorial(n: i32) -> i32 {
    lowkey (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

/// Add two numbers together
/// @param a First number
/// @param b Second number
/// @return The sum
slay add(a: i32, b: i32) -> i32 {
    return a + b;
}
"#;
        
        let comments = parser.extract_comments_from_source(source_code);
        assert!(comments.is_ok());
        
        let extracted = comments.unwrap();
        assert_eq!(extracted.len(), 2);
        
        // First comment should be for factorial
        assert!(extracted[0].contains("factorial"));
        assert!(extracted[0].contains("@param n"));
        
        // Second comment should be for add
        assert!(extracted[1].contains("Add two numbers"));
        assert!(extracted[1].contains("@param a"));
        assert!(extracted[1].contains("@param b"));
        
        debug!("Extracted {} comments from source code", extracted.len());
    }
}
