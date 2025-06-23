/// Template Filters - Built-in functions and filter registry for CURSED templates
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, instrument, warn, info};
use base64::{Engine as _, engine::general_purpose};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;

/// Filter function type
pub type FilterFn = Box<dyn Fn(&FilterContext, &[CursedObject]) -> Result<(), Error> + Send + Sync>;

/// Template filter trait for implementing custom filters
pub trait TemplateFilter: Send + Sync {
    /// Filter name
    fn name(&self) -> &str;
    
    /// Filter description
    fn description(&self) -> &str { "" }
    
    /// Required number of arguments (None for variable args)
    fn required_args(&self) -> Option<usize> { None }
    
    /// Apply the filter
    fn apply(&self, context: &FilterContext, args: &[CursedObject]) -> Result<(), Error>;
    
    /// Whether this filter can be cached
    fn cacheable(&self) -> bool { true }
}

/// Filter execution context with metadata and performance tracking
#[derive(Debug, Clone)]
pub struct FilterContext {
    /// Current template being rendered
    pub template_name: Option<String>,
    /// Filter chain depth (for preventing infinite recursion)
    pub chain_depth: usize,
    /// Performance tracking enabled
    pub track_performance: bool,
    /// Cache enabled
    pub cache_enabled: bool,
    /// Maximum chain depth
    pub max_chain_depth: usize,
    /// Start time for performance tracking
    pub start_time: Option<Instant>,
}

impl FilterContext {
    /// Create new filter context
    pub fn new() -> Self {
        Self {
            template_name: None,
            chain_depth: 0,
            track_performance: true,
            cache_enabled: true,
            max_chain_depth: 10,
            start_time: None,
        }
    }
    
    /// Create context for template
    pub fn for_template(template_name: String) -> Self {
        Self {
            template_name: Some(template_name),
            ..Self::new()
        }
    }
    
    /// Increment chain depth
    pub fn deeper(&self) -> Result<(), Error> {
        if self.chain_depth >= self.max_chain_depth {
            return Err(CursedError::TemplateError {
                message: format!("Filter chain depth exceeded maximum of {}", self.max_chain_depth),
                source_location: None,
            });
        }
        
        Ok(Self {
            chain_depth: self.chain_depth + 1,
            ..self.clone()
        })
    }
}

impl Default for FilterContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Filter performance statistics
#[derive(Debug, Clone)]
pub struct FilterStats {
    pub call_count: u64,
    pub total_duration: Duration,
    pub avg_duration: Duration,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl FilterStats {
    fn new() -> Self {
        Self {
            call_count: 0,
            total_duration: Duration::from_nanos(0),
            avg_duration: Duration::from_nanos(0),
            cache_hits: 0,
            cache_misses: 0,
        }
    }
    
    fn record_call(&mut self, duration: Duration, cache_hit: bool) {
        self.call_count += 1;
        self.total_duration += duration;
        self.avg_duration = self.total_duration / self.call_count as u32;
        
        if cache_hit {
            self.cache_hits += 1;
        } else {
            self.cache_misses += 1;
        }
    }
}

/// Registry of template filters and functions
pub struct FilterRegistry {
    filters: Arc<RwLock<HashMap<String, FilterFn>>>,
    trait_filters: Arc<RwLock<HashMap<String, Box<dyn TemplateFilter>>>>,
    stats: Arc<RwLock<HashMap<String, FilterStats>>>,
    cache: Arc<RwLock<HashMap<String, CursedObject>>>,
}

impl std::fmt::Debug for FilterRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let filter_count = self.filters.read().unwrap().len();
        let trait_filter_count = self.trait_filters.read().unwrap().len();
        f.debug_struct("FilterRegistry")
            .field("filter_count", &filter_count)
            .field("trait_filter_count", &trait_filter_count)
            .field("total_filters", &(filter_count + trait_filter_count))
            .finish()
    }
}

impl FilterRegistry {
    /// Create a new filter registry with built-in filters
    pub fn new() -> Self {
        let registry = Self {
            filters: Arc::new(RwLock::new(HashMap::new())),
            trait_filters: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
        };
        registry.register_builtin_filters();
        registry.register_cursed_filters();
        registry
    }

    /// Register a custom filter function
    pub fn register<F>(&self, name: &str, filter: F)
    where
        F: Fn(&FilterContext, &[CursedObject]) -> Result<(), Error> + Send + Sync + 'static,
    {
        if let Ok(mut filters) = self.filters.write() {
            filters.insert(name.to_string(), Box::new(filter));
        }
    }
    
    /// Register a trait-based filter
    pub fn register_trait_filter<T>(&self, filter: T)
    where
        T: TemplateFilter + 'static,
    {
        let name = filter.name().to_string();
        if let Ok(mut trait_filters) = self.trait_filters.write() {
            trait_filters.insert(name, Box::new(filter));
        }
    }

    /// Apply a filter to values with context
    #[instrument(skip(self, args))]
    pub fn apply(&self, name: &str, args: &[CursedObject]) -> Result<(), Error> {
        let context = FilterContext::new();
        self.apply_with_context(name, &context, args)
    }
    
    /// Apply a filter with explicit context
    #[instrument(skip(self, args))]
    pub fn apply_with_context(&self, name: &str, context: &FilterContext, args: &[CursedObject]) -> Result<(), Error> {
        let start_time = Instant::now();
        
        // Check cache first if enabled
        if context.cache_enabled {
            let cache_key = format!("{}:{:?}", name, args);
            if let Ok(cache) = self.cache.read() {
                if let Some(cached_result) = cache.get(&cache_key) {
                    self.record_stats(name, start_time.elapsed(), true);
                    return Ok(cached_result.clone());
                }
            }
        }
        
        // Try trait-based filters first
        if let Ok(trait_filters) = self.trait_filters.read() {
            if let Some(filter) = trait_filters.get(name) {
                let result = filter.apply(context, args)?;
                
                // Cache result if cacheable
                if context.cache_enabled && filter.cacheable() {
                    let cache_key = format!("{}:{:?}", name, args);
                    if let Ok(mut cache) = self.cache.write() {
                        cache.insert(cache_key, result.clone());
                    }
                }
                
                self.record_stats(name, start_time.elapsed(), false);
                return Ok(result);
            }
        }
        
        // Try function-based filters
        if let Ok(filters) = self.filters.read() {
            if let Some(filter) = filters.get(name) {
                let result = filter(context, args)?;
                
                // Cache result
                if context.cache_enabled {
                    let cache_key = format!("{}:{:?}", name, args);
                    if let Ok(mut cache) = self.cache.write() {
                        cache.insert(cache_key, result.clone());
                    }
                }
                
                self.record_stats(name, start_time.elapsed(), false);
                return Ok(result);
            }
        }
        
        warn!(filter = name, "Unknown filter");
        Err(CursedError::TemplateError {
            message: format!("Unknown filter: {}", name),
            source_location: None,
        })
    }
    
    /// Clear filter cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }
    
    /// Get filter statistics
    pub fn get_stats(&self, name: &str) -> Option<FilterStats> {
        if let Ok(stats) = self.stats.read() {
            stats.get(name).cloned()
        } else {
            None
        }
    }
    
    /// Get all filter statistics
    pub fn get_all_stats(&self) -> HashMap<String, FilterStats> {
        if let Ok(stats) = self.stats.read() {
            stats.clone()
        } else {
            HashMap::new()
        }
    }
    
    /// Record filter execution statistics
    fn record_stats(&self, name: &str, duration: Duration, cache_hit: bool) {
        if let Ok(mut stats) = self.stats.write() {
            let filter_stats = stats.entry(name.to_string()).or_insert_with(FilterStats::new);
            filter_stats.record_call(duration, cache_hit);
        }
    }
    
    /// List all available filter names
    pub fn list_filters(&self) -> Vec<String> {
        let mut names = Vec::new();
        
        if let Ok(filters) = self.filters.read() {
            names.extend(filters.keys().cloned());
        }
        
        if let Ok(trait_filters) = self.trait_filters.read() {
            names.extend(trait_filters.keys().cloned());
        }
        
        names.sort();
        names
    }
    
