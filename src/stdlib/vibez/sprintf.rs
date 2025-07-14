//! Sprintf module for vibez - C-style formatted printing for CURSED

use crate::error_types::CursedError;
use crate::runtime::value::Value;
use std::fmt::Write;

/// C-style format specifier
#[derive(Debug, Clone)]
pub struct FormatSpecifier {
    pub position: usize,
    pub flags: FormatFlags,
    pub width: Option<usize>,
    pub precision: Option<usize>,
    pub conversion: ConversionType,
}

/// Format flags for C-style formatting
#[derive(Debug, Clone, Default)]
pub struct FormatFlags {
    pub left_justify: bool,    // -
    pub show_sign: bool,       // +
    pub space_prefix: bool,    // (space)
    pub alternate_form: bool,  // #
    pub zero_pad: bool,        // 0
}

/// Conversion types for sprintf
#[derive(Debug, Clone)]
pub enum ConversionType {
    /// Signed decimal integer
    Decimal,
    /// Unsigned decimal integer
    UnsignedDecimal,
    /// Unsigned hexadecimal (lowercase)
    HexLower,
    /// Unsigned hexadecimal (uppercase)
    HexUpper,
    /// Unsigned octal
    Octal,
    /// Floating point (lowercase)
    FloatLower,
    /// Floating point (uppercase)
    FloatUpper,
    /// Scientific notation (lowercase)
    ScientificLower,
    /// Scientific notation (uppercase)
    ScientificUpper,
    /// Shorter of %f or %e (lowercase)
    GeneralLower,
    /// Shorter of %F or %E (uppercase)
    GeneralUpper,
    /// Single character
    Character,
    /// String
    String,
    /// Pointer (hexadecimal)
    Pointer,
    /// Percentage sign
    Percent,
}

/// Validate a C-style format string
pub fn validate_format_string(format: &str) -> Result<bool, CursedError> {
    let specifiers = parse_format_specifiers(format)?;
    
    // Check for valid specifiers
    for spec in &specifiers {
        // Validate width and precision
        if let Some(width) = spec.width {
            if width > 1000 {
                return Err(CursedError::Runtime("Format width too large".to_string()));
            }
        }
        
        if let Some(precision) = spec.precision {
            if precision > 1000 {
                return Err(CursedError::Runtime("Format precision too large".to_string()));
            }
        }
        
        // Validate flag combinations
        if spec.flags.left_justify && spec.flags.zero_pad {
            return Err(CursedError::Runtime("Cannot use both left-justify and zero-pad flags".to_string()));
        }
    }
    
    Ok(true)
}

/// Count format specifiers in format string
pub fn count_format_specifiers(format: &str) -> usize {
    match parse_format_specifiers(format) {
        Ok(specs) => specs.len(),
        Err(_) => 0,
    }
}

/// Parse format specifiers from C-style format string
pub fn parse_format_specifiers(format: &str) -> Result<Vec<FormatSpecifier>, CursedError> {
    let mut specifiers = Vec::new();
    let mut chars = format.char_indices().peekable();
    
    while let Some((i, ch)) = chars.next() {
        if ch == '%' {
            if chars.peek().map(|(_, c)| *c) == Some('%') {
                // Escaped percent - skip
                chars.next();
                continue;
            }
            
            // Parse format specifier
            let spec = parse_single_specifier(&mut chars, i)?;
            specifiers.push(spec);
        }
    }
    
    Ok(specifiers)
}

