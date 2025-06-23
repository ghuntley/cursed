//! Comment Parser for Documentation Generation
//! 
//! Extracts and parses documentation comments from CURSED source code,
//! including support for various documentation tags and code examples.

use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, instrument};

/// Documentation comment parsing error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentParsingError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for CommentParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Comment parsing error at {}:{}: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for CommentParsingError {}

/// Parsed documentation comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedComment {
    /// Main description text
    pub description: String,
    /// Documentation tags
    pub tags: Vec<DocTag>,
    /// Code examples
    pub examples: Vec<CodeExample>,
    /// Raw comment text
    pub raw_text: String,
    /// Source location
    pub location: (usize, usize), // (line, column)
}

/// Documentation tag (e.g., @param, @return, @example)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocTag {
    /// Tag name (param, return, example, etc.)
    pub name: String,
    /// Tag value/content
    pub value: String,
    /// Additional attributes
    pub attributes: HashMap<String, String>,
}

/// Code example extracted from documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    /// Example title
    pub title: Option<String>,
    /// Example code
    pub code: String,
    /// Programming language
    pub language: String,
    /// Expected output
    pub expected_output: Option<String>,
    /// Whether the example is runnable
    pub is_runnable: bool,
}

/// Comment parser for documentation extraction
pub struct CommentParser {
    /// Known documentation tags
    known_tags: HashMap<String, DocTagConfig>,
}

/// Configuration for a documentation tag
#[derive(Debug, Clone)]
struct DocTagConfig {
    /// Whether the tag requires a value
    requires_value: bool,
    /// Whether the tag can appear multiple times
    repeatable: bool,
    /// Tag description
    description: String,
}

impl CommentParser {
    /// Create a new comment parser
    #[instrument]
    pub fn new() -> Result<(), Error> {
        let mut known_tags = HashMap::new();
        
        // Standard documentation tags
        known_tags.insert("param".to_string(), DocTagConfig {
            requires_value: true,
            repeatable: true,
            description: "Function parameter documentation".to_string(),
        });
        
        known_tags.insert("return".to_string(), DocTagConfig {
            requires_value: true,
            repeatable: false,
            description: "Function return value documentation".to_string(),
        });
        
        known_tags.insert("returns".to_string(), DocTagConfig {
            requires_value: true,
            repeatable: false,
            description: "Function return value documentation (alias for @return)".to_string(),
        });
        
        known_tags.insert("example".to_string(), DocTagConfig {
            requires_value: true,
            repeatable: true,
            description: "Code example".to_string(),
        });
        
        known_tags.insert("throws".to_string(), DocTagConfig {
            requires_value: true,
            repeatable: true,
            description: "Exception that can be thrown".to_string(),
        });
        
        known_tags.insert("see".to_string(), DocTagConfig {
            requires_value: true,
            repeatable: true,
            description: "Reference to related item".to_string(),
        });
        
        known_tags.insert("since".to_string(), DocTagConfig {
            requires_value: true,
            repeatable: false,
            description: "Version when item was introduced".to_string(),
        });
        
        known_tags.insert("deprecated".to_string(), DocTagConfig {
            requires_value: false,
            repeatable: false,
            description: "Mark item as deprecated".to_string(),
        });
        
        known_tags.insert("author".to_string(), DocTagConfig {
            requires_value: true,
            repeatable: true,
            description: "Author information".to_string(),
        });
        
        known_tags.insert("version".to_string(), DocTagConfig {
            requires_value: true,
            repeatable: false,
            description: "Version information".to_string(),
        });
        
        known_tags.insert("todo".to_string(), DocTagConfig {
            requires_value: true,
            repeatable: true,
            description: "TODO item".to_string(),
        });
        
        known_tags.insert("note".to_string(), DocTagConfig {
            requires_value: true,
            repeatable: true,
            description: "Note or warning".to_string(),
        });
        
        Ok(Self { known_tags })
    }

