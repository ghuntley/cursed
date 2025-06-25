/// Advanced string formatting with placeholders and format specifiers
/// 
/// Provides comprehensive string formatting capabilities including
/// placeholder interpolation, format specifications, and context-aware formatting.

// use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use std::fmt;

/// Format error types
#[derive(Debug, Clone, PartialEq)]
pub enum FormatError {
// impl fmt::Display for FormatError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             FormatError::InvalidPlaceholder(placeholder) => {
//                 write!(f, "Invalid placeholder: {}", placeholder)
//             }
//             FormatError::MissingArgument(index) => {
//                 write!(f, "Missing argument at index: {}", index) 
//             }
//             FormatError::TooManyArguments => {
//                 write!(f, "Too many arguments provided")
//             }
//             FormatError::InvalidFormatSpec(spec) => {
//                 write!(f, "Invalid format specification: {}", spec)
//             }
//             FormatError::CircularReference(name) => {
//                 write!(f, "Circular reference detected: {}", name)
//             }
//             FormatError::TypeMismatch(msg) => {
//                 write!(f, "Type mismatch: {}", msg)
//             }
//             FormatError::IndexOutOfBounds(index) => {
//                 write!(f, "Index out of bounds: {}", index)
//             }
//             FormatError::InvalidContext(msg) => {
//                 write!(f, "Invalid context: {}", msg)
//             }
//         }
//     }
// }

// impl std::error::CursedError for FormatError {}
// 
pub type FormatResult<T> = std::result::Result<T, FormatError>;

/// Format specification options
#[derive(Debug, Clone)]
pub struct FormatSpec {
impl Default for FormatSpec {
    fn default() -> Self {
        Self {
        }
    }
/// Format alignment options
#[derive(Debug, Clone, Copy)]
pub enum FormatAlignment {
/// Format sign options
#[derive(Debug, Clone, Copy)]
pub enum FormatSign {
    Plus,      // Always show sign
    Minus,     // Only show negative (default)
    Space,     // Space for positive, minus for negative
/// Placeholder types
#[derive(Debug, Clone)]
pub enum PlaceholderType {
    Positional(usize),          // {0}, {1}, etc.
    Named(String),              // {name}, {value}, etc.
    Auto,                       // {} - uses next available argument
/// Format placeholder information
#[derive(Debug, Clone)]
pub struct FormatPlaceholder {
/// Format context for named placeholders and advanced features
#[derive(Debug, Clone)]
pub struct FormatContext {
impl Default for FormatContext {
    fn default() -> Self {
        Self {
        }
    }
/// Format options for controlling behavior
#[derive(Debug, Clone)]
pub struct FormatOptions {
    pub strict_mode: bool,          // CursedError on missing arguments vs. using placeholder
    pub html_escape: bool,          // HTML escape output
    pub trim_whitespace: bool,      // Trim leading/trailing whitespace
    pub allow_functions: bool,      // Allow function calls in placeholders
impl Default for FormatOptions {
    fn default() -> Self {
        Self {
        }
    }
/// Format a template string with positional arguments
/// Example: format("Hello {}, you are {} years old", &[name, age])
pub fn format(template: &str, args: &[Value]) -> FormatResult<String> {
    let placeholders = parse_placeholders(template)?;
    let mut result = String::new();
    let mut last_end = 0;
    let mut auto_index = 0;

    for placeholder in placeholders {
        // Add text before placeholder
        result.push_str(&template[last_end..placeholder.start]);

        // Get the argument for this placeholder
        let arg_index = match &placeholder.placeholder_type {
            PlaceholderType::Auto => {
                let index = auto_index;
                auto_index += 1;
                index
            }
            PlaceholderType::Named(_) => {
                return Err(FormatError::InvalidPlaceholder(
                    "Named placeholders not supported without context".to_string()
                ));
            }

        // Check if argument exists
        if arg_index >= args.len() {
            return Err(FormatError::MissingArgument(arg_index));
        // Format the argument
        let formatted = format_value_with_spec(&args[arg_index], &placeholder.format_spec)?;
        result.push_str(&formatted);

        last_end = placeholder.end;
    // Add remaining text
    result.push_str(&template[last_end..]);

    Ok(result)
/// Format with named arguments using context
/// Example: format_with_context("Hello {name}", &[], &context)
pub fn format_with_context(
    context: &FormatContext
) -> FormatResult<String> {
    let mut context = context.clone();
    context.current_depth += 1;

    if context.current_depth > context.max_recursion_depth {
        return Err(FormatError::CircularReference("Maximum recursion depth exceeded".to_string()));
    let placeholders = parse_placeholders(template)?;
    let mut result = String::new();
    let mut last_end = 0;
    let mut auto_index = 0;

    for placeholder in placeholders {
        // Add text before placeholder
        result.push_str(&template[last_end..placeholder.start]);

        // Get the value for this placeholder
        let value = match &placeholder.placeholder_type {
            PlaceholderType::Positional(index) => {
                if *index >= args.len() {
                    return Err(FormatError::MissingArgument(*index));
                }
                &args[*index]
            }
            PlaceholderType::Auto => {
                if auto_index >= args.len() {
                    return Err(FormatError::MissingArgument(auto_index));
                }
                let value = &args[auto_index];
                auto_index += 1;
                value
            }
            PlaceholderType::Named(name) => {
                context.variables.get(name)
                    .ok_or_else(|| FormatError::MissingArgument(0))?
            }

        // Format the value
        let formatted = format_value_with_spec(value, &placeholder.format_spec)?;
        result.push_str(&formatted);

        last_end = placeholder.end;
    // Add remaining text
    result.push_str(&template[last_end..]);

    Ok(result)
/// Simple argument formatting (like print functions)
/// Example: format_args(&[Value::String("Hello".to_string()), Value::Int(42)])
pub fn format_args(args: &[Value]) -> String {
    args.iter()
        .map(|arg| format_value_simple(arg))
        .collect::<Vec<_>>()
        .join(" ")
/// Interpolate variables in a string using ${variable} syntax
/// Example: interpolate("Hello ${name}!", &context)
pub fn interpolate(template: &str, context: &FormatContext) -> FormatResult<String> {
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '$' && chars.peek() == Some(&'{') {
            chars.next(); // consume '{'
            
            // Parse variable name
            let mut var_name = String::new();
            while let Some(ch) = chars.next() {
                if ch == '}' {
                    break;
                }
                var_name.push(ch);
            // Look up variable
            if let Some(value) = context.variables.get(&var_name) {
                result.push_str(&format_value_simple(value));
            } else {
                return Err(FormatError::InvalidContext(format!("Unknown variable: {}", var_name)));
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok(result)
/// Parse placeholders from a template string
fn parse_placeholders(template: &str) -> FormatResult<Vec<FormatPlaceholder>> {
    let mut placeholders = Vec::new();
    let mut chars = template.char_indices().peekable();
    
    while let Some((i, ch)) = chars.next() {
        if ch == '{' {
            if chars.peek().map(|(_, c)| *c) == Some('{') {
                // Escaped brace {{ -> skip
                chars.next();
                continue;
            // Find end of placeholder
            let start = i;
            let mut end = None;
            let mut placeholder_content = String::new();
            
            while let Some((j, ch)) = chars.next() {
                if ch == '}' {
                    end = Some(j + 1);
                    break;
                }
                placeholder_content.push(ch);
            let end = end.ok_or_else(|| {
                FormatError::InvalidPlaceholder("Unclosed placeholder".to_string())
            })?;
            
            // Parse placeholder content
            let placeholder = parse_placeholder_content(&placeholder_content, start, end)?;
            placeholders.push(placeholder);
        }
    }
    
    Ok(placeholders)
/// Parse the content of a placeholder
fn parse_placeholder_content(content: &str, start: usize, end: usize) -> FormatResult<FormatPlaceholder> {
    let original = format!("{{{}}}", content);
    
    if content.is_empty() {
        return Ok(FormatPlaceholder {
        });
    // Split on ':' to separate identifier from format spec
    let parts: Vec<&str> = content.splitn(2, ':').collect();
    let identifier = parts[0];
    let format_spec_str = parts.get(1).unwrap_or(&"");
    
    // Parse identifier
    let placeholder_type = if identifier.is_empty() {
        PlaceholderType::Auto
    } else if identifier.chars().all(|c| c.is_ascii_digit()) {
        PlaceholderType::Positional(identifier.parse().map_err(|_| {
            FormatError::InvalidPlaceholder(format!("Invalid index: {}", identifier))
        })?)
    } else {
        PlaceholderType::Named(identifier.to_string())
    
    // Parse format specification
    let format_spec = parse_format_spec(format_spec_str)?;
    
    Ok(FormatPlaceholder {
    })
/// Parse format specification string
fn parse_format_spec(spec: &str) -> FormatResult<FormatSpec> {
    if spec.is_empty() {
        return Ok(FormatSpec::default());
    let mut format_spec = FormatSpec::default();
    
    // Basic parsing - can be extended for more complex format specs
    if let Ok(width) = spec.parse::<usize>() {
        format_spec.width = Some(width);
    // TODO: Add more sophisticated format spec parsing
    // This is a simplified implementation
    
    Ok(format_spec)
/// Format a value with a format specification
fn format_value_with_spec(value: &Value, spec: &FormatSpec) -> FormatResult<String> {
    let mut formatted = format_value_simple(value);
    
    // Apply width formatting
    if let Some(width) = spec.width {
        if formatted.len() < width {
            let padding = width - formatted.len();
            match spec.alignment {
                Some(FormatAlignment::Left) => {
                    formatted.push_str(&" ".repeat(padding));
                }
                Some(FormatAlignment::Center) => {
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;
                    formatted = format!("{}{}{}", " ".repeat(left_pad), formatted, " ".repeat(right_pad));
                }
                Some(FormatAlignment::Right) | None => {
                    formatted = format!("{}{}", " ".repeat(padding), formatted);
                }
            }
        }
    }
    
    Ok(formatted)
/// Simple value formatting without format specifications
fn format_value_simple(value: &Value) -> String {
    match value {
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(format_value_simple).collect();
            format!("[{}]", items.join(", "))
        Value::Object(obj) => {
            let items: Vec<String> = obj.iter()
                .map(|(k, v)| format!("{}: {}", k, format_value_simple(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
    }
}

