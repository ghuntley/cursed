/// Template rendering integration utilities
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::SystemTime;
use crate::config::TemplateConfig;

/// Template engine types
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateEngineType {
    Handlebars,
    Mustache,
    Jinja2,
    Simple,
}

/// Template engine for rendering HTML templates
pub struct TemplateEngine {
    config: TemplateConfig,
    templates: HashMap<String, CachedTemplate>,
    template_cache: HashMap<String, (String, SystemTime)>,
    engine_type: TemplateEngineType,
    inheritance_chain: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub content: String,
    pub last_modified: std::time::SystemTime,
    pub extends: Option<String>,
    pub includes: Vec<String>,
    pub variables: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CachedTemplate {
    pub template: Template,
    pub compiled_content: String,
    pub cached_at: SystemTime,
    pub dependencies: Vec<String>,
}

/// Template context for variable substitution
#[derive(Debug, Clone)]
pub struct TemplateContext {
    pub variables: HashMap<String, TemplateValue>,
    pub globals: HashMap<String, TemplateValue>,
}

#[derive(Debug, Clone)]
pub enum TemplateValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<TemplateValue>),
    Object(HashMap<String, TemplateValue>),
    Null,
}

/// Template renderer
pub struct TemplateRenderer {
    engine: TemplateEngine,
}

impl TemplateEngine {
    pub fn new(config: TemplateConfig) -> Self {
        Self::with_engine_type(config, TemplateEngineType::Simple)
    }

    pub fn with_engine_type(config: TemplateConfig, engine_type: TemplateEngineType) -> Self {
        Self {
            config,
            templates: HashMap::new(),
            template_cache: HashMap::new(),
            engine_type,
            inheritance_chain: HashMap::new(),
        }
    }

    /// Load template from filesystem
    pub fn load_template(&mut self, name: &str) -> Result<(), TemplateError> {
        let template_path = self.resolve_template_path(name)?;
        
        // Check if template exists
        if !template_path.exists() {
            return Err(TemplateError::TemplateNotFound(format!("Template file not found: {}", template_path.display())));
        }

        // Check cache if enabled
        if self.config.cache_templates {
            if let Some((cached_content, cached_time)) = self.template_cache.get(name) {
                let file_time = fs::metadata(&template_path)
                    .map_err(|e| TemplateError::LoadError(format!("Failed to read file metadata: {}", e)))?
                    .modified()
                    .map_err(|e| TemplateError::LoadError(format!("Failed to get modification time: {}", e)))?;
                
                // Use cached version if file hasn't changed
                if file_time <= *cached_time {
                    return Ok(());
                }
            }
        }

        // Read template content
        let content = fs::read_to_string(&template_path)
            .map_err(|e| TemplateError::LoadError(format!("Failed to read template file: {}", e)))?;

        let metadata = fs::metadata(&template_path)
            .map_err(|e| TemplateError::LoadError(format!("Failed to read file metadata: {}", e)))?;
        
        let last_modified = metadata.modified()
            .map_err(|e| TemplateError::LoadError(format!("Failed to get modification time: {}", e)))?;

        // Parse template for inheritance and includes
        let (extends, includes, variables) = self.parse_template_metadata(&content)?;

        let template = Template {
            name: name.to_string(),
            content: content.clone(),
            last_modified,
            extends,
            includes: includes.clone(),
            variables,
        };

        // Build inheritance chain
        if let Some(parent) = &template.extends {
            self.build_inheritance_chain(name, parent)?;
        }

        // Load includes recursively
        for include_name in &includes {
            if !self.templates.contains_key(include_name) {
                self.load_template(include_name)?;
            }
        }

        // Compile template content
        let compiled_content = self.compile_template(&template)?;

        let cached_template = CachedTemplate {
            template,
            compiled_content,
            cached_at: SystemTime::now(),
            dependencies: includes,
        };

        self.templates.insert(name.to_string(), cached_template);
        
        // Update cache
        if self.config.cache_templates {
            self.template_cache.insert(name.to_string(), (content, SystemTime::now()));
        }

        Ok(())
    }

