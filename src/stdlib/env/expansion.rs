use crate::error::CursedError;
/// Environment variable expansion and substitution for CURSED standard library

use std::collections::HashMap;

use super::core::get_env;
use super::error::{EnvError, EnvResult, expansion_error};

/// Expand environment variables in a string
/// 
/// Supports the following formats:
/// - `${VAR}` - Standard format, errors if variable doesn't exist
/// - `${VAR:-default}` - Use default value if variable doesn't exist
/// - `${VAR:+value}` - Use value if variable exists, empty otherwise
/// - `$VAR` - Simplified format (letters, numbers, underscores only)
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::{set_env, expand_env_vars};
/// 
/// set_env("USER", "alice")?;
/// set_env("HOME", "/home/alice")?;
/// 
/// let expanded = expand_env_vars("Welcome $USER, your home is ${HOME}/documents")?;
/// // Result: "Welcome alice, your home is /home/alice/documents"
/// ```
pub fn expand_env_vars(input: &str) -> EnvResult<String> {
    expand_env_vars_with_custom_resolver(input, &get_env)
}

/// Expand environment variables with a custom default map
/// 
/// Uses provided defaults when variables are not found in environment.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::expand_env_vars_with_defaults;
/// use std::collections::HashMap;
/// 
/// let mut defaults = HashMap::new();
/// defaults.insert("USER".to_string(), "guest".to_string());
/// 
/// let expanded = expand_env_vars_with_defaults("Hello $USER", &defaults)?;
/// ```
pub fn expand_env_vars_with_defaults(
    input: &str, 
    defaults: &HashMap<String, String>
) -> EnvResult<String> {
    expand_env_vars_with_custom_resolver(input, &|key| {
        get_env(key).or_else(|| defaults.get(key).cloned())
    })
}

/// Check if a string contains environment variable references
/// 
/// Returns true if the string contains `$VAR` or `${VAR}` patterns.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::has_env_vars;
/// 
/// assert!(has_env_vars("Path: $HOME/documents"));
/// assert!(has_env_vars("User: ${USER}"));
/// assert!(!has_env_vars("No variables here"));
/// ```
pub fn has_env_vars(input: &str) -> bool {
    input.contains('$')
}

/// Validate environment variable syntax in a string
/// 
/// Checks for proper syntax without performing expansion.
/// Returns Ok(()) if syntax is valid, Err if malformed.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::validate_env_syntax;
/// 
/// assert!(validate_env_syntax("${HOME}/docs").is_ok());
/// assert!(validate_env_syntax("${INVALID").is_err()); // Missing }
/// ```
pub fn validate_env_syntax(input: &str) -> EnvResult<()> {
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '$' {
            match chars.peek() {
                Some('{') => {
                    chars.next(); // consume '{'
                    let mut found_closing = false;
                    let mut var_content = String::new();
                    
                    while let Some(ch) = chars.next() {
                        if ch == '}' {
                            found_closing = true;
                            break;
                        }
                        var_content.push(ch);
                    }
                    
                    if !found_closing {
                        return Err(expansion_error(input, "Unclosed ${} variable reference"));
                    }
                    
                    if var_content.is_empty() {
                        return Err(expansion_error(input, "Empty variable name in ${}"));
                    }
                }
                Some(c) if c.is_alphabetic() || *c == '_' => {
                    // Simple $VAR format - consume valid identifier characters
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                _ => {
                    return Err(expansion_error(input, "Invalid character after $"));
                }
            }
        }
    }
    
    Ok(())
}

/// Extract all environment variable names from a string
/// 
/// Returns a vector of unique variable names found in the string.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::extract_env_vars;
/// 
/// let vars = extract_env_vars("${USER} works in ${HOME}/projects")?;
/// assert!(vars.contains(&"USER".to_string()));
/// assert!(vars.contains(&"HOME".to_string()));
/// ```
pub fn extract_env_vars(input: &str) -> EnvResult<Vec<String>> {
    validate_env_syntax(input)?;
    
    let mut vars = std::collections::HashSet::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '$' {
            match chars.peek() {
                Some('{') => {
                    chars.next(); // consume '{'
                    let mut var_name = String::new();
                    
                    while let Some(ch) = chars.next() {
                        if ch == '}' {
                            break;
                        }
                        if ch == ':' {
                            // Handle ${VAR:-default} and ${VAR:+value} syntax
                            break;
                        }
                        var_name.push(ch);
                    }
                    
                    if !var_name.is_empty() {
                        vars.insert(var_name);
                    }
                }
                Some(c) if c.is_alphabetic() || *c == '_' => {
                    let mut var_name = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            var_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    vars.insert(var_name);
                }
                _ => {}
            }
        }
    }
    
    Ok(vars.into_iter().collect())
}

