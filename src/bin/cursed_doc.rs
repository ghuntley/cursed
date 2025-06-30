//! CURSED Documentation Generator binary

use clap::{Arg, Command, ArgMatches};
use std::path::{Path, PathBuf};
use std::process;
use std::fs;
use std::collections::HashMap;
use glob::glob;
use colored::*;
use serde_json::json;

fn main() {
    env_logger::init();
    
    let matches = build_cli().get_matches();
    
    if let Err(e) = run(matches) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn build_cli() -> Command {
    Command::new("cursed-doc")
        .version("0.1.0")
        .about("CURSED Documentation Generator - Generate docs from CURSED code")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("generate")
                .about("Generate documentation")
                .arg(Arg::new("input")
                    .help("Input directory or file")
                    .short('i')
                    .long("input")
                    .value_name("PATH")
                    .default_value("."))
                .arg(Arg::new("output")
                    .help("Output directory")
                    .short('o')
                    .long("output")
                    .value_name("DIR")
                    .default_value("docs"))
                .arg(Arg::new("format")
                    .help("Output format")
                    .short('f')
                    .long("format")
                    .value_name("FORMAT")
                    .value_parser(["html", "markdown", "json"])
                    .default_value("html"))
                .arg(Arg::new("title")
                    .help("Documentation title")
                    .short('t')
                    .long("title")
                    .value_name("TITLE")
                    .default_value("CURSED Documentation"))
        )
        .subcommand(
            Command::new("serve")
                .about("Serve documentation locally")
                .arg(Arg::new("docs-dir")
                    .help("Documentation directory")
                    .short('d')
                    .long("docs-dir")  
                    .value_name("DIR")
                    .default_value("docs"))
                .arg(Arg::new("port")
                    .help("Port to serve on")
                    .short('p')
                    .long("port")
                    .value_name("PORT")
                    .default_value("8080"))
        )
        .subcommand(
            Command::new("init")
                .about("Initialize documentation configuration")
        )
        .subcommand(
            Command::new("validate")
                .about("Validate documentation comments")
                .arg(Arg::new("input")
                    .help("Input directory or file")
                    .short('i')
                    .long("input")
                    .value_name("PATH")
                    .default_value("."))
        )
}

#[derive(Debug)]
struct DocItem {
    name: String,
    item_type: String,
    description: String,
    file_path: PathBuf,
    line_number: usize,
    parameters: Vec<Parameter>,
    return_type: Option<String>,
    examples: Vec<String>,
}

#[derive(Debug)]
struct Parameter {
    name: String,
    param_type: String,
    description: String,
}

#[derive(Debug)]
struct DocumentationIndex {
    items: Vec<DocItem>,
    files: Vec<PathBuf>,
}

fn run(matches: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").unwrap();
            let output = sub_matches.get_one::<String>("output").unwrap();
            let format = sub_matches.get_one::<String>("format").unwrap();
            let title = sub_matches.get_one::<String>("title").unwrap();
            
            println!("{}", "CURSED Documentation Generator".bold().cyan());
            println!("Input: {}", input);
            println!("Output: {}", output);
            println!("Format: {}", format);
            
            generate_docs(input, output, format, title)?;
        }
        
        Some(("serve", sub_matches)) => {
            let docs_dir = sub_matches.get_one::<String>("docs-dir").unwrap();
            let port: u16 = sub_matches.get_one::<String>("port").unwrap().parse()?;
            
            serve_docs(docs_dir, port)?;
        }
        
        Some(("init", _)) => {
            init_docs_config()?;
        }
        
        Some(("validate", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").unwrap();
            
            validate_docs(input)?;
        }
        
        _ => unreachable!()
    }
    
    Ok(())
}

fn generate_docs(input: &str, output: &str, format: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Scanning for source files...");
    
    let doc_index = scan_for_docs(input)?;
    
    println!("Found {} documented items in {} files", 
        doc_index.items.len(), 
        doc_index.files.len()
    );
    
    // Create output directory
    fs::create_dir_all(output)?;
    
    match format {
        "html" => generate_html_docs(&doc_index, output, title)?,
        "markdown" => generate_markdown_docs(&doc_index, output, title)?,
        "json" => generate_json_docs(&doc_index, output)?,
        _ => return Err("Unsupported format".into()),
    }
    
    println!("{} Documentation generated successfully in {}", "✓".green(), output);
    Ok(())
}

fn scan_for_docs(input: &str) -> Result<DocumentationIndex, Box<dyn std::error::Error>> {
    let mut items = Vec::new();
    let mut files = Vec::new();
    
    // Scan for CURSED source files
    let cursed_pattern = Path::new(input).join("**/*.csd");
    for entry in glob(&cursed_pattern.to_string_lossy())? {
        let path = entry?;
        if path.is_file() {
            files.push(path.clone());
            let file_items = parse_cursed_file(&path)?;
            items.extend(file_items);
        }
    }
    
    // Also scan Rust files for integration
    let rust_pattern = Path::new(input).join("**/*.rs");
    for entry in glob(&rust_pattern.to_string_lossy())? {
        let path = entry?;
        if path.is_file() {
            files.push(path.clone());
            let file_items = parse_rust_file(&path)?;
            items.extend(file_items);
        }
    }
    
    Ok(DocumentationIndex { items, files })
}

