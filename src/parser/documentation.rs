//! Documentation comment parsing for the CURSED language
//!
//! This module implements parsing for CURSED documentation comments, which use
//! the /// style for multi-line documentation and support various tags for
//! structured documentation.

use crate::ast::traits::Node;
use crate::ast::documentation::{DocFunction, DocType, DocModule, DocPosition, DocMetadata};
use crate::error::{Error, SourceLocation};
use crate::lexer::Token;
use super::Parser;
use std::collections::HashMap;
use tracing::{debug, instrument, warn};

/// Represents a documentation comment in the CURSED language
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentationComment {
    /// Raw text content of the documentation comment
    pub raw_content: String,
    /// Summary/brief description (first paragraph)
    pub summary: String,
    /// Detailed description (remaining paragraphs)
    pub description: String,
    /// Structured tags (@param, @return, etc.)
    pub tags: HashMap<String, Vec<String>>,
    /// Code examples found in the documentation
    pub examples: Vec<CodeExample>,
    /// Source location of the documentation comment
    pub location: SourceLocation,
}

/// Represents a code example within documentation
#[derive(Debug, Clone, PartialEq)]
pub struct CodeExample {
    /// Language of the code example (e.g., "cursed", "text")
    pub language: String,
    /// Code content
    pub code: String,
    /// Optional description of the example
    pub description: Option<String>,
}

/// Types of documentation comments
#[derive(Debug, Clone, PartialEq)]
pub enum DocumentationType {
    /// Package/module level documentation
    Package,
    /// Function/method documentation
    Function,
    /// Type documentation (struct, interface, etc.)
    Type,
    /// Field/variable documentation
    Field,
    /// General documentation
    General,
}

impl DocumentationComment {
    /// Create a new empty documentation comment
    pub fn new(location: SourceLocation) -> Self {
        Self {
            raw_content: String::new(),
            summary: String::new(),
            description: String::new(),
            tags: HashMap::new(),
            examples: Vec::new(),
            location,
        }
    }

    /// Parse documentation tags from the content
    #[instrument(skip(self), level = "debug")]
    pub fn parse_tags(&mut self) {
        let lines: Vec<&str> = self.raw_content.lines().collect();
        let mut current_tag: Option<String> = None;
        let mut tag_content: Vec<String> = Vec::new();

        for line in lines {
            let trimmed = line.trim_start_matches("/// fr fr")
                             .trim_start_matches("///")
                             .trim();
            
            if trimmed.starts_with('@') {
                // Save previous tag if any
                if let Some(tag_name) = current_tag.take() {
                    self.tags.entry(tag_name).or_insert_with(Vec::new).extend(tag_content.clone());
                    tag_content.clear();
                }
                
                // Parse new tag
                if let Some(space_pos) = trimmed.find(' ') {
                    current_tag = Some(trimmed[1..space_pos].to_string());
                    tag_content.push(trimmed[space_pos + 1..].to_string());
                } else {
                    current_tag = Some(trimmed[1..].to_string());
                }
            } else if current_tag.is_some() && !trimmed.is_empty() {
                // Continue current tag content
                tag_content.push(trimmed.to_string());
            } else if current_tag.is_some() && trimmed.is_empty() {
                // Empty line within tag - preserve spacing
                tag_content.push(String::new());
            }
        }

        // Save final tag
        if let Some(tag_name) = current_tag {
            self.tags.entry(tag_name).or_insert_with(Vec::new).extend(tag_content);
        }
    }

