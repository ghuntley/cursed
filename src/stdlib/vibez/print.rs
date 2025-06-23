/// Core printing functions for CURSED with Gen Z flair
/// 
/// Provides basic printing functionality that integrates with the CURSED
/// type system and supports styled/colored output. Includes the essential
/// `spill` functions for Gen Z-style output and input operations.

use crate::stdlib::value::Value;
use std::io::{self, Write, stdout, stderr, stdin, BufRead};
use std::fmt;

/// Print arguments to stdout without a newline
/// Example: print("Hello", " ", "World")
pub fn print(args: &[Value]) -> io::Result<()> {
    let mut output = stdout();
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            write!(output, " ")?;
        }
        write!(output, "{}", format_value(arg))?;
    }
    output.flush()
}

/// Print arguments to stdout with a newline
/// Example: println("Hello World", 42)
pub fn println(args: &[Value]) -> io::Result<()> {
    print(args)?;
    println!();
    Ok(())
}

/// Print arguments to stderr without a newline
/// Example: eprint("Error:", error_msg)
pub fn eprint(args: &[Value]) -> io::Result<()> {
    let mut output = stderr();
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            write!(output, " ")?;
        }
        write!(output, "{}", format_value(arg))?;
    }
    output.flush()
}

/// Print arguments to stderr with a newline
/// Example: eprintln("Fatal error occurred")
pub fn eprintln(args: &[Value]) -> io::Result<()> {
    eprint(args)?;
    eprintln!();
    Ok(())
}

/// Print to a specific writer
/// Example: print_to(&mut buffer, &[Value::String("test".to_string())])
pub fn print_to<W: Write>(writer: &mut W, args: &[Value]) -> io::Result<()> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            write!(writer, " ")?;
        }
        write!(writer, "{}", format_value(arg))?;
    }
    writer.flush()
}

/// Print to a specific writer with newline
/// Example: println_to(&mut buffer, &[Value::Int(42)])
pub fn println_to<W: Write>(writer: &mut W, args: &[Value]) -> io::Result<()> {
    print_to(writer, args)?;
    writeln!(writer)?;
    Ok(())
}

/// Print styling options
#[derive(Debug, Clone)]
pub enum PrintStyle {
    Normal,
    Bold,
    Italic,
    Underline,
    Strikethrough,
}

/// Print color options
#[derive(Debug, Clone)]
pub enum PrintColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

/// Print with styling
/// Example: print_styled(&[Value::String("Important".to_string())], PrintStyle::Bold)
pub fn print_styled(args: &[Value], style: PrintStyle) -> io::Result<()> {
    let style_code = match style {
        PrintStyle::Normal => "",
        PrintStyle::Bold => "\x1b[1m",
        PrintStyle::Italic => "\x1b[3m",
        PrintStyle::Underline => "\x1b[4m",
        PrintStyle::Strikethrough => "\x1b[9m",
    };
    
    let reset_code = if matches!(style, PrintStyle::Normal) { "" } else { "\x1b[0m" };
    
    print!("{}", style_code);
    print(args)?;
    print!("{}", reset_code);
    
    Ok(())
}

/// Print with styling and newline
/// Example: println_styled(&[Value::String("Success!".to_string())], PrintStyle::Bold)
pub fn println_styled(args: &[Value], style: PrintStyle) -> io::Result<()> {
    print_styled(args, style)?;
    println!();
    Ok(())
}

/// Print with color
/// Example: print_colored(&[Value::String("Error".to_string())], PrintColor::Red)
pub fn print_colored(args: &[Value], color: PrintColor) -> io::Result<()> {
    let color_code = match color {
        PrintColor::Default => "",
        PrintColor::Black => "\x1b[30m",
        PrintColor::Red => "\x1b[31m",
        PrintColor::Green => "\x1b[32m",
        PrintColor::Yellow => "\x1b[33m",
        PrintColor::Blue => "\x1b[34m",
        PrintColor::Magenta => "\x1b[35m",
        PrintColor::Cyan => "\x1b[36m",
        PrintColor::White => "\x1b[37m",
        PrintColor::BrightRed => "\x1b[91m",
        PrintColor::BrightGreen => "\x1b[92m",
        PrintColor::BrightYellow => "\x1b[93m",
        PrintColor::BrightBlue => "\x1b[94m",
        PrintColor::BrightMagenta => "\x1b[95m",
        PrintColor::BrightCyan => "\x1b[96m",
        PrintColor::BrightWhite => "\x1b[97m",
    };
    
    let reset_code = if matches!(color, PrintColor::Default) { "" } else { "\x1b[0m" };
    
    print!("{}", color_code);
    print(args)?;
    print!("{}", reset_code);
    
    Ok(())
}

/// Print with color and newline
/// Example: println_colored(&[Value::String("Success!".to_string())], PrintColor::Green)
pub fn println_colored(args: &[Value], color: PrintColor) -> io::Result<()> {
    print_colored(args, color)?;
    println!();
    Ok(())
}