    /// Check if a filter exists
    pub fn has_filter(&self, name: &str) -> bool {
        if let Ok(filters) = self.filters.read() {
            if filters.contains_key(name) {
                return true;
            }
        }
        
        if let Ok(trait_filters) = self.trait_filters.read() {
            if trait_filters.contains_key(name) {
                return true;
            }
        }
        
        false
    }

    /// Register all built-in filters
    fn register_builtin_filters(&self) {
        // Text manipulation filters
        self.register("lower", |_context, args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s.to_lowercase()))
        });

        self.register("upper", |_context, args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s.to_uppercase()))
        });

        self.register("title", |_context, args| {
            let s = extract_string(&args[0])?;
            let result = s.split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            Ok(CursedObject::String(result))
        });

        self.register("trim", |_context, args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s.trim().to_string()))
        });

        self.register("trimSpace", |_context, args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s.trim().to_string()))
        });

        self.register("trimPrefix", |_context, args| {
            let s = extract_string(&args[0])?;
            let prefix = extract_string(&args[1])?;
            let result = s.strip_prefix(&prefix).unwrap_or(&s);
            Ok(CursedObject::String(result.to_string()))
        });

        self.register("trimSuffix", |_context, args| {
            let s = extract_string(&args[0])?;
            let suffix = extract_string(&args[1])?;
            let result = s.strip_suffix(&suffix).unwrap_or(&s);
            Ok(CursedObject::String(result.to_string()))
        });

        self.register("replace", |_context, args| {
            let s = extract_string(&args[0])?;
            let old = extract_string(&args[1])?;
            let new = extract_string(&args[2])?;
            Ok(CursedObject::String(s.replacen(&old, &new, 1)))
        });

        self.register("replaceAll", |_context, args| {
            let s = extract_string(&args[0])?;
            let old = extract_string(&args[1])?;
            let new = extract_string(&args[2])?;
            Ok(CursedObject::String(s.replace(&old, &new)))
        });

        self.register("split", |_context, args| {
            let s = extract_string(&args[0])?;
            let sep = extract_string(&args[1])?;
            let parts: Vec<CursedObject> = s.split(&sep)
                .map(|part| CursedObject::String(part.to_string()))
                .collect();
            Ok(CursedObject::Array(parts))
        });

        self.register("join", |_context, args| {
            let arr = extract_array(&args[0])?;
            let sep = extract_string(&args[1])?;
            let strings: Result<(), Error> = arr.iter()
                .map(extract_string)
                .collect();
            Ok(CursedObject::String(strings?.join(&sep)))
        });

        self.register("contains", |_context, args| {
            let s = extract_string(&args[0])?;
            let substr = extract_string(&args[1])?;
            Ok(CursedObject::Boolean(s.contains(&substr)))
        });

        self.register("hasPrefix", |_context, args| {
            let s = extract_string(&args[0])?;
            let prefix = extract_string(&args[1])?;
            Ok(CursedObject::Boolean(s.starts_with(&prefix)))
        });

        self.register("hasSuffix", |_context, args| {
            let s = extract_string(&args[0])?;
            let suffix = extract_string(&args[1])?;
            Ok(CursedObject::Boolean(s.ends_with(&suffix)))
        });

        self.register("substr", |_context, args| {
            let s = extract_string(&args[0])?;
            let start = extract_int(&args[1])? as usize;
            let end = extract_int(&args[2])? as usize;
            let chars: Vec<char> = s.chars().collect();
            if start >= chars.len() {
                return Ok(CursedObject::String("".to_string()));
            }
            let end = end.min(chars.len());
            let result: String = chars[start..end].iter().collect();
            Ok(CursedObject::String(result))
        });

        self.register("repeat", |_context, args| {
            let s = extract_string(&args[0])?;
            let count = extract_int(&args[1])? as usize;
            Ok(CursedObject::String(s.repeat(count)))
        });

        self.register("runeCount", |_context, args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::Integer(s.chars().count() as i64))
        });

        self.register("index", |_context, args| {
            let s = extract_string(&args[0])?;
            let substr = extract_string(&args[1])?;
            match s.find(&substr) {
                Some(pos) => Ok(CursedObject::Integer(pos as i64)),
                None => Ok(CursedObject::Integer(-1)),
            }
        });

        self.register("lastIndex", |_context, args| {
            let s = extract_string(&args[0])?;
            let substr = extract_string(&args[1])?;
            match s.rfind(&substr) {
                Some(pos) => Ok(CursedObject::Integer(pos as i64)),
                None => Ok(CursedObject::Integer(-1)),
            }
        });

        // Formatting filters
        self.register("printf", |_context, args| {
            if args.is_empty() {
                return Err(CursedError::TemplateError {
                    message: "printf filter requires at least a format string".to_string(),
                    source_location: None,
                });
            }
            
            let format = extract_string(&args[0])?;
            let format_args = &args[1..];
            
            format_string_cursed(&format, format_args)
        });

        self.register("sprintf", |_context, args| {
            if args.is_empty() {
                return Err(CursedError::TemplateError {
                    message: "sprintf filter requires at least a format string".to_string(),
                    source_location: None,
                });
            }
            
            let format = extract_string(&args[0])?;
            let format_args = &args[1..];
            
            format_string_cursed(&format, format_args)
        });

        self.register("numFormat", |_context, args| {
            let num = extract_float(&args[0])?;
            let precision = extract_int(&args[1])? as usize;
            Ok(CursedObject::String(format!("{:.prec$}", num, prec = precision)))
        });

        self.register("currency", |_context, args| {
            let num = extract_float(&args[0])?;
            Ok(CursedObject::String(format!("${:.2}", num)))
        });

        self.register("byteSize", |_context, args| {
            let bytes = extract_int(&args[0])? as f64;
            let units = ["B", "KB", "MB", "GB", "TB"];
            let mut size = bytes;
            let mut unit_index = 0;
            
            while size >= 1024.0 && unit_index < units.len() - 1 {
                size /= 1024.0;
                unit_index += 1;
            }
            
            Ok(CursedObject::String(format!("{:.1} {}", size, units[unit_index])))
        });

        self.register("percentage", |_context, args| {
            let num = extract_float(&args[0])?;
            Ok(CursedObject::String(format!("{:.1}%", num * 100.0)))
        });

        self.register("plural", |_context, args| {
            let count = extract_int(&args[0])?;
            let singular = extract_string(&args[1])?;
            let plural = extract_string(&args[2])?;
            if count == 1 {
                Ok(CursedObject::String(singular))
            } else {
                Ok(CursedObject::String(plural))
            }
        });

        // Collection operations
        self.register("len", |_context, args| {
            match &args[0] {
                CursedObject::String(s) => Ok(CursedObject::Integer(s.chars().count() as i64)),
                CursedObject::Array(arr) => Ok(CursedObject::Integer(arr.len() as i64)),
                CursedObject::Map(map) => Ok(CursedObject::Integer(map.len() as i64)),
                _ => Ok(CursedObject::Integer(0)),
            }
        });

        self.register("slice", |_context, args| {
            let arr = extract_array(&args[0])?;
            let start = extract_int(&args[1])? as usize;
            let end = extract_int(&args[2])? as usize;
            let end = end.min(arr.len());
            if start >= arr.len() {
                return Ok(CursedObject::Array(vec![]));
            }
            Ok(CursedObject::Array(arr[start..end].to_vec()))
        });

        self.register("reverse", |_context, args| {
            let mut arr = extract_array(&args[0])?;
            arr.reverse();
            Ok(CursedObject::Array(arr))
        });

        self.register("first", |_context, args| {
            let arr = extract_array(&args[0])?;
            if arr.is_empty() {
                Ok(CursedObject::Nil)
            } else {
                Ok(arr[0].clone())
            }
        });

        self.register("last", |_context, args| {
            let arr = extract_array(&args[0])?;
            if arr.is_empty() {
                Ok(CursedObject::Nil)
            } else {
                Ok(arr[arr.len() - 1].clone())
            }
        });

        self.register("keys", |_context, args| {
            match &args[0] {
                CursedObject::Map(map) => {
                    let keys: Vec<CursedObject> = map.keys()
                        .map(|k| CursedObject::String(k.clone()))
                        .collect();
                    Ok(CursedObject::Array(keys))
                }
                _ => Err(CursedError::TemplateError {
                    message: "keys filter requires a map".to_string(),
                    source_location: None,
                }),
            }
        });

        self.register("values", |_context, args| {
            match &args[0] {
                CursedObject::Map(map) => {
                    let values: Vec<CursedObject> = map.values().cloned().collect();
                    Ok(CursedObject::Array(values))
                }
                _ => Err(CursedError::TemplateError {
                    message: "values filter requires a map".to_string(),
                    source_location: None,
                }),
            }
        });

        // Data conversion filters
        self.register("toJSON", |_context, args| {
            // Simplified JSON serialization
            match &args[0] {
                CursedObject::String(s) => Ok(CursedObject::String(format!("\"{}\"", s))),
                CursedObject::Integer(n) => Ok(CursedObject::String(n.to_string())),
                CursedObject::Float(n) => Ok(CursedObject::String(n.to_string())),
                CursedObject::Boolean(b) => Ok(CursedObject::String(b.to_string())),
                CursedObject::Nil => Ok(CursedObject::String("null".to_string())),
                _ => Ok(CursedObject::String("{}".to_string())),
            }
        });

        self.register("toBase64", |_context, args| {
            let s = extract_string(&args[0])?;
            let encoded = general_purpose::STANDARD.encode(s.as_bytes());
            Ok(CursedObject::String(encoded))
        });

        self.register("fromBase64", |_context, args| {
            let s = extract_string(&args[0])?;
            match general_purpose::STANDARD.decode(&s) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(decoded) => Ok(CursedObject::String(decoded)),
                    Err(_) => Err(CursedError::TemplateError {
                        message: "Invalid UTF-8 in base64 data".to_string(),
                        source_location: None,
                    }),
                },
                Err(_) => Err(CursedError::TemplateError {
                    message: "Invalid base64 encoding".to_string(),
                    source_location: None,
                }),
            }
        });

        self.register("toBool", |_context, args| {
            let result = match &args[0] {
                CursedObject::Boolean(b) => *b,
                CursedObject::String(s) => !s.is_empty(),
                CursedObject::Integer(n) => *n != 0,
                CursedObject::Float(n) => *n != 0.0,
                CursedObject::Nil => false,
                _ => true,
            };
            Ok(CursedObject::Boolean(result))
        });

        self.register("toString", |_context, args| {
            let result = match &args[0] {
                CursedObject::String(s) => s.clone(),
                CursedObject::Integer(n) => n.to_string(),
                CursedObject::Float(n) => n.to_string(),
                CursedObject::Boolean(b) => b.to_string(),
            CursedObject::Char(c) => c.to_string(),
                CursedObject::Nil => "".to_string(),
                _ => format!("{:?}", args[0]),
            };
            Ok(CursedObject::String(result))
        });

        self.register("toInt", |_context, args| {
            match &args[0] {
                CursedObject::Integer(n) => Ok(CursedObject::Integer(*n)),
                CursedObject::Float(n) => Ok(CursedObject::Integer(*n as i64)),
                CursedObject::String(s) => {
                    match s.parse::<i64>() {
                        Ok(n) => Ok(CursedObject::Integer(n)),
                        Err(_) => Err(CursedError::TemplateError {
                            message: format!("Cannot convert '{}' to integer", s),
                            source_location: None,
                        }),
                    }
                }
                _ => Err(CursedError::TemplateError {
                    message: "Cannot convert to integer".to_string(),
                    source_location: None,
                }),
            }
        });

        self.register("toFloat", |_context, args| {
            match &args[0] {
                CursedObject::Float(n) => Ok(CursedObject::Float(*n)),
                CursedObject::Integer(n) => Ok(CursedObject::Float(*n as f64)),
                CursedObject::String(s) => {
                    match s.parse::<f64>() {
                        Ok(n) => Ok(CursedObject::Float(n)),
                        Err(_) => Err(CursedError::TemplateError {
                            message: format!("Cannot convert '{}' to float", s),
                            source_location: None,
                        }),
                    }
                }
                _ => Err(CursedError::TemplateError {
                    message: "Cannot convert to float".to_string(),
                    source_location: None,
                }),
            }
        });

        // Control flow filters
        self.register("eq", |_context, args| {
            Ok(CursedObject::Boolean(objects_equal(&args[0], &args[1])))
        });

        self.register("ne", |_context, args| {
            Ok(CursedObject::Boolean(!objects_equal(&args[0], &args[1])))
        });

        self.register("lt", |_context, args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Boolean(a < b))
        });

        self.register("le", |_context, args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Boolean(a <= b))
        });

        self.register("gt", |_context, args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Boolean(a > b))
        });

        self.register("ge", |_context, args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Boolean(a >= b))
        });

        self.register("and", |_context, args| {
            let a = is_truthy(&args[0]);
            let b = is_truthy(&args[1]);
            Ok(CursedObject::Boolean(a && b))
        });

        self.register("or", |_context, args| {
            let a = is_truthy(&args[0]);
            let b = is_truthy(&args[1]);
            Ok(CursedObject::Boolean(a || b))
        });

        self.register("not", |_context, args| {
            Ok(CursedObject::Boolean(!is_truthy(&args[0])))
        });

        self.register("ternary", |_context, args| {
            let condition = is_truthy(&args[0]);
            if condition {
                Ok(args[1].clone())
            } else {
                Ok(args[2].clone())
            }
        });

        self.register("isZero", |_context, args| {
            let result = match &args[0] {
                CursedObject::Integer(n) => *n == 0,
                CursedObject::Float(n) => *n == 0.0,
                _ => false,
            };
            Ok(CursedObject::Boolean(result))
        });

        self.register("isNil", |_context, args| {
            Ok(CursedObject::Boolean(matches!(args[0], CursedObject::Nil)))
        });

        self.register("isEmpty", |_context, args| {
            let result = match &args[0] {
                CursedObject::String(s) => s.is_empty(),
                CursedObject::Array(arr) => arr.is_empty(),
                CursedObject::Map(map) => map.is_empty(),
                CursedObject::Nil => true,
                _ => false,
            };
            Ok(CursedObject::Boolean(result))
        });

        // URL and HTML filters
        self.register("urlEncode", |_context, args| {
            let s = extract_string(&args[0])?;
            let encoded = urlencoding::encode(&s);
            Ok(CursedObject::String(encoded.to_string()))
        });

        self.register("urlDecode", |_context, args| {
            let s = extract_string(&args[0])?;
            match urlencoding::decode(&s) {
                Ok(decoded) => Ok(CursedObject::String(decoded.to_string())),
                Err(_) => Err(CursedError::TemplateError {
                    message: "Invalid URL encoding".to_string(),
                    source_location: None,
                }),
            }
        });

        self.register("htmlEscape", |_context, args| {
            let s = extract_string(&args[0])?;
            let escaped = s
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#x27;");
            Ok(CursedObject::String(escaped))
        });

        self.register("htmlUnescape", |_context, args| {
            let s = extract_string(&args[0])?;
            let unescaped = s
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#x27;", "'");
            Ok(CursedObject::String(unescaped))
        });

        self.register("pathEscape", |_context, args| {
            let s = extract_string(&args[0])?;
            let escaped = urlencoding::encode(&s);
            Ok(CursedObject::String(escaped.to_string()))
        });

        self.register("queryEscape", |_context, args| {
            let s = extract_string(&args[0])?;
            let escaped = urlencoding::encode(&s);
            Ok(CursedObject::String(escaped.to_string()))
        });

        self.register("cssEscape", |_context, args| {
            let s = extract_string(&args[0])?;
            // Simplified CSS escaping
            let escaped = s.chars()
                .map(|c| if c.is_alphanumeric() { c.to_string() } else { format!("\\{:x}", c as u32) })
                .collect::<String>();
            Ok(CursedObject::String(escaped))
        });

        self.register("jsEscape", |_context, args| {
            let s = extract_string(&args[0])?;
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\'', "\\'")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t");
            Ok(CursedObject::String(escaped))
        });

        // Safe HTML/URL/JS/CSS markers (return the string as-is but mark as safe)
        self.register("safeHTML", |_context, args| Ok(args[0].clone()));
        self.register("safeURL", |_context, args| Ok(args[0].clone()));
        self.register("safeJS", |_context, args| Ok(args[0].clone()));
        self.register("safeCSS", |_context, args| Ok(args[0].clone()));

        // Random and Math filters
        self.register("randomInt", |_context, args| {
            let min = extract_int(&args[0])?;
            let max = extract_int(&args[1])?;
            let result = min + (rand::random::<i64>() % (max - min + 1));
            Ok(CursedObject::Integer(result))
        });

        self.register("randomString", |_context, args| {
            let length = extract_int(&args[0])? as usize;
            let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            let result: String = (0..length)
                .map(|_| {
                    let idx = rand::random::<usize>() % charset.len();
                    charset[idx] as char
                })
                .collect();
            Ok(CursedObject::String(result))
        });

        self.register("uuid", |_context, _args| {
            let uuid = Uuid::new_v4();
            Ok(CursedObject::String(uuid.to_string()))
        });

        self.register("add", |_context, args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Float(a + b))
        });

        self.register("sub", |_context, args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Float(a - b))
        });

        self.register("mul", |_context, args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Float(a * b))
        });

        self.register("div", |_context, args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            if b == 0.0 {
                return Err(CursedError::TemplateError {
                    message: "Division by zero".to_string(),
                    source_location: None,
                });
            }
            Ok(CursedObject::Float(a / b))
        });

        self.register("mod", |_context, args| {
            let a = extract_int(&args[0])?;
            let b = extract_int(&args[1])?;
            if b == 0 {
                return Err(CursedError::TemplateError {
                    message: "Modulo by zero".to_string(),
                    source_location: None,
                });
            }
            Ok(CursedObject::Integer(a % b))
        });

        self.register("max", |_context, args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Float(a.max(b)))
        });

        self.register("min", |_context, args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Float(a.min(b)))
        });

        self.register("round", |_context, args| {
            let num = extract_float(&args[0])?;
            let precision = extract_int(&args[1])? as u32;
            let multiplier = 10.0_f64.powi(precision as i32);
            Ok(CursedObject::Float((num * multiplier).round() / multiplier))
        });

        self.register("ceil", |_context, args| {
            let num = extract_float(&args[0])?;
            Ok(CursedObject::Float(num.ceil()))
        });

        self.register("floor", |_context, args| {
            let num = extract_float(&args[0])?;
            Ok(CursedObject::Float(num.floor()))
        });
    }
    
    /// Register CURSED-style Gen Z slang filters
    fn register_cursed_filters(&self) {
        // Gen Z slang aliases for common filters
        self.register("no_cap", |_context, args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s.trim().to_string()))
        });
        
        self.register("vibes", |_context, args| {
            let num = extract_float(&args[0])?;
            let precision = if args.len() > 1 { extract_int(&args[1])? as usize } else { 2 };
            Ok(CursedObject::String(format!("{:.prec$}", num, prec = precision)))
        });
        
        self.register("bestie", |_context, args| {
            let arr = extract_array(&args[0])?;
            let sep = if args.len() > 1 { extract_string(&args[1])? } else { ", ".to_string() };
            let strings: Result<(), Error> = arr.iter()
                .map(extract_string)
                .collect();
            Ok(CursedObject::String(strings?.join(&sep)))
        });
        
        self.register("sus_check", |_context, args| {
            Ok(CursedObject::Boolean(is_truthy(&args[0])))
        });
        
        self.register("slay_contains", |_context, args| {
            let s = extract_string(&args[0])?;
            let substr = extract_string(&args[1])?;
            Ok(CursedObject::Boolean(s.contains(&substr)))
        });
        
        self.register("periodt", |_context, args| {
            let mut s = extract_string(&args[0])?;
            if !s.ends_with('.') && !s.ends_with('!') && !s.ends_with('?') {
                s.push('.');
            }
            Ok(CursedObject::String(s))
        });
        
        self.register("lowkey", |_context, args| {
            let condition = is_truthy(&args[0]);
            if condition && args.len() > 1 {
                Ok(args[1].clone())
            } else if !condition && args.len() > 2 {
                Ok(args[2].clone())
            } else {
                Ok(CursedObject::Nil)
            }
        });
        
        self.register("stan", |_context, args| {
            let s = extract_string(&args[0])?;
            let count = if args.len() > 1 { extract_int(&args[1])? as usize } else { 3 };
            let result = (0..count).map(|_| s.clone()).collect::<Vec<_>>().join(" ");
            Ok(CursedObject::String(result))
        });
        
        self.register("flex", |_context, args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s.to_uppercase() + " 💪"))
        });
        
        self.register("fr_fr", |_context, args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s + " for real"))
        });
        
        self.register("bet", |_context, args| {
            let condition = is_truthy(&args[0]);
            Ok(CursedObject::String(if condition { "bet" } else { "nah" }.to_string()))
        });
        
        self.register("fire", |_context, args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s + " 🔥"))
        });
        
        self.register("bussin", |_context, args| {
            let rating = extract_float(&args[0])?;
            let result = if rating >= 8.0 {
                "bussin 😍"
            } else if rating >= 6.0 {
                "it's alright"
            } else {
                "nah chief"
            };
            Ok(CursedObject::String(result.to_string()))
        });
        
        // Date/time filters
        self.register("date_format", |_context, args| {
            let timestamp = extract_int(&args[0])? as i64;
            let format = if args.len() > 1 { extract_string(&args[1])? } else { "%Y-%m-%d".to_string() };
            
            if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
                Ok(CursedObject::String(dt.format(&format).to_string()))
            } else {
                Err(CursedError::TemplateError {
                    message: "Invalid timestamp".to_string(),
                    source_location: None,
                })
            }
        });
        
        self.register("time_ago", |_context, args| {
            let timestamp = extract_int(&args[0])? as i64;
            let now = Utc::now().timestamp();
            let diff = now - timestamp;
            
            let result = if diff < 60 {
                "just now".to_string()
            } else if diff < 3600 {
                format!("{} minutes ago", diff / 60)
            } else if diff < 86400 {
                format!("{} hours ago", diff / 3600)
            } else {
                format!("{} days ago", diff / 86400)
            };
            
            Ok(CursedObject::String(result))
        });
        
        self.register("strftime", |_context, args| {
            let timestamp = extract_int(&args[0])? as i64;
            let format = extract_string(&args[1])?;
            
            if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
                Ok(CursedObject::String(dt.format(&format).to_string()))
            } else {
                Err(CursedError::TemplateError {
                    message: "Invalid timestamp".to_string(),
                    source_location: None,
                })
            }
        });
        
        // HTML/Markdown filters
        self.register("markdown", |_context, args| {
            let text = extract_string(&args[0])?;
            // Simple markdown-to-HTML conversion
            let html = text
                .replace("**", "<strong>")
                .replace("__", "</strong>")
                .replace("*", "<em>")
                .replace("_", "</em>")
                .replace("\n\n", "</p><p>")
                .replace("\n", "<br>");
            Ok(CursedObject::String(format!("<p>{}</p>", html)))
        });
        
        self.register("strip_tags", |_context, args| {
            let html = extract_string(&args[0])?;
            // Simple HTML tag removal
            let result = html.chars()
                .fold((String::new(), false), |(mut acc, in_tag), c| {
                    match c {
                        '<' => (acc, true),
                        '>' => (acc, false),
                        _ if !in_tag => {
                            acc.push(c);
                            (acc, in_tag)
                        }
                        _ => (acc, in_tag),
                    }
                }).0;
            Ok(CursedObject::String(result))
        });
        
        self.register("escape", |_context, args| {
            let s = extract_string(&args[0])?;
            let escaped = s
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#x27;");
            Ok(CursedObject::String(escaped))
        });
        
        self.register("unescape", |_context, args| {
            let s = extract_string(&args[0])?;
            let unescaped = s
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#x27;", "'");
            Ok(CursedObject::String(unescaped))
        });
        
        // Collection filters with more functionality
        self.register("sort", |_context, args| {
            let mut arr = extract_array(&args[0])?;
            arr.sort_by(|a, b| {
                match (a, b) {
                    (CursedObject::String(s1), CursedObject::String(s2)) => s1.cmp(s2),
                    (CursedObject::Integer(i1), CursedObject::Integer(i2)) => i1.cmp(i2),
                    (CursedObject::Float(f1), CursedObject::Float(f2)) => f1.partial_cmp(f2).unwrap_or(std::cmp::Ordering::Equal),
                    _ => std::cmp::Ordering::Equal,
                }
            });
            Ok(CursedObject::Array(arr))
        });
        
        self.register("filter", |_context, args| {
            let arr = extract_array(&args[0])?;
            let condition = if args.len() > 1 { &args[1] } else { &CursedObject::Boolean(true) };
            
            let filtered: Vec<CursedObject> = arr.into_iter()
                .filter(|item| {
                    match condition {
                        CursedObject::String(field) if field == "truthy" => is_truthy(item),
                        CursedObject::String(field) if field == "non_empty" => match item {
                            CursedObject::String(s) => !s.is_empty(),
                            CursedObject::Array(a) => !a.is_empty(),
                            _ => true,
                        },
                        _ => is_truthy(condition),
                    }
                })
                .collect();
            
            Ok(CursedObject::Array(filtered))
        });
        
        self.register("map", |_context, args| {
            let arr = extract_array(&args[0])?;
            let operation = if args.len() > 1 { extract_string(&args[1])? } else { "toString".to_string() };
            
            let mapped: Result<(), Error> = arr.iter()
                .map(|item| {
                    match operation.as_str() {
                        "upper" => match item {
                            CursedObject::String(s) => Ok(CursedObject::String(s.to_uppercase())),
                            _ => Ok(item.clone()),
                        },
                        "lower" => match item {
                            CursedObject::String(s) => Ok(CursedObject::String(s.to_lowercase())),
                            _ => Ok(item.clone()),
                        },
                        "toString" => Ok(CursedObject::String(match item {
                            CursedObject::String(s) => s.clone(),
                            CursedObject::Integer(n) => n.to_string(),
                            CursedObject::Float(n) => n.to_string(),
                            CursedObject::Boolean(b) => b.to_string(),
                            CursedObject::Char(c) => c.to_string(),
                            _ => format!("{:?}", item),
                        })),
                        _ => Ok(item.clone()),
                    }
                })
                .collect();
            
            Ok(CursedObject::Array(mapped?))
        });
        
        self.register("length", |_context, args| {
            match &args[0] {
                CursedObject::String(s) => Ok(CursedObject::Integer(s.chars().count() as i64)),
                CursedObject::Array(arr) => Ok(CursedObject::Integer(arr.len() as i64)),
                CursedObject::Map(map) => Ok(CursedObject::Integer(map.len() as i64)),
                _ => Ok(CursedObject::Integer(0)),
            }
        });
        
        self.register("truncate", |_context, args| {
            let s = extract_string(&args[0])?;
            let max_len = extract_int(&args[1])? as usize;
            let suffix = if args.len() > 2 { extract_string(&args[2])? } else { "...".to_string() };
            
            if s.chars().count() <= max_len {
                Ok(CursedObject::String(s))
            } else {
                let truncated: String = s.chars().take(max_len.saturating_sub(suffix.chars().count())).collect();
                Ok(CursedObject::String(truncated + &suffix))
            }
        });
        
        // Number formatting filters
        self.register("format", |_context, args| {
            let num = extract_float(&args[0])?;
            let precision = if args.len() > 1 { extract_int(&args[1])? as usize } else { 2 };
            Ok(CursedObject::String(format!("{:.prec$}", num, prec = precision)))
        });
        
        self.register("abs", |_context, args| {
            match &args[0] {
                CursedObject::Integer(n) => Ok(CursedObject::Integer(n.abs())),
                CursedObject::Float(n) => Ok(CursedObject::Float(n.abs())),
                _ => Err(CursedError::TemplateError {
                    message: "abs filter requires a number".to_string(),
                    source_location: None,
                }),
            }
        });
        
        self.register("to_string", |_context, args| {
            let result = match &args[0] {
                CursedObject::String(s) => s.clone(),
                CursedObject::Integer(n) => n.to_string(),
                CursedObject::Float(n) => n.to_string(),
                CursedObject::Boolean(b) => b.to_string(),
                CursedObject::Char(c) => c.to_string(),
                CursedObject::Nil => "".to_string(),
                _ => format!("{:?}", args[0]),
            };
            Ok(CursedObject::String(result))
        });
    }
}

