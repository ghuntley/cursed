use crate::error::CursedError;
/// Template Filters - Built-in functions and filter registry for CURSED templates
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, instrument, warn, info};
use base64::{Engine as _, engine::general_purpose};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::object::Object as CursedObject;

/// Filter function type
// pub type FilterFn = Box<dyn Fn(&FilterContext, &[CursedObject]) -> std::result::Result<CursedObject, crate::stdlib::template::TemplateError> + Send + Sync>;

/// Template filter trait for implementing custom filters
pub trait TemplateFilter: Send + Sync {
    /// Filter name
    fn name(&self) -> &str;
    
    /// Filter description
    /// Required number of arguments (None for variable args)
    /// Apply the filter
    fn apply(&self, context: &FilterContext, args: &[CursedObject]) -> crate::error::Result<()>;
    
    /// Whether this filter can be cached
    fn cacheable(&self) -> bool { true }
}

/// Filter execution context with metadata and performance tracking
#[derive(Debug, Clone)]
pub struct FilterContext {
    /// Current template being rendered
    /// Filter chain depth (for preventing infinite recursion)
    /// Performance tracking enabled
    /// Cache enabled
    /// Maximum chain depth
    /// Start time for performance tracking
impl FilterContext {
    /// Create new filter context
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Create context for template
    pub fn for_template(template_name: String) -> Self {
        Self {
            ..Self::new()
        }
    }
    
