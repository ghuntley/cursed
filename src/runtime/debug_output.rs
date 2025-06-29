//! Debug output and logging system for CURSED runtime
//!
//! Provides comprehensive debugging capabilities including structured logging,
//! debug levels, output formatting, and runtime introspection.

use crate::error_types::{Error, Result as CursedResult};
use crate::runtime::stack_trace::{StackTrace, StackFrame};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Instant, SystemTime};
use std::io::{self, Write};

/// Debug output levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DebugLevel {
    /// Trace level - most verbose
    Trace = 0,
    /// Debug level - detailed information
    Debug = 1,
    /// Info level - general information
    Info = 2,
    /// Warning level - potential issues
    Warn = 3,
    /// Error level - errors that don't stop execution
    Error = 4,
    /// Fatal level - critical errors
    Fatal = 5,
}

impl fmt::Display for DebugLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DebugLevel::Trace => write!(f, "TRACE"),
            DebugLevel::Debug => write!(f, "DEBUG"),
            DebugLevel::Info => write!(f, "INFO"),
            DebugLevel::Warn => write!(f, "WARN"),
            DebugLevel::Error => write!(f, "ERROR"),
            DebugLevel::Fatal => write!(f, "FATAL"),
        }
    }
}

impl DebugLevel {
    /// Get ANSI color code for this level
    pub fn color_code(&self) -> &'static str {
        match self {
            DebugLevel::Trace => "\\x1b[37m",    // White
            DebugLevel::Debug => "\\x1b[36m",    // Cyan
            DebugLevel::Info => "\\x1b[32m",     // Green
            DebugLevel::Warn => "\\x1b[33m",     // Yellow
            DebugLevel::Error => "\\x1b[31m",    // Red
            DebugLevel::Fatal => "\\x1b[35m",    // Magenta
        }
    }

    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            DebugLevel::Trace => "🔍",
            DebugLevel::Debug => "🐛",
            DebugLevel::Info => "ℹ️",
            DebugLevel::Warn => "⚠️",
            DebugLevel::Error => "❌",
            DebugLevel::Fatal => "💀",
        }
    }
}

/// Debug message with metadata
#[derive(Debug, Clone)]
pub struct DebugMessage {
    /// Message ID for tracking
    pub id: u64,
    /// Debug level
    pub level: DebugLevel,
    /// Message content
    pub message: String,
    /// Module/component that generated the message
    pub module: String,
    /// Function name where message was generated
    pub function: Option<String>,
    /// File and line number
    pub location: Option<(String, u32)>,
    /// Timestamp when message was created
    pub timestamp: SystemTime,
    /// Additional structured data
    pub data: HashMap<String, DebugValue>,
    /// Tags for filtering
    pub tags: Vec<String>,
    /// Thread ID
    pub thread_id: Option<usize>,
    /// Goroutine ID (if applicable)
    pub goroutine_id: Option<usize>,
}

/// Debug value that can hold different types
#[derive(Debug, Clone)]
pub enum DebugValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<DebugValue>),
    Object(HashMap<String, DebugValue>),
    Null,
}

impl fmt::Display for DebugValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DebugValue::String(s) => write!(f, "\"{}\"", s),
            DebugValue::Integer(i) => write!(f, "{}", i),
            DebugValue::Float(fl) => write!(f, "{}", fl),
            DebugValue::Boolean(b) => write!(f, "{}", b),
            DebugValue::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            DebugValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "\"{}\": {}", key, value)?;
                }
                write!(f, "}}")
            }
            DebugValue::Null => write!(f, "null"),
        }
    }
}

