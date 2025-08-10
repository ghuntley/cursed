//! Format module for vibez - Advanced string formatting for CURSED

use crate::error_types::CursedError;
use crate::runtime::value::Value;
use std::collections::HashMap;
use std::fmt::Write;

/// Format specification for placeholders
#[derive(Debug, Clone, PartialEq)]
pub struct FormatSpec {
    pub placeholder_type: PlaceholderType,
    pub alignment: FormatAlignment,
    pub sign: FormatSign,
    pub width: Option<usize>,
    pub precision: Option<usize>,
    pub fill_char: char,
}

/// Types of format placeholders
#[derive(Debug, Clone, PartialEq)]
pub enum PlaceholderType {
    /// Default formatting
    Default,
    /// Debug formatting (:?)
    Debug,
    /// Hexadecimal (:x, :X)
    Hex(bool), // true for uppercase
    /// Binary (:b)
    Binary,
    /// Octal (:o)
    Octal,
    /// Scientific notation (:e, :E)
    Scientific(bool), // true for uppercase
    /// Fixed point (:f)
    Fixed,
    /// General number format (:g, :G)
    General(bool), // true for uppercase
}

/// Format alignment options
#[derive(Debug, Clone, PartialEq)]
pub enum FormatAlignment {
    /// No specific alignment
    None,
    /// Left align (<)
    Left,
    /// Right align (>)
    Right,
    /// Center align (^)
    Center,
}

/// Format sign options
#[derive(Debug, Clone, PartialEq)]
pub enum FormatSign {
    /// Default sign behavior
    Default,
    /// Always show sign (+)
    Always,
    /// Use space for positive numbers ( )
    Space,
}

/// A parsed format placeholder
#[derive(Debug, Clone)]
pub struct FormatPlaceholder {
    pub start: usize,
    pub end: usize,
    pub spec: FormatSpec,
    pub arg_index: Option<usize>,
    pub arg_name: Option<String>,
}

impl Default for FormatSpec {
    fn default() -> Self {
        Self {
            placeholder_type: PlaceholderType::Default,
            alignment: FormatAlignment::None,
            sign: FormatSign::Default,
            width: None,
            precision: None,
            fill_char: ' ',
        }
    }
}

