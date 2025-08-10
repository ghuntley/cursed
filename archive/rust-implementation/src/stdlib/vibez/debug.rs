//! Debug module for vibez - Debug output and inspection utilities for CURSED

use crate::error_types::CursedError;
use crate::runtime::value::Value;
use std::sync::{RwLock, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, Write};

/// Debug levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugLevel {
    Off = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl From<u8> for DebugLevel {
    fn from(level: u8) -> Self {
        match level {
            0 => DebugLevel::Off,
            1 => DebugLevel::Error,
            2 => DebugLevel::Warn,
            3 => DebugLevel::Info,
            4 => DebugLevel::Debug,
            5 => DebugLevel::Trace,
            _ => DebugLevel::Trace,
        }
    }
}

impl From<DebugLevel> for u8 {
    fn from(level: DebugLevel) -> Self {
        level as u8
    }
}

/// Global debug system state
struct DebugSystem {
    level: DebugLevel,
    enabled: bool,
    output_file: Option<String>,
    timestamps: bool,
    thread_info: bool,
    color_output: bool,
    filters: Vec<String>,
    stats: DebugStats,
}

/// Debug statistics
#[derive(Debug, Clone, Default)]
pub struct DebugStats {
    pub messages_logged: u64,
    pub errors_logged: u64,
    pub warnings_logged: u64,
    pub info_logged: u64,
    pub debug_logged: u64,
    pub trace_logged: u64,
}

/// Global debug system instance
static DEBUG_SYSTEM: RwLock<DebugSystem> = RwLock::new(DebugSystem {
    level: DebugLevel::Info,
    enabled: true,
    output_file: None,
    timestamps: true,
    thread_info: false,
    color_output: true,
    filters: Vec::new(),
    stats: DebugStats {
        messages_logged: 0,
        errors_logged: 0,
        warnings_logged: 0,
        info_logged: 0,
        debug_logged: 0,
        trace_logged: 0,
    },
});

/// Set the global debug level
pub fn set_debug_level(level: u8) -> Result<(), CursedError> {
    let mut system = DEBUG_SYSTEM.write().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    system.level = DebugLevel::from(level);
    Ok(())
}

/// Get the current debug level
pub fn get_debug_level() -> u8 {
    match DEBUG_SYSTEM.read() {
        Ok(system) => system.level.into(),
        Err(_) => DebugLevel::Info.into(),
    }
}

/// Check if debug is enabled at current level
pub fn is_debug_enabled() -> bool {
    match DEBUG_SYSTEM.read() {
        Ok(system) => system.enabled && system.level >= DebugLevel::Debug,
        Err(_) => false,
    }
}

/// Check if debug is enabled at specific level
pub fn is_level_enabled(level: DebugLevel) -> bool {
    match DEBUG_SYSTEM.read() {
        Ok(system) => system.enabled && system.level >= level,
        Err(_) => false,
    }
}

/// Initialize the debug system
pub fn init_debug_system() {
    // Try to read environment variables for configuration
    if let Ok(level_str) = std::env::var("CURSED_DEBUG_LEVEL") {
        if let Ok(level) = level_str.parse::<u8>() {
            let _ = set_debug_level(level);
        }
    }
    
    if let Ok(enabled_str) = std::env::var("CURSED_DEBUG") {
        let enabled = enabled_str.eq_ignore_ascii_case("true") || enabled_str == "1";
        let _ = set_debug_enabled(enabled);
    }
    
    if let Ok(file_path) = std::env::var("CURSED_DEBUG_FILE") {
        let _ = set_debug_output_file(Some(file_path));
    }
}

/// Enable or disable debug output
pub fn set_debug_enabled(enabled: bool) -> Result<(), CursedError> {
    let mut system = DEBUG_SYSTEM.write().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    system.enabled = enabled;
    Ok(())
}

/// Set debug output file
pub fn set_debug_output_file(file_path: Option<String>) -> Result<(), CursedError> {
    let mut system = DEBUG_SYSTEM.write().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    system.output_file = file_path;
    Ok(())
}

/// Enable or disable timestamps in debug output
pub fn set_timestamps_enabled(enabled: bool) -> Result<(), CursedError> {
    let mut system = DEBUG_SYSTEM.write().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    system.timestamps = enabled;
    Ok(())
}

