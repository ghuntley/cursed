//! Comment Parser for CURSED Documentation
//! 
//! Extracts and parses documentation comments from CURSED source code.
//! Supports both single-line and multi-line comment formats.

use std::collections::HashMap;
use regex::Regex;
use crate::error::CursedError;
use crate::documentation::{DocumentedFunction, Parameter};

/// Comment parser for extracting documentation from CURSED source code
pub struct CommentParser {
    /// Patterns for different comment types
    comment_patterns: Vec<Regex>,
    /// Pattern for function documentation
    function_doc_pattern: Regex,
    /// Pattern for parameter documentation
    param_pattern: Regex,
    /// Pattern for return documentation
    return_pattern: Regex,
    /// Pattern for example documentation
    example_pattern: Regex,
}

impl CommentParser {
    /// Create a new comment parser
    pub fn new() -> Result<Self, CursedError> {
        let comment_patterns = vec![
            Regex::new(r"^\s*fr fr\s+(.*)$").unwrap(),
            Regex::new(r"^\s*//\s+(.*)$").unwrap(),
            Regex::new(r"/\*\*(.*?)\*/").unwrap(),
            Regex::new(r"/\*(.*?)\*/").unwrap(),
        ];

        let function_doc_pattern = Regex::new(r"@description\s+(.*)").unwrap();
        let param_pattern = Regex::new(r"@param\s+(\w+)\s+\(([^)]+)\)\s+(.*)").unwrap();
        let return_pattern = Regex::new(r"@return\s+\(([^)]+)\)\s+(.*)").unwrap();
        let example_pattern = Regex::new(r"@example\s+(.*)").unwrap();

        Ok(Self {
            comment_patterns,
            function_doc_pattern,
            param_pattern,
            return_pattern,
            example_pattern,
        })
    }

    /// Parse documentation comments from source code
    pub fn parse_comments(&self, source: &str) -> Result<Vec<DocumentationComment>, CursedError> {
        let mut comments = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        let mut current_comment = String::new();
        let mut comment_start_line = 0;
        let mut in_block_comment = false;

        for (line_num, line) in lines.iter().enumerate() {
            if let Some(comment_text) = self.extract_comment_text(line) {
                if current_comment.is_empty() {
                    comment_start_line = line_num + 1;
                }
                
                if !current_comment.is_empty() {
                    current_comment.push('\n');
                }
                current_comment.push_str(&comment_text);
                
                // Check if this is a block comment
                if line.contains("/*") && !line.contains("*/") {
                    in_block_comment = true;
                } else if line.contains("*/") && in_block_comment {
                    in_block_comment = false;
                }
            } else if !current_comment.is_empty() && !in_block_comment {
                // End of comment block
                let parsed_comment = self.parse_documentation_comment(&current_comment, comment_start_line)?;
                comments.push(parsed_comment);
                current_comment.clear();
            }
        }

        // Handle trailing comment
        if !current_comment.is_empty() {
            let parsed_comment = self.parse_documentation_comment(&current_comment, comment_start_line)?;
            comments.push(parsed_comment);
        }

        Ok(comments)
    }

    /// Extract comment text from a line
    fn extract_comment_text(&self, line: &str) -> Option<String> {
        let line = line.trim();
        
        // Handle inline comments (fr fr and //)
        if line.starts_with("fr fr") {
            return Some(line.strip_prefix("fr fr").unwrap().trim().to_string());
        }
        if line.starts_with("//") {
            return Some(line.strip_prefix("//").unwrap().trim().to_string());
        }
        
        // Handle block comment start/end
        if line.starts_with("/*") && line.ends_with("*/") {
            // Single line block comment
            let content = line.strip_prefix("/*").unwrap().strip_suffix("*/").unwrap();
            return Some(content.trim().to_string());
        }
        
        // Handle multi-line block comment lines
        if line.starts_with("/*") {
            let content = line.strip_prefix("/*").unwrap();
            return Some(content.trim().to_string());
        }
        if line.ends_with("*/") {
            let content = line.strip_suffix("*/").unwrap();
            return Some(content.trim().to_string());
        }
        if line.starts_with("*") {
            let content = line.strip_prefix("*").unwrap();
            return Some(content.trim().to_string());
        }
        
        None
    }