    /// Render template with context
    pub fn render(&self, name: &str, context: &TemplateContext) -> Result<String, TemplateError> {
        let cached_template = self.templates.get(name)
            .ok_or_else(|| TemplateError::TemplateNotFound(name.to_string()))?;

        // Handle template inheritance
        let mut content = if let Some(extends) = &cached_template.template.extends {
            self.render_with_inheritance(name, extends, context)?
        } else {
            cached_template.compiled_content.clone()
        };

        // Process template includes
        content = self.process_includes(&content, context)?;

        // Render variables based on engine type
        content = match self.engine_type {
            TemplateEngineType::Handlebars => self.render_handlebars(&content, context)?,
            TemplateEngineType::Mustache => self.render_mustache(&content, context)?,
            TemplateEngineType::Jinja2 => self.render_jinja2(&content, context)?,
            TemplateEngineType::Simple => self.render_simple(&content, context)?,
        };

        Ok(content)
    }

    /// Render template with legacy string context (for backward compatibility)
    pub fn render_legacy(&self, name: &str, context: &HashMap<String, String>) -> Result<String, TemplateError> {
        let template_context = TemplateContext::from_string_map(context);
        self.render(name, &template_context)
    }

    /// Resolve template file path
    fn resolve_template_path(&self, name: &str) -> Result<PathBuf, TemplateError> {
        let mut path = self.config.template_dir.join(name);
        
        // Add extension if not present
        if path.extension().is_none() {
            path.set_extension(&self.config.template_extension);
        }

        Ok(path)
    }

    /// Parse template metadata (extends, includes, variables)
    fn parse_template_metadata(&self, content: &str) -> Result<(Option<String>, Vec<String>, Vec<String>), TemplateError> {
        let mut extends = None;
        let mut includes = Vec::new();
        let mut variables = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            
            // Parse extends directive
            if line.starts_with("{% extends") {
                if let Some(start) = line.find('"') {
                    if let Some(end) = line.rfind('"') {
                        if start != end {
                            extends = Some(line[start + 1..end].to_string());
                        }
                    }
                }
            }
            
            // Parse include directive
            if line.starts_with("{% include") {
                if let Some(start) = line.find('"') {
                    if let Some(end) = line.rfind('"') {
                        if start != end {
                            includes.push(line[start + 1..end].to_string());
                        }
                    }
                }
            }

            // Extract variables (simple regex-like extraction)
            let mut chars = line.chars().peekable();
            while let Some(ch) = chars.next() {
                if ch == '{' && chars.peek() == Some(&'{') {
                    chars.next(); // consume second '{'
                    let mut var_name = String::new();
                    let mut depth = 2;
                    
                    while let Some(ch) = chars.next() {
                        if ch == '{' {
                            depth += 1;
                        } else if ch == '}' {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        } else if depth == 2 && ch.is_alphanumeric() || ch == '_' {
                            var_name.push(ch);
                        } else if depth == 2 && !var_name.is_empty() && (ch.is_whitespace() || ch == '|' || ch == '}') {
                            if !variables.contains(&var_name) {
                                variables.push(var_name.clone());
                            }
                            var_name.clear();
                        }
                    }
                    
                    if !var_name.is_empty() && !variables.contains(&var_name) {
                        variables.push(var_name);
                    }
                }
            }
        }

