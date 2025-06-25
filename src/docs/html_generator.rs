// HTML Documentation Generator
// 
// Generates comprehensive HTML documentation with navigation, search, and responsive design.

use crate::docs::generator::{DocGeneratorConfig, ExtractedDocumentation, DocumentationItem, SearchIndexEntry};
use crate::error::CursedError;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// HTML documentation generator
pub struct HtmlGenerator {
    config: DocGeneratorConfig,
}

impl HtmlGenerator {
    pub fn new(config: &DocGeneratorConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Generate main index page
    pub fn generate_index_page(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> crate::error::Result<()> {
        let index_path = output_dir.join("index.html");
        
        let mut content = String::new();
        content.push_str(&self.get_html_header("CURSED Documentation", &self.config.title)?);
        
        // Main content
        content.push_str("<main class=\"container\">\n");
        content.push_str(&format!("<h1>{}</h1>\n", self.config.title));
        
        if let Some(description) = &self.config.description {
            content.push_str(&format!("<p class=\"lead\">{}</p>\n", description));
        }
        
        // Project information
        if let Some(version) = &self.config.version {
            content.push_str(&format!("<p><strong>Version:</strong> {}</p>\n", version));
        }
        
        if !self.config.authors.is_empty() {
            content.push_str(&format!("<p><strong>Authors:</strong> {}</p>\n", self.config.authors.join(", ")));
        }
        
        // Module index
        content.push_str("<h2>Modules</h2>\n");
        content.push_str("<div class=\"module-grid\">\n");
        
        for doc in docs {
            content.push_str("<div class=\"module-card\">\n");
            content.push_str(&format!("<h3><a href=\"{}.html\">{}</a></h3>\n", 
                self.sanitize_filename(&doc.module_name), doc.module_name));
            
            if let Some(package) = &doc.package_name {
                content.push_str(&format!("<p><strong>Package:</strong> {}</p>\n", package));
            }
            
            content.push_str(&format!("<p>{} items</p>\n", doc.items.len()));
            content.push_str("</div>\n");
        }
        
        content.push_str("</div>\n");
        
        // Quick statistics
        let total_items: usize = docs.iter().map(|d| d.items.len()).sum();
        let total_functions = docs.iter().map(|d| d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function)).count()).sum::<usize>();
        let total_structs = docs.iter().map(|d| d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Struct)).count()).sum::<usize>();
        
        content.push_str("<h2>Statistics</h2>\n");
        content.push_str("<div class=\"stats\">\n");
        content.push_str(&format!("<div class=\"stat-item\"><strong>{}</strong> modules</div>\n", docs.len()));
        content.push_str(&format!("<div class=\"stat-item\"><strong>{}</strong> total items</div>\n", total_items));
        content.push_str(&format!("<div class=\"stat-item\"><strong>{}</strong> functions</div>\n", total_functions));
        content.push_str(&format!("<div class=\"stat-item\"><strong>{}</strong> structs</div>\n", total_structs));
        content.push_str("</div>\n");
        
        content.push_str("</main>\n");
        content.push_str(&self.get_html_footer()?);
        
        fs::write(index_path, content).map_err(CursedError::Io)?;
        Ok(())
    }

    /// Generate module documentation page
    pub fn generate_module_page(&self, doc: &ExtractedDocumentation, output_dir: &Path) -> crate::error::Result<()> {
        let module_path = output_dir.join(format!("{}.html", self.sanitize_filename(&doc.module_name)));
        
        let mut content = String::new();
        content.push_str(&self.get_html_header(&format!("{} - Module", doc.module_name), &doc.module_name)?);
        
        // Module header
        content.push_str("<main class=\"container\">\n");
        content.push_str(&format!("<h1>Module: {}</h1>\n", doc.module_name));
        
        if let Some(package) = &doc.package_name {
            content.push_str(&format!("<p><strong>Package:</strong> {}</p>\n", package));
        }
        
        // Module information
        content.push_str("<div class=\"module-info\">\n");
        content.push_str(&format!("<p><strong>File:</strong> {}</p>\n", doc.file_path.display()));
        content.push_str(&format!("<p><strong>Lines:</strong> {}</p>\n", doc.source_info.line_count));
        content.push_str(&format!("<p><strong>Size:</strong> {} bytes</p>\n", doc.source_info.file_size));
        content.push_str("</div>\n");
        
        // Imports
        if !doc.imports.is_empty() {
            content.push_str("<h2>Imports</h2>\n");
            content.push_str("<ul class=\"imports-list\">\n");
            for import in &doc.imports {
                content.push_str(&format!("<li><code>{}</code></li>\n", import));
            }
            content.push_str("</ul>\n");
        }
        
        // Group items by type
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut interfaces = Vec::new();
        let mut variables = Vec::new();
        let mut constants = Vec::new();
        
        for item in &doc.items {
            match item.kind {
                crate::docs::generator::ItemKind::Function => functions.push(item),
                crate::docs::generator::ItemKind::Struct => structs.push(item),
                crate::docs::generator::ItemKind::Interface => interfaces.push(item),
                crate::docs::generator::ItemKind::Variable => variables.push(item),
                crate::docs::generator::ItemKind::Constant => constants.push(item),
                _ => {}
            }
        }
        
        // Generate sections for each type
        if !functions.is_empty() {
            content.push_str("<h2>Functions</h2>\n");
            for func in functions {
                content.push_str(&self.generate_item_documentation(func)?);
            }
        }
        
        if !structs.is_empty() {
            content.push_str("<h2>Structs</h2>\n");
            for struct_item in structs {
                content.push_str(&self.generate_item_documentation(struct_item)?);
            }
        }
        
        if !interfaces.is_empty() {
            content.push_str("<h2>Interfaces</h2>\n");
            for interface in interfaces {
                content.push_str(&self.generate_item_documentation(interface)?);
            }
        }
        
        if !constants.is_empty() {
            content.push_str("<h2>Constants</h2>\n");
            for constant in constants {
                content.push_str(&self.generate_item_documentation(constant)?);
            }
        }
        
        if !variables.is_empty() {
            content.push_str("<h2>Variables</h2>\n");
            for variable in variables {
                content.push_str(&self.generate_item_documentation(variable)?);
            }
        }
        
        content.push_str("</main>\n");
        content.push_str(&self.get_html_footer()?);
        
        fs::write(module_path, content).map_err(CursedError::Io)?;
        Ok(())
    }

    /// Generate documentation for a single item
    fn generate_item_documentation(&self, item: &DocumentationItem) -> crate::error::Result<()> {
        let mut content = String::new();
        
        content.push_str(&format!("<div class=\"item\" id=\"{}\">\n", item.name.to_lowercase()));
        content.push_str(&format!("<h3>{}</h3>\n", item.name));
        
        // Signature
        if let Some(signature) = &item.signature {
            content.push_str("<div class=\"signature\">\n");
            content.push_str(&format!("<pre><code>{}</code></pre>\n", signature));
            content.push_str("</div>\n");
        }
        
        // Summary and description
        content.push_str(&format!("<p class=\"summary\">{}</p>\n", item.summary));
        if !item.description.is_empty() && item.description != item.summary {
            content.push_str(&format!("<div class=\"description\">{}</div>\n", self.format_description(&item.description)));
        }
        
        // Parameters
        if !item.parameters.is_empty() {
            content.push_str("<h4>Parameters</h4>\n");
            content.push_str("<table class=\"parameters\">\n");
            content.push_str("<thead><tr><th>Name</th><th>Type</th><th>Description</th><th>Default</th></tr></thead>\n");
            content.push_str("<tbody>\n");
            
            for param in &item.parameters {
                content.push_str("<tr>\n");
                content.push_str(&format!("<td><code>{}</code></td>\n", param.name));
                content.push_str(&format!("<td><code>{}</code></td>\n", param.type_name.as_deref().unwrap_or("unknown")));
                content.push_str(&format!("<td>{}</td>\n", param.description));
                content.push_str(&format!("<td><code>{}</code></td>\n", param.default_value.as_deref().unwrap_or("None")));
                content.push_str("</tr>\n");
            }
            
            content.push_str("</tbody>\n");
            content.push_str("</table>\n");
        }
        
        // Return type
        if let Some(return_type) = &item.return_type {
            content.push_str("<h4>Returns</h4>\n");
            content.push_str(&format!("<p><code>{}</code></p>\n", return_type));
        }
        
        // Examples
        if !item.examples.is_empty() {
            content.push_str("<h4>Examples</h4>\n");
            for example in &item.examples {
                if let Some(title) = &example.title {
                    content.push_str(&format!("<h5>{}</h5>\n", title));
                }
                if let Some(description) = &example.description {
                    content.push_str(&format!("<p>{}</p>\n", description));
                }
                content.push_str("<div class=\"example\">\n");
                content.push_str(&format!("<pre><code class=\"language-{}\">{}</code></pre>\n", example.language, example.code));
                content.push_str("</div>\n");
                
                if let Some(output) = &example.output {
                    content.push_str("<div class=\"example-output\">\n");
                    content.push_str("<h6>Output:</h6>\n");
                    content.push_str(&format!("<pre><code>{}</code></pre>\n", output));
                    content.push_str("</div>\n");
                }
            }
        }
        
        // Source code
        if self.config.include_examples && item.source_code.is_some() {
            content.push_str("<details class=\"source-code\">\n");
            content.push_str("<summary>Source Code</summary>\n");
            content.push_str(&format!("<pre><code class=\"language-cursed\">{}</code></pre>\n", 
                item.source_code.as_ref().unwrap()));
            content.push_str("</details>\n");
        }
        
        content.push_str("</div>\n");
        Ok(content)
    }

    /// Generate search index
    pub fn generate_search_index(&self, search_index: &[SearchIndexEntry], output_dir: &Path) -> crate::error::Result<()> {
        let search_path = output_dir.join("search.js");
        
        let json_index = serde_json::to_string_pretty(search_index)
            .map_err(|e| CursedError::General(format!("Failed to serialize search index: {}", e)))?;
        
        let content = format!(
            "window.searchIndex = {};\n\n// Search functionality will be loaded by script.js",
            json_index
        );
        
        fs::write(search_path, content).map_err(CursedError::Io)?;
        Ok(())
    }

    /// Copy static assets (CSS, JS, images)
    pub fn copy_static_assets(&self, output_dir: &Path) -> crate::error::Result<()> {
        // Create CSS file
        let css_path = output_dir.join("styles.css");
        let css_content = if let Some(custom_css) = &self.config.custom_css {
            format!("{}\n\n{}", self.get_default_css(), custom_css)
        } else {
            self.get_default_css()
        };
        fs::write(css_path, css_content).map_err(CursedError::Io)?;
        
        // Create JavaScript file
        let js_path = output_dir.join("script.js");
        fs::write(js_path, self.get_default_js()).map_err(CursedError::Io)?;
        
        Ok(())
    }

    /// Get HTML header with navigation
    fn get_html_header(&self, title: &str, current_module: &str) -> crate::error::Result<()> {
        let header = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <link rel="stylesheet" href="styles.css">
    <script src="search.js" defer></script>
    <script src="script.js" defer></script>
</head>
<body>
    <header class="header">
        <nav class="navbar">
            <div class="navbar-brand">
                <a href="index.html">{}</a>
            </div>
            <div class="navbar-nav">
                <a href="index.html" class="nav-link">Home</a>
                <div class="search-container">
                    <input type="text" id="search-input" placeholder="Search documentation..." />
                    <div id="search-results" class="search-results"></div>
                </div>
            </div>
        </nav>
    </header>
"#, title, self.config.title);
        Ok(header)
    }

    /// Get HTML footer
    fn get_html_footer(&self) -> crate::error::Result<()> {
        let footer = format!(r#"    <footer class="footer">
        <div class="container">
            <p>&copy; {} {}. Generated with CURSED documentation system.</p>
        </div>
    </footer>
</body>
</html>"#, 
            chrono::Utc::now().format("%Y"),
            self.config.authors.first().unwrap_or(&"CURSED Project".to_string())
        );
        Ok(footer)
    }

    /// Format description with basic markdown support
    fn format_description(&self, description: &str) -> String {
        // Basic markdown formatting
        let mut formatted = description.to_string();
        
        // Bold
        formatted = formatted.replace("**", "<strong>").replace("**", "</strong>");
        
        // Italic
        formatted = formatted.replace("*", "<em>").replace("*", "</em>");
        
        // Code inline
        formatted = formatted.replace("`", "<code>").replace("`", "</code>");
        
        // Line breaks
        formatted = formatted.replace("\n", "<br>");
        
        formatted
    }

    /// Sanitize filename for web use
    fn sanitize_filename(&self, name: &str) -> String {
        name.replace("::", "_")
            .replace(" ", "_")
            .replace("/", "_")
            .to_lowercase()
    }

    /// Get default CSS content
    fn get_default_css(&self) -> String {
        // Read from template file if it exists, otherwise use embedded content
        let template_path = Path::new("src/docs/templates/styles.css");
        if template_path.exists() {
            fs::read_to_string(template_path).unwrap_or_else(|_| self.get_embedded_css())
        } else {
            self.get_embedded_css()
        }
    }

    /// Get default JavaScript content
    fn get_default_js(&self) -> String {
        // Read from template file if it exists, otherwise use embedded content
        let template_path = Path::new("src/docs/templates/script.js");
        if template_path.exists() {
            fs::read_to_string(template_path).unwrap_or_else(|_| self.get_embedded_js())
        } else {
            self.get_embedded_js()
        }
    }

    /// Get embedded CSS content
    fn get_embedded_css(&self) -> String {
        r#"/* CURSED Documentation Styles */
:root {
    --primary-color: #6366f1;
    --background-color: #ffffff;
    --text-color: #1e293b;
    --border-color: #e2e8f0;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    line-height: 1.6;
    color: var(--text-color);
    background-color: var(--background-color);
}

.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 2rem;
}