    /// Parse documentation comments from source code
    #[instrument(skip(self, source_code))]
    pub fn parse_comments(&self, source_code: &str) -> Result<(), Error> {
        let mut comments = Vec::new();
        let lines: Vec<&str> = source_code.split("\n").collect();
        
        let mut i = 0;
        while i < lines.len() {
            if let Some(comment) = self.extract_doc_comment(&lines, &mut i)? {
                comments.push(comment);
            } else {
                i += 1;
            }
        }
        
        Ok(comments)
    }

    /// Extract a single documentation comment starting at the given line
    fn extract_doc_comment(
        &self,
        lines: &[&str],
        line_index: &mut usize,
    ) -> Result<(), Error> {
        let start_line = *line_index;
        let mut comment_lines = Vec::new();
        let mut found_doc_comment = false;
        
        // Look for documentation comments (///, /** */, //!)
        while *line_index < lines.len() {
            let line = lines[*line_index].trim();
            
            if line.starts_with("///") {
                // Single-line documentation comment
                found_doc_comment = true;
                let content = line.trim_start_matches("///").trim();
                comment_lines.push(content.to_string());
                *line_index += 1;
            } else if line.starts_with("//!") {
                // Module-level documentation comment
                found_doc_comment = true;
                let content = line.trim_start_matches("//!").trim();
                comment_lines.push(content.to_string());
                *line_index += 1;
            } else if line.starts_with("/**") && line.ends_with("*/") {
                // Single-line block comment
                found_doc_comment = true;
                let content = line
                    .trim_start_matches("/**")
                    .trim_end_matches("*/")
                    .trim();
                comment_lines.push(content.to_string());
                *line_index += 1;
            } else if line.starts_with("/**") {
                // Multi-line block comment start
                found_doc_comment = true;
                let mut content = line.trim_start_matches("/**").trim().to_string();
                if !content.is_empty() {
                    comment_lines.push(content);
                }
                *line_index += 1;
                
                // Continue until end of block comment
                while *line_index < lines.len() {
                    let block_line = lines[*line_index].trim();
                    
                    if block_line.ends_with("*/") {
                        let content = block_line
                            .trim_end_matches("*/")
                            .trim_start_matches("*")
                            .trim();
                        if !content.is_empty() {
                            comment_lines.push(content.to_string());
                        }
                        *line_index += 1;
                        break;
                    } else {
                        let content = block_line.trim_start_matches("*").trim();
                        comment_lines.push(content.to_string());
                        *line_index += 1;
                    }
                }
            } else if line.starts_with("//") || line.is_empty() {
                // Regular comment or empty line - skip
                *line_index += 1;
            } else {
                // Non-comment line - stop
                break;
            }
        }
        
        if !found_doc_comment || comment_lines.is_empty() {
            return Ok(None);
        }
        
        // Parse the extracted comment
        let raw_text = comment_lines.join("\n");
        let parsed = self.parse_comment_content(&raw_text, start_line + 1)?;
        
        Ok(Some(parsed))
    }

