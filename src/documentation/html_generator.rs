//! HTML Documentation Generator
//! 
//! Generates comprehensive HTML documentation with responsive design,
//! syntax highlighting, and interactive features.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::error::CursedError;
use crate::documentation::{DocConfig, Documentation, DocumentedModule, DocumentedFunction};

/// HTML documentation generator
pub struct HtmlGenerator<'a> {
    config: &'a DocConfig,
}

impl<'a> HtmlGenerator<'a> {
    /// Create new HTML generator
    pub fn new(config: &'a DocConfig) -> Self {
        Self { config }
    }

    /// Generate HTML documentation
    pub fn generate(&self, documentation: &Documentation) -> Result<(), CursedError> {
        let output_dir = Path::new(&self.config.output.output_dir);
        
        // Create HTML output directory
        let html_dir = output_dir.join("html");
        fs::create_dir_all(&html_dir)
            .map_err(|e| CursedError::IoError(format!("Failed to create HTML directory: {}", e)))?;

        // Generate CSS and JS assets
        self.generate_assets(&html_dir)?;

        // Generate main index page
        self.generate_index(&html_dir, documentation)?;

        // Generate module pages
        for module in &documentation.modules {
            self.generate_module_page(&html_dir, module, documentation)?;
        }

        // Generate search index
        if self.config.html.search_enabled {
            self.generate_search_index(&html_dir, documentation)?;
        }

        // Generate API reference
        if self.config.api.generate_api_docs {
            self.generate_api_reference(&html_dir, documentation)?;
        }

        // Generate examples index
        if self.config.examples.generate_example_index {
            self.generate_examples_index(&html_dir, documentation)?;
        }

        Ok(())
    }

    /// Generate CSS and JavaScript assets
    fn generate_assets(&self, html_dir: &Path) -> Result<(), CursedError> {
        let assets_dir = html_dir.join("assets");
        fs::create_dir_all(&assets_dir)
            .map_err(|e| CursedError::IoError(format!("Failed to create assets directory: {}", e)))?;

        // Generate main CSS
        self.generate_css(&assets_dir)?;

        // Generate main JavaScript
        self.generate_js(&assets_dir)?;

        // Copy custom assets
        self.copy_custom_assets(&assets_dir)?;

        // Generate syntax highlighting CSS
        self.generate_syntax_css(&assets_dir)?;

        Ok(())
    }

