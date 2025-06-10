/// Template Filters - Built-in functions and filter registry for CURSED templates
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, instrument, warn};
use base64::{Engine as _, engine::general_purpose};
use uuid::Uuid;

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;

/// Filter function type
pub type FilterFn = Box<dyn Fn(&[CursedObject]) -> Result<CursedObject, CursedError> + Send + Sync>;

/// Registry of template filters and functions
pub struct FilterRegistry {
    filters: Arc<RwLock<HashMap<String, FilterFn>>>,
}

impl std::fmt::Debug for FilterRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FilterRegistry")
            .field("filter_count", &self.filters.read().unwrap().len())
            .finish()
    }
}

impl FilterRegistry {
    /// Create a new filter registry with built-in filters
    pub fn new() -> Self {
        let registry = Self {
            filters: Arc::new(RwLock::new(HashMap::new())),
        };
        registry.register_builtin_filters();
        registry
    }

    /// Register a custom filter
    pub fn register<F>(&self, name: &str, filter: F)
    where
        F: Fn(&[CursedObject]) -> Result<CursedObject, CursedError> + Send + Sync + 'static,
    {
        if let Ok(mut filters) = self.filters.write() {
            filters.insert(name.to_string(), Box::new(filter));
        }
    }

    /// Apply a filter to values
    #[instrument(skip(self, args))]
    pub fn apply(&self, name: &str, args: &[CursedObject]) -> Result<CursedObject, CursedError> {
        if let Ok(filters) = self.filters.read() {
            if let Some(filter) = filters.get(name) {
                filter(args)
            } else {
                warn!(filter = name, "Unknown filter");
                Err(CursedError::TemplateError {
                    message: format!("Unknown filter: {}", name),
                    source_location: None,
                })
            }
        } else {
            Err(CursedError::TemplateError {
                message: "Failed to acquire filter registry lock".to_string(),
                source_location: None,
            })
        }
    }

