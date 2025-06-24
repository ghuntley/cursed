/// Printf-style formatting functions for C-style compatibility
/// 
/// Provides sprintf, snprintf and related functions that are compatible
/// with C-style format strings for developers familiar with that syntax.

use crate::stdlib::value::Value;
use crate::error::Error;
use std::io::Write;
use std::fmt;

/// Sprintf error types
#[derive(Debug, Clone, PartialEq)]
pub enum SprintfError {
    InvalidFormatString(String),
    MissingArgument(usize),
    TooManyArguments,
    TypeMismatch(String),
    ConversionError(String),
    BufferOverflow,
    InvalidSpecifier(char),
    InvalidFlags(String),
}

impl fmt::Display for SprintfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SprintfError::InvalidFormatString(msg) => {
                write!(f, "Invalid format string: {}", msg)
            }
            SprintfError::MissingArgument(index) => {
                write!(f, "Missing argument at position: {}", index)
            }
            SprintfError::TooManyArguments => {
                write!(f, "Too many arguments provided")
            }
            SprintfError::TypeMismatch(msg) => {
                write!(f, "Type mismatch: {}", msg)
            }
            SprintfError::ConversionError(msg) => {
                write!(f, "Conversion error: {}", msg)
            }
            SprintfError::BufferOverflow => {
                write!(f, "Buffer overflow")
            }
            SprintfError::InvalidSpecifier(spec) => {
                write!(f, "Invalid format specifier: {}", spec)
            }
            SprintfError::InvalidFlags(flags) => {
                write!(f, "Invalid format flags: {}", flags)
            }
        }
    }
}

impl std::error::Error for SprintfError {}

pub type SprintfResult<T> = std::result::Result<T, SprintfError>;

/// Format specifier information
#[derive(Debug, Clone)]
pub struct FormatSpecifier {
    pub flags: String,
    pub width: Option<usize>,
    pub precision: Option<usize>,
    pub length_modifier: Option<char>,
    pub conversion: char,
    pub original: String,
    pub start: usize,
    pub end: usize,
}

/// Printf-style string formatting
/// Example: sprintf("Hello %s, you are %d years old", &[name, age])
pub fn sprintf(format_str: &str, args: &[Value]) -> SprintfResult<String> {
    let specifiers = parse_format_specifiers(format_str)?;
    let mut result = String::new();
    let mut last_end = 0;
    let mut arg_index = 0;

    for specifier in specifiers {
        // Add text before specifier
        result.push_str(&format_str[last_end..specifier.start]);

        // Check if we have enough arguments
        if arg_index >= args.len() {
            return Err(SprintfError::MissingArgument(arg_index));
        }

        // Format the argument according to the specifier
        let formatted = format_with_specifier(&args[arg_index], &specifier)?;
        result.push_str(&formatted);

        arg_index += 1;
        last_end = specifier.end;
    }

    // Add remaining text
    result.push_str(&format_str[last_end..]);

    // Check for unused arguments
    if arg_index < args.len() {
        return Err(SprintfError::TooManyArguments);
    }

    Ok(result)
}

/// Safe sprintf with buffer size limit
/// Example: snprintf(100, "Hello %s", &[name])
pub fn snprintf(max_len: usize, format_str: &str, args: &[Value]) -> SprintfResult<String> {
    let result = sprintf(format_str, args)?;
    
    if result.len() > max_len {
        return Err(SprintfError::BufferOverflow);
    }
    
    Ok(result)
}

/// Printf-style formatting to a writer
/// Example: sprintf_to_writer(&mut buffer, "Value: %d", &[value])
pub fn sprintf_to_writer<W: Write>(
    writer: &mut W, 
    format_str: &str, 
    args: &[Value]
) -> SprintfResult<()> {
    let formatted = sprintf(format_str, args)?;
    writer.write_all(formatted.as_bytes())
        .map_err(|e| SprintfError::ConversionError(e.to_string()))?;
    Ok(())
}

/// Validate a format string
/// Example: validate_format_string("Hello %s %d")
pub fn validate_format_string(format_str: &str) -> SprintfResult<()> {
    parse_format_specifiers(format_str)?;
    Ok(())
}

