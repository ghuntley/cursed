/// Basic console I/O operations for CURSED
use crate::stdlib::{
    io::{
        error::{IoError, IoResult},
        streams::{stdin, stdout, stderr}
    },
    value::Value
};

/// Print message to stdout without newline
pub fn print(msg: &str) -> IoResult<()> {
    stdout().print(msg)
}

/// Print message to stdout with newline
pub fn println(msg: &str) -> IoResult<()> {
    stdout().println(msg)
}

/// Print message to stderr without newline
pub fn eprint(msg: &str) -> IoResult<()> {
    stderr().eprint(msg)
}

/// Print message to stderr with newline
pub fn eprintln(msg: &str) -> IoResult<()> {
    stderr().eprintln(msg)
}

/// Formatted printing to stdout
pub fn printf(format: &str, args: &[Value]) -> IoResult<()> {
    let formatted = format_string(format, args)?;
    print(&formatted)
}

/// Formatted printing to stdout with newline
pub fn printfln(format: &str, args: &[Value]) -> IoResult<()> {
    let formatted = format_string(format, args)?;
    println(&formatted)
}

/// Formatted printing to stderr
pub fn eprintf(format: &str, args: &[Value]) -> IoResult<()> {
    let formatted = format_string(format, args)?;
    eprint(&formatted)
}

/// Formatted printing to stderr with newline
pub fn eprintfln(format: &str, args: &[Value]) -> IoResult<()> {
    let formatted = format_string(format, args)?;
    eprintln(&formatted)
}

/// Read a line from stdin
pub fn read_line() -> IoResult<String> {
    stdin().read_line()
}

/// Read a single character from stdin
pub fn read_char() -> IoResult<char> {
    let line = stdin().read_line()?;
    line.chars().next()
        .ok_or_else(|| IoError::UnexpectedEof)
}

/// Read until a specific delimiter character
pub fn read_until(delimiter: char) -> IoResult<String> {
    let delimiter_byte = if delimiter.is_ascii() {
        delimiter as u8
    } else {
        return Err(IoError::InvalidInput("Delimiter must be ASCII character".to_string()));
    };
    
    stdin().read_until(delimiter_byte)
}

/// Read all input until EOF
pub fn read_all() -> IoResult<String> {
    stdin().read_all()
}

/// Flush all output buffers
pub fn flush() -> IoResult<()> {
    stdout().flush()?;
    stderr().flush()?;
    Ok(())
}

/// Format string with arguments (simple placeholder replacement)
fn format_string(format: &str, args: &[Value]) -> IoResult<String> {
    let mut result = String::new();
    let mut chars = format.chars().peekable();
    let mut arg_index = 0;

    while let Some(ch) = chars.next() {
        if ch == '{' {
            if chars.peek() == Some(&'{') {
                // Escaped brace
                chars.next();
                result.push('{');
            } else {
                // Placeholder
                let mut placeholder = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '}' {
                        chars.next();
                        break;
                    }
                    placeholder.push(chars.next().unwrap());
                }

                if placeholder.is_empty() {
                    // Positional argument
                    if arg_index < args.len() {
                        result.push_str(&format_value(&args[arg_index]));
                        arg_index += 1;
                    } else {
                        return Err(IoError::InvalidInput("Not enough arguments for format string".to_string()));
                    }
                } else {
                    // Named or indexed placeholder - simplified implementation
                    if let Ok(index) = placeholder.parse::<usize>() {
                        if index < args.len() {
                            result.push_str(&format_value(&args[index]));
                        } else {
                            return Err(IoError::InvalidInput(format!("Argument index {} out of bounds", index)));
                        }
                    } else {
                        return Err(IoError::InvalidInput(format!("Unsupported placeholder: {}", placeholder)));
                    }
                }
            }
        } else if ch == '}' {
            if chars.peek() == Some(&'}') {
                // Escaped brace
                chars.next();
                result.push('}');
            } else {
                return Err(IoError::InvalidInput("Unmatched '}' in format string".to_string()));
            }
        } else {
            result.push(ch);
        }
    }

    Ok(result)
}

/// Convert Value to string representation for formatting
fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Integer(i) => i.to_string(),
        Value::Number(f) => f.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(format_value).collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Object(obj) => {
            let entries: Vec<String> = obj.iter()
                .map(|(k, v)| format!("{}: {}", k, format_value(v)))
                .collect();
            format!("{{{}}}", entries.join(", "))
        }
        Value::Bytes(bytes) => {
            format!("Bytes({} bytes)", bytes.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_format_string_basic() {
        let args = vec![Value::String("world".to_string())];
        let result = format_string("Hello, {}!", &args).unwrap();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_format_string_multiple_args() {
        let args = vec![
            Value::String("John".to_string()),
            Value::Integer(25),
        ];
        let result = format_string("Name: {}, Age: {}", &args).unwrap();
        assert_eq!(result, "Name: John, Age: 25");
    }

    #[test]
    fn test_format_string_indexed() {
        let args = vec![
            Value::String("first".to_string()),
            Value::String("second".to_string()),
        ];
        let result = format_string("{1} comes after {0}", &args).unwrap();
        assert_eq!(result, "second comes after first");
    }

    #[test]
    fn test_format_string_escaped_braces() {
        let args = vec![Value::String("test".to_string())];
        let result = format_string("{{{}}} is in braces", &args).unwrap();
        assert_eq!(result, "{test} is in braces");
    }

    #[test]
    fn test_format_value_types() {
        assert_eq!(format_value(&Value::Bool(true)), "true");
        assert_eq!(format_value(&Value::Number(3.14)), "3.14");
        assert_eq!(format_value(&Value::Null), "null");
    }

    #[test]
    fn test_format_value_array() {
        let arr = vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)];
        let result = format_value(&Value::Array(arr));
        assert_eq!(result, "[1, 2, 3]");
    }

    #[test]
    fn test_format_value_object() {
        let mut obj = HashMap::new();
        obj.insert("name".to_string(), Value::String("John".to_string()));
        obj.insert("age".to_string(), Value::Integer(30));
        let result = format_value(&Value::Object(obj));
        // Note: HashMap iteration order is not guaranteed, so we check both possibilities
        assert!(result.contains("name: John") && result.contains("age: 30"));
    }
}
