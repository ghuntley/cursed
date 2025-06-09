//! Documentation AST nodes for the CURSED language
//!
//! This module defines AST structures for representing documentation comments
//! and associated metadata that can be extracted from source code and used
//! for documentation generation.

use crate::ast::expressions::identifiers::Identifier;
use crate::ast::{Expression, Node, Statement};
use std::any::Any;
use std::collections::HashMap;
use std::fmt;

/// Position information for documentation elements
#[derive(Debug, Clone, PartialEq)]
pub struct DocPosition {
    pub line: usize,
    pub column: usize,
    pub file: String,
}

impl DocPosition {
    pub fn new(line: usize, column: usize, file: String) -> Self {
        Self { line, column, file }
    }
}

impl fmt::Display for DocPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// Metadata for documentation elements
#[derive(Debug, Clone, PartialEq)]
pub struct DocMetadata {
    pub author: Option<String>,
    pub version: Option<String>,
    pub since: Option<String>,
    pub deprecated: Option<String>,
    pub stability: Option<String>,
    pub tags: Vec<String>,
    pub see_also: Vec<String>,
    pub custom_fields: HashMap<String, String>,
}

impl DocMetadata {
    pub fn new() -> Self {
        Self {
            author: None,
            version: None,
            since: None,
            deprecated: None,
            stability: None,
            tags: Vec::new(),
            see_also: Vec::new(),
            custom_fields: HashMap::new(),
        }
    }

    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_since(mut self, since: String) -> Self {
        self.since = Some(since);
        self
    }

    pub fn with_deprecated(mut self, deprecated: String) -> Self {
        self.deprecated = Some(deprecated);
        self
    }

    pub fn with_stability(mut self, stability: String) -> Self {
        self.stability = Some(stability);
        self
    }

    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn add_see_also(mut self, reference: String) -> Self {
        self.see_also.push(reference);
        self
    }

    pub fn add_custom_field(mut self, key: String, value: String) -> Self {
        self.custom_fields.insert(key, value);
        self
    }

    pub fn is_deprecated(&self) -> bool {
        self.deprecated.is_some()
    }
}

impl Default for DocMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DocMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if let Some(author) = &self.author {
            parts.push(format!("@author {}", author));
        }
        if let Some(version) = &self.version {
            parts.push(format!("@version {}", version));
        }
        if let Some(since) = &self.since {
            parts.push(format!("@since {}", since));
        }
        if let Some(deprecated) = &self.deprecated {
            parts.push(format!("@deprecated {}", deprecated));
        }
        if let Some(stability) = &self.stability {
            parts.push(format!("@stability {}", stability));
        }

        for tag in &self.tags {
            parts.push(format!("@tag {}", tag));
        }

        for reference in &self.see_also {
            parts.push(format!("@see {}", reference));
        }

        for (key, value) in &self.custom_fields {
            parts.push(format!("@{} {}", key, value));
        }

        write!(f, "{}", parts.join("\n"))
    }
}

/// Basic documentation comment node
#[derive(Debug, Clone)]
pub struct DocComment {
    pub content: String,
    pub position: DocPosition,
    pub associated_symbol: Option<String>,
    pub metadata: DocMetadata,
    pub is_multiline: bool,
}

impl DocComment {
    pub fn new(content: String, position: DocPosition) -> Self {
        Self {
            content,
            position,
            associated_symbol: None,
            metadata: DocMetadata::new(),
            is_multiline: false,
        }
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.associated_symbol = Some(symbol);
        self
    }

    pub fn with_metadata(mut self, metadata: DocMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn multiline(mut self) -> Self {
        self.is_multiline = true;
        self
    }

    pub fn get_summary(&self) -> String {
        // Return the first sentence or first line as summary
        let first_line = self.content.lines().next().unwrap_or("").trim();
        if let Some(pos) = first_line.find('.') {
            first_line[..=pos].to_string()
        } else {
            first_line.to_string()
        }
    }
}

impl Node for DocComment {
    fn token_literal(&self) -> String {
        if self.is_multiline { "/**" } else { "///" }.to_string()
    }

