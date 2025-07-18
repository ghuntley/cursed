//! Basic CURSED Documentation Generator
//! 
//! Generates simple HTML documentation from CURSED stdlib modules.

use clap::{Arg, Command};
use std::path::{Path, PathBuf};
use std::fs;
use colored::*;
use glob::glob;
use regex::Regex;
use serde_json::json;

fn main() {
    let matches = Command::new("cursed-doc-basic")
        .version("0.1.0")
        .about("Basic CURSED Documentation Generator")
        .arg(Arg::new("input")
            .help("Input directory")
            .short('i')
            .long("input")
            .value_name("DIR")
            .default_value("stdlib"))
        .arg(Arg::new("output")
            .help("Output directory")
            .short('o')
            .long("output")
            .value_name("DIR")
            .default_value("docs"))
        .get_matches();

    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    
    if let Err(e) = generate_docs(input, output) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    description: String,
    functions: Vec<Function>,
    variables: Vec<Variable>,
}

#[derive(Debug, Clone)]
struct Function {
    name: String,
    description: String,
    signature: String,
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    description: String,
    var_type: String,
}

fn generate_docs(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "CURSED Documentation Generator".bold().cyan());
    println!("Scanning {} for modules...", input);
    
    let modules = scan_modules(input)?;
    println!("Found {} modules", modules.len());
    
    // Create output directory
    fs::create_dir_all(output)?;
    
    // Generate documentation files
    generate_index(&modules, output)?;
    generate_module_pages(&modules, output)?;
    
    println!("{} Documentation generated in {}", "✓".green(), output);
    Ok(())
}

fn scan_modules(input: &str) -> Result<Vec<Module>, Box<dyn std::error::Error>> {
    let mut modules = Vec::new();
    
    let pattern = format!("{}/**/mod.csd", input);
    for entry in glob(&pattern)? {
        let path = entry?;
        if let Some(module) = parse_module(&path)? {
            modules.push(module);
        }
    }
    
    Ok(modules)
}

fn parse_module(file_path: &Path) -> Result<Option<Module>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    
    let module_name = file_path.parent().unwrap().file_name().unwrap().to_string_lossy().to_string();
    
    let mut module = Module {
        name: module_name,
        description: extract_description(&content),
        functions: extract_functions(&content)?,
        variables: extract_variables(&content)?,
    };
    
    Ok(Some(module))
}

fn extract_description(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut description = String::new();
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            description.push_str(trimmed.trim_start_matches("# ").trim());
            description.push(' ');
        } else if !trimmed.is_empty() && !trimmed.starts_with('#') {
            break;
        }
    }
    
    description.trim().to_string()
}

fn extract_functions(content: &str) -> Result<Vec<Function>, Box<dyn std::error::Error>> {
    let mut functions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    let function_regex = Regex::new(r"^\s*slay\s+(\w+)\s*\(")?;
    
    for (i, line) in lines.iter().enumerate() {
        if let Some(captures) = function_regex.captures(line) {
            let function_name = captures.get(1).unwrap().as_str().to_string();
            
            // Look for preceding comments
            let mut description = String::new();
            if i > 0 {
                let prev_line = lines[i - 1].trim();
                if prev_line.starts_with("# ") {
                    description = prev_line.trim_start_matches("# ").trim().to_string();
                }
            }
            
            functions.push(Function {
                name: function_name,
                description,
                signature: line.trim().to_string(),
            });
        }
    }
    
    Ok(functions)
}

fn extract_variables(content: &str) -> Result<Vec<Variable>, Box<dyn std::error::Error>> {
    let mut variables = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    let variable_regex = Regex::new(r"^\s*sus\s+(\w+)\s+(\w+)\s*=")?;
    
    for (i, line) in lines.iter().enumerate() {
        if let Some(captures) = variable_regex.captures(line) {
            let var_name = captures.get(1).unwrap().as_str().to_string();
            let var_type = captures.get(2).unwrap().as_str().to_string();
            
            let mut description = String::new();
            if i > 0 {
                let prev_line = lines[i - 1].trim();
                if prev_line.starts_with("# ") {
                    description = prev_line.trim_start_matches("# ").trim().to_string();
                }
            }
            
            variables.push(Variable {
                name: var_name,
                description,
                var_type,
            });
        }
    }
    
    Ok(variables)
}