/// Debug output configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    /// Minimum debug level to output
    pub min_level: DebugLevel,
    /// Whether to show timestamps
    pub show_timestamps: bool,
    /// Whether to show source locations
    pub show_locations: bool,
    /// Whether to use colors
    pub use_colors: bool,
    /// Whether to show thread IDs
    pub show_thread_ids: bool,
    /// Maximum message length before truncation
    pub max_message_length: Option<usize>,
    /// Output format
    pub format: DebugFormat,
    /// Modules to include (empty = all)
    pub included_modules: Vec<String>,
    /// Modules to exclude
    pub excluded_modules: Vec<String>,
    /// Tags to include (empty = all)
    pub included_tags: Vec<String>,
}

/// Debug output format options
#[derive(Debug, Clone, PartialEq)]
pub enum DebugFormat {
    /// Simple text format
    Simple,
    /// Detailed format with all metadata
    Detailed,
    /// JSON format for structured logging
    Json,
    /// Custom format with template
    Custom(String),
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            min_level: DebugLevel::Info,
            show_timestamps: true,
            show_locations: false,
            use_colors: true,
            show_thread_ids: false,
            max_message_length: Some(1000),
            format: DebugFormat::Simple,
            included_modules: Vec::new(),
            excluded_modules: Vec::new(),
            included_tags: Vec::new(),
        }
    }
}

/// Debug output writer trait
pub trait DebugWriter: Send + Sync {
    fn write_message(&mut self, message: &DebugMessage, config: &DebugConfig) -> CursedResult<()>;
    fn flush(&mut self) -> CursedResult<()>;
}

/// Console debug writer
pub struct ConsoleDebugWriter {
    stdout: Box<dyn Write + Send + Sync>,
    stderr: Box<dyn Write + Send + Sync>,
}

impl ConsoleDebugWriter {
    pub fn new() -> Self {
        Self {
            stdout: Box::new(io::stdout()),
            stderr: Box::new(io::stderr()),
        }
    }
}

impl Default for ConsoleDebugWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugWriter for ConsoleDebugWriter {
    fn write_message(&mut self, message: &DebugMessage, config: &DebugConfig) -> CursedResult<()> {
        let formatted = format_debug_message(message, config);
        
        let writer: &mut dyn Write = if message.level >= DebugLevel::Error {
            &mut *self.stderr
        } else {
            &mut *self.stdout
        };
        
        writeln!(writer, "{}", formatted)
            .map_err(|e| Error::Io(format!("Failed to write debug message: {}", e)))?;
        
        Ok(())
    }

    fn flush(&mut self) -> CursedResult<()> {
        self.stdout.flush()
            .map_err(|e| Error::Io(format!("Failed to flush stdout: {}", e)))?;
        self.stderr.flush()
            .map_err(|e| Error::Io(format!("Failed to flush stderr: {}", e)))?;
        Ok(())
    }
}

/// File debug writer
pub struct FileDebugWriter {
    file: Mutex<std::fs::File>,
    path: String,
}

impl FileDebugWriter {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> CursedResult<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| Error::Io(format!("Failed to open debug log file: {}", e)))?;
        
        Ok(Self {
            file: Mutex::new(file),
            path: path_str,
        })
    }
}

impl DebugWriter for FileDebugWriter {
    fn write_message(&mut self, message: &DebugMessage, config: &DebugConfig) -> CursedResult<()> {
        let formatted = format_debug_message(message, config);
        
        let mut file = self.file.lock()
            .map_err(|_| Error::Runtime("Failed to acquire file lock".to_string()))?;
        
        writeln!(file, "{}", formatted)
            .map_err(|e| Error::Io(format!("Failed to write to debug log: {}", e)))?;
        
        Ok(())
    }

    fn flush(&mut self) -> CursedResult<()> {
        let mut file = self.file.lock()
            .map_err(|_| Error::Runtime("Failed to acquire file lock".to_string()))?;
        
        file.flush()
            .map_err(|e| Error::Io(format!("Failed to flush debug log: {}", e)))?;
        
        Ok(())
    }
}

