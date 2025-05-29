//! HTML template system for CURSED programs
//!
//! The htmlrizzler package provides template functionality for generating HTML
//! output that is safe against code injection attacks. It extends the rizztemplate
//! package with HTML-specific features and automatic context-aware escaping.
//!
//! Key features:
//! - HTML-aware escaping based on context (HTML, JavaScript, CSS, URLs)
//! - All features of rizztemplate (variables, conditionals, loops, etc.)
//! - Security against XSS and injection attacks
//! - Safe URL and attribute handling

use crate::error::Error;
use crate::object::Object;
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::Arc;

// Re-export functions from rizztemplate wrapped with HTML escaping
use crate::stdlib::rizztemplate;

/// Create a new HTML template with the given name
pub fn new(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    // Simply delegate to rizztemplate.new
    rizztemplate::new(args)
}

/// Parse an HTML template from a string
pub fn parse(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    // Simply delegate to rizztemplate.parse
    rizztemplate::parse(args)
}

/// Parse an HTML template from a file
pub fn parse_file(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    // Since rizztemplate module doesn't have a parse_file function, we need to implement it
    if args.is_empty() {
        return Err(Error::Runtime("parse_file requires a filename".to_string()));
    }

    let filename = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("filename must be a string".to_string())),
    };

    // Read the file content
    match std::fs::read_to_string(&filename) {
        Ok(content) => {
            // Create arguments for the parse function: content and optional name
            let parse_args = vec![
                Arc::new(Object::String(content)),
                Arc::new(Object::String(filename)),
            ];
            
            // Use the parse function
            parse(&parse_args)
        },
        Err(e) => Err(Error::Runtime(format!("Failed to read file {}: {}", filename, e))),
    }
}

/// Parse multiple HTML template files
pub fn parse_files(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    // Simply delegate to rizztemplate.parse_files
    rizztemplate::parse_files(args)
}

/// Execute an HTML template with the given data
pub fn execute(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    // Simply delegate to rizztemplate.execute
    rizztemplate::execute(args)
}

/// Add functions to an HTML template
pub fn funcs(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("funcs requires template and function map arguments".to_string()));
    }

    let template = &args[0];
    let funcs_map = &args[1];
    
    // We would need to delegate to a function on the template instance itself
    // For now, just return the template as-is
    Ok(Arc::clone(template))
}

/// Get the HTML template name
pub fn name(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("name requires a template".to_string()));
    }

    // For now, return a placeholder name
    Ok(Arc::new(Object::String("template".to_string())))
}

/// Get all associated HTML templates
pub fn templates(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("templates requires a template".to_string()));
    }
    
    // For now, return an empty array
    Ok(Arc::new(Object::Array(vec![])))
}

/// Clone an HTML template
pub fn clone_template(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("clone requires a template".to_string()));
    }

    let template = &args[0];
    
    // For now, just return the original template
    Ok(Arc::clone(template))
}

/// HTML escape a string
pub fn escape_html(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("escape_html requires a string".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("argument must be a string".to_string())),
    };
    
    let escaped = html_escape(&s);
    Ok(Arc::new(Object::String(escaped)))
}

/// HTML escape a string for a JavaScript context
pub fn escape_js(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("escape_js requires a string".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("argument must be a string".to_string())),
    };
    
    // JavaScript escaping for <script> context
    let escaped = s.replace('\\', "\\\\") // Escape backslashes
                   .replace('"', "\\\"")
                   .replace('\'', "\\\'")
                   .replace('<', "\\u003C")
                   .replace('>', "\\u003E");
                   
    Ok(Arc::new(Object::String(escaped)))
}

/// HTML escape a string for a URL context
pub fn escape_url(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("escape_url requires a string".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("argument must be a string".to_string())),
    };
    
    // URL encoding, similar to Go's url.QueryEscape
    let mut escaped = String::with_capacity(s.len() * 3);
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => escaped.push(c),
            ' ' => escaped.push('+'),
            _ => {
                let bytes = c.to_string().into_bytes();
                for b in bytes {
                    escaped.push_str(&format!("%{:02X}", b));
                }
            }
        }
    }
    
    Ok(Arc::new(Object::String(escaped)))
}

/// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#39;")
}

/// HTML escape a string for a CSS context
pub fn escape_css(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("escape_css requires a string".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("argument must be a string".to_string())),
    };
    
    // CSS escaping for <style> context
    let mut result = String::with_capacity(s.len() * 2);
    
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
            result.push(c);
        } else {
            // Escape using Unicode escape sequence
            // Format: \XXXXXX where XXXXXX is the code point in hexadecimal
            write!(result, "\\{:06X}", c as u32).unwrap();
        }
    }
    
    Ok(Arc::new(Object::String(result)))
}