    /// Parse a documentation comment block
    fn parse_documentation_comment(&self, comment: &str, line_number: usize) -> Result<DocumentationComment, CursedError> {
        let mut doc_comment = DocumentationComment {
            line_number,
            description: String::new(),
            parameters: Vec::new(),
            return_type: String::new(),
            return_description: String::new(),
            examples: Vec::new(),
            tags: Vec::new(),
            raw_text: comment.to_string(),
        };

        // Extract description
        let mut description_lines = Vec::new();
        let mut current_section = Section::Description;

        for line in comment.lines() {
            let trimmed = line.trim();
            
            // Check for special tags
            if trimmed.starts_with("@") {
                // Process previous section
                if !description_lines.is_empty() && current_section == Section::Description {
                    doc_comment.description = description_lines.join("\n").trim().to_string();
                    description_lines.clear();
                }

                // Parse the tag
                self.parse_tag(trimmed, &mut doc_comment)?;
                current_section = Section::Tag;
            } else if current_section == Section::Description {
                description_lines.push(trimmed);
            }
        }

        // Finalize description
        if !description_lines.is_empty() {
            doc_comment.description = description_lines.join("\n").trim().to_string();
        }

        Ok(doc_comment)
    }

    /// Parse a documentation tag
    fn parse_tag(&self, tag_line: &str, doc_comment: &mut DocumentationComment) -> Result<(), CursedError> {
        // Parse parameters
        if let Some(captures) = self.param_pattern.captures(tag_line) {
            let param_name = captures.get(1).unwrap().as_str().to_string();
            let param_type = captures.get(2).unwrap().as_str().to_string();
            let param_description = captures.get(3).unwrap().as_str().to_string();
            
            doc_comment.parameters.push(Parameter {
                name: param_name,
                param_type,
                description: param_description,
                default_value: None,
            });
        }
        // Parse return information
        else if let Some(captures) = self.return_pattern.captures(tag_line) {
            doc_comment.return_type = captures.get(1).unwrap().as_str().to_string();
            doc_comment.return_description = captures.get(2).unwrap().as_str().to_string();
        }
        // Parse examples
        else if tag_line.starts_with("@example") {
            let example_text = tag_line.strip_prefix("@example").unwrap().trim();
            if !example_text.is_empty() {
                doc_comment.examples.push(example_text.to_string());
            }
        }
        // Parse other tags
        else if tag_line.starts_with("@") {
            if let Some(space_pos) = tag_line.find(' ') {
                let tag_name = &tag_line[1..space_pos];
                let tag_value = &tag_line[space_pos + 1..];
                doc_comment.tags.push(DocumentationTag {
                    name: tag_name.to_string(),
                    value: tag_value.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Extract function documentation from comments
    pub fn extract_function_docs(&self, comments: &[DocumentationComment], function_name: &str) -> Option<DocumentedFunction> {
        // Find the comment that appears right before the function
        for comment in comments {
            if !comment.description.is_empty() {
                // Basic function documentation structure
                let function_doc = DocumentedFunction {
                    name: function_name.to_string(),
                    signature: String::new(), // Will be filled by AST parser
                    description: comment.description.clone(),
                    parameters: comment.parameters.clone(),
                    return_type: comment.return_type.clone(),
                    return_description: comment.return_description.clone(),
                    examples: comment.examples.clone(),
                    source_file: String::new(), // Will be filled by caller
                    source_line: comment.line_number,
                    visibility: "public".to_string(), // Default visibility
                };

                return Some(function_doc);
            }
        }

        None
    }

    /// Parse module-level documentation
    pub fn parse_module_docs(&self, source: &str) -> Result<ModuleDocumentation, CursedError> {
        let comments = self.parse_comments(source)?;
        
        let mut module_doc = ModuleDocumentation {
            description: String::new(),
            author: String::new(),
            version: String::new(),
            since: String::new(),
            examples: Vec::new(),
            tags: Vec::new(),
        };

        // Look for module-level documentation (usually at the top of the file)
        for comment in &comments {
            if comment.line_number <= 10 { // Consider first 10 lines as module-level
                if !comment.description.is_empty() {
                    module_doc.description = comment.description.clone();
                }
                
                // Extract module-level tags
                for tag in &comment.tags {
                    match tag.name.as_str() {
                        "author" => module_doc.author = tag.value.clone(),
                        "version" => module_doc.version = tag.value.clone(),
                        "since" => module_doc.since = tag.value.clone(),
                        _ => module_doc.tags.push(tag.clone()),
                    }
                }
                
                module_doc.examples.extend(comment.examples.clone());
                break;
            }
        }

        Ok(module_doc)
    }

    /// Extract inline documentation from code
    pub fn extract_inline_docs(&self, source: &str) -> Result<Vec<InlineDocumentation>, CursedError> {
        let mut inline_docs = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            // Look for inline comments after code
            if let Some(code_comment_pos) = line.find("fr fr") {
                let code_part = &line[..code_comment_pos].trim();
                let comment_part = &line[code_comment_pos + 5..].trim();
                
                if !code_part.is_empty() && !comment_part.is_empty() {
                    inline_docs.push(InlineDocumentation {
                        line_number: line_num + 1,
                        code: code_part.to_string(),
                        comment: comment_part.to_string(),
                    });
                }
            }
        }

        Ok(inline_docs)
    }

    /// Validate documentation completeness
    pub fn validate_documentation(&self, source: &str) -> Result<ValidationReport, CursedError> {
        let comments = self.parse_comments(source)?;
        let inline_docs = self.extract_inline_docs(source)?;
        
        let mut report = ValidationReport {
            total_functions: 0,
            documented_functions: 0,
            missing_docs: Vec::new(),
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        // Simple function detection (this would be more sophisticated in practice)
        let function_regex = Regex::new(r"^\s*slay\s+(\w+)\s*\(").unwrap();
        let lines: Vec<&str> = source.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            if let Some(captures) = function_regex.captures(line) {
                let function_name = captures.get(1).unwrap().as_str();
                report.total_functions += 1;
                
                // Check if there's documentation for this function
                let has_docs = comments.iter().any(|comment| {
                    comment.line_number <= line_num && 
                    comment.line_number > line_num.saturating_sub(3) &&
                    !comment.description.is_empty()
                });
                
                if has_docs {
                    report.documented_functions += 1;
                } else {
                    report.missing_docs.push(format!("Function '{}' at line {}", function_name, line_num + 1));
                }
            }
        }

        // Check for common documentation issues
        for comment in &comments {
            if comment.description.is_empty() {
                report.warnings.push(format!("Empty comment block at line {}", comment.line_number));
            }
            
            if comment.description.len() < 10 {
                report.warnings.push(format!("Very short description at line {}", comment.line_number));
            }
        }

        Ok(report)
    }
}

/// Represents a parsed documentation comment
#[derive(Debug, Clone)]
pub struct DocumentationComment {
    pub line_number: usize,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub return_description: String,
    pub examples: Vec<String>,
    pub tags: Vec<DocumentationTag>,
    pub raw_text: String,
}

/// Documentation tag
#[derive(Debug, Clone)]
pub struct DocumentationTag {
    pub name: String,
    pub value: String,
}

/// Module-level documentation
#[derive(Debug, Clone)]
pub struct ModuleDocumentation {
    pub description: String,
    pub author: String,
    pub version: String,
    pub since: String,
    pub examples: Vec<String>,
    pub tags: Vec<DocumentationTag>,
}

/// Inline documentation
#[derive(Debug, Clone)]
pub struct InlineDocumentation {
    pub line_number: usize,
    pub code: String,
    pub comment: String,
}

/// Documentation validation report
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub total_functions: usize,
    pub documented_functions: usize,
    pub missing_docs: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Current parsing section
#[derive(Debug, PartialEq)]
enum Section {
    Description,
    Tag,
}

impl Default for CommentParser {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_comment_parsing() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
fr fr This is a function that adds two numbers
fr fr @param a (normie) First number
fr fr @param b (normie) Second number  
fr fr @return (normie) Sum of the two numbers
slay add(a normie, b normie) normie {
    damn a + b
}
"#;

        let comments = parser.parse_comments(source).unwrap();
        assert_eq!(comments.len(), 1);
        assert!(comments[0].description.contains("adds two numbers"));
        assert_eq!(comments[0].parameters.len(), 2);
        assert_eq!(comments[0].return_type, "normie");
    }

    #[test]
    fn test_block_comment_parsing() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
/*
 * This is a block comment
 * @param x (normie) Input parameter
 * @return (normie) Output value
 */
slay process(x normie) normie {
    damn x * 2
}
"#;

        let comments = parser.parse_comments(source).unwrap();
        assert_eq!(comments.len(), 1);
        assert!(comments[0].description.contains("block comment"));
        assert_eq!(comments[0].parameters.len(), 1);
    }

    #[test]
    fn test_inline_documentation_extraction() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
sus x normie = 42  fr fr Magic number
sus y normie = x * 2  fr fr Double the magic
"#;

        let inline_docs = parser.extract_inline_docs(source).unwrap();
        assert_eq!(inline_docs.len(), 2);
        assert!(inline_docs[0].comment.contains("Magic number"));
        assert!(inline_docs[1].comment.contains("Double the magic"));
    }

    #[test]
    fn test_validation_report() {
        let parser = CommentParser::new().unwrap();
        let source = r#"
fr fr This function is documented
slay documented_function() {
    damn 42
}

slay undocumented_function() {
    damn 0
}
"#;

        let report = parser.validate_documentation(source).unwrap();
        assert_eq!(report.total_functions, 2);
        assert_eq!(report.documented_functions, 1);
        assert_eq!(report.missing_docs.len(), 1);
    }
}
