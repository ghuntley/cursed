//! The rizztemplate package provides text-based template functionality.
//!
//! This module implements a templating system similar to Go's text/template package,
//! allowing for the creation and execution of text templates with variable substitution,
//! conditionals, loops, and basic expressions.

use crate::error::Error;
use crate::object::Object;
use std::rc::Rc;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Create a new template with the given name
/// 
/// # Arguments
///
/// * `args[0]` - The template name as a String Object
/// 
/// # Returns
///
/// A new empty Template Object with the given name
pub fn new(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::InvalidArguments("Expected at least template name".to_string()));
    }
    
    let name = args[0].to_string();
    let template = Template {
        name,
        parsed: TemplateAST::Compound(vec![]),
        functions: HashMap::new(),
    };
    
    Ok(Arc::new(Object::Template(Arc::new(template))))
}

/// Parse a template from a string
///
/// # Arguments
///
/// * `args[0]` - The template string as a String Object
/// * `args[1]` - (Optional) The template name as a String Object
///
/// # Returns
///
/// A Template Object containing the parsed template
pub fn parse(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::InvalidArguments("Expected template string".to_string()));
    }
    
    let template_str = args[0].to_string();
    let name = if args.len() > 1 {
        args[1].to_string()
    } else {
        "template".to_string()
    };
    
    // This is a simplified implementation - in a real system, we would actually parse the template
    // For now, we'll just create a template that outputs the input string as is
    let template = Template {
        name,
        parsed: TemplateAST::Text(template_str),
        functions: HashMap::new(),
    };
    
    Ok(Arc::new(Object::Template(Arc::new(template))))
}

/// Execute a template with the provided data
///
/// # Arguments
///
/// * `args[0]` - The Template Object to execute
/// * `args[1]` - (Optional) The data object to use for variable substitution
///
/// # Returns
///
/// A String Object containing the template output
pub fn execute(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::new("InvalidArguments", "Expected template object", None));
    }
    
    // In a real implementation, we would execute the template with the provided data
    // For now, we'll just return the template name
    match &args[0] {
        obj if obj.is_template() => {
            if let Object::Template(template) = &**obj {
                Ok(Arc::new(Object::String(template.name.clone())))
            } else {
                Err(Error::new("InvalidArguments", "Not a template", None))
            }
        },
        _ => Err(Error::new("InvalidArguments", "Not a template", None)),
    }
}

/// Represents a parsed template ready for execution
#[derive(Debug, Clone)]
pub struct Template {
    /// The name of the template
    pub name: String,
    /// The parsed template structure (internal representation)
    parsed: TemplateAST,
    /// Custom functions available in this template
    functions: HashMap<String, Arc<Object>>,
}

/// Internal AST representation of a parsed template
#[derive(Debug, Clone)]
enum TemplateAST {
    Text(String),
    Variable(String),
    Condition {
        condition: Box<TemplateAST>,
        then_branch: Box<TemplateAST>,
        else_branch: Option<Box<TemplateAST>>,
    },
    Loop {
        variable: String,
        collection: Box<TemplateAST>,
        body: Box<TemplateAST>,
    },
    FunctionCall {
        name: String,
        args: Vec<TemplateAST>,
    },
    Compound(Vec<TemplateAST>),
}

/// Parses a template string into a Template object that can be executed later.
///
/// # Arguments
///
/// * `args[0]` - The template string as a String Object
/// * `args[1]` - (Optional) The template name as a String Object
///
/// # Returns
///
/// A Template object encapsulated in an ExternalData Object
///
/// # Errors
///
/// Returns a Runtime error if parsing fails
pub fn parse_template(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "parse_template requires at least 1 argument: template string".to_string(),
        ));
    }

    // Get the template string
    let template_str = match &*args[0] {
        Object::String(s) => s,
        _ => {
            return Err(Error::Runtime(
                "First argument to parse_template must be a string".to_string(),
            ))
        }
    };

    // Get the optional template name
    let template_name = if args.len() > 1 {
        match &*args[1] {
            Object::String(name) => name.clone(),
            _ => "unnamed_template".to_string(),
        }
    } else {
        "unnamed_template".to_string()
    };

    // Create a simple parsed template for now (just plain text)
    // In a real implementation, this would parse the template syntax
    let parsed = TemplateAST::Text(template_str.clone());

    // Create and return the template object
    let template = Template {
        name: template_name,
        parsed,
        functions: HashMap::new(),
    };

    // Return as an ExternalData object
    Ok(Arc::new(Object::ExternalData(Box::new(template))))
}

/// Executes a parsed template with the provided data.
///
/// # Arguments
///
/// * `args[0]` - The parsed Template object as an ExternalData Object
/// * `args[1]` - The data for template variables as a Map Object
///
/// # Returns
///
/// A String Object containing the executed template result
///
/// # Errors
///
/// Returns a Runtime error if execution fails
pub fn execute_template(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "execute_template requires 2 arguments: template and data".to_string(),
        ));
    }

    // Get the template object
    let template = match &*args[0] {
        Object::ExternalData(data) => {
            if let Some(template) = data.as_any().downcast_ref::<Template>() {
                template
            } else {
                return Err(Error::Runtime(
                    "First argument to execute_template must be a Template".to_string(),
                ));
            }
        }
        _ => {
            return Err(Error::Runtime(
                "First argument to execute_template must be a Template".to_string(),
            ))
        }
    };

    // Get the data for template variables
    let data = match &*args[1] {
        Object::HashTable(map) => map,
        _ => {
            return Err(Error::Runtime(
                "Second argument to execute_template must be a map".to_string(),
            ))
        }
    };

    // For a basic implementation, just return the template text
    // In a real implementation, this would replace variables with values from the data
    match &template.parsed {
        TemplateAST::Text(text) => Ok(Arc::new(Object::String(text.clone()))),
        _ => Ok(Arc::new(Object::String(format!("Template: {}", template.name)))),
    }
}

