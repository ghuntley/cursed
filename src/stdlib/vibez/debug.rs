/// Debug printing and formatting utilities for CURSED development
/// 
/// Provides comprehensive debug output functionality including pretty printing,
/// debug inspection, trace output, and configurable debug levels.

// use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::io::{self, Write, stderr};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::fmt;

/// Debug level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugLevel {
    Off = 0,
    CursedError = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl fmt::Display for DebugLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DebugLevel::Off => write!(f, "OFF"),
            DebugLevel::CursedError => write!(f, "ERROR"),
            DebugLevel::Warning => write!(f, "WARN"),
            DebugLevel::Info => write!(f, "INFO"),
            DebugLevel::Debug => write!(f, "DEBUG"),
            DebugLevel::Trace => write!(f, "TRACE"),
        }
    }
}

/// Debug output styling options
#[derive(Debug, Clone)]
pub enum DebugStyle {
    Plain,
    Colored,
    Compact,
    Pretty,
    Json,
}

/// Debug output options
#[derive(Debug, Clone)]
pub struct DebugOptions {
    pub level: DebugLevel,
    pub style: DebugStyle,
    pub show_timestamp: bool,
    pub show_location: bool,
    pub max_depth: usize,
    pub max_length: usize,
    pub indent_size: usize,
}

impl Default for DebugOptions {
    fn default() -> Self {
        Self {
            level: DebugLevel::Info,
            style: DebugStyle::Pretty,
            show_timestamp: true,
            show_location: false,
            max_depth: 10,
            max_length: 1000,
            indent_size: 2,
        }
    }
}

/// Global debug state
struct DebugState {
    level: DebugLevel,
    options: DebugOptions,
    enabled: bool,
}

static DEBUG_STATE: Mutex<DebugState> = Mutex::new(DebugState {
    level: DebugLevel::Info,
    options: DebugOptions {
        level: DebugLevel::Info,
        style: DebugStyle::Pretty,
        show_timestamp: true,
        show_location: false,
        max_depth: 10,
        max_length: 1000,
        indent_size: 2,
    },
    enabled: true,
});

/// Initialize the debug system
pub fn init_debug_system() {
        // TODO: implement
    }
    // Initialize with default settings
    if let Ok(mut state) = DEBUG_STATE.lock() {
        state.enabled = true;
        
        // Check environment variables for configuration
        if let Ok(level_str) = std::env::var("CURSED_DEBUG_LEVEL") {
            if let Ok(level) = parse_debug_level(&level_str) {
                state.level = level;
                state.options.level = level;
            }
        }
        
        if let Ok(_) = std::env::var("CURSED_DEBUG_DISABLE") {
            state.enabled = false;
        }
    }
}

/// Set the global debug level
pub fn set_debug_level(level: DebugLevel) {
    if let Ok(mut state) = DEBUG_STATE.lock() {
        state.level = level;
        state.options.level = level;
    }
}

/// Get the current debug level
pub fn get_debug_level() -> DebugLevel {
    DEBUG_STATE.lock()
        .map(|state| state.level)
        .unwrap_or(DebugLevel::Info)
}

/// Check if debug output is enabled for a given level
pub fn is_debug_enabled(level: DebugLevel) -> bool {
    DEBUG_STATE.lock()
        .map(|state| state.enabled && level <= state.level)
        .unwrap_or(false)
}

/// Print debug message at specified level
/// Example: debug_print(DebugLevel::Info, &[Value::String("Debug message".to_string())])
pub fn debug_print(level: DebugLevel, args: &[Value]) -> io::Result<()> {
    if !is_debug_enabled(level) {
        return Ok(());
    }
    
    let options = DEBUG_STATE.lock()
        .map(|state| state.options.clone())
        .unwrap_or_default();
    
    let formatted = format_debug_message(level, args, &options)?;
    
    let mut output = stderr();
    write!(output, "{}", formatted)?;
    output.flush()
}

/// Print debug message with newline
/// Example: debug_println(DebugLevel::Debug, &[Value::String("Debug info".to_string())])
pub fn debug_println(level: DebugLevel, args: &[Value]) -> io::Result<()> {
    debug_print(level, args)?;
    eprintln!();
    Ok(())
}

/// Format debug message with template
/// Example: debug_format(DebugLevel::CursedError, "CursedError in {}: {}", &[func_name, error])
pub fn debug_format(level: DebugLevel, template: &str, args: &[Value]) -> io::Result<()> {
    if !is_debug_enabled(level) {
        return Ok(());
    }
    
    // Simple template substitution
    let mut result = template.to_string();
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        if result.contains(&placeholder) {
            result = result.replace(&placeholder, &format_value_debug(arg));
        } else {
            // If no numbered placeholder, replace first {}
            if let Some(pos) = result.find("{}") {
                result.replace_range(pos..pos+2, &format_value_debug(arg));
            }
        }
    }
    
    debug_println(level, &[Value::String(result)])
}