fn generate_index(modules: &[Module], output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut html = String::new();
    
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("    <title>CURSED Standard Library Documentation</title>\n");
    html.push_str("    <style>\n");
    html.push_str("        body { font-family: Arial, sans-serif; margin: 40px; }\n");
    html.push_str("        h1 { color: #333; }\n");
    html.push_str("        .module { margin: 20px 0; padding: 20px; border: 1px solid #ddd; }\n");
    html.push_str("        .module h2 { color: #2c3e50; }\n");
    html.push_str("        .function { margin: 10px 0; padding: 10px; background: #f9f9f9; }\n");
    html.push_str("        .variable { margin: 10px 0; padding: 10px; background: #f0f0f0; }\n");
    html.push_str("        code { background: #e8e8e8; padding: 2px 4px; }\n");
    html.push_str("    </style>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("    <h1>CURSED Standard Library Documentation</h1>\n");
    html.push_str("    <p>Auto-generated documentation for CURSED standard library modules.</p>\n");
    html.push_str("    <h2>Modules</h2>\n");
    html.push_str("    <ul>\n");
    
    for module in modules {
        html.push_str(&format!("        <li><a href=\"{}.html\">{}</a> - {}</li>\n", 
            module.name, module.name, module.description));
    }
    
    html.push_str("    </ul>\n");
    html.push_str("</body>\n");
    html.push_str("</html>\n");
    
    fs::write(Path::new(output).join("index.html"), html)?;
    Ok(())
}

fn generate_module_pages(modules: &[Module], output: &str) -> Result<(), Box<dyn std::error::Error>> {
    for module in modules {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("    <title>{} Module - CURSED Documentation</title>\n", module.name));
        html.push_str("    <style>\n");
        html.push_str("        body { font-family: Arial, sans-serif; margin: 40px; }\n");
        html.push_str("        h1 { color: #333; }\n");
        html.push_str("        .function { margin: 20px 0; padding: 15px; border: 1px solid #ddd; }\n");
        html.push_str("        .variable { margin: 20px 0; padding: 15px; border: 1px solid #ccc; background: #f9f9f9; }\n");
        html.push_str("        code { background: #e8e8e8; padding: 2px 4px; }\n");
        html.push_str("        .back-link { margin-bottom: 20px; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("    <div class=\"back-link\"><a href=\"index.html\">&larr; Back to Index</a></div>\n");
        html.push_str(&format!("    <h1>{} Module</h1>\n", module.name));
        html.push_str(&format!("    <p>{}</p>\n", module.description));
        
        if !module.functions.is_empty() {
            html.push_str("    <h2>Functions</h2>\n");
            for function in &module.functions {
                html.push_str("    <div class=\"function\">\n");
                html.push_str(&format!("        <h3>{}</h3>\n", function.name));
                html.push_str(&format!("        <p>{}</p>\n", function.description));
                html.push_str(&format!("        <p><code>{}</code></p>\n", escape_html(&function.signature)));
                html.push_str("    </div>\n");
            }
        }
        
        if !module.variables.is_empty() {
            html.push_str("    <h2>Variables</h2>\n");
            for variable in &module.variables {
                html.push_str("    <div class=\"variable\">\n");
                html.push_str(&format!("        <h3>{}</h3>\n", variable.name));
                html.push_str(&format!("        <p>{}</p>\n", variable.description));
                html.push_str(&format!("        <p>Type: <code>{}</code></p>\n", variable.var_type));
                html.push_str("    </div>\n");
            }
        }
        
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        fs::write(Path::new(output).join(format!("{}.html", module.name)), html)?;
    }
    
    Ok(())
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