// ================================
// SPILL FUNCTIONS (GEN Z OUTPUT/INPUT VIBES)
// ================================

/// Spill args followed by newline (the main spill vibes) 
/// This is the core Gen Z output function equivalent to Go's `fmt.Println`
/// 
/// # Examples
/// ```cursed
/// spill("Hello", "World", 42) // Output: "Hello World 42\n"
/// spill() // Output: "\n" (just newline)
/// ```
pub fn spill(args: &[Value]) -> io::Result<()> {
    if args.is_empty() {
        println!();
        return Ok(());
    }
    
    let mut output = stdout();
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            write!(output, " ")?;
        }
        write!(output, "{}", format_value(arg))?;
    }
    writeln!(output)?;
    output.flush()
}

/// Spill with format string (formatted spill vibes)
/// Equivalent to Go's `fmt.Printf` but with Gen Z naming
/// 
/// # Examples  
/// ```cursed
/// spillf("Hello %s, you are %d years old", "Alice", 25)
/// spillf("Progress: %.2f%%", 85.67)
/// ```
pub fn spillf(format: &str, args: &[Value]) -> io::Result<()> {
    let formatted = format_with_args(format, args)?;
    print!("{}", formatted);
    stdout().flush()
}

/// Spill and return formatted string (string spill vibes)
/// Equivalent to Go's `fmt.Sprintf` with Gen Z naming
/// 
/// # Examples
/// ```cursed
/// let result = spillstr("Hello %s!", "World") // "Hello World!"
/// let msg = spillstr("Count: %d", 42) // "Count: 42"
/// ```
pub fn spillstr(format: &str, args: &[Value]) -> io::Result<String> {
    format_with_args(format, args)
}

/// Scan input into args (input scan vibes)
/// Equivalent to Go's `fmt.Scan` with Gen Z naming
/// 
/// # Examples
/// ```cursed
/// let mut name = Value::String(String::new());
/// let mut age = Value::Int(0);
/// scan(&mut [&mut name, &mut age]) // Reads "Alice 25" -> name="Alice", age=25
/// ```
pub fn scan(args: &mut [&mut Value]) -> io::Result<usize> {
    let input = read_line_input()?;
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    let mut scanned = 0;
    for (i, arg) in args.iter_mut().enumerate() {
        if i >= parts.len() {
            break;
        }
        
        if let Ok(parsed) = parse_value_from_str(parts[i], arg) {
            **arg = parsed;
            scanned += 1;
        } else {
            break;
        }
    }
    
    Ok(scanned)
}

/// Scan line into args (line scan vibes)
/// Equivalent to Go's `fmt.Scanln` with Gen Z naming
/// 
/// # Examples
/// ```cursed
/// let mut input = Value::String(String::new());
/// scanln(&mut [&mut input]) // Reads entire line
/// ```
pub fn scanln(args: &mut [&mut Value]) -> io::Result<usize> {
    let input = read_line_input()?;
    
    if args.is_empty() {
        return Ok(0);
    }
    
    // For scanln, if there's only one arg, give it the whole line
    if args.len() == 1 {
        *args[0] = Value::String(input);
        return Ok(1);
    }
    
    // Otherwise, split by whitespace like scan
    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut scanned = 0;
    
    for (i, arg) in args.iter_mut().enumerate() {
        if i >= parts.len() {
            break;
        }
        
        if let Ok(parsed) = parse_value_from_str(parts[i], arg) {
            **arg = parsed;
            scanned += 1;
        } else {
            break;
        }
    }
    
    Ok(scanned)
}

// ================================
// HELPER FUNCTIONS FOR SPILL OPERATIONS
// ================================

/// Format string with arguments using printf-style formatting
fn format_with_args(format: &str, args: &[Value]) -> io::Result<String> {
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
                    continue;
                }
                
                // Parse format specifier
                let mut spec = String::new();
                spec.push('%');
                
                // Skip the %
                while let Some(&next_ch) = chars.peek() {
                    let consumed = chars.next().unwrap();
                    spec.push(consumed);
                    
                    // Format specifier ends at these characters
                    if "diouxXeEfFgGaAcspv".contains(consumed) {
                        break;
                    }
                }
                
                // Format the argument
                if arg_index < args.len() {
                    let formatted = format_arg_with_spec(&spec, &args[arg_index])?;
                    result.push_str(&formatted);
                    arg_index += 1;
                } else {
                    // Not enough arguments, just append the spec
                    result.push_str(&spec);
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok(result)
}

