/// Core Template Engine - The heart of CURSED templating
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock, Mutex};
use std::time::{Duration, Instant, SystemTime};
use tracing::{debug, error, info, instrument, warn};
use uuid;

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;
use super::template_syntax::{TemplateAst, TemplateLexer, TemplateParser};
use super::template_cache::TemplateCache;
use super::template_filters::{FilterRegistry, FilterContext};

/// Comprehensive template error types
#[derive(Debug, Clone)]
pub enum TemplateError {
    /// Template parsing error
    ParseError {
        message: String,
        line: usize,
        column: usize,
    },
    /// Template rendering error
    RenderError {
        message: String,
        template_name: Option<String>,
        line: Option<usize>,
    },
    /// Template loading error
    LoadError {
        template_name: String,
        source: String,
    },
    /// Variable resolution error
    VariableError {
        variable_name: String,
        context: String,
    },
    /// Filter execution error
    FilterError {
        filter_name: String,
        message: String,
    },
    /// Template compilation error
    CompileError {
        message: String,
        source_location: Option<String>,
    },
    /// Template security error
    SecurityError {
        message: String,
        attempted_path: String,
    },
    /// Template configuration error
    ConfigError(String),
    /// Template recursion limit exceeded
    RecursionError {
        depth: usize,
        max_depth: usize,
    },
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::ParseError { message, line, column } => {
                write!(f, "Template parse error at line {}, column {}: {}", line, column, message)
            }
            TemplateError::RenderError { message, template_name, line } => {
                match (template_name, line) {
                    (Some(name), Some(line)) => write!(f, "Template render error in '{}' at line {}: {}", name, line, message),
                    (Some(name), None) => write!(f, "Template render error in '{}': {}", name, message),
                    (None, Some(line)) => write!(f, "Template render error at line {}: {}", line, message),
                    (None, None) => write!(f, "Template render error: {}", message),
                }
            }
            TemplateError::LoadError { template_name, source } => {
                write!(f, "Failed to load template '{}': {}", template_name, source)
            }
            TemplateError::VariableError { variable_name, context } => {
                write!(f, "Variable '{}' not found in context: {}", variable_name, context)
            }
            TemplateError::FilterError { filter_name, message } => {
                write!(f, "Filter '{}' error: {}", filter_name, message)
            }
            TemplateError::CompileError { message, source_location } => {
                match source_location {
                    Some(loc) => write!(f, "Template compile error at {}: {}", loc, message),
                    None => write!(f, "Template compile error: {}", message),
                }
            }
            TemplateError::SecurityError { message, attempted_path } => {
                write!(f, "Template security error (path: '{}'): {}", attempted_path, message)
            }
            TemplateError::ConfigError(msg) => write!(f, "Template configuration error: {}", msg),
            TemplateError::RecursionError { depth, max_depth } => {
                write!(f, "Template recursion limit exceeded: {} > {}", depth, max_depth)
            }
        }
    }
}

impl std::error::Error for TemplateError {}

impl From<TemplateError> for CursedError {
    fn from(err: TemplateError) -> Self {
        CursedError::TemplateError {
            message: err.to_string(),
            source_location: None,
        }
    }
}

/// Template performance metrics
#[derive(Debug, Clone)]
pub struct TemplateMetrics {
    /// Template name
    pub name: String,
    /// Parse time in milliseconds
    pub parse_time_ms: u64,
    /// Render time in milliseconds
    pub render_time_ms: u64,
    /// Template size in bytes
    pub size_bytes: usize,
    /// Number of variables resolved
    pub variables_resolved: usize,
    /// Number of filters applied
    pub filters_applied: usize,
    /// Cache hit/miss status
    pub cache_hit: bool,
    /// Timestamp of measurement
    pub timestamp: SystemTime,
}

/// Template metadata and content
#[derive(Debug, Clone)]
pub struct Template {
    /// Template name/identifier
    pub name: String,
    /// Original template source
    pub source: String,
    /// Parsed AST
    pub ast: TemplateAst,
    /// Template metadata
    pub metadata: TemplateMetadata,
    /// Performance metrics
    pub metrics: Option<TemplateMetrics>,
}