/// Count format specifiers in a format string
/// Example: count_format_specifiers("Hello %s, age %d") -> 2
pub fn count_format_specifiers(format_str: &str) -> SprintfResult<usize> {
    let specifiers = parse_format_specifiers(format_str)?;
    Ok(specifiers.len())
}

/// Parse format specifiers from a format string
fn parse_format_specifiers(format_str: &str) -> SprintfResult<Vec<FormatSpecifier>> {
    let mut specifiers = Vec::new();
    let mut chars = format_str.char_indices().peekable();

    while let Some((i, ch)) = chars.next() {
        if ch == '%' {
            // Check for escaped %
            if chars.peek().map(|(_, c)| *c) == Some('%') {
                chars.next(); // Skip escaped %
                continue;
            }

            let start = i;
            let mut flags = String::new();
            let mut width: Option<usize> = None;
            let mut precision: Option<usize> = None;
            let mut length_modifier: Option<char> = None;
            let mut conversion: Option<char> = None;
            let mut end_pos = start + 1;

            // Parse flags
            while let Some((j, ch)) = chars.peek() {
                match ch {
                    '-' | '+' | ' ' | '#' | '0' => {
                        flags.push(*ch);
                        chars.next();
                        end_pos = j + 1;
                    }
                    _ => break,
                }
            }

            // Parse width
            let mut width_str = String::new();
            while let Some((j, ch)) = chars.peek() {
                if ch.is_ascii_digit() {
                    width_str.push(*ch);
                    chars.next();
                    end_pos = j + 1;
                } else if *ch == '*' {
                    // Dynamic width - not implemented yet
                    chars.next();
                    end_pos = j + 1;
                    break;
                } else {
                    break;
                }
            }
            if !width_str.is_empty() {
                width = width_str.parse().ok();
            }

            // Parse precision
            if chars.peek().map(|(_, c)| *c) == Some('.') {
                chars.next(); // consume '.'
                if let Some((j, _)) = chars.peek() {
                    end_pos = j + 1;
                }

                let mut precision_str = String::new();
                while let Some((j, ch)) = chars.peek() {
                    if ch.is_ascii_digit() {
                        precision_str.push(*ch);
                        chars.next();
                        end_pos = j + 1;
                    } else if *ch == '*' {
                        // Dynamic precision - not implemented yet
                        chars.next();
                        end_pos = j + 1;
                        break;
                    } else {
                        break;
                    }
                }
                if !precision_str.is_empty() {
                    precision = precision_str.parse().ok();
                } else {
                    precision = Some(0); // .0 means zero precision
                }
            }

            // Parse length modifier
            if let Some((j, ch)) = chars.peek() {
                match ch {
                    'h' | 'l' | 'L' | 'z' | 'j' | 't' => {
                        length_modifier = Some(*ch);
                        chars.next();
                        end_pos = j + 1;
                    }
                    _ => {}
                }
            }

            // Parse conversion specifier
            if let Some((j, ch)) = chars.next() {
                end_pos = j + 1;
                match ch {
                    'd' | 'i' | 'o' | 'x' | 'X' | 'u' | 'c' |
                    'f' | 'F' | 'e' | 'E' | 'g' | 'G' |
                    's' | 'p' | 'n' => {
                        conversion = Some(ch);
                    }
                    _ => {
                        return Err(SprintfError::InvalidSpecifier(ch));
                    }
                }
            } else {
                return Err(SprintfError::InvalidFormatString(
                    "Incomplete format specifier".to_string()
                ));
            }

            if let Some(conv) = conversion {
                let original = format_str[start..end_pos].to_string();
                specifiers.push(FormatSpecifier {
                    flags,
                    width,
                    precision,
                    length_modifier,
                    conversion: conv,
                    original,
                    start,
                    end: end_pos,
                });
            }
        }
    }

    Ok(specifiers)
}

