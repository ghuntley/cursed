/// Core Template Engine - The heart of CURSED templating
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tracing::{debug, error, info, instrument, warn};

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;
use super::template_syntax::{TemplateAst, TemplateLexer, TemplateParser};
use super::template_cache::TemplateCache;
use super::template_filters::FilterRegistry;

/// Main template engine that coordinates all templating operations
#[derive(Clone)]
pub struct TemplateEngine {
    /// Template cache for performance optimization
    cache: Arc<TemplateCache>,
    /// Registry of available filters and functions
    filters: Arc<FilterRegistry>,
    /// Template loader for finding and loading templates
    loader: Arc<dyn TemplateLoader>,
    /// Global template configuration
    config: TemplateConfig,
    /// Context variables available to all templates
    global_context: Arc<RwLock<HashMap<String, CursedObject>>>,
}

impl std::fmt::Debug for TemplateEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateEngine")
            .field("config", &self.config)
            .field("cache", &self.cache)
            .finish()
    }
}

/// Configuration for the template engine
#[derive(Debug, Clone)]
pub struct TemplateConfig {
    /// Auto-escape HTML by default
    pub auto_escape: bool,
    /// Enable template caching
    pub enable_cache: bool,
    /// Cache size limit (in number of templates)
    pub cache_size: usize,
    /// Template file extensions to recognize
    pub template_extensions: Vec<String>,
    /// Enable strict mode (fail on undefined variables)
    pub strict_mode: bool,
    /// Maximum template nesting depth
    pub max_nesting_depth: usize,
    /// Template syntax delimiters
    pub delimiters: TemplateDelimiters,
}

/// Template syntax delimiters using Gen Z slang
#[derive(Debug, Clone)]
pub struct TemplateDelimiters {
    pub variable: (String, String),      // {{ }}
    pub block: (String, String),         // {% %}
    pub comment: (String, String),       // {# #}
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            auto_escape: true,
            enable_cache: true,
            cache_size: 1000,
            template_extensions: Vec::from(["html".to_string(), "txt".to_string(), "md".to_string()]),
            strict_mode: false,
            max_nesting_depth: 20,
            delimiters: TemplateDelimiters {
                variable: ("{{".to_string(), "}}".to_string()),
                block: ("{%".to_string(), "%}".to_string()),
                comment: ("{#".to_string(), "#}".to_string()),
            },
        }
    }
}

/// Trait for loading templates from various sources
pub trait TemplateLoader: Send + Sync {
    /// Load a template by name/path
    fn load(&self, name: &str) -> Result<String, CursedError>;
    
    /// Check if a template exists
    fn exists(&self, name: &str) -> bool;
    
    /// Get the last modified time of a template
    fn last_modified(&self, name: &str) -> Option<std::time::SystemTime>;
}

/// File system template loader
#[derive(Debug)]
pub struct FileSystemLoader {
    /// Base directory for templates
    base_dir: PathBuf,
    /// Allowed template extensions
    extensions: Vec<String>,
}

impl FileSystemLoader {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
            extensions: Vec::from(["html".to_string(), "txt".to_string(), "md".to_string()]),
        }
    }
    
    pub fn with_extensions<P: AsRef<Path>>(base_dir: P, extensions: Vec<String>) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
            extensions,
        }
    }
}

impl TemplateLoader for FileSystemLoader {
    #[instrument(skip(self))]
    fn load(&self, name: &str) -> Result<String, CursedError> {
        let template_path = self.base_dir.join(name);
        
        // Security check: ensure template is within base directory
        if !template_path.starts_with(&self.base_dir) {
            return Err(CursedError::TemplateError {
                message: format!("Template path '{}' is outside base directory", name),
                source_location: None,
            });
        }
        
        std::fs::read_to_string(&template_path)
            .map_err(|e| CursedError::TemplateError {
                message: format!("Failed to load template '{}': {}", name, e),
                source_location: None,
            })
    }
    
    fn exists(&self, name: &str) -> bool {
        let template_path = self.base_dir.join(name);
        template_path.exists() && template_path.is_file()
    }
    