    /// Extract summary and description from content
    #[instrument(skip(self), level = "debug")]
    pub fn parse_content(&mut self) {
        let lines: Vec<&str> = self.raw_content
            .lines()
            .map(|line| {
                line.trim_start_matches("/// fr fr")
                    .trim_start_matches("///")
                    .trim()
            })
            .filter(|line| !line.starts_with('@'))
            .collect();

        if lines.is_empty() {
            return;
        }

        // Find first empty line to separate summary from description
        let mut summary_end = lines.len();
        for (i, line) in lines.iter().enumerate() {
            if line.is_empty() {
                summary_end = i;
                break;
            }
        }

        // Extract summary
        self.summary = lines[..summary_end].join(" ").trim().to_string();

        // Extract description if there's content after the summary
        if summary_end < lines.len() {
            let description_lines: Vec<&str> = lines[summary_end + 1..]
                .iter()
                .skip_while(|line| line.is_empty())
                .copied()
                .collect();
            
            if !description_lines.is_empty() {
                self.description = description_lines.join("\n").trim().to_string();
            }
        }
    }

    /// Extract code examples from the content
    #[instrument(skip(self), level = "debug")]
    pub fn parse_examples(&mut self) {
        let content = &self.raw_content;
        let mut in_code_block = false;
        let mut current_language = String::new();
        let mut current_code = String::new();
        let mut lines = content.lines();

        while let Some(line) = lines.next() {
            let trimmed = line.trim_start_matches("/// fr fr")
                             .trim_start_matches("///")
                             .trim();
            
            if trimmed.starts_with("```") {
                if in_code_block {
                    // End of code block
                    self.examples.push(CodeExample {
                        language: if current_language.is_empty() { "text".to_string() } else { current_language.clone() },
                        code: current_code.trim().to_string(),
                        description: None,
                    });
                    current_code.clear();
                    current_language.clear();
                    in_code_block = false;
                } else {
                    // Start of code block
                    current_language = trimmed[3..].trim().to_string();
                    in_code_block = true;
                }
            } else if in_code_block {
                current_code.push_str(trimmed);
                current_code.push('\n');
            }
        }

        // Handle unterminated code block
        if in_code_block && !current_code.is_empty() {
            warn!("Unterminated code block in documentation");
            self.examples.push(CodeExample {
                language: if current_language.is_empty() { "text".to_string() } else { current_language },
                code: current_code.trim().to_string(),
                description: None,
            });
        }
    }
}

impl Node for DocumentationComment {
    fn token_literal(&self) -> String {
        "///".to_string()
    }

    fn string(&self) -> String {
        self.raw_content.clone()
    }
}

/// Documentation parsing functionality for the Parser
pub trait DocumentationParsing {
    /// Parse documentation comments preceding a declaration
    fn parse_documentation(&mut self) -> Result<Option<DocumentationComment>, Error>;
    
    /// Check if current token is start of documentation comment
    fn is_documentation_comment(&self) -> bool;
    
    /// Parse a single documentation line
    fn parse_documentation_line(&mut self) -> Result<String, Error>;
    
    /// Parse multiple consecutive documentation lines
    fn parse_documentation_block(&mut self) -> Result<Vec<String>, Error>;
    
    /// Associate documentation with the next declaration
    fn parse_documented_declaration(&mut self) -> Result<Box<dyn crate::ast::traits::Statement>, Error>;
}

impl<'a> DocumentationParsing for Parser<'a> {
    #[instrument(skip(self), level = "debug")]
    fn parse_documentation(&mut self) -> Result<Option<DocumentationComment>, Error> {
        if !self.is_documentation_comment() {
            return Ok(None);
        }

        let start_location = SourceLocation {
            line: self.lexer.line,
            column: self.lexer.column,
            file: Some("".to_string()),
            source_line: "".to_string(),
        };

        let doc_lines = self.parse_documentation_block()?;
        
        if doc_lines.is_empty() {
            return Ok(None);
        }

        let mut doc_comment = DocumentationComment::new(start_location);
        doc_comment.raw_content = doc_lines.join("\n");
        
        // Parse the documentation content
        doc_comment.parse_content();
        doc_comment.parse_tags();
        doc_comment.parse_examples();

        debug!(
            summary = %doc_comment.summary,
            tags_count = doc_comment.tags.len(),
            examples_count = doc_comment.examples.len(),
            "Parsed documentation comment"
        );

        Ok(Some(doc_comment))
    }