    /// Register all built-in filters
    fn register_builtin_filters(&self) {
        // Text manipulation filters
        self.register("lower", |args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s.to_lowercase()))
        });

        self.register("upper", |args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s.to_uppercase()))
        });

        self.register("title", |args| {
            let s = extract_string(&args[0])?;
            let result = s.split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            Ok(CursedObject::String(result))
        });

        self.register("trim", |args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s.trim().to_string()))
        });

        self.register("trimSpace", |args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::String(s.trim().to_string()))
        });

        self.register("trimPrefix", |args| {
            let s = extract_string(&args[0])?;
            let prefix = extract_string(&args[1])?;
            let result = s.strip_prefix(&prefix).unwrap_or(&s);
            Ok(CursedObject::String(result.to_string()))
        });

        self.register("trimSuffix", |args| {
            let s = extract_string(&args[0])?;
            let suffix = extract_string(&args[1])?;
            let result = s.strip_suffix(&suffix).unwrap_or(&s);
            Ok(CursedObject::String(result.to_string()))
        });

        self.register("replace", |args| {
            let s = extract_string(&args[0])?;
            let old = extract_string(&args[1])?;
            let new = extract_string(&args[2])?;
            Ok(CursedObject::String(s.replacen(&old, &new, 1)))
        });

        self.register("replaceAll", |args| {
            let s = extract_string(&args[0])?;
            let old = extract_string(&args[1])?;
            let new = extract_string(&args[2])?;
            Ok(CursedObject::String(s.replace(&old, &new)))
        });

        self.register("split", |args| {
            let s = extract_string(&args[0])?;
            let sep = extract_string(&args[1])?;
            let parts: Vec<CursedObject> = s.split(&sep)
                .map(|part| CursedObject::String(part.to_string()))
                .collect();
            Ok(CursedObject::Array(parts))
        });

        self.register("join", |args| {
            let arr = extract_array(&args[0])?;
            let sep = extract_string(&args[1])?;
            let strings: Result<Vec<String>, CursedError> = arr.iter()
                .map(extract_string)
                .collect();
            Ok(CursedObject::String(strings?.join(&sep)))
        });

        self.register("contains", |args| {
            let s = extract_string(&args[0])?;
            let substr = extract_string(&args[1])?;
            Ok(CursedObject::Boolean(s.contains(&substr)))
        });

        self.register("hasPrefix", |args| {
            let s = extract_string(&args[0])?;
            let prefix = extract_string(&args[1])?;
            Ok(CursedObject::Boolean(s.starts_with(&prefix)))
        });

        self.register("hasSuffix", |args| {
            let s = extract_string(&args[0])?;
            let suffix = extract_string(&args[1])?;
            Ok(CursedObject::Boolean(s.ends_with(&suffix)))
        });

        self.register("substr", |args| {
            let s = extract_string(&args[0])?;
            let start = extract_int(&args[1])? as usize;
            let end = extract_int(&args[2])? as usize;
            let chars: Vec<char> = s.chars().collect();
            if start >= chars.len() {
                return Ok(CursedObject::String("".to_string()));
            }
            let end = end.min(chars.len());
            let result: String = chars[start..end].iter().collect();
            Ok(CursedObject::String(result))
        });

        self.register("repeat", |args| {
            let s = extract_string(&args[0])?;
            let count = extract_int(&args[1])? as usize;
            Ok(CursedObject::String(s.repeat(count)))
        });

        self.register("runeCount", |args| {
            let s = extract_string(&args[0])?;
            Ok(CursedObject::Integer(s.chars().count() as i64))
        });

        self.register("index", |args| {
            let s = extract_string(&args[0])?;
            let substr = extract_string(&args[1])?;
            match s.find(&substr) {
                Some(pos) => Ok(CursedObject::Integer(pos as i64)),
                None => Ok(CursedObject::Integer(-1)),
            }
        });

        self.register("lastIndex", |args| {
            let s = extract_string(&args[0])?;
            let substr = extract_string(&args[1])?;
            match s.rfind(&substr) {
                Some(pos) => Ok(CursedObject::Integer(pos as i64)),
                None => Ok(CursedObject::Integer(-1)),
            }
        });

        // Formatting filters
        self.register("printf", |args| {
            // Simplified printf implementation
            let format = extract_string(&args[0])?;
            Ok(CursedObject::String(format)) // TODO: Implement proper formatting
        });

        self.register("sprintf", |args| {
            // Simplified sprintf implementation
            let format = extract_string(&args[0])?;
            Ok(CursedObject::String(format)) // TODO: Implement proper formatting
        });

        self.register("numFormat", |args| {
            let num = extract_float(&args[0])?;
            let precision = extract_int(&args[1])? as usize;
            Ok(CursedObject::String(format!("{:.prec$}", num, prec = precision)))
        });

        self.register("currency", |args| {
            let num = extract_float(&args[0])?;
            Ok(CursedObject::String(format!("${:.2}", num)))
        });

        self.register("byteSize", |args| {
            let bytes = extract_int(&args[0])? as f64;
            let units = ["B", "KB", "MB", "GB", "TB"];
            let mut size = bytes;
            let mut unit_index = 0;
            
            while size >= 1024.0 && unit_index < units.len() - 1 {
                size /= 1024.0;
                unit_index += 1;
            }
            
            Ok(CursedObject::String(format!("{:.1} {}", size, units[unit_index])))
        });

        self.register("percentage", |args| {
            let num = extract_float(&args[0])?;
            Ok(CursedObject::String(format!("{:.1}%", num * 100.0)))
        });

        self.register("plural", |args| {
            let count = extract_int(&args[0])?;
            let singular = extract_string(&args[1])?;
            let plural = extract_string(&args[2])?;
            if count == 1 {
                Ok(CursedObject::String(singular))
            } else {
                Ok(CursedObject::String(plural))
            }
        });

        // Collection operations
        self.register("len", |args| {
            match &args[0] {
                CursedObject::String(s) => Ok(CursedObject::Integer(s.chars().count() as i64)),
                CursedObject::Array(arr) => Ok(CursedObject::Integer(arr.len() as i64)),
                CursedObject::Map(map) => Ok(CursedObject::Integer(map.len() as i64)),
                _ => Ok(CursedObject::Integer(0)),
            }
        });

        self.register("slice", |args| {
            let arr = extract_array(&args[0])?;
            let start = extract_int(&args[1])? as usize;
            let end = extract_int(&args[2])? as usize;
            let end = end.min(arr.len());
            if start >= arr.len() {
                return Ok(CursedObject::Array(vec![]));
            }
            Ok(CursedObject::Array(arr[start..end].to_vec()))
        });

        self.register("reverse", |args| {
            let mut arr = extract_array(&args[0])?;
            arr.reverse();
            Ok(CursedObject::Array(arr))
        });

        self.register("first", |args| {
            let arr = extract_array(&args[0])?;
            if arr.is_empty() {
                Ok(CursedObject::Nil)
            } else {
                Ok(arr[0].clone())
            }
        });

        self.register("last", |args| {
            let arr = extract_array(&args[0])?;
            if arr.is_empty() {
                Ok(CursedObject::Nil)
            } else {
                Ok(arr[arr.len() - 1].clone())
            }
        });

        self.register("keys", |args| {
            match &args[0] {
                CursedObject::Map(map) => {
                    let keys: Vec<CursedObject> = map.keys()
                        .map(|k| CursedObject::String(k.clone()))
                        .collect();
                    Ok(CursedObject::Array(keys))
                }
                _ => Err(CursedError::TemplateError {
                    message: "keys filter requires a map".to_string(),
                    source_location: None,
                }),
            }
        });

        self.register("values", |args| {
            match &args[0] {
                CursedObject::Map(map) => {
                    let values: Vec<CursedObject> = map.values().cloned().collect();
                    Ok(CursedObject::Array(values))
                }
                _ => Err(CursedError::TemplateError {
                    message: "values filter requires a map".to_string(),
                    source_location: None,
                }),
            }
        });

        // Data conversion filters
        self.register("toJSON", |args| {
            // Simplified JSON serialization
            match &args[0] {
                CursedObject::String(s) => Ok(CursedObject::String(format!("\"{}\"", s))),
                CursedObject::Integer(n) => Ok(CursedObject::String(n.to_string())),
                CursedObject::Float(n) => Ok(CursedObject::String(n.to_string())),
                CursedObject::Boolean(b) => Ok(CursedObject::String(b.to_string())),
                CursedObject::Nil => Ok(CursedObject::String("null".to_string())),
                _ => Ok(CursedObject::String("{}".to_string())),
            }
        });

        self.register("toBase64", |args| {
            let s = extract_string(&args[0])?;
            let encoded = general_purpose::STANDARD.encode(s.as_bytes());
            Ok(CursedObject::String(encoded))
        });

        self.register("fromBase64", |args| {
            let s = extract_string(&args[0])?;
            match general_purpose::STANDARD.decode(&s) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(decoded) => Ok(CursedObject::String(decoded)),
                    Err(_) => Err(CursedError::TemplateError {
                        message: "Invalid UTF-8 in base64 data".to_string(),
                        source_location: None,
                    }),
                },
                Err(_) => Err(CursedError::TemplateError {
                    message: "Invalid base64 encoding".to_string(),
                    source_location: None,
                }),
            }
        });

        self.register("toBool", |args| {
            let result = match &args[0] {
                CursedObject::Boolean(b) => *b,
                CursedObject::String(s) => !s.is_empty(),
                CursedObject::Integer(n) => *n != 0,
                CursedObject::Float(n) => *n != 0.0,
                CursedObject::Nil => false,
                _ => true,
            };
            Ok(CursedObject::Boolean(result))
        });

        self.register("toString", |args| {
            let result = match &args[0] {
                CursedObject::String(s) => s.clone(),
                CursedObject::Integer(n) => n.to_string(),
                CursedObject::Float(n) => n.to_string(),
                CursedObject::Boolean(b) => b.to_string(),
            CursedObject::Char(c) => c.to_string(),
                CursedObject::Nil => "".to_string(),
                _ => format!("{:?}", args[0]),
            };
            Ok(CursedObject::String(result))
        });

        self.register("toInt", |args| {
            match &args[0] {
                CursedObject::Integer(n) => Ok(CursedObject::Integer(*n)),
                CursedObject::Float(n) => Ok(CursedObject::Integer(*n as i64)),
                CursedObject::String(s) => {
                    match s.parse::<i64>() {
                        Ok(n) => Ok(CursedObject::Integer(n)),
                        Err(_) => Err(CursedError::TemplateError {
                            message: format!("Cannot convert '{}' to integer", s),
                            source_location: None,
                        }),
                    }
                }
                _ => Err(CursedError::TemplateError {
                    message: "Cannot convert to integer".to_string(),
                    source_location: None,
                }),
            }
        });

        self.register("toFloat", |args| {
            match &args[0] {
                CursedObject::Float(n) => Ok(CursedObject::Float(*n)),
                CursedObject::Integer(n) => Ok(CursedObject::Float(*n as f64)),
                CursedObject::String(s) => {
                    match s.parse::<f64>() {
                        Ok(n) => Ok(CursedObject::Float(n)),
                        Err(_) => Err(CursedError::TemplateError {
                            message: format!("Cannot convert '{}' to float", s),
                            source_location: None,
                        }),
                    }
                }
                _ => Err(CursedError::TemplateError {
                    message: "Cannot convert to float".to_string(),
                    source_location: None,
                }),
            }
        });

        // Control flow filters
        self.register("eq", |args| {
            Ok(CursedObject::Boolean(objects_equal(&args[0], &args[1])))
        });

        self.register("ne", |args| {
            Ok(CursedObject::Boolean(!objects_equal(&args[0], &args[1])))
        });

        self.register("lt", |args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Boolean(a < b))
        });

        self.register("le", |args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Boolean(a <= b))
        });

        self.register("gt", |args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Boolean(a > b))
        });

        self.register("ge", |args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Boolean(a >= b))
        });

        self.register("and", |args| {
            let a = is_truthy(&args[0]);
            let b = is_truthy(&args[1]);
            Ok(CursedObject::Boolean(a && b))
        });

        self.register("or", |args| {
            let a = is_truthy(&args[0]);
            let b = is_truthy(&args[1]);
            Ok(CursedObject::Boolean(a || b))
        });

        self.register("not", |args| {
            Ok(CursedObject::Boolean(!is_truthy(&args[0])))
        });

        self.register("ternary", |args| {
            let condition = is_truthy(&args[0]);
            if condition {
                Ok(args[1].clone())
            } else {
                Ok(args[2].clone())
            }
        });

        self.register("isZero", |args| {
            let result = match &args[0] {
                CursedObject::Integer(n) => *n == 0,
                CursedObject::Float(n) => *n == 0.0,
                _ => false,
            };
            Ok(CursedObject::Boolean(result))
        });

        self.register("isNil", |args| {
            Ok(CursedObject::Boolean(matches!(args[0], CursedObject::Nil)))
        });

        self.register("isEmpty", |args| {
            let result = match &args[0] {
                CursedObject::String(s) => s.is_empty(),
                CursedObject::Array(arr) => arr.is_empty(),
                CursedObject::Map(map) => map.is_empty(),
                CursedObject::Nil => true,
                _ => false,
            };
            Ok(CursedObject::Boolean(result))
        });

        // URL and HTML filters
        self.register("urlEncode", |args| {
            let s = extract_string(&args[0])?;
            let encoded = urlencoding::encode(&s);
            Ok(CursedObject::String(encoded.to_string()))
        });

        self.register("urlDecode", |args| {
            let s = extract_string(&args[0])?;
            match urlencoding::decode(&s) {
                Ok(decoded) => Ok(CursedObject::String(decoded.to_string())),
                Err(_) => Err(CursedError::TemplateError {
                    message: "Invalid URL encoding".to_string(),
                    source_location: None,
                }),
            }
        });

        self.register("htmlEscape", |args| {
            let s = extract_string(&args[0])?;
            let escaped = s
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#x27;");
            Ok(CursedObject::String(escaped))
        });

        self.register("htmlUnescape", |args| {
            let s = extract_string(&args[0])?;
            let unescaped = s
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#x27;", "'");
            Ok(CursedObject::String(unescaped))
        });

        self.register("pathEscape", |args| {
            let s = extract_string(&args[0])?;
            let escaped = urlencoding::encode(&s);
            Ok(CursedObject::String(escaped.to_string()))
        });

        self.register("queryEscape", |args| {
            let s = extract_string(&args[0])?;
            let escaped = urlencoding::encode(&s);
            Ok(CursedObject::String(escaped.to_string()))
        });

        self.register("cssEscape", |args| {
            let s = extract_string(&args[0])?;
            // Simplified CSS escaping
            let escaped = s.chars()
                .map(|c| if c.is_alphanumeric() { c.to_string() } else { format!("\\{:x}", c as u32) })
                .collect::<String>();
            Ok(CursedObject::String(escaped))
        });

        self.register("jsEscape", |args| {
            let s = extract_string(&args[0])?;
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\'', "\\'")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t");
            Ok(CursedObject::String(escaped))
        });

        // Safe HTML/URL/JS/CSS markers (return the string as-is but mark as safe)
        self.register("safeHTML", |args| Ok(args[0].clone()));
        self.register("safeURL", |args| Ok(args[0].clone()));
        self.register("safeJS", |args| Ok(args[0].clone()));
        self.register("safeCSS", |args| Ok(args[0].clone()));

        // Random and Math filters
        self.register("randomInt", |args| {
            let min = extract_int(&args[0])?;
            let max = extract_int(&args[1])?;
            let result = min + (rand::random::<i64>() % (max - min + 1));
            Ok(CursedObject::Integer(result))
        });

        self.register("randomString", |args| {
            let length = extract_int(&args[0])? as usize;
            let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            let result: String = (0..length)
                .map(|_| {
                    let idx = rand::random::<usize>() % charset.len();
                    charset[idx] as char
                })
                .collect();
            Ok(CursedObject::String(result))
        });

        self.register("uuid", |_args| {
            let uuid = Uuid::new_v4();
            Ok(CursedObject::String(uuid.to_string()))
        });

        self.register("add", |args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Float(a + b))
        });

        self.register("sub", |args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Float(a - b))
        });

        self.register("mul", |args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Float(a * b))
        });

        self.register("div", |args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            if b == 0.0 {
                return Err(CursedError::TemplateError {
                    message: "Division by zero".to_string(),
                    source_location: None,
                });
            }
            Ok(CursedObject::Float(a / b))
        });

        self.register("mod", |args| {
            let a = extract_int(&args[0])?;
            let b = extract_int(&args[1])?;
            if b == 0 {
                return Err(CursedError::TemplateError {
                    message: "Modulo by zero".to_string(),
                    source_location: None,
                });
            }
            Ok(CursedObject::Integer(a % b))
        });

        self.register("max", |args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Float(a.max(b)))
        });

        self.register("min", |args| {
            let a = extract_float(&args[0])?;
            let b = extract_float(&args[1])?;
            Ok(CursedObject::Float(a.min(b)))
        });

        self.register("round", |args| {
            let num = extract_float(&args[0])?;
            let precision = extract_int(&args[1])? as u32;
            let multiplier = 10.0_f64.powi(precision as i32);
            Ok(CursedObject::Float((num * multiplier).round() / multiplier))
        });

        self.register("ceil", |args| {
            let num = extract_float(&args[0])?;
            Ok(CursedObject::Float(num.ceil()))
        });

        self.register("floor", |args| {
            let num = extract_float(&args[0])?;
            Ok(CursedObject::Float(num.floor()))
        });
    }
}