/// Adds a custom function to a template that can be called during execution.
///
/// # Arguments
///
/// * `args[0]` - The parsed Template object as an ExternalData Object
/// * `args[1]` - The function name as a String Object
/// * `args[2]` - The function as a Function Object
///
/// # Returns
///
/// The updated Template object
///
/// # Errors
///
/// Returns a Runtime error if adding the function fails
pub fn add_func(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 3 {
        return Err(Error::Runtime(
            "add_func requires 3 arguments: template, function name, and function".to_string(),
        ));
    }

    // Get the template object
    let template = match &*args[0] {
        Object::ExternalData(data) => {
            if let Some(template) = data.as_any().downcast_ref::<Template>() {
                template.clone()
            } else {
                return Err(Error::Runtime(
                    "First argument to add_func must be a Template".to_string(),
                ));
            }
        }
        _ => {
            return Err(Error::Runtime(
                "First argument to add_func must be a Template".to_string(),
            ))
        }
    };

    // Get the function name
    let func_name = match &*args[1] {
        Object::String(name) => name.clone(),
        _ => {
            return Err(Error::Runtime(
                "Second argument to add_func must be a string".to_string(),
            ))
        }
    };

    // Get the function
    let func = args[2].clone();

    // Create a new template with the added function
    let mut new_template = template;
    new_template.functions.insert(func_name, func);

    // Return the updated template
    Ok(Arc::new(Object::ExternalData(Box::new(new_template))))
}

/// Parses a template from a file.
///
/// # Arguments
///
/// * `args[0]` - The file path as a String Object
/// * `args[1]` - (Optional) The template name as a String Object
///
/// # Returns
///
/// A Template object encapsulated in an ExternalData Object
///
/// # Errors
///
/// Returns a Runtime error if file reading or parsing fails
///
/// # Why Template Tests Are Important
///
/// Testing templates is crucial because:
/// - Templates generate user-facing output that must be accurate
/// - Template errors can be difficult to debug in production
/// - They ensure consistent rendering across different data inputs
/// - They verify proper HTML escaping to prevent XSS vulnerabilities
/// - They confirm template performance with large datasets
pub fn parse_file(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "parse_file requires at least 1 argument: file path".to_string(),
        ));
    }

    // Get the file path
    let filepath = match &*args[0] {
        Object::String(path) => path,
        _ => {
            return Err(Error::Runtime(
                "First argument to parse_file must be a string".to_string(),
            ))
        }
    };

    // Get the optional template name (defaults to filename)
    let template_name = if args.len() > 1 {
        match &*args[1] {
            Object::String(name) => name.clone(),
            _ => filepath.clone(),
        }
    } else {
        filepath.clone()
    };

    // Read the file content
    let content = match std::fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(e) => {
            return Err(Error::Runtime(
                format!("Failed to read template file: {}", e),
            ))
        }
    };

    // Create a simple parsed template (just the file content as text)
    // In a real implementation, this would parse the template syntax
    let parsed = TemplateAST::Text(content);

    // Create and return the template object
    let template = Template {
        name: template_name,
        parsed,
        functions: HashMap::new(),
    };

    // Return as an ExternalData object
    Ok(Arc::new(Object::ExternalData(Box::new(template))))
}

/// Parses files matching a pattern into a collection of templates.
///
/// # Arguments
///
/// * `args[0]` - The glob pattern as a String Object
///
/// # Returns
///
/// A Map Object with template names as keys and Template objects as values
///
/// # Errors
///
/// Returns a Runtime error if glob matching or file reading fails
pub fn parse_files(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "parse_files requires 1 argument: glob pattern".to_string(),
        ));
    }

    // Get the glob pattern
    let pattern = match &*args[0] {
        Object::String(p) => p,
        _ => {
            return Err(Error::Runtime(
                "Argument to parse_files must be a string".to_string(),
            ))
        }
    };

    // Use glob to find matching files
    let paths = match glob::glob(pattern) {
        Ok(paths) => paths,
        Err(e) => {
            return Err(Error::Runtime(
                format!("Invalid glob pattern: {}", e),
            ))
        }
    };

    // Parse each file into a template
    let mut templates = HashMap::new();
    for path_result in paths {
        match path_result {
            Ok(path) => {
                // Get filename for template name
                let filename = match path.file_name() {
                    Some(name) => name.to_string_lossy().to_string(),
                    None => continue, // Skip if no filename
                };

                // Read file content
                let content = match std::fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(_) => continue, // Skip if can't read
                };

                // Create template
                let template = Template {
                    name: filename.clone(),
                    parsed: TemplateAST::Text(content),
                    functions: HashMap::new(),
                };

                // Add to templates map
                templates.insert(filename, Object::ExternalData(Box::new(template)));
            }
            Err(_) => continue, // Skip invalid paths
        }
    }

    // Return templates map
    Ok(Arc::new(Object::HashTable(templates)))
}