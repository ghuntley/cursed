//! HTML template system for CURSED documentation generation
//!
//! Provides HTML templates for documentation pages, navigation generation,
//! CSS styling, responsive design, and search functionality integration.

use crate::docs::{DocError, DocResult, DocumentationItem, ItemType};
use std::collections::HashMap;
use tracing::{debug, instrument};

/// HTML template for documentation pages
pub struct HtmlTemplate {
    /// Template name
    pub name: String,
    /// Template content
    pub content: String,
    /// Template variables
    pub variables: HashMap<String, String>,
}

impl HtmlTemplate {
    /// Create a new HTML template
    pub fn new(name: String, content: String) -> Self {
        Self {
            name,
            content,
            variables: HashMap::new(),
        }
    }

    /// Set a template variable
    pub fn set_variable<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
        self.variables.insert(key.into(), value.into());
    }

    /// Render the template with current variables
    pub fn render(&self) -> String {
        let mut result = self.content.clone();
        
        for (key, value) in &self.variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        result
    }
}

/// Template engine for CURSED documentation
pub struct TemplateEngine {
    /// Registered templates
    templates: HashMap<String, HtmlTemplate>,
}

impl TemplateEngine {
    /// Create a new template engine with default templates
    pub fn new() -> Self {
        let mut engine = Self {
            templates: HashMap::new(),
        };
        
        engine.register_default_templates();
        engine
    }

    /// Register a template
    pub fn register_template(&mut self, name: String, template: HtmlTemplate) {
        self.templates.insert(name, template);
    }

    /// Get a template by name
    pub fn get_template(&self, name: &str) -> Option<&HtmlTemplate> {
        self.templates.get(name)
    }

    /// Register default templates
    fn register_default_templates(&mut self) {
        // Main page template
        self.register_template(
            "main".to_string(),
            HtmlTemplate::new("main".to_string(), MAIN_PAGE_TEMPLATE.to_string())
        );

        // Item page template
        self.register_template(
            "item".to_string(),
            HtmlTemplate::new("item".to_string(), ITEM_PAGE_TEMPLATE.to_string())
        );

        // Index template
        self.register_template(
            "index".to_string(),
            HtmlTemplate::new("index".to_string(), INDEX_PAGE_TEMPLATE.to_string())
        );

        // Search template
        self.register_template(
            "search".to_string(),
            HtmlTemplate::new("search".to_string(), SEARCH_PAGE_TEMPLATE.to_string())
        );
    }

    /// Render main page
    #[instrument(skip(self, package_name, items))]
    pub fn render_main_page(&self, package_name: &str, items: &[DocumentationItem]) -> DocResult<String> {
        let mut template = self.get_template("main")
            .ok_or_else(|| DocError::TemplateError("Main template not found".to_string()))?
            .clone();

        template.set_variable("package_name", package_name);
        template.set_variable("title", format!("{} Documentation", package_name));
        template.set_variable("navigation", self.generate_navigation(items)?);
        template.set_variable("content", self.generate_overview(items)?);
        template.set_variable("styles", CURSED_STYLES);
        template.set_variable("scripts", CURSED_SCRIPTS);

        Ok(template.render())
    }

    /// Render item page
    #[instrument(skip(self, item, all_items))]
    pub fn render_item_page(&self, item: &DocumentationItem, all_items: &[DocumentationItem]) -> DocResult<String> {
        let mut template = self.get_template("item")
            .ok_or_else(|| DocError::TemplateError("Item template not found".to_string()))?
            .clone();

        template.set_variable("title", format!("{} - CURSED Documentation", item.name));
        template.set_variable("item_name", &item.name);
        template.set_variable("item_type", &item.item_type.to_string());
        template.set_variable("navigation", self.generate_navigation(all_items)?);
        template.set_variable("content", self.generate_item_content(item)?);
        template.set_variable("styles", CURSED_STYLES);
        template.set_variable("scripts", CURSED_SCRIPTS);

        Ok(template.render())
    }

    /// Render index page
    #[instrument(skip(self, items))]
    pub fn render_index_page(&self, items: &[DocumentationItem]) -> DocResult<String> {
        let mut template = self.get_template("index")
            .ok_or_else(|| DocError::TemplateError("Index template not found".to_string()))?
            .clone();

        template.set_variable("title", "CURSED Documentation Index");
        template.set_variable("navigation", self.generate_navigation(items)?);
        template.set_variable("content", self.generate_index_content(items)?);
        template.set_variable("styles", CURSED_STYLES);
        template.set_variable("scripts", CURSED_SCRIPTS);

        Ok(template.render())
    }