    fn string(&self) -> String {
        if self.is_multiline {
            format!("/**\n{}\n*/", self.content)
        } else {
            format!("/// {}", self.content)
        }
    }
}

impl fmt::Display for DocComment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

/// Parameter documentation
#[derive(Debug, Clone)]
pub struct DocParameter {
    pub name: String,
    pub type_name: Option<String>,
    pub description: String,
    pub is_optional: bool,
    pub default_value: Option<String>,
}

impl DocParameter {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            type_name: None,
            description,
            is_optional: false,
            default_value: None,
        }
    }

    pub fn with_type(mut self, type_name: String) -> Self {
        self.type_name = Some(type_name);
        self
    }

    pub fn optional(mut self) -> Self {
        self.is_optional = true;
        self
    }

    pub fn with_default(mut self, default_value: String) -> Self {
        self.default_value = Some(default_value);
        self
    }
}

impl fmt::Display for DocParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(type_name) = &self.type_name {
            write!(f, " {}", type_name)?;
        }
        if self.is_optional {
            write!(f, " (optional)")?;
        }
        if let Some(default_value) = &self.default_value {
            write!(f, " = {}", default_value)?;
        }
        write!(f, " - {}", self.description)
    }
}

/// Return value documentation
#[derive(Debug, Clone)]
pub struct DocReturn {
    pub type_name: Option<String>,
    pub description: String,
    pub examples: Vec<String>,
}

impl DocReturn {
    pub fn new(description: String) -> Self {
        Self {
            type_name: None,
            description,
            examples: Vec::new(),
        }
    }

    pub fn with_type(mut self, type_name: String) -> Self {
        self.type_name = Some(type_name);
        self
    }

    pub fn add_example(mut self, example: String) -> Self {
        self.examples.push(example);
        self
    }
}

impl fmt::Display for DocReturn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(type_name) = &self.type_name {
            write!(f, "{} - {}", type_name, self.description)
        } else {
            write!(f, "{}", self.description)
        }
    }
}

/// Code example in documentation
#[derive(Debug, Clone)]
pub struct DocExample {
    pub title: Option<String>,
    pub code: String,
    pub description: Option<String>,
    pub language: String,
    pub is_runnable: bool,
}

impl DocExample {
    pub fn new(code: String) -> Self {
        Self {
            title: None,
            code,
            description: None,
            language: "cursed".to_string(),
            is_runnable: true,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_language(mut self, language: String) -> Self {
        self.language = language;
        self
    }

    pub fn not_runnable(mut self) -> Self {
        self.is_runnable = false;
        self
    }
}

impl fmt::Display for DocExample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(title) = &self.title {
            writeln!(f, "# {}", title)?;
        }
        if let Some(description) = &self.description {
            writeln!(f, "{}", description)?;
        }
        writeln!(f, "```{}", self.language)?;
        writeln!(f, "{}", self.code)?;
        write!(f, "```")
    }
}

/// Module-level documentation
#[derive(Debug, Clone)]
pub struct DocModule {
    pub name: String,
    pub description: String,
    pub position: DocPosition,
    pub metadata: DocMetadata,
    pub examples: Vec<DocExample>,
    pub sections: HashMap<String, String>,
}

impl DocModule {
    pub fn new(name: String, description: String, position: DocPosition) -> Self {
        Self {
            name,
            description,
            position,
            metadata: DocMetadata::new(),
            examples: Vec::new(),
            sections: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, metadata: DocMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_example(mut self, example: DocExample) -> Self {
        self.examples.push(example);
        self
    }

    pub fn add_section(mut self, name: String, content: String) -> Self {
        self.sections.insert(name, content);
        self
    }
}

impl Node for DocModule {
    fn token_literal(&self) -> String {
        "module".to_string()
    }

    fn string(&self) -> String {
        format!("Module: {}\n{}", self.name, self.description)
    }
}

impl fmt::Display for DocModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# Module: {}", self.name)?;
        writeln!(f, "{}", self.description)?;

        if !self.metadata.tags.is_empty() {
            writeln!(f, "\nTags: {}", self.metadata.tags.join(", "))?;
        }

        for (section_name, content) in &self.sections {
            writeln!(f, "\n## {}", section_name)?;
            writeln!(f, "{}", content)?;
        }

        for example in &self.examples {
            writeln!(f, "\n{}", example)?;
        }

        Ok(())
    }
}

