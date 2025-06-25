// Enhanced Comment Extractor for CURSED Documentation
// 
// This module provides comprehensive comment extraction from token streams,
// handling multi-line comments, inline documentation, JSDoc-style tags,
// code examples, and cross-references.

use crate::error::{CursedError, SourceLocation};
use crate::documentation::comment_parser::{ParsedComment, DocTag, CodeExample};
use crate::lexer::{Token, TokenType};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tracing::{debug, instrument, warn};

/// Enhanced comment extractor with token stream support
pub struct CommentExtractor {
    /// Known JSDoc-style tags
    /// Cross-reference patterns
/// Cross-reference pattern for detecting references
#[derive(Debug, Clone)]
pub struct CrossReferencePattern {
    /// Pattern to match
    /// Type of reference
/// Types of cross-references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossReferenceType {
/// Enhanced comment with token-level information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedComment {
    /// Base parsed comment
    /// Token range for the comment
    /// Associated code elements
    /// Cross-references found
    /// Code examples with enhanced metadata
    /// JSDoc-style tags
/// Cross-reference information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    /// Referenced item name
    /// Type of reference
    /// Location in comment
    /// Context around the reference
/// Enhanced code example with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedCodeExample {
    /// Base code example
    /// Syntax validation result
    /// Dependencies required
    /// Expected compilation result
    /// Performance characteristics
/// Compilation result for code examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResult {
    /// Whether compilation succeeds
    /// Compilation warnings
    /// Compilation errors
    /// Generated code size
/// JSDoc-style tag with enhanced metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JSDocTag {
    /// Tag name
    /// Tag type information
    /// Tag description
    /// Additional attributes
    /// Nested tags
impl CommentExtractor {
    /// Create a new enhanced comment extractor
    #[instrument]
    pub fn new() -> crate::error::Result<()> {
        let mut jsdoc_tags = HashSet::new();
        
        // Standard JSDoc tags
        jsdoc_tags.insert("param".to_string());
        jsdoc_tags.insert("returns".to_string());
        jsdoc_tags.insert("return".to_string());
        jsdoc_tags.insert("throws".to_string());
        jsdoc_tags.insert("example".to_string());
        jsdoc_tags.insert("see".to_string());
        jsdoc_tags.insert("since".to_string());
        jsdoc_tags.insert("deprecated".to_string());
        jsdoc_tags.insert("author".to_string());
        jsdoc_tags.insert("version".to_string());
        jsdoc_tags.insert("todo".to_string());
        jsdoc_tags.insert("note".to_string());
        jsdoc_tags.insert("warning".to_string());
        jsdoc_tags.insert("internal".to_string());
        jsdoc_tags.insert("public".to_string());
        jsdoc_tags.insert("private".to_string());
        jsdoc_tags.insert("protected".to_string());
        jsdoc_tags.insert("static".to_string());
        jsdoc_tags.insert("abstract".to_string());
        jsdoc_tags.insert("override".to_string());
        jsdoc_tags.insert("implements".to_string());
        jsdoc_tags.insert("extends".to_string());
        jsdoc_tags.insert("namespace".to_string());
        jsdoc_tags.insert("module".to_string());
        jsdoc_tags.insert("class".to_string());
        jsdoc_tags.insert("interface".to_string());
        jsdoc_tags.insert("typedef".to_string());
        jsdoc_tags.insert("callback".to_string());
        jsdoc_tags.insert("enum".to_string());
        jsdoc_tags.insert("memberof".to_string());
        jsdoc_tags.insert("namespace".to_string());
        jsdoc_tags.insert("readonly".to_string());
        jsdoc_tags.insert("async".to_string());
        jsdoc_tags.insert("generator".to_string());

        // Cross-reference patterns
        let crossref_patterns = vec![
            CrossReferencePattern {
            CrossReferencePattern {
            CrossReferencePattern {
        ];

        Ok(Self {
        })
    /// Extract comments from token stream with enhanced analysis
    #[instrument(skip(self, tokens))]
    pub fn extract_comments_from_tokens(
    ) -> crate::error::Result<()> {
        debug!("Extracting comments from {} tokens", tokens.len());
        
        let mut comments = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            if let Some(comment) = self.extract_comment_at_index(tokens, &mut i)? {
                comments.push(comment);
            } else {
                i += 1;
            }
        }

        debug!("Extracted {} enhanced comments", comments.len());
        Ok(comments)
    /// Extract comment starting at a specific token index
    #[instrument(skip(self, tokens))]
    fn extract_comment_at_index(
    ) -> crate::error::Result<()> {
        if *index >= tokens.len() {
            return Ok(None);
        let token = &tokens[*index];
        
        match &token.token_type {
            TokenType::Comment => {
                let start_index = *index;
                let mut comment_text = token.value.clone();
                *index += 1;

                // Check for multi-line comment continuation
                while *index < tokens.len() {
                    let next_token = &tokens[*index];
                    if matches!(next_token.token_type, TokenType::Comment) &&
                       self.is_continuation_comment(&token.value, &next_token.value) {
                        comment_text.push('\n');
                        comment_text.push_str(&next_token.value);
                        *index += 1;
                    } else {
                        break;
                    }
                }

                let end_index = *index - 1;
                
                // Parse the combined comment text
                let parsed_comment = self.parse_enhanced_comment(
                )?;

                // Extract cross-references
                let cross_references = self.extract_cross_references(&comment_text)?;

                // Extract enhanced examples
                let enhanced_examples = self.extract_enhanced_examples(&parsed_comment.examples)?;

                // Extract JSDoc tags
                let jsdoc_tags = self.extract_jsdoc_tags(&parsed_comment.tags)?;

                Ok(Some(EnhancedComment {
                }))
            }
        }
    }

    /// Extract comments before a specific source location
    #[instrument(skip(self, source_code))]
    pub fn extract_comments_before(
    ) -> crate::error::Result<()> {
        let lines: Vec<&str> = source_code.split("\n").collect();
        
        if location.line <= 1 || location.line > lines.len() {
            return Ok(Vec::new());
        let mut comments = Vec::new();
        let mut line_idx = location.line - 2; // Start from line before the declaration
        let mut comment_lines = Vec::new();
        let mut found_comment = false;

        // Look backwards for documentation comments
        loop {
            if line_idx >= lines.len() {
                break;
            let line = lines[line_idx].trim();

            if line.starts_with("///") || line.starts_with("//!") {
                // Documentation comment
                found_comment = true;
                let comment_content = if line.starts_with("///") {
                    line.trim_start_matches("///").trim()
                } else {
                    line.trim_start_matches("//!").trim()
                comment_lines.insert(0, comment_content.to_string());
            } else if line.starts_with("/**") {
                // Start of block comment
                found_comment = true;
                let mut block_lines = Vec::new();
                let mut block_line_idx = line_idx;

                // Extract entire block comment
                while block_line_idx < lines.len() {
                    let block_line = lines[block_line_idx].trim();
                    
                    if block_line.starts_with("/**") {
                        let content = block_line.trim_start_matches("/**").trim();
                        if !content.is_empty() && !content.starts_with("*") {
                            block_lines.push(content.to_string());
                        }
                    } else if block_line.ends_with("*/") {
                        let content = block_line
                            .trim_end_matches("*/")
                            .trim_start_matches("*")
                            .trim();
                        if !content.is_empty() {
                            block_lines.push(content.to_string());
                        }
                        break;
                    } else {
                        let content = block_line.trim_start_matches("*").trim();
                        if !content.is_empty() {
                            block_lines.push(content.to_string());
                        }
                    }

                    block_line_idx += 1;
                comment_lines.splice(0..0, block_lines);
                break;
            } else if line.starts_with("//") || line.is_empty() {
                // Regular comment or empty line - continue looking
            } else {
                // Non-comment line - stop looking
                break;
            if line_idx == 0 {
                break;
            }
            line_idx -= 1;
        if found_comment && !comment_lines.is_empty() {
            let comment_text = comment_lines.join("\n");
            let parsed = self.parse_enhanced_comment(&comment_text, location.line, 1)?;
            comments.push(parsed);
        Ok(comments)
    /// Parse enhanced comment with JSDoc support
    #[instrument(skip(self, content))]
    fn parse_enhanced_comment(
    ) -> crate::error::Result<()> {
        let mut description_lines = Vec::new();
        let mut tags = Vec::new();
        let mut examples = Vec::new();
        let mut current_tag: Option<(String, Vec<String>)> = None;
        let mut in_code_block = false;
        let mut current_example: Option<CodeExample> = None;

        for (line_offset, line) in content.split("\n").enumerate() {
            let line = line.trim();

            // Handle code blocks
            if line.starts_with("```") {
                if in_code_block {
                    // End of code block
                    in_code_block = false;
                    if let Some(example) = current_example.take() {
                        examples.push(example);
                    }
                } else {
                    // Start of code block
                    in_code_block = true;
                    let language = line.trim_start_matches("```").trim();
                    current_example = Some(CodeExample {
                        language: if language.is_empty() { 
                            "cursed".to_string() 
                        } else { 
                            language.to_string() 
                    });
                }
                continue;
            if in_code_block {
                // Add line to current code example
                if let Some(ref mut example) = current_example {
                    if !example.code.is_empty() {
                        example.code.push('\n');
                    }
                    example.code.push_str(line);
                }
                continue;
            // Handle JSDoc-style tags
            if line.starts_with('@') {
                // Finish previous tag if any
                if let Some((tag_name, tag_lines)) = current_tag.take() {
                    tags.push(DocTag {
                    });
                // Parse new tag with type information
                let tag_content = line.trim_start_matches('@');
                let (tag_name, tag_value) = self.parse_jsdoc_tag(tag_content)?;
                current_tag = Some((tag_name, vec![tag_value]));
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
            });
        // Finish last example if any
        if let Some(example) = current_example {
            examples.push(example);
        Ok(ParsedComment {
        })
    /// Parse JSDoc-style tag with type information
    fn parse_jsdoc_tag(&self, tag_content: &str) -> crate::error::Result<()> {
        // Handle typed parameters: @param {string} name - The name parameter
        if let Some(type_end) = tag_content.find('}') {
            if tag_content.starts_with("param {") || tag_content.starts_with("return {") {
                let tag_name = tag_content.split_whitespace().next().unwrap_or("").to_string();
                let rest = &tag_content[tag_name.len()..].trim();
                return Ok((tag_name, rest.to_string()));
            }
        }

        // Handle regular tags
        if let Some(space_pos) = tag_content.find(' ') {
            let tag_name = tag_content[..space_pos].to_string();
            let tag_value = tag_content[space_pos + 1..].trim().to_string();
            Ok((tag_name, tag_value))
        } else {
            Ok((tag_content.to_string(), String::new()))
        }
    }

    /// Check if a comment is a continuation of a previous comment
    fn is_continuation_comment(&self, first: &str, second: &str) -> bool {
        // Check if both are documentation comments
        (first.starts_with("///") && second.starts_with("///")) ||
        (first.starts_with("//!") && second.starts_with("//!")) ||
        (first.starts_with("/**") && !first.ends_with("*/"))
    /// Extract cross-references from comment text
    #[instrument(skip(self, text))]
    fn extract_cross_references(&self, text: &str) -> crate::error::Result<()> {
        let mut references = Vec::new();

        for pattern in &self.crossref_patterns {
            // This is a simplified implementation - a real regex would be better
            if let Some(pos) = text.find(&pattern.pattern) {
                references.push(CrossReference {
                    target: "placeholder".to_string(), // Would extract from regex capture
                });
            }
        }

        Ok(references)
    /// Extract enhanced examples with validation
    fn extract_enhanced_examples(
    ) -> crate::error::Result<()> {
        let mut enhanced = Vec::new();

        for example in examples {
            // Basic syntax validation (simplified)
            let is_valid_syntax = self.validate_example_syntax(&example.code)?;

            enhanced.push(EnhancedCodeExample {
                compilation_result: None, // Would require actual compilation
            });
        Ok(enhanced)
    /// Validate example syntax (simplified)
    fn validate_example_syntax(&self, code: &str) -> crate::error::Result<()> {
        // Basic validation - check for balanced braces and semicolons
        let mut brace_count = 0;
        let mut in_string = false;
        let mut escape_next = false;

        for ch in code.chars() {
            if escape_next {
                escape_next = false;
                continue;
            match ch {
                _ => {}
            }
        Ok(brace_count == 0 && !in_string)
    /// Extract dependencies from example code
    fn extract_example_dependencies(&self, code: &str) -> crate::error::Result<()> {
        let mut dependencies = Vec::new();

        // Look for import statements
        for line in code.split("\n") {
            let line = line.trim();
            if line.starts_with("import ") {
                // Extract module name from import statement
                if let Some(module_start) = line.find('"') {
                    if let Some(module_end) = line[module_start + 1..].find('"') {
                        let module_name = &line[module_start + 1..module_start + 1 + module_end];
                        dependencies.push(module_name.to_string());
                    }
                }
            }
        }

        Ok(dependencies)
    /// Extract JSDoc tags from parsed tags
    fn extract_jsdoc_tags(&self, tags: &[DocTag]) -> crate::error::Result<()> {
        let mut jsdoc_tags = Vec::new();

        for tag in tags {
            if self.jsdoc_tags.contains(&tag.name) {
                let (type_info, description) = self.parse_tag_type_info(&tag.value)?;

                jsdoc_tags.push(JSDocTag {
                });
            }
        }

        Ok(jsdoc_tags)
    /// Parse type information from tag value
    fn parse_tag_type_info(&self, value: &str) -> crate::error::Result<()> {
        // Handle typed format: {string} name - Description
        if value.starts_with('{') {
            if let Some(type_end) = value.find('}') {
                let type_info = value[1..type_end].to_string();
                let rest = value[type_end + 1..].trim();
                
                // Extract description after parameter name
                if let Some(dash_pos) = rest.find(" - ") {
                    let description = rest[dash_pos + 3..].to_string();
                    return Ok((Some(type_info), description));
                } else {
                    return Ok((Some(type_info), rest.to_string()));
                }
            }
        Ok((None, value.to_string()))
    /// Get main description from parsed comments
    pub fn get_main_description(&self, comments: &[ParsedComment]) -> Option<String> {
        for comment in comments {
            if !comment.description.is_empty() {
                return Some(comment.description.clone());
            }
        }
        None
    /// Get tags by name from parsed comments
    pub fn get_tags_by_name(&self, comments: &[ParsedComment], tag_name: &str) -> Vec<String> {
        let mut values = Vec::new();
        
        for comment in comments {
            for tag in &comment.tags {
                if tag.name == tag_name {
                    values.push(tag.value.clone());
                }
            }
        values
    }
}