/// Format a single argument with a format specifier
fn format_arg_with_spec(spec: &str, arg: &Value) -> io::Result<String> {
    let last_char = spec.chars().last().unwrap_or('s');
    
    match last_char {
        'd' | 'i' => {
            match arg {
                Value::Int(i) => Ok(i.to_string()),
                Value::Float(f) => Ok((*f as i64).to_string()),
                Value::String(s) => s.parse::<i64>()
                    .map(|i| i.to_string())
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid integer")),
                _ => Ok("0".to_string()),
            }
        },
        'f' | 'F' => {
            match arg {
                Value::Float(f) => {
                    if spec.contains('.') {
                        // Extract precision
                        if let Some(dot_pos) = spec.rfind('.') {
                            let precision_str = &spec[dot_pos+1..spec.len()-1];
                            if let Ok(precision) = precision_str.parse::<usize>() {
                                return Ok(format!("{:.prec$}", f, prec = precision));
                            }
                        }
                    }
                    Ok(format!("{}", f))
                },
                Value::Int(i) => Ok(format!("{:.6}", *i as f64)),
                Value::String(s) => s.parse::<f64>()
                    .map(|f| format!("{}", f))
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid float")),
                _ => Ok("0.000000".to_string()),
            }
        },
        's' => Ok(format_value(arg)),
        'c' => {
            match arg {
                Value::Int(i) => Ok(char::from_u32(*i as u32).unwrap_or('?').to_string()),
                Value::String(s) => Ok(s.chars().next().unwrap_or('?').to_string()),
                _ => Ok("?".to_string()),
            }
        },
        'x' => {
            match arg {
                Value::Int(i) => Ok(format!("{:x}", i)),
                _ => Ok("0".to_string()),
            }
        },
        'X' => {
            match arg {
                Value::Int(i) => Ok(format!("{:X}", i)),
                _ => Ok("0".to_string()),
            }
        },
        'o' => {
            match arg {
                Value::Int(i) => Ok(format!("{:o}", i)),
                _ => Ok("0".to_string()),
            }
        },
        'v' => Ok(format_value(arg)), // Default format
        _ => Ok(format_value(arg)),
    }
}

/// Read a line from stdin
fn read_line_input() -> io::Result<String> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    
    // Remove trailing newline
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    
    Ok(line)
}

/// Parse a string value into the appropriate CURSED Value type
fn parse_value_from_str(s: &str, target_type: &Value) -> Result<(), Error> {
    match target_type {
        Value::String(_) => Ok(Value::String(s.to_string())),
        Value::Int(_) => {
            s.parse::<i64>()
                .map(Value::Int)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid integer"))
        },
        Value::Float(_) => {
            s.parse::<f64>()
                .map(Value::Float)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid float"))
        },
        Value::Bool(_) => {
            match s.to_lowercase().as_str() {
                "true" | "t" | "1" | "yes" | "y" => Ok(Value::Bool(true)),
                "false" | "f" | "0" | "no" | "n" => Ok(Value::Bool(false)),
                _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid boolean")),
            }
        },
        _ => {
            // Default to string for complex types
            Ok(Value::String(s.to_string()))
        }
    }
}

/// Format a CURSED Value for printing
fn format_value(value: &Value) -> String {
    match value {
        Value::Nil => "nil".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(format_value).collect();
            format!("[{}]", items.join(", "))
        },
        Value::Object(obj) => {
            let items: Vec<String> = obj.iter()
                .map(|(k, v)| format!("{}: {}", k, format_value(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        },
        Value::Function(_) => "<function>".to_string(),
        Value::NativeFunction(_) => "<native_function>".to_string(),
        Value::Channel(_) => "<channel>".to_string(),
        Value::Interface(_) => "<interface>".to_string(),
        Value::Error(e) => format!("Error: {}", e),
        Value::Bytes(b) => format!("<bytes[{}]>", b.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_format_value_basic_types() {
        assert_eq!(format_value(&Value::Nil), "nil");
        assert_eq!(format_value(&Value::Bool(true)), "true");
        assert_eq!(format_value(&Value::Int(42)), "42");
        assert_eq!(format_value(&Value::Float(3.14)), "3.14");
        assert_eq!(format_value(&Value::String("hello".to_string())), "hello");
    }

    #[test]
    fn test_format_value_collections() {
        let arr = vec![Value::Int(1), Value::Int(2), Value::Int(3)];
        assert_eq!(format_value(&Value::Array(arr)), "[1, 2, 3]");

        let mut obj = HashMap::new();
        obj.insert("key".to_string(), Value::String("value".to_string()));
        assert_eq!(format_value(&Value::Object(obj)), "{key: value}");
    }

    #[test]
    fn test_print_to_buffer() {
        let mut buffer = Vec::new();
        let args = vec![Value::String("test".to_string()), Value::Int(123)];
        
        print_to(&mut buffer, &args).unwrap();
        
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "test 123");
    }

    #[test]
    fn test_println_to_buffer() {
        let mut buffer = Vec::new();
        let args = vec![Value::String("test".to_string())];
        
        println_to(&mut buffer, &args).unwrap();
        
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "test\n");
    }
}