/// Enable or disable thread info in debug output
pub fn set_thread_info_enabled(enabled: bool) -> Result<(), CursedError> {
    let mut system = DEBUG_SYSTEM.write().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    system.thread_info = enabled;
    Ok(())
}

/// Enable or disable color output
pub fn set_color_output_enabled(enabled: bool) -> Result<(), CursedError> {
    let mut system = DEBUG_SYSTEM.write().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    system.color_output = enabled;
    Ok(())
}

/// Add a filter for debug messages
pub fn add_debug_filter(filter: String) -> Result<(), CursedError> {
    let mut system = DEBUG_SYSTEM.write().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    system.filters.push(filter);
    Ok(())
}

/// Clear all debug filters
pub fn clear_debug_filters() -> Result<(), CursedError> {
    let mut system = DEBUG_SYSTEM.write().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    system.filters.clear();
    Ok(())
}

/// Get debug statistics
pub fn get_debug_stats() -> Result<DebugStats, CursedError> {
    let system = DEBUG_SYSTEM.read().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    Ok(system.stats.clone())
}

/// Log a debug message at specified level
pub fn debug_log(level: DebugLevel, message: &str, module: Option<&str>) -> Result<(), CursedError> {
    let mut system = DEBUG_SYSTEM.write().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    if !system.enabled || system.level < level {
        return Ok(());
    }
    
    // Apply filters
    if !system.filters.is_empty() {
        let module_name = module.unwrap_or("");
        let should_log = system.filters.iter().any(|filter| {
            message.contains(filter) || module_name.contains(filter)
        });
        if !should_log {
            return Ok(());
        }
    }
    
    // Update statistics
    system.stats.messages_logged += 1;
    match level {
        DebugLevel::Error => system.stats.errors_logged += 1,
        DebugLevel::Warn => system.stats.warnings_logged += 1,
        DebugLevel::Info => system.stats.info_logged += 1,
        DebugLevel::Debug => system.stats.debug_logged += 1,
        DebugLevel::Trace => system.stats.trace_logged += 1,
        DebugLevel::Off => {}
    }
    
    // Format message
    let formatted = format_debug_message(level, message, module, &system)?;
    
    // Output message
    if let Some(ref file_path) = system.output_file {
        // Write to file
        use std::fs::OpenOptions;
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path) {
            let _ = writeln!(file, "{}", formatted);
        }
    } else {
        // Write to stderr
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        let _ = writeln!(handle, "{}", formatted);
    }
    
    Ok(())
}

/// Format a debug message
fn format_debug_message(
    level: DebugLevel,
    message: &str,
    module: Option<&str>,
    system: &DebugSystem
) -> Result<String, CursedError> {
    let mut formatted = String::new();
    
    // Add timestamp
    if system.timestamps {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| CursedError::Runtime("Failed to get timestamp".to_string()))?;
        formatted.push_str(&format!("[{}.{:03}] ", 
            timestamp.as_secs(), 
            timestamp.subsec_millis()));
    }
    
    // Add level with color
    let level_str = if system.color_output {
        match level {
            DebugLevel::Error => "\x1b[31mERROR\x1b[0m",
            DebugLevel::Warn => "\x1b[33mWARN\x1b[0m",
            DebugLevel::Info => "\x1b[32mINFO\x1b[0m",
            DebugLevel::Debug => "\x1b[36mDEBUG\x1b[0m",
            DebugLevel::Trace => "\x1b[35mTRACE\x1b[0m",
            DebugLevel::Off => "OFF",
        }
    } else {
        match level {
            DebugLevel::Error => "ERROR",
            DebugLevel::Warn => "WARN",
            DebugLevel::Info => "INFO",
            DebugLevel::Debug => "DEBUG",
            DebugLevel::Trace => "TRACE",
            DebugLevel::Off => "OFF",
        }
    };
    formatted.push_str(&format!("[{}] ", level_str));
    
    // Add thread info
    if system.thread_info {
        formatted.push_str(&format!("[{:?}] ", std::thread::current().id()));
    }
    
    // Add module
    if let Some(module) = module {
        formatted.push_str(&format!("[{}] ", module));
    }
    
    // Add message
    formatted.push_str(message);
    
    Ok(formatted)
}

