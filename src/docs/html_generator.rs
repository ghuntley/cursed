//! HTML Documentation Generator
//! 
//! Generates HTML documentation with modern styling, navigation, and search functionality.

use super::{DocGeneratorConfig, ExtractedDocumentation, DocumentationItem, SearchIndexEntry, Example};
use crate::error::Error;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde_json;
use handlebars::{Handlebars, no_escape};

/// HTML documentation generator
pub struct HtmlGenerator<'a> {
    config: &'a DocGeneratorConfig,
    handlebars: Handlebars<'static>,
}

impl<'a> HtmlGenerator<'a> {
    /// Create a new HTML generator
    pub fn new(config: &'a DocGeneratorConfig) -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_escape_fn(no_escape);
        
        // Register built-in templates
        Self::register_templates(&mut handlebars);
        
        Self {
            config,
            handlebars,
        }
    }

    /// Generate the main index page
    pub fn generate_index_page(
        &self,
        docs: &[ExtractedDocumentation],
        output_dir: &Path,
    ) -> Result<(), Error> {
        let mut data = serde_json::Map::new();
        data.insert("title".to_string(), serde_json::Value::String(self.config.title.clone()));
        data.insert("description".to_string(), 
            serde_json::Value::String(self.config.description.clone().unwrap_or_default()));
        data.insert("version".to_string(), 
            serde_json::Value::String(self.config.version.clone().unwrap_or_default()));
        data.insert("authors".to_string(), 
            serde_json::Value::Array(self.config.authors.iter().map(|a| serde_json::Value::String(a.clone())).collect()));

        // Build module list
        let modules: Vec<serde_json::Value> = docs.iter().map(|doc| {
            let mut module = serde_json::Map::new();
            module.insert("name".to_string(), serde_json::Value::String(doc.module_name.clone()));
            module.insert("file_path".to_string(), serde_json::Value::String(doc.file_path.display().to_string()));
            module.insert("item_count".to_string(), serde_json::Value::Number(doc.items.len().into()));
            
            // Categorize items by type
            let mut item_counts = serde_json::Map::new();
            for item in &doc.items {
                let key = format!("{}_count", item.kind.to_string());
                let current = item_counts.get(&key).and_then(|v| v.as_u64()).unwrap_or(0);
                item_counts.insert(key, serde_json::Value::Number((current + 1).into()));
            }
            module.insert("item_counts".to_string(), serde_json::Value::Object(item_counts));
            
            serde_json::Value::Object(module)
        }).collect();
        
        data.insert("modules".to_string(), serde_json::Value::Array(modules));
        data.insert("base_url".to_string(), 
            serde_json::Value::String(self.config.base_url.clone().unwrap_or_default()));

        let html = self.handlebars.render("index", &data)
            .map_err(|e| Error::GenerationError(format!("Failed to render index template: {}", e)))?;

        let output_file = output_dir.join("index.html");
        fs::write(&output_file, html).map_err(Error::Io)?;

        Ok(())
    }

    /// Generate a module documentation page
    pub fn generate_module_page(
        &self,
        doc: &ExtractedDocumentation,
        output_dir: &Path,
    ) -> Result<(), Error> {
        let mut data = serde_json::Map::new();
        data.insert("module_name".to_string(), serde_json::Value::String(doc.module_name.clone()));
        data.insert("file_path".to_string(), serde_json::Value::String(doc.file_path.display().to_string()));
        
        // Group items by type
        let mut grouped_items = HashMap::new();
        for item in &doc.items {
            let key = item.kind.to_string();
            grouped_items.entry(key).or_insert_with(Vec::new).push(item);
        }

        let mut item_groups = serde_json::Map::new();
        for (kind, items) in grouped_items {
            let items_json: Vec<serde_json::Value> = items.iter().map(|item| {
                self.item_to_json(item)
            }).collect();
            item_groups.insert(kind, serde_json::Value::Array(items_json));
        }
        
        data.insert("item_groups".to_string(), serde_json::Value::Object(item_groups));
        data.insert("imports".to_string(), 
            serde_json::Value::Array(doc.imports.iter().map(|i| serde_json::Value::String(i.clone())).collect()));

        let html = self.handlebars.render("module", &data)
            .map_err(|e| Error::GenerationError(format!("Failed to render module template: {}", e)))?;

        let output_file = output_dir.join(format!("{}.html", doc.module_name.replace("::", "_")));
        fs::write(&output_file, html).map_err(Error::Io)?;

        Ok(())
    }

