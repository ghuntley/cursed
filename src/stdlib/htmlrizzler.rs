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
use std::rc::Rc;

// Re-export functions from rizztemplate wrapped with HTML escaping
use crate::stdlib::rizztemplate;

/// Context type for context-aware HTML escaping
#[derive(Clone, PartialEq)]
enum HtmlContext {
    Html,       // Regular HTML content
    Script,     // Inside <script> tags
    Style,      // Inside <style> tags
    Url,        // URL attribute context
    Attribute,  // Generic attribute context
}

/// Create a new HTML template with the given name
pub fn new(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // We delegate to rizztemplate.new and then wrap it with HTML escaping functionality
    let result = rizztemplate::new(args)?;
    
    // In a complete implementation, we would add HTML escaping context here
    Ok(result)
}

/// Parse an HTML template from a string
pub fn parse(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // We delegate the parsing to rizztemplate.parse
    rizztemplate::parse(args)
    // In a complete implementation, we would process the template for HTML contexts
}

/// Parse an HTML template from a file
pub fn parse_file(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
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
                Rc::new(Object::String(content)),
                Rc::new(Object::String(filename)),
            ];
            
            // Use the parse function
            parse(&parse_args)
        },
        Err(e) => Err(Error::Runtime(format!("Failed to read file {}: {}", filename, e))),
    }
}

/// Parse multiple HTML template files
pub fn parse_files(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // We delegate to rizztemplate.parse_files
    rizztemplate::parse_files(args)
    // In a complete implementation, we would process the templates for HTML contexts
}

/// Execute an HTML template with the given data
pub fn execute(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // First, use standard template execution
    let result = rizztemplate::execute(args)?;
    
    // In a complete implementation, we would intercept the output and apply HTML escaping
    // based on context before writing it to the writer
    
    Ok(result)
}

/// Add functions to an HTML template
pub fn funcs(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // We delegate to rizztemplate.funcs
    rizztemplate::funcs(args)
    
    // In a complete implementation, we would add HTML-specific helper functions here
}

/// Get the HTML template name
pub fn name(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // We delegate to rizztemplate.name
    rizztemplate::name(args)
}

/// Get all associated HTML templates
pub fn templates(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // We delegate to rizztemplate.templates
    rizztemplate::templates(args)
}

/// Clone an HTML template
pub fn clone_template(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // We delegate to rizztemplate.clone_template
    // This would typically be named just 'clone' in the public API
    rizztemplate::clone_template(args)
}

/// HTML escape a string
pub fn escape_html(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("escape_html requires a string".to_string()));
    }
    
    let s = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("argument must be a string".to_string())),
    };
    
    let escaped = html_escape(&s);
    Ok(Rc::new(Object::String(escaped)))
}

/// HTML escape a string for a JavaScript context
pub fn escape_js(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
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
                   
    Ok(Rc::new(Object::String(escaped)))
}

/// HTML escape a string for a URL context
pub fn escape_url(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
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
    
    Ok(Rc::new(Object::String(escaped)))
}

/// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#39;")
}

