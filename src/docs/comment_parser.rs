//! Documentation Comment Parser
//! 
//! Parses documentation comments from CURSED source code and extracts structured information.

use crate::error::{Error, SourceLocation};
use crate::docs::generator::Example;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Parsed documentation structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedDocumentation {
    pub summary: String,
    pub description: String,
    pub tags: HashMap<String, Vec<String>>,
    pub examples: Vec<Example>,
    pub see_also: Vec<String>,
    pub since: Option<String>,
    pub deprecated: Option<String>,
    pub author: Option<String>,
}

/// Documentation comment parser
pub struct CommentParser {
    // Parser configuration
    allow_html: bool,
    extract_examples: bool,
    validate_links: bool,
}

impl CommentParser {
    /// Create a new comment parser with default settings
    pub fn new() -> Result<(), Error> {
        Ok(Self {
            allow_html: true,
            extract_examples: true,
            validate_links: false,
        })
    }

    /// Create parser with custom configuration
    pub fn with_config(allow_html: bool, extract_examples: bool, validate_links: bool) -> Self {
        Self {
            allow_html,
            extract_examples,
            validate_links,
        }
    }

    /// Parse documentation content from a raw string
    pub fn parse_doc_content(&self, content: &str) -> Result<(), Error> {
        let mut parsed = ParsedDocumentation {
            summary: String::new(),
            description: String::new(),
            tags: HashMap::new(),
            examples: Vec::new(),
            see_also: Vec::new(),
            since: None,
            deprecated: None,
            author: None,
        };

        let lines = content.split("\n").collect::<Vec<_>>();
        let mut current_section = ParsingSection::Summary;
        let mut current_example: Option<ExampleBuilder> = None;
        let mut description_lines = Vec::new();

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Skip empty lines in summary
            if current_section == ParsingSection::Summary && trimmed.is_empty() {
                current_section = ParsingSection::Description;
                continue;
            }

            // Handle doc comment prefixes
            let clean_line = self.clean_doc_line(trimmed);

            // Process tags
            if let Some(tag_info) = self.parse_tag_line(&clean_line) {
                self.process_tag(&mut parsed, &mut current_example, tag_info)?;
                current_section = ParsingSection::Tags;
                continue;
            }

            // Process code blocks for examples
            if self.extract_examples {
                if let Some(example) = &mut current_example {
                    if clean_line.starts_with("```") {
                        // End code block
                        let built_example = example.build();
                        parsed.examples.push(built_example);
                        current_example = None;
                    } else {
                        example.add_code_line(&clean_line);
                    }
                    continue;
                } else if clean_line.starts_with("```") {
                    // Start code block
                    let language = clean_line.strip_prefix("```").unwrap_or("cursed").to_string();
                    current_example = Some(ExampleBuilder::new(language));
                    continue;
                }
            }

            // Process content based on current section
            match current_section {
                ParsingSection::Summary => {
                    if !clean_line.is_empty() {
                        if parsed.summary.is_empty() {
                            parsed.summary = clean_line.to_string();
                        } else {
                            parsed.summary.push(' ');
                            parsed.summary.push_str(&clean_line);
                        }
                    }
                }
                ParsingSection::Description | ParsingSection::Tags => {
                    if !clean_line.is_empty() {
                        description_lines.push(clean_line);
                    }
                }
            }
        }

        // Finalize any remaining example
        if let Some(example) = current_example {
            parsed.examples.push(example.build());
        }

        // Join description lines
        if !description_lines.is_empty() {
            parsed.description = description_lines.join(" ");
        }

        // Post-process parsed content
        self.post_process(&mut parsed)?;

