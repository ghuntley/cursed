//! `chadlogging` - Structured logging for CURSED language
//! Provides leveled, structured logging with attributes and groups.

use crate::memory::{Traceable, Tag, Visitor};
use crate::object::{self, Object};
use crate::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

// Log level constants
pub const LEVEL_DEBUG: i64 = -4;
pub const LEVEL_INFO: i64 = 0;
pub const LEVEL_WARN: i64 = 4;
pub const LEVEL_ERROR: i64 = 8;

/// Represents a key-value attribute pair
#[derive(Clone, Debug)]
pub struct Attr {
    pub key: String,
    pub value: Object,
}

impl Attr {
    /// Create a new attribute
    pub fn new(key: impl Into<String>, value: impl Into<Object>) -> Self {
        Attr {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Traceable for Attr {
    fn trace(&self, visitor: &mut dyn Visitor) {
        self.value.trace(visitor);
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + self.key.len() + self.value.size()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

/// A log record contains all information about a logging event
#[derive(Clone, Debug)]
pub struct Record {
    pub timestamp: SystemTime,
    pub level: i64,
    pub message: String,
    pub attrs: Vec<Attr>,
}

impl Record {
    /// Create a new log record
    pub fn new(level: i64, message: impl Into<String>, attrs: Vec<Attr>) -> Self {
        Record {
            timestamp: SystemTime::now(),
            level,
            message: message.into(),
            attrs,
        }
    }
    
    /// Get level as a string
    pub fn level_string(&self) -> &str {
        match self.level {
            LEVEL_DEBUG => "DEBUG",
            LEVEL_INFO => "INFO",
            LEVEL_WARN => "WARN",
            LEVEL_ERROR => "ERROR",
            _ if self.level < LEVEL_DEBUG => "DEBUG",
            _ if self.level < LEVEL_INFO => "DEBUG",
            _ if self.level < LEVEL_WARN => "INFO",
            _ if self.level < LEVEL_ERROR => "WARN",
            _ => "ERROR",
        }
    }
    
    /// Get timestamp as RFC3339 string
    pub fn time_string(&self) -> String {
        let unix_time = self.timestamp.duration_since(UNIX_EPOCH).unwrap_or_default();
        let secs = unix_time.as_secs();
        let nanos = unix_time.subsec_nanos();
        
        // Basic RFC3339 format: YYYY-MM-DDTHH:MM:SS.sssZ
        // This is a simplified implementation
        format!("{}.{:09}Z", secs, nanos)
    }
}

impl Traceable for Record {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for attr in &self.attrs {
            attr.trace(visitor);
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + 
            self.message.len() + 
            self.attrs.iter().map(|a| a.size()).sum::<usize>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

/// Handler options for configuring handlers
pub struct HandlerOptions {
    pub level: i64,
    pub add_source: bool,
    pub replace_attr: Option<Box<dyn Fn(Vec<String>, Attr) -> Attr>>,
}

impl std::fmt::Debug for HandlerOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HandlerOptions")
            .field("level", &self.level)
            .field("add_source", &self.add_source)
            .field("replace_attr", &format!("<function pointer>"))
            .finish()
    }
}

impl Clone for HandlerOptions {
    fn clone(&self) -> Self {
        HandlerOptions {
            level: self.level,
            add_source: self.add_source,
            replace_attr: None, // We can't clone the function
        }
    }
}

impl Default for HandlerOptions {
    fn default() -> Self {
        HandlerOptions {
            level: LEVEL_INFO,
            add_source: false,
            replace_attr: None,
        }
    }
}

impl Traceable for HandlerOptions {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // Nothing to trace
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

/// Handler interface for processing log records
pub trait Handler: Traceable {
    fn enabled(&self, level: i64) -> bool;
    fn handle(&self, record: Record) -> bool;
    fn with_attrs(&self, attrs: Vec<Attr>) -> Box<dyn Handler>;
    fn with_group(&self, name: String) -> Box<dyn Handler>;
}

/// TextHandler formats logs as key=value pairs
pub struct TextHandler {
    writer: Rc<RefCell<Vec<String>>>,
    attrs: Vec<Attr>,
    groups: Vec<String>,
    options: HandlerOptions,
}

impl TextHandler {
    /// Create a new TextHandler with default options
    pub fn new(writer: Rc<RefCell<Vec<String>>>) -> Self {
        TextHandler {
            writer,
            attrs: Vec::new(),
            groups: Vec::new(),
            options: HandlerOptions::default(),
        }
    }
    
    /// Create a new TextHandler with specified options
    pub fn new_with_options(writer: Rc<RefCell<Vec<String>>>, options: HandlerOptions) -> Self {
        TextHandler {
            writer,
            attrs: Vec::new(),
            groups: Vec::new(),
            options,
        }
    }
    
    /// Create a clone with added attributes
    fn clone_with_attrs(&self, attrs: Vec<Attr>) -> Self {
        let mut new_attrs = self.attrs.clone();
        new_attrs.extend(attrs);
        
        TextHandler {
            writer: self.writer.clone(),
            attrs: new_attrs,
            groups: self.groups.clone(),
            options: self.options.clone(),
        }
    }
    
    /// Create a clone with added group
    fn clone_with_group(&self, name: String) -> Self {
        let mut new_groups = self.groups.clone();
        new_groups.push(name);
        
        TextHandler {
            writer: self.writer.clone(),
            attrs: self.attrs.clone(),
            groups: new_groups,
            options: self.options.clone(),
        }
    }
    
    /// Format an attribute
    fn format_attr(&self, attr: &Attr, groups: &[String]) -> String {
        let key = if groups.is_empty() {
            attr.key.clone()
        } else {
            format!("{}.{}", groups.join("."), attr.key)
        };
        
        match &attr.value {
            Object::String(s) => format!("{key}=\"{s}\""),
            Object::Integer(i) => format!("{key}={i}"),
            Object::Float(f) => format!("{key}={f}"),
            Object::Boolean(b) => format!("{key}={b}"),
            Object::Array(arr) => format!("{key}=[{}]", 
                arr.iter().map(|v| std::format!("{:?}", v)).collect::<Vec<_>>().join(", ")),
            Object::HashTable(map) => {
                if map.contains_key("__type") && map["__type"] == Object::String("attrs".to_string()) {
                    // Special group formatting
                    format!("{key}=<group>")
                } else {
                    format!("{key}={{{}}}", 
                        map.iter().map(|(k, v)| std::format!("{k}:{:?}", v)).collect::<Vec<_>>().join(", "))
                }
            },
            _ => format!("{key}=null"),
        }
    }
}

impl Handler for TextHandler {
    fn enabled(&self, level: i64) -> bool {
        level >= self.options.level
    }
    
    fn handle(&self, record: Record) -> bool {
        if !self.enabled(record.level) {
            return false;
        }
        
        // Start with basic fields
        let mut parts = vec![
            format!("time=*"),  // Simplified time handling
            format!("level={}", record.level_string()),
            format!("msg=\"{}\"", record.message),
        ];
        
        // Add handler's attributes
        for attr in &self.attrs {
            parts.push(self.format_attr(attr, &self.groups));
        }
        
        // Add record's attributes
        for attr in &record.attrs {
            parts.push(self.format_attr(attr, &self.groups));
        }
        
        // Join all parts
        let log_line = parts.join(" ");
        
        // Write to the output
        self.writer.borrow_mut().push(log_line);
        
        true
    }
    
    fn with_attrs(&self, attrs: Vec<Attr>) -> Box<dyn Handler> {
        Box::new(self.clone_with_attrs(attrs))
    }
    
    fn with_group(&self, name: String) -> Box<dyn Handler> {
        Box::new(self.clone_with_group(name))
    }
}

impl Traceable for TextHandler {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for attr in &self.attrs {
            attr.trace(visitor);
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + 
            self.attrs.iter().map(|a| a.size()).sum::<usize>() + 
            self.groups.iter().map(|g| g.len()).sum::<usize>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

/// JSONHandler formats logs as JSON objects
pub struct JSONHandler {
    writer: Rc<RefCell<Vec<String>>>,
    attrs: Vec<Attr>,
    groups: Vec<String>,
    options: HandlerOptions,
}

impl JSONHandler {
    /// Create a new JSONHandler with default options
    pub fn new(writer: Rc<RefCell<Vec<String>>>) -> Self {
        JSONHandler {
            writer,
            attrs: Vec::new(),
            groups: Vec::new(),
            options: HandlerOptions::default(),
        }
    }
    
    /// Create a new JSONHandler with specified options
    pub fn new_with_options(writer: Rc<RefCell<Vec<String>>>, options: HandlerOptions) -> Self {
        JSONHandler {
            writer,
            attrs: Vec::new(),
            groups: Vec::new(),
            options,
        }
    }
    
    /// Create a clone with added attributes
    fn clone_with_attrs(&self, attrs: Vec<Attr>) -> Self {
        let mut new_attrs = self.attrs.clone();
        new_attrs.extend(attrs);
        
        JSONHandler {
            writer: self.writer.clone(),
            attrs: new_attrs,
            groups: self.groups.clone(),
            options: self.options.clone(),
        }
    }
    
    /// Create a clone with added group
    fn clone_with_group(&self, name: String) -> Self {
        let mut new_groups = self.groups.clone();
        new_groups.push(name);
        
        JSONHandler {
            writer: self.writer.clone(),
            attrs: self.attrs.clone(),
            groups: new_groups,
            options: self.options.clone(),
        }
    }
    
    /// Convert an attribute to a JSON key/value pair
    fn json_attr(&self, attr: &Attr, groups: &[String]) -> String {
        let key = if groups.is_empty() {
            format!("\"{}\"", attr.key)
        } else {
            format!("\"{}.{}\"", groups.join("."), attr.key)
        };
        
        match &attr.value {
            Object::String(s) => format!("{key}:\"{s}\""),
            Object::Integer(i) => format!("{key}:{i}"),
            Object::Float(f) => format!("{key}:{f}"),
            Object::Boolean(b) => format!("{key}:{}", if *b { "true" } else { "false" }),
            Object::Array(arr) => format!("{key}:[{}]", 
                arr.iter().map(|v| self.json_value(v)).collect::<Vec<_>>().join(",")),
            Object::HashTable(map) => {
                if map.contains_key("__type") && map["__type"] == Object::String("attrs".to_string()) {
                    // Special group formatting
                    format!("{key}:{{\"type\":\"group\"}}")
                } else {
                    format!("{key}:{{{}}}", 
                        map.iter().map(|(k, v)| format!("\"{k}\":{}", self.json_value(v))).collect::<Vec<_>>().join(","))
                }
            },
            _ => format!("{key}:null"),
        }
    }
    
    /// Convert an Object to a JSON value
    fn json_value(&self, value: &Object) -> String {
        match value {
            Object::String(s) => format!("\"{s}\""),
            Object::Integer(i) => format!("{i}"),
            Object::Float(f) => format!("{f}"),
            Object::Boolean(b) => format!("{}", if *b { "true" } else { "false" }),
            Object::Array(arr) => format!("[{}]", 
                arr.iter().map(|v| self.json_value(v)).collect::<Vec<_>>().join(",")),
            Object::HashTable(map) => format!("{{{}}}", 
                map.iter().map(|(k, v)| format!("\"{k}\":{}", self.json_value(v))).collect::<Vec<_>>().join(",")),
            _ => "null".to_string(),
        }
    }
}

impl Handler for JSONHandler {
    fn enabled(&self, level: i64) -> bool {
        level >= self.options.level
    }
    
    fn handle(&self, record: Record) -> bool {
        if !self.enabled(record.level) {
            return false;
        }
        
        // Start with basic fields
        let mut json_fields = vec![
            format!("\"time\":\"{}\"*", record.time_string()),  // Simplified time handling
            format!("\"level\":\"{}\"", record.level_string()),
            format!("\"msg\":\"{}\"", record.message),
        ];
        
        // Add handler's attributes
        for attr in &self.attrs {
            json_fields.push(self.json_attr(attr, &self.groups));
        }
        
        // Add record's attributes
        for attr in &record.attrs {
            json_fields.push(self.json_attr(attr, &self.groups));
        }
        
        // Join all parts into a JSON object
        let json_line = format!("{{{}}}", json_fields.join(","));
        
        // Write to the output
        self.writer.borrow_mut().push(json_line);
        
        true
    }
    
    fn with_attrs(&self, attrs: Vec<Attr>) -> Box<dyn Handler> {
        Box::new(self.clone_with_attrs(attrs))
    }
    
    fn with_group(&self, name: String) -> Box<dyn Handler> {
        Box::new(self.clone_with_group(name))
    }
}

impl Traceable for JSONHandler {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for attr in &self.attrs {
            attr.trace(visitor);
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + 
            self.attrs.iter().map(|a| a.size()).sum::<usize>() + 
            self.groups.iter().map(|g| g.len()).sum::<usize>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

/// TestHandler for capturing logs in tests
pub struct TestHandler {
    logs: Rc<RefCell<Vec<Record>>>,
    attrs: Vec<Attr>,
    groups: Vec<String>,
}

impl TestHandler {
    /// Create a new TestHandler
    pub fn new(logs: Rc<RefCell<Vec<Record>>>) -> Self {
        TestHandler {
            logs,
            attrs: Vec::new(),
            groups: Vec::new(),
        }
    }
    
    /// Create a clone with added attributes
    fn clone_with_attrs(&self, attrs: Vec<Attr>) -> Self {
        let mut new_attrs = self.attrs.clone();
        new_attrs.extend(attrs);
        
        TestHandler {
            logs: self.logs.clone(),
            attrs: new_attrs,
            groups: self.groups.clone(),
        }
    }
    
    /// Create a clone with added group
    fn clone_with_group(&self, name: String) -> Self {
        let mut new_groups = self.groups.clone();
        new_groups.push(name);
        
        TestHandler {
            logs: self.logs.clone(),
            attrs: self.attrs.clone(),
            groups: new_groups,
        }
    }
}

impl Handler for TestHandler {
    fn enabled(&self, _level: i64) -> bool {
        true  // TestHandler captures all logs
    }
    
    fn handle(&self, record: Record) -> bool {
        // Create a new record that includes the handler's attributes
        let mut all_attrs = self.attrs.clone();
        all_attrs.extend(record.attrs.clone());
        
        let new_record = Record {
            timestamp: record.timestamp,
            level: record.level,
            message: record.message,
            attrs: all_attrs,
        };
        
        // Store the record
        self.logs.borrow_mut().push(new_record);
        true
    }
    
    fn with_attrs(&self, attrs: Vec<Attr>) -> Box<dyn Handler> {
        Box::new(self.clone_with_attrs(attrs))
    }
    
    fn with_group(&self, name: String) -> Box<dyn Handler> {
        Box::new(self.clone_with_group(name))
    }
}

impl Traceable for TestHandler {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for attr in &self.attrs {
            attr.trace(visitor);
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + 
            self.attrs.iter().map(|a| a.size()).sum::<usize>() + 
            self.groups.iter().map(|g| g.len()).sum::<usize>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

/// The Logger struct for generating log records
pub struct Logger {
    handler: Box<dyn Handler>,
    attrs: Vec<Attr>,
}

impl Logger {
    /// Create a new logger with the specified handler
    pub fn new(handler: Box<dyn Handler>) -> Self {
        Logger {
            handler,
            attrs: Vec::new(),
        }
    }
    
    /// Log a message at DEBUG level
    pub fn debug(&self, msg: impl Into<String>, args: Vec<impl Into<Object>>) {
        self.log(LEVEL_DEBUG, msg, args);
    }
    
    /// Log a message at INFO level
    pub fn info(&self, msg: impl Into<String>, args: Vec<impl Into<Object>>) {
        self.log(LEVEL_INFO, msg, args);
    }
    
    /// Log a message at WARN level
    pub fn warn(&self, msg: impl Into<String>, args: Vec<impl Into<Object>>) {
        self.log(LEVEL_WARN, msg, args);
    }
    
    /// Log a message at ERROR level
    pub fn error(&self, msg: impl Into<String>, args: Vec<impl Into<Object>>) {
        self.log(LEVEL_ERROR, msg, args);
    }
    
    /// Log a message at the specified level with args
    pub fn log(&self, level: i64, msg: impl Into<String>, args: Vec<impl Into<Object>>) {
        if !self.handler.enabled(level) {
            return;
        }
        
        // Convert args to attributes
        let attrs = self.parse_args(args);
        
        // Create record
        let record = Record::new(level, msg, attrs);
        
        // Handle record
        self.handler.handle(record);
    }
    
    /// Create a new logger with additional attributes
    pub fn with(&self, args: Vec<impl Into<Object>>) -> Self {
        let attrs = self.parse_args(args);
        let handler = self.handler.with_attrs(attrs.clone());
        
        let mut new_attrs = self.attrs.clone();
        new_attrs.extend(attrs);
        
        Logger {
            handler,
            attrs: new_attrs,
        }
    }
    
    /// Create a new logger with a group
    pub fn with_group(&self, name: impl Into<String>) -> Self {
        let name_str = name.into();
        let handler = self.handler.with_group(name_str.clone());
        
        Logger {
            handler,
            attrs: self.attrs.clone(),
        }
    }
    
    /// Parse variable arguments into Attr objects
    fn parse_args(&self, args: Vec<impl Into<Object>>) -> Vec<Attr> {
        let mut attrs = Vec::new();
        let objects: Vec<Object> = args.into_iter().map(|a| a.into()).collect();
        
        let mut i = 0;
        while i < objects.len() {
            if i + 1 < objects.len() {
                if let Object::String(key) = &objects[i] {
                    if let Object::HashTable(group_map) = &objects[i + 1] {
                        if group_map.contains_key("__type") && group_map["__type"] == Object::String("attrs".to_string()) {
                            // Handle group
                            attrs.push(Attr::new(key.clone(), objects[i + 1].clone()));
                        }
                    } else {
                        // Handle key-value pair
                        attrs.push(Attr::new(key.clone(), objects[i + 1].clone()));
                    }
                    i += 2;
                } else {
                    // Skip non-string keys
                    i += 1;
                }
            } else {
                // Skip odd items
                i += 1;
            }
        }
        
        attrs
    }
}

impl Traceable for Logger {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for attr in &self.attrs {
            attr.trace(visitor);
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>() + 
            self.attrs.iter().map(|a| a.size()).sum::<usize>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

/// Global state for default logger
static mut DEFAULT_LOGGER: Option<Logger> = None;

/// Create a new logger with the specified handler
pub fn new(handler: impl Handler + 'static) -> Logger {
    Logger::new(Box::new(handler))
}

/// Get the default logger
pub fn default() -> Logger {
    unsafe {
        if DEFAULT_LOGGER.is_none() {
            // Create a default logger with a TextHandler that writes to stdout
            let buffer = Rc::new(RefCell::new(Vec::new()));
            let handler = TextHandler::new(buffer);
            DEFAULT_LOGGER = Some(Logger::new(Box::new(handler)));
        }
        DEFAULT_LOGGER.as_ref().unwrap().clone()
    }
}

/// Set the default logger
pub fn set_default(logger: Logger) {
    unsafe {
        DEFAULT_LOGGER = Some(logger);
    }
}

/// Create a string attribute
pub fn string(key: impl Into<String>, value: impl Into<String>) -> Attr {
    Attr::new(key, Object::String(value.into()))
}

/// Create an integer attribute
pub fn int(key: impl Into<String>, value: i64) -> Attr {
    Attr::new(key, Object::Integer(value))
}

/// Create a float attribute
pub fn float(key: impl Into<String>, value: f64) -> Attr {
    Attr::new(key, Object::Float(value))
}

/// Create a boolean attribute
pub fn boolean(key: impl Into<String>, value: bool) -> Attr {
    Attr::new(key, Object::Boolean(value))
}

/// Create a group of attributes
pub fn group(key: impl Into<String>, args: Vec<impl Into<Object>>) -> Object {
    let key_str = key.into();
    let objects: Vec<Object> = args.into_iter().map(|a| a.into()).collect();
    
    // Create a dict of key-value pairs
    let mut group_map = HashMap::new();
    group_map.insert("__type".to_string(), Object::String("attrs".to_string()));
    
    // Add the key-value pairs from args
    let mut i = 0;
    while i + 1 < objects.len() {
        if let Object::String(key) = &objects[i] {
            group_map.insert(key.clone(), objects[i + 1].clone());
        }
        i += 2;
    }
    
    Object::HashTable(group_map)
}

/// Log at DEBUG level using the default logger
pub fn debug(msg: impl Into<String>, args: Vec<impl Into<Object>>) {
    default().debug(msg, args);
}

/// Log at INFO level using the default logger
pub fn info(msg: impl Into<String>, args: Vec<impl Into<Object>>) {
    default().info(msg, args);
}

/// Log at WARN level using the default logger
pub fn warn(msg: impl Into<String>, args: Vec<impl Into<Object>>) {
    default().warn(msg, args);
}

/// Log at ERROR level using the default logger
pub fn error(msg: impl Into<String>, args: Vec<impl Into<Object>>) {
    default().error(msg, args);
}

// Implementation of Clone for Logger
impl Clone for Logger {
    fn clone(&self) -> Self {
        Logger {
            handler: self.handler.with_attrs(Vec::new()),  // This creates a clone of the handler
            attrs: self.attrs.clone(),
        }
    }
}