    /// Generate navigation HTML
    fn generate_navigation(&self, items: &[DocumentationItem]) -> DocResult<String> {
        let mut nav = String::from(r#"<nav class="sidebar">"#);
        nav.push_str(r#"<div class="nav-section"><h3>Documentation</h3><ul>"#);
        nav.push_str(r#"<li><a href="index.html">Overview</a></li>"#);
        nav.push_str(r#"<li><a href="search.html">Search</a></li>"#);
        nav.push_str(r#"</ul></div>"#);

        // Group items by type
        let mut functions = Vec::new();
        let mut squads = Vec::new();
        let mut collabs = Vec::new();
        let mut others = Vec::new();

        for item in items {
            match item.item_type {
                ItemType::Function => functions.push(item),
                ItemType::Squad => squads.push(item),
                ItemType::Collab => collabs.push(item),
                _ => others.push(item),
            }
        }

        // Add functions section
        if !functions.is_empty() {
            nav.push_str(r#"<div class="nav-section"><h3>Functions</h3><ul>"#);
            for func in functions {
                nav.push_str(&format!(
                    r#"<li><a href="{}.html">{}</a></li>"#,
                    self.sanitize_filename(&func.name),
                    func.name
                ));
            }
            nav.push_str(r#"</ul></div>"#);
        }

        // Add squads section
        if !squads.is_empty() {
            nav.push_str(r#"<div class="nav-section"><h3>Squads</h3><ul>"#);
            for squad in squads {
                nav.push_str(&format!(
                    r#"<li><a href="{}.html">{}</a></li>"#,
                    self.sanitize_filename(&squad.name),
                    squad.name
                ));
            }
            nav.push_str(r#"</ul></div>"#);
        }

        // Add collabs section
        if !collabs.is_empty() {
            nav.push_str(r#"<div class="nav-section"><h3>Collabs</h3><ul>"#);
            for collab in collabs {
                nav.push_str(&format!(
                    r#"<li><a href="{}.html">{}</a></li>"#,
                    self.sanitize_filename(&collab.name),
                    collab.name
                ));
            }
            nav.push_str(r#"</ul></div>"#);
        }

        nav.push_str(r#"</nav>"#);
        Ok(nav)
    }

    /// Generate overview content
    fn generate_overview(&self, items: &[DocumentationItem]) -> DocResult<String> {
        let mut content = String::new();
        
        content.push_str(r#"<div class="overview">"#);
        content.push_str(r#"<h1>CURSED Documentation</h1>"#);
        content.push_str(r#"<p class="lead">Welcome to the CURSED programming language documentation.</p>"#);
        
        // Statistics
        let function_count = items.iter().filter(|i| i.item_type == ItemType::Function).count();
        let squad_count = items.iter().filter(|i| i.item_type == ItemType::Squad).count();
        let collab_count = items.iter().filter(|i| i.item_type == ItemType::Collab).count();

        content.push_str(r#"<div class="stats">"#);
        content.push_str(&format!(r#"<div class="stat"><span class="number">{}</span><span class="label">Functions</span></div>"#, function_count));
        content.push_str(&format!(r#"<div class="stat"><span class="number">{}</span><span class="label">Squads</span></div>"#, squad_count));
        content.push_str(&format!(r#"<div class="stat"><span class="number">{}</span><span class="label">Collabs</span></div>"#, collab_count));
        content.push_str(r#"</div>"#);

        // Quick links
        content.push_str(r#"<h2>Quick Navigation</h2>"#);
        content.push_str(r#"<div class="quick-links">"#);
        if function_count > 0 {
            content.push_str(r##"<a href="#functions" class="quick-link">Functions</a>"##);
        }
        if squad_count > 0 {
            content.push_str(r##"<a href="#squads" class="quick-link">Squads</a>"##);
        }
        if collab_count > 0 {
            content.push_str(r##"<a href="#collabs" class="quick-link">Collabs</a>"##);
        }
        content.push_str(r#"</div>"#);

        content.push_str(r#"</div>"#);
        Ok(content)
    }

    /// Generate item content
    fn generate_item_content(&self, item: &DocumentationItem) -> DocResult<String> {
        let mut content = String::new();

        content.push_str(r#"<div class="item-content">"#);
        
        // Header
        content.push_str(&format!(
            r#"<header class="item-header"><h1 class="item-title">{} <span class="item-type">{}</span></h1>"#,
            item.name, item.item_type
        ));

        // Deprecated warning
        if item.is_deprecated() {
            content.push_str(r#"<div class="deprecated-warning">⚠️ This item is deprecated</div>"#);
        }

        content.push_str(r#"</header>"#);

        // Signature
        if let Some(signature) = &item.signature {
            content.push_str(r#"<div class="signature">"#);
            content.push_str(r#"<h2>Signature</h2>"#);
            content.push_str(&format!(r#"<pre><code class="language-cursed">{}</code></pre>"#, signature));
            content.push_str(r#"</div>"#);
        }

        // Description
        if let Some(description) = item.description() {
            content.push_str(r#"<div class="description">"#);
            content.push_str(r#"<h2>Description</h2>"#);
            content.push_str(&format!(r#"<div class="description-content">{}</div>"#, 
                self.markdown_to_html(description)?));
            content.push_str(r#"</div>"#);
        }

        // Parameters
        if !item.parameters.is_empty() {
            content.push_str(r#"<div class="parameters">"#);
            content.push_str(r#"<h2>Parameters</h2>"#);
            content.push_str(r#"<table class="params-table">"#);
            content.push_str(r#"<thead><tr><th>Name</th><th>Type</th><th>Description</th></tr></thead><tbody>"#);
            
            for param in &item.parameters {
                content.push_str(&format!(
                    r#"<tr><td><code>{}</code></td><td><code>{}</code></td><td>{}</td></tr>"#,
                    param.name,
                    param.param_type,
                    param.description.as_deref().unwrap_or("No description")
                ));
            }
            
            content.push_str(r#"</tbody></table></div>"#);
        }

        // Return type
        if let Some(return_type) = &item.return_type {
            content.push_str(r#"<div class="return-type">"#);
            content.push_str(r#"<h2>Returns</h2>"#);
            content.push_str(&format!(r#"<p><code>{}</code>"#, return_type));
            
            if let Some(return_desc) = item.return_description() {
                content.push_str(&format!(r#" - {}"#, return_desc));
            }
            
            content.push_str(r#"</p></div>"#);
        }

        // Examples
        if !item.examples.is_empty() {
            content.push_str(r#"<div class="examples">"#);
            content.push_str(r#"<h2>Examples</h2>"#);
            
            for (i, example) in item.examples.iter().enumerate() {
                content.push_str(&format!(
                    r#"<div class="example"><h3>Example {}</h3><pre><code class="language-cursed">{}</code></pre></div>"#,
                    i + 1, example
                ));
            }
            
            content.push_str(r#"</div>"#);
        }

        // Fields (for squads/collabs)
        if !item.fields.is_empty() {
            content.push_str(r#"<div class="fields">"#);
            content.push_str(r#"<h2>Fields</h2>"#);
            content.push_str(r#"<table class="fields-table">"#);
            content.push_str(r#"<thead><tr><th>Name</th><th>Type</th><th>Description</th></tr></thead><tbody>"#);
            
            for field in &item.fields {
                content.push_str(&format!(
                    r#"<tr><td><code>{}</code></td><td><code>{}</code></td><td>{}</td></tr>"#,
                    field.name,
                    field.field_type,
                    field.description.as_deref().unwrap_or("No description")
                ));
            }
            
            content.push_str(r#"</tbody></table></div>"#);
        }

        // Methods (for squads/collabs)
        if !item.methods.is_empty() {
            content.push_str(r#"<div class="methods">"#);
            content.push_str(r#"<h2>Methods</h2>"#);
            
            for method in &item.methods {
                content.push_str(&format!(
                    r#"<div class="method"><h3>{}</h3><pre><code class="language-cursed">{}</code></pre></div>"#,
                    method.name,
                    method.signature.as_deref().unwrap_or("No signature")
                ));
            }
            
            content.push_str(r#"</div>"#);
        }

        content.push_str(r#"</div>"#);
        Ok(content)
    }

    /// Generate index content
    fn generate_index_content(&self, items: &[DocumentationItem]) -> DocResult<String> {
        let mut content = String::new();

        content.push_str(r#"<div class="index-content">"#);
        content.push_str(r#"<h1>Documentation Index</h1>"#);

        // Group items by type
        let mut groups: HashMap<ItemType, Vec<&DocumentationItem>> = HashMap::new();
        for item in items {
            groups.entry(item.item_type.clone()).or_insert_with(Vec::new).push(item);
        }

        for (item_type, group_items) in groups {
            content.push_str(&format!(r#"<h2>{}</h2>"#, item_type));
            content.push_str(r#"<div class="item-list">"#);

            for item in group_items {
                content.push_str(&format!(
                    r#"<div class="item-card"><a href="{}.html"><h3>{}</h3>"#,
                    self.sanitize_filename(&item.name),
                    item.name
                ));

                if let Some(description) = item.description() {
                    let short_desc = if description.len() > 100 {
                        format!("{}...", &description[..97])
                    } else {
                        description.to_string()
                    };
                    content.push_str(&format!(r#"<p>{}</p>"#, short_desc));
                }

                content.push_str(r#"</a></div>"#);
            }

            content.push_str(r#"</div>"#);
        }

        content.push_str(r#"</div>"#);
        Ok(content)
    }

    /// Convert markdown to HTML (simplified)
    fn markdown_to_html(&self, markdown: &str) -> DocResult<String> {
        // This is a simplified markdown converter
        // In a real implementation, you'd use a proper markdown library
        let mut html = markdown.to_string();
        
        // Convert **bold** to <strong>
        html = html.replace("**", "<strong>").replace("</strong>**", "</strong>");
        
        // Convert *italic* to <em>
        html = html.replace("*", "<em>").replace("</em>*", "</em>");
        
        // Convert `code` to <code>
        html = html.replace("`", "<code>").replace("</code>`", "</code>");
        
        // Convert newlines to <br>
        html = html.replace('\n', "<br>");
        
        Ok(html)
    }

    /// Sanitize filename for HTML links
    fn sanitize_filename(&self, name: &str) -> String {
        name.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect()
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for HtmlTemplate {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            content: self.content.clone(),
            variables: self.variables.clone(),
        }
    }
}

// Template constants
const MAIN_PAGE_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <style>{{styles}}</style>
</head>
<body>
    {{navigation}}
    <main class="main-content">
        {{content}}
    </main>
    <script>{{scripts}}</script>
</body>
</html>"#;

const ITEM_PAGE_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <style>{{styles}}</style>
</head>
<body>
    {{navigation}}
    <main class="main-content">
        {{content}}
    </main>
    <script>{{scripts}}</script>
</body>
</html>"#;

const INDEX_PAGE_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <style>{{styles}}</style>
</head>
<body>
    {{navigation}}
    <main class="main-content">
        {{content}}
    </main>
    <script>{{scripts}}</script>
</body>
</html>"#;

const SEARCH_PAGE_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Search - CURSED Documentation</title>
    <style>{{styles}}</style>
</head>
<body>
    {{navigation}}
    <main class="main-content">
        <div class="search-page">
            <h1>Search Documentation</h1>
            <div class="search-container">
                <input type="text" id="search-input" placeholder="Search functions, squads, collabs..." />
                <button id="search-button">Search</button>
            </div>
            <div id="search-results"></div>
        </div>
    </main>
    <script>{{scripts}}</script>
</body>
</html>"#;

const CURSED_STYLES: &str = r#"
/* CURSED Documentation Styles */
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    line-height: 1.6;
    color: #333;
    background-color: #f8f9fa;
    display: flex;
    min-height: 100vh;
}

.sidebar {
    width: 300px;
    background: #2c3e50;
    color: white;
    padding: 20px;
    overflow-y: auto;
    position: fixed;
    height: 100vh;
}

.nav-section {
    margin-bottom: 30px;
}

.nav-section h3 {
    color: #ecf0f1;
    margin-bottom: 10px;
    font-size: 16px;
    text-transform: uppercase;
    letter-spacing: 1px;
}

.nav-section ul {
    list-style: none;
}

.nav-section li {
    margin-bottom: 5px;
}

.nav-section a {
    color: #bdc3c7;
    text-decoration: none;
    padding: 5px 0;
    display: block;
    transition: color 0.3s ease;
}

.nav-section a:hover {
    color: #3498db;
}

.main-content {
    margin-left: 300px;
    padding: 30px;
    flex: 1;
    max-width: calc(100% - 300px);
}

.overview {
    max-width: 800px;
}

.overview h1 {
    color: #2c3e50;
    margin-bottom: 20px;
    font-size: 2.5rem;
}

.lead {
    font-size: 1.2rem;
    color: #7f8c8d;
    margin-bottom: 30px;
}

.stats {
    display: flex;
    gap: 30px;
    margin-bottom: 40px;
}

.stat {
    text-align: center;
    padding: 20px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.stat .number {
    display: block;
    font-size: 2rem;
    font-weight: bold;
    color: #3498db;
}

.stat .label {
    color: #7f8c8d;
    text-transform: uppercase;
    font-size: 0.9rem;
    letter-spacing: 1px;
}

.quick-links {
    display: flex;
    gap: 15px;
    flex-wrap: wrap;
}

.quick-link {
    background: #3498db;
    color: white;
    padding: 10px 20px;
    border-radius: 5px;
    text-decoration: none;
    transition: background 0.3s ease;
}

.quick-link:hover {
    background: #2980b9;
}

.item-content {
    max-width: 900px;
}

.item-header {
    border-bottom: 2px solid #ecf0f1;
    margin-bottom: 30px;
    padding-bottom: 20px;
}

.item-title {
    color: #2c3e50;
    font-size: 2rem;
    margin-bottom: 10px;
}

.item-type {
    background: #3498db;
    color: white;
    padding: 4px 12px;
    border-radius: 15px;
    font-size: 0.8rem;
    text-transform: uppercase;
    letter-spacing: 1px;
}

.deprecated-warning {
    background: #e74c3c;
    color: white;
    padding: 10px 15px;
    border-radius: 5px;
    margin-top: 10px;
    font-weight: bold;
}

.signature {
    background: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 5px;
    padding: 20px;
    margin-bottom: 30px;
}

.signature h2 {
    margin-bottom: 15px;
    color: #495057;
}

.signature pre {
    background: #2c3e50;
    color: #ecf0f1;
    padding: 15px;
    border-radius: 5px;
    overflow-x: auto;
    font-family: 'Consolas', 'Monaco', monospace;
}

.description {
    margin-bottom: 30px;
}

.description h2 {
    color: #495057;
    margin-bottom: 15px;
}

.description-content {
    line-height: 1.8;
}

.parameters, .fields, .methods {
    margin-bottom: 30px;
}

.parameters h2, .fields h2, .methods h2 {
    color: #495057;
    margin-bottom: 15px;
}

.params-table, .fields-table {
    width: 100%;
    border-collapse: collapse;
    background: white;
    border-radius: 5px;
    overflow: hidden;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.params-table th, .fields-table th {
    background: #34495e;
    color: white;
    padding: 15px;
    text-align: left;
}

.params-table td, .fields-table td {
    padding: 12px 15px;
    border-bottom: 1px solid #ecf0f1;
}

.params-table code, .fields-table code {
    background: #f8f9fa;
    padding: 2px 6px;
    border-radius: 3px;
    font-family: 'Consolas', 'Monaco', monospace;
}

.examples {
    margin-bottom: 30px;
}

.example {
    margin-bottom: 20px;
}

.example h3 {
    color: #495057;
    margin-bottom: 10px;
}

.example pre {
    background: #2c3e50;
    color: #ecf0f1;
    padding: 15px;
    border-radius: 5px;
    overflow-x: auto;
    font-family: 'Consolas', 'Monaco', monospace;
}

.return-type {
    margin-bottom: 30px;
}

.return-type h2 {
    color: #495057;
    margin-bottom: 15px;
}

.method {
    background: white;
    border: 1px solid #dee2e6;
    border-radius: 5px;
    padding: 20px;
    margin-bottom: 15px;
}

.method h3 {
    color: #495057;
    margin-bottom: 10px;
}

.index-content {
    max-width: 1000px;
}

.item-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 20px;
    margin-bottom: 40px;
}

.item-card {
    background: white;
    border: 1px solid #dee2e6;
    border-radius: 8px;
    padding: 20px;
    transition: box-shadow 0.3s ease;
}

.item-card:hover {
    box-shadow: 0 4px 8px rgba(0,0,0,0.15);
}

.item-card a {
    text-decoration: none;
    color: inherit;
}

.item-card h3 {
    color: #2c3e50;
    margin-bottom: 10px;
}

.item-card p {
    color: #7f8c8d;
    line-height: 1.5;
}

.search-page {
    max-width: 800px;
}

.search-container {
    display: flex;
    gap: 10px;
    margin-bottom: 30px;
}

#search-input {
    flex: 1;
    padding: 12px;
    border: 1px solid #dee2e6;
    border-radius: 5px;
    font-size: 16px;
}

#search-button {
    padding: 12px 24px;
    background: #3498db;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    transition: background 0.3s ease;
}

#search-button:hover {
    background: #2980b9;
}

#search-results {
    background: white;
    border-radius: 5px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    min-height: 200px;
}

/* Responsive design */
@media (max-width: 768px) {
    body {
        flex-direction: column;
    }
    
    .sidebar {
        width: 100%;
        position: relative;
        height: auto;
    }
    
    .main-content {
        margin-left: 0;
        max-width: 100%;
    }
    
    .stats {
        flex-direction: column;
        gap: 15px;
    }
    
    .item-list {
        grid-template-columns: 1fr;
    }
}
"#;

const CURSED_SCRIPTS: &str = r#"
// CURSED Documentation Scripts
document.addEventListener('DOMContentLoaded', function() {
    // Search functionality
    const searchInput = document.getElementById('search-input');
    const searchButton = document.getElementById('search-button');
    const searchResults = document.getElementById('search-results');
    
    if (searchInput && searchButton && searchResults) {
        searchButton.addEventListener('click', performSearch);
        searchInput.addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                performSearch();
            }
        });
    }
    
    function performSearch() {
        const query = searchInput.value.trim().toLowerCase();
        if (!query) return;
        
        // This would be implemented with actual search data
        searchResults.innerHTML = '<p>Search functionality would be implemented here with actual documentation data.</p>';
    }
    
    // Syntax highlighting for code blocks
    const codeBlocks = document.querySelectorAll('code.language-cursed');
    codeBlocks.forEach(block => {
        highlightCursedSyntax(block);
    });
    
    function highlightCursedSyntax(block) {
        let content = block.innerHTML;
        
        // Highlight CURSED keywords
        const keywords = ['slay', 'sus', 'facts', 'lowkey', 'highkey', 'periodt', 'bestie', 'flex', 'yolo', 'squad', 'collab'];
        keywords.forEach(keyword => {
            const regex = new RegExp('\\b' + keyword + '\\b', 'g');
            content = content.replace(regex, '<span style="color: #e74c3c; font-weight: bold;">' + keyword + '</span>');
        });
        
        // Highlight types
        const types = ['normie', 'str', 'bool', 'void'];
        types.forEach(type => {
            const regex = new RegExp('\\b' + type + '\\b', 'g');
            content = content.replace(regex, '<span style="color: #3498db; font-weight: bold;">' + type + '</span>');
        });
        
        block.innerHTML = content;
    }
});
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_creation() {
        let template = HtmlTemplate::new(
            "test".to_string(),
            "Hello {{name}}!".to_string()
        );
        
        assert_eq!(template.name, "test");
        assert!(template.content.contains("{{name}}"));
    }

    #[test]
    fn test_template_rendering() {
        let mut template = HtmlTemplate::new(
            "test".to_string(),
            "Hello {{name}}! Welcome to {{site}}.".to_string()
        );
        
        template.set_variable("name", "World");
        template.set_variable("site", "CURSED Docs");
        
        let result = template.render();
        assert_eq!(result, "Hello World! Welcome to CURSED Docs.");
    }

    #[test]
    fn test_template_engine_creation() {
        let engine = TemplateEngine::new();
        
        assert!(engine.get_template("main").is_some());
        assert!(engine.get_template("item").is_some());
        assert!(engine.get_template("index").is_some());
        assert!(engine.get_template("nonexistent").is_none());
    }

    #[test]
    fn test_filename_sanitization() {
        let engine = TemplateEngine::new();
        
        assert_eq!(engine.sanitize_filename("test-function"), "test_function");
        assert_eq!(engine.sanitize_filename("test.function"), "test_function");
        assert_eq!(engine.sanitize_filename("test function"), "test_function");
        assert_eq!(engine.sanitize_filename("test_function"), "test_function");
    }
}