    /// Generate search index JavaScript file
    pub fn generate_search_index(
        &self,
        search_index: &[SearchIndexEntry],
        output_dir: &Path,
    ) -> Result<(), Error> {
        let json_data = serde_json::to_string_pretty(search_index)
            .map_err(|e| Error::GenerationError(format!("Failed to serialize search index: {}", e)))?;

        let js_content = format!(
            "// Search index generated by CURSED documentation generator\nwindow.searchIndex = {};",
            json_data
        );

        let output_file = output_dir.join("search-index.js");
        fs::write(&output_file, js_content).map_err(Error::Io)?;

        Ok(())
    }

    /// Copy static assets (CSS, JS, etc.)
    pub fn copy_static_assets(&self, output_dir: &Path) -> Result<(), Error> {
        let assets_dir = output_dir.join("assets");
        fs::create_dir_all(&assets_dir).map_err(Error::Io)?;

        // Generate main CSS file
        let css_content = self.generate_main_css();
        fs::write(assets_dir.join("main.css"), css_content).map_err(Error::Io)?;

        // Generate main JavaScript file
        let js_content = self.generate_main_js();
        fs::write(assets_dir.join("main.js"), js_content).map_err(Error::Io)?;

        // Copy custom CSS files if specified
        for css_file in &self.config.custom_css.clone().unwrap_or_default() {
            if css_file.exists() {
                let filename = css_file.file_name().unwrap_or_default();
                let dest = assets_dir.join(filename);
                fs::copy(css_file, dest).map_err(Error::Io)?;
            }
        }

        Ok(())
    }

    /// Convert documentation item to JSON
    fn item_to_json(&self, item: &DocumentationItem) -> serde_json::Value {
        let mut json = serde_json::Map::new();
        json.insert("name".to_string(), serde_json::Value::String(item.name.clone()));
        json.insert("kind".to_string(), serde_json::Value::String(item.kind.to_string()));
        json.insert("summary".to_string(), serde_json::Value::String(item.summary.clone()));
        json.insert("description".to_string(), serde_json::Value::String(item.description.clone()));
        
        if let Some(signature) = &item.signature {
            json.insert("signature".to_string(), serde_json::Value::String(signature.clone()));
        }
        
        if let Some(return_type) = &item.return_type {
            json.insert("return_type".to_string(), serde_json::Value::String(return_type.clone()));
        }

        // Parameters
        let params: Vec<serde_json::Value> = item.parameters.iter().map(|p| {
            let mut param = serde_json::Map::new();
            param.insert("name".to_string(), serde_json::Value::String(p.name.clone()));
            param.insert("description".to_string(), serde_json::Value::String(p.description.clone()));
            if let Some(type_name) = &p.type_name {
                param.insert("type".to_string(), serde_json::Value::String(type_name.clone()));
            }
            if let Some(default) = &p.default_value {
                param.insert("default".to_string(), serde_json::Value::String(default.clone()));
            }
            serde_json::Value::Object(param)
        }).collect();
        json.insert("parameters".to_string(), serde_json::Value::Array(params));

        // Examples
        let examples: Vec<serde_json::Value> = item.examples.iter().map(|ex| {
            let mut example = serde_json::Map::new();
            if let Some(title) = &ex.title {
                example.insert("title".to_string(), serde_json::Value::String(title.clone()));
            }
            if let Some(description) = &ex.description {
                example.insert("description".to_string(), serde_json::Value::String(description.clone()));
            }
            example.insert("code".to_string(), serde_json::Value::String(ex.code.clone()));
            example.insert("language".to_string(), serde_json::Value::String(ex.language.clone()));
            if let Some(output) = &ex.output {
                example.insert("output".to_string(), serde_json::Value::String(output.clone()));
            }
            serde_json::Value::Object(example)
        }).collect();
        json.insert("examples".to_string(), serde_json::Value::Array(examples));

        if let Some(source_code) = &item.source_code {
            json.insert("source_code".to_string(), serde_json::Value::String(source_code.clone()));
        }

        serde_json::Value::Object(json)
    }

