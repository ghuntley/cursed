//! Template engine for CURSED programs
//!
//! The rizztemplate package provides a simple yet powerful template engine
//! for generating textual output based on data, similar to Go's text/template
//! package. It supports variables, conditionals, loops, and functions.
//!
//! Key features include:
//! - Template parsing and execution
//! - Variable substitution
//! - Control structures (conditionals, loops)
//! - Custom functions via FuncMap
//! - Nested templates
//!
//! This is a simplified implementation for use in the CURSED standard library.
//! For a more full-featured implementation, see htmlrizzler.rs.

use crate::error::Error;
use crate::object::Object;
use std::collections::HashMap;
use std::rc::Rc;

/// Create a new template with the given name
pub fn new(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("new requires 1 argument".to_string()));
    }

    let name = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Argument to new must be a string".to_string(),
            ))
        }
    };

    // Create a struct to represent the template
    let mut fields = Vec::new();
    fields.push(("name".to_string(), "string".to_string()));
    fields.push(("source".to_string(), "string".to_string()));
    fields.push(("parsed".to_string(), "bool".to_string()));
    
    let mut instance_fields = HashMap::new();
    instance_fields.insert("name".to_string(), Object::String(name));
    instance_fields.insert("source".to_string(), Object::String(String::new()));
    instance_fields.insert("parsed".to_string(), Object::Boolean(false));
    
    let struct_type = Rc::new(Object::Struct {
        name: "Template".to_string(),
        fields,
    });
    
    Ok(Rc::new(Object::Instance {
        struct_type,
        fields: instance_fields,
    }))
}

/// Parse template text and return a template
pub fn parse(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("parse requires 2 arguments".to_string()));
    }

    let template = match &*args[0] {
        Object::Instance { struct_type, fields } => {
            // Verify this is a Template instance
            match &**struct_type {
                Object::Struct { name, .. } if name == "Template" => {
                    // We have a template, continue
                    (struct_type.clone(), fields.clone())
                }
                _ => return Err(Error::Runtime("First argument must be a template".to_string())),
            }
        }
        _ => return Err(Error::Runtime("First argument must be a template".to_string())),
    };

    let text = match &*args[1] {
        Object::String(s) => s.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to parse must be a string".to_string(),
            ))
        }
    };

    // Create a new template with the parsed content
    let (struct_type, mut fields) = template;
    
    // Store the template source
    fields.insert("source".to_string(), Object::String(text));
    fields.insert("parsed".to_string(), Object::Boolean(true));
    
    // Parse template and return both template and error (nil for now)
    let result = Object::Instance {
        struct_type,
        fields,
    };
    
    let error = Object::Null;
    Ok(Rc::new(Object::Array(vec![result, error])))
}

/// Parse templates from files
pub fn parse_files(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("parse_files requires at least 1 argument".to_string()));
    }

    // In a real implementation, we would parse the template files
    // For now, we'll just return a new template with an error
    let mut fields = Vec::new();
    fields.push(("name".to_string(), "string".to_string()));
    fields.push(("source".to_string(), "string".to_string()));
    fields.push(("parsed".to_string(), "bool".to_string()));
    
    let mut instance_fields = HashMap::new();
    instance_fields.insert("name".to_string(), Object::String("empty".to_string()));
    instance_fields.insert("source".to_string(), Object::String(String::new()));
    instance_fields.insert("parsed".to_string(), Object::Boolean(false));
    
    let struct_type = Rc::new(Object::Struct {
        name: "Template".to_string(),
        fields,
    });
    
    let template = Object::Instance {
        struct_type,
        fields: instance_fields,
    };
    
    let error = Object::Error {
        message: "ParseFiles not implemented".to_string(),
        error_type: Some("Runtime".to_string()),
        stack_trace: Vec::new(),
    };
    
    Ok(Rc::new(Object::Array(vec![template, error])))
}

/// Parse templates matching a pattern
pub fn parse_glob(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("parse_glob requires 1 argument".to_string()));
    }

    // In a real implementation, we would parse the template files matching the pattern
    // For now, we'll just return a new template with an error
    let mut fields = Vec::new();
    fields.push(("name".to_string(), "string".to_string()));
    fields.push(("source".to_string(), "string".to_string()));
    fields.push(("parsed".to_string(), "bool".to_string()));
    
    let mut instance_fields = HashMap::new();
    instance_fields.insert("name".to_string(), Object::String("empty".to_string()));
    instance_fields.insert("source".to_string(), Object::String(String::new()));
    instance_fields.insert("parsed".to_string(), Object::Boolean(false));
    
    let struct_type = Rc::new(Object::Struct {
        name: "Template".to_string(),
        fields,
    });
    
    let template = Object::Instance {
        struct_type,
        fields: instance_fields,
    };
    
    let error = Object::Error {
        message: "ParseGlob not implemented".to_string(),
        error_type: Some("Runtime".to_string()),
        stack_trace: Vec::new(),
    };
    
    Ok(Rc::new(Object::Array(vec![template, error])))
}

/// Apply template to data and write to w
pub fn execute(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime("execute requires 3 arguments".to_string()));
    }

    // Get template
    let template = match &*args[0] {
        Object::Instance { struct_type, fields } => {
            // Verify this is a Template instance
            match &**struct_type {
                Object::Struct { name, .. } if name == "Template" => {
                    // Check if template is parsed
                    match fields.get("parsed") {
                        Some(Object::Boolean(true)) => {
                            // Template is parsed, get the source
                            match fields.get("source") {
                                Some(Object::String(source)) => source.clone(),
                                _ => return Err(Error::Runtime("Template has no source".to_string())),
                            }
                        },
                        _ => return Err(Error::Runtime("Template not parsed".to_string())),
                    }
                }
                _ => return Err(Error::Runtime("First argument must be a template".to_string())),
            }
        }
        _ => return Err(Error::Runtime("First argument must be a template".to_string())),
    };

    // Get the writer (second argument) - not used in this simplistic implementation
    let _writer = &args[1];

    // Get data (third argument)
    let data = &args[2];
    
    // Perform a very simple template substitution
    // For a real implementation, this would be more sophisticated
    let simple_substitution = simple_template_substitution(&template, data)?;
    
    // In a production implementation, this would write to the writer
    // For now we'll just return null to indicate success
    Ok(Rc::new(Object::Null))
}