    fn last_modified(&self, name: &str) -> Option<std::time::SystemTime> {
        let template_path = self.base_dir.join(name);
        std::fs::metadata(&template_path)
            .and_then(|meta| meta.modified())
            .ok()
    }
}

/// Template rendering context
#[derive(Debug, Clone)]
pub struct TemplateContext {
    /// Variables available in the template
    variables: HashMap<String, CursedObject>,
    /// Parent context for variable lookup
    parent: Option<Box<TemplateContext>>,
}

impl TemplateContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }
    
    /// Create a child context with a parent
    pub fn with_parent(parent: TemplateContext) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }
    
    /// Set a variable in this context
    pub fn set<K: Into<String>>(&mut self, key: K, value: CursedObject) {
        self.variables.insert(key.into(), value);
    }
    
    /// Get a variable from this context or parent contexts
    pub fn get(&self, key: &str) -> Option<&CursedObject> {
        self.variables.get(key)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(key)))
    }
    
    /// Check if a variable exists in this context or parent contexts
    pub fn contains(&self, key: &str) -> bool {
        self.variables.contains_key(key) || 
            self.parent.as_ref().map_or(false, |p| p.contains(key))
    }
    
    /// Merge another context into this one
    pub fn merge(&mut self, other: TemplateContext) {
        self.variables.extend(other.variables);
    }
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateEngine {
    /// Create a new template engine with default configuration
    pub fn new() -> Self {
        let config = TemplateConfig::default();
        let loader = Arc::new(FileSystemLoader::new("templates"));
        Self::with_config_and_loader(config, loader)
    }
    
    /// Create a template engine with custom configuration and loader
    pub fn with_config_and_loader(
        config: TemplateConfig, 
        loader: Arc<dyn TemplateLoader>
    ) -> Self {
        Self {
            cache: Arc::new(TemplateCache::new(config.cache_size)),
            filters: Arc::new(FilterRegistry::new()),
            loader,
            config,
            global_context: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Set a global context variable available to all templates
    #[instrument(skip(self, key, value))]
    pub fn set_global<K: Into<String> + std::fmt::Debug>(&self, key: K, value: CursedObject) -> Result<(), CursedError> {
        let mut context = self.global_context.write()
            .map_err(|_| CursedError::TemplateError {
                message: "Failed to acquire global context lock".to_string(),
                source_location: None,
            })?;
        
        context.insert(key.into(), value);
        Ok(())
    }
    
    /// Register a custom filter
    pub fn register_filter<F>(&self, name: &str, filter: F) -> Result<(), CursedError>
    where
        F: Fn(&[CursedObject]) -> Result<CursedObject, CursedError> + Send + Sync + 'static,
    {
        Arc::get_mut(&mut self.filters.clone())
            .ok_or_else(|| CursedError::TemplateError {
                message: "Cannot register filter: filter registry is shared".to_string(),
                source_location: None,
            })?
            .register(name, Box::new(filter));
        Ok(())
    }
    
    /// Render a template by name with the given context
    #[instrument(skip(self, context))]
    pub fn render(&self, template_name: &str, context: TemplateContext) -> Result<String, CursedError> {
        info!(template = template_name, "Starting template render");
        
        // Load and parse template
        let template_source = self.loader.load(template_name)?;
        let template_ast = self.parse_template(&template_source)?;
        
        // Create rendering context with global variables
        let mut render_context = context;
        if let Ok(global_context) = self.global_context.read() {
            for (key, value) in global_context.iter() {
                if !render_context.contains(key) {
                    render_context.set(key.clone(), value.clone());
                }
            }
        }
        
        // Render the template
        let renderer = super::template_render::TemplateRenderer::new(
            Arc::clone(&self.filters),
            Arc::clone(&self.loader),
            &self.config,
        );
        
        let result = renderer.render(&template_ast, render_context)?;
        
        info!(template = template_name, output_length = result.len(), "Template render completed");
        Ok(result)
    }
    
    /// Render a template from a string with the given context
    #[instrument(skip(self, template_source, context))]
    pub fn render_string(&self, template_source: &str, context: TemplateContext) -> Result<String, CursedError> {
        debug!(source_length = template_source.len(), "Rendering template from string");
        
        let template_ast = self.parse_template(template_source)?;
        
        let renderer = super::template_render::TemplateRenderer::new(
            Arc::clone(&self.filters),
            Arc::clone(&self.loader),
            &self.config,
        );
        
        renderer.render(&template_ast, context)
    }
    
    /// Parse a template source into an AST
    #[instrument(skip(self, source))]
    pub fn parse_template(&self, source: &str) -> Result<TemplateAst, CursedError> {
        debug!(source_length = source.len(), "Parsing template");
        
        let mut lexer = TemplateLexer::new(source, &self.config.delimiters);
        let tokens = lexer.tokenize()?;
        
        let mut parser = TemplateParser::new(tokens);
        let ast = parser.parse()?;
        
        debug!(nodes = ast.nodes.len(), "Template parsing completed");
        Ok(ast)
    }
    
    /// Check if a template exists
    pub fn template_exists(&self, name: &str) -> bool {
        self.loader.exists(name)
    }
    
    /// Clear the template cache
    pub fn clear_cache(&self) {
        self.cache.clear();
    }
    
    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        self.cache.stats()
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::CursedObject;

    #[test]
    fn test_template_context_operations() {
        let mut context = TemplateContext::new();
        
        // Test setting and getting variables
        context.set("name", CursedObject::String("Alice".to_string()));
        context.set("age", CursedObject::Integer(25));
        
        assert_eq!(context.get("name"), Some(&CursedObject::String("Alice".to_string())));
        assert_eq!(context.get("age"), Some(&CursedObject::Integer(25)));
        assert_eq!(context.get("nonexistent"), None);
        
        // Test contains
        assert!(context.contains("name"));
        assert!(context.contains("age"));
        assert!(!context.contains("nonexistent"));
    }
    
    #[test]
    fn test_template_context_inheritance() {
        let mut parent = TemplateContext::new();
        parent.set("global_var", CursedObject::String("global".to_string()));
        parent.set("override_me", CursedObject::String("parent".to_string()));
        
        let mut child = TemplateContext::with_parent(parent);
        child.set("local_var", CursedObject::String("local".to_string()));
        child.set("override_me", CursedObject::String("child".to_string()));
        
        // Test variable lookup
        assert_eq!(child.get("global_var"), Some(&CursedObject::String("global".to_string())));
        assert_eq!(child.get("local_var"), Some(&CursedObject::String("local".to_string())));
        assert_eq!(child.get("override_me"), Some(&CursedObject::String("child".to_string())));
    }
    
    #[test]
    fn test_template_config_defaults() {
        let config = TemplateConfig::default();
        
        assert!(config.auto_escape);
        assert!(config.enable_cache);
        assert_eq!(config.cache_size, 1000);
        assert!(!config.strict_mode);
        assert_eq!(config.max_nesting_depth, 20);
        assert_eq!(config.delimiters.variable, ("{{".to_string(), "}}".to_string()));
        assert_eq!(config.delimiters.block, ("{%".to_string(), "%}".to_string()));
        assert_eq!(config.delimiters.comment, ("{#".to_string(), "#}".to_string()));
    }
    
    #[test]
    fn test_filesystem_loader_creation() {
        let loader = FileSystemLoader::new("templates");
        assert_eq!(loader.base_dir, PathBuf::from("templates"));
        assert_eq!(loader.extensions, Vec::from(["html", "txt", "md"]));
        
        let loader_with_exts = FileSystemLoader::with_extensions(
            "custom_templates", 
            Vec::from(["csd".to_string(), "tmpl".to_string()])
        );
        assert_eq!(loader_with_exts.base_dir, PathBuf::from("custom_templates"));
        assert_eq!(loader_with_exts.extensions, Vec::from(["csd", "tmpl"]));
    }
}