fn parse_cursed_file(file_path: &Path) -> Result<Vec<DocItem>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut items = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Look for documentation comments
        if line.starts_with("//!") || line.starts_with("///") {
            let doc_item = parse_doc_comment(&lines, i, file_path)?;
            if let Some(item) = doc_item {
                items.push(item);
            }
        }
        
        i += 1;
    }
    
    Ok(items)
}

fn parse_rust_file(file_path: &Path) -> Result<Vec<DocItem>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut items = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Look for Rust documentation comments
        if line.starts_with("///") || line.starts_with("//!") {
            let doc_item = parse_rust_doc_comment(&lines, i, file_path)?;
            if let Some(item) = doc_item {
                items.push(item);
            }
        }
        
        i += 1;
    }
    
    Ok(items)
}

fn parse_doc_comment(lines: &[&str], start_idx: usize, file_path: &Path) -> Result<Option<DocItem>, Box<dyn std::error::Error>> {
    let mut description = String::new();
    let mut examples = Vec::new();
    let mut i = start_idx;
    
    // Collect documentation lines
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with("//!") || line.starts_with("///") {
            let content = line.trim_start_matches("//!").trim_start_matches("///").trim();
            if content.starts_with("# Example") || content.starts_with("## Example") {
                // Start collecting example
                i += 1;
                while i < lines.len() && (lines[i].trim().starts_with("///") || lines[i].trim().starts_with("//!")) {
                    let example_line = lines[i].trim().trim_start_matches("///").trim_start_matches("//!").trim();
                    if !example_line.is_empty() {
                        examples.push(example_line.to_string());
                    }
                    i += 1;
                }
            } else {
                description.push_str(content);
                description.push('\n');
            }
        } else {
            break;
        }
        i += 1;
    }
    
    // Look for the next significant line (function, struct, etc.)
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }
    
    if i < lines.len() {
        let next_line = lines[i].trim();
        let (name, item_type) = if next_line.starts_with("fn ") {
            let name = extract_function_name(next_line);
            (name, "function".to_string())
        } else if next_line.starts_with("struct ") {
            let name = extract_struct_name(next_line);
            (name, "struct".to_string())
        } else if next_line.starts_with("enum ") {
            let name = extract_enum_name(next_line);
            (name, "enum".to_string())
        } else {
            ("unknown".to_string(), "item".to_string())
        };
        
        if name != "unknown" {
            return Ok(Some(DocItem {
                name,
                item_type,
                description: description.trim().to_string(),
                file_path: file_path.to_path_buf(),
                line_number: i + 1,
                parameters: Vec::new(), // TODO: Parse parameters
                return_type: None, // TODO: Parse return type
                examples,
            }));
        }
    }
    
    Ok(None)
}

fn parse_rust_doc_comment(lines: &[&str], start_idx: usize, file_path: &Path) -> Result<Option<DocItem>, Box<dyn std::error::Error>> {
    // Similar to parse_doc_comment but with Rust-specific parsing
    parse_doc_comment(lines, start_idx, file_path)
}

fn extract_function_name(line: &str) -> String {
    line.split_whitespace()
        .nth(1)
        .and_then(|s| s.split('(').next())
        .unwrap_or("unknown")
        .to_string()
}

fn extract_struct_name(line: &str) -> String {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("unknown")
        .to_string()
}

fn extract_enum_name(line: &str) -> String {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("unknown")
        .to_string()
}

fn generate_html_docs(doc_index: &DocumentationIndex, output: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let template = include_str!("../../docs/templates/index.html.template");
    
    let mut items_html = String::new();
    for item in &doc_index.items {
        items_html.push_str(&format!(
            r#"<div class="doc-item">
                <h3>{} <span class="type">({})</span></h3>
                <p class="description">{}</p>
                <p class="source">Source: {}:{}</p>
                {}
            </div>"#,
            item.name,
            item.item_type,
            item.description,
            item.file_path.display(),
            item.line_number,
            if !item.examples.is_empty() {
                format!("<div class=\"examples\"><h4>Examples:</h4><pre><code>{}</code></pre></div>", 
                    item.examples.join("\n"))
            } else {
                String::new()
            }
        ));
    }
    
    let html_content = template
        .replace("{{title}}", title)
        .replace("{{items}}", &items_html)
        .replace("{{count}}", &doc_index.items.len().to_string());
    
    let index_path = Path::new(output).join("index.html");
    fs::write(index_path, html_content)?;
    
    // Generate CSS
    let css_content = include_str!("../../docs/templates/style.css");
    let css_path = Path::new(output).join("style.css");
    fs::write(css_path, css_content)?;
    
    Ok(())
}