/// Format a value according to a format specifier
fn format_with_specifier(value: &Value, spec: &FormatSpecifier) -> SprintfResult<String> {
    let base_formatted = match spec.conversion {
        'd' | 'i' => format_integer(value, 10, false)?,
        'o' => format_integer(value, 8, false)?,
        'x' => format_integer(value, 16, false)?,
        'X' => format_integer(value, 16, true)?,
        'u' => format_unsigned_integer(value)?,
        'c' => format_character(value)?,
        'f' | 'F' => format_float(value, spec.precision.unwrap_or(6), false)?,
        'e' => format_float_exponential(value, spec.precision.unwrap_or(6), false)?,
        'E' => format_float_exponential(value, spec.precision.unwrap_or(6), true)?,
        'g' => format_float_general(value, spec.precision.unwrap_or(6), false)?,
        'G' => format_float_general(value, spec.precision.unwrap_or(6), true)?,
        's' => format_string(value)?,
        'p' => format_pointer(value)?,
        _ => {
            return Err(SprintfError::InvalidSpecifier(spec.conversion));
        }
    };

    // Apply width and alignment
    let mut result = base_formatted;
    if let Some(width) = spec.width {
        if result.len() < width {
            let padding = width - result.len();
            let fill_char = if spec.flags.contains('0') && 
                           !spec.flags.contains('-') &&
                           matches!(spec.conversion, 'd' | 'i' | 'o' | 'x' | 'X' | 'u' | 'f' | 'F' | 'e' | 'E' | 'g' | 'G') {
                '0'
            } else {
                ' '
            };

            if spec.flags.contains('-') {
                // Left-align
                result.push_str(&fill_char.to_string().repeat(padding));
            } else {
                // Right-align
                result = format!("{}{}", fill_char.to_string().repeat(padding), result);
            }
        }
    }

    Ok(result)
}

/// Format integer value
fn format_integer(value: &Value, base: u32, uppercase: bool) -> SprintfResult<String> {
    match value {
        Value::Int(i) => {
            let result = if base == 10 {
                i.to_string()
            } else {
                format!("{:x}", i.abs())
            };
            
            let formatted = if uppercase {
                result.to_uppercase()
            } else {
                result
            };
            
            Ok(if *i < 0 && base != 10 {
                format!("-{}", formatted)
            } else {
                formatted
            })
        }
        Value::Float(f) => {
            format_integer(&Value::Int(*f as i64), base, uppercase)
        }
        _ => Err(SprintfError::TypeMismatch(
            format!("Expected number for integer format, got {:?}", value)
        ))
    }
}

/// Format unsigned integer value
fn format_unsigned_integer(value: &Value) -> SprintfResult<String> {
    match value {
        Value::Int(i) => Ok((*i as u64).to_string()),
        Value::Float(f) => Ok((*f as u64).to_string()),
        _ => Err(SprintfError::TypeMismatch(
            format!("Expected number for unsigned format, got {:?}", value)
        ))
    }
}

/// Format character value
fn format_character(value: &Value) -> SprintfResult<String> {
    match value {
        Value::Int(i) => {
            if *i >= 0 && *i <= 127 {
                Ok((*i as u8 as char).to_string())
            } else {
                Err(SprintfError::ConversionError(
                    format!("Integer {} out of ASCII range", i)
                ))
            }
        }
        Value::String(s) => {
            if s.len() == 1 {
                Ok(s.clone())
            } else {
                Err(SprintfError::ConversionError(
                    "String too long for character format".to_string()
                ))
            }
        }
        _ => Err(SprintfError::TypeMismatch(
            format!("Expected integer or single character for char format, got {:?}", value)
        ))
    }
}

/// Format floating point value
fn format_float(value: &Value, precision: usize, _uppercase: bool) -> SprintfResult<String> {
    match value {
        Value::Float(f) => Ok(format!("{:.prec$}", f, prec = precision)),
        Value::Int(i) => Ok(format!("{:.prec$}", *i as f64, prec = precision)),
        _ => Err(SprintfError::TypeMismatch(
            format!("Expected number for float format, got {:?}", value)
        ))
    }
}

/// Format floating point in exponential notation
fn format_float_exponential(value: &Value, precision: usize, uppercase: bool) -> SprintfResult<String> {
    match value {
        Value::Float(f) => {
            let result = format!("{:.prec$e}", f, prec = precision);
            Ok(if uppercase { result.to_uppercase() } else { result })
        }
        Value::Int(i) => {
            let result = format!("{:.prec$e}", *i as f64, prec = precision);
            Ok(if uppercase { result.to_uppercase() } else { result })
        }
        _ => Err(SprintfError::TypeMismatch(
            format!("Expected number for exponential format, got {:?}", value)
        ))
    }
}