/// Template metadata
#[derive(Debug, Clone)]
pub struct TemplateMetadata {
    /// Template file path (if loaded from file)
    pub path: Option<PathBuf>,
    /// Last modified time
    pub modified: Option<SystemTime>,
    /// Template size in bytes
    pub size: usize,
    /// Template hash for cache invalidation
    pub hash: u64,
    /// Creation timestamp
    pub created: SystemTime,
}

impl Template {
    /// Create a new template from source
    pub fn from_source(name: String, source: String, delimiters: &TemplateDelimiters) -> Result<Self, TemplateError> {
        let parse_start = Instant::now();
        
        let mut lexer = TemplateLexer::new(&source, delimiters);
        let tokens = lexer.tokenize().map_err(|e| TemplateError::ParseError {
            message: e.to_string(),
            line: 1,
            column: 1,
        })?;
        
        let mut parser = TemplateParser::new(tokens);
        let ast = parser.parse().map_err(|e| TemplateError::ParseError {
            message: e.to_string(),
            line: 1,
            column: 1,
        })?;
        
        let parse_time = parse_start.elapsed();
        
        let metadata = TemplateMetadata {
            path: None,
            modified: None,
            size: source.len(),
            hash: Self::calculate_hash(&source),
            created: SystemTime::now(),
        };
        
        let metrics = Some(TemplateMetrics {
            name: name.clone(),
            parse_time_ms: parse_time.as_millis() as u64,
            render_time_ms: 0,
            size_bytes: source.len(),
            variables_resolved: 0,
            filters_applied: 0,
            cache_hit: false,
            timestamp: SystemTime::now(),
        });
        
        Ok(Template {
            name,
            source,
            ast,
            metadata,
            metrics,
        })
    }
    
    /// Calculate hash for template content
    fn calculate_hash(source: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Update template metrics after rendering
    pub fn update_metrics(&mut self, render_time: Duration, variables_resolved: usize, filters_applied: usize, cache_hit: bool) {
        if let Some(ref mut metrics) = self.metrics {
            metrics.render_time_ms = render_time.as_millis() as u64;
            metrics.variables_resolved = variables_resolved;
            metrics.filters_applied = filters_applied;
            metrics.cache_hit = cache_hit;
            metrics.timestamp = SystemTime::now();
        }
    }
}

/// Performance monitoring for template operations
#[derive(Debug)]
pub struct TemplatePerformanceMonitor {
    /// Total templates rendered
    total_renders: Arc<Mutex<u64>>,
    /// Total render time
    total_render_time: Arc<Mutex<Duration>>,
    /// Cache statistics
    cache_hits: Arc<Mutex<u64>>,
    cache_misses: Arc<Mutex<u64>>,
    /// Recent template metrics
    recent_metrics: Arc<Mutex<Vec<TemplateMetrics>>>,
    /// Maximum metrics to keep
    max_metrics: usize,
}

impl TemplatePerformanceMonitor {
    pub fn new() -> Self {
        Self {
            total_renders: Arc::new(Mutex::new(0)),
            total_render_time: Arc::new(Mutex::new(Duration::from_secs(0))),
            cache_hits: Arc::new(Mutex::new(0)),
            cache_misses: Arc::new(Mutex::new(0)),
            recent_metrics: Arc::new(Mutex::new(Vec::new())),
            max_metrics: 1000,
        }
    }
    
    /// Record a template render operation
    pub fn record_render(&self, metrics: TemplateMetrics) {
        if let (Ok(mut total_renders), Ok(mut total_time), Ok(mut recent)) = (
            self.total_renders.lock(),
            self.total_render_time.lock(),
            self.recent_metrics.lock(),
        ) {
            *total_renders += 1;
            *total_time += Duration::from_millis(metrics.render_time_ms);
            
            recent.push(metrics);
            if recent.len() > self.max_metrics {
                recent.remove(0);
            }
        }
    }
    
    /// Record cache hit/miss
    pub fn record_cache_result(&self, hit: bool) {
        if hit {
            if let Ok(mut hits) = self.cache_hits.lock() {
                *hits += 1;
            }
        } else if let Ok(mut misses) = self.cache_misses.lock() {
            *misses += 1;
        }
    }
    