/// Pretty print a value with full details
/// Example: pretty_print(&Value::Object(map))
pub fn pretty_print(value: &Value) -> io::Result<()> {
    let options = DEBUG_STATE.lock()
        .map(|state| state.options.clone())
        .unwrap_or_default();
    
    let formatted = format_value_pretty(value, 0, &options);
    println!("{}", formatted);
    Ok(())
}

/// Dump detailed information about a value
/// Example: debug_dump("variable_name", &value)
pub fn debug_dump(name: &str, value: &Value) -> io::Result<()> {
    if !is_debug_enabled(DebugLevel::Debug) {
        return Ok(());
    }
    
    let mut output = stderr();
    writeln!(output, "=== DEBUG DUMP: {} ===", name)?;
    writeln!(output, "Type: {}", get_value_type_name(value))?;
    writeln!(output, "Value: {}", format_value_debug(value))?;
    writeln!(output, "Pretty:")?;
    pretty_print(value)?;
    writeln!(output, "========================")?;
    output.flush()
}

/// Inspect a value and return detailed information
/// Example: debug_inspect(&value)
pub fn debug_inspect(value: &Value) -> String {
    let mut result = String::new();
    
    result.push_str(&format!("Type: {}\n", get_value_type_name(value)));
    result.push_str(&format!("Value: {}\n", format_value_debug(value)));
    
    match value {
        Value::Array(arr) => {
            result.push_str(&format!("Length: {}\n", arr.len()));
            if !arr.is_empty() {
                result.push_str("Elements:\n");
                for (i, elem) in arr.iter().enumerate().take(10) {
                    result.push_str(&format!("  [{}]: {}\n", i, format_value_debug(elem)));
                }
                if arr.len() > 10 {
                    result.push_str(&format!("  ... {} more elements\n", arr.len() - 10));
                }
            }
        }
        Value::Object(obj) => {
            result.push_str(&format!("Properties: {}\n", obj.len()));
            if !obj.is_empty() {
                result.push_str("Properties:\n");
                for (i, (key, value)) in obj.iter().enumerate().take(10) {
                    result.push_str(&format!("  {}: {}\n", key, format_value_debug(value)));
                }
                if obj.len() > 10 {
                    result.push_str(&format!("  ... {} more properties\n", obj.len() - 10));
                }
            }
        }
        Value::String(s) => {
            result.push_str(&format!("Length: {}\n", s.len()));
            if s.len() > 100 {
                result.push_str(&format!("Preview: {}...\n", &s[..100]));
            }
        }
        Value::Bytes(b) => {
            result.push_str(&format!("Size: {} bytes\n", b.len()));
            if !b.is_empty() {
                let preview_len = std::cmp::min(b.len(), 16);
                let hex_preview: String = b[..preview_len]
                    .iter()
                    .map(|byte| format!("{:02x}", byte))
                    .collect::<Vec<_>>()
                    .join(" ");
                result.push_str(&format!("Hex preview: {}\n", hex_preview));
                if b.len() > 16 {
                    result.push_str(&format!("... {} more bytes\n", b.len() - 16));
                }
            }
        }
        _ => {}
    }
    
    result
}

/// Print a trace message with call stack information
/// Example: debug_trace("Entering function", &[Value::String(func_name.to_string())])
pub fn debug_trace(message: &str, args: &[Value]) -> io::Result<()> {
    if !is_debug_enabled(DebugLevel::Trace) {
        return Ok(());
    }
    
    let timestamp = get_timestamp();
    let mut formatted_args = String::new();
    
    if !args.is_empty() {
        formatted_args = args.iter()
            .map(format_value_debug)
            .collect::<Vec<_>>()
            .join(", ");
    }
    
    let trace_msg = if formatted_args.is_empty() {
        format!("[{}] TRACE: {}", timestamp, message)
    } else {
        format!("[{}] TRACE: {} ({})", timestamp, message, formatted_args)
    };
    
    eprintln!("{}", trace_msg);
    Ok(())
}

/// Format a debug message with level and options
fn format_debug_message(level: DebugLevel, args: &[Value], options: &DebugOptions) -> io::Result<String> {
    let mut result = String::new();
    
    // Add timestamp if enabled
    if options.show_timestamp {
        result.push_str(&format!("[{}] ", get_timestamp()));
    }
    
    // Add level
    result.push_str(&format!("{}: ", level));
    
    // Format arguments
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        
        match options.style {
            DebugStyle::Pretty => {
                result.push_str(&format_value_pretty(arg, 0, options));
            }
            DebugStyle::Json => {
                result.push_str(&format_value_json(arg));
            }
            DebugStyle::Compact => {
                result.push_str(&format_value_compact(arg));
            }
            DebugStyle::Colored => {
                result.push_str(&format_value_colored(arg));
            }
            DebugStyle::Plain => {
                result.push_str(&format_value_debug(arg));
            }
        }
    }
    
    Ok(result)
}

