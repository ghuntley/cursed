//! Documentation comment parser for CURSED source files
//!
//! Parses both single-line (///) and multi-line (/** */) documentation comments,
//! extracts @param, @return, @example tags, and supports markdown formatting.

use crate::docs::{DocError, DocResult};
use std::collections::HashMap;
use regex::Regex;
use tracing::{debug, instrument};

/// Types of documentation tags
#[derive(Debug, Clone, PartialEq)]
pub enum DocTag {
    Param { name: String, description: String },
    Return { description: String },
    Example { code: String, description: Option<String> },
    See { reference: String },
    Since { version: String },
    Deprecated { reason: Option<String> },
    Author { name: String },
    Custom { tag: String, content: String },
}

/// Parsed documentation comment
#[derive(Debug, Clone)]
pub struct DocComment {
    /// Main description text (markdown formatted)
    pub description: String,
    /// Parsed documentation tags
    pub tags: Vec<DocTag>,
    /// Source location information
    pub line: usize,
    /// Raw comment text
    pub raw: String,
}

impl DocComment {
    pub fn new(description: String, line: usize, raw: String) -> Self {
        Self {
            description,
            tags: Vec::new(),
            line,
            raw,
        }
    }

    /// Add a documentation tag
    pub fn add_tag(&mut self, tag: DocTag) {
        self.tags.push(tag);
    }

    /// Get all tags of a specific type
    pub fn get_params(&self) -> Vec<&DocTag> {
        self.tags.iter().filter(|tag| matches!(tag, DocTag::Param { .. })).collect()
    }

    /// Get return tag if present
    pub fn get_return(&self) -> Option<&DocTag> {
        self.tags.iter().find(|tag| matches!(tag, DocTag::Return { .. }))
    }

    /// Get example tags
    pub fn get_examples(&self) -> Vec<&DocTag> {
        self.tags.iter().filter(|tag| matches!(tag, DocTag::Example { .. })).collect()
    }

    /// Check if deprecated
    pub fn is_deprecated(&self) -> bool {
        self.tags.iter().any(|tag| matches!(tag, DocTag::Deprecated { .. }))
    }
}

/// Documentation comment parser
pub struct CommentParser {
    /// Regex for single-line doc comments
    single_line_regex: Regex,
    /// Regex for multi-line doc comments
    multi_line_regex: Regex,
    /// Regex for extracting tags
    tag_regex: Regex,
}

impl CommentParser {
    /// Create a new comment parser
    pub fn new() -> DocResult<Self> {
        let single_line_regex = Regex::new(r"^\s*///\s?(.*)$")
            .map_err(|e| DocError::ParseError(format!("Failed to compile single-line regex: {}", e)))?;
        
        let multi_line_regex = Regex::new(r"/\*\*(.*?)\*/")
            .map_err(|e| DocError::ParseError(format!("Failed to compile multi-line regex: {}", e)))?;

        let tag_regex = Regex::new(r"@(\w+)(?:\s+(\w+))?\s*(.*)")
            .map_err(|e| DocError::ParseError(format!("Failed to compile tag regex: {}", e)))?;

        Ok(Self {
            single_line_regex,
            multi_line_regex,
            tag_regex,
        })
    }

    /// Parse documentation comments from source code
    #[instrument(skip(self, source))]
    pub fn parse_comments(&self, source: &str) -> DocResult<Vec<DocComment>> {
        let mut comments = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        // Parse single-line comments
        self.parse_single_line_comments(&lines, &mut comments)?;

        // Parse multi-line comments
        self.parse_multi_line_comments(source, &mut comments)?;

        debug!("Parsed {} documentation comments", comments.len());
        Ok(comments)
    }

    /// Parse single-line documentation comments (///)
    fn parse_single_line_comments(&self, lines: &[&str], comments: &mut Vec<DocComment>) -> DocResult<()> {
        let mut current_comment: Option<(String, usize, String)> = None;

        for (line_num, line) in lines.iter().enumerate() {
            if let Some(captures) = self.single_line_regex.captures(line) {
                let content = captures.get(1).map_or("", |m| m.as_str());
                
                match &mut current_comment {
                    Some((description, _, raw)) => {
                        // Continue existing comment
                        if !description.is_empty() {
                            description.push('\n');
                        }
                        description.push_str(content);
                        raw.push('\n');
                        raw.push_str(line);
                    }
                    None => {
                        // Start new comment
                        current_comment = Some((content.to_string(), line_num + 1, line.to_string()));
                    }
                }
            } else if let Some((description, line, raw)) = current_comment.take() {
                // End of comment block - process it
                let doc_comment = self.parse_comment_content(description, line, raw)?;
                comments.push(doc_comment);
            }
        }

        // Handle comment at end of file
        if let Some((description, line, raw)) = current_comment {
            let doc_comment = self.parse_comment_content(description, line, raw)?;
            comments.push(doc_comment);
        }

        Ok(())
    }

    /// Parse multi-line documentation comments (/** */)
    fn parse_multi_line_comments(&self, source: &str, comments: &mut Vec<DocComment>) -> DocResult<()> {
        for captures in self.multi_line_regex.captures_iter(source) {
            let content = captures.get(1).map_or("", |m| m.as_str());
            let raw = captures.get(0).map_or("", |m| m.as_str()).to_string();
            
            // Find line number
            let start_pos = captures.get(0).unwrap().start();
            let line_num = source[..start_pos].lines().count();
            
            // Clean up the content (remove leading * and whitespace)
            let cleaned_content = self.clean_multiline_content(content);
            let doc_comment = self.parse_comment_content(cleaned_content, line_num, raw)?;
            comments.push(doc_comment);
        }

        Ok(())
    }