/// Debug output system
pub struct DebugOutputSystem {
    /// Configuration
    config: RwLock<DebugConfig>,
    /// Debug writers
    writers: RwLock<Vec<Box<dyn DebugWriter>>>,
    /// Message counter for IDs
    message_counter: Mutex<u64>,
    /// Message buffer for recent messages
    message_buffer: RwLock<Vec<DebugMessage>>,
    /// Maximum buffer size
    max_buffer_size: usize,
    /// Statistics
    stats: Mutex<DebugStats>,
}

/// Debug statistics
#[derive(Debug, Default, Clone)]
pub struct DebugStats {
    pub messages_generated: u64,
    pub messages_filtered: u64,
    pub messages_written: u64,
    pub write_errors: u64,
    pub by_level: HashMap<DebugLevel, u64>,
    pub by_module: HashMap<String, u64>,
}

impl DebugOutputSystem {
    /// Create a new debug output system
    pub fn new() -> Self {
        let mut system = Self {
            config: RwLock::new(DebugConfig::default()),
            writers: RwLock::new(Vec::new()),
            message_counter: Mutex::new(0),
            message_buffer: RwLock::new(Vec::new()),
            max_buffer_size: 1000,
            stats: Mutex::new(DebugStats::default()),
        };
        
        // Add default console writer
        system.add_writer(Box::new(ConsoleDebugWriter::new()));
        system
    }

    /// Set debug configuration
    pub fn set_config(&self, config: DebugConfig) {
        let mut current_config = self.config.write().unwrap();
        *current_config = config;
    }

    /// Get current configuration
    pub fn get_config(&self) -> DebugConfig {
        self.config.read().unwrap().clone()
    }

    /// Add a debug writer
    pub fn add_writer(&mut self, writer: Box<dyn DebugWriter>) {
        let mut writers = self.writers.write().unwrap();
        writers.push(writer);
    }

    /// Log a debug message
    pub fn log(&self, level: DebugLevel, module: &str, message: &str) -> CursedResult<()> {
        self.log_with_data(level, module, message, HashMap::new(), Vec::new())
    }