    /// Increment chain depth
    pub fn deeper(&self) -> crate::error::Result<()> {
        if self.chain_depth >= self.max_chain_depth {
            return Err(CursedError::TemplateError {
            });
        Ok(Self {
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
impl FilterStats {
    fn new() -> Self {
        Self {
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
/// Registry of template filters and functions
pub struct FilterRegistry {
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
        registry.register_builtin_filters();
        registry.register_cursed_filters();
        registry
    /// Register a custom filter function
    pub fn register<F>(&self, name: &str, filter: F)
    where
    {
        if let Ok(mut filters) = self.filters.write() {
            filters.insert(name.to_string(), Box::new(filter));
        }
    }
    
    /// Register a trait-based filter
    pub fn register_trait_filter<T>(&self, filter: T)
    where
    {
        let name = filter.name().to_string();
        if let Ok(mut trait_filters) = self.trait_filters.write() {
            trait_filters.insert(name, Box::new(filter));
        }
    }

    /// Apply a filter to values with context
    #[instrument(skip(self, args))]
    pub fn apply(&self, name: &str, args: &[CursedObject]) -> crate::error::Result<()> {
        let context = FilterContext::new();
        self.apply_with_context(name, &context, args)
    /// Apply a filter with explicit context
    #[instrument(skip(self, args))]
    pub fn apply_with_context(&self, name: &str, context: &FilterContext, args: &[CursedObject]) -> crate::error::Result<()> {
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
        })
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
        if let Ok(trait_filters) = self.trait_filters.read() {
            names.extend(trait_filters.keys().cloned());
        names.sort();
        names
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
            let strings: crate::error::Result<()> = arr.iter()
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
            }
        });

        self.register("lastIndex", |_context, args| {
            let s = extract_string(&args[0])?;
            let substr = extract_string(&args[1])?;
            match s.rfind(&substr) {
            }
        });

        // Formatting filters
        self.register("printf", |_context, args| {
            if args.is_empty() {
                return Err(CursedError::TemplateError {
                });
            let format = extract_string(&args[0])?;
            let format_args = &args[1..];
            
            format_string_cursed(&format, format_args)
        });

        self.register("sprintf", |_context, args| {
            if args.is_empty() {
                return Err(CursedError::TemplateError {
                });
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
            }
        });

        self.register("values", |_context, args| {
            match &args[0] {
                CursedObject::Map(map) => {
                    let values: Vec<CursedObject> = map.values().cloned().collect();
                    Ok(CursedObject::Array(values))
                }
                _ => Err(CursedError::TemplateError {
            }
        });

        // Data conversion filters
        self.register("toJSON", |_context, args| {
            // Simplified JSON serialization
            match &args[0] {
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
                    Err(_) => Err(CursedError::TemplateError {
                Err(_) => Err(CursedError::TemplateError {
            }
        });

        self.register("toBool", |_context, args| {
            let result = match &args[0] {
            Ok(CursedObject::Boolean(result))
        });

        self.register("toString", |_context, args| {
            let result = match &args[0] {
            Ok(CursedObject::String(result))
        });

        self.register("toInt", |_context, args| {
            match &args[0] {
                CursedObject::String(s) => {
                    match s.parse::<i64>() {
                        Err(_) => Err(CursedError::TemplateError {
                    }
                }
                _ => Err(CursedError::TemplateError {
            }
        });

        self.register("toFloat", |_context, args| {
            match &args[0] {
                CursedObject::String(s) => {
                    match s.parse::<f64>() {
                        Err(_) => Err(CursedError::TemplateError {
                    }
                }
                _ => Err(CursedError::TemplateError {
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
            Ok(CursedObject::Boolean(result))
        });

        self.register("isNil", |_context, args| {
            Ok(CursedObject::Boolean(matches!(args[0], CursedObject::Nil)))
        });

        self.register("isEmpty", |_context, args| {
            let result = match &args[0] {
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
                Err(_) => Err(CursedError::TemplateError {
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
                });
            }
            Ok(CursedObject::Float(a / b))
        });

        self.register("mod", |_context, args| {
            let a = extract_int(&args[0])?;
            let b = extract_int(&args[1])?;
            if b == 0 {
                return Err(CursedError::TemplateError {
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
            let strings: crate::error::Result<()> = arr.iter()
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
            
            Ok(CursedObject::String(result))
        });
        
        self.register("strftime", |_context, args| {
            let timestamp = extract_int(&args[0])? as i64;
            let format = extract_string(&args[1])?;
            
            if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
                Ok(CursedObject::String(dt.format(&format).to_string()))
            } else {
                Err(CursedError::TemplateError {
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
                        _ if !in_tag => {
                            acc.push(c);
                            (acc, in_tag)
                        }
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
                        CursedObject::String(field) if field == "non_empty" => match item {
                    }
                })
                .collect();
            
            Ok(CursedObject::Array(filtered))
        });
        
        self.register("map", |_context, args| {
            let arr = extract_array(&args[0])?;
            let operation = if args.len() > 1 { extract_string(&args[1])? } else { "toString".to_string() };
            
            let mapped: crate::error::Result<()> = arr.iter()
                .map(|item| {
                    match operation.as_str() {
                        "upper" => match item {
                        "lower" => match item {
                        "toString" => Ok(CursedObject::String(match item {
                    }
                })
                .collect();
            
            Ok(CursedObject::Array(mapped?))
        });
        
        self.register("length", |_context, args| {
            match &args[0] {
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
                _ => Err(CursedError::TemplateError {
            }
        });
        
        self.register("to_string", |_context, args| {
            let result = match &args[0] {
            Ok(CursedObject::String(result))
        });
    }
}

/// Example trait-based filters
pub struct CapitalizeFilter;

impl TemplateFilter for CapitalizeFilter {
    fn name(&self) -> &str { "capitalize" }
    fn description(&self) -> &str { "Capitalize the first letter of each word" }
    fn apply(&self, _context: &FilterContext, args: &[CursedObject]) -> crate::error::Result<()> {
        let s = extract_string(&args[0])?;
        let result = s.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
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
    fn apply(&self, _context: &FilterContext, args: &[CursedObject]) -> crate::error::Result<()> {
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
    
    fn apply(&self, context: &FilterContext, args: &[CursedObject]) -> crate::error::Result<()> {
        if args.len() < 2 {
            return Err(CursedError::TemplateError {
            });
        let mut result = args[0].clone();
        let context = context.deeper()?;
        
        // Apply each filter in sequence
        for filter_arg in &args[1..] {
            let filter_name = extract_string(filter_arg)?;
            
            // For now, we need access to the registry to apply nested filters
            // This would need to be passed through context in a real implementation
            warn!("Chain filter needs registry access - not fully implemented");
            return Ok(result);
        Ok(result)
    }
}

impl Default for FilterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to extract string from CursedObject
fn extract_string(obj: &CursedObject) -> crate::error::Result<()> {
    match obj {
        _ => Err(CursedError::TemplateError {
    }
}

/// Helper function to extract integer from CursedObject
fn extract_int(obj: &CursedObject) -> crate::error::Result<()> {
    match obj {
        CursedObject::String(s) => {
            s.parse::<i64>().map_err(|_| CursedError::TemplateError {
            })
        }
        _ => Err(CursedError::TemplateError {
    }
}

/// Helper function to extract float from CursedObject
fn extract_float(obj: &CursedObject) -> crate::error::Result<()> {
    match obj {
        CursedObject::String(s) => {
            s.parse::<f64>().map_err(|_| CursedError::TemplateError {
            })
        }
        _ => Err(CursedError::TemplateError {
    }
}

/// Helper function to extract array from CursedObject
fn extract_array(obj: &CursedObject) -> crate::error::Result<()> {
    match obj {
        _ => Err(CursedError::TemplateError {
    }
}

/// Helper function to check if objects are equal
fn objects_equal(left: &CursedObject, right: &CursedObject) -> bool {
    match (left, right) {
    }
}

/// Helper function to check if object is truthy
fn is_truthy(obj: &CursedObject) -> bool {
    match obj {
        CursedObject::Char(_) => true, // Characters are always truthy
    }
}

/// Advanced string formatting function with printf-style placeholders
fn format_string_cursed(format: &str, args: &[CursedObject]) -> crate::error::Result<()> {
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
                        });
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
                        });
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
                                });
                            }
                            extract_string(&args[index])?
                        }
                        Err(_) => {
                            // For named placeholders, just use current arg_index
                            if arg_index >= args.len() {
                                return Err(CursedError::TemplateError {
                                });
                            }
                            let formatted = extract_string(&args[arg_index])?;
                            arg_index += 1;
                            formatted
                        }
                    
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
/// Parse a printf-style format specifier
fn parse_format_specifier(chars: &mut std::iter::Peekable<std::str::Chars>) -> crate::error::Result<()> {
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
    Ok(spec)
/// Parse a placeholder like {0} or {name}
fn parse_placeholder(chars: &mut std::iter::Peekable<std::str::Chars>) -> crate::error::Result<()> {
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
/// Format specifier for printf-style formatting
#[derive(Debug, Default)]
struct FormatSpecifier {
/// Format an argument according to a format specifier
fn format_argument(arg: &CursedObject, spec: &FormatSpecifier) -> crate::error::Result<()> {
    let result = match spec.conversion {
        'c' => {
            match arg {
                CursedObject::Integer(n) => {
                    if let Some(ch) = char::from_u32(*n as u32) {
                        ch.to_string()
                    } else {
                        return Err(CursedError::TemplateError {
                        });
                    }
                }
                CursedObject::String(s) => {
                    if let Some(ch) = s.chars().next() {
                        ch.to_string()
                    } else {
                        return Err(CursedError::TemplateError {
                        });
                    }
                }
                _ => return Err(CursedError::TemplateError {
            }
        }
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
impl std::fmt::Display for FormatSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
               self.conversion, self.width, self.precision, self.left_align, self.show_sign, self.space_pad, self.alternate_form)
    }
}

