//! CURSED Documentation Generator binary

use clap::{Arg, Command, ArgMatches};
use std::path::{Path, PathBuf};
use std::process;
use std::fs;
use std::collections::HashMap;
use glob::glob;
use colored::*;
use serde_json::json;
use cursed::documentation::{DocumentationGenerator, DocConfig, GeneralConfig, InputConfig, OutputConfig, HtmlConfig, MarkdownConfig, ProcessingConfig, ValidationConfig, ExamplesConfig, ApiConfig};
use cursed::documentation::api_extractor::ApiExtractor;
use cursed::error::CursedError;

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
    
    // Use the enhanced documentation generator
    let config = create_doc_config(input, output, format, title)?;
    let mut generator = DocumentationGenerator::new(Some(".cursed-doc.toml"))?;
    
    // Generate documentation
    generator.generate()?;
    
    println!("{} Documentation generated successfully in {}", "✓".green(), output);
    
    // Generate documentation for stdlib specifically
    if Path::new("stdlib").exists() {
        println!("Generating stdlib documentation...");
        generate_stdlib_docs(output)?;
    }
    
    Ok(())
}

fn create_doc_config(input: &str, output: &str, format: &str, title: &str) -> Result<DocConfig, Box<dyn std::error::Error>> {
    let mut config = DocConfig {
        general: GeneralConfig {
            project_name: title.to_string(),
            project_version: "1.0.0".to_string(),
            project_description: "CURSED language documentation".to_string(),
            project_url: "https://github.com/ghuntley/cursed".to_string(),
            authors: vec!["CURSED Team".to_string()],
            license: "MIT".to_string(),
            repository: "https://github.com/ghuntley/cursed".to_string(),
        },
        input: InputConfig {
            source_dirs: vec![input.to_string()],
            include_patterns: vec!["*.csd".to_string()],
            exclude_patterns: vec![],
            max_file_size: 1024 * 1024, // 1MB
        },
        output: OutputConfig {
            output_dir: output.to_string(),
            formats: vec![format.to_string()],
            clean_output: true,
            base_url: "".to_string(),
        },
        html: HtmlConfig {
            theme: "default".to_string(),
            syntax_highlighting: true,
            search_enabled: true,
            table_of_contents: true,
            responsive_design: true,
            custom_css: vec![],
            custom_js: vec![],
            offline_mode: false,
        },
        markdown: MarkdownConfig {
            flavor: "github".to_string(),
            table_of_contents: true,
            code_block_style: "fenced".to_string(),
            link_style: "inline".to_string(),
        },
        processing: ProcessingConfig {
            extract_comments: true,
            extract_examples: true,
            generate_cross_references: true,
            analyze_dependencies: true,
            process_cursed_files: true,
            process_rust_files: false,
            process_markdown_files: false,
            cursed_comment_patterns: vec!["# ".to_string()],
        },
        validation: ValidationConfig {
            check_links: true,
            check_examples: true,
            validate_syntax: true,
            require_descriptions: false,
            treat_warnings_as_errors: false,
        },
        examples: ExamplesConfig {
            extract_examples: true,
            validate_examples: true,
            run_examples: true,
            categorize_by_directory: true,
            generate_example_index: true,
        },
        api: ApiConfig {
            generate_api_docs: true,
            include_private: false,
            include_internal: false,
            show_source_links: true,
            require_doc_comments: false,
            coverage_threshold: 0.8,
        },
    };
    
    // Update configuration based on parameters
    config.general.project_name = title.to_string();
    config.input.source_dirs = vec![input.to_string()];
    config.output.output_dir = output.to_string();
    config.output.formats = vec![format.to_string()];
    
    // Enable all features for comprehensive documentation
    config.html.search_enabled = true;
    config.html.syntax_highlighting = true;
    config.html.table_of_contents = true;
    config.processing.extract_comments = true;
    config.processing.extract_examples = true;
    config.processing.generate_cross_references = true;
    config.api.generate_api_docs = true;
    config.examples.generate_example_index = true;
    
    // Write configuration file
    let config_content = toml::to_string_pretty(&config)?;
    fs::write(".cursed-doc.toml", config_content)?;
    
    Ok(config)
}