/// Example trait-based filters
pub struct CapitalizeFilter;

impl TemplateFilter for CapitalizeFilter {
    fn name(&self) -> &str { "capitalize" }
    fn description(&self) -> &str { "Capitalize the first letter of each word" }
    fn required_args(&self) -> Option<usize> { Some(1) }
    
    fn apply(&self, _context: &FilterContext, args: &[CursedObject]) -> Result<(), Error> {
        let s = extract_string(&args[0])?;
        let result = s.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        Ok(CursedObject::String(result))
    }
}

pub struct ReverseStringFilter;

impl TemplateFilter for ReverseStringFilter {
    fn name(&self) -> &str { "reverse_string" }
    fn description(&self) -> &str { "Reverse a string character by character" }
    fn required_args(&self) -> Option<usize> { Some(1) }
    
    fn apply(&self, _context: &FilterContext, args: &[CursedObject]) -> Result<(), Error> {
        let s = extract_string(&args[0])?;
        let reversed: String = s.chars().rev().collect();
        Ok(CursedObject::String(reversed))
    }
}

pub struct ChainableFilter;

impl TemplateFilter for ChainableFilter {
    fn name(&self) -> &str { "chain" }
    fn description(&self) -> &str { "Apply multiple filters in sequence" }
    fn cacheable(&self) -> bool { false } // Don't cache chained operations
    