/// Function documentation
#[derive(Debug, Clone)]
pub struct DocFunction {
    pub name: String,
    pub description: String,
    pub position: DocPosition,
    pub metadata: DocMetadata,
    pub parameters: Vec<DocParameter>,
    pub returns: Option<DocReturn>,
    pub examples: Vec<DocExample>,
    pub throws: Vec<String>,
    pub complexity: Option<String>,
}

impl DocFunction {
    pub fn new(name: String, description: String, position: DocPosition) -> Self {
        Self {
            name,
            description,
            position,
            metadata: DocMetadata::new(),
            parameters: Vec::new(),
            returns: None,
            examples: Vec::new(),
            throws: Vec::new(),
            complexity: None,
        }
    }

    pub fn with_metadata(mut self, metadata: DocMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_parameter(mut self, parameter: DocParameter) -> Self {
        self.parameters.push(parameter);
        self
    }

    pub fn with_returns(mut self, returns: DocReturn) -> Self {
        self.returns = Some(returns);
        self
    }

    pub fn add_example(mut self, example: DocExample) -> Self {
        self.examples.push(example);
        self
    }

    pub fn add_throws(mut self, error: String) -> Self {
        self.throws.push(error);
        self
    }

    pub fn with_complexity(mut self, complexity: String) -> Self {
        self.complexity = Some(complexity);
        self
    }
}

impl Node for DocFunction {
    fn token_literal(&self) -> String {
        "function".to_string()
    }

    fn string(&self) -> String {
        format!("Function: {}\n{}", self.name, self.description)
    }
}

impl fmt::Display for DocFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "## Function: {}", self.name)?;
        writeln!(f, "{}", self.description)?;

        if !self.parameters.is_empty() {
            writeln!(f, "\n### Parameters")?;
            for param in &self.parameters {
                writeln!(f, "- {}", param)?;
            }
        }

        if let Some(returns) = &self.returns {
            writeln!(f, "\n### Returns")?;
            writeln!(f, "{}", returns)?;
        }

        if !self.throws.is_empty() {
            writeln!(f, "\n### Throws")?;
            for error in &self.throws {
                writeln!(f, "- {}", error)?;
            }
        }

        if let Some(complexity) = &self.complexity {
            writeln!(f, "\n### Complexity")?;
            writeln!(f, "{}", complexity)?;
        }

        for example in &self.examples {
            writeln!(f, "\n### Example")?;
            writeln!(f, "{}", example)?;
        }

        Ok(())
    }
}

/// Type documentation (structs, interfaces, etc.)
#[derive(Debug, Clone)]
pub struct DocType {
    pub name: String,
    pub kind: String, // "struct", "interface", "enum", etc.
    pub description: String,
    pub position: DocPosition,
    pub metadata: DocMetadata,
    pub fields: Vec<DocField>,
    pub methods: Vec<DocMethod>,
    pub examples: Vec<DocExample>,
    pub generic_parameters: Vec<String>,
}

impl DocType {
    pub fn new(name: String, kind: String, description: String, position: DocPosition) -> Self {
        Self {
            name,
            kind,
            description,
            position,
            metadata: DocMetadata::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            examples: Vec::new(),
            generic_parameters: Vec::new(),
        }
    }

    pub fn with_metadata(mut self, metadata: DocMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_field(mut self, field: DocField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn add_method(mut self, method: DocMethod) -> Self {
        self.methods.push(method);
        self
    }

    pub fn add_example(mut self, example: DocExample) -> Self {
        self.examples.push(example);
        self
    }

    pub fn add_generic_parameter(mut self, param: String) -> Self {
        self.generic_parameters.push(param);
        self
    }
}

impl Node for DocType {
    fn token_literal(&self) -> String {
        self.kind.clone()
    }

    fn string(&self) -> String {
        format!("{}: {}\n{}", self.kind, self.name, self.description)
    }
}

impl fmt::Display for DocType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "## {} {}", self.kind, self.name)?;
        if !self.generic_parameters.is_empty() {
            write!(f, "<{}>", self.generic_parameters.join(", "))?;
        }
        writeln!(f)?;
        writeln!(f, "{}", self.description)?;