/// Format value for debug output
fn format_value_debug(value: &Value) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::Array(arr) => {
            if arr.len() <= 5 {
                let items: Vec<String> = arr.iter().map(format_value_debug).collect();
                format!("[{}]", items.join(", "))
            } else {
                let preview: Vec<String> = arr.iter().take(3).map(format_value_debug).collect();
                format!("[{}, ... {} more]", preview.join(", "), arr.len() - 3)
            }
        }
        Value::Object(obj) => {
            if obj.len() <= 3 {
                let items: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, format_value_debug(v)))
                    .collect();
                format!("{{{}}}", items.join(", "))
            } else {
                format!("{{...{} properties...}}", obj.len())
            }
        }
        Value::Function(_) => "<function>".to_string(),
        Value::NativeFunction(_) => "<native_function>".to_string(),
        Value::Channel(_) => "<channel>".to_string(),
        Value::Interface(_) => "<interface>".to_string(),
        Value::CursedError(e) => format!("CursedError({})", e),
        Value::Bytes(b) => format!("<bytes[{}]>", b.len()),
    }
}

/// Pretty format value with indentation
fn format_value_pretty(value: &Value, depth: usize, options: &DebugOptions) -> String {
    if depth >= options.max_depth {
        return "...".to_string();
    }
    
    let indent = " ".repeat(depth * options.indent_size);
    let next_indent = " ".repeat((depth + 1) * options.indent_size);
    
    match value {
        Value::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else if arr.len() == 1 {
                format!("[{}]", format_value_pretty(&arr[0], depth + 1, options))
            } else {
                let items: Vec<String> = arr.iter()
                    .map(|v| format!("{}{}", next_indent, format_value_pretty(v, depth + 1, options)))
                    .collect();
                format!("[\n{}\n{}]", items.join(",\n"), indent)
            }
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                "{}".to_string()
            } else {
                let items: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{}{}: {}", next_indent, k, format_value_pretty(v, depth + 1, options)))
                    .collect();
                format!("{{\n{}\n{}}}", items.join(",\n"), indent)
            }
        }
        _ => format_value_debug(value)
    }
}

/// Format value as JSON
fn format_value_json(value: &Value) -> String {
    // Simplified JSON formatting
    match value {
        Value::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(format_value_json).collect();
            format!("[{}]", items.join(","))
        }
        Value::Object(obj) => {
            let items: Vec<String> = obj.iter()
                .map(|(k, v)| format!("\"{}\":{}", k, format_value_json(v)))
                .collect();
            format!("{{{}}}", items.join(","))
        }
        _ => format_value_debug(value)
    }
}

/// Format value in compact form
fn format_value_compact(value: &Value) -> String {
    match value {
        Value::Array(arr) => {
            if arr.len() > 10 {
                format!("[...{}...]", arr.len())
            } else {
                format_value_debug(value)
            }
        }
        Value::Object(obj) => {
            if obj.len() > 5 {
                format!("{{...{}...}}", obj.len())
            } else {
                format_value_debug(value)
            }
        }
        Value::String(s) => {
            if s.len() > 50 {
                format!("\"{}...\"", &s[..47])
            } else {
                format!("\"{}\"", s)
            }
        }
        _ => format_value_debug(value)
    }
}

/// Format value with color codes
fn format_value_colored(value: &Value) -> String {
    match value {
        Value::Nil => "\x1b[90mnil\x1b[0m".to_string(),
        Value::Bool(true) => "\x1b[92mtrue\x1b[0m".to_string(),
        Value::Bool(false) => "\x1b[91mfalse\x1b[0m".to_string(),
        Value::Int(i) => format!("\x1b[94m{}\x1b[0m", i),
        Value::Float(f) => format!("\x1b[96m{}\x1b[0m", f),
        Value::String(s) => format!("\x1b[93m\"{}\"\x1b[0m", s),
        _ => format_value_debug(value)
    }
}

/// Get type name for a value
fn get_value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Nil => "nil",
        Value::Bool(_) => "bool",
        Value::Int(_) => "int",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        Value::Function(_) => "function",
        Value::NativeFunction(_) => "native_function",
        Value::Channel(_) => "channel",
        Value::Interface(_) => "interface",
        Value::CursedError(_) => "error",
        Value::Bytes(_) => "bytes",
    }
}

/// Get current timestamp as string
fn get_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    // Simple timestamp format
    format!("{}.{:03}", secs, millis)
}

/// Parse debug level from string
fn parse_debug_level(level_str: &str) -> Result<DebugLevel, ()> {
    match level_str.to_uppercase().as_str() {
        "OFF" | "0" => Ok(DebugLevel::Off),
        "ERROR" | "1" => Ok(DebugLevel::CursedError),
        "WARN" | "WARNING" | "2" => Ok(DebugLevel::Warning),
        "INFO" | "3" => Ok(DebugLevel::Info),
        "DEBUG" | "4" => Ok(DebugLevel::Debug),
        "TRACE" | "5" => Ok(DebugLevel::Trace),
        _ => Err(())
    }
}