    /// Parse the content of a documentation comment
    fn parse_comment_content(
        &self,
        content: &str,
        start_line: usize,
    ) -> Result<(), Error> {
        let mut description_lines = Vec::new();
        let mut tags = Vec::new();
        let mut examples = Vec::new();
        let mut current_tag: Option<(String, Vec<String>)> = None;
        let mut in_code_block = false;
        let mut current_example: Option<CodeExample> = None;
        
        for (line_offset, line) in content.split("\n").enumerate() {
            let line = line.trim();
            let current_line = start_line + line_offset;
            
            // Handle code blocks for examples
            if line.starts_with("```") {
                if in_code_block {
                    // End of code block
                    in_code_block = false;
                    if let Some(mut example) = current_example.take() {
                        examples.push(example);
                    }
                } else {
                    // Start of code block
                    in_code_block = true;
                    let language = line.trim_start_matches("```").trim();
                    current_example = Some(CodeExample {
                        title: None,
                        code: String::new(),
                        language: if language.is_empty() { "cursed".to_string() } else { language.to_string() },
                        expected_output: None,
                        is_runnable: language == "cursed" || language == "csd",
                    });
                }
                continue;
            }
            
            if in_code_block {
                // Add line to current code example
                if let Some(ref mut example) = current_example {
                    if !example.code.is_empty() {
                        example.code.push('\n');
                    }
                    example.code.push_str(line);
                }
                continue;
            }
            
            // Handle documentation tags
            if line.starts_with('@') {
                // Finish previous tag if any
                if let Some((tag_name, tag_lines)) = current_tag.take() {
                    tags.push(DocTag {
                        name: tag_name,
                        value: tag_lines.join("\n").trim().to_string(),
                        attributes: HashMap::new(),
                    });
                }
                
                // Parse new tag
                let tag_content = line.trim_start_matches('@');
                if let Some(space_pos) = tag_content.find(' ') {
                    let tag_name = tag_content[..space_pos].to_string();
                    let tag_value = tag_content[space_pos + 1..].trim().to_string();
                    current_tag = Some((tag_name, vec![tag_value]));
                } else {
                    let tag_name = tag_content.to_string();
                    current_tag = Some((tag_name, Vec::new()));
                }
            } else if let Some((ref tag_name, ref mut tag_lines)) = current_tag {
                // Continue previous tag
                tag_lines.push(line.to_string());
            } else if !line.is_empty() {
                // Regular description line
                description_lines.push(line.to_string());
            }
        }
        
        // Finish last tag if any
        if let Some((tag_name, tag_lines)) = current_tag {
            tags.push(DocTag {
                name: tag_name,
                value: tag_lines.join("\n").trim().to_string(),
                attributes: HashMap::new(),
            });
        }
        
        // Finish last example if any
        if let Some(example) = current_example {
            examples.push(example);
        }
        
        // Validate tags
        for tag in &tags {
            self.validate_tag(tag, start_line)?;
        }
        
        Ok(ParsedComment {
            description: description_lines.join("\n").trim().to_string(),
            tags,
            examples,
            raw_text: content.to_string(),
            location: (start_line, 1),
        })
    }

    /// Validate a documentation tag
    fn validate_tag(&self, tag: &DocTag, line: usize) -> Result<(), Error> {
        if let Some(config) = self.known_tags.get(&tag.name) {
            if config.requires_value && tag.value.is_empty() {
                return Err(CommentParsingError {
                    message: format!("Tag @{} requires a value", tag.name),
                    line,
                    column: 1,
                });
            }
        } else {
            debug!("Unknown documentation tag: @{}", tag.name);
        }
        
        Ok(())
    }

    /// Extract code examples from parsed comments
    pub fn extract_examples(&self, comments: &[ParsedComment]) -> Vec<CodeExample> {
        let mut all_examples = Vec::new();
        
        for comment in comments {
            // Add examples from code blocks
            all_examples.extend(comment.examples.clone());
            
            // Add examples from @example tags
            for tag in &comment.tags {
                if tag.name == "example" {
                    all_examples.push(CodeExample {
                        title: None,
                        code: tag.value.clone(),
                        language: "cursed".to_string(),
                        expected_output: None,
                        is_runnable: true,
                    });
                }
            }
        }
        
        all_examples
    }

    /// Get parameter documentation from tags
    pub fn get_parameter_docs(&self, comments: &[ParsedComment]) -> HashMap<String, String> {
        let mut param_docs = HashMap::new();
        
        for comment in comments {
            for tag in &comment.tags {
                if tag.name == "param" {
                    // Parse parameter format: "param_name Description text"
                    if let Some(space_pos) = tag.value.find(' ') {
                        let param_name = tag.value[..space_pos].to_string();
                        let param_desc = tag.value[space_pos + 1..].to_string();
                        param_docs.insert(param_name, param_desc);
                    }
                }
            }
        }
        
        param_docs
    }

    /// Get return documentation from tags
    pub fn get_return_docs(&self, comments: &[ParsedComment]) -> Option<String> {
        for comment in comments {
            for tag in &comment.tags {
                if tag.name == "return" || tag.name == "returns" {
                    return Some(tag.value.clone());
                }
            }
        }
        None
    }