    fn apply(&self, context: &FilterContext, args: &[CursedObject]) -> Result<(), Error> {
        if args.len() < 2 {
            return Err(CursedError::TemplateError {
                message: "chain filter requires at least 2 arguments: value and filter name".to_string(),
                source_location: None,
            });
        }
        
        let mut result = args[0].clone();
        let context = context.deeper()?;
        
        // Apply each filter in sequence
        for filter_arg in &args[1..] {
            let filter_name = extract_string(filter_arg)?;
            
            // For now, we need access to the registry to apply nested filters
            // This would need to be passed through context in a real implementation
            warn!("Chain filter needs registry access - not fully implemented");
            return Ok(result);
        }
        
        Ok(result)
    }
}

impl Default for FilterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to extract string from CursedObject
fn extract_string(obj: &CursedObject) -> Result<(), Error> {
    match obj {
        CursedObject::String(s) => Ok(s.clone()),
        CursedObject::Integer(n) => Ok(n.to_string()),
        CursedObject::Float(n) => Ok(n.to_string()),
        CursedObject::Boolean(b) => Ok(b.to_string()),
            CursedObject::Char(c) => Ok(c.to_string()),
        _ => Err(CursedError::TemplateError {
            message: "Expected string value".to_string(),
            source_location: None,
        }),
    }
}