/// Parse format string and extract placeholders
pub fn parse_format_string(format: &str) -> Result<(String, Vec<FormatPlaceholder>), CursedError> {
    let mut result = String::new();
    let mut placeholders = Vec::new();
    let mut chars = format.char_indices().peekable();
    
    while let Some((i, ch)) = chars.next() {
        if ch == '{' {
            if chars.peek().map(|(_, c)| *c) == Some('{') {
                // Escaped brace
                chars.next();
                result.push('{');
            } else {
                // Start of placeholder
                let start = i;
                let mut spec_str = String::new();
                let mut brace_count = 1;
                
                while let Some((j, ch)) = chars.next() {
                    if ch == '{' {
                        brace_count += 1;
                        spec_str.push(ch);
                    } else if ch == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            let end = j + 1;
                            let placeholder = parse_placeholder_spec(&spec_str)?;
                            placeholders.push(FormatPlaceholder {
                                start,
                                end,
                                spec: placeholder.0,
                                arg_index: placeholder.1,
                                arg_name: placeholder.2,
                            });
                            result.push_str("{}"); // Standard placeholder for later formatting
                            break;
                        } else {
                            spec_str.push(ch);
                        }
                    } else {
                        spec_str.push(ch);
                    }
                }
                
                if brace_count != 0 {
                    return Err(CursedError::Runtime("Unmatched brace in format string".to_string()));
                }
            }
        } else if ch == '}' {
            if chars.peek().map(|(_, c)| *c) == Some('}') {
                // Escaped brace
                chars.next();
                result.push('}');
            } else {
                return Err(CursedError::Runtime("Unmatched '}' in format string".to_string()));
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok((result, placeholders))
}

/// Parse a single placeholder specification
fn parse_placeholder_spec(spec: &str) -> Result<(FormatSpec, Option<usize>, Option<String>), CursedError> {
    if spec.is_empty() {
        return Ok((FormatSpec::default(), None, None));
    }
    
    let mut format_spec = FormatSpec::default();
    let mut arg_index = None;
    let mut arg_name = None;
    
    // Parse arg identifier (number or name)
    let parts: Vec<&str> = spec.splitn(2, ':').collect();
    let (identifier, format_part) = if parts.len() == 2 {
        (parts[0], Some(parts[1]))
    } else {
        (parts[0], None)
    };
    
    // Parse argument identifier
    if !identifier.is_empty() {
        if let Ok(index) = identifier.parse::<usize>() {
            arg_index = Some(index);
        } else if identifier.chars().all(|c| c.is_alphabetic() || c == '_') {
            arg_name = Some(identifier.to_string());
        } else {
            return Err(CursedError::Runtime(format!("Invalid argument identifier: {}", identifier)));
        }
    }
    
    // Parse format specification
    if let Some(fmt) = format_part {
        parse_format_spec(fmt, &mut format_spec)?;
    }
    
    Ok((format_spec, arg_index, arg_name))
}

/// Parse format specification part
fn parse_format_spec(spec: &str, format_spec: &mut FormatSpec) -> Result<(), CursedError> {
    let chars_vec: Vec<char> = spec.chars().collect();
    let mut i = 0;
    
    // Parse fill and alignment
    if let Some(&first) = chars_vec.get(i) {
        let second = chars_vec.get(i + 1).copied();
        
        match second {
            Some('<') => {
                format_spec.fill_char = first;
                format_spec.alignment = FormatAlignment::Left;
                i += 2; // consume fill char and alignment char
            },
            Some('>') => {
                format_spec.fill_char = first;
                format_spec.alignment = FormatAlignment::Right;
                i += 2; // consume fill char and alignment char
            },
            Some('^') => {
                format_spec.fill_char = first;
                format_spec.alignment = FormatAlignment::Center;
                i += 2; // consume fill char and alignment char
            },
            _ => {
                // Check for alignment without fill char
                match first {
                    '<' => {
                        format_spec.alignment = FormatAlignment::Left;
                        i += 1;
                    },
                    '>' => {
                        format_spec.alignment = FormatAlignment::Right;
                        i += 1;
                    },
                    '^' => {
                        format_spec.alignment = FormatAlignment::Center;
                        i += 1;
                    },
                    _ => {}
                }
            }
        }
    }
    
    // Parse sign
    if let Some(&ch) = chars_vec.get(i) {
        match ch {
            '+' => {
                format_spec.sign = FormatSign::Always;
                i += 1;
            },
            ' ' => {
                format_spec.sign = FormatSign::Space;
                i += 1;
            },
            _ => {}
        }
    }
    
    // Parse width
    let mut width_str = String::new();
    while let Some(&ch) = chars_vec.get(i) {
        if ch.is_ascii_digit() {
            width_str.push(ch);
            i += 1;
        } else {
            break;
        }
    }
    if !width_str.is_empty() {
        format_spec.width = Some(width_str.parse().map_err(|_| {
            CursedError::Runtime("Invalid width in format spec".to_string())
        })?);
    }
    
    // Parse precision
    if chars_vec.get(i) == Some(&'.') {
        i += 1; // consume '.'
        let mut precision_str = String::new();
        while let Some(&ch) = chars_vec.get(i) {
            if ch.is_ascii_digit() {
                precision_str.push(ch);
                i += 1;
            } else {
                break;
            }
        }
        if !precision_str.is_empty() {
            format_spec.precision = Some(precision_str.parse().map_err(|_| {
                CursedError::Runtime("Invalid precision in format spec".to_string())
            })?);
        }
    }
    
    // Parse type
    if let Some(&type_char) = chars_vec.get(i) {
        format_spec.placeholder_type = match type_char {
            '?' => PlaceholderType::Debug,
            'x' => PlaceholderType::Hex(false),
            'X' => PlaceholderType::Hex(true),
            'b' => PlaceholderType::Binary,
            'o' => PlaceholderType::Octal,
            'e' => PlaceholderType::Scientific(false),
            'E' => PlaceholderType::Scientific(true),
            'f' => PlaceholderType::Fixed,
            'g' => PlaceholderType::General(false),
            'G' => PlaceholderType::General(true),
            _ => return Err(CursedError::Runtime(format!("Invalid format type: {}", type_char))),
        };
    }
    
    Ok(())
}

/// Format a value according to format specification
pub fn format_value_with_spec(value: &Value, spec: &FormatSpec) -> Result<String, CursedError> {
    let mut formatted = match (&spec.placeholder_type, value) {
        (PlaceholderType::Default, _) => format_value_default(value),
        (PlaceholderType::Debug, _) => format!("{:?}", value),
        (PlaceholderType::Hex(uppercase), Value::Integer(i)) => {
            if *uppercase {
                format!("{:X}", i)
            } else {
                format!("{:x}", i)
            }
        },
        (PlaceholderType::Binary, Value::Integer(i)) => format!("{:b}", i),
        (PlaceholderType::Octal, Value::Integer(i)) => format!("{:o}", i),
        (PlaceholderType::Scientific(uppercase), Value::Number(f)) => {
            if *uppercase {
                format!("{:E}", f)
            } else {
                format!("{:e}", f)
            }
        },
        (PlaceholderType::Fixed, Value::Number(f)) => {
            if let Some(precision) = spec.precision {
                format!("{:.1$}", f, precision)
            } else {
                format!("{}", f)
            }
        },
        (PlaceholderType::General(uppercase), Value::Number(f)) => {
            // Rust doesn't have :g/:G format, so we'll use scientific notation as fallback
            if *uppercase {
                format!("{:E}", f)
            } else {
                format!("{:e}", f)
            }
        },
        _ => format_value_default(value),
    };
    
    // Apply sign formatting for numbers
    if matches!(value, Value::Integer(_) | Value::Number(_)) {
        match spec.sign {
            FormatSign::Always => {
                if !formatted.starts_with('-') && !formatted.starts_with('+') {
                    formatted = format!("+{}", formatted);
                }
            },
            FormatSign::Space => {
                if !formatted.starts_with('-') && !formatted.starts_with('+') {
                    formatted = format!(" {}", formatted);
                }
            },
            FormatSign::Default => {}
        }
    }
    
    // Apply width and alignment
    if let Some(width) = spec.width {
        if formatted.len() < width {
            let padding = width - formatted.len();
            match spec.alignment {
                FormatAlignment::Left => {
                    formatted.push_str(&spec.fill_char.to_string().repeat(padding));
                },
                FormatAlignment::Right | FormatAlignment::None => {
                    formatted = format!("{}{}", spec.fill_char.to_string().repeat(padding), formatted);
                },
                FormatAlignment::Center => {
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;
                    formatted = format!("{}{}{}",
                        spec.fill_char.to_string().repeat(left_pad),
                        formatted,
                        spec.fill_char.to_string().repeat(right_pad)
                    );
                },
            }
        }
    }
    
    Ok(formatted)
}

/// Default value formatting
fn format_value_default(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Number(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(format_value_default).collect();
            format!("[{}]", elements.join(", "))
        },
        Value::Object(obj) => {
            let entries: Vec<String> = obj.iter()
                .map(|(k, v)| format!("{}: {}", k, format_value_default(v)))
                .collect();
            format!("{{{}}}", entries.join(", "))
        },
        Value::Binary(data) => format!("<binary: {} bytes>", data.len()),
        Value::Function { name, arity } => format!("<function {}({} args)>", name, arity),
        Value::Interface { .. } => "<interface>".to_string(),
    }
}
