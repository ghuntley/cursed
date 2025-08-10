//! Simple CURSED Documentation Generator
//! 
//! A streamlined version of the documentation generator focused on
//! generating HTML documentation from CURSED source files.

use clap::{Arg, Command};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use colored::*;
use glob::glob;
use regex::Regex;
use serde_json::json;

fn main() {
    env_logger::init();
    
    let matches = Command::new("cursed-doc-simple")
        .version("0.1.0")
        .about("Simple CURSED Documentation Generator")
        .subcommand(
            Command::new("generate")
                .about("Generate documentation")
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
                .arg(Arg::new("title")
                    .help("Documentation title")
                    .short('t')
                    .long("title")
                    .value_name("TITLE")
                    .default_value("CURSED Documentation"))
        )
        .subcommand(
            Command::new("serve")
                .about("Serve documentation")
                .arg(Arg::new("port")
                    .help("Port to serve on")
                    .short('p')
                    .long("port")
                    .value_name("PORT")
                    .default_value("8080"))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let input = sub_matches.get_one::<String>("input").unwrap();
            let output = sub_matches.get_one::<String>("output").unwrap();
            let title = sub_matches.get_one::<String>("title").unwrap();
            
            if let Err(e) = generate_docs(input, output, title) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(("serve", sub_matches)) => {
            let port: u16 = sub_matches.get_one::<String>("port").unwrap().parse().unwrap();
            serve_docs("docs", port);
        }
        _ => {
            eprintln!("No subcommand specified. Use --help for usage information.");
            std::process::exit(1);
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    description: String,
    file_path: PathBuf,
    functions: Vec<Function>,
    variables: Vec<Variable>,
}

#[derive(Debug, Clone)]
struct Function {
    name: String,
    description: String,
    signature: String,
    parameters: Vec<String>,
    return_type: String,
    examples: Vec<String>,
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    description: String,
    var_type: String,
}

fn generate_docs(input: &str, output: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "CURSED Documentation Generator".bold().cyan());
    println!("Scanning {} for CURSED modules...", input);
    
    let modules = scan_modules(input)?;
    
    println!("Found {} modules", modules.len());
    
    // Create output directory
    fs::create_dir_all(output)?;
    
    // Copy assets
    copy_assets(output)?;
    
    // Generate index page
    generate_index_page(&modules, output, title)?;
    
    // Generate module pages
    generate_module_pages(&modules, output)?;
    
    // Generate search index
    generate_search_index(&modules, output)?;
    
    println!("{} Documentation generated successfully in {}", "✓".green(), output);
    Ok(())
}

fn scan_modules(input: &str) -> Result<Vec<Module>, Box<dyn std::error::Error>> {
    let mut modules = Vec::new();
    
    // Look for mod.csd files in subdirectories
    let pattern = format!("{}/**/mod.csd", input);
    for entry in glob(&pattern)? {
        let path = entry?;
        if let Some(module) = parse_module(&path)? {
            modules.push(module);
        }
    }
    
    // Also look for standalone .csd files
    let pattern = format!("{}/**/*.csd", input);
    for entry in glob(&pattern)? {
        let path = entry?;
        if path.file_name().unwrap() != "mod.csd" {
            if let Some(module) = parse_module(&path)? {
                modules.push(module);
            }
        }
    }
    
    Ok(modules)
}

fn parse_module(file_path: &Path) -> Result<Option<Module>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    
    let module_name = if file_path.file_name().unwrap() == "mod.csd" {
        file_path.parent().unwrap().file_name().unwrap().to_string_lossy().to_string()
    } else {
        file_path.file_stem().unwrap().to_string_lossy().to_string()
    };
    
    let mut module = Module {
        name: module_name,
        description: extract_module_description(&content),
        file_path: file_path.to_path_buf(),
        functions: Vec::new(),
        variables: Vec::new(),
    };
    
    // Extract functions
    module.functions = extract_functions(&content)?;
    
    // Extract variables
    module.variables = extract_variables(&content)?;
    
    Ok(Some(module))
}