    /// Log a debug message with structured data
    pub fn log_with_data(
        &self,
        level: DebugLevel,
        module: &str,
        message: &str,
        data: HashMap<String, DebugValue>,
        tags: Vec<String>,
    ) -> CursedResult<()> {
        let config = self.config.read().unwrap().clone();
        
        // Check if message should be filtered
        if !self.should_log(&config, level, module, &tags) {
            let mut stats = self.stats.lock().unwrap();
            stats.messages_filtered += 1;
            return Ok(());
        }

        // Create debug message
        let message_id = {
            let mut counter = self.message_counter.lock().unwrap();
            *counter += 1;
            *counter
        };

        let debug_message = DebugMessage {
            id: message_id,
            level,
            message: message.to_string(),
            module: module.to_string(),
            function: None, // TODO: Capture from stack trace
            location: None, // TODO: Capture from caller
            timestamp: SystemTime::now(),
            data,
            tags,
            thread_id: Some(get_current_thread_id()),
            goroutine_id: None, // TODO: Get from goroutine context
        };

        // Add to buffer
        {
            let mut buffer = self.message_buffer.write().unwrap();
            if buffer.len() >= self.max_buffer_size {
                buffer.remove(0); // Remove oldest
            }
            buffer.push(debug_message.clone());
        }

        // Write to all writers
        let mut writers = self.writers.write().unwrap();
        for writer in writers.iter_mut() {
            if let Err(e) = writer.write_message(&debug_message, &config) {
                let mut stats = self.stats.lock().unwrap();
                stats.write_errors += 1;
                eprintln!("Debug writer error: {}", e);
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.messages_generated += 1;
            stats.messages_written += 1;
            *stats.by_level.entry(level).or_insert(0) += 1;
            *stats.by_module.entry(module.to_string()).or_insert(0) += 1;
        }

        Ok(())
    }

    /// Check if a message should be logged
    fn should_log(&self, config: &DebugConfig, level: DebugLevel, module: &str, tags: &[String]) -> bool {
        // Check level
        if level < config.min_level {
            return false;
        }

        // Check included modules
        if !config.included_modules.is_empty() {
            if !config.included_modules.iter().any(|m| module.contains(m)) {
                return false;
            }
        }

        // Check excluded modules
        if config.excluded_modules.iter().any(|m| module.contains(m)) {
            return false;
        }

        // Check included tags
        if !config.included_tags.is_empty() {
            if !config.included_tags.iter().any(|t| tags.contains(t)) {
                return false;
            }
        }

        true
    }

    /// Get recent messages
    pub fn get_recent_messages(&self, count: usize) -> Vec<DebugMessage> {
        let buffer = self.message_buffer.read().unwrap();
        let start = buffer.len().saturating_sub(count);
        buffer[start..].to_vec()
    }

    /// Get messages by level
    pub fn get_messages_by_level(&self, level: DebugLevel) -> Vec<DebugMessage> {
        let buffer = self.message_buffer.read().unwrap();
        buffer.iter()
            .filter(|msg| msg.level == level)
            .cloned()
            .collect()
    }

    /// Get messages by module
    pub fn get_messages_by_module(&self, module: &str) -> Vec<DebugMessage> {
        let buffer = self.message_buffer.read().unwrap();
        buffer.iter()
            .filter(|msg| msg.module.contains(module))
            .cloned()
            .collect()
    }

    /// Clear message buffer
    pub fn clear_buffer(&self) {
        let mut buffer = self.message_buffer.write().unwrap();
        buffer.clear();
    }

    /// Get debug statistics
    pub fn get_stats(&self) -> DebugStats {
        self.stats.lock().unwrap().clone()
    }

    /// Flush all writers
    pub fn flush(&self) -> CursedResult<()> {
        let mut writers = self.writers.write().unwrap();
        for writer in writers.iter_mut() {
            writer.flush()?;
        }
        Ok(())
    }
}

impl Default for DebugOutputSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Format a debug message according to configuration
pub fn format_debug_message(message: &DebugMessage, config: &DebugConfig) -> String {
    match &config.format {
        DebugFormat::Simple => format_simple(message, config),
        DebugFormat::Detailed => format_detailed(message, config),
        DebugFormat::Json => format_json(message, config),
        DebugFormat::Custom(template) => format_custom(message, config, template),
    }
}

/// Format message in simple format
fn format_simple(message: &DebugMessage, config: &DebugConfig) -> String {
    let mut parts = Vec::new();
    
    // Timestamp
    if config.show_timestamps {
        if let Ok(elapsed) = message.timestamp.elapsed() {
            parts.push(format!("[{:.3}s ago]", elapsed.as_secs_f64()));
        }
    }
    
    // Level with color
    let level_str = if config.use_colors {
        format!("{}{}\\x1b[0m", message.level.color_code(), message.level)
    } else {
        format!("{}", message.level)
    };
    parts.push(format!("[{}]", level_str));
    
    // Module
    parts.push(format!("[{}]", message.module));
    
    // Thread ID
    if config.show_thread_ids {
        if let Some(thread_id) = message.thread_id {
            parts.push(format!("[T{}]", thread_id));
        }
    }
    
    // Message
    let msg = if let Some(max_len) = config.max_message_length {
        if message.message.len() > max_len {
            format!("{}...", &message.message[..max_len.saturating_sub(3)])
        } else {
            message.message.clone()
        }
    } else {
        message.message.clone()
    };
    
    format!("{} {}", parts.join(" "), msg)
}

/// Format message in detailed format
fn format_detailed(message: &DebugMessage, config: &DebugConfig) -> String {
    let mut output = format_simple(message, config);
    
    // Add location if available and enabled
    if config.show_locations {
        if let Some((ref file, line)) = message.location {
            output.push_str(&format!(" [{}:{}]", file, line));
        }
    }
    
    // Add structured data
    if !message.data.is_empty() {
        output.push_str(" Data: {");
        for (i, (key, value)) in message.data.iter().enumerate() {
            if i > 0 { output.push_str(", "); }
            output.push_str(&format!("{}: {}", key, value));
        }
        output.push('}');
    }
    
    // Add tags
    if !message.tags.is_empty() {
        output.push_str(&format!(" Tags: [{}]", message.tags.join(", ")));
    }
    
    output
}

/// Format message in JSON format
fn format_json(message: &DebugMessage, _config: &DebugConfig) -> String {
    // This is a simplified JSON serialization
    // In a real implementation, you'd use serde or similar
    format!(
        r#"{{"id":{},"level":"{}","message":"{}","module":"{}","timestamp":"{}","data":{},"tags":{}}}"#,
        message.id,
        message.level,
        message.message.replace('"', "\\\""),
        message.module,
        message.timestamp.duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default().as_secs(),
        format_debug_data_as_json(&message.data),
        format_tags_as_json(&message.tags)
    )
}

/// Format custom template
fn format_custom(message: &DebugMessage, config: &DebugConfig, template: &str) -> String {
    // Simple template substitution
    let mut result = template.to_string();
    result = result.replace("{level}", &format!("{}", message.level));
    result = result.replace("{message}", &message.message);
    result = result.replace("{module}", &message.module);
    result = result.replace("{id}", &format!("{}", message.id));
    
    if let Some(thread_id) = message.thread_id {
        result = result.replace("{thread_id}", &format!("{}", thread_id));
    }
    
    result
}

/// Helper function to format debug data as JSON
fn format_debug_data_as_json(data: &HashMap<String, DebugValue>) -> String {
    if data.is_empty() {
        return "{}".to_string();
    }
    
    let mut items = Vec::new();
    for (key, value) in data {
        items.push(format!("\"{}\":{}", key, debug_value_to_json(value)));
    }
    format!("{{{}}}", items.join(","))
}

/// Helper function to format tags as JSON
fn format_tags_as_json(tags: &[String]) -> String {
    if tags.is_empty() {
        return "[]".to_string();
    }
    
    let quoted_tags: Vec<String> = tags.iter().map(|t| format!("\"{}\"", t)).collect();
    format!("[{}]", quoted_tags.join(","))
}

/// Convert DebugValue to JSON string
fn debug_value_to_json(value: &DebugValue) -> String {
    match value {
        DebugValue::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        DebugValue::Integer(i) => format!("{}", i),
        DebugValue::Float(f) => format!("{}", f),
        DebugValue::Boolean(b) => format!("{}", b),
        DebugValue::Null => "null".to_string(),
        DebugValue::Array(arr) => {
            let items: Vec<String> = arr.iter().map(debug_value_to_json).collect();
            format!("[{}]", items.join(","))
        }
        DebugValue::Object(obj) => {
            let items: Vec<String> = obj.iter()
                .map(|(k, v)| format!("\"{}\":{}", k, debug_value_to_json(v)))
                .collect();
            format!("{{{}}}", items.join(","))
        }
    }
}

/// Get current thread ID (simplified implementation)
fn get_current_thread_id() -> usize {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    
    let mut hasher = DefaultHasher::new();
    std::thread::current().id().hash(&mut hasher);
    hasher.finish() as usize
}

/// Global debug output system
static GLOBAL_DEBUG_OUTPUT_SYSTEM: std::sync::LazyLock<Mutex<DebugOutputSystem>> = 
    std::sync::LazyLock::new(|| Mutex::new(DebugOutputSystem::new()));

/// Get the global debug output system
pub fn get_global_debug_output_system() -> &'static Mutex<DebugOutputSystem> {
    &GLOBAL_DEBUG_OUTPUT_SYSTEM
}