        Ok(parsed)
    }

    /// Parse documentation for a specific item at a location
    pub fn parse_item_documentation(&self, source: &str, location: &SourceLocation) -> Result<(), Error> {
        let doc_content = self.extract_doc_comments_at_location(source, location)?;
        self.parse_doc_content(&doc_content)
    }

    /// Extract documentation comments preceding a location
    fn extract_doc_comments_at_location(&self, source: &str, location: &SourceLocation) -> Result<(), Error> {
        let lines = source.split("\n").collect::<Vec<_>>();
        let mut doc_lines = Vec::new();
        
        // Look backwards from the location for doc comments
        let start_line = if location.line > 20 { location.line - 20 } else { 1 };
        let end_line = location.line.saturating_sub(1);
        
        let mut found_doc = false;
        for line_num in (start_line..=end_line).rev() {
            if let Some(line) = lines.get((line_num - 1) as usize) {
                let trimmed = line.trim();
                
                if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                    doc_lines.insert(0, trimmed.to_string());
                    found_doc = true;
                } else if found_doc && (trimmed.is_empty() || trimmed.starts_with("//")) {
                    // Continue collecting if we're in a doc block
                    if trimmed.starts_with("//") && !trimmed.starts_with("///") {
                        doc_lines.insert(0, trimmed.to_string());
                    }
                } else if found_doc {
                    // End of doc block
                    break;
                }
            }
        }
        
        Ok(doc_lines.join("\n"))
    }

    /// Clean documentation line by removing comment prefixes
    fn clean_doc_line(&self, line: &str) -> String {
        if line.starts_with("///") {
            line.strip_prefix("///").unwrap_or("").trim().to_string()
        } else if line.starts_with("//!") {
            line.strip_prefix("//!").unwrap_or("").trim().to_string()
        } else if line.starts_with("//") {
            line.strip_prefix("//").unwrap_or("").trim().to_string()
        } else {
            line.to_string()
        }
    }

    /// Parse tag line (e.g., @param, @return, etc.)
    fn parse_tag_line(&self, line: &str) -> Option<TagInfo> {
        if let Some(line) = line.strip_prefix('@') {
            if let Some((tag_name, rest)) = line.split_once(' ') {
                Some(TagInfo {
                    name: tag_name.to_string(),
                    content: rest.trim().to_string(),
                })
            } else {
                Some(TagInfo {
                    name: line.to_string(),
                    content: String::new(),
                })
            }
        } else {
            None
        }
    }

    /// Process a parsed tag
    fn process_tag(&self, parsed: &mut ParsedDocumentation, current_example: &mut Option<ExampleBuilder>, tag: TagInfo) -> Result<(), Error> {
        match tag.name.as_str() {
            "param" | "parameter" => {
                parsed.tags.entry("parameters".to_string())
                    .or_insert_with(Vec::new)
                    .push(tag.content);
            }
            "return" | "returns" => {
                parsed.tags.entry("returns".to_string())
                    .or_insert_with(Vec::new)
                    .push(tag.content);
            }
            "throws" | "throw" => {
                parsed.tags.entry("throws".to_string())
                    .or_insert_with(Vec::new)
                    .push(tag.content);
            }
            "example" => {
                if self.extract_examples {
                    let mut example = ExampleBuilder::new("cursed".to_string());
                    example.set_title(tag.content);
                    *current_example = Some(example);
                }
            }
            "see" | "see_also" => {
                parsed.see_also.push(tag.content);
            }
            "since" => {
                parsed.since = Some(tag.content);
            }
            "deprecated" => {
                parsed.deprecated = Some(tag.content);
            }
            "author" => {
                parsed.author = Some(tag.content);
            }
            "version" => {
                parsed.tags.entry("version".to_string())
                    .or_insert_with(Vec::new)
                    .push(tag.content);
            }
            "todo" | "fixme" => {
                parsed.tags.entry("todo".to_string())
                    .or_insert_with(Vec::new)
                    .push(tag.content);
            }
            "note" => {
                parsed.tags.entry("notes".to_string())
                    .or_insert_with(Vec::new)
                    .push(tag.content);
            }
            _ => {
                // Generic tag
                parsed.tags.entry(tag.name)
                    .or_insert_with(Vec::new)
                    .push(tag.content);
            }
        }
        Ok(())
    }

    /// Post-process parsed documentation
    fn post_process(&self, parsed: &mut ParsedDocumentation) -> Result<(), Error> {
        // Clean up summary and description
        parsed.summary = parsed.summary.trim().to_string();
        parsed.description = parsed.description.trim().to_string();
        
        // Remove HTML tags if not allowed
        if !self.allow_html {
            parsed.summary = self.strip_html(&parsed.summary);
            parsed.description = self.strip_html(&parsed.description);
        }
        
        // Validate and normalize examples
        for example in &mut parsed.examples {
            example.code = example.code.trim().to_string();
            if example.language.is_empty() {
                example.language = "cursed".to_string();
            }
        }
        
        // Validate links if enabled
        if self.validate_links {
            self.validate_documentation_links(parsed)?;
        }
        
        Ok(())
    }

    /// Strip HTML tags from text
    fn strip_html(&self, text: &str) -> String {
        // Simple HTML tag removal
        let mut result = String::new();
        let mut in_tag = false;
        
        for ch in text.chars() {
            match ch {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ => if !in_tag { result.push(ch); }
            }
        }
        
        result
    }

    /// Validate links in documentation
    fn validate_documentation_links(&self, _parsed: &ParsedDocumentation) -> Result<(), Error> {
        // TODO: Implement link validation
        // This would check for broken internal references, invalid URLs, etc.
        Ok(())
    }

    /// Extract all documentation from source file
    pub fn extract_all_documentation(&self, source: &str) -> Result<(), Error> {
        let lines = source.split("\n").collect::<Vec<_>>();
        let mut results = Vec::new();
        let mut current_doc_start: Option<usize> = None;
        let mut doc_lines = Vec::new();
        
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                if current_doc_start.is_none() {
                    current_doc_start = Some(line_num + 1);
                }
                doc_lines.push(trimmed.to_string());
            } else if !doc_lines.is_empty() {
                // End of doc block - parse it
                let doc_content = doc_lines.join("\n");
                if let Ok(parsed) = self.parse_doc_content(&doc_content) {
                    let location = SourceLocation {
                        line: current_doc_start.unwrap_or(line_num + 1) as u32,
                        column: 1,
                        file: None,
                    };
                    results.push((location, parsed));
                }
                
                // Reset for next doc block
                doc_lines.clear();
                current_doc_start = None;
            }
        }
        
        // Handle final doc block if any
        if !doc_lines.is_empty() {
            let doc_content = doc_lines.join("\n");
            if let Ok(parsed) = self.parse_doc_content(&doc_content) {
                let location = SourceLocation {
                    line: current_doc_start.unwrap_or(lines.len()) as u32,
                    column: 1,
                    file: None,
                };
                results.push((location, parsed));
            }
        }
        
        Ok(results)
    }
}

/// Internal parsing section tracker
#[derive(Debug, PartialEq)]
enum ParsingSection {
    Summary,
    Description,
    Tags,
}

/// Tag information
#[derive(Debug)]
struct TagInfo {
    name: String,
    content: String,
}

/// Example builder for constructing examples during parsing
#[derive(Debug)]
struct ExampleBuilder {
    title: Option<String>,
    description: Option<String>,
    language: String,
    code_lines: Vec<String>,
    output: Option<String>,
}

impl ExampleBuilder {
    fn new(language: String) -> Self {
        Self {
            title: None,
            description: None,
            language,
            code_lines: Vec::new(),
            output: None,
        }
    }

    fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    fn add_code_line(&mut self, line: &str) {
        self.code_lines.push(line.to_string());
    }

    fn set_output(&mut self, output: String) {
        self.output = Some(output);
    }

    fn build(self) -> Example {
        Example {
            title: self.title,
            description: self.description,
            code: self.code_lines.join("\n"),
            language: self.language,
            output: self.output,
        }
    }
}

impl Default for CommentParser {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