        Ok((extends, includes, variables))
    }

    /// Build inheritance chain for template
    fn build_inheritance_chain(&mut self, child: &str, parent: &str) -> Result<(), TemplateError> {
        let mut chain = vec![child.to_string()];
        let mut current = parent;

        // Prevent infinite loops
        let mut visited = std::collections::HashSet::new();
        visited.insert(child.to_string());

        loop {
            if visited.contains(current) {
                return Err(TemplateError::RenderError("Circular template inheritance detected".to_string()));
            }
            
            visited.insert(current.to_string());
            chain.push(current.to_string());

            // Load parent template if not loaded
            if !self.templates.contains_key(current) {
                self.load_template(current)?;
            }

            if let Some(cached_template) = self.templates.get(current) {
                if let Some(grandparent) = &cached_template.template.extends {
                    current = grandparent;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        self.inheritance_chain.insert(child.to_string(), chain);
        Ok(())
    }

    /// Compile template content
    fn compile_template(&self, template: &Template) -> Result<String, TemplateError> {
        // For now, return content as-is
        // In a full implementation, this would compile to an optimized format
        Ok(template.content.clone())
    }

    /// Render template with inheritance
    fn render_with_inheritance(&self, child: &str, parent: &str, context: &TemplateContext) -> Result<String, TemplateError> {
        let parent_template = self.templates.get(parent)
            .ok_or_else(|| TemplateError::TemplateNotFound(parent.to_string()))?;

        let mut content = parent_template.compiled_content.clone();
        let child_template = self.templates.get(child)
            .ok_or_else(|| TemplateError::TemplateNotFound(child.to_string()))?;

        // Simple block replacement (in real implementation, would be more sophisticated)
        content = content.replace("{% block content %}", &child_template.template.content);

        Ok(content)
    }

    /// Process template includes
    fn process_includes(&self, content: &str, context: &TemplateContext) -> Result<String, TemplateError> {
        let mut result = content.to_string();
        
        // Find and replace include directives
        while let Some(start) = result.find("{% include") {
            if let Some(end) = result[start..].find("%}") {
                let directive = &result[start..start + end + 2];
                
                if let Some(quote_start) = directive.find('"') {
                    if let Some(quote_end) = directive.rfind('"') {
                        if quote_start != quote_end {
                            let include_name = &directive[quote_start + 1..quote_end];
                            
                            if let Some(included_template) = self.templates.get(include_name) {
                                let included_content = self.render_simple(&included_template.compiled_content, context)?;
                                result = result.replace(directive, &included_content);
                            } else {
                                return Err(TemplateError::TemplateNotFound(include_name.to_string()));
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }

        Ok(result)
    }

    /// Simple template rendering
    fn render_simple(&self, content: &str, context: &TemplateContext) -> Result<String, TemplateError> {
        let mut rendered = content.to_string();
        
        // Replace variables in context
        for (key, value) in &context.variables {
            let placeholder = format!("{{{{{}}}}}", key);
            let value_str = value.to_string();
            rendered = rendered.replace(&placeholder, &value_str);
        }

        // Replace global variables
        for (key, value) in &context.globals {
            let placeholder = format!("{{{{{}}}}}", key);
            let value_str = value.to_string();
            rendered = rendered.replace(&placeholder, &value_str);
        }

        Ok(rendered)
    }

    /// Handlebars-style template rendering
    fn render_handlebars(&self, content: &str, context: &TemplateContext) -> Result<String, TemplateError> {
        // Basic handlebars-style rendering
        self.render_simple(content, context)
    }

    /// Mustache-style template rendering
    fn render_mustache(&self, content: &str, context: &TemplateContext) -> Result<String, TemplateError> {
        // Basic mustache-style rendering
        self.render_simple(content, context)
    }

    /// Jinja2-style template rendering
    fn render_jinja2(&self, content: &str, context: &TemplateContext) -> Result<String, TemplateError> {
        // Basic jinja2-style rendering with different delimiters
        let mut rendered = content.to_string();
        
        // Replace variables with jinja2 style
        for (key, value) in &context.variables {
            let placeholder = format!("{{{{{}}}}}", key);
            let value_str = value.to_string();
            rendered = rendered.replace(&placeholder, &value_str);
        }

        Ok(rendered)
    }

    /// Get template info
    pub fn get_template_info(&self, name: &str) -> Option<&Template> {
        self.templates.get(name).map(|cached| &cached.template)
    }

    /// Clear template cache
    pub fn clear_cache(&mut self) {
        self.template_cache.clear();
        self.templates.clear();
        self.inheritance_chain.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.templates.len(), self.template_cache.len())
    }
}

impl TemplateRenderer {
    pub fn new(engine: TemplateEngine) -> Self {
        Self { engine }
    }

    /// Render template to string with rich context
    pub fn render_to_string(&self, template: &str, context: &TemplateContext) -> Result<String, TemplateError> {
        self.engine.render(template, context)
    }

    /// Render template to string with string context (backward compatibility)
    pub fn render_to_string_legacy(&self, template: &str, context: &HashMap<String, String>) -> Result<String, TemplateError> {
        self.engine.render_legacy(template, context)
    }

    /// Load template into the engine
    pub fn load_template(&mut self, name: &str) -> Result<(), TemplateError> {
        self.engine.load_template(name)
    }

    /// Get template information
    pub fn get_template_info(&self, name: &str) -> Option<&Template> {
        self.engine.get_template_info(name)
    }

    /// Clear template cache
    pub fn clear_cache(&mut self) {
        self.engine.clear_cache()
    }
}

#[derive(Debug)]
pub enum TemplateError {
    TemplateNotFound(String),
    RenderError(String),
    LoadError(String),
    ParseError(String),
    InheritanceError(String),
    CacheError(String),
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TemplateError::TemplateNotFound(name) => write!(f, "Template not found: {}", name),
            TemplateError::RenderError(msg) => write!(f, "Render error: {}", msg),
            TemplateError::LoadError(msg) => write!(f, "Load error: {}", msg),
            TemplateError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            TemplateError::InheritanceError(msg) => write!(f, "Inheritance error: {}", msg),
            TemplateError::CacheError(msg) => write!(f, "Cache error: {}", msg),
        }
    }
}

impl std::error::Error for TemplateError {}

// Implementation for TemplateContext
impl TemplateContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            globals: HashMap::new(),
        }
    }

    pub fn from_string_map(map: &HashMap<String, String>) -> Self {
        let mut context = Self::new();
        for (key, value) in map {
            context.variables.insert(key.clone(), TemplateValue::String(value.clone()));
        }
        context
    }

    pub fn set_variable<K: Into<String>>(&mut self, key: K, value: TemplateValue) {
        self.variables.insert(key.into(), value);
    }

    pub fn set_global<K: Into<String>>(&mut self, key: K, value: TemplateValue) {
        self.globals.insert(key.into(), value);
    }

    pub fn set_string<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.variables.insert(key.into(), TemplateValue::String(value.into()));
    }

    pub fn set_number<K: Into<String>>(&mut self, key: K, value: f64) {
        self.variables.insert(key.into(), TemplateValue::Number(value));
    }

    pub fn set_boolean<K: Into<String>>(&mut self, key: K, value: bool) {
        self.variables.insert(key.into(), TemplateValue::Boolean(value));
    }

    pub fn get_variable(&self, key: &str) -> Option<&TemplateValue> {
        self.variables.get(key).or_else(|| self.globals.get(key))
    }
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

// Implementation for TemplateValue
impl TemplateValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            TemplateValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            TemplateValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            TemplateValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            TemplateValue::Boolean(b) => *b,
            TemplateValue::String(s) => !s.is_empty(),
            TemplateValue::Number(n) => *n != 0.0,
            TemplateValue::Array(arr) => !arr.is_empty(),
            TemplateValue::Object(obj) => !obj.is_empty(),
            TemplateValue::Null => false,
        }
    }
}