    #[instrument(skip(self), level = "trace")]
    fn is_documentation_comment(&self) -> bool {
        matches!(self.current_token, Token::LineComment) && 
        self.lexer.input.get(self.lexer.position.saturating_sub(10)..)
            .unwrap_or("")
            .starts_with("/// fr fr")
    }

    #[instrument(skip(self), level = "debug")]
    fn parse_documentation_line(&mut self) -> Result<String, Error> {
        if !matches!(self.current_token, Token::LineComment) {
            return Err(Error::syntax(
                "Expected documentation comment",
                SourceLocation {
                    line: self.lexer.line,
                    column: self.lexer.column,
                    file: Some("".to_string()),
                    source_line: "".to_string(),
                }
            ));
        }

        // Extract the line content - we need to work with the lexer's input directly
        let line_start = self.lexer.line_start_position();
        let line_end = self.lexer.find_line_end();
        let line_content = &self.lexer.input[line_start..line_end];
        
        let doc_content = if line_content.starts_with("/// fr fr") {
            line_content[9..].to_string()
        } else if line_content.starts_with("///") {
            line_content[3..].to_string()
        } else {
            String::new()
        };

        self.next_token()?; // Move past the comment token
        Ok(doc_content)
    }

    #[instrument(skip(self), level = "debug")]
    fn parse_documentation_block(&mut self) -> Result<Vec<String>, Error> {
        let mut doc_lines = Vec::new();

        while self.is_documentation_comment() {
            let line = self.parse_documentation_line()?;
            doc_lines.push(line);
            
            // Skip whitespace to check for next documentation line
            self.skip_whitespace();
        }

        Ok(doc_lines)
    }

    #[instrument(skip(self), level = "debug")]
    fn parse_documented_declaration(&mut self) -> Result<Box<dyn crate::ast::traits::Statement>, Error> {
        // Parse any preceding documentation
        let documentation = self.parse_documentation()?;
        
        // Parse the actual declaration
        let statement = self.parse_statement()?;
        
        // Associate documentation with the statement if both exist
        if let Some(doc) = documentation {
            debug!(
                summary = %doc.summary,
                statement_type = ?statement.token_literal(),
                "Associated documentation with statement"
            );
            
            // Try to downcast and associate documentation with specific statement types
            if let Some(func_stmt) = statement.as_any().downcast_ref::<crate::ast::declarations::FunctionStatement>() {
                // For function statements, we need to create a new one with documentation
                // Since we can't mutate the existing statement, we'll log for now
                debug!("Would associate function documentation");
            } else if let Some(struct_stmt) = statement.as_any().downcast_ref::<crate::ast::declarations::SquadStatement>() {
                // For struct statements
                debug!("Would associate struct documentation");
            }
        }

        Ok(statement)
    }
}

impl<'a> Parser<'a> {
    /// Skip whitespace and handle empty lines while preserving position info
    #[instrument(skip(self), level = "trace")]
    pub(super) fn skip_whitespace(&mut self) {
        // This should be implemented to skip whitespace without affecting tokens
        // For now, it's a placeholder that advances if we hit whitespace tokens
        while matches!(self.current_token, Token::Illegal(_)) {
            if let Err(_) = self.next_token() {
                break;
            }
        }
    }
}

// Helper functions for documentation parsing
impl DocumentationComment {
    /// Validate the documentation structure
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check for required tags in function documentation
        if self.tags.contains_key("param") && !self.tags.contains_key("return") {
            // This might be acceptable for void functions
        }

        // Validate @param tags have descriptions
        if let Some(params) = self.tags.get("param") {
            for param in params {
                if param.trim().is_empty() {
                    errors.push("Empty @param tag found".to_string());
                }
            }
        }