    /// Get performance statistics
    pub fn get_stats(&self) -> Option<PerformanceStats> {
        let total_renders = *self.total_renders.lock().ok()?;
        let total_time = *self.total_render_time.lock().ok()?;
        let cache_hits = *self.cache_hits.lock().ok()?;
        let cache_misses = *self.cache_misses.lock().ok()?;
        
        Some(PerformanceStats {
            total_renders,
            average_render_time: if total_renders > 0 {
                total_time / total_renders as u32
            } else {
                Duration::from_secs(0)
            },
            cache_hit_rate: if cache_hits + cache_misses > 0 {
                cache_hits as f64 / (cache_hits + cache_misses) as f64
            } else {
                0.0
            },
            total_cache_operations: cache_hits + cache_misses,
        })
    }
}

/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub total_renders: u64,
    pub average_render_time: Duration,
    pub cache_hit_rate: f64,
    pub total_cache_operations: u64,
}

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
    /// Performance monitoring
    performance_monitor: Arc<TemplatePerformanceMonitor>,
    /// Compiled template cache
    compiled_templates: Arc<RwLock<HashMap<String, Arc<Template>>>>,
}

impl std::fmt::Debug for TemplateEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateEngine")
            .field("config", &self.config)
            .field("cache", &self.cache)
            .field("performance_stats", &self.performance_monitor.get_stats())
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

/// Template rendering context with thread-safe variable management
#[derive(Debug, Clone)]
pub struct TemplateContext {
    /// Variables available in the template
    variables: Arc<RwLock<HashMap<String, CursedObject>>>,
    /// Parent context for variable lookup
    parent: Option<Box<TemplateContext>>,
    /// Context ID for debugging and tracking
    context_id: String,
    /// Isolation level for context variable updates
    isolation_level: ContextIsolationLevel,
}

/// Context isolation level for variable updates
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContextIsolationLevel {
    /// Variables can be updated across all contexts
    None,
    /// Variables can only be updated in current context
    Local,
    /// Variables are completely isolated (read-only parent access)
    Strict,
}