/// Substitute environment variables with explicit replacements
/// 
/// Replaces environment variables with values from the provided map.
/// Variables not in the map remain unexpanded.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::substitute_env_vars;
/// use std::collections::HashMap;
/// 
/// let mut replacements = HashMap::new();
/// replacements.insert("USER".to_string(), "bob".to_string());
/// 
/// let result = substitute_env_vars("Hello $USER", &replacements)?;
/// // Result: "Hello bob"
/// ```
pub fn substitute_env_vars(
    input: &str, 
    replacements: &HashMap<String, String>
) -> EnvResult<String> {
    expand_env_vars_with_custom_resolver(input, &|key| replacements.get(key).cloned())
}

/// Core expansion function with custom variable resolver
fn expand_env_vars_with_custom_resolver<F>(input: &str, resolver: &F) -> EnvResult<String>
where
    F: Fn(&str) -> Option<String>,
{
    validate_env_syntax(input)?;
    
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '$' {
            match chars.peek() {
                Some('{') => {
                    chars.next(); // consume '{'
                    let expansion_result = expand_braced_variable(&mut chars, resolver)?;
                    result.push_str(&expansion_result);
                }
                Some(c) if c.is_alphabetic() || *c == '_' => {
                    let var_name = consume_simple_variable(&mut chars);
                    if let Some(value) = resolver(&var_name) {
                        result.push_str(&value);
                    } else {
                        return Err(expansion_error(
                            input,
                            &format!("Environment variable '{}' not found", var_name)
                        ));
                    }
                }
                _ => {
                    result.push(ch);
                }
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok(result)
}

/// Expand a braced variable like ${VAR} or ${VAR:-default}
fn expand_braced_variable<F>(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    resolver: &F
) -> EnvResult<String>
where
    F: Fn(&str) -> Option<String>,
{
    let mut content = String::new();
    
    while let Some(ch) = chars.next() {
        if ch == '}' {
            break;
        }
        content.push(ch);
    }
    
    // Handle expansion modifiers
    if let Some(colon_pos) = content.find(':') {
        let var_name = &content[..colon_pos];
        let modifier = &content[colon_pos..];
        
        match modifier.chars().nth(1) {
            Some('-') => {
                // ${VAR:-default} - use default if variable is unset
                let default_value = &modifier[2..];
                Ok(resolver(var_name).unwrap_or_else(|| default_value.to_string()))
            }
            Some('+') => {
                // ${VAR:+value} - use value if variable is set
                let alt_value = &modifier[2..];
                if resolver(var_name).is_some() {
                    Ok(alt_value.to_string())
                } else {
                    Ok(String::new())
                }
            }
            _ => Err(expansion_error(&content, "Unsupported expansion modifier")),
        }
    } else {
        // Simple ${VAR} format
        if let Some(value) = resolver(&content) {
            Ok(value)
        } else {
            Err(expansion_error(&content, &format!("Environment variable '{}' not found", content)))
        }
    }
}

/// Consume a simple variable name like $VAR
fn consume_simple_variable(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut var_name = String::new();
    
    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' {
            var_name.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    
    var_name
}

/// Escape special characters in environment variable values
/// 
/// Escapes characters that have special meaning in environment variable expansion.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::escape_env_value;
/// 
/// let escaped = escape_env_value("Value with $ and {}")?;
/// // Result: "Value with \\$ and \\{\\}"
/// ```
pub fn escape_env_value(value: &str) -> String {
    value.chars()
        .map(|c| match c {
            '$' => "\\$".to_string(),
            '{' => "\\{".to_string(),
            '}' => "\\}".to_string(),
            '\\' => "\\\\".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

/// Unescape special characters in environment variable values
/// 
/// Unescapes characters that were escaped for environment variable storage.
/// 
/// # Examples
/// ```
/// use crate::stdlib::env::unescape_env_value;
/// 
/// let unescaped = unescape_env_value("Value with \\$ and \\{\\}")?;
/// // Result: "Value with $ and {}"
/// ```
pub fn unescape_env_value(value: &str) -> EnvResult<String> {
    let mut result = String::new();
    let mut chars = value.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('$') => result.push('$'),
                Some('{') => result.push('{'),
                Some('}') => result.push('}'),
                Some('\\') => result.push('\\'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok(result)
}