fn generate_markdown_docs(doc_index: &DocumentationIndex, output: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut markdown = format!("# {}\n\n", title);
    markdown.push_str(&format!("Generated documentation for {} items.\n\n", doc_index.items.len()));
    
    let mut items_by_type: HashMap<String, Vec<&DocItem>> = HashMap::new();
    for item in &doc_index.items {
        items_by_type.entry(item.item_type.clone()).or_default().push(item);
    }
    
    for (item_type, items) in items_by_type {
        markdown.push_str(&format!("## {}\n\n", item_type.to_uppercase()));
        
        for item in items {
            markdown.push_str(&format!("### {}\n\n", item.name));
            markdown.push_str(&format!("{}\n\n", item.description));
            markdown.push_str(&format!("**Source:** `{}:{}`\n\n", item.file_path.display(), item.line_number));
            
            if !item.examples.is_empty() {
                markdown.push_str("**Examples:**\n\n");
                markdown.push_str("```cursed\n");
                markdown.push_str(&item.examples.join("\n"));
                markdown.push_str("\n```\n\n");
            }
            
            markdown.push_str("---\n\n");
        }
    }
    
    let readme_path = Path::new(output).join("README.md");
    fs::write(readme_path, markdown)?;
    
    Ok(())
}

fn generate_json_docs(doc_index: &DocumentationIndex, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = json!({
        "title": "CURSED Documentation",
        "generated_at": chrono::Utc::now().to_rfc3339(),
        "items": doc_index.items.iter().map(|item| {
            json!({
                "name": item.name,
                "type": item.item_type,
                "description": item.description,
                "file_path": item.file_path.to_string_lossy(),
                "line_number": item.line_number,
                "examples": item.examples
            })
        }).collect::<Vec<_>>(),
        "files": doc_index.files.iter().map(|f| f.to_string_lossy()).collect::<Vec<_>>()
    });
    
    let json_path = Path::new(output).join("docs.json");
    fs::write(json_path, serde_json::to_string_pretty(&json_data)?)?;
    
    Ok(())
}

fn serve_docs(docs_dir: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} Starting documentation server...", "→".cyan());
    println!("Serving {} on http://localhost:{}", docs_dir, port);
    println!("Press Ctrl+C to stop");
    
    // Simple HTTP server using warp
    use warp::Filter;
    
    let docs = warp::fs::dir(docs_dir.to_string());
    let routes = docs.or(warp::path::end().and(warp::fs::file(format!("{}/index.html", docs_dir))));
    
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        warp::serve(routes)
            .run(([127, 0, 0, 1], port))
            .await;
    });
    
    Ok(())
}

fn init_docs_config() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing documentation configuration...");
    
    let config_content = r#"# CURSED Documentation Configuration
[documentation]
title = "My CURSED Project Documentation"
input_dir = "src"
output_dir = "docs"
format = "html"

[generator]
include_private = false
include_examples = true
generate_index = true
"#;
    
    fs::write("docs.toml", config_content)?;
    
    // Create docs templates directory
    fs::create_dir_all("docs/templates")?;
    
    let html_template = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{{title}}</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <header>
        <h1>{{title}}</h1>
        <p>Documentation for {{count}} items</p>
    </header>
    <main>
        {{items}}
    </main>
</body>
</html>"#;
    
    let css_template = r#"body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.6;
    color: #333;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

header {
    border-bottom: 2px solid #eee;
    margin-bottom: 30px;
    padding-bottom: 20px;
}

.doc-item {
    margin-bottom: 30px;
    padding: 20px;
    border: 1px solid #ddd;
    border-radius: 5px;
}

.type {
    color: #666;
    font-size: 0.8em;
}

.description {
    margin: 10px 0;
}

.source {
    color: #888;
    font-size: 0.9em;
}

.examples {
    margin-top: 15px;
}

.examples pre {
    background: #f5f5f5;
    padding: 10px;
    border-radius: 3px;
    overflow-x: auto;
}"#;
    
    fs::write("docs/templates/index.html.template", html_template)?;
    fs::write("docs/templates/style.css", css_template)?;
    
    println!("{} Documentation configuration initialized", "✓".green());
    println!("  - Created docs.toml");
    println!("  - Created docs/templates/");
    
    Ok(())
}

fn validate_docs(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Validating documentation...");
    
    let doc_index = scan_for_docs(input)?;
    let mut issues = Vec::new();
    
    // Check for undocumented items
    for file in &doc_index.files {
        let content = fs::read_to_string(file)?;
        let lines: Vec<&str> = content.lines().collect();
        
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Look for public functions without documentation
            if (line.starts_with("pub fn ") || line.starts_with("fn ")) && i > 0 {
                let prev_line = lines[i - 1].trim();
                if !prev_line.starts_with("///") && !prev_line.starts_with("//!") {
                    let func_name = extract_function_name(line);
                    issues.push(format!("{}:{} - Function '{}' is not documented", 
                        file.display(), i + 1, func_name));
                }
            }
            
            i += 1;
        }
    }
    
    if issues.is_empty() {
        println!("{} All items are properly documented", "✓".green());
    } else {
        println!("{} Found {} documentation issues:", "⚠".yellow(), issues.len());
        for issue in issues {
            println!("  {}", issue);
        }
    }
    
    Ok(())
}