        // Validate code examples
        for (i, example) in self.examples.iter().enumerate() {
            if example.code.trim().is_empty() {
                errors.push(format!("Empty code example at index {}", i));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get all parameter documentation
    pub fn get_parameters(&self) -> Vec<&str> {
        self.tags.get("param")
            .map(|params| params.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get return value documentation
    pub fn get_return_documentation(&self) -> Option<&str> {
        self.tags.get("return")
            .and_then(|returns| returns.first())
            .map(|s| s.as_str())
    }

    /// Get all examples
    pub fn get_examples(&self) -> &[CodeExample] {
        &self.examples
    }

    /// Convert to AST DocFunction for function documentation
    pub fn to_doc_function(&self, name: String) -> DocFunction {
        let mut metadata = DocMetadata::new();
        
        // Map tags to metadata fields
        if let Some(version) = self.tags.get("version").and_then(|v| v.first()) {
            metadata.version = Some(version.clone());
        }
        if let Some(since) = self.tags.get("since").and_then(|v| v.first()) {
            metadata.since = Some(since.clone());
        }
        if let Some(deprecated) = self.tags.get("deprecated").and_then(|v| v.first()) {
            metadata.deprecated = Some(deprecated.clone());
        }

        let mut doc_func = DocFunction::new(
            name,
            format!("{}\n{}", self.summary, self.description),
            DocPosition::new(
                self.location.line,
                self.location.column,
                self.location.file.clone().unwrap_or_default(),
            ),
        ).with_metadata(metadata);

        // Add parameters 
        for param_str in self.tags.get("param").unwrap_or(&vec![]) {
            if let Some(space_pos) = param_str.find(' ') {
                let param_name = param_str[..space_pos].to_string();
                let param_desc = param_str[space_pos + 1..].to_string();
                let doc_param = crate::ast::documentation::DocParameter::new(param_name, param_desc);
                doc_func = doc_func.add_parameter(doc_param);
            }
        }
        
        // Add return information
        if let Some(return_desc) = self.tags.get("return").and_then(|r| r.first()) {
            let doc_return = crate::ast::documentation::DocReturn::new(return_desc.clone());
            doc_func = doc_func.with_returns(doc_return);
        }

        // Add examples
        for example in &self.examples {
            let doc_example = crate::ast::documentation::DocExample::new(example.code.clone())
                .with_title(example.language.clone())
                .with_description(example.description.clone().unwrap_or_default());
            doc_func = doc_func.add_example(doc_example);
        }

        doc_func
    }

    /// Convert to AST DocType for struct/interface documentation
    pub fn to_doc_type(&self, name: String, kind: String) -> DocType {
        let mut metadata = DocMetadata::new();
        
        // Map tags to metadata fields
        if let Some(version) = self.tags.get("version").and_then(|v| v.first()) {
            metadata.version = Some(version.clone());
        }
        if let Some(since) = self.tags.get("since").and_then(|v| v.first()) {
            metadata.since = Some(since.clone());
        }

        let mut doc_type = DocType::new(
            name,
            kind,
            format!("{}\n{}", self.summary, self.description),
            DocPosition::new(
                self.location.line,
                self.location.column,
                self.location.file.clone().unwrap_or_default(),
            ),
        ).with_metadata(metadata);

        // Add fields
        for field_str in self.tags.get("field").unwrap_or(&vec![]) {
            if let Some(space_pos) = field_str.find(' ') {
                let field_name = field_str[..space_pos].to_string();
                let field_desc = field_str[space_pos + 1..].to_string();
                let doc_field = crate::ast::documentation::DocField::new(field_name, field_desc);
                doc_type = doc_type.add_field(doc_field);
            }
        }

        // Add examples
        for example in &self.examples {
            let doc_example = crate::ast::documentation::DocExample::new(example.code.clone())
                .with_title(example.language.clone())
                .with_description(example.description.clone().unwrap_or_default());
            doc_type = doc_type.add_example(doc_example);
        }

        doc_type
    }

    /// Convert to AST DocModule for package/module documentation
    pub fn to_doc_module(&self, name: String) -> DocModule {
        let mut metadata = DocMetadata::new();
        
        // Map tags to metadata fields
        if let Some(version) = self.tags.get("version").and_then(|v| v.first()) {
            metadata.version = Some(version.clone());
        }
        if let Some(author) = self.tags.get("author").and_then(|v| v.first()) {
            metadata.author = Some(author.clone());
        }

        let mut doc_module = DocModule::new(
            name,
            format!("{}\n{}", self.summary, self.description),
            DocPosition::new(
                self.location.line,
                self.location.column,
                self.location.file.clone().unwrap_or_default(),
            ),
        ).with_metadata(metadata);

        // Add examples
        for example in &self.examples {
            let doc_example = crate::ast::documentation::DocExample::new(example.code.clone())
                .with_title(example.language.clone())
                .with_description(example.description.clone().unwrap_or_default());
            doc_module = doc_module.add_example(doc_example);
        }

        // Add sections from tags
        for (tag_name, tag_values) in &self.tags {
            if !["param", "return", "version", "author", "example"].contains(&tag_name.as_str()) {
                doc_module = doc_module.add_section(tag_name.clone(), tag_values.join("\n"));
            }
        }

        doc_module
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_documentation_comment_creation() {
        let location = SourceLocation { line: 1, column: 1 };
        let doc = DocumentationComment::new(location);
        
        assert_eq!(doc.summary, "");
        assert_eq!(doc.description, "");
        assert!(doc.tags.is_empty());
        assert!(doc.examples.is_empty());
    }

    #[test]
    fn test_parse_simple_content() {
        let location = SourceLocation { line: 1, column: 1 };
        let mut doc = DocumentationComment::new(location);
        doc.raw_content = "/// Simple documentation\n/// on multiple lines".to_string();
        
        doc.parse_content();
        
        assert_eq!(doc.summary, "Simple documentation on multiple lines");
        assert_eq!(doc.description, "");
    }

    #[test]
    fn test_parse_content_with_description() {
        let location = SourceLocation { line: 1, column: 1 };
        let mut doc = DocumentationComment::new(location);
        doc.raw_content = "/// Brief summary\n///\n/// Detailed description\n/// continues here".to_string();
        
        doc.parse_content();
        
        assert_eq!(doc.summary, "Brief summary");
        assert_eq!(doc.description, "Detailed description\ncontinues here");
    }

    #[test]
    fn test_parse_tags() {
        let location = SourceLocation { line: 1, column: 1 };
        let mut doc = DocumentationComment::new(location);
        doc.raw_content = "/// Function docs\n/// @param x the input value\n/// @return the result".to_string();
        
        doc.parse_tags();
        
        assert_eq!(doc.tags.get("param").unwrap(), &vec!["x the input value"]);
        assert_eq!(doc.tags.get("return").unwrap(), &vec!["the result"]);
    }

    #[test]
    fn test_parse_code_examples() {
        let location = SourceLocation { line: 1, column: 1 };
        let mut doc = DocumentationComment::new(location);
        doc.raw_content = "/// Example usage:\n/// ```cursed\n/// let x = 42\n/// ```".to_string();
        
        doc.parse_examples();
        
        assert_eq!(doc.examples.len(), 1);
        assert_eq!(doc.examples[0].language, "cursed");
        assert_eq!(doc.examples[0].code, "let x = 42");
    }

    #[test]
    fn test_validation() {
        let location = SourceLocation { line: 1, column: 1 };
        let mut doc = DocumentationComment::new(location);
        doc.tags.insert("param".to_string(), vec!["x the value".to_string()]);
        doc.tags.insert("return".to_string(), vec!["the result".to_string()]);
        
        assert!(doc.validate().is_ok());
    }

    #[test]
    fn test_validation_with_empty_param() {
        let location = SourceLocation { line: 1, column: 1 };
        let mut doc = DocumentationComment::new(location);
        doc.tags.insert("param".to_string(), vec!["".to_string()]);
        
        let result = doc.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&"Empty @param tag found".to_string()));
    }
}