        if !self.fields.is_empty() {
            writeln!(f, "\n### Fields")?;
            for field in &self.fields {
                writeln!(f, "- {}", field)?;
            }
        }

        if !self.methods.is_empty() {
            writeln!(f, "\n### Methods")?;
            for method in &self.methods {
                writeln!(f, "{}", method)?;
            }
        }

        for example in &self.examples {
            writeln!(f, "\n### Example")?;
            writeln!(f, "{}", example)?;
        }

        Ok(())
    }
}

/// Field documentation for structs/interfaces
#[derive(Debug, Clone)]
pub struct DocField {
    pub name: String,
    pub type_name: Option<String>,
    pub description: String,
    pub is_public: bool,
    pub is_optional: bool,
    pub default_value: Option<String>,
}

impl DocField {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            type_name: None,
            description,
            is_public: true,
            is_optional: false,
            default_value: None,
        }
    }

    pub fn with_type(mut self, type_name: String) -> Self {
        self.type_name = Some(type_name);
        self
    }

    pub fn private(mut self) -> Self {
        self.is_public = false;
        self
    }

    pub fn optional(mut self) -> Self {
        self.is_optional = true;
        self
    }

    pub fn with_default(mut self, default_value: String) -> Self {
        self.default_value = Some(default_value);
        self
    }
}

impl fmt::Display for DocField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(type_name) = &self.type_name {
            write!(f, ": {}", type_name)?;
        }
        if self.is_optional {
            write!(f, " (optional)")?;
        }
        if let Some(default_value) = &self.default_value {
            write!(f, " = {}", default_value)?;
        }
        write!(f, " - {}", self.description)
    }
}

/// Method documentation
#[derive(Debug, Clone)]
pub struct DocMethod {
    pub name: String,
    pub description: String,
    pub position: DocPosition,
    pub metadata: DocMetadata,
    pub parameters: Vec<DocParameter>,
    pub returns: Option<DocReturn>,
    pub examples: Vec<DocExample>,
    pub throws: Vec<String>,
    pub is_static: bool,
    pub visibility: String, // "public", "private", "protected"
}

impl DocMethod {
    pub fn new(name: String, description: String, position: DocPosition) -> Self {
        Self {
            name,
            description,
            position,
            metadata: DocMetadata::new(),
            parameters: Vec::new(),
            returns: None,
            examples: Vec::new(),
            throws: Vec::new(),
            is_static: false,
            visibility: "public".to_string(),
        }
    }

    pub fn with_metadata(mut self, metadata: DocMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn add_parameter(mut self, parameter: DocParameter) -> Self {
        self.parameters.push(parameter);
        self
    }

    pub fn with_returns(mut self, returns: DocReturn) -> Self {
        self.returns = Some(returns);
        self
    }

    pub fn add_example(mut self, example: DocExample) -> Self {
        self.examples.push(example);
        self
    }

    pub fn add_throws(mut self, error: String) -> Self {
        self.throws.push(error);
        self
    }

    pub fn static_method(mut self) -> Self {
        self.is_static = true;
        self
    }

    pub fn with_visibility(mut self, visibility: String) -> Self {
        self.visibility = visibility;
        self
    }
}

impl Node for DocMethod {
    fn token_literal(&self) -> String {
        "method".to_string()
    }

    fn string(&self) -> String {
        format!("Method: {}\n{}", self.name, self.description)
    }
}

impl fmt::Display for DocMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#### ")?;
        if self.is_static {
            write!(f, "static ")?;
        }
        writeln!(f, "{} {}", self.visibility, self.name)?;
        writeln!(f, "{}", self.description)?;

        if !self.parameters.is_empty() {
            writeln!(f, "\n##### Parameters")?;
            for param in &self.parameters {
                writeln!(f, "- {}", param)?;
            }
        }

        if let Some(returns) = &self.returns {
            writeln!(f, "\n##### Returns")?;
            writeln!(f, "{}", returns)?;
        }

        if !self.throws.is_empty() {
            writeln!(f, "\n##### Throws")?;
            for error in &self.throws {
                writeln!(f, "- {}", error)?;
            }
        }

        for example in &self.examples {
            writeln!(f, "\n##### Example")?;
            writeln!(f, "{}", example)?;
        }

        Ok(())
    }
}