/// Convenience macros for logging
#[macro_export]
macro_rules! debug_trace {
    ($module:expr, $($arg:tt)*) => {
        if let Ok(system) = $crate::runtime::debug_output::get_global_debug_output_system().lock() {
            let _ = system.log($crate::runtime::debug_output::DebugLevel::Trace, $module, &format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! debug_info {
    ($module:expr, $($arg:tt)*) => {
        if let Ok(system) = $crate::runtime::debug_output::get_global_debug_output_system().lock() {
            let _ = system.log($crate::runtime::debug_output::DebugLevel::Info, $module, &format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! debug_warn {
    ($module:expr, $($arg:tt)*) => {
        if let Ok(system) = $crate::runtime::debug_output::get_global_debug_output_system().lock() {
            let _ = system.log($crate::runtime::debug_output::DebugLevel::Warn, $module, &format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! debug_error {
    ($module:expr, $($arg:tt)*) => {
        if let Ok(system) = $crate::runtime::debug_output::get_global_debug_output_system().lock() {
            let _ = system.log($crate::runtime::debug_output::DebugLevel::Error, $module, &format!($($arg)*));
        }
    };
}

/// Utility functions for debug output
pub mod utils {
    use super::*;

    /// Create debug value from various types
    pub fn debug_value_from_string(s: &str) -> DebugValue {
        DebugValue::String(s.to_string())
    }

    pub fn debug_value_from_int(i: i64) -> DebugValue {
        DebugValue::Integer(i)
    }

    pub fn debug_value_from_float(f: f64) -> DebugValue {
        DebugValue::Float(f)
    }

    pub fn debug_value_from_bool(b: bool) -> DebugValue {
        DebugValue::Boolean(b)
    }

    /// Create structured debug data
    pub fn create_debug_data() -> HashMap<String, DebugValue> {
        HashMap::new()
    }

    /// Add data to debug data map
    pub fn add_debug_data(
        data: &mut HashMap<String, DebugValue>,
        key: &str,
        value: DebugValue,
    ) {
        data.insert(key.to_string(), value);
    }

    /// Log an error with stack trace
    pub fn log_error_with_trace(module: &str, error: &Error, trace: Option<&StackTrace>) {
        let mut data = create_debug_data();
        add_debug_data(&mut data, "error_type", debug_value_from_string(&format!("{:?}", error)));
        
        if let Some(trace) = trace {
            add_debug_data(&mut data, "stack_depth", debug_value_from_int(trace.depth() as i64));
            if let Some(top_frame) = trace.top_frame() {
                add_debug_data(&mut data, "top_function", debug_value_from_string(&top_frame.function_name));
            }
        }

        if let Ok(system) = get_global_debug_output_system().lock() {
            let _ = system.log_with_data(
                DebugLevel::Error,
                module,
                &format!("{}", error),
                data,
                vec!["error".to_string()],
            );
        }
    }

    /// Configure debug output for development
    pub fn setup_development_debug() -> CursedResult<()> {
        let config = DebugConfig {
            min_level: DebugLevel::Debug,
            show_timestamps: true,
            show_locations: true,
            use_colors: true,
            show_thread_ids: true,
            max_message_length: None,
            format: DebugFormat::Detailed,
            included_modules: Vec::new(),
            excluded_modules: Vec::new(),
            included_tags: Vec::new(),
        };

        if let Ok(system) = get_global_debug_output_system().lock() {
            system.set_config(config);
        }

        Ok(())
    }

    /// Configure debug output for production
    pub fn setup_production_debug() -> CursedResult<()> {
        let config = DebugConfig {
            min_level: DebugLevel::Warn,
            show_timestamps: true,
            show_locations: false,
            use_colors: false,
            show_thread_ids: false,
            max_message_length: Some(500),
            format: DebugFormat::Json,
            included_modules: Vec::new(),
            excluded_modules: Vec::new(),
            included_tags: Vec::new(),
        };

        if let Ok(system) = get_global_debug_output_system().lock() {
            system.set_config(config);
        }

        Ok(())
    }
}

/// Public function that was likely being used by external code
pub fn get_minimal_result() -> CursedResult<String> {
    Ok("CURSED debug output system initialized".to_string())
}
