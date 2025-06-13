//! HTML Documentation Generator
//! 
//! Generates HTML documentation with navigation, search, and cross-references.

use super::*;
use std::fs;
use std::path::Path;

/// HTML documentation generator
pub struct HtmlGenerator<'a> {
    config: &'a DocGeneratorConfig,
}

impl<'a> HtmlGenerator<'a> {
    pub fn new(config: &'a DocGeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate main index page
    pub fn generate_index_page(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> Result<(), Error> {
        let html = self.build_index_html(docs)?;
        let index_path = output_dir.join("index.html");
        fs::write(index_path, html).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate documentation page for a module
    pub fn generate_module_page(&self, doc: &ExtractedDocumentation, output_dir: &Path) -> Result<(), Error> {
        let html = self.build_module_html(doc)?;
        let module_path = output_dir.join(format!("{}.html", doc.module_name));
        fs::write(module_path, html).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate search index JavaScript file
    pub fn generate_search_index(&self, search_index: &[SearchIndexEntry], output_dir: &Path) -> Result<(), Error> {
        let js_content = format!(
            "const SEARCH_INDEX = {};",
            serde_json::to_string_pretty(search_index).map_err(|e| Error::Parse(e.to_string()))?
        );
        
        let search_path = output_dir.join("search-index.js");
        fs::write(search_path, js_content).map_err(Error::Io)?;
        Ok(())
    }

    /// Copy static assets (CSS, JS, images)
    pub fn copy_static_assets(&self, output_dir: &Path) -> Result<(), Error> {
        // Create static directory
        let static_dir = output_dir.join("static");
        fs::create_dir_all(&static_dir).map_err(Error::Io)?;

        // Generate CSS
        let css_content = self.generate_default_css();
        fs::write(static_dir.join("docs.css"), css_content).map_err(Error::Io)?;

        // Generate JavaScript
        let js_content = self.generate_default_js();
        fs::write(static_dir.join("docs.js"), js_content).map_err(Error::Io)?;

        Ok(())
    }

    /// Build HTML for index page
    fn build_index_html(&self, docs: &[ExtractedDocumentation]) -> Result<String, Error> {
        let mut html = String::new();
        
        // HTML header
        html.push_str(&self.build_html_header("Index")?);
        
        // Main content
        html.push_str("<div class=\"container\">\n");
        html.push_str("<header class=\"header\">\n");
        html.push_str(&format!("<h1 class=\"title\">{}</h1>\n", self.config.title));
        
        if let Some(ref description) = self.config.description {
            html.push_str(&format!("<p class=\"description\">{}</p>\n", description));
        }
        
        if let Some(ref version) = self.config.version {
            html.push_str(&format!("<span class=\"version\">Version {}</span>\n", version));
        }
        
        html.push_str("</header>\n");
        
        // Search bar
        html.push_str(r#"
<div class="search-container">
    <input type="text" id="search-input" placeholder="Search documentation..." />
    <div id="search-results" class="search-results"></div>
</div>
"#);

        // Module list
        html.push_str("<nav class=\"module-nav\">\n");
        html.push_str("<h2>Modules</h2>\n");
        html.push_str("<ul class=\"module-list\">\n");
        
        for doc in docs {
            let item_count = doc.items.len();
            html.push_str(&format!(
                "<li><a href=\"{}.html\" class=\"module-link\">{}</a> <span class=\"item-count\">({} items)</span></li>\n",
                doc.module_name, doc.module_name, item_count
            ));
        }
        
        html.push_str("</ul>\n");
        html.push_str("</nav>\n");

        // Quick stats
        let total_items: usize = docs.iter().map(|d| d.items.len()).sum();
        let total_modules = docs.len();
        
        html.push_str("<div class=\"stats\">\n");
        html.push_str("<h3>Documentation Statistics</h3>\n");
        html.push_str(&format!("<p>📦 {} modules</p>\n", total_modules));
        html.push_str(&format!("<p>📄 {} documented items</p>\n", total_items));
        html.push_str("</div>\n");

        html.push_str("</div>\n");
        html.push_str(&self.build_html_footer()?);
        
        Ok(html)
    }

    /// Build HTML for module page
    fn build_module_html(&self, doc: &ExtractedDocumentation) -> Result<String, Error> {
        let mut html = String::new();
        
        // HTML header
        html.push_str(&self.build_html_header(&doc.module_name)?);
        
        html.push_str("<div class=\"container\">\n");
        
        // Module header
        html.push_str("<header class=\"module-header\">\n");
        html.push_str(&format!("<h1 class=\"module-title\">{}</h1>\n", doc.module_name));
        
        if let Some(ref package) = doc.package_name {
            html.push_str(&format!("<p class=\"package-name\">Package: {}</p>\n", package));
        }
        
        html.push_str(&format!("<p class=\"file-path\">{}</p>\n", doc.file_path.display()));
        html.push_str("</header>\n");

        // Navigation
        html.push_str("<nav class=\"item-nav\">\n");
        html.push_str("<h2>Contents</h2>\n");
        html.push_str("<ul class=\"item-list\">\n");
        
        for item in &doc.items {
            html.push_str(&format!(
                "<li><a href=\"#{}\" class=\"{}-link\">{}</a></li>\n",
                item.name.to_lowercase(),
                item.kind.to_string(),
                item.name
            ));
        }
        
        html.push_str("</ul>\n");
        html.push_str("</nav>\n");

        // Module imports
        if !doc.imports.is_empty() {
            html.push_str("<section class=\"imports\">\n");
            html.push_str("<h2>Imports</h2>\n");
            html.push_str("<ul class=\"import-list\">\n");
            
            for import in &doc.imports {
                html.push_str(&format!("<li><code>{}</code></li>\n", import));
            }
            
            html.push_str("</ul>\n");
            html.push_str("</section>\n");
        }

        // Documentation items
        for item in &doc.items {
            html.push_str(&self.build_item_html(item)?);
        }

        html.push_str("</div>\n");
        html.push_str(&self.build_html_footer()?);
        
        Ok(html)
    }

    /// Build HTML for a documentation item
    fn build_item_html(&self, item: &DocumentationItem) -> Result<String, Error> {
        let mut html = String::new();
        
        html.push_str(&format!("<section id=\"{}\" class=\"doc-item {}\">\n", 
            item.name.to_lowercase(), item.kind.to_string()));
        
        // Item header
        html.push_str("<header class=\"item-header\">\n");
        html.push_str(&format!("<h3 class=\"item-name\">{}</h3>\n", item.name));
        html.push_str(&format!("<span class=\"item-kind\">{}</span>\n", item.kind));
        
        if matches!(item.visibility, Visibility::Public) {
            html.push_str("<span class=\"visibility public\">public</span>\n");
        }
        
        html.push_str("</header>\n");

        // Signature
        if let Some(ref signature) = item.signature {
            html.push_str("<div class=\"signature\">\n");
            html.push_str(&format!("<code class=\"cursed\">{}</code>\n", self.escape_html(signature)));
            html.push_str("</div>\n");
        }

        // Summary
        if !item.summary.is_empty() {
            html.push_str(&format!("<p class=\"summary\">{}</p>\n", self.escape_html(&item.summary)));
        }

        // Description
        if !item.description.is_empty() {
            html.push_str("<div class=\"description\">\n");
            html.push_str(&self.markdown_to_html(&item.description)?);
            html.push_str("</div>\n");
        }

        // Parameters
        if !item.parameters.is_empty() {
            html.push_str("<div class=\"parameters\">\n");
            html.push_str("<h4>Parameters</h4>\n");
            html.push_str("<table class=\"param-table\">\n");
            html.push_str("<thead><tr><th>Name</th><th>Type</th><th>Description</th></tr></thead>\n");
            html.push_str("<tbody>\n");
            
            for param in &item.parameters {
                html.push_str("<tr>\n");
                html.push_str(&format!("<td><code>{}</code></td>\n", param.name));
                html.push_str(&format!("<td><code>{}</code></td>\n", 
                    param.type_name.as_deref().unwrap_or("any")));
                html.push_str(&format!("<td>{}</td>\n", self.escape_html(&param.description)));
                html.push_str("</tr>\n");
            }
            
            html.push_str("</tbody>\n");
            html.push_str("</table>\n");
            html.push_str("</div>\n");
        }

        // Return type
        if let Some(ref return_type) = item.return_type {
            html.push_str("<div class=\"return-type\">\n");
            html.push_str(&format!("<h4>Returns</h4>\n"));
            html.push_str(&format!("<code>{}</code>\n", self.escape_html(return_type)));
            html.push_str("</div>\n");
        }

        // Examples
        if !item.examples.is_empty() {
            html.push_str("<div class=\"examples\">\n");
            html.push_str("<h4>Examples</h4>\n");
            
            for example in &item.examples {
                if let Some(ref title) = example.title {
                    html.push_str(&format!("<h5>{}</h5>\n", self.escape_html(title)));
                }
                
                if let Some(ref description) = example.description {
                    html.push_str(&format!("<p>{}</p>\n", self.escape_html(description)));
                }
                
                html.push_str("<div class=\"example-code\">\n");
                html.push_str(&format!("<pre><code class=\"{}\">{}</code></pre>\n", 
                    example.language, self.escape_html(&example.code)));
                html.push_str("</div>\n");
                
                if let Some(ref output) = example.output {
                    html.push_str("<div class=\"example-output\">\n");
                    html.push_str("<h6>Output:</h6>\n");
                    html.push_str(&format!("<pre>{}</pre>\n", self.escape_html(output)));
                    html.push_str("</div>\n");
                }
            }
            
            html.push_str("</div>\n");
        }

        // Source location
        html.push_str("<div class=\"source-location\">\n");
        html.push_str(&format!("<small>Defined at line {} column {}</small>\n", 
            item.location.line, item.location.column));
        html.push_str("</div>\n");

        html.push_str("</section>\n");
        Ok(html)
    }

    /// Build HTML document header
    fn build_html_header(&self, title: &str) -> Result<String, Error> {
        Ok(format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - {}</title>
    <link rel="stylesheet" href="static/docs.css">
    <script src="static/docs.js" defer></script>
    <script src="search-index.js" defer></script>
</head>
<body>
    <nav class="top-nav">
        <a href="index.html" class="home-link">🏠 Home</a>
        <span class="nav-title">{}</span>
    </nav>
"#, title, self.config.title, self.config.title))
    }

    /// Build HTML document footer
    fn build_html_footer(&self) -> Result<String, Error> {
        let mut footer = String::new();
        
        footer.push_str("<footer class=\"footer\">\n");
        footer.push_str(&format!("<p>Generated by CURSED Documentation Generator</p>\n"));
        
        if !self.config.authors.is_empty() {
            footer.push_str(&format!("<p>Authors: {}</p>\n", self.config.authors.join(", ")));
        }
        
        footer.push_str(&format!("<p><small>Generated on {}</small></p>\n", 
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        footer.push_str("</footer>\n");
        footer.push_str("</body>\n");
        footer.push_str("</html>\n");
        
        Ok(footer)
    }

    /// Convert markdown to HTML (simplified)
    fn markdown_to_html(&self, markdown: &str) -> Result<String, Error> {
        // Simple markdown conversion
        let mut html = String::new();
        let lines: Vec<&str> = markdown.lines().collect();
        let mut in_code_block = false;
        
        for line in lines {
            if line.starts_with("```") {
                if in_code_block {
                    html.push_str("</pre></code>\n");
                    in_code_block = false;
                } else {
                    html.push_str("<code><pre>\n");
                    in_code_block = true;
                }
            } else if in_code_block {
                html.push_str(&self.escape_html(line));
                html.push('\n');
            } else {
                // Simple paragraph conversion
                if line.trim().is_empty() {
                    html.push_str("</p>\n<p>\n");
                } else {
                    html.push_str(&self.escape_html(line));
                    html.push(' ');
                }
            }
        }
        
        if !html.starts_with("<p>") {
            html = format!("<p>{}</p>", html);
        }
        
        Ok(html)
    }

    /// Escape HTML entities
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }

    /// Generate default CSS
    fn generate_default_css(&self) -> String {
        r#"
/* CURSED Documentation CSS */
:root {
    --primary-color: #6366f1;
    --secondary-color: #8b5cf6;
    --background-color: #ffffff;
    --surface-color: #f8fafc;
    --text-color: #1e293b;
    --text-secondary: #64748b;
    --border-color: #e2e8f0;
    --accent-color: #10b981;
    --warning-color: #f59e0b;
    --error-color: #ef4444;
}

* {
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.6;
    color: var(--text-color);
    background-color: var(--background-color);
    margin: 0;
    padding: 0;
}

.top-nav {
    background: var(--primary-color);
    color: white;
    padding: 1rem 2rem;
    display: flex;
    align-items: center;
    gap: 1rem;
}

.home-link {
    color: white;
    text-decoration: none;
    font-weight: 500;
}

.home-link:hover {
    text-decoration: underline;
}

.nav-title {
    font-weight: 600;
}

.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

.header {
    text-align: center;
    margin-bottom: 3rem;
}

.title {
    font-size: 3rem;
    font-weight: 700;
    margin-bottom: 1rem;
    background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
}

.description {
    font-size: 1.25rem;
    color: var(--text-secondary);
    margin-bottom: 1rem;
}

.version {
    background: var(--accent-color);
    color: white;
    padding: 0.25rem 0.75rem;
    border-radius: 1rem;
    font-size: 0.875rem;
    font-weight: 500;
}

.search-container {
    margin-bottom: 2rem;
    position: relative;
}

#search-input {
    width: 100%;
    padding: 1rem;
    border: 2px solid var(--border-color);
    border-radius: 0.5rem;
    font-size: 1rem;
    outline: none;
    transition: border-color 0.2s;
}

#search-input:focus {
    border-color: var(--primary-color);
}

.search-results {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: white;
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
    max-height: 300px;
    overflow-y: auto;
    z-index: 100;
    display: none;
}

.search-result-item {
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--border-color);
    cursor: pointer;
}

.search-result-item:hover {
    background: var(--surface-color);
}

.search-result-item:last-child {
    border-bottom: none;
}

.module-nav {
    background: var(--surface-color);
    padding: 2rem;
    border-radius: 0.5rem;
    margin-bottom: 2rem;
}

.module-list {
    list-style: none;
    padding: 0;
    margin: 0;
}

.module-list li {
    padding: 0.5rem 0;
    border-bottom: 1px solid var(--border-color);
}

.module-list li:last-child {
    border-bottom: none;
}

.module-link {
    color: var(--primary-color);
    text-decoration: none;
    font-weight: 500;
}

.module-link:hover {
    text-decoration: underline;
}

.item-count {
    color: var(--text-secondary);
    font-size: 0.875rem;
}

.stats {
    background: var(--surface-color);
    padding: 2rem;
    border-radius: 0.5rem;
    text-align: center;
}

.stats h3 {
    margin-top: 0;
}

.module-header {
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 2px solid var(--border-color);
}

.module-title {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
}

.package-name {
    color: var(--text-secondary);
    margin-bottom: 0.5rem;
}

.file-path {
    font-family: 'Monaco', 'Menlo', monospace;
    font-size: 0.875rem;
    color: var(--text-secondary);
    background: var(--surface-color);
    padding: 0.5rem;
    border-radius: 0.25rem;
}

.item-nav {
    background: var(--surface-color);
    padding: 1.5rem;
    border-radius: 0.5rem;
    margin-bottom: 2rem;
}

.item-list {
    list-style: none;
    padding: 0;
    margin: 0;
}

.item-list li {
    padding: 0.25rem 0;
}

.doc-item {
    margin-bottom: 3rem;
    padding: 2rem;
    background: white;
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
}

.item-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1rem;
}

.item-name {
    margin: 0;
    font-size: 1.5rem;
}

.item-kind {
    background: var(--primary-color);
    color: white;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    text-transform: uppercase;
    font-weight: 600;
}

.visibility.public {
    background: var(--accent-color);
    color: white;
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    text-transform: uppercase;
    font-weight: 600;
}

.signature {
    background: var(--surface-color);
    padding: 1rem;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    overflow-x: auto;
}

.signature code {
    font-family: 'Monaco', 'Menlo', monospace;
    font-size: 0.875rem;
}

.summary {
    font-size: 1.125rem;
    font-weight: 500;
    margin-bottom: 1rem;
}

.description {
    margin-bottom: 1.5rem;
}

.param-table {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 1rem;
}

.param-table th,
.param-table td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid var(--border-color);
}

.param-table th {
    background: var(--surface-color);
    font-weight: 600;
}

.example-code {
    margin: 1rem 0;
}

.example-code pre {
    background: var(--surface-color);
    padding: 1rem;
    border-radius: 0.5rem;
    overflow-x: auto;
    border: 1px solid var(--border-color);
}

.example-output {
    margin: 1rem 0;
}

.example-output pre {
    background: #f1f5f9;
    padding: 1rem;
    border-radius: 0.5rem;
    border-left: 4px solid var(--accent-color);
}

.source-location {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
}

.footer {
    text-align: center;
    padding: 2rem;
    margin-top: 3rem;
    border-top: 1px solid var(--border-color);
    color: var(--text-secondary);
}

/* Responsive design */
@media (max-width: 768px) {
    .container {
        padding: 1rem;
    }
    
    .title {
        font-size: 2rem;
    }
    
    .doc-item {
        padding: 1rem;
    }
    
    .item-header {
        flex-direction: column;
        align-items: flex-start;
    }
}

/* Code syntax highlighting for CURSED */
.cursed {
    color: var(--primary-color);
    font-weight: 500;
}

.keyword {
    color: var(--secondary-color);
    font-weight: 600;
}

.string {
    color: var(--accent-color);
}

.comment {
    color: var(--text-secondary);
    font-style: italic;
}
"#.to_string()
    }

    /// Generate default JavaScript
    fn generate_default_js(&self) -> String {
        r#"
// CURSED Documentation JavaScript

// Search functionality
document.addEventListener('DOMContentLoaded', function() {
    const searchInput = document.getElementById('search-input');
    const searchResults = document.getElementById('search-results');
    
    if (searchInput && searchResults && typeof SEARCH_INDEX !== 'undefined') {
        searchInput.addEventListener('input', function() {
            const query = this.value.toLowerCase().trim();
            
            if (query.length < 2) {
                searchResults.style.display = 'none';
                return;
            }
            
            const results = searchItems(query);
            displaySearchResults(results);
        });
        
        // Hide search results when clicking outside
        document.addEventListener('click', function(e) {
            if (!searchInput.contains(e.target) && !searchResults.contains(e.target)) {
                searchResults.style.display = 'none';
            }
        });
    }
});

function searchItems(query) {
    if (typeof SEARCH_INDEX === 'undefined') {
        return [];
    }
    
    return SEARCH_INDEX.filter(item => {
        return item.name.toLowerCase().includes(query) ||
               item.description.toLowerCase().includes(query) ||
               item.keywords.some(keyword => keyword.includes(query));
    }).slice(0, 10); // Limit to 10 results
}

function displaySearchResults(results) {
    const searchResults = document.getElementById('search-results');
    
    if (results.length === 0) {
        searchResults.style.display = 'none';
        return;
    }
    
    let html = '';
    results.forEach(result => {
        html += `
            <div class="search-result-item" onclick="navigateToItem('${result.url}')">
                <div style="font-weight: 500;">${result.name}</div>
                <div style="font-size: 0.875rem; color: var(--text-secondary);">
                    ${result.kind} in ${result.module}
                </div>
                <div style="font-size: 0.875rem; margin-top: 0.25rem;">
                    ${result.description}
                </div>
            </div>
        `;
    });
    
    searchResults.innerHTML = html;
    searchResults.style.display = 'block';
}

function navigateToItem(url) {
    window.location.href = url;
}

// Smooth scrolling for anchor links
document.addEventListener('DOMContentLoaded', function() {
    const links = document.querySelectorAll('a[href^="#"]');
    
    links.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            
            const targetId = this.getAttribute('href').substring(1);
            const targetElement = document.getElementById(targetId);
            
            if (targetElement) {
                targetElement.scrollIntoView({
                    behavior: 'smooth'
                });
            }
        });
    });
});