/// Format floating point in general format
fn format_float_general(value: &Value, precision: usize, uppercase: bool) -> SprintfResult<String> {
    match value {
        Value::Float(f) => {
            let result = format!("{:.prec$}", f, prec = precision);
            Ok(if uppercase { result.to_uppercase() } else { result })
        }
        Value::Int(i) => {
            let result = format!("{:.prec$}", *i as f64, prec = precision);
            Ok(if uppercase { result.to_uppercase() } else { result })
        }
        _ => Err(SprintfError::TypeMismatch(
            format!("Expected number for general format, got {:?}", value)
        ))
    }
}

/// Format string value
fn format_string(value: &Value) -> SprintfResult<String> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Nil => Ok("(null)".to_string()),
        _ => Ok(format!("{:?}", value)) // Debug format for non-strings
    }
}

/// Format pointer value
fn format_pointer(value: &Value) -> SprintfResult<String> {
    match value {
        Value::Nil => Ok("(nil)".to_string()),
        _ => Ok(format!("0x{:x}", value as *const Value as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprintf_string() {
        let args = vec![Value::String("World".to_string())];
        let result = sprintf("Hello %s", &args).unwrap();
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_sprintf_integer() {
        let args = vec![Value::Int(42)];
        let result = sprintf("The answer is %d", &args).unwrap();
        assert_eq!(result, "The answer is 42");
    }

    #[test]
    fn test_sprintf_float() {
        let args = vec![Value::Float(3.14159)];
        let result = sprintf("Pi is %.2f", &args).unwrap();
        assert_eq!(result, "Pi is 3.14");
    }

    #[test]
    fn test_sprintf_multiple() {
        let args = vec![
            Value::String("John".to_string()),
            Value::Int(25),
            Value::Float(180.5)
        ];
        let result = sprintf("%s is %d years old and %.1f cm tall", &args).unwrap();
        assert_eq!(result, "John is 25 years old and 180.5 cm tall");
    }

    #[test]
    fn test_sprintf_hex() {
        let args = vec![Value::Int(255)];
        let result = sprintf("0x%x", &args).unwrap();
        assert_eq!(result, "0xff");
        
        let result = sprintf("0x%X", &args).unwrap();
        assert_eq!(result, "0xFF");
    }

    #[test]
    fn test_sprintf_width() {
        let args = vec![Value::Int(42)];
        let result = sprintf("%5d", &args).unwrap();
        assert_eq!(result, "   42");
        
        let result = sprintf("%-5d", &args).unwrap();
        assert_eq!(result, "42   ");
    }

    #[test]
    fn test_sprintf_zero_padding() {
        let args = vec![Value::Int(42)];
        let result = sprintf("%05d", &args).unwrap();
        assert_eq!(result, "00042");
    }

    #[test]
    fn test_sprintf_missing_argument() {
        let args = vec![Value::String("Hello".to_string())];
        let result = sprintf("Hello %s %d", &args);
        assert!(matches!(result, Err(SprintfError::MissingArgument(1))));
    }

    #[test]
    fn test_sprintf_too_many_arguments() {
        let args = vec![
            Value::String("Hello".to_string()),
            Value::Int(42)
        ];
        let result = sprintf("Hello %s", &args);
        assert!(matches!(result, Err(SprintfError::TooManyArguments)));
    }

    #[test]
    fn test_count_format_specifiers() {
        assert_eq!(count_format_specifiers("Hello %s, age %d").unwrap(), 2);
        assert_eq!(count_format_specifiers("No specifiers").unwrap(), 0);
        assert_eq!(count_format_specifiers("%%escaped%%").unwrap(), 0);
    }

    #[test]
    fn test_snprintf_overflow() {
        let args = vec![Value::String("Very long string".to_string())];
        let result = snprintf(5, "Hello %s", &args);
        assert!(matches!(result, Err(SprintfError::BufferOverflow)));
    }
}