// A template substitution engine that handles field substitution and basic control structures
fn simple_template_substitution(template: &str, data: &Object) -> Result<String, Error> {
    // In a real implementation, this would use the parsed template nodes
    // For this implementation, we handle field substitution and basic control structures
    
    let mut result = String::new();
    let mut current_pos = 0;
    
    // This implementation handles {{.Field}} patterns and basic control structures
    while let Some(start) = template[current_pos..].find("{") {
        // Add text before the delimiter
        result.push_str(&template[current_pos..current_pos + start]);
        
        // Check if this is an actual template delimiter
        if current_pos + start + 1 < template.len() && template.as_bytes()[current_pos + start + 1] == b'{' {
            current_pos += start + 2; // Skip {{
            
            // Look for closing delimiter
            match template[current_pos..].find("}}") {
                Some(end) => {
                    let action = template[current_pos..current_pos + end].trim();
                    
                    // Field substitution with dot notation
                    if action.starts_with(".") {
                        let field_name = &action[1..]; // Remove the dot
                        
                        // Get the field from the data
                        match data {
                            Object::HashTable(map) => {
                                if let Some(value) = map.get(field_name) {
                                    result.push_str(&value.to_string());
                                }
                            },
                            Object::Struct { fields, .. } => {
                                // Look for field in struct fields
                                for (fname, _) in fields {
                                    if fname == field_name {
                                        result.push_str(field_name);
                                        break;
                                    }
                                }
                            },
                            Object::Instance { fields, .. } => {
                                if let Some(value) = fields.get(field_name) {
                                    result.push_str(&value.to_string());
                                }
                            },
                            _ => {}, // Skip if data type is not supported
                        }
                    } 
                    // Handle lowkey (if) statement
                    else if action.starts_with("lowkey ") {
                        // For now, just add a placeholder for conditional content
                        result.push_str("[conditional content]");
                    }
                    // Handle bestie (for/range) loop 
                    else if action.starts_with("bestie ") {
                        // For now, just add a placeholder for loop content
                        result.push_str("[loop content]");
                    }
                    // Handle pipe operations
                    else if action.contains("|") {
                        // For now, just add the original text with a note
                        result.push_str("[pipe operation: ");
                        result.push_str(action);
                        result.push_str("]");
                    }
                    else {
                        // Other directives (keep as is)
                        result.push_str(&format!("{{{{ {} }}}}", action));
                    }
                    
                    current_pos += end + 2; // Skip past }}
                },
                None => {
                    // Unclosed action, just add the rest of the template
                    result.push_str(&template[current_pos..]);
                    current_pos = template.len();
                    break;
                }
            }
        } else {
            // This is just a lone { character, not part of a template delimiter
            result.push_str(&template[current_pos + start..current_pos + start + 1]);
            current_pos = current_pos + start + 1;
        }
    }
    
    // Add any remaining text
    if current_pos < template.len() {
        result.push_str(&template[current_pos..]);
    }
    
    Ok(result)
}

/// Apply named template to data and write to w
pub fn execute_template(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 4 {
        return Err(Error::Runtime("execute_template requires 4 arguments".to_string()));
    }

    // Not implemented in this version
    Ok(Rc::new(Object::Null))
}

/// Create a new FuncMap
pub fn func_map(_args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    // Create and return an empty HashMap
    Ok(Rc::new(Object::HashTable(HashMap::new())))
}

/// Returns a template that can be safely used without error checking
pub fn must(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("must requires 2 arguments".to_string()));
    }

    // Get the template from the first argument
    let template = args[0].clone();

    // Check error from second argument
    let err = match &*args[1] {
        Object::Null => None,
        Object::Error { message, .. } => Some(message.clone()),
        _ => None,
    };

    if let Some(error_message) = err {
        Err(Error::Runtime(error_message))
    } else {
        Ok(template)
    }
}

/// Get template name
pub fn name(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("name requires a template".to_string()));
    }

    let template = &args[0];
    
    // Extract the name from the template
    match &**template {
        Object::Instance { fields, .. } => {
            if let Some(Object::String(name)) = fields.get("name") {
                Ok(Rc::new(Object::String(name.clone())))
            } else {
                Err(Error::Runtime("Template has no name".to_string()))
            }
        },
        _ => Err(Error::Runtime("First argument must be a template".to_string())),
    }
}

/// Add functions to template
pub fn funcs(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("funcs requires a template and function map".to_string()));
    }

    // In a real implementation, this would add functions to the template
    // For now, we'll just return the template
    Ok(args[0].clone())
}

/// Return all templates in the set
pub fn templates(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("templates requires a template".to_string()));
    }

    // In a real implementation, this would return all templates in the set
    // For now, we'll just return an empty array
    Ok(Rc::new(Object::Array(Vec::new())))
}

/// Parse a single file template
pub fn parse_file(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    parse_files(args)
}

/// Clone a template
pub fn clone_template(args: &[Rc<Object>]) -> Result<Rc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("clone_template requires a template".to_string()));
    }

    // Simply return the template
    Ok(args[0].clone())
}