    /// Check if item is marked as deprecated
    pub fn is_deprecated(&self, comments: &[ParsedComment]) -> bool {
        for comment in comments {
            for tag in &comment.tags {
                if tag.name == "deprecated" {
                    return true;
                }
            }
        }
        false
    }

    /// Get all tags of a specific type
    pub fn get_tags_by_name(&self, comments: &[ParsedComment], tag_name: &str) -> Vec<String> {
        let mut values = Vec::new();
        
        for comment in comments {
            for tag in &comment.tags {
                if tag.name == tag_name {
                    values.push(tag.value.clone());
                }
            }
        }
        
        values
    }

    /// Get the main description from comments
    pub fn get_main_description(&self, comments: &[ParsedComment]) -> Option<String> {
        for comment in comments {
            if !comment.description.is_empty() {
                return Some(comment.description.clone());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_line_doc_comment() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
/// This is a function that does something
/// @param x The input value
/// @return The result
slay function test(x: i32) -> i32 {
    return x * 2;
}
"#;
        
        let comments = parser.parse_comments(source).unwrap();
        assert_eq!(comments.len(), 1);
        
        let comment = &comments[0];
        assert_eq!(comment.description, "This is a function that does something");
        assert_eq!(comment.tags.len(), 2);
        assert_eq!(comment.tags[0].name, "param");
        assert_eq!(comment.tags[0].value, "x The input value");
        assert_eq!(comment.tags[1].name, "return");
        assert_eq!(comment.tags[1].value, "The result");
    }

    #[test]
    fn test_parse_block_doc_comment() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
/**
 * This is a multi-line documentation comment
 * that spans multiple lines.
 * 
 * @param name The name parameter
 * @return The greeting string
 */
slay function greet(name: string) -> string {
    return "Hello, " + name;
}
"#;
        
        let comments = parser.parse_comments(source).unwrap();
        assert_eq!(comments.len(), 1);
        
        let comment = &comments[0];
        assert!(comment.description.contains("multi-line documentation comment"));
        assert_eq!(comment.tags.len(), 2);
    }

    #[test]
    fn test_parse_code_example() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
/// Calculate the square of a number
/// 
/// ```cursed
/// let result = square(5);
/// spill(result); // Outputs: 25
/// ```
slay function square(x: i32) -> i32 {
    return x * x;
}
"#;
        
        let comments = parser.parse_comments(source).unwrap();
        assert_eq!(comments.len(), 1);
        
        let comment = &comments[0];
        assert_eq!(comment.examples.len(), 1);
        
        let example = &comment.examples[0];
        assert_eq!(example.language, "cursed");
        assert!(example.code.contains("square(5)"));
        assert!(example.is_runnable);
    }

    #[test]
    fn test_module_level_comment() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
//! This is a module-level documentation comment
//! that describes the entire module.
//! 
//! @author John Doe
//! @version 1.0.0

slay function some_function() {
    // implementation
}
"#;
        
        let comments = parser.parse_comments(source).unwrap();
        assert_eq!(comments.len(), 1);
        
        let comment = &comments[0];
        assert!(comment.description.contains("module-level documentation"));
        assert_eq!(comment.tags.len(), 2);
    }

    #[test]
    fn test_extract_parameter_docs() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
/// A function with multiple parameters
/// @param x The first parameter
/// @param y The second parameter  
/// @param z The third parameter
slay function multi_param(x: i32, y: string, z: bool) {
    // implementation
}
"#;
        
        let comments = parser.parse_comments(source).unwrap();
        let param_docs = parser.get_parameter_docs(&comments);
        
        assert_eq!(param_docs.len(), 3);
        assert_eq!(param_docs.get("x"), Some(&"The first parameter".to_string()));
        assert_eq!(param_docs.get("y"), Some(&"The second parameter".to_string()));
        assert_eq!(param_docs.get("z"), Some(&"The third parameter".to_string()));
    }

    #[test]
    fn test_deprecated_detection() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
/// This function is deprecated
/// @deprecated Use new_function() instead
slay function old_function() {
    // implementation
}
"#;
        
        let comments = parser.parse_comments(source).unwrap();
        assert!(parser.is_deprecated(&comments));
    }
}