impl Default for FilterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to extract string from CursedObject
fn extract_string(obj: &CursedObject) -> Result<String, CursedError> {
    match obj {
        CursedObject::String(s) => Ok(s.clone()),
        CursedObject::Integer(n) => Ok(n.to_string()),
        CursedObject::Float(n) => Ok(n.to_string()),
        CursedObject::Boolean(b) => Ok(b.to_string()),
            CursedObject::Char(c) => Ok(c.to_string()),
        _ => Err(CursedError::TemplateError {
            message: "Expected string value".to_string(),
            source_location: None,
        }),
    }
}

/// Helper function to extract integer from CursedObject
fn extract_int(obj: &CursedObject) -> Result<i64, CursedError> {
    match obj {
        CursedObject::Integer(n) => Ok(*n),
        CursedObject::Float(n) => Ok(*n as i64),
        CursedObject::String(s) => {
            s.parse::<i64>().map_err(|_| CursedError::TemplateError {
                message: "Expected integer value".to_string(),
                source_location: None,
            })
        }
        _ => Err(CursedError::TemplateError {
            message: "Expected integer value".to_string(),
            source_location: None,
        }),
    }
}

/// Helper function to extract float from CursedObject
fn extract_float(obj: &CursedObject) -> Result<f64, CursedError> {
    match obj {
        CursedObject::Float(n) => Ok(*n),
        CursedObject::Integer(n) => Ok(*n as f64),
        CursedObject::String(s) => {
            s.parse::<f64>().map_err(|_| CursedError::TemplateError {
                message: "Expected numeric value".to_string(),
                source_location: None,
            })
        }
        _ => Err(CursedError::TemplateError {
            message: "Expected numeric value".to_string(),
            source_location: None,
        }),
    }
}

