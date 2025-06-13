//! Documentation Comment Parser
//! 
//! Parses documentation comments from CURSED source code and extracts structured information.

use super::generator::{DocumentationItem, Parameter, Example};
use crate::error::{Error, SourceLocation};
use crate::lexer::{Lexer, Token, TokenType};
use std::collections::HashMap;
use regex::Regex;

/// Documentation comment parser
pub struct CommentParser {
    tag_regex: Regex,
    example_regex: Regex,
    code_block_regex: Regex,
}

impl CommentParser {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            tag_regex: Regex::new(r"@(\w+)\s+(.+)")
                .map_err(|e| Error::Parse(format!("Regex error: {}", e)))?,
            example_regex: Regex::new(r"```(\w+)?\s*\n(.*?)\n```")
                .map_err(|e| Error::Parse(format!("Regex error: {}", e)))?,
            code_block_regex: Regex::new(r"`([^`]+)`")
                .map_err(|e| Error::Parse(format!("Regex error: {}", e)))?,
        })
    }

    /// Parse documentation comments for a specific item
    pub fn parse_item_documentation(
        &self,
        source: &str,
        item_location: &SourceLocation,
    ) -> Result<ParsedDocumentation, Error> {
        let lines: Vec<&str> = source.lines().collect();
        let doc_lines = self.extract_doc_comments(&lines, item_location)?;
        
        if doc_lines.is_empty() {
            return Ok(ParsedDocumentation::empty());
        }

        let raw_content = doc_lines.join("\n");
        self.parse_doc_content(&raw_content)
    }

    /// Extract documentation comment lines preceding an item
    fn extract_doc_comments(
        &self,
        lines: &[&str],
        item_location: &SourceLocation,
    ) -> Result<Vec<String>, Error> {
        let item_line = item_location.line.saturating_sub(1); // Convert to 0-based
        let mut doc_lines = Vec::new();
        
        // Look backwards from the item line to find documentation comments
        let mut current_line = item_line;
        
        while current_line > 0 {
            current_line -= 1;
            
            if let Some(line) = lines.get(current_line) {
                let trimmed = line.trim();
                
                if trimmed.starts_with("///") {
                    // Extract the comment content (remove ///)
                    let content = trimmed.strip_prefix("///").unwrap_or("").trim();
                    doc_lines.insert(0, content.to_string());
                } else if trimmed.starts_with("//") {
                    // Skip regular comments
                    continue;
                } else if trimmed.is_empty() {
                    // Skip empty lines within doc comments
                    if !doc_lines.is_empty() {
                        doc_lines.insert(0, String::new());
                    }
                } else {
                    // Hit non-documentation content, stop
                    break;
                }
            } else {
                break;
            }
        }
        
        // Remove leading empty lines
        while doc_lines.first().map_or(false, |line| line.is_empty()) {
            doc_lines.remove(0);
        }
        
        Ok(doc_lines)
    }

    /// Parse documentation content into structured format
    pub fn parse_doc_content(&self, content: &str) -> Result<ParsedDocumentation, Error> {
        let mut parsed = ParsedDocumentation::empty();
        
        // Extract summary (first non-empty line)
        if let Some(first_line) = content.lines().find(|line| !line.trim().is_empty()) {
            parsed.summary = first_line.trim().to_string();
        }

        // Parse tags and structured content
        let mut current_section = String::new();
        let mut in_example = false;
        let mut current_example = String::new();
        let mut example_language = String::new();
        let mut example_title: Option<String> = None;
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            // Handle code blocks (examples)
            if trimmed.starts_with("```") {
                if in_example {
                    // End of example
                    if !current_example.trim().is_empty() {
                        parsed.examples.push(Example {
                            title: example_title.take(),
                            description: None,
                            code: current_example.trim().to_string(),
                            language: if example_language.is_empty() { 
                                "cursed".to_string() 
                            } else { 
                                example_language.clone() 
                            },
                            output: None,
                        });
                    }
                    current_example.clear();
                    example_language.clear();
                    in_example = false;
                } else {
                    // Start of example
                    example_language = trimmed.strip_prefix("```").unwrap_or("").to_string();
                    in_example = true;
                }
                continue;
            }
            
            if in_example {
                current_example.push_str(line);
                current_example.push('\n');
                continue;
            }

            // Handle @ tags
            if let Some(captures) = self.tag_regex.captures(trimmed) {
                let tag_name = captures.get(1).unwrap().as_str();
                let tag_content = captures.get(2).unwrap().as_str();
                
                match tag_name {
                    "param" => {
                        if let Some(param) = self.parse_param_tag(tag_content) {
                            parsed.parameters.push(param);
                        }
                    }
                    "return" | "returns" => {
                        parsed.return_doc = Some(tag_content.to_string());
                    }
                    "example" => {
                        example_title = Some(tag_content.to_string());
                    }
                    "throws" | "error" => {
                        parsed.tags.entry("throws".to_string())
                            .or_insert_with(Vec::new)
                            .push(tag_content.to_string());
                    }
                    "since" => {
                        parsed.tags.entry("since".to_string())
                            .or_insert_with(Vec::new)
                            .push(tag_content.to_string());
                    }
                    "deprecated" => {
                        parsed.tags.entry("deprecated".to_string())
                            .or_insert_with(Vec::new)
                            .push(tag_content.to_string());
                    }
                    _ => {
                        // Generic tag
                        parsed.tags.entry(tag_name.to_string())
                            .or_insert_with(Vec::new)
                            .push(tag_content.to_string());
                    }
                }
            } else if !trimmed.is_empty() {
                // Regular description content
                if !current_section.is_empty() {
                    current_section.push('\n');
                }
                current_section.push_str(line);
            }
        }
        
        // Clean up description
        if !current_section.trim().is_empty() {
            parsed.description = current_section.trim().to_string();
        }

        Ok(parsed)
    }

    /// Parse @param tag content
    fn parse_param_tag(&self, content: &str) -> Option<Parameter> {
        // Expected formats:
        // @param name Description
        // @param name type Description
        // @param name: type Description
        
        let parts: Vec<&str> = content.splitn(3, ' ').collect();
        
        if parts.is_empty() {
            return None;
        }

        let name = parts[0].trim_end_matches(':').to_string();
        
        if parts.len() == 1 {
            // Just name
            return Some(Parameter {
                name,
                type_name: None,
                description: String::new(),
                default_value: None,
            });
        }

        // Try to determine if second part is type or description
        let (type_name, description) = if parts.len() == 2 {
            // Could be either "name type" or "name description"
            let second_part = parts[1];
            if self.looks_like_type(second_part) {
                (Some(second_part.to_string()), String::new())
            } else {
                (None, second_part.to_string())
            }
        } else {
            // Three parts: name, type, description
            (Some(parts[1].to_string()), parts[2..].join(" "))
        };

        Some(Parameter {
            name,
            type_name,
            description,
            default_value: None,
        })
    }

    /// Heuristic to determine if a string looks like a type annotation
    fn looks_like_type(&self, s: &str) -> bool {
        // Common type patterns in CURSED
        let type_patterns = [
            "i32", "i64", "f64", "bool", "string", "str",
            "[]", "map", "chan", "interface", "struct",
            "Option", "Result", "Vec", "Array",
        ];

        let lowercase = s.to_lowercase();
        
        // Check exact matches
        if type_patterns.iter().any(|&pattern| lowercase.contains(pattern)) {
            return true;
        }

        // Check for type-like patterns
        s.contains('[') && s.contains(']') || // Array types
        s.contains('<') && s.contains('>') || // Generic types
        s.chars().next().map_or(false, |c| c.is_uppercase()) // Capitalized types
    }

    /// Extract inline code snippets from description
    pub fn extract_inline_code(&self, text: &str) -> Vec<String> {
        self.code_block_regex
            .captures_iter(text)
            .map(|cap| cap.get(1).unwrap().as_str().to_string())
            .collect()
    }

    /// Parse multi-line documentation from token stream
    pub fn parse_from_tokens(&self, tokens: &[Token]) -> Result<Vec<ParsedDocumentation>, Error> {
        let mut docs = Vec::new();
        let mut current_doc_tokens = Vec::new();
        
        for (i, token) in tokens.iter().enumerate() {
            match &token.token_type {
                TokenType::Comment if token.literal.starts_with("///") => {
                    current_doc_tokens.push(token.clone());
                }
                _ => {
                    // If we have accumulated doc tokens and hit a non-doc token,
                    // this might be the item the docs apply to
                    if !current_doc_tokens.is_empty() {
                        if let Some(item_token) = self.find_next_documentable_token(&tokens[i..]) {
                            let doc_content = current_doc_tokens
                                .iter()
                                .map(|t| t.literal.strip_prefix("///").unwrap_or("").trim())
                                .collect::<Vec<_>>()
                                .join("\n");
                            
                            if let Ok(parsed) = self.parse_doc_content(&doc_content) {
                                docs.push(parsed);
                            }
                        }
                        current_doc_tokens.clear();
                    }
                }
            }
        }
        
        Ok(docs)
    }

    /// Find the next token that can have documentation
    fn find_next_documentable_token(&self, tokens: &[Token]) -> Option<&Token> {
        for token in tokens {
            match &token.token_type {
                TokenType::Slay |       // function
                TokenType::Squad |      // struct
                TokenType::Collab |     // interface
                TokenType::Sus |        // variable
                TokenType::Facts |      // constant
                TokenType::Vibe |       // package
                TokenType::Identifier => return Some(token),
                TokenType::Newline | TokenType::Comment => continue,
                _ => break,
            }
        }
        None
    }
}