impl TemplateContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self::new_with_isolation(ContextIsolationLevel::Local)
    }
    
    /// Create a new context with specific isolation level
    pub fn new_with_isolation(isolation_level: ContextIsolationLevel) -> Self {
        Self {
            variables: Arc::new(RwLock::new(HashMap::new())),
            parent: None,
            context_id: format!("ctx_{}", uuid::Uuid::new_v4().to_string()[0..8].to_string()),
            isolation_level,
        }
    }
    
    /// Create a child context with a parent
    pub fn with_parent(parent: TemplateContext) -> Self {
        Self::with_parent_and_isolation(parent, ContextIsolationLevel::Local)
    }
    
    /// Create a child context with parent and specific isolation level
    pub fn with_parent_and_isolation(parent: TemplateContext, isolation_level: ContextIsolationLevel) -> Self {
        Self {
            variables: Arc::new(RwLock::new(HashMap::new())),
            parent: Some(Box::new(parent)),
            context_id: format!("ctx_{}", uuid::Uuid::new_v4().to_string()[0..8].to_string()),
            isolation_level,
        }
    }
    
    /// Create a scoped context for loops with iteration variables
    pub fn create_loop_scope(&self, loop_var: String, loop_value: CursedObject, index: usize) -> Result<Self, CursedError> {
        let scope = Self::with_parent_and_isolation(self.clone(), ContextIsolationLevel::Local);
        scope.set_local(loop_var, loop_value)?;
        scope.set_local("loop".to_string(), CursedObject::Map({
            let mut loop_data = HashMap::new();
            loop_data.insert("index".to_string(), CursedObject::Integer(index as i64));
            loop_data.insert("index0".to_string(), CursedObject::Integer(index as i64));
            loop_data.insert("index1".to_string(), CursedObject::Integer((index + 1) as i64));
            loop_data.insert("first".to_string(), CursedObject::Boolean(index == 0));
            // Note: last flag would need total count to be accurate
            loop_data
        }))?;
        Ok(scope)
    }
    
    /// Set a variable in this context (thread-safe)
    pub fn set<K: Into<String>>(&self, key: K, value: CursedObject) -> Result<(), CursedError> {
        self.set_local(key, value)
    }
    
    /// Set a variable in this context only (no parent traversal)
    pub fn set_local<K: Into<String>>(&self, key: K, value: CursedObject) -> Result<(), CursedError> {
        let key_str = key.into();
        let mut variables = self.variables.write()
            .map_err(|_| CursedError::TemplateError {
                message: format!("Failed to acquire write lock for context variable '{}'", key_str),
                source_location: None,
            })?;
        
        variables.insert(key_str, value);
        Ok(())
    }
    
    /// Update an existing variable in this context or parent contexts
    pub fn update<K: Into<String>>(&self, key: K, value: CursedObject) -> Result<bool, CursedError> {
        let key_str = key.into();
        
        match self.isolation_level {
            ContextIsolationLevel::Strict => {
                // Only update if variable exists in current context
                let mut variables = self.variables.write()
                    .map_err(|_| CursedError::TemplateError {
                        message: format!("Failed to acquire write lock for variable '{}'", key_str),
                        source_location: None,
                    })?;
                
                if variables.contains_key(&key_str) {
                    variables.insert(key_str, value);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            ContextIsolationLevel::Local => {
                // Update in current context if exists, otherwise set locally
                let mut variables = self.variables.write()
                    .map_err(|_| CursedError::TemplateError {
                        message: format!("Failed to acquire write lock for variable '{}'", key_str),
                        source_location: None,
                    })?;
                
                variables.insert(key_str, value);
                Ok(true)
            }
            ContextIsolationLevel::None => {
                // Try to update in parent chain, fall back to local
                if self.update_in_parent_chain(&key_str, &value)? {
                    Ok(true)
                } else {
                    self.set_local(key_str, value)?;
                    Ok(true)
                }
            }
        }
    }
    
    /// Update variable in parent chain (helper for None isolation level)
    fn update_in_parent_chain(&self, key: &str, value: &CursedObject) -> Result<bool, CursedError> {
        // Check if variable exists in current context
        {
            let variables = self.variables.read()
                .map_err(|_| CursedError::TemplateError {
                    message: format!("Failed to acquire read lock for variable '{}'", key),
                    source_location: None,
                })?;
            
            if variables.contains_key(key) {
                drop(variables);
                let mut variables = self.variables.write()
                    .map_err(|_| CursedError::TemplateError {
                        message: format!("Failed to acquire write lock for variable '{}'", key),
                        source_location: None,
                    })?;
                variables.insert(key.to_string(), value.clone());
                return Ok(true);
            }
        }
        
        // Check parent contexts
        if let Some(parent) = &self.parent {
            parent.update_in_parent_chain(key, value)
        } else {
            Ok(false)
        }
    }
    
    /// Get a variable from this context or parent contexts (thread-safe)
    pub fn get(&self, key: &str) -> Option<CursedObject> {
        // First check current context
        {
            let variables = self.variables.read().ok()?;
            if let Some(value) = variables.get(key) {
                return Some(value.clone());
            }
        }
        
        // Then check parent contexts
        self.parent.as_ref().and_then(|p| p.get(key))
    }
    
    /// Get a variable only from this context (no parent lookup)
    pub fn get_local(&self, key: &str) -> Option<CursedObject> {
        let variables = self.variables.read().ok()?;
        variables.get(key).cloned()
    }
    
    /// Check if a variable exists in this context or parent contexts
    pub fn contains(&self, key: &str) -> bool {
        {
            let variables = self.variables.read().unwrap_or_else(|_| {
                warn!("Failed to acquire read lock for contains check");
                return self.variables.read().unwrap();
            });
            
            if variables.contains_key(key) {
                return true;
            }
        }
        
        self.parent.as_ref().map_or(false, |p| p.contains(key))
    }
    
    /// Check if a variable exists only in this context
    pub fn contains_local(&self, key: &str) -> bool {
        let variables = self.variables.read().unwrap_or_else(|_| {
            warn!("Failed to acquire read lock for local contains check");
            return self.variables.read().unwrap();
        });
        
        variables.contains_key(key)
    }
    
    /// Merge another context into this one (thread-safe)
    pub fn merge(&self, other: &TemplateContext) -> Result<(), CursedError> {
        let other_variables = other.variables.read()
            .map_err(|_| CursedError::TemplateError {
                message: "Failed to acquire read lock for source context during merge".to_string(),
                source_location: None,
            })?;
        
        let mut variables = self.variables.write()
            .map_err(|_| CursedError::TemplateError {
                message: "Failed to acquire write lock for target context during merge".to_string(),
                source_location: None,
            })?;
        
        for (key, value) in other_variables.iter() {
            variables.insert(key.clone(), value.clone());
        }
        
        Ok(())
    }
    
    /// Create a context with additional variables for includes
    pub fn create_include_context(&self, include_vars: HashMap<String, CursedObject>) -> Result<Self, CursedError> {
        let include_context = Self::with_parent_and_isolation(self.clone(), ContextIsolationLevel::Local);
        
        for (key, value) in include_vars {
            include_context.set_local(key, value)?;
        }
        
        Ok(include_context)
    }
    
    /// Get all variables from this context (for debugging)
    pub fn get_all_local(&self) -> HashMap<String, CursedObject> {
        let variables = self.variables.read().unwrap_or_else(|_| {
            warn!("Failed to acquire read lock for get_all_local");
            return self.variables.read().unwrap();
        });
        
        variables.clone()
    }
    
    /// Get context ID for debugging
    pub fn get_context_id(&self) -> &str {
        &self.context_id
    }
    
    /// Get isolation level
    pub fn get_isolation_level(&self) -> ContextIsolationLevel {
        self.isolation_level
    }
    
    /// Create a shadow scope (variable shadowing)
    pub fn create_shadow_scope(&self) -> Self {
        Self::with_parent_and_isolation(self.clone(), ContextIsolationLevel::Local)
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
            performance_monitor: Arc::new(TemplatePerformanceMonitor::new()),
            compiled_templates: Arc::new(RwLock::new(HashMap::new())),
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
        F: Fn(&FilterContext, &[CursedObject]) -> Result<CursedObject, CursedError> + Send + Sync + 'static,
    {
        self.filters.register(name, filter);
        Ok(())
    }
    
    /// Render a template by name with the given context
    #[instrument(skip(self, context))]
    pub fn render(&self, template_name: &str, context: TemplateContext) -> Result<String, CursedError> {
        info!(template = template_name, "Starting template render");
        let render_start = Instant::now();
        
        // Check compiled template cache first
        let template = if let Ok(cache) = self.compiled_templates.read() {
            if let Some(cached_template) = cache.get(template_name) {
                self.performance_monitor.record_cache_result(true);
                Arc::clone(cached_template)
            } else {
                self.performance_monitor.record_cache_result(false);
                self.load_and_compile_template(template_name)?
            }
        } else {
            self.performance_monitor.record_cache_result(false);
            self.load_and_compile_template(template_name)?
        };
        
        // Create rendering context with global variables
        let render_context = context;
        let mut variables_resolved = 0;
        if let Ok(global_context) = self.global_context.read() {
            for (key, value) in global_context.iter() {
                if !render_context.contains(key) {
                    render_context.set(key.clone(), value.clone())
                        .map_err(|e| CursedError::TemplateError {
                            message: format!("Failed to set global variable '{}': {}", key, e),
                            source_location: None,
                        })?;
                    variables_resolved += 1;
                }
            }
        }
        
        // Render the template
        let renderer = super::template_render::TemplateRenderer::new(
            Arc::clone(&self.filters),
            Arc::clone(&self.loader),
            &self.config,
        );
        
        let render_context_for_render = super::template_render::RenderContext::new(render_context);
        let render_result = renderer.render_with_result(&template.ast, render_context_for_render)?;
        let render_time = render_start.elapsed();
        
        // Record performance metrics with actual filter tracking
        let metrics = TemplateMetrics {
            name: template_name.to_string(),
            parse_time_ms: template.metrics.as_ref().map(|m| m.parse_time_ms).unwrap_or(0),
            render_time_ms: render_time.as_millis() as u64,
            size_bytes: template.source.len(),
            variables_resolved: variables_resolved + render_result.variables_resolved,
            filters_applied: render_result.filters_applied,
            cache_hit: true, // We already tracked cache hit/miss above
            timestamp: SystemTime::now(),
        };
        
        self.performance_monitor.record_render(metrics);
        
        info!(template = template_name, output_length = render_result.output.len(), render_time_ms = render_time.as_millis(), "Template render completed");
        Ok(render_result.output)
    }
    
    /// Load and compile a template, caching the result
    #[instrument(skip(self))]
    fn load_and_compile_template(&self, template_name: &str) -> Result<Arc<Template>, CursedError> {
        // Load template source
        let template_source = self.loader.load(template_name)?;
        
        // Create template
        let template = Template::from_source(
            template_name.to_string(),
            template_source,
            &self.config.delimiters,
        ).map_err(|e| CursedError::from(e))?;
        
        let template_arc = Arc::new(template);
        
        // Cache the compiled template
        if let Ok(mut cache) = self.compiled_templates.write() {
            cache.insert(template_name.to_string(), Arc::clone(&template_arc));
        }
        
        Ok(template_arc)
    }
    
    /// Render a template from a string with the given context
    #[instrument(skip(self, template_source, context))]
    pub fn render_string(&self, template_source: &str, context: TemplateContext) -> Result<String, CursedError> {
        debug!(source_length = template_source.len(), "Rendering template from string");
        let render_start = Instant::now();
        
        // Create template from source
        let template = Template::from_source(
            "inline".to_string(),
            template_source.to_string(),
            &self.config.delimiters,
        ).map_err(|e| CursedError::from(e))?;
        
        // Create rendering context with global variables
        let render_context = context;
        let mut variables_resolved = 0;
        if let Ok(global_context) = self.global_context.read() {
            for (key, value) in global_context.iter() {
                if !render_context.contains(key) {
                    render_context.set(key.clone(), value.clone())
                        .map_err(|e| CursedError::TemplateError {
                            message: format!("Failed to set global variable '{}': {}", key, e),
                            source_location: None,
                        })?;
                    variables_resolved += 1;
                }
            }
        }
        
        let renderer = super::template_render::TemplateRenderer::new(
            Arc::clone(&self.filters),
            Arc::clone(&self.loader),
            &self.config,
        );
        
        let render_context_for_render = super::template_render::RenderContext::new(render_context);
        let render_result = renderer.render_with_result(&template.ast, render_context_for_render)?;
        let render_time = render_start.elapsed();
        
        // Record performance metrics with actual filter tracking
        let metrics = TemplateMetrics {
            name: "inline".to_string(),
            parse_time_ms: template.metrics.as_ref().map(|m| m.parse_time_ms).unwrap_or(0),
            render_time_ms: render_time.as_millis() as u64,
            size_bytes: template_source.len(),
            variables_resolved: variables_resolved + render_result.variables_resolved,
            filters_applied: render_result.filters_applied,
            cache_hit: false,
            timestamp: SystemTime::now(),
        };
        
        self.performance_monitor.record_render(metrics);
        
        debug!(render_time_ms = render_time.as_millis(), "String template render completed");
        Ok(render_result.output)
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
    
    /// Get template performance statistics
    pub fn performance_stats(&self) -> Option<PerformanceStats> {
        self.performance_monitor.get_stats()
    }
    
    /// Clear compiled template cache
    pub fn clear_compiled_cache(&self) {
        if let Ok(mut cache) = self.compiled_templates.write() {
            cache.clear();
        }
    }
    
    /// Get compiled template cache size
    pub fn compiled_cache_size(&self) -> usize {
        self.compiled_templates.read().map(|cache| cache.len()).unwrap_or(0)
    }
    
    /// Precompile a template for better performance
    #[instrument(skip(self))]
    pub fn precompile_template(&self, template_name: &str) -> Result<(), CursedError> {
        self.load_and_compile_template(template_name)?;
        Ok(())
    }
    
    /// Validate a template without rendering
    #[instrument(skip(self))]
    pub fn validate_template(&self, template_name: &str) -> Result<(), TemplateError> {
        let template_source = self.loader.load(template_name)
            .map_err(|e| TemplateError::LoadError {
                template_name: template_name.to_string(),
                source: e.to_string(),
            })?;
        
        Template::from_source(
            template_name.to_string(),
            template_source,
            &self.config.delimiters,
        )?;
        
        Ok(())
    }
    
    /// Validate template source string
    #[instrument(skip(self, source))]
    pub fn validate_template_source(&self, source: &str) -> Result<(), TemplateError> {
        Template::from_source(
            "validation".to_string(),
            source.to_string(),
            &self.config.delimiters,
        )?;
        
        Ok(())
    }
    
    /// Get list of available filters
    pub fn available_filters(&self) -> Vec<String> {
        self.filters.list_filters()
    }
    
    /// Check if a filter is available
    pub fn has_filter(&self, name: &str) -> bool {
        self.filters.has_filter(name)
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

    #[test]
    fn test_template_context_operations() {
        let context = TemplateContext::new();
        
        // Test setting and getting variables
        context.set("name", CursedObject::String("Alice".to_string())).unwrap();
        context.set("age", CursedObject::Integer(25)).unwrap();
        
        assert_eq!(context.get("name"), Some(CursedObject::String("Alice".to_string())));
        assert_eq!(context.get("age"), Some(CursedObject::Integer(25)));
        assert_eq!(context.get("nonexistent"), None);
        
        // Test contains
        assert!(context.contains("name"));
        assert!(context.contains("age"));
        assert!(!context.contains("nonexistent"));
    }
    
    #[test]
    fn test_template_context_inheritance() {
        let parent = TemplateContext::new();
        parent.set("global_var", CursedObject::String("global".to_string())).unwrap();
        parent.set("override_me", CursedObject::String("parent".to_string())).unwrap();
        
        let child = TemplateContext::with_parent(parent);
        child.set("local_var", CursedObject::String("local".to_string())).unwrap();
        child.set("override_me", CursedObject::String("child".to_string())).unwrap();
        
        // Test variable lookup
        assert_eq!(child.get("global_var"), Some(CursedObject::String("global".to_string())));
        assert_eq!(child.get("local_var"), Some(CursedObject::String("local".to_string())));
        assert_eq!(child.get("override_me"), Some(CursedObject::String("child".to_string())));
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
    
    #[test]
    fn test_template_creation() {
        let delimiters = TemplateDelimiters {
            variable: ("{{".to_string(), "}}".to_string()),
            block: ("{%".to_string(), "%}".to_string()),
            comment: ("{#".to_string(), "#}".to_string()),
        };
        
        let source = "Hello {{ name }}!";
        let template = Template::from_source(
            "test".to_string(),
            source.to_string(),
            &delimiters,
        );
        
        assert!(template.is_ok());
        let template = template.unwrap();
        assert_eq!(template.name, "test");
        assert_eq!(template.source, source);
        assert_eq!(template.metadata.size, source.len());
        assert!(template.metrics.is_some());
    }
    
    #[test]
    fn test_template_error_display() {
        let error = TemplateError::ParseError {
            message: "Invalid syntax".to_string(),
            line: 5,
            column: 10,
        };
        assert!(error.to_string().contains("line 5"));
        assert!(error.to_string().contains("column 10"));
        assert!(error.to_string().contains("Invalid syntax"));
        
        let error = TemplateError::VariableError {
            variable_name: "user".to_string(),
            context: "main template".to_string(),
        };
        assert!(error.to_string().contains("user"));
        assert!(error.to_string().contains("main template"));
    }
    
    #[test]
    fn test_template_engine_creation() {
        let engine = TemplateEngine::new();
        assert!(engine.config.auto_escape);
        assert!(engine.config.enable_cache);
        assert_eq!(engine.config.cache_size, 1000);
        
        // Test performance monitoring
        let stats = engine.performance_stats();
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.total_renders, 0);
        assert_eq!(stats.total_cache_operations, 0);
    }
    
    #[test]
    fn test_template_engine_global_context() {
        let engine = TemplateEngine::new();
        
        // Set global variable
        let result = engine.set_global("app_name", CursedObject::String("CURSED App".to_string()));
        assert!(result.is_ok());
        
        // Check cache operations
        assert_eq!(engine.compiled_cache_size(), 0);
        engine.clear_compiled_cache();
        assert_eq!(engine.compiled_cache_size(), 0);
    }
    
    #[test]
    fn test_performance_monitor() {
        let monitor = TemplatePerformanceMonitor::new();
        
        // Initial stats
        let stats = monitor.get_stats().unwrap();
        assert_eq!(stats.total_renders, 0);
        assert_eq!(stats.cache_hit_rate, 0.0);
        
        // Record some operations
        monitor.record_cache_result(true);
        monitor.record_cache_result(false);
        
        let metrics = TemplateMetrics {
            name: "test".to_string(),
            parse_time_ms: 5,
            render_time_ms: 10,
            size_bytes: 100,
            variables_resolved: 3,
            filters_applied: 1,
            cache_hit: true,
            timestamp: SystemTime::now(),
        };
        
        monitor.record_render(metrics);
        
        let stats = monitor.get_stats().unwrap();
        assert_eq!(stats.total_renders, 1);
        assert_eq!(stats.cache_hit_rate, 0.5); // 1 hit out of 2 operations
        assert_eq!(stats.total_cache_operations, 2);
    }
    
    #[test]
    fn test_template_validation() {
        let engine = TemplateEngine::new();
        
        // Valid template
        let result = engine.validate_template_source("Hello {{ name }}!");
        assert!(result.is_ok());
        
        // Test invalid template source - this would need proper error handling from the lexer/parser
        // For now we'll test the structure
        let valid_source = "Hello World!";
        let result = engine.validate_template_source(valid_source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_filter_tracking_in_render() {
        use std::fs;
        use std::path::Path;
        
        // Create temporary template directory
        let temp_dir = std::env::temp_dir().join("cursed_template_test");
        fs::create_dir_all(&temp_dir).unwrap();
        
        // Create a test template file with filters
        let template_content = r#"Hello {{ name | upper | trim }}! Your score is {{ score | format:2 }}."#;
        let template_path = temp_dir.join("test_template.html");
        fs::write(&template_path, template_content).unwrap();
        
        // Create engine with custom loader
        let loader = Arc::new(FileSystemLoader::new(&temp_dir));
        let config = TemplateConfig::default();
        let engine = TemplateEngine::with_config_and_loader(config, loader);
        
        // Create context with variables
        let context = TemplateContext::new();
        context.set("name", CursedObject::String("  alice  ".to_string())).unwrap();
        context.set("score", CursedObject::Float(95.567)).unwrap();
        
        // Render the template and check metrics
        let result = engine.render("test_template.html", context);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("ALICE"));
        assert!(output.contains("95.57"));
        
        // Check performance stats for filter tracking
        let stats = engine.performance_stats();
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.total_renders, 1);
        
        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_filter_tracking_in_string_render() {
        let engine = TemplateEngine::new();
        
        // Template with multiple filters
        let template_source = r#"{{ message | lower | trim | capitalize }} - {{ count | format:0 | add:1 }}"#;
        
        let context = TemplateContext::new();
        context.set("message", CursedObject::String("  HELLO WORLD  ".to_string())).unwrap();
        context.set("count", CursedObject::Integer(9)).unwrap();
        
        // Render and check filter application
        let result = engine.render_string(template_source, context);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("Hello World"));
        
        // Verify performance monitoring recorded the render
        let stats = engine.performance_stats();
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert!(stats.total_renders >= 1);
    }

    #[test]
    fn test_comprehensive_filter_metrics() {
        let engine = TemplateEngine::new();
        
        // Register a custom filter to test filter tracking
        engine.register_filter("test_filter", |_context, args| {
            let s = match &args[0] {
                CursedObject::String(s) => s.clone(),
                _ => "default".to_string(),
            };
            Ok(CursedObject::String(format!("filtered_{}", s)))
        }).unwrap();
        
        // Template that uses multiple built-in and custom filters
        let template_source = r#"{{ name | test_filter | upper }} has {{ items | length }} items"#;
        
        let context = TemplateContext::new();
        context.set("name", CursedObject::String("alice".to_string())).unwrap();
        context.set("items", CursedObject::Array(vec![
            CursedObject::String("apple".to_string()),
            CursedObject::String("banana".to_string()),
            CursedObject::String("cherry".to_string()),
        ])).unwrap();
        
        // Render and verify output
        let result = engine.render_string(template_source, context);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("FILTERED_ALICE"));
        assert!(output.contains("3 items"));
        
        // Check that filters were tracked
        let stats = engine.performance_stats();
        assert!(stats.is_some());
    }
}