    /// Clean multi-line comment content
    fn clean_multiline_content(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| {
                let trimmed = line.trim();
                if trimmed.starts_with('*') {
                    trimmed.strip_prefix('*').unwrap_or(trimmed).trim()
                } else {
                    trimmed
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }

    /// Parse comment content into description and tags
    fn parse_comment_content(&self, content: String, line: usize, raw: String) -> DocResult<DocComment> {
        let mut doc_comment = DocComment::new(String::new(), line, raw);

        // Split content into description and tags
        let tag_start = content.find('@');
        
        let (description, tag_content) = if let Some(pos) = tag_start {
            (content[..pos].trim().to_string(), &content[pos..])
        } else {
            (content.trim().to_string(), "")
        };

        doc_comment.description = description;

        // Parse tags
        if !tag_content.is_empty() {
            self.parse_tags(tag_content, &mut doc_comment)?;
        }

        Ok(doc_comment)
    }

    /// Parse documentation tags
    fn parse_tags(&self, tag_content: &str, doc_comment: &mut DocComment) -> DocResult<()> {
        // Split by @ and process each tag
        let parts: Vec<&str> = tag_content.split('@').collect();
        
        for part in parts.iter().skip(1) { // Skip first empty part
            if part.trim().is_empty() {
                continue;
            }
            
            // Find the tag name (first word)
            let mut words = part.trim().split_whitespace();
            let tag_name = words.next().unwrap_or("");
            
            // Get the rest as description
            let remaining: Vec<&str> = words.collect();
            let description = remaining.join(" ");
            
            // For param tags, try to extract parameter name
            let (param_name, final_description) = if tag_name == "param" && !remaining.is_empty() {
                (Some(remaining[0].to_string()), remaining[1..].join(" "))
            } else {
                (None, description)
            };

            let tag = match tag_name {
                "param" => {
                    if let Some(name) = param_name {
                        DocTag::Param { name, description: final_description }
                    } else {
                        return Err(DocError::ParseError("@param tag missing parameter name".to_string()));
                    }
                }
                "return" | "returns" => DocTag::Return { description: final_description },
                "example" => {
                    // For examples, we might want to parse code blocks differently
                    let (code, desc) = self.parse_example_content(&final_description);
                    DocTag::Example { 
                        code, 
                        description: if desc.is_empty() { None } else { Some(desc) }
                    }
                }
                "see" => DocTag::See { reference: final_description },
                "since" => DocTag::Since { version: final_description },
                "deprecated" => DocTag::Deprecated { 
                    reason: if final_description.is_empty() { None } else { Some(final_description) }
                },
                "author" => DocTag::Author { name: final_description },
                _ => DocTag::Custom { tag: tag_name.to_string(), content: final_description },
            };

            doc_comment.add_tag(tag);
        }

        Ok(())
    }

    /// Parse example content, separating code from description
    fn parse_example_content(&self, content: &str) -> (String, String) {
        // Look for code blocks marked with ```
        if let Some(code_start) = content.find("```") {
            if let Some(code_end) = content.rfind("```") {
                if code_end > code_start + 3 {
                    let before = content[..code_start].trim();
                    let code = content[code_start + 3..code_end].trim();
                    let after = content[code_end + 3..].trim();
                    
                    let description = if !before.is_empty() && !after.is_empty() {
                        format!("{}\n{}", before, after)
                    } else if !before.is_empty() {
                        before.to_string()
                    } else {
                        after.to_string()
                    };
                    
                    return (code.to_string(), description);
                }
            }
        }

        // No code blocks found, treat everything as code
        (content.to_string(), String::new())
    }

    /// Find documentation comment preceding a given line
    pub fn find_preceding_comment<'a>(&self, comments: &'a [DocComment], target_line: usize) -> Option<&'a DocComment> {
        comments
            .iter()
            .filter(|comment| comment.line < target_line)
            .max_by_key(|comment| comment.line)
    }
}

impl Default for CommentParser {
    fn default() -> Self {
        Self::new().expect("Failed to create default CommentParser")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_comment_parsing() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
/// This is a function
/// that does something cool
/// @param x the input value
/// @return the result
slay test_function(x normie) normie {
}
"#;

        let comments = parser.parse_comments(source).unwrap();
        assert_eq!(comments.len(), 1);
        
        let comment = &comments[0];
        assert!(comment.description.contains("This is a function"));
        assert_eq!(comment.tags.len(), 2);
    }

    #[test]
    fn test_multiline_comment_parsing() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
/**
 * This is a multiline comment
 * with multiple lines
 * @param x the input
 * @return the output
 */
slay test_function(x normie) normie {
}
"#;

        let comments = parser.parse_comments(source).unwrap();
        assert_eq!(comments.len(), 1);
        
        let comment = &comments[0];
        assert!(comment.description.contains("This is a multiline comment"));
        assert_eq!(comment.tags.len(), 2);
    }

    #[test]
    fn test_example_parsing() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
/// Calculate fibonacci number
/// @example
/// ```
/// let result = fibonacci(5)
/// assert(result == 5)
/// ```
slay fibonacci(n normie) normie {
}
"#;

        let comments = parser.parse_comments(source).unwrap();
        assert_eq!(comments.len(), 1);
        
        let examples = comments[0].get_examples();
        assert_eq!(examples.len(), 1);
        
        if let DocTag::Example { code, .. } = &examples[0] {
            assert!(code.contains("fibonacci(5)"));
        }
    }
}