impl Default for CommentParser {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            tag_regex: Regex::new("").unwrap(),
            example_regex: Regex::new("").unwrap(), 
            code_block_regex: Regex::new("").unwrap(),
        })
    }
}

/// Parsed documentation content
#[derive(Debug, Clone)]
pub struct ParsedDocumentation {
    pub summary: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub return_doc: Option<String>,
    pub examples: Vec<Example>,
    pub tags: HashMap<String, Vec<String>>,
}

impl ParsedDocumentation {
    pub fn empty() -> Self {
        Self {
            summary: String::new(),
            description: String::new(),
            parameters: Vec::new(),
            return_doc: None,
            examples: Vec::new(),
            tags: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.summary.is_empty() && 
        self.description.is_empty() && 
        self.parameters.is_empty() && 
        self.return_doc.is_none() && 
        self.examples.is_empty() && 
        self.tags.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_doc_comment() {
        let parser = CommentParser::new().unwrap();
        let content = "This is a summary\n\nThis is a longer description.";
        
        let result = parser.parse_doc_content(content).unwrap();
        
        assert_eq!(result.summary, "This is a summary");
        assert_eq!(result.description, "This is a summary\n\nThis is a longer description.");
    }

    #[test]
    fn test_parse_param_tag() {
        let parser = CommentParser::new().unwrap();
        let content = "@param name string The name parameter";
        
        let result = parser.parse_doc_content(content).unwrap();
        
        assert_eq!(result.parameters.len(), 1);
        assert_eq!(result.parameters[0].name, "name");
        assert_eq!(result.parameters[0].type_name, Some("string".to_string()));
        assert_eq!(result.parameters[0].description, "The name parameter");
    }

    #[test]
    fn test_parse_example() {
        let parser = CommentParser::new().unwrap();
        let content = r#"Function example
        
```cursed
slay hello() {
    println("Hello!")
}
```"#;
        
        let result = parser.parse_doc_content(content).unwrap();
        
        assert_eq!(result.examples.len(), 1);
        assert_eq!(result.examples[0].language, "cursed");
        assert!(result.examples[0].code.contains("slay hello()"));
    }

    #[test]
    fn test_extract_doc_comments() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
/// This is a doc comment
/// with multiple lines
/// @param x The parameter
slay function(x i32) {
    // Regular comment
    return x
}
"#;
        
        let lines: Vec<&str> = source.lines().collect();
        let location = SourceLocation { line: 5, column: 1, file: None };
        
        let doc_lines = parser.extract_doc_comments(&lines, &location).unwrap();
        
        assert_eq!(doc_lines.len(), 3);
        assert_eq!(doc_lines[0], "This is a doc comment");
        assert_eq!(doc_lines[1], "with multiple lines");
        assert_eq!(doc_lines[2], "@param x The parameter");
    }
}