/// Helper function to extract array from CursedObject
fn extract_array(obj: &CursedObject) -> Result<Vec<CursedObject>, CursedError> {
    match obj {
        CursedObject::Array(arr) => Ok(arr.clone()),
        _ => Err(CursedError::TemplateError {
            message: "Expected array value".to_string(),
            source_location: None,
        }),
    }
}

/// Helper function to check if objects are equal
fn objects_equal(left: &CursedObject, right: &CursedObject) -> bool {
    match (left, right) {
        (CursedObject::String(a), CursedObject::String(b)) => a == b,
        (CursedObject::Integer(a), CursedObject::Integer(b)) => a == b,
        (CursedObject::Float(a), CursedObject::Float(b)) => (a - b).abs() < f64::EPSILON,
        (CursedObject::Boolean(a), CursedObject::Boolean(b)) => a == b,
        (CursedObject::Nil, CursedObject::Nil) => true,
        _ => false,
    }
}

/// Helper function to check if object is truthy
fn is_truthy(obj: &CursedObject) -> bool {
    match obj {
        CursedObject::Boolean(b) => *b,
        CursedObject::Nil => false,
        CursedObject::Integer(n) => *n != 0,
        CursedObject::Float(n) => *n != 0.0,
        CursedObject::String(s) => !s.is_empty(),
        CursedObject::Char(_) => true, // Characters are always truthy
        CursedObject::Array(arr) => !arr.is_empty(),
        CursedObject::Map(map) => !map.is_empty(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_filters() {
        let registry = FilterRegistry::new();

        // Test lower filter
        let result = registry.apply("lower", &[CursedObject::String("HELLO".to_string())]).unwrap();
        assert_eq!(result, CursedObject::String("hello".to_string()));

        // Test upper filter
        let result = registry.apply("upper", &[CursedObject::String("hello".to_string())]).unwrap();
        assert_eq!(result, CursedObject::String("HELLO".to_string()));

        // Test title filter
        let result = registry.apply("title", &[CursedObject::String("hello world".to_string())]).unwrap();
        assert_eq!(result, CursedObject::String("Hello World".to_string()));
    }

    #[test]
    fn test_math_filters() {
        let registry = FilterRegistry::new();

        // Test add filter
        let result = registry.apply("add", &[
            CursedObject::Integer(5),
            CursedObject::Integer(3),
        ]).unwrap();
        assert_eq!(result, CursedObject::Float(8.0));

        // Test max filter
        let result = registry.apply("max", &[
            CursedObject::Float(5.5),
            CursedObject::Float(3.2),
        ]).unwrap();
        assert_eq!(result, CursedObject::Float(5.5));
    }

    #[test]
    fn test_collection_filters() {
        let registry = FilterRegistry::new();

        // Test len filter
        let arr = vec![
            CursedObject::String("a".to_string()),
            CursedObject::String("b".to_string()),
            CursedObject::String("c".to_string()),
        ];
        let result = registry.apply("len", &[CursedObject::Array(arr)]).unwrap();
        assert_eq!(result, CursedObject::Integer(3));

        // Test first filter
        let arr = vec![
            CursedObject::String("first".to_string()),
            CursedObject::String("second".to_string()),
        ];
        let result = registry.apply("first", &[CursedObject::Array(arr)]).unwrap();
        assert_eq!(result, CursedObject::String("first".to_string()));
    }

    #[test]
    fn test_conversion_filters() {
        let registry = FilterRegistry::new();

        // Test toString filter
        let result = registry.apply("toString", &[CursedObject::Integer(42)]).unwrap();
        assert_eq!(result, CursedObject::String("42".to_string()));

        // Test toInt filter
        let result = registry.apply("toInt", &[CursedObject::String("123".to_string())]).unwrap();
        assert_eq!(result, CursedObject::Integer(123));
    }

    #[test]
    fn test_html_filters() {
        let registry = FilterRegistry::new();

        // Test htmlEscape filter
        let result = registry.apply("htmlEscape", &[
            CursedObject::String("<script>alert('xss')</script>".to_string())
        ]).unwrap();
        
        if let CursedObject::String(escaped) = result {
            assert!(escaped.contains("&lt;script&gt;"));
        } else {
            panic!("Expected string result");
        }
    }
}
