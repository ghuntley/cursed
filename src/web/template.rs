/// Template system for CURSED web applications
/// Provides template rendering with variable substitution and control flow

use crate::error::CursedError;
use std::collections::HashMap;
use std::fmt;

/// Template error types  
#[derive(Debug, Clone)]
pub enum TemplateError {
    /// Template not found
    /// Template parsing error
    /// Template rendering error
    /// Variable not found
    /// Invalid template syntax
    /// I/O error
// impl fmt::Display for TemplateError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             TemplateError::NotFound(name) => write!(f, "Template not found: {}", name),
//             TemplateError::Parse(msg) => write!(f, "Template parse error: {}", msg),
//             TemplateError::Render(msg) => write!(f, "Template render error: {}", msg),
//             TemplateError::VariableNotFound(var) => write!(f, "Variable not found: {}", var),
//             TemplateError::InvalidSyntax(msg) => write!(f, "Invalid template syntax: {}", msg),
//             TemplateError::Io(msg) => write!(f, "Template I/O error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for TemplateError {}
// 
// impl From<std::io::Error> for TemplateError {
//     fn from(err: std::io::Error) -> Self {
//         TemplateError::Io(err.to_string())
//     }
// }

/// Template context for variable substitution
#[derive(Debug, Clone)]
pub struct TemplateContext {
impl TemplateContext {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn set<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.variables.insert(key.into(), value.into());
    pub fn get(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    pub fn from_map(variables: HashMap<String, String>) -> Self {
        Self { variables }
    }

    pub fn merge(&mut self, other: &TemplateContext) {
        for (key, value) in &other.variables {
            self.variables.insert(key.clone(), value.clone());
        }
    }
impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Compiled template
#[derive(Debug, Clone)]
pub struct Template {
/// Template tokens for compilation
#[derive(Debug, Clone)]
enum TemplateToken {
    /// Raw text content
    /// Variable substitution {{ variable }}
    /// If condition {% if condition %}
    /// Else clause {% else %}
    /// End if {% endif %}
    /// For loop {% for item in items %}
    /// End for {% endfor %}
impl Template {
    pub fn new(name: String, content: String) -> Result<Self, TemplateError> {
        let compiled = Self::compile(&content)?;
        Ok(Self {
        })
    pub fn name(&self) -> &str {
        &self.name
    pub fn content(&self) -> &str {
        &self.content
    fn compile(content: &str) -> Result<Vec<TemplateToken>, TemplateError> {
        let mut tokens = Vec::new();
        let mut chars = content.chars().peekable();
        let mut current_text = String::new();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                if chars.peek() == Some(&'{') {
                    // Variable {{ ... }}
                    chars.next(); // consume second {
                    if !current_text.is_empty() {
                        tokens.push(TemplateToken::Text(current_text.clone()));
                        current_text.clear();
                    let mut var_name = String::new();
                    let mut found_end = false;
                    
                    while let Some(ch) = chars.next() {
                        if ch == '}' && chars.peek() == Some(&'}') {
                            chars.next(); // consume second }
                            found_end = true;
                            break;
                        }
                        var_name.push(ch);
                    if !found_end {
                        return Err(TemplateError::InvalidSyntax("Unclosed variable".to_string()));
                    tokens.push(TemplateToken::Variable(var_name.trim().to_string()));
                } else if chars.peek() == Some(&'%') {
                    // Control structure {% ... %}
                    chars.next(); // consume %
                    if !current_text.is_empty() {
                        tokens.push(TemplateToken::Text(current_text.clone()));
                        current_text.clear();
                    let mut control = String::new();
                    let mut found_end = false;
                    
                    while let Some(ch) = chars.next() {
                        if ch == '%' && chars.peek() == Some(&'}') {
                            chars.next(); // consume }
                            found_end = true;
                            break;
                        }
                        control.push(ch);
                    if !found_end {
                        return Err(TemplateError::InvalidSyntax("Unclosed control structure".to_string()));
                    let control = control.trim();
                    if control.starts_with("if ") {
                        tokens.push(TemplateToken::If(control[3..].trim().to_string()));
                    } else if control == "else" {
                        tokens.push(TemplateToken::Else);
                    } else if control == "endif" {
                        tokens.push(TemplateToken::EndIf);
                    } else if control.starts_with("for ") {
                        let parts: Vec<&str> = control[4..].split(" in ").collect();
                        if parts.len() == 2 {
                            tokens.push(TemplateToken::For {
                            });
                        } else {
                            return Err(TemplateError::InvalidSyntax(format!("Invalid for loop: {}", control)));
                        }
                    } else if control == "endfor" {
                        tokens.push(TemplateToken::EndFor);
                    } else {
                        return Err(TemplateError::InvalidSyntax(format!("Unknown control structure: {}", control)));
                    }
                } else {
                    current_text.push(ch);
                }
            } else {
                current_text.push(ch);
            }
        }

        if !current_text.is_empty() {
            tokens.push(TemplateToken::Text(current_text));
        Ok(tokens)
    pub fn render(&self, context: &TemplateContext) -> Result<String, TemplateError> {
        let mut output = String::new();
        let mut token_index = 0;

        while token_index < self.compiled.len() {
            match &self.compiled[token_index] {
                TemplateToken::Text(text) => {
                    output.push_str(text);
                }
                TemplateToken::Variable(var_name) => {
                    if let Some(value) = context.get(var_name) {
                        output.push_str(value);
                    } else {
                        return Err(TemplateError::VariableNotFound(var_name.clone()));
                    }
                }
                TemplateToken::If(condition) => {
                    // Simple boolean evaluation - just check if variable exists and is "true"
                    let should_render = context.get(condition).map_or(false, |v| v == "true");
                    if !should_render {
                        // Skip to else or endif
                        let mut depth = 1;
                        token_index += 1;
                        while token_index < self.compiled.len() && depth > 0 {
                            match &self.compiled[token_index] {
                                _ => {}
                            }
                            token_index += 1;
                        }
                        token_index -= 1; // Adjust for the increment at the end of the loop
                    }
                }
                TemplateToken::Else => {
                    // Skip to endif
                    let mut depth = 1;
                    token_index += 1;
                    while token_index < self.compiled.len() && depth > 0 {
                        match &self.compiled[token_index] {
                            _ => {}
                        }
                        token_index += 1;
                    }
                    token_index -= 1; // Adjust for the increment at the end of the loop
                }
                TemplateToken::EndIf => {
                    // Nothing to do
                }
                TemplateToken::For { var: _, collection: _ } => {
                    // Simplified for loop - just skip for now
                    let mut depth = 1;
                    token_index += 1;
                    while token_index < self.compiled.len() && depth > 0 {
                        match &self.compiled[token_index] {
                            _ => {}
                        }
                        token_index += 1;
                    }
                    token_index -= 1; // Adjust for the increment at the end of the loop
                }
                TemplateToken::EndFor => {
                    // Nothing to do
                }
            }
            token_index += 1;
        Ok(output)
    }
}

/// Template loader for loading templates from files
#[derive(Debug)]
pub struct TemplateLoader {
impl TemplateLoader {
    pub fn new(template_dir: String) -> Self {
        Self { template_dir }
    }

    pub fn load(&self, name: &str) -> Result<Template, TemplateError> {
        let path = format!("{}/{}", self.template_dir, name);
        let content = std::fs::read_to_string(&path)
            .map_err(|_| TemplateError::NotFound(name.to_string()))?;
        Template::new(name.to_string(), content)
    }
}

/// Template cache for performance
#[derive(Debug)]
pub struct TemplateCache {
impl TemplateCache {
    pub fn new(loader: TemplateLoader) -> Self {
        Self {
        }
    }

    pub fn get(&mut self, name: &str) -> Result<&Template, TemplateError> {
        if !self.templates.contains_key(name) {
            let template = self.loader.load(name)?;
            self.templates.insert(name.to_string(), template);
        Ok(self.templates.get(name).unwrap())
    pub fn clear(&mut self) {
        self.templates.clear();
    }
}

/// Template renderer combining engine and context
#[derive(Debug)]
pub struct TemplateRenderer {
impl TemplateRenderer {
    pub fn new(template_dir: String) -> Self {
        let loader = TemplateLoader::new(template_dir);
        let cache = TemplateCache::new(loader);
        Self { cache }
    }

    pub fn render(&mut self, template_name: &str, context: &TemplateContext) -> Result<String, TemplateError> {
        let template = self.cache.get(template_name)?;
        template.render(context)
    }
}

/// Template engine for managing templates
#[derive(Debug)]
pub struct TemplateEngine {
impl TemplateEngine {
    pub fn new(template_dir: String) -> Self {
        Self {
        }
    }

    pub fn set_global<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.global_context.set(key, value);
    pub fn render(&mut self, template_name: &str, mut context: TemplateContext) -> Result<String, TemplateError> {
        context.merge(&self.global_context);
        self.renderer.render(template_name, &context)
    pub fn render_string(&self, template_content: &str, context: &TemplateContext) -> Result<String, TemplateError> {
        let template = Template::new("inline".to_string(), template_content.to_string())?;
        let mut merged_context = context.clone();
        merged_context.merge(&self.global_context);
        template.render(&merged_context)
    }
}