    /// Register built-in templates
    fn register_templates(handlebars: &mut Handlebars<'static>) {
        // Index page template
        let index_template = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <link rel="stylesheet" href="assets/main.css">
</head>
<body>
    <header class="header">
        <h1>{{title}}</h1>
        {{#if description}}<p class="description">{{description}}</p>{{/if}}
        {{#if version}}<span class="version">v{{version}}</span>{{/if}}
    </header>

    <nav class="navigation">
        <div class="search-container">
            <input type="text" id="search" placeholder="Search documentation...">
            <div id="search-results"></div>
        </div>
    </nav>

    <main class="main-content">
        <section class="modules-section">
            <h2>Modules</h2>
            <div class="modules-grid">
                {{#each modules}}
                <div class="module-card">
                    <h3><a href="{{name}}.html">{{name}}</a></h3>
                    <p class="module-path">{{file_path}}</p>
                    <div class="module-stats">
                        <span class="item-count">{{item_count}} items</span>
                        {{#if item_counts.function_count}}<span class="stat">{{item_counts.function_count}} functions</span>{{/if}}
                        {{#if item_counts.struct_count}}<span class="stat">{{item_counts.struct_count}} structs</span>{{/if}}
                        {{#if item_counts.interface_count}}<span class="stat">{{item_counts.interface_count}} interfaces</span>{{/if}}
                    </div>
                </div>
                {{/each}}
            </div>
        </section>

        {{#if authors}}
        <section class="authors-section">
            <h2>Authors</h2>
            <ul>
                {{#each authors}}<li>{{this}}</li>{{/each}}
            </ul>
        </section>
        {{/if}}
    </main>

    <script src="search-index.js"></script>
    <script src="assets/main.js"></script>
</body>
</html>
"#;

        // Module page template
        let module_template = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{module_name}} - CURSED Documentation</title>
    <link rel="stylesheet" href="assets/main.css">
</head>
<body>
    <header class="header">
        <h1><a href="index.html">CURSED Documentation</a></h1>
        <h2>Module: {{module_name}}</h2>
        <p class="file-path">{{file_path}}</p>
    </header>

    <nav class="navigation">
        <div class="breadcrumb">
            <a href="index.html">Home</a> > {{module_name}}
        </div>
        <div class="toc">
            {{#each item_groups}}
            <h3>{{@key}}</h3>
            <ul>
                {{#each this}}
                <li><a href="#{{name}}">{{name}}</a></li>
                {{/each}}
            </ul>
            {{/each}}
        </div>
    </nav>

    <main class="main-content">
        {{#if imports}}
        <section class="imports-section">
            <h2>Imports</h2>
            <ul>
                {{#each imports}}<li><code>{{this}}</code></li>{{/each}}
            </ul>
        </section>
        {{/if}}

        {{#each item_groups}}
        <section class="items-section">
            <h2>{{@key}}s</h2>
            {{#each this}}
            <div class="item" id="{{name}}">
                <h3>{{name}}</h3>
                {{#if signature}}<pre class="signature"><code>{{signature}}</code></pre>{{/if}}
                <p class="summary">{{summary}}</p>
                {{#if description}}<div class="description">{{description}}</div>{{/if}}
                
                {{#if parameters}}
                <h4>Parameters</h4>
                <ul class="parameters">
                    {{#each parameters}}
                    <li>
                        <strong>{{name}}</strong>
                        {{#if type}}<span class="type">: {{type}}</span>{{/if}}
                        {{#if default}}<span class="default"> = {{default}}</span>{{/if}}
                        - {{description}}
                    </li>
                    {{/each}}
                </ul>
                {{/if}}

                {{#if return_type}}
                <h4>Returns</h4>
                <p><code>{{return_type}}</code></p>
                {{/if}}

                {{#if examples}}
                <h4>Examples</h4>
                {{#each examples}}
                <div class="example">
                    {{#if title}}<h5>{{title}}</h5>{{/if}}
                    {{#if description}}<p>{{description}}</p>{{/if}}
                    <pre class="code"><code class="language-{{language}}">{{code}}</code></pre>
                    {{#if output}}<div class="output"><strong>Output:</strong><pre>{{output}}</pre></div>{{/if}}
                </div>
                {{/each}}
                {{/if}}

                {{#if source_code}}
                <details class="source-code">
                    <summary>Source Code</summary>
                    <pre class="code"><code class="language-cursed">{{source_code}}</code></pre>
                </details>
                {{/if}}
            </div>
            {{/each}}
        </section>
        {{/each}}
    </main>

    <script src="search-index.js"></script>
    <script src="assets/main.js"></script>
</body>
</html>
"#;

        handlebars.register_template_string("index", index_template).unwrap();
        handlebars.register_template_string("module", module_template).unwrap();
    }

    /// Generate main CSS file
    fn generate_main_css(&self) -> String {
        r#"
/* CURSED Documentation Styles */
:root {
    --primary-color: #6366f1;
    --secondary-color: #8b5cf6;
    --text-color: #374151;
    --bg-color: #ffffff;
    --border-color: #e5e7eb;
    --code-bg: #f9fafb;
    --hover-color: #f3f4f6;
}

[data-theme="dark"] {
    --text-color: #f9fafb;
    --bg-color: #111827;
    --border-color: #374151;
    --code-bg: #1f2937;
    --hover-color: #374151;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.6;
    color: var(--text-color);
    background-color: var(--bg-color);
}

.header {
    background: var(--primary-color);
    color: white;
    padding: 2rem;
    text-align: center;
}

.header h1 {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
}

.header h1 a {
    color: white;
    text-decoration: none;
}

.header h2 {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
    opacity: 0.9;
}

.description {
    font-size: 1.1rem;
    opacity: 0.9;
}

.version {
    background: rgba(255, 255, 255, 0.2);
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.9rem;
}

.navigation {
    background: var(--code-bg);
    border-bottom: 1px solid var(--border-color);
    padding: 1rem;
}

.search-container {
    position: relative;
    max-width: 500px;
}

#search {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    font-size: 1rem;
    background: var(--bg-color);
    color: var(--text-color);
}

.breadcrumb {
    margin-bottom: 1rem;
}

.breadcrumb a {
    color: var(--primary-color);
    text-decoration: none;
}

.toc {
    background: var(--bg-color);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    padding: 1rem;
    max-width: 300px;
}

.toc h3 {
    color: var(--primary-color);
    margin-bottom: 0.5rem;
}

.toc ul {
    list-style: none;
    margin-bottom: 1rem;
}

.toc a {
    color: var(--text-color);
    text-decoration: none;
    padding: 0.25rem 0;
    display: block;
}

.toc a:hover {
    color: var(--primary-color);
}

.main-content {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

.modules-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-top: 1rem;
}

.module-card {
    background: var(--bg-color);
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    padding: 1.5rem;
    transition: transform 0.2s, box-shadow 0.2s;
}

.module-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.module-card h3 {
    margin-bottom: 0.5rem;
}

.module-card h3 a {
    color: var(--primary-color);
    text-decoration: none;
}

.module-path {
    color: #6b7280;
    font-size: 0.9rem;
    margin-bottom: 1rem;
}

.module-stats {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
}

.stat, .item-count {
    background: var(--code-bg);
    padding: 0.25rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.8rem;
}

.items-section {
    margin-bottom: 3rem;
}

.items-section h2 {
    color: var(--primary-color);
    border-bottom: 2px solid var(--primary-color);
    padding-bottom: 0.5rem;
    margin-bottom: 1.5rem;
}

.item {
    background: var(--bg-color);
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
}

.item h3 {
    color: var(--secondary-color);
    margin-bottom: 1rem;
}

.signature {
    background: var(--code-bg);
    border: 1px solid var(--border-color);
    border-radius: 0.25rem;
    padding: 1rem;
    margin: 1rem 0;
    overflow-x: auto;
}

.summary {
    font-weight: 500;
    margin-bottom: 0.5rem;
}

.description {
    margin-bottom: 1rem;
}

.parameters {
    list-style: none;
    margin: 0.5rem 0;
}

.parameters li {
    padding: 0.5rem;
    border-left: 3px solid var(--primary-color);
    margin-bottom: 0.5rem;
    background: var(--hover-color);
}

.type {
    color: var(--secondary-color);
    font-family: 'Courier New', monospace;
}

.default {
    color: #059669;
    font-family: 'Courier New', monospace;
}

.example {
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    padding: 1rem;
    margin: 1rem 0;
}

.code {
    background: var(--code-bg);
    border: 1px solid var(--border-color);
    border-radius: 0.25rem;
    padding: 1rem;
    overflow-x: auto;
    font-family: 'Courier New', monospace;
    font-size: 0.9rem;
}

.output {
    margin-top: 0.5rem;
    padding: 0.5rem;
    background: #f0f9ff;
    border-left: 3px solid #0ea5e9;
}

.source-code {
    margin-top: 1rem;
}

.source-code summary {
    cursor: pointer;
    padding: 0.5rem;
    background: var(--hover-color);
    border-radius: 0.25rem;
}

@media (max-width: 768px) {
    .header {
        padding: 1rem;
    }
    
    .header h1 {
        font-size: 2rem;
    }
    
    .main-content {
        padding: 1rem;
    }
    
    .modules-grid {
        grid-template-columns: 1fr;
    }
}
"#.to_string()
    }

    /// Generate main JavaScript file
    fn generate_main_js(&self) -> String {
        r#"
// CURSED Documentation JavaScript
document.addEventListener('DOMContentLoaded', function() {
    initializeSearch();
    initializeTheme();
    initializeNavigation();
});

function initializeSearch() {
    const searchInput = document.getElementById('search');
    const searchResults = document.getElementById('search-results');
    
    if (!searchInput || !window.searchIndex) return;
    
    let searchTimeout;
    
    searchInput.addEventListener('input', function() {
        clearTimeout(searchTimeout);
        searchTimeout = setTimeout(() => {
            const query = this.value.toLowerCase().trim();
            
            if (query.length < 2) {
                hideSearchResults();
                return;
            }
            
            const results = searchDocumentation(query);
            displaySearchResults(results);
        }, 300);
    });
    
    // Hide results when clicking outside
    document.addEventListener('click', function(e) {
        if (!searchInput.contains(e.target) && !searchResults.contains(e.target)) {
            hideSearchResults();
        }
    });
}

function searchDocumentation(query) {
    const results = [];
    
    for (const item of window.searchIndex) {
        let score = 0;
        
        // Exact name match
        if (item.name.toLowerCase() === query) {
            score += 100;
        }
        // Name starts with query
        else if (item.name.toLowerCase().startsWith(query)) {
            score += 50;
        }
        // Name contains query
        else if (item.name.toLowerCase().includes(query)) {
            score += 25;
        }
        
        // Description contains query
        if (item.description.toLowerCase().includes(query)) {
            score += 10;
        }
        
        // Keywords match
        for (const keyword of item.keywords) {
            if (keyword.includes(query)) {
                score += 5;
            }
        }
        
        if (score > 0) {
            results.push({ ...item, score });
        }
    }
    
    return results.sort((a, b) => b.score - a.score).slice(0, 10);
}

function displaySearchResults(results) {
    const searchResults = document.getElementById('search-results');
    if (!searchResults) return;
    
    if (results.length === 0) {
        searchResults.innerHTML = '<div class="search-result">No results found</div>';
    } else {
        searchResults.innerHTML = results.map(result => `
            <div class="search-result">
                <a href="${result.url}" class="search-result-link">
                    <div class="search-result-name">${result.name}</div>
                    <div class="search-result-type">${result.kind}</div>
                    <div class="search-result-description">${result.description}</div>
                    <div class="search-result-module">${result.module}</div>
                </a>
            </div>
        `).join('');
    }
    
    searchResults.style.display = 'block';
}

function hideSearchResults() {
    const searchResults = document.getElementById('search-results');
    if (searchResults) {
        searchResults.style.display = 'none';
    }
}

function initializeTheme() {
    // Auto-detect system theme preference
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    const storedTheme = localStorage.getItem('cursed-docs-theme');
    
    const theme = storedTheme || (prefersDark ? 'dark' : 'light');
    setTheme(theme);
    
    // Listen for system theme changes
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
        if (!localStorage.getItem('cursed-docs-theme')) {
            setTheme(e.matches ? 'dark' : 'light');
        }
    });
}

function setTheme(theme) {
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('cursed-docs-theme', theme);
}

function initializeNavigation() {
    // Smooth scrolling for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
    
    // Highlight current section in TOC
    const sections = document.querySelectorAll('.item');
    const tocLinks = document.querySelectorAll('.toc a');
    
    if (sections.length > 0 && tocLinks.length > 0) {
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    const id = entry.target.id;
                    tocLinks.forEach(link => {
                        link.classList.remove('active');
                        if (link.getAttribute('href') === `#${id}`) {
                            link.classList.add('active');
                        }
                    });
                }
            });
        }, {
            rootMargin: '-20% 0px -80% 0px'
        });
        
        sections.forEach(section => observer.observe(section));
    }
}

// Add CSS for search results
const searchCSS = `
#search-results {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: var(--bg-color);
    border: 1px solid var(--border-color);
    border-top: none;
    border-radius: 0 0 0.5rem 0.5rem;
    max-height: 400px;
    overflow-y: auto;
    z-index: 1000;
    display: none;
}

.search-result {
    border-bottom: 1px solid var(--border-color);
}

.search-result:last-child {
    border-bottom: none;
}

.search-result-link {
    display: block;
    padding: 0.75rem;
    text-decoration: none;
    color: var(--text-color);
    transition: background-color 0.2s;
}

.search-result-link:hover {
    background-color: var(--hover-color);
}

.search-result-name {
    font-weight: 600;
    color: var(--primary-color);
    margin-bottom: 0.25rem;
}

.search-result-type {
    font-size: 0.8rem;
    color: var(--secondary-color);
    text-transform: uppercase;
    letter-spacing: 0.05em;
}

.search-result-description {
    font-size: 0.9rem;
    color: var(--text-color);
    margin: 0.25rem 0;
    opacity: 0.8;
}

.search-result-module {
    font-size: 0.8rem;
    color: #6b7280;
}

.toc a.active {
    color: var(--primary-color);
    font-weight: 600;
    background-color: var(--hover-color);
    border-radius: 0.25rem;
    padding-left: 0.5rem;
}
`;

// Inject the CSS
const style = document.createElement('style');
style.textContent = searchCSS;
document.head.appendChild(style);
"#.to_string()
    }
}