/// Convenience functions for different log levels
pub fn debug_error(message: &str, module: Option<&str>) -> Result<(), CursedError> {
    debug_log(DebugLevel::Error, message, module)
}

pub fn debug_warn(message: &str, module: Option<&str>) -> Result<(), CursedError> {
    debug_log(DebugLevel::Warn, message, module)
}

pub fn debug_info(message: &str, module: Option<&str>) -> Result<(), CursedError> {
    debug_log(DebugLevel::Info, message, module)
}

pub fn debug_debug(message: &str, module: Option<&str>) -> Result<(), CursedError> {
    debug_log(DebugLevel::Debug, message, module)
}

pub fn debug_trace(message: &str, module: Option<&str>) -> Result<(), CursedError> {
    debug_log(DebugLevel::Trace, message, module)
}

/// Debug inspect a value
pub fn debug_inspect(value: &Value, label: Option<&str>) -> Result<(), CursedError> {
    if !is_level_enabled(DebugLevel::Debug) {
        return Ok(());
    }
    
    let label_str = label.unwrap_or("Value");
    let formatted = format_value_for_debug(value, 0)?;
    
    debug_debug(&format!("{}: {}", label_str, formatted), Some("inspect"))
}

/// Format a value for debug output with indentation
fn format_value_for_debug(value: &Value, indent: usize) -> Result<String, CursedError> {
    let indent_str = "  ".repeat(indent);
    
    match value {
        Value::Null => Ok("null".to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Integer(i) => Ok(i.to_string()),
        Value::Number(f) => Ok(f.to_string()),
        Value::String(s) => Ok(format!("\"{}\"", s.replace('\"', "\\\""))),
        Value::Array(arr) => {
            if arr.is_empty() {
                Ok("[]".to_string())
            } else {
                let mut result = "[\n".to_string();
                for (i, item) in arr.iter().enumerate() {
                    result.push_str(&format!("{}  {}", indent_str, format_value_for_debug(item, indent + 1)?));
                    if i < arr.len() - 1 {
                        result.push(',');
                    }
                    result.push('\n');
                }
                result.push_str(&format!("{}]", indent_str));
                Ok(result)
            }
        },
        Value::Object(obj) => {
            if obj.is_empty() {
                Ok("{}".to_string())
            } else {
                let mut result = "{\n".to_string();
                let mut entries: Vec<_> = obj.iter().collect();
                entries.sort_by_key(|(k, _)| *k);
                
                for (i, (key, val)) in entries.iter().enumerate() {
                    result.push_str(&format!("{}  \"{}\": {}", 
                        indent_str, 
                        key.replace('\"', "\\\""),
                        format_value_for_debug(val, indent + 1)?));
                    if i < entries.len() - 1 {
                        result.push(',');
                    }
                    result.push('\n');
                }
                result.push_str(&format!("{}}}", indent_str));
                Ok(result)
            }
        },
        Value::Binary(data) => Ok(format!("<Binary: {} bytes>", data.len())),
        Value::Function { name, arity } => Ok(format!("<Function: {}({} args)>", name, arity)),
        Value::Interface { .. } => Ok("<Interface>".to_string()),
    }
}

/// Reset debug statistics
pub fn reset_debug_stats() -> Result<(), CursedError> {
    let mut system = DEBUG_SYSTEM.write().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    system.stats = DebugStats::default();
    Ok(())
}

/// Print debug configuration
pub fn debug_config() -> Result<(), CursedError> {
    let system = DEBUG_SYSTEM.read().map_err(|_| {
        CursedError::Runtime("Failed to acquire debug system lock".to_string())
    })?;
    
    println!("Debug Configuration:");
    println!("  Level: {:?} ({})", system.level, u8::from(system.level));
    println!("  Enabled: {}", system.enabled);
    println!("  Output file: {:?}", system.output_file);
    println!("  Timestamps: {}", system.timestamps);
    println!("  Thread info: {}", system.thread_info);
    println!("  Color output: {}", system.color_output);
    println!("  Filters: {:?}", system.filters);
    println!("  Statistics: {:?}", system.stats);
    
    Ok(())
}