fn generate_stdlib_docs(output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stdlib_dir = Path::new("stdlib");
    if !stdlib_dir.exists() {
        return Ok(());
    }
    
    let mut api_extractor = ApiExtractor::new()?;
    let mut stdlib_modules = Vec::new();
    
    // Process each stdlib module
    for entry in fs::read_dir(stdlib_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Look for mod.csd file
            let mod_file = path.join("mod.csd");
            if mod_file.exists() {
                let source = fs::read_to_string(&mod_file)?;
                match api_extractor.extract_api(&source, &mod_file.to_string_lossy()) {
                    Ok(module) => {
                        stdlib_modules.push(module);
                        println!("  ✓ Processed module: {}", path.file_name().unwrap().to_string_lossy());
                    }
                    Err(e) => {
                        eprintln!("  ⚠ Failed to process module {}: {}", 
                            path.file_name().unwrap().to_string_lossy(), e);
                    }
                }
            }
        }
    }
    
    // Generate stdlib index
    generate_stdlib_index(&stdlib_modules, output)?;
    
    println!("Generated documentation for {} stdlib modules", stdlib_modules.len());
    Ok(())
}

fn generate_stdlib_index(modules: &[cursed::documentation::DocumentedModule], output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut html = String::new();
    
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("    <title>CURSED Standard Library Documentation</title>\n");
    html.push_str("    <link rel=\"stylesheet\" href=\"assets/style.css\">\n");
    html.push_str("    <link rel=\"stylesheet\" href=\"assets/syntax.css\">\n");
    html.push_str("    <script src=\"assets/script.js\"></script>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("    <div class=\"progress-bar\"></div>\n");
    html.push_str("    <header class=\"header\">\n");
    html.push_str("        <div class=\"container\">\n");
    html.push_str("            <h1 class=\"logo\">CURSED Standard Library</h1>\n");
    html.push_str("            <nav class=\"nav\">\n");
    html.push_str("                <a href=\"index.html\">Home</a>\n");
    html.push_str("                <a href=\"#modules\">Modules</a>\n");
    html.push_str("                <a href=\"api.html\">API Reference</a>\n");
    html.push_str("            </nav>\n");
    html.push_str("        </div>\n");
    html.push_str("    </header>\n");
    html.push_str("    \n");
    html.push_str("    <main class=\"main\">\n");
    html.push_str("        <div class=\"container\">\n");
    html.push_str("            <section class=\"hero\">\n");
    html.push_str("                <h2>CURSED Standard Library</h2>\n");
    html.push_str("                <p>Complete documentation for all standard library modules</p>\n");
    html.push_str("                <div class=\"search-container\">\n");
    html.push_str("                    <input type=\"text\" class=\"search-input\" placeholder=\"Search stdlib modules (Ctrl+K)\">\n");
    html.push_str("                    <div class=\"search-results\"></div>\n");
    html.push_str("                </div>\n");
    html.push_str("            </section>\n");
    html.push_str("            \n");
    html.push_str("            <section id=\"modules\" class=\"modules\">\n");
    html.push_str("                <h2>Standard Library Modules</h2>\n");
    html.push_str("                <div class=\"module-grid\">\n");
    
    for module in modules {
        html.push_str(&format!(r#"
                    <div class="module-card">
                        <h3><a href="stdlib/{}.html">{}</a></h3>
                        <p>{}</p>
                        <div class="module-stats">
                            <span>{} functions</span>
                            <span>{} variables</span>
                            <span>{} types</span>
                        </div>
                    </div>
"#, 
            module.name, 
            module.name, 
            module.description,
            module.functions.len(),
            module.variables.len(),
            module.types.len()
        ));
    }
    
    html.push_str(r#"
                </div>
            </section>
        </div>
    </main>
    
    <footer class="footer">
        <div class="container">
            <p>&copy; 2024 CURSED Standard Library Documentation. Generated by cursed-doc.</p>
        </div>
    </footer>
</body>
</html>"#);
    
    let stdlib_index = Path::new(output).join("stdlib.html");
    fs::write(stdlib_index, html)?;
    
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