/// Parse a single format specifier
fn parse_single_specifier(
    chars: &mut std::iter::Peekable<std::str::CharIndices>,
    position: usize
) -> Result<FormatSpecifier, CursedError> {
    let mut flags = FormatFlags::default();
    let mut width = None;
    let mut precision = None;
    
    // Parse flags
    while let Some(&(_, ch)) = chars.peek() {
        match ch {
            '-' => {
                flags.left_justify = true;
                chars.next();
            },
            '+' => {
                flags.show_sign = true;
                chars.next();
            },
            ' ' => {
                flags.space_prefix = true;
                chars.next();
            },
            '#' => {
                flags.alternate_form = true;
                chars.next();
            },
            '0' => {
                flags.zero_pad = true;
                chars.next();
            },
            _ => break,
        }
    }
    
    // Parse width
    if let Some(&(_, ch)) = chars.peek() {
        if ch.is_ascii_digit() && ch != '0' {
            let mut width_str = String::new();
            while let Some(&(_, ch)) = chars.peek() {
                if ch.is_ascii_digit() {
                    width_str.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            width = Some(width_str.parse().map_err(|_| {
                CursedError::Runtime("Invalid width in format specifier".to_string())
            })?);
        }
    }
    
    // Parse precision
    if chars.peek().map(|(_, c)| *c) == Some('.') {
        chars.next(); // consume '.'
        let mut precision_str = String::new();
        while let Some(&(_, ch)) = chars.peek() {
            if ch.is_ascii_digit() {
                precision_str.push(ch);
                chars.next();
            } else {
                break;
            }
        }
        precision = Some(if precision_str.is_empty() {
            0
        } else {
            precision_str.parse().map_err(|_| {
                CursedError::Runtime("Invalid precision in format specifier".to_string())
            })?
        });
    }
    
    // Parse conversion type
    let conversion = match chars.next() {
        Some((_, 'd')) | Some((_, 'i')) => ConversionType::Decimal,
        Some((_, 'u')) => ConversionType::UnsignedDecimal,
        Some((_, 'x')) => ConversionType::HexLower,
        Some((_, 'X')) => ConversionType::HexUpper,
        Some((_, 'o')) => ConversionType::Octal,
        Some((_, 'f')) => ConversionType::FloatLower,
        Some((_, 'F')) => ConversionType::FloatUpper,
        Some((_, 'e')) => ConversionType::ScientificLower,
        Some((_, 'E')) => ConversionType::ScientificUpper,
        Some((_, 'g')) => ConversionType::GeneralLower,
        Some((_, 'G')) => ConversionType::GeneralUpper,
        Some((_, 'c')) => ConversionType::Character,
        Some((_, 's')) => ConversionType::String,
        Some((_, 'p')) => ConversionType::Pointer,
        Some((_, '%')) => ConversionType::Percent,
        Some((_, ch)) => return Err(CursedError::Runtime(format!("Invalid conversion specifier: {}", ch))),
        None => return Err(CursedError::Runtime("Incomplete format specifier".to_string())),
    };
    
    Ok(FormatSpecifier {
        position,
        flags,
        width,
        precision,
        conversion,
    })
}

/// Format values using C-style sprintf
pub fn sprintf(format: &str, args: &[Value]) -> Result<String, CursedError> {
    let specifiers = parse_format_specifiers(format)?;
    
    if specifiers.len() > args.len() {
        return Err(CursedError::Runtime("Not enough arguments for format string".to_string()));
    }
    
    let mut result = String::new();
    let mut spec_index = 0;
    let mut chars = format.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '%' {
            if chars.peek() == Some(&'%') {
                // Escaped percent
                chars.next();
                result.push('%');
            } else {
                // Format specifier
                if spec_index < specifiers.len() && spec_index < args.len() {
                    let formatted = format_value_sprintf(&args[spec_index], &specifiers[spec_index])?;
                    result.push_str(&formatted);
                    spec_index += 1;
                }
                
                // Skip the format specifier in the input string
                while let Some(ch) = chars.next() {
                    if matches!(ch, 'd' | 'i' | 'u' | 'x' | 'X' | 'o' | 'f' | 'F' | 'e' | 'E' | 'g' | 'G' | 'c' | 's' | 'p') {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok(result)
}

/// Format a single value according to sprintf specifier
fn format_value_sprintf(value: &Value, spec: &FormatSpecifier) -> Result<String, CursedError> {
    let mut formatted = match (&spec.conversion, value) {
        (ConversionType::Decimal, Value::Integer(i)) => format!("{}", i),
        (ConversionType::Decimal, Value::Number(f)) => format!("{}", *f as i64),
        (ConversionType::UnsignedDecimal, Value::Integer(i)) => format!("{}", *i as u64),
        (ConversionType::UnsignedDecimal, Value::Number(f)) => format!("{}", *f as u64),
        (ConversionType::HexLower, Value::Integer(i)) => format!("{:x}", *i as u64),
        (ConversionType::HexUpper, Value::Integer(i)) => format!("{:X}", *i as u64),
        (ConversionType::Octal, Value::Integer(i)) => format!("{:o}", *i as u64),
        (ConversionType::FloatLower, Value::Number(f)) => {
            if let Some(precision) = spec.precision {
                format!("{:.1$}", f, precision)
            } else {
                format!("{:.6}", f)
            }
        },
        (ConversionType::FloatLower, Value::Integer(i)) => {
            let f = *i as f64;
            if let Some(precision) = spec.precision {
                format!("{:.1$}", f, precision)
            } else {
                format!("{:.6}", f)
            }
        },
        (ConversionType::FloatUpper, Value::Number(f)) => {
            if let Some(precision) = spec.precision {
                format!("{:.1$}", f, precision).to_uppercase()
            } else {
                format!("{:.6}", f).to_uppercase()
            }
        },
        (ConversionType::ScientificLower, Value::Number(f)) => {
            if let Some(precision) = spec.precision {
                format!("{:.1$e}", f, precision)
            } else {
                format!("{:e}", f)
            }
        },
        (ConversionType::ScientificUpper, Value::Number(f)) => {
            if let Some(precision) = spec.precision {
                format!("{:.1$E}", f, precision)
            } else {
                format!("{:E}", f)
            }
        },
        (ConversionType::GeneralLower, Value::Number(f)) => {
            // Rust doesn't have :g format, use scientific notation
            if let Some(precision) = spec.precision {
                format!("{:.1$e}", f, precision)
            } else {
                format!("{:e}", f)
            }
        },
        (ConversionType::GeneralUpper, Value::Number(f)) => {
            // Rust doesn't have :G format, use scientific notation
            if let Some(precision) = spec.precision {
                format!("{:.1$E}", f, precision)
            } else {
                format!("{:E}", f)
            }
        },
        (ConversionType::Character, Value::Integer(i)) => {
            if *i >= 0 && *i <= 127 {
                format!("{}", char::from(*i as u8))
            } else {
                return Err(CursedError::Runtime("Invalid character code".to_string()));
            }
        },
        (ConversionType::Character, Value::String(s)) => {
            if let Some(ch) = s.chars().next() {
                format!("{}", ch)
            } else {
                return Err(CursedError::Runtime("Empty string for character conversion".to_string()));
            }
        },
        (ConversionType::String, Value::String(s)) => {
            if let Some(precision) = spec.precision {
                s.chars().take(precision).collect()
            } else {
                s.clone()
            }
        },
        (ConversionType::String, _) => format_value_default(value),
        (ConversionType::Pointer, _) => format!("0x{:x}", value as *const _ as usize),
        (ConversionType::Percent, _) => "%".to_string(),
        _ => return Err(CursedError::Runtime(format!("Type mismatch for format specifier: {:?}", spec.conversion))),
    };
    
    // Apply sign formatting
    if matches!(spec.conversion, ConversionType::Decimal | ConversionType::FloatLower | ConversionType::FloatUpper | ConversionType::ScientificLower | ConversionType::ScientificUpper | ConversionType::GeneralLower | ConversionType::GeneralUpper) {
        if spec.flags.show_sign && !formatted.starts_with('-') {
            formatted = format!("+{}", formatted);
        } else if spec.flags.space_prefix && !formatted.starts_with('-') && !formatted.starts_with('+') {
            formatted = format!(" {}", formatted);
        }
    }
    
    // Apply alternate form
    if spec.flags.alternate_form {
        match spec.conversion {
            ConversionType::HexLower => {
                if !formatted.is_empty() && formatted != "0" {
                    formatted = format!("0x{}", formatted);
                }
            },
            ConversionType::HexUpper => {
                if !formatted.is_empty() && formatted != "0" {
                    formatted = format!("0X{}", formatted);
                }
            },
            ConversionType::Octal => {
                if !formatted.starts_with('0') {
                    formatted = format!("0{}", formatted);
                }
            },
            _ => {}
        }
    }
    
    // Apply width and padding
    if let Some(width) = spec.width {
        if formatted.len() < width {
            let padding = width - formatted.len();
            if spec.flags.left_justify {
                formatted.push_str(&" ".repeat(padding));
            } else if spec.flags.zero_pad && !spec.flags.left_justify {
                // Insert zeros after sign
                if formatted.starts_with('+') || formatted.starts_with('-') || formatted.starts_with(' ') {
                    let sign = formatted.chars().next().unwrap();
                    formatted = format!("{}{}{}", sign, "0".repeat(padding), &formatted[1..]);
                } else {
                    formatted = format!("{}{}", "0".repeat(padding), formatted);
                }
            } else {
                formatted = format!("{}{}", " ".repeat(padding), formatted);
            }
        }
    }
    
    Ok(formatted)
}

/// Default value formatting for sprintf
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