    /// Generate main CSS file
    fn generate_css(&self, assets_dir: &Path) -> Result<(), CursedError> {
        let css_content = include_str!("templates/style.css");
        let css_file = assets_dir.join("style.css");
        
        let mut file = File::create(&css_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create CSS file: {}", e)))?;

        file.write_all(css_content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write CSS file: {}", e)))?;

        Ok(())
    }

    /// Generate main JavaScript file
    fn generate_js(&self, assets_dir: &Path) -> Result<(), CursedError> {
        let js_content = include_str!("templates/script.js");
        let js_file = assets_dir.join("script.js");
        
        let mut file = File::create(&js_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create JS file: {}", e)))?;

        file.write_all(js_content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write JS file: {}", e)))?;

        Ok(())
    }

    /// Generate syntax highlighting CSS
    fn generate_syntax_css(&self, assets_dir: &Path) -> Result<(), CursedError> {
        let syntax_css = r#"
/* Syntax highlighting for CURSED language */
.highlight {
    background-color: #f8f9fa;
    border: 1px solid #e9ecef;
    border-radius: 4px;
    padding: 1rem;
    overflow-x: auto;
    font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, monospace;
    font-size: 0.9rem;
    line-height: 1.4;
}

.highlight .keyword {
    color: #d73a49;
    font-weight: bold;
}

.highlight .string {
    color: #032f62;
}

.highlight .comment {
    color: #6a737d;
    font-style: italic;
}

.highlight .number {
    color: #005cc5;
}

.highlight .function {
    color: #6f42c1;
}

.highlight .variable {
    color: #e36209;
}

.highlight .constant {
    color: #005cc5;
    font-weight: bold;
}

.highlight .operator {
    color: #d73a49;
}

.highlight .punctuation {
    color: #24292e;
}

/* Dark theme syntax highlighting */
@media (prefers-color-scheme: dark) {
    .highlight {
        background-color: #2d3748;
        border-color: #4a5568;
        color: #e2e8f0;
    }
    
    .highlight .keyword {
        color: #f56565;
    }
    
    .highlight .string {
        color: #68d391;
    }
    
    .highlight .comment {
        color: #a0aec0;
    }
    
    .highlight .number {
        color: #63b3ed;
    }
    
    .highlight .function {
        color: #d6bcfa;
    }
    
    .highlight .variable {
        color: #f6ad55;
    }
    
    .highlight .constant {
        color: #63b3ed;
    }
}
"#;

        let css_file = assets_dir.join("syntax.css");
        let mut file = File::create(&css_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create syntax CSS file: {}", e)))?;

        file.write_all(syntax_css.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write syntax CSS file: {}", e)))?;

        Ok(())
    }

    /// Copy custom CSS and JS assets
    fn copy_custom_assets(&self, assets_dir: &Path) -> Result<(), CursedError> {
        // Copy custom CSS files
        for css_file in &self.config.html.custom_css {
            if Path::new(css_file).exists() {
                let dest = assets_dir.join(Path::new(css_file).file_name().unwrap());
                fs::copy(css_file, dest)
                    .map_err(|e| CursedError::IoError(format!("Failed to copy custom CSS: {}", e)))?;
            }
        }

        // Copy custom JS files
        for js_file in &self.config.html.custom_js {
            if Path::new(js_file).exists() {
                let dest = assets_dir.join(Path::new(js_file).file_name().unwrap());
                fs::copy(js_file, dest)
                    .map_err(|e| CursedError::IoError(format!("Failed to copy custom JS: {}", e)))?;
            }
        }

        Ok(())
    }

    /// Generate main index page
    fn generate_index(&self, html_dir: &Path, documentation: &Documentation) -> Result<(), CursedError> {
        let index_file = html_dir.join("index.html");
        let mut file = File::create(&index_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create index.html: {}", e)))?;

        let html_content = self.generate_index_html(documentation)?;
        file.write_all(html_content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write index.html: {}", e)))?;

        Ok(())
    }

    /// Generate HTML content for index page
    fn generate_index_html(&self, documentation: &Documentation) -> Result<String, CursedError> {
        let mut html = String::new();

        html.push_str(&format!(r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} Documentation</title>
    <link rel="stylesheet" href="assets/style.css">
    <link rel="stylesheet" href="assets/syntax.css">
    <script src="assets/script.js"></script>
    <meta name="description" content="{}">
    <meta name="keywords" content="CURSED, programming language, documentation">
</head>
<body>
    <div class="progress-bar"></div>
    <header class="header">
        <div class="container">
            <h1 class="logo">{}</h1>
            <nav class="nav">
                <a href="#modules">Modules</a>
                <a href="#examples">Examples</a>
                <a href="api.html">API Reference</a>
                <button class="theme-toggle" title="Toggle dark mode">🌙</button>
            </nav>
        </div>
    </header>

    <main class="main">
        <div class="container">
            <section class="hero">
                <h2>{}</h2>
                <p>{}</p>
                {}
                <div class="stats">
                    <div class="stat">
                        <span class="stat-number">{}</span>
                        <span class="stat-label">Modules</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">{}</span>
                        <span class="stat-label">Functions</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">{:.1}%</span>
                        <span class="stat-label">Documented</span>
                    </div>
                </div>
            </section>

            <section id="modules" class="modules">
                <h2>Modules</h2>
                <div class="module-grid">
"##, 
            documentation.project_info.project_name,
            documentation.project_info.project_description,
            documentation.project_info.project_name,
            documentation.project_info.project_name,
            documentation.project_info.project_description,
            if self.config.html.search_enabled {
                r#"<div class="search-container">
                    <input type="text" class="search-input" placeholder="Search documentation... (Ctrl+K)">
                    <div class="search-results"></div>
                </div>"#
            } else {
                ""
            },
            documentation.modules.len(),
            documentation.coverage_stats.total_functions,
            documentation.coverage_stats.coverage_percentage
        ));

        // Add module cards
        for module in &documentation.modules {
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

            <section id="examples" class="examples">
                <h2>Examples</h2>
                <div class="example-grid">
"#);

        // Add example cards
        for example in &documentation.examples {
            html.push_str(&format!(r#"
                    <div class="example-card">
                        <h3>{}</h3>
                        <p>{}</p>
                        <div class="example-category">{}</div>
                    </div>
"#, 
                example.title, 
                example.description,
                example.category
            ));
        }

        html.push_str(r#"
                </div>
            </section>
        </div>
    </main>

    <footer class="footer">
        <div class="container">
            <p>&copy; 2024 {} Documentation. Generated by CURSED Doc Generator.</p>
        </div>
    </footer>
</body>
</html>"#);

        Ok(html)
    }

    /// Generate module page
    fn generate_module_page(&self, html_dir: &Path, module: &DocumentedModule, documentation: &Documentation) -> Result<(), CursedError> {
        let modules_dir = html_dir.join("modules");
        fs::create_dir_all(&modules_dir)
            .map_err(|e| CursedError::IoError(format!("Failed to create modules directory: {}", e)))?;

        let module_file = modules_dir.join(format!("{}.html", module.name));
        let mut file = File::create(&module_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create module file: {}", e)))?;

        let html_content = self.generate_module_html(module, documentation)?;
        file.write_all(html_content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write module file: {}", e)))?;

        Ok(())
    }

    /// Generate HTML content for module page
    fn generate_module_html(&self, module: &DocumentedModule, documentation: &Documentation) -> Result<String, CursedError> {
        let mut html = String::new();

        html.push_str(&format!(r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} Module - {} Documentation</title>
    <link rel="stylesheet" href="../assets/style.css">
    <script src="../assets/script.js"></script>
</head>
<body>
    <header class="header">
        <div class="container">
            <h1 class="logo"><a href="../index.html">{}</a></h1>
            <nav class="nav">
                <a href="../index.html#modules">Modules</a>
                <a href="../index.html#examples">Examples</a>
                <a href="../index.html#api">API Reference</a>
            </nav>
        </div>
    </header>

    <main class="main">
        <div class="container">
            <div class="module-header">
                <h1>{} Module</h1>
                <p>{}</p>
                <div class="module-info">
                    <span class="source-file">Source: {}</span>
                </div>
            </div>

            <div class="module-content">
                <nav class="module-nav">
                    <ul>
                        <li><a href="#functions">Functions</a></li>
                        <li><a href="#variables">Variables</a></li>
                        <li><a href="#constants">Constants</a></li>
                        <li><a href="#types">Types</a></li>
                    </ul>
                </nav>

                <section id="functions" class="functions">
                    <h2>Functions</h2>
"##, 
            module.name,
            documentation.project_info.project_name,
            documentation.project_info.project_name,
            module.name,
            module.description,
            module.source_file
        ));

        // Add function documentation
        for function in &module.functions {
            html.push_str(&self.generate_function_html(function));
        }

        html.push_str(r#"
                </section>

                <section id="variables" class="variables">
                    <h2>Variables</h2>
"#);

        // Add variable documentation
        for variable in &module.variables {
            html.push_str(&format!(r#"
                    <div class="variable">
                        <h3>{}</h3>
                        <div class="variable-type">{}</div>
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
        </div>
    </main>

    <footer class="footer">
        <div class="container">
            <p>&copy; 2024 {} Documentation. Generated by CURSED Doc Generator.</p>
        </div>
    </footer>
</body>
</html>"#);

        Ok(html)
    }

    /// Generate HTML for a function
    fn generate_function_html(&self, function: &DocumentedFunction) -> String {
        let mut html = String::new();

        html.push_str(&format!(r#"
                    <div class="function" id="{}">
                        <div class="function-header">
                            <h3>{}</h3>
                            <div class="function-signature">
                                <code>{}</code>
                            </div>
                        </div>
                        <div class="function-body">
                            <p>{}</p>
"#, 
            function.name, 
            function.name, 
            function.signature, 
            function.description
        ));

        // Add parameters
        if !function.parameters.is_empty() {
            html.push_str(r#"
                            <h4>Parameters</h4>
                            <ul class="parameters">
"#);
            
            for param in &function.parameters {
                html.push_str(&format!(r#"
                                <li>
                                    <strong>{}</strong> (<code>{}</code>): {}
                                </li>
"#, 
                    param.name, 
                    param.param_type, 
                    param.description
                ));
            }
            
            html.push_str(r#"
                            </ul>
"#);
        }

        // Add return information
        if !function.return_type.is_empty() {
            html.push_str(&format!(r#"
                            <h4>Returns</h4>
                            <p><code>{}</code> - {}</p>
"#, 
                function.return_type, 
                function.return_description
            ));
        }

        // Add examples
        if !function.examples.is_empty() {
            html.push_str(r#"
                            <h4>Examples</h4>
"#);
            
            for example in &function.examples {
                html.push_str(&format!(r#"
                            <pre class="example"><code>{}</code></pre>
"#, 
                    html_escape(example)
                ));
            }
        }

        html.push_str(r#"
                        </div>
                    </div>
"#);

        html
    }

    /// Generate search index
    fn generate_search_index(&self, html_dir: &Path, documentation: &Documentation) -> Result<(), CursedError> {
        let search_file = html_dir.join("search.json");
        let mut file = File::create(&search_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create search.json: {}", e)))?;

        let search_data = self.generate_search_data(documentation)?;
        file.write_all(search_data.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write search.json: {}", e)))?;

        Ok(())
    }

    /// Generate search data
    fn generate_search_data(&self, documentation: &Documentation) -> Result<String, CursedError> {
        let mut search_items = Vec::new();

        // Add modules to search
        for module in &documentation.modules {
            search_items.push(format!(r#"{{
                "title": "{} Module",
                "url": "modules/{}.html",
                "content": "{}",
                "type": "module"
            }}"#, 
                module.name, 
                module.name, 
                module.description
            ));

            // Add functions to search
            for function in &module.functions {
                search_items.push(format!(r#"{{
                    "title": "{}",
                    "url": "modules/{}.html#{}",
                    "content": "{}",
                    "type": "function"
                }}"#, 
                    function.name, 
                    module.name, 
                    function.name, 
                    function.description
                ));
            }
        }

        Ok(format!("[\n{}\n]", search_items.join(",\n")))
    }

    /// Generate API reference
    fn generate_api_reference(&self, html_dir: &Path, documentation: &Documentation) -> Result<(), CursedError> {
        let api_file = html_dir.join("api.html");
        let mut file = File::create(&api_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create api.html: {}", e)))?;

        let html_content = self.generate_api_html(documentation)?;
        file.write_all(html_content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write api.html: {}", e)))?;

        Ok(())
    }

    /// Generate API reference HTML
    fn generate_api_html(&self, documentation: &Documentation) -> Result<String, CursedError> {
        // Implementation for API reference page
        Ok(String::new())
    }

    /// Generate examples index
    fn generate_examples_index(&self, html_dir: &Path, documentation: &Documentation) -> Result<(), CursedError> {
        let examples_file = html_dir.join("examples.html");
        let mut file = File::create(&examples_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create examples.html: {}", e)))?;

        let html_content = self.generate_examples_html(documentation)?;
        file.write_all(html_content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write examples.html: {}", e)))?;

        Ok(())
    }

    /// Generate examples HTML
    fn generate_examples_html(&self, documentation: &Documentation) -> Result<String, CursedError> {
        // Implementation for examples page
        Ok(String::new())
    }
}

/// Escape HTML special characters
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