/// Helper function to extract integer from CursedObject
fn extract_int(obj: &CursedObject) -> Result<(), Error> {
    match obj {
        CursedObject::Integer(n) => Ok(*n),
        CursedObject::Float(n) => Ok(*n as i64),
        CursedObject::String(s) => {
            s.parse::<i64>().map_err(|_| CursedError::TemplateError {
                message: "Expected integer value".to_string(),
                source_location: None,
            })
        }
        _ => Err(CursedError::TemplateError {
            message: "Expected integer value".to_string(),
            source_location: None,
        }),
    }
}

/// Helper function to extract float from CursedObject
fn extract_float(obj: &CursedObject) -> Result<(), Error> {
    match obj {
        CursedObject::Float(n) => Ok(*n),
        CursedObject::Integer(n) => Ok(*n as f64),
        CursedObject::String(s) => {
            s.parse::<f64>().map_err(|_| CursedError::TemplateError {
                message: "Expected numeric value".to_string(),
                source_location: None,
            })
        }
        _ => Err(CursedError::TemplateError {
            message: "Expected numeric value".to_string(),
            source_location: None,
        }),
    }
}

/// Helper function to extract array from CursedObject
fn extract_array(obj: &CursedObject) -> Result<(), Error> {
    match obj {
        CursedObject::Array(arr) => Ok(arr.clone()),
        _ => Err(CursedError::TemplateError {
            message: "Expected array value".to_string(),
            source_location: None,
        }),
    }
}

