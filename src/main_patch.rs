/// Module for patching the main function with direct implementations
/// This module provides a general solution for handling dot expression calls
/// until the compiler fully supports them

use std::env;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;
use serde_json::{Value, json};

// Import the registry
use cursed::stdlib::dot_registry::{execute_dot, is_supported, get_packages, get_functions};

/// Regular expression to find dot expressions: package.function("arguments")
static DOT_EXPR_REGEX: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)\.([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)").unwrap()
});

/// Regular expression to find method calls on user-defined types: object.method("arguments")
static METHOD_CALL_REGEX: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)\.([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)").unwrap()
});

/// Regular expression to extract string arguments: "string"
static STRING_ARG_REGEX: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    Regex::new(r#""([^"]*)""#).unwrap()
});

/// Regular expression to extract number arguments: digits or decimal numbers
static NUMBER_ARG_REGEX: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    Regex::new(r"\b(\d+\.?\d*)\b").unwrap()
});

/// Regular expression to extract boolean arguments: true/false
static BOOL_ARG_REGEX: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    Regex::new(r"\b(true|false)\b").unwrap()
});

/// Regular expression to extract object definition: Person{...}
static OBJECT_DEF_REGEX: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)\s*\{([^}]*)\}").unwrap()
});

/// Regular expression to extract field definitions: name: "value"
static FIELD_DEF_REGEX: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*([^,}]+)").unwrap()
});

/// Detects if a file contains dot expression calls and processes them
/// Returns true if the file was processed and the dot expression output should be considered
/// the result, false if regular compilation should proceed
pub fn patch_for_vibez_spill(file_path: &str) -> bool {
    // Check if the file contains dot expressions we can handle
    if let Ok(content) = fs::read_to_string(file_path) {
        // Find all dot expressions in the content
        let mut dot_expressions = Vec::new();
        
        for cap in DOT_EXPR_REGEX.captures_iter(&content) {
            let package = &cap[1];
            let function = &cap[2];
            let args_str = &cap[3];
            
            // Check if we support this dot expression
            if is_supported(package, function) {
                // Extract arguments of various types
                let mut args = Vec::new();
                
                // Extract string arguments
                for arg_cap in STRING_ARG_REGEX.captures_iter(args_str) {
                    args.push(arg_cap[1].to_string());
                }
                
                // If no string arguments were found, look for numeric or boolean arguments
                if args.is_empty() {
                    // Extract number arguments
                    for arg_cap in NUMBER_ARG_REGEX.captures_iter(args_str) {
                        args.push(arg_cap[1].to_string());
                    }
                    
                    // Extract boolean arguments
                    for arg_cap in BOOL_ARG_REGEX.captures_iter(args_str) {
                        args.push(arg_cap[1].to_string());
                    }
                }
                
                dot_expressions.push((package.to_string(), function.to_string(), args));
            }
        }
        
        // If we found supported dot expressions, process them
        if !dot_expressions.is_empty() {
            // Execute each dot expression
            for (package, function, args) in dot_expressions {
                match execute_dot(&package, &function, args) {
                    Ok(result) => {
                        // Only print result for functions that produce output, like vibez.spill
                        if package == "vibez" && function == "spill" {
                            println!("{}", result);
                        } else if package == "htmlrizzler" && function == "escape_html" {
                            // Special handling for HTML escaping to show both original and escaped
                            println!("HTML escaped: {}", result);
                        } else if package == "timez" && function == "Now" {
                            println!("Current time: {}", result);
                        } else if package == "mathz" {
                            // Special handling for mathz functions
                            println!("{} result: {}", function, result);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error executing {}.{}: {}", package, function, e);
                    }
                }
            }
            
            return true;
        }
    }
    
    false
}

/// Parse a string argument to a JSON Value based on its content
fn parse_argument(arg: &str) -> Value {
    // Try to parse as a number
    if let Ok(num) = arg.parse::<f64>() {
        return json!(num);
    }
    
    // Check if it's a boolean
    match arg {
        "true" => return json!(true),
        "false" => return json!(false),
        _ => {}
    }
    
    // Default to a string
    json!(arg)
}