fn extract_module_description(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut description = String::new();
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            description.push_str(trimmed.trim_start_matches("# ").trim());
            description.push('\n');
        } else if trimmed.starts_with("//!") {
            description.push_str(trimmed.trim_start_matches("//!").trim());
            description.push('\n');
        } else if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("//") {
            break;
        }
    }
    
    description.trim().to_string()
}

fn extract_functions(content: &str) -> Result<Vec<Function>, Box<dyn std::error::Error>> {
    let mut functions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Function pattern: slay function_name(params) return_type {
    let function_regex = Regex::new(r"^\s*slay\s+(\w+)\s*\((.*?)\)(?:\s*(\w+))?\s*\{")?;
    
    for (i, line) in lines.iter().enumerate() {
        if let Some(captures) = function_regex.captures(line) {
            let function_name = captures.get(1).unwrap().as_str().to_string();
            let params = captures.get(2).map_or("", |m| m.as_str());
            let return_type = captures.get(3).map_or("", |m| m.as_str());
            
            // Look for preceding comments
            let mut description = String::new();
            let mut examples = Vec::new();
            let mut j = i.saturating_sub(1);
            
            while j > 0 {
                let comment_line = lines[j].trim();
                if comment_line.starts_with("# ") {
                    let comment_text = comment_line.trim_start_matches("# ").trim();
                    if comment_text.starts_with("Example:") {
                        examples.push(comment_text.to_string());
                    } else {
                        description = format!("{}\n{}", comment_text, description);
                    }
                } else if comment_line.starts_with("///") {
                    let comment_text = comment_line.trim_start_matches("///").trim();
                    description = format!("{}\n{}", comment_text, description);
                } else if !comment_line.is_empty() {
                    break;
                }
                j = j.saturating_sub(1);
            }
            
            functions.push(Function {
                name: function_name,
                description: description.trim().to_string(),
                signature: line.to_string(),
                parameters: params.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
                return_type: return_type.to_string(),
                examples,
            });
        }
    }
    
    Ok(functions)
}

fn extract_variables(content: &str) -> Result<Vec<Variable>, Box<dyn std::error::Error>> {
    let mut variables = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Variable pattern: sus variable_name type = value
    let variable_regex = Regex::new(r"^\s*sus\s+(\w+)\s+(\w+)\s*=")?;
    
    for (i, line) in lines.iter().enumerate() {
        if let Some(captures) = variable_regex.captures(line) {
            let var_name = captures.get(1).unwrap().as_str().to_string();
            let var_type = captures.get(2).unwrap().as_str().to_string();
            
            // Look for preceding comments
            let mut description = String::new();
            let mut j = i.saturating_sub(1);
            
            while j > 0 {
                let comment_line = lines[j].trim();
                if comment_line.starts_with("# ") {
                    let comment_text = comment_line.trim_start_matches("# ").trim();
                    description = format!("{}\n{}", comment_text, description);
                } else if comment_line.starts_with("///") {
                    let comment_text = comment_line.trim_start_matches("///").trim();
                    description = format!("{}\n{}", comment_text, description);
                } else if !comment_line.is_empty() {
                    break;
                }
                j = j.saturating_sub(1);
            }
            
            variables.push(Variable {
                name: var_name,
                description: description.trim().to_string(),
                var_type,
            });
        }
    }
    
    Ok(variables)
}

fn copy_assets(output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let assets_dir = Path::new(output).join("assets");
    fs::create_dir_all(&assets_dir)?;
    
    // Copy CSS
    let css_content = include_str!("../documentation/templates/style.css");
    fs::write(assets_dir.join("style.css"), css_content)?;
    
    // Copy JavaScript
    let js_content = include_str!("../documentation/templates/script.js");
    fs::write(assets_dir.join("script.js"), js_content)?;
    
    Ok(())
}

fn generate_index_page(modules: &[Module], output: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut html = String::new();
    
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!("    <title>{}</title>\n", title));
    html.push_str("    <link rel=\"stylesheet\" href=\"assets/style.css\">\n");
    html.push_str("    <script src=\"assets/script.js\"></script>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("    <div class=\"progress-bar\"></div>\n");
    html.push_str("    <header class=\"header\">\n");
    html.push_str("        <div class=\"container\">\n");
    html.push_str(&format!("            <h1 class=\"logo\">{}</h1>\n", title));
    html.push_str("            <nav class=\"nav\">\n");
    html.push_str("                <a href=\"#modules\">Modules</a>\n");
    html.push_str("                <a href=\"#search\">Search</a>\n");
    html.push_str("            </nav>\n");
    html.push_str("        </div>\n");
    html.push_str("    </header>\n");
    html.push_str("    \n");
    html.push_str("    <main class=\"main\">\n");
    html.push_str("        <div class=\"container\">\n");
    html.push_str("            <section class=\"hero\">\n");
    html.push_str(&format!("                <h2>{}</h2>\n", title));
    html.push_str("                <p>Complete documentation for all modules</p>\n");
    html.push_str("                <div class=\"search-container\">\n");
    html.push_str("                    <input type=\"text\" class=\"search-input\" placeholder=\"Search modules and functions...\">\n");
    html.push_str("                    <div class=\"search-results\"></div>\n");
    html.push_str("                </div>\n");
    html.push_str("            </section>\n");
    html.push_str("            \n");
    html.push_str("            <section id=\"modules\" class=\"modules\">\n");
    html.push_str("                <h2>Modules</h2>\n");
    html.push_str("                <div class=\"module-grid\">\n");
    
    for module in modules {
        html.push_str(&format!(r#"
                    <div class="module-card">
                        <h3><a href="modules/{}.html">{}</a></h3>
                        <p>{}</p>
                        <div class="module-stats">
                            <span>{} functions</span>
                            <span>{} variables</span>
                        </div>
                    </div>
"#, 
            module.name, 
            module.name, 
            module.description,
            module.functions.len(),
            module.variables.len()
        ));
    }
    
    html.push_str(r#"
                </div>
            </section>
        </div>
    </main>
    
    <footer class="footer">
        <div class="container">
            <p>&copy; 2024 CURSED Documentation. Generated by cursed-doc.</p>
        </div>
    </footer>
</body>
</html>"#);
    
    fs::write(Path::new(output).join("index.html"), html)?;
    Ok(())
}

fn generate_module_pages(modules: &[Module], output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let modules_dir = Path::new(output).join("modules");
    fs::create_dir_all(&modules_dir)?;
    
    for module in modules {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("    <title>{} Module</title>\n", module.name));
        html.push_str("    <link rel=\"stylesheet\" href=\"../assets/style.css\">\n");
        html.push_str("    <script src=\"../assets/script.js\"></script>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("    <header class=\"header\">\n");
        html.push_str("        <div class=\"container\">\n");
        html.push_str("            <h1 class=\"logo\"><a href=\"../index.html\">CURSED Docs</a></h1>\n");
        html.push_str("            <nav class=\"nav\">\n");
        html.push_str("                <a href=\"../index.html\">Home</a>\n");
        html.push_str("                <a href=\"#functions\">Functions</a>\n");
        html.push_str("                <a href=\"#variables\">Variables</a>\n");
        html.push_str("            </nav>\n");
        html.push_str("        </div>\n");
        html.push_str("    </header>\n");
        html.push_str("    \n");
        html.push_str("    <main class=\"main\">\n");
        html.push_str("        <div class=\"container\">\n");
        html.push_str("            <div class=\"module-header\">\n");
        html.push_str(&format!("                <h1>{} Module</h1>\n", module.name));
        html.push_str(&format!("                <p>{}</p>\n", module.description));
        html.push_str("                <div class=\"module-info\">\n");
        html.push_str(&format!("                    <span class=\"source-file\">Source: {}</span>\n", module.file_path.display()));
        html.push_str("                </div>\n");
        html.push_str("            </div>\n");
        html.push_str("            \n");
        html.push_str("            <section id=\"functions\" class=\"functions\">\n");
        html.push_str("                <h2>Functions</h2>\n");
        
        for function in &module.functions {
            html.push_str(&format!(r#"
                <div class="function">
                    <div class="function-header">
                        <h3>{}</h3>
                        <div class="function-signature">
                            <code>{}</code>
                        </div>
                    </div>
                    <div class="function-body">
                        <p>{}</p>
                        {}
                        {}
                    </div>
                </div>
"#, 
                function.name,
                escape_html(&function.signature),
                function.description,
                if !function.parameters.is_empty() {
                    format!("<h4>Parameters</h4><ul>{}</ul>", 
                        function.parameters.iter()
                            .map(|p| format!("<li><code>{}</code></li>", escape_html(p)))
                            .collect::<Vec<_>>()
                            .join(""))
                } else {
                    String::new()
                },
                if !function.examples.is_empty() {
                    format!("<h4>Examples</h4>{}", 
                        function.examples.iter()
                            .map(|e| format!("<pre><code>{}</code></pre>", escape_html(e)))
                            .collect::<Vec<_>>()
                            .join(""))
                } else {
                    String::new()
                }
            ));
        }
        
        html.push_str(r#"
            </section>
            
            <section id="variables" class="variables">
                <h2>Variables</h2>
"#);
        
        for variable in &module.variables {
            html.push_str(&format!(r#"
                <div class="variable">
                    <h3>{}</h3>
                    <div class="variable-type"><code>{}</code></div>
                    <p>{}</p>
                </div>
"#, 
                variable.name,
                variable.var_type,
                variable.description
            ));
        }
        
        html.push_str(r#"
            </section>
        </div>
    </main>
    
    <footer class="footer">
        <div class="container">
            <p>&copy; 2024 CURSED Documentation. Generated by cursed-doc.</p>
        </div>
    </footer>
</body>
</html>"#);
        
        fs::write(modules_dir.join(format!("{}.html", module.name)), html)?;
    }
    
    Ok(())
}

fn generate_search_index(modules: &[Module], output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut search_items = Vec::new();
    
    for module in modules {
        // Add module to search
        search_items.push(json!({
            "title": format!("{} Module", module.name),
            "url": format!("modules/{}.html", module.name),
            "content": module.description,
            "type": "module"
        }));
        
        // Add functions to search
        for function in &module.functions {
            search_items.push(json!({
                "title": function.name,
                "url": format!("modules/{}.html#functions", module.name),
                "content": function.description,
                "type": "function"
            }));
        }
        
        // Add variables to search
        for variable in &module.variables {
            search_items.push(json!({
                "title": variable.name,
                "url": format!("modules/{}.html#variables", module.name),
                "content": variable.description,
                "type": "variable"
            }));
        }
    }
    
    let search_data = serde_json::to_string_pretty(&search_items)?;
    fs::write(Path::new(output).join("search.json"), search_data)?;
    
    Ok(())
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn serve_docs(docs_dir: &str, port: u16) {
    use std::net::{TcpListener, TcpStream};
    use std::io::{BufRead, BufReader, Write};
    use std::thread;
    
    println!("{} Starting documentation server...", "→".cyan());
    println!("Serving {} on http://localhost:{}", docs_dir, port);
    println!("Press Ctrl+C to stop");
    
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let docs_dir = docs_dir.to_string();
        
        thread::spawn(move || {
            handle_request(stream, &docs_dir);
        });
    }
}

fn handle_request(mut stream: TcpStream, docs_dir: &str) {
    let reader = BufReader::new(&stream);
    let request_line = reader.lines().next().unwrap().unwrap();
    
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    
    let method = parts[0];
    let path = parts[1];
    
    if method != "GET" {
        let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }
    
    let file_path = if path == "/" {
        format!("{}/index.html", docs_dir)
    } else {
        format!("{}{}", docs_dir, path)
    };
    
    match fs::read(&file_path) {
        Ok(contents) => {
            let content_type = get_content_type(&file_path);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                content_type,
                contents.len()
            );
            
            stream.write_all(response.as_bytes()).unwrap();
            stream.write_all(&contents).unwrap();
        }
        Err(_) => {
            let response = "HTTP/1.1 404 Not Found\r\n\r\n<h1>404 Not Found</h1>";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn get_content_type(path: &str) -> &str {
    if path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".json") {
        "application/json"
    } else {
        "text/plain"
    }
}