/// Helper function to check if objects are equal
fn objects_equal(left: &CursedObject, right: &CursedObject) -> bool {
    match (left, right) {
        (CursedObject::String(a), CursedObject::String(b)) => a == b,
        (CursedObject::Integer(a), CursedObject::Integer(b)) => a == b,
        (CursedObject::Float(a), CursedObject::Float(b)) => (a - b).abs() < f64::EPSILON,
        (CursedObject::Boolean(a), CursedObject::Boolean(b)) => a == b,
        (CursedObject::Nil, CursedObject::Nil) => true,
        _ => false,
    }
}

/// Helper function to check if object is truthy
fn is_truthy(obj: &CursedObject) -> bool {
    match obj {
        CursedObject::Boolean(b) => *b,
        CursedObject::Nil => false,
        CursedObject::Integer(n) => *n != 0,
        CursedObject::Float(n) => *n != 0.0,
        CursedObject::String(s) => !s.is_empty(),
        CursedObject::Char(_) => true, // Characters are always truthy
        CursedObject::Array(arr) => !arr.is_empty(),
        CursedObject::Map(map) => !map.is_empty(),
    }
}

/// Advanced string formatting function with printf-style placeholders
fn format_string_cursed(format: &str, args: &[CursedObject]) -> Result<(), Error> {
    let mut result = String::new();
    let mut chars = format.chars().peekable();
    let mut arg_index = 0;
    
    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '%' {
                    // Escaped %
                    chars.next();
                    result.push('%');
                } else {
                    // Format specifier
                    let format_spec = parse_format_specifier(&mut chars)?;
                    
                    if arg_index >= args.len() {
                        return Err(CursedError::TemplateError {
                            message: format!("Not enough arguments for format string: expected at least {}", arg_index + 1),
                            source_location: None,
                        });
                    }
                    
                    let formatted = format_argument(&args[arg_index], &format_spec)?;
                    result.push_str(&formatted);
                    arg_index += 1;
                }
            } else {
                result.push(ch);
            }
        } else if ch == '{' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '{' {
                    // Escaped { - consume the second { and add single {
                    chars.next();
                    result.push('{');
                } else if next_ch == '}' {
                    // Empty braces {} - simple placeholder
                    chars.next();
                    
                    if arg_index >= args.len() {
                        return Err(CursedError::TemplateError {
                            message: format!("Not enough arguments for format string: expected at least {}", arg_index + 1),
                            source_location: None,
                        });
                    }
                    
                    let formatted = extract_string(&args[arg_index])?;
                    result.push_str(&formatted);
                    arg_index += 1;
                } else {
                    // Numbered or named placeholder
                    let placeholder = parse_placeholder(&mut chars)?;
                    
                    let formatted = match placeholder.parse::<usize>() {
                        Ok(index) => {
                            if index >= args.len() {
                                return Err(CursedError::TemplateError {
                                    message: format!("Argument index {} out of range", index),
                                    source_location: None,
                                });
                            }
                            extract_string(&args[index])?
                        }
                        Err(_) => {
                            // For named placeholders, just use current arg_index
                            if arg_index >= args.len() {
                                return Err(CursedError::TemplateError {
                                    message: format!("Not enough arguments for format string: expected at least {}", arg_index + 1),
                                    source_location: None,
                                });
                            }
                            let formatted = extract_string(&args[arg_index])?;
                            arg_index += 1;
                            formatted
                        }
                    };
                    
                    result.push_str(&formatted);
                }
            } else {
                result.push(ch);
            }
        } else if ch == '}' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '}' {
                    // Escaped } - consume the second } and add single }
                    chars.next();
                    result.push('}');
                } else {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok(CursedObject::String(result))
}