.header {
    background-color: var(--background-color);
    border-bottom: 1px solid var(--border-color);
    padding: 1rem 0;
}

.navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.navbar-brand a {
    font-size: 1.5rem;
    font-weight: bold;
    color: var(--primary-color);
    text-decoration: none;
}

h1, h2, h3 {
    color: var(--text-color);
    margin-bottom: 1rem;
}

code {
    background-color: #f1f5f9;
    padding: 0.125rem 0.25rem;
    border-radius: 0.25rem;
    font-family: monospace;
}

pre {
    background-color: #f1f5f9;
    padding: 1rem;
    border-radius: 0.5rem;
    overflow-x: auto;
}

.item {
    background-color: #f8fafc;
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    padding: 2rem;
    margin: 2rem 0;
}

.module-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
    margin: 2rem 0;
}

.module-card {
    background-color: #f8fafc;
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    padding: 1.5rem;
}

table {
    width: 100%;
    border-collapse: collapse;
    margin: 1rem 0;
}

th, td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid var(--border-color);
}

th {
    background-color: #f8fafc;
    font-weight: 600;
}

.footer {
    background-color: #f8fafc;
    border-top: 1px solid var(--border-color);
    padding: 2rem 0;
    margin-top: 2rem;
    text-align: center;
    color: #64748b;
}"#.to_string()
    }

    /// Get embedded JavaScript content
    fn get_embedded_js(&self) -> String {
        r#"// CURSED Documentation JavaScript
(function() {
    'use strict';
    
    // Simple search functionality
    let searchIndex = window.searchIndex || [];
    const searchInput = document.getElementById('search-input');
    const searchResults = document.getElementById('search-results');
    
    if (searchInput && searchResults) {
        searchInput.addEventListener('input', function(e) {
            const query = e.target.value.trim().toLowerCase();
            
            if (query.length < 2) {
                searchResults.style.display = 'none';
                return;
            }
            
            const results = searchIndex.filter(item => 
                item.name.toLowerCase().includes(query) ||
                item.description.toLowerCase().includes(query)
            ).slice(0, 10);
            
            if (results.length > 0) {
                searchResults.innerHTML = results.map(result => `
                    <div class="search-result">
                        <div class="search-result-title">${result.name}</div>
                        <div class="search-result-description">${result.description}</div>
                        <div class="search-result-module">${result.module}</div>
                    </div>
                `).join('');
                searchResults.style.display = 'block';
            } else {
                searchResults.innerHTML = '<div class="search-result">No results found</div>';
                searchResults.style.display = 'block';
            }
        });
        
        // Hide search results when clicking outside
        document.addEventListener('click', function(e) {
            if (!searchInput.contains(e.target) && !searchResults.contains(e.target)) {
                searchResults.style.display = 'none';
            }
        });
    }
    
    // Copy code blocks
    document.querySelectorAll('pre code').forEach(code => {
        const pre = code.parentElement;
        const button = document.createElement('button');
        button.textContent = 'Copy';
        button.style.cssText = 'position: absolute; top: 0.5rem; right: 0.5rem; padding: 0.25rem 0.5rem; font-size: 0.75rem; background: #6366f1; color: white; border: none; border-radius: 0.25rem; cursor: pointer;';
        
        pre.style.position = 'relative';
        pre.appendChild(button);
        
        button.addEventListener('click', () => {
            navigator.clipboard.writeText(code.textContent).then(() => {
                button.textContent = 'Copied!';
                setTimeout(() => button.textContent = 'Copy', 2000);
            });
        });
    });
})();"#.to_string()
    }
}