impl std::fmt::Display for TemplateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TemplateValue::String(s) => write!(f, "{}", s),
            TemplateValue::Number(n) => write!(f, "{}", n),
            TemplateValue::Boolean(b) => write!(f, "{}", b),
            TemplateValue::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            TemplateValue::Object(obj) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in obj {
                    if !first { write!(f, ", ")?; }
                    write!(f, "{}: {}", key, value)?;
                    first = false;
                }
                write!(f, "}}")
            },
            TemplateValue::Null => write!(f, "null"),
        }
    }
}

impl From<String> for TemplateValue {
    fn from(s: String) -> Self {
        TemplateValue::String(s)
    }
}

impl From<&str> for TemplateValue {
    fn from(s: &str) -> Self {
        TemplateValue::String(s.to_string())
    }
}

impl From<f64> for TemplateValue {
    fn from(n: f64) -> Self {
        TemplateValue::Number(n)
    }
}

impl From<i32> for TemplateValue {
    fn from(n: i32) -> Self {
        TemplateValue::Number(n as f64)
    }
}

impl From<bool> for TemplateValue {
    fn from(b: bool) -> Self {
        TemplateValue::Boolean(b)
    }
}

impl From<Vec<TemplateValue>> for TemplateValue {
    fn from(arr: Vec<TemplateValue>) -> Self {
        TemplateValue::Array(arr)
    }
}

impl From<HashMap<String, TemplateValue>> for TemplateValue {
    fn from(obj: HashMap<String, TemplateValue>) -> Self {
        TemplateValue::Object(obj)
    }
}