// Code copy functionality
document.addEventListener('DOMContentLoaded', function() {
    const codeBlocks = document.querySelectorAll('pre code');
    
    codeBlocks.forEach(block => {
        const wrapper = document.createElement('div');
        wrapper.style.position = 'relative';
        
        const copyButton = document.createElement('button');
        copyButton.textContent = 'Copy';
        copyButton.style.position = 'absolute';
        copyButton.style.top = '0.5rem';
        copyButton.style.right = '0.5rem';
        copyButton.style.padding = '0.25rem 0.5rem';
        copyButton.style.fontSize = '0.75rem';
        copyButton.style.border = '1px solid var(--border-color)';
        copyButton.style.borderRadius = '0.25rem';
        copyButton.style.background = 'white';
        copyButton.style.cursor = 'pointer';
        
        copyButton.addEventListener('click', function() {
            navigator.clipboard.writeText(block.textContent).then(() => {
                copyButton.textContent = 'Copied!';
                setTimeout(() => {
                    copyButton.textContent = 'Copy';
                }, 2000);
            });
        });
        
        block.parentNode.insertBefore(wrapper, block);
        wrapper.appendChild(block);
        wrapper.appendChild(copyButton);
    });
});

// Dark mode toggle (if implemented)
function toggleDarkMode() {
    document.body.classList.toggle('dark-mode');
    localStorage.setItem('darkMode', document.body.classList.contains('dark-mode'));
}

// Load dark mode preference
document.addEventListener('DOMContentLoaded', function() {
    if (localStorage.getItem('darkMode') === 'true') {
        document.body.classList.add('dark-mode');
    }
});
"#.to_string()
    }
}