/// Parse a printf-style format specifier
fn parse_format_specifier(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<(), Error> {
    let mut spec = FormatSpecifier::default();
    
    // Parse flags, width, precision, and conversion character
    while let Some(&ch) = chars.peek() {
        match ch {
            'c' => {
                chars.next();
                spec.conversion = 'c';
                break;
            }
            's' => {
                chars.next();
                spec.conversion = 's';
                break;
            }
            'd' | 'i' => {
                chars.next();
                spec.conversion = 'd';
                break;
            }
            'f' => {
                chars.next();
                spec.conversion = 'f';
                break;
            }
            'e' => {
                chars.next();
                spec.conversion = 'e';
                break;
            }
            'g' => {
                chars.next();
                spec.conversion = 'g';
                break;
            }
            'x' => {
                chars.next();
                spec.conversion = 'x';
                break;
            }
            'X' => {
                chars.next();
                spec.conversion = 'X';
                break;
            }
            'o' => {
                chars.next();
                spec.conversion = 'o';
                break;
            }
            '0'..='9' => {
                // Parse width or precision
                let mut num_str = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        num_str.push(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }
                spec.width = num_str.parse().unwrap_or(0);
            }
            '.' => {
                chars.next();
                if let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        let mut prec_str = String::new();
                        while let Some(&d) = chars.peek() {
                            if d.is_ascii_digit() {
                                prec_str.push(d);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        spec.precision = Some(prec_str.parse().unwrap_or(2));
                    }
                }
            }
            '-' => {
                chars.next();
                spec.left_align = true;
            }
            '+' => {
                chars.next();
                spec.show_sign = true;
            }
            ' ' => {
                chars.next();
                spec.space_pad = true;
            }
            '#' => {
                chars.next();
                spec.alternate_form = true;
            }
            _ => {
                chars.next();
                spec.conversion = ch;
                break;
            }
        }
    }
    
    Ok(spec)
}

/// Parse a placeholder like {0} or {name}
fn parse_placeholder(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<(), Error> {
    let mut placeholder = String::new();
    
    while let Some(&ch) = chars.peek() {
        if ch == '}' {
            chars.next();
            break;
        } else {
            placeholder.push(ch);
            chars.next();
        }
    }
    
    Ok(placeholder)
}

/// Format specifier for printf-style formatting
#[derive(Debug, Default)]
struct FormatSpecifier {
    conversion: char,
    width: usize,
    precision: Option<usize>,
    left_align: bool,
    show_sign: bool,
    space_pad: bool,
    alternate_form: bool,
}

/// Format an argument according to a format specifier
fn format_argument(arg: &CursedObject, spec: &FormatSpecifier) -> Result<(), Error> {
    let result = match spec.conversion {
        'c' => {
            match arg {
                CursedObject::Integer(n) => {
                    if let Some(ch) = char::from_u32(*n as u32) {
                        ch.to_string()
                    } else {
                        return Err(CursedError::TemplateError {
                            message: format!("Invalid character code: {}", n),
                            source_location: None,
                        });
                    }
                }
                CursedObject::Char(ch) => ch.to_string(),
                CursedObject::String(s) => {
                    if let Some(ch) = s.chars().next() {
                        ch.to_string()
                    } else {
                        return Err(CursedError::TemplateError {
                            message: "Cannot convert empty string to character".to_string(),
                            source_location: None,
                        });
                    }
                }
                _ => return Err(CursedError::TemplateError {
                    message: "Invalid argument type for %c format".to_string(),
                    source_location: None,
                }),
            }
        }
        's' => extract_string(arg)?,
        'd' | 'i' => {
            let num = extract_int(arg)?;
            if spec.show_sign && num >= 0 {
                format!("+{}", num)
            } else {
                num.to_string()
            }
        }
        'f' => {
            let num = extract_float(arg)?;
            let precision = spec.precision.unwrap_or(6);
            if spec.show_sign && num >= 0.0 {
                format!("+{:.prec$}", num, prec = precision)
            } else {
                format!("{:.prec$}", num, prec = precision)
            }
        }
        'e' => {
            let num = extract_float(arg)?;
            let precision = spec.precision.unwrap_or(6);
            if spec.show_sign && num >= 0.0 {
                format!("+{:.prec$e}", num, prec = precision)
            } else {
                format!("{:.prec$e}", num, prec = precision)
            }
        }
        'g' => {
            let num = extract_float(arg)?;
            let precision = spec.precision.unwrap_or(6);
            if spec.show_sign && num >= 0.0 {
                format!("+{:.prec$}", num, prec = precision)
            } else {
                format!("{:.prec$}", num, prec = precision)
            }
        }
        'x' => {
            let num = extract_int(arg)?;
            format!("{:x}", num)
        }
        'X' => {
            let num = extract_int(arg)?;
            format!("{:X}", num)
        }
        'o' => {
            let num = extract_int(arg)?;
            format!("{:o}", num)
        }
        _ => extract_string(arg)?,
    };
    
    // Apply width and alignment
    Ok(if spec.width > 0 && result.len() < spec.width {
        let padding = spec.width - result.len();
        if spec.left_align {
            format!("{}{}", result, " ".repeat(padding))
        } else {
            format!("{}{}", " ".repeat(padding), result)
        }
    } else {
        result
    })
}

impl std::fmt::Display for FormatSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FormatSpecifier {{ conversion: '{}', width: {}, precision: {:?}, left_align: {}, show_sign: {}, space_pad: {}, alternate_form: {} }}", 
               self.conversion, self.width, self.precision, self.left_align, self.show_sign, self.space_pad, self.alternate_form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_context() {
        let context = FilterContext::new();
        assert_eq!(context.chain_depth, 0);
        assert!(context.track_performance);
        assert!(context.cache_enabled);
        
        let deeper = context.deeper().unwrap();
        assert_eq!(deeper.chain_depth, 1);
        
        // Test max depth
        let mut deep_context = context;
        for _ in 0..10 {
            deep_context = deep_context.deeper().unwrap();
        }
        assert!(deep_context.deeper().is_err());
    }

    #[test]
    fn test_string_filters() {
        let registry = FilterRegistry::new();

        // Test lower filter
        let result = registry.apply("lower", &[CursedObject::String("HELLO".to_string())]).unwrap();
        assert_eq!(result, CursedObject::String("hello".to_string()));

        // Test upper filter
        let result = registry.apply("upper", &[CursedObject::String("hello".to_string())]).unwrap();
        assert_eq!(result, CursedObject::String("HELLO".to_string()));

        // Test title filter
        let result = registry.apply("title", &[CursedObject::String("hello world".to_string())]).unwrap();
        assert_eq!(result, CursedObject::String("Hello World".to_string()));
    }

    #[test]
    fn test_math_filters() {
        let registry = FilterRegistry::new();

        // Test add filter
        let result = registry.apply("add", &[
            CursedObject::Integer(5),
            CursedObject::Integer(3),
        ]).unwrap();
        assert_eq!(result, CursedObject::Float(8.0));

        // Test max filter
        let result = registry.apply("max", &[
            CursedObject::Float(5.5),
            CursedObject::Float(3.2),
        ]).unwrap();
        assert_eq!(result, CursedObject::Float(5.5));
    }

    #[test]
    fn test_collection_filters() {
        let registry = FilterRegistry::new();

        // Test len filter
        let arr = vec![
            CursedObject::String("a".to_string()),
            CursedObject::String("b".to_string()),
            CursedObject::String("c".to_string()),
        ];
        let result = registry.apply("len", &[CursedObject::Array(arr)]).unwrap();
        assert_eq!(result, CursedObject::Integer(3));

        // Test first filter
        let arr = vec![
            CursedObject::String("first".to_string()),
            CursedObject::String("second".to_string()),
        ];
        let result = registry.apply("first", &[CursedObject::Array(arr)]).unwrap();
        assert_eq!(result, CursedObject::String("first".to_string()));
    }

    #[test]
    fn test_conversion_filters() {
        let registry = FilterRegistry::new();

        // Test toString filter
        let result = registry.apply("toString", &[CursedObject::Integer(42)]).unwrap();
        assert_eq!(result, CursedObject::String("42".to_string()));

        // Test toInt filter
        let result = registry.apply("toInt", &[CursedObject::String("123".to_string())]).unwrap();
        assert_eq!(result, CursedObject::Integer(123));
    }

    #[test]
    fn test_html_filters() {
        let registry = FilterRegistry::new();

        // Test htmlEscape filter
        let result = registry.apply("htmlEscape", &[
            CursedObject::String("<script>alert('xss')</script>".to_string())
        ]).unwrap();
        
        if let CursedObject::String(escaped) = result {
            assert!(escaped.contains("&lt;script&gt;"));
        } else {
            panic!("Expected string result");
        }
    }

    #[test]
    fn test_cursed_filters() {
        let registry = FilterRegistry::new();

        // Test no_cap filter (trim alias)
        let result = registry.apply("no_cap", &[CursedObject::String("  hello  ".to_string())]).unwrap();
        assert_eq!(result, CursedObject::String("hello".to_string()));

        // Test vibes filter (format alias)
        let result = registry.apply("vibes", &[CursedObject::Float(3.14159), CursedObject::Integer(2)]).unwrap();
        assert_eq!(result, CursedObject::String("3.14".to_string()));

        // Test periodt filter
        let result = registry.apply("periodt", &[CursedObject::String("hello".to_string())]).unwrap();
        assert_eq!(result, CursedObject::String("hello.".to_string()));

        // Test flex filter
        let result = registry.apply("flex", &[CursedObject::String("strong".to_string())]).unwrap();
        assert_eq!(result, CursedObject::String("STRONG 💪".to_string()));

        // Test bussin filter
        let result = registry.apply("bussin", &[CursedObject::Float(9.0)]).unwrap();
        assert_eq!(result, CursedObject::String("bussin 😍".to_string()));

        let result = registry.apply("bussin", &[CursedObject::Float(5.0)]).unwrap();
        assert_eq!(result, CursedObject::String("nah chief".to_string()));
    }

    #[test]
    fn test_trait_filters() {
        let registry = FilterRegistry::new();
        
        // Register trait-based filters
        registry.register_trait_filter(CapitalizeFilter);
        registry.register_trait_filter(ReverseStringFilter);

        // Test capitalize filter
        let result = registry.apply("capitalize", &[CursedObject::String("hello world".to_string())]).unwrap();
        assert_eq!(result, CursedObject::String("Hello World".to_string()));

        // Test reverse_string filter
        let result = registry.apply("reverse_string", &[CursedObject::String("hello".to_string())]).unwrap();
        assert_eq!(result, CursedObject::String("olleh".to_string()));
    }

    #[test]
    fn test_filter_caching() {
        let registry = FilterRegistry::new();

        // Apply the same filter multiple times
        let args = vec![CursedObject::String("HELLO".to_string())];
        
        let result1 = registry.apply("lower", &args).unwrap();
        let result2 = registry.apply("lower", &args).unwrap();
        
        assert_eq!(result1, result2);
        assert_eq!(result1, CursedObject::String("hello".to_string()));

        // Check stats
        if let Some(stats) = registry.get_stats("lower") {
            assert!(stats.call_count >= 2);
        }
    }

    #[test]
    fn test_advanced_filters() {
        let registry = FilterRegistry::new();

        // Test sort filter
        let arr = vec![
            CursedObject::String("zebra".to_string()),
            CursedObject::String("apple".to_string()),
            CursedObject::String("banana".to_string()),
        ];
        let result = registry.apply("sort", &[CursedObject::Array(arr)]).unwrap();
        
        if let CursedObject::Array(sorted) = result {
            assert_eq!(sorted[0], CursedObject::String("apple".to_string()));
            assert_eq!(sorted[1], CursedObject::String("banana".to_string()));
            assert_eq!(sorted[2], CursedObject::String("zebra".to_string()));
        } else {
            panic!("Expected array result");
        }

        // Test truncate filter
        let result = registry.apply("truncate", &[
            CursedObject::String("This is a very long string".to_string()),
            CursedObject::Integer(10),
            CursedObject::String("...".to_string())
        ]).unwrap();
        assert_eq!(result, CursedObject::String("This is...".to_string()));

        // Test filter with condition
        let arr = vec![
            CursedObject::String("".to_string()),
            CursedObject::String("hello".to_string()),
            CursedObject::String("".to_string()),
            CursedObject::String("world".to_string()),
        ];
        let result = registry.apply("filter", &[
            CursedObject::Array(arr),
            CursedObject::String("non_empty".to_string())
        ]).unwrap();
        
        if let CursedObject::Array(filtered) = result {
            assert_eq!(filtered.len(), 2);
            assert_eq!(filtered[0], CursedObject::String("hello".to_string()));
            assert_eq!(filtered[1], CursedObject::String("world".to_string()));
        } else {
            panic!("Expected array result");
        }
    }

    #[test]
    fn test_date_filters() {
        let registry = FilterRegistry::new();

        // Test time_ago filter
        let now = chrono::Utc::now().timestamp();
        let one_hour_ago = now - 3600;
        let result = registry.apply("time_ago", &[CursedObject::Integer(one_hour_ago)]).unwrap();
        
        if let CursedObject::String(time_str) = result {
            assert!(time_str.contains("hour"));
        } else {
            panic!("Expected string result");
        }

        // Test date_format filter
        let timestamp = 1640995200; // 2022-01-01 00:00:00 UTC
        let result = registry.apply("date_format", &[
            CursedObject::Integer(timestamp),
            CursedObject::String("%Y-%m-%d".to_string())
        ]).unwrap();
        
        if let CursedObject::String(date_str) = result {
            assert_eq!(date_str, "2022-01-01");
        } else {
            panic!("Expected string result");
        }
    }

    #[test]
    fn test_error_handling() {
        let registry = FilterRegistry::new();

        // Test unknown filter
        let result = registry.apply("unknown_filter", &[CursedObject::String("test".to_string())]);
        assert!(result.is_err());

        // Test invalid arguments
        let result = registry.apply("add", &[CursedObject::String("not_a_number".to_string()), CursedObject::Integer(5)]);
        assert!(result.is_err());

        // Test division by zero
        let result = registry.apply("div", &[CursedObject::Float(10.0), CursedObject::Float(0.0)]);
        assert!(result.is_err());
    }

    #[test]
    fn test_printf_sprintf_filters() {
        let registry = FilterRegistry::new();

        // Test printf with string format
        let result = registry.apply("printf", &[
            CursedObject::String("Hello %s!".to_string()),
            CursedObject::String("World".to_string())
        ]).unwrap();
        assert_eq!(result, CursedObject::String("Hello World!".to_string()));

        // Test sprintf with integer format
        let result = registry.apply("sprintf", &[
            CursedObject::String("Number: %d".to_string()),
            CursedObject::Integer(42)
        ]).unwrap();
        assert_eq!(result, CursedObject::String("Number: 42".to_string()));

        // Test printf with float format
        let result = registry.apply("printf", &[
            CursedObject::String("Value: %.2f".to_string()),
            CursedObject::Float(3.14159)
        ]).unwrap();
        assert_eq!(result, CursedObject::String("Value: 3.14".to_string()));

        // Test printf with multiple arguments
        let result = registry.apply("printf", &[
            CursedObject::String("Name: %s, Age: %d, Height: %.1f".to_string()),
            CursedObject::String("Alice".to_string()),
            CursedObject::Integer(25),
            CursedObject::Float(5.6)
        ]).unwrap();
        assert_eq!(result, CursedObject::String("Name: Alice, Age: 25, Height: 5.6".to_string()));

        // Test placeholder-style formatting
        let result = registry.apply("printf", &[
            CursedObject::String("Hello {}! You are {} years old.".to_string()),
            CursedObject::String("Bob".to_string()),
            CursedObject::Integer(30)
        ]).unwrap();
        assert_eq!(result, CursedObject::String("Hello Bob! You are 30 years old.".to_string()));

        // Test indexed placeholders
        let result = registry.apply("sprintf", &[
            CursedObject::String("Second: {1}, First: {0}".to_string()),
            CursedObject::String("First".to_string()),
            CursedObject::String("Second".to_string())
        ]).unwrap();
        assert_eq!(result, CursedObject::String("Second: Second, First: First".to_string()));

        // Test hexadecimal format
        let result = registry.apply("printf", &[
            CursedObject::String("Hex: %x, HEX: %X".to_string()),
            CursedObject::Integer(255),
            CursedObject::Integer(255)
        ]).unwrap();
        assert_eq!(result, CursedObject::String("Hex: ff, HEX: FF".to_string()));

        // Test octal format
        let result = registry.apply("sprintf", &[
            CursedObject::String("Octal: %o".to_string()),
            CursedObject::Integer(64)
        ]).unwrap();
        assert_eq!(result, CursedObject::String("Octal: 100".to_string()));

        // Test character format
        let result = registry.apply("printf", &[
            CursedObject::String("Char: %c".to_string()),
            CursedObject::Integer(65)
        ]).unwrap();
        assert_eq!(result, CursedObject::String("Char: A".to_string()));

        // Test scientific notation
        let result = registry.apply("sprintf", &[
            CursedObject::String("Scientific: %.2e".to_string()),
            CursedObject::Float(1234.5)
        ]).unwrap();
        assert_eq!(result, CursedObject::String("Scientific: 1.23e3".to_string()));

        // Test escaped characters
        let result = registry.apply("printf", &[
            CursedObject::String("Escaped: %% and {{}}".to_string())
        ]).unwrap();
        assert_eq!(result, CursedObject::String("Escaped: % and {}".to_string()));
    }

    #[test]
    fn test_printf_error_cases() {
        let registry = FilterRegistry::new();

        // Test insufficient arguments
        let result = registry.apply("printf", &[
            CursedObject::String("Hello %s %s!".to_string()),
            CursedObject::String("World".to_string())
        ]);
        assert!(result.is_err());

        // Test no arguments
        let result = registry.apply("sprintf", &[]);
        assert!(result.is_err());

        // Test invalid character conversion
        let result = registry.apply("printf", &[
            CursedObject::String("Char: %c".to_string()),
            CursedObject::String("".to_string())
        ]);
        assert!(result.is_err());

        // Test out of range index
        let result = registry.apply("sprintf", &[
            CursedObject::String("Index: {5}".to_string()),
            CursedObject::String("only one arg".to_string())
        ]);
        assert!(result.is_err());
    }
}
