//! Advanced HTML documentation generator for CURSED language
//!
//! Provides production-quality HTML generation with:
//! - Template-based generation system
//! - Responsive CSS styling
//! - Syntax highlighting for CURSED code
//! - Cross-reference linking
//! - Navigation generation
//! - Search functionality preparation

use crate::docs::{DocError, DocResult, DocumentationItem, ItemType, PackageDocumentation};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument};

/// Configuration for HTML generation
#[derive(Debug, Clone)]
pub struct HtmlGeneratorConfig {
    /// Output directory for generated files
    pub output_dir: PathBuf,
    /// Whether to include syntax highlighting
    pub enable_syntax_highlighting: bool,
    /// Whether to generate search index
    pub enable_search: bool,
    /// Custom CSS to include
    pub custom_css: Option<String>,
    /// Custom JavaScript to include
    pub custom_js: Option<String>,
    /// Project name for branding
    pub project_name: String,
    /// Base URL for absolute links
    pub base_url: Option<String>,
    /// Whether to minify output
    pub minify_output: bool,
}

impl Default for HtmlGeneratorConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("docs"),
            enable_syntax_highlighting: true,
            enable_search: true,
            custom_css: None,
            custom_js: None,
            project_name: "CURSED".to_string(),
            base_url: None,
            minify_output: false,
        }
    }
}

/// Result of HTML generation
#[derive(Debug)]
pub struct GenerationResult {
    /// Number of files generated
    pub files_generated: usize,
    /// Paths of generated files
    pub generated_files: Vec<PathBuf>,
    /// Generation statistics
    pub stats: GenerationStats,
}

/// Statistics about the generation process
#[derive(Debug)]
pub struct GenerationStats {
    /// Number of functions documented
    pub function_count: usize,
    /// Number of types documented
    pub type_count: usize,
    /// Number of packages documented
    pub package_count: usize,
    /// Total documentation size in bytes
    pub total_size: usize,
}

/// Advanced HTML documentation generator
pub struct HtmlGenerator {
    /// Configuration
    config: HtmlGeneratorConfig,
    /// Template registry
    templates: HashMap<String, String>,
    /// Cross-reference map
    cross_refs: HashMap<String, String>,
    /// Generated files tracking
    generated_files: Vec<PathBuf>,
}

impl HtmlGenerator {
    /// Create a new HTML generator
    pub fn new(config: HtmlGeneratorConfig) -> Self {
        let mut generator = Self {
            config,
            templates: HashMap::new(),
            cross_refs: HashMap::new(),
            generated_files: Vec::new(),
        };
        
        generator.register_default_templates();
        generator
    }

    /// Generate complete HTML documentation
    #[instrument(skip(self, package))]
    pub fn generate(&mut self, package: &PackageDocumentation) -> DocResult<GenerationResult> {
        info!("Starting HTML documentation generation for: {}", package.name);

        // Prepare output directory
        self.prepare_output_directory()?;

        // Collect all documentation items
        let all_items = package.root_module.all_items();
        let items: Vec<DocumentationItem> = all_items.into_iter().cloned().collect();

        // Build cross-reference map
        self.build_cross_reference_map(&items);

        // Generate main index page
        self.generate_index_page(&package.name, &items)?;

        // Generate package overview page
        self.generate_package_page(package)?;

        // Generate individual item pages
        for item in &items {
            self.generate_item_page(item, &items)?;
        }

        // Generate search page and data
        if self.config.enable_search {
            self.generate_search_page(&items)?;
            self.generate_search_data(&items)?;
        }

        // Copy static assets
        self.copy_static_assets()?;

        // Generate navigation files
        self.generate_navigation_data(&items)?;

        // Create generation result
        let stats = self.calculate_stats(&items)?;
        let result = GenerationResult {
            files_generated: self.generated_files.len(),
            generated_files: self.generated_files.clone(),
            stats,
        };

        info!("Generated {} documentation files", result.files_generated);
        Ok(result)
    }

    /// Prepare output directory
    fn prepare_output_directory(&self) -> DocResult<()> {
        // Create main output directory
        fs::create_dir_all(&self.config.output_dir)
            .map_err(|e| DocError::IoError(format!("Failed to create output directory: {}", e)))?;

        // Create subdirectories
        let subdirs = ["assets", "types", "functions", "packages"];
        for subdir in &subdirs {
            let path = self.config.output_dir.join(subdir);
            fs::create_dir_all(&path)
                .map_err(|e| DocError::IoError(format!("Failed to create {}: {}", subdir, e)))?;
        }

        Ok(())
    }

    /// Build cross-reference map for linking
    fn build_cross_reference_map(&mut self, items: &[DocumentationItem]) {
        self.cross_refs.clear();
        
        for item in items {
            let filename = format!("{}.html", self.sanitize_filename(&item.name));
            let path = match item.item_type {
                ItemType::Function => format!("functions/{}", filename),
                ItemType::Squad | ItemType::Collab => format!("types/{}", filename),
                _ => filename,
            };
            
            self.cross_refs.insert(item.name.clone(), path);
        }
    }

    /// Generate main index page
    fn generate_index_page(&mut self, package_name: &str, items: &[DocumentationItem]) -> DocResult<()> {
        debug!("Generating index page");

        let template = self.get_template("index")?;
        let content = self.render_template(&template, &[
            ("title", &format!("{} Documentation", package_name)),
            ("project_name", package_name),
            ("content", &self.generate_index_content(items)?),
            ("navigation", &self.generate_navigation_html(items)?),
        ])?;

        let file_path = self.config.output_dir.join("index.html");
        self.write_file(&file_path, &content)?;
        Ok(())
    }

    /// Generate package overview page
    fn generate_package_page(&mut self, package: &PackageDocumentation) -> DocResult<()> {
        debug!("Generating package page");

        let template = self.get_template("package")?;
        let content = self.render_template(&template, &[
            ("title", &format!("{} Package", package.name)),
            ("package_name", &package.name),
            ("content", &self.generate_package_content(package)?),
            ("navigation", &self.generate_navigation_html(&package.root_module.all_items().into_iter().cloned().collect::<Vec<_>>())?),
        ])?;

        let file_path = self.config.output_dir.join("packages").join("index.html");
        self.write_file(&file_path, &content)?;
        Ok(())
    }

    /// Generate individual item documentation page
    fn generate_item_page(&mut self, item: &DocumentationItem, all_items: &[DocumentationItem]) -> DocResult<()> {
        debug!("Generating page for: {}", item.name);

        let template = match item.item_type {
            ItemType::Function => self.get_template("function")?,
            ItemType::Squad | ItemType::Collab => self.get_template("type")?,
            _ => self.get_template("item")?,
        };

        let enhanced_item = self.enhance_item_with_links(item)?;
        let content = self.render_template(&template, &[
            ("title", &format!("{} - {} Documentation", item.name, self.config.project_name)),
            ("item_name", &item.name),
            ("item_type", &item.item_type.to_string()),
            ("content", &self.generate_item_content(&enhanced_item)?),
            ("navigation", &self.generate_navigation_html(all_items)?),
            ("breadcrumbs", &self.generate_breadcrumbs(item)?),
        ])?;

        let filename = format!("{}.html", self.sanitize_filename(&item.name));
        let file_path = match item.item_type {
            ItemType::Function => self.config.output_dir.join("functions").join(filename),
            ItemType::Squad | ItemType::Collab => self.config.output_dir.join("types").join(filename),
            _ => self.config.output_dir.join(filename),
        };

        self.write_file(&file_path, &content)?;
        Ok(())
    }

    /// Generate search page
    fn generate_search_page(&mut self, items: &[DocumentationItem]) -> DocResult<()> {
        debug!("Generating search page");

        let template = self.get_template("search")?;
        let content = self.render_template(&template, &[
            ("title", &format!("Search - {} Documentation", self.config.project_name)),
            ("project_name", &self.config.project_name),
            ("navigation", &self.generate_navigation_html(items)?),
        ])?;

        let file_path = self.config.output_dir.join("search.html");
        self.write_file(&file_path, &content)?;
        Ok(())
    }

    /// Generate search data for client-side search
    fn generate_search_data(&mut self, items: &[DocumentationItem]) -> DocResult<()> {
        debug!("Generating search data");

        let mut search_items = Vec::new();
        
        for item in items {
            let url = self.cross_refs.get(&item.name)
                .map(|path| format!("/{}", path))
                .unwrap_or_else(|| format!("/{}.html", self.sanitize_filename(&item.name)));

            let search_item = serde_json::json!({
                "name": item.name,
                "type": item.item_type.to_string(),
                "description": item.description().unwrap_or(""),
                "url": url,
                "keywords": self.extract_search_keywords(item),
                "category": self.categorize_item(item),
            });
            search_items.push(search_item);
        }

        let search_data = serde_json::json!({
            "items": search_items,
            "meta": {
                "generated": chrono::Utc::now().to_rfc3339(),
                "total_items": search_items.len(),
                "project": self.config.project_name,
            }
        });

        let js_content = format!(
            "window.CURSED_SEARCH_INDEX = {};",
            serde_json::to_string_pretty(&search_data)
                .map_err(|e| DocError::IoError(format!("Failed to serialize search data: {}", e)))?
        );

        let file_path = self.config.output_dir.join("assets").join("search-index.js");
        self.write_file(&file_path, &js_content)?;
        Ok(())
    }

    /// Generate navigation data
    fn generate_navigation_data(&mut self, items: &[DocumentationItem]) -> DocResult<()> {
        debug!("Generating navigation data");

        let mut nav_data = HashMap::new();
        
        // Group by type
        let mut by_type: HashMap<String, Vec<&DocumentationItem>> = HashMap::new();
        for item in items {
            by_type.entry(item.item_type.to_string())
                .or_insert_with(Vec::new)
                .push(item);
        }

        for (type_name, type_items) in by_type {
            let items_data: Vec<_> = type_items.iter().map(|item| {
                serde_json::json!({
                    "name": item.name,
                    "url": self.cross_refs.get(&item.name).unwrap_or(&format!("{}.html", self.sanitize_filename(&item.name))),
                    "description": item.description().unwrap_or("").chars().take(100).collect::<String>(),
                })
            }).collect();
            
            nav_data.insert(type_name, items_data);
        }

        let nav_js = format!(
            "window.CURSED_NAVIGATION = {};",
            serde_json::to_string_pretty(&nav_data)
                .map_err(|e| DocError::IoError(format!("Failed to serialize navigation data: {}", e)))?
        );

        let file_path = self.config.output_dir.join("assets").join("navigation.js");
        self.write_file(&file_path, &nav_js)?;
        Ok(())
    }

    /// Copy static assets (CSS, JS, images)
    fn copy_static_assets(&mut self) -> DocResult<()> {
        debug!("Copying static assets");

        // Generate main CSS file
        let css_content = self.generate_main_css()?;
        let css_path = self.config.output_dir.join("assets").join("docs.css");
        self.write_file(&css_path, &css_content)?;

        // Generate main JavaScript file
        let js_content = self.generate_main_js()?;
        let js_path = self.config.output_dir.join("assets").join("docs.js");
        self.write_file(&js_path, &js_content)?;

        // Generate favicon
        let favicon_content = self.generate_favicon();
        let favicon_path = self.config.output_dir.join("favicon.ico");
        fs::write(&favicon_path, favicon_content)
            .map_err(|e| DocError::IoError(format!("Failed to write favicon: {}", e)))?;
        self.generated_files.push(favicon_path);

        // Generate additional assets if syntax highlighting is enabled
        if self.config.enable_syntax_highlighting {
            let highlight_css = self.generate_syntax_highlight_css();
            let highlight_path = self.config.output_dir.join("assets").join("highlight.css");
            self.write_file(&highlight_path, &highlight_css)?;

            let highlight_js = self.generate_syntax_highlight_js();
            let highlight_js_path = self.config.output_dir.join("assets").join("highlight.js");
            self.write_file(&highlight_js_path, &highlight_js)?;
        }

        Ok(())
    }

    /// Enhance item with cross-reference links
    fn enhance_item_with_links(&self, item: &DocumentationItem) -> DocResult<DocumentationItem> {
        let mut enhanced = item.clone();
        
        // Enhance description
        if let Some(description) = enhanced.description() {
            let enhanced_desc = self.add_cross_reference_links(&description);
            if let Some(ref mut doc_comment) = enhanced.doc_comment {
                doc_comment.description = enhanced_desc;
            }
        }

        // Enhance parameter descriptions
        for param in &mut enhanced.parameters {
            if let Some(ref description) = param.description {
                param.description = Some(self.add_cross_reference_links(description));
            }
        }

        Ok(enhanced)
    }

    /// Add cross-reference links to text
    fn add_cross_reference_links(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        for (item_name, url) in &self.cross_refs {
            // Simple word replacement (avoiding regex for now)
            if result.contains(item_name) {
                let replacement = format!(r#"<a href="{}" class="cross-ref">{}</a>"#, url, item_name);
                result = result.replace(item_name, &replacement);
            }
        }
        
        result
    }

    /// Generate breadcrumbs for an item
    fn generate_breadcrumbs(&self, item: &DocumentationItem) -> DocResult<String> {
        let mut breadcrumbs = String::new();
        
        breadcrumbs.push_str(r#"<nav class="breadcrumbs">"#);
        breadcrumbs.push_str(r#"<a href="/">Home</a>"#);
        breadcrumbs.push_str(r#"<span class="separator">›</span>"#);
        
        match item.item_type {
            ItemType::Function => {
                breadcrumbs.push_str(r#"<a href="/functions/">Functions</a>"#);
            }
            ItemType::Squad | ItemType::Collab => {
                breadcrumbs.push_str(r#"<a href="/types/">Types</a>"#);
            }
            _ => {}
        }
        
        breadcrumbs.push_str(r#"<span class="separator">›</span>"#);
        breadcrumbs.push_str(&format!(r#"<span class="current">{}</span>"#, item.name));
        breadcrumbs.push_str(r#"</nav>"#);
        
        Ok(breadcrumbs)
    }

    /// Generate content for index page
    fn generate_index_content(&self, items: &[DocumentationItem]) -> DocResult<String> {
        let mut content = String::new();
        
        content.push_str(&format!(r#"<div class="hero">
            <h1>Welcome to {} Documentation</h1>
            <p class="lead">Comprehensive documentation for the CURSED programming language</p>
        </div>"#, self.config.project_name));

        // Statistics
        let stats = self.calculate_stats(items)?;
        content.push_str(&format!(r#"<div class="stats-grid">
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Functions</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Types</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">{}</div>
                <div class="stat-label">Packages</div>
            </div>
        </div>"#, stats.function_count, stats.type_count, stats.package_count));

        // Recent items
        let recent_items: Vec<_> = items.iter().take(6).collect();
        if !recent_items.is_empty() {
            content.push_str(r#"<section class="recent-items">
                <h2>Recent Documentation</h2>
                <div class="item-grid">"#);
            
            for item in recent_items {
                let url = self.cross_refs.get(&item.name).unwrap_or(&format!("{}.html", self.sanitize_filename(&item.name)));
                content.push_str(&format!(r#"<div class="item-preview">
                    <h3><a href="{}">{}</a></h3>
                    <span class="item-type">{}</span>
                    <p>{}</p>
                </div>"#, 
                    url, 
                    item.name, 
                    item.item_type,
                    item.description().unwrap_or("No description").chars().take(120).collect::<String>()
                ));
            }
            
            content.push_str(r#"</div></section>"#);
        }

        Ok(content)
    }

    /// Generate content for package page
    fn generate_package_content(&self, package: &PackageDocumentation) -> DocResult<String> {
        let mut content = String::new();
        
        content.push_str(&format!(r#"<div class="package-header">
            <h1>Package: {}</h1>
            <p class="package-description">Package documentation and module overview</p>
        </div>"#, package.name));

        // Module information
        content.push_str(r#"<section class="modules">
            <h2>Modules</h2>
            <div class="module-list">"#);
        
        let all_items = package.root_module.all_items();
        let items: Vec<_> = all_items.into_iter().collect();
        
        // Group by module/category
        let mut by_type: HashMap<String, Vec<&DocumentationItem>> = HashMap::new();
        for item in &items {
            by_type.entry(item.item_type.to_string())
                .or_insert_with(Vec::new)
                .push(item);
        }

        for (type_name, type_items) in by_type {
            content.push_str(&format!(r#"<div class="module-section">
                <h3>{}</h3>
                <ul class="item-list">"#, type_name));
            
            for item in type_items {
                let url = self.cross_refs.get(&item.name).unwrap_or(&format!("{}.html", self.sanitize_filename(&item.name)));
                content.push_str(&format!(r#"<li><a href="{}">{}</a></li>"#, url, item.name));
            }
            
            content.push_str(r#"</ul></div>"#);
        }

        content.push_str(r#"</div></section>"#);
        Ok(content)
    }

    /// Generate content for individual item
    fn generate_item_content(&self, item: &DocumentationItem) -> DocResult<String> {
        let mut content = String::new();

        // Header with title and type
        content.push_str(&format!(r#"<header class="item-header">
            <h1>{}</h1>
            <span class="item-type-badge item-type-{}">{}</span>"#, 
            item.name, 
            item.item_type.to_string().to_lowercase(),
            item.item_type
        ));

        if item.is_deprecated() {
            content.push_str(r#"<div class="deprecated-notice">⚠️ This item is deprecated and may be removed in future versions.</div>"#);
        }

        content.push_str(r#"</header>"#);

        // Signature
        if let Some(signature) = &item.signature {
            content.push_str(&format!(r#"<section class="signature">
                <h2>Signature</h2>
                <pre class="code-block"><code class="language-cursed">{}</code></pre>
            </section>"#, self.escape_html(signature)));
        }

        // Description
        if let Some(description) = item.description() {
            content.push_str(&format!(r#"<section class="description">
                <h2>Description</h2>
                <div class="description-content">{}</div>
            </section>"#, self.format_description(description)?));
        }

        // Parameters
        if !item.parameters.is_empty() {
            content.push_str(r#"<section class="parameters">
                <h2>Parameters</h2>
                <div class="parameter-list">"#);
            
            for param in &item.parameters {
                content.push_str(&format!(r#"<div class="parameter">
                    <div class="parameter-header">
                        <code class="parameter-name">{}</code>
                        <code class="parameter-type">{}</code>
                    </div>"#, param.name, param.param_type));
                
                if let Some(description) = &param.description {
                    content.push_str(&format!(r#"<div class="parameter-description">{}</div>"#, description));
                }
                
                content.push_str(r#"</div>"#);
            }
            
            content.push_str(r#"</div></section>"#);
        }

        // Return type
        if let Some(return_type) = &item.return_type {
            content.push_str(&format!(r#"<section class="return-type">
                <h2>Returns</h2>
                <div class="return-info">
                    <code class="return-type-code">{}</code>"#, return_type));
            
            if let Some(return_desc) = item.return_description() {
                content.push_str(&format!(r#"<p class="return-description">{}</p>"#, return_desc));
            }
            
            content.push_str(r#"</div></section>"#);
        }

        // Examples
        if !item.examples.is_empty() {
            content.push_str(r#"<section class="examples">
                <h2>Examples</h2>"#);
            
            for (i, example) in item.examples.iter().enumerate() {
                content.push_str(&format!(r#"<div class="example">
                    <h3>Example {}</h3>
                    <pre class="code-block"><code class="language-cursed">{}</code></pre>
                </div>"#, i + 1, self.escape_html(example)));
            }
            
            content.push_str(r#"</section>"#);
        }

        // Fields (for types)
        if !item.fields.is_empty() {
            content.push_str(r#"<section class="fields">
                <h2>Fields</h2>
                <div class="field-list">"#);
            
            for field in &item.fields {
                content.push_str(&format!(r#"<div class="field">
                    <div class="field-header">
                        <code class="field-name">{}</code>
                        <code class="field-type">{}</code>
                    </div>"#, field.name, field.field_type));
                
                if let Some(description) = &field.description {
                    content.push_str(&format!(r#"<div class="field-description">{}</div>"#, description));
                }
                
                content.push_str(r#"</div>"#);
            }
            
            content.push_str(r#"</div></section>"#);
        }

        Ok(content)
    }

    /// Generate navigation HTML
    fn generate_navigation_html(&self, items: &[DocumentationItem]) -> DocResult<String> {
        let mut nav = String::new();
        
        nav.push_str(r#"<nav class="sidebar">
            <div class="sidebar-header">
                <h2>Documentation</h2>
            </div>
            <div class="nav-sections">"#);

        // Main links
        nav.push_str(r#"<div class="nav-section">
            <h3>Overview</h3>
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/packages/">Packages</a></li>
                <li><a href="/search.html">Search</a></li>
            </ul>
        </div>"#);

        // Group items by type
        let mut by_type: HashMap<String, Vec<&DocumentationItem>> = HashMap::new();
        for item in items {
            by_type.entry(item.item_type.to_string())
                .or_insert_with(Vec::new)
                .push(item);
        }

        for (type_name, type_items) in by_type {
            nav.push_str(&format!(r#"<div class="nav-section">
                <h3>{}</h3>
                <ul>"#, type_name));
            
            for item in type_items.iter().take(10) { // Limit for navigation
                let url = self.cross_refs.get(&item.name)
                    .unwrap_or(&format!("{}.html", self.sanitize_filename(&item.name)));
                nav.push_str(&format!(r#"<li><a href="{}">{}</a></li>"#, url, item.name));
            }
            
            if type_items.len() > 10 {
                nav.push_str(&format!(r#"<li><a href="#{}" class="see-more">... and {} more</a></li>"#, 
                    type_name.to_lowercase(), type_items.len() - 10));
            }
            
            nav.push_str(r#"</ul></div>"#);
        }

        nav.push_str(r#"</div></nav>"#);
        Ok(nav)
    }

    /// Register default templates
    fn register_default_templates(&mut self) {
        self.templates.insert("index".to_string(), DEFAULT_INDEX_TEMPLATE.to_string());
        self.templates.insert("package".to_string(), DEFAULT_PACKAGE_TEMPLATE.to_string());
        self.templates.insert("function".to_string(), DEFAULT_FUNCTION_TEMPLATE.to_string());
        self.templates.insert("type".to_string(), DEFAULT_TYPE_TEMPLATE.to_string());
        self.templates.insert("search".to_string(), DEFAULT_SEARCH_TEMPLATE.to_string());
        self.templates.insert("item".to_string(), DEFAULT_ITEM_TEMPLATE.to_string());
    }

    /// Get template by name
    fn get_template(&self, name: &str) -> DocResult<&String> {
        self.templates.get(name)
            .ok_or_else(|| DocError::TemplateError(format!("Template '{}' not found", name)))
    }

    /// Render template with variables
    fn render_template(&self, template: &str, vars: &[(&str, &str)]) -> DocResult<String> {
        let mut result = template.to_string();
        
        for (key, value) in vars {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        // Add common variables
        result = result.replace("{{styles}}", &self.generate_main_css()?);
        result = result.replace("{{scripts}}", &self.generate_main_js()?);
        result = result.replace("{{project_name}}", &self.config.project_name);

        Ok(result)
    }

    /// Generate main CSS
    fn generate_main_css(&self) -> DocResult<String> {
        let mut css = DEFAULT_CSS.to_string();
        
        if let Some(custom_css) = &self.config.custom_css {
            css.push_str("\n/* Custom CSS */\n");
            css.push_str(custom_css);
        }

        Ok(css)
    }

    /// Generate main JavaScript
    fn generate_main_js(&self) -> DocResult<String> {
        let mut js = DEFAULT_JS.to_string();
        
        if let Some(custom_js) = &self.config.custom_js {
            js.push_str("\n// Custom JavaScript\n");
            js.push_str(custom_js);
        }

        Ok(js)
    }

    /// Generate syntax highlighting CSS
    fn generate_syntax_highlight_css(&self) -> String {
        DEFAULT_HIGHLIGHT_CSS.to_string()
    }

    /// Generate syntax highlighting JavaScript
    fn generate_syntax_highlight_js(&self) -> String {
        DEFAULT_HIGHLIGHT_JS.to_string()
    }

    /// Generate favicon (simple base64 encoded icon)
    fn generate_favicon(&self) -> Vec<u8> {
        // Simple favicon data (you would replace with actual favicon)
        vec![0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x10, 0x10]
    }

    /// Write file and track it
    fn write_file(&mut self, path: &Path, content: &str) -> DocResult<()> {
        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| DocError::IoError(format!("Failed to create directory: {}", e)))?;
        }

        let content = if self.config.minify_output {
            self.minify_html(content)
        } else {
            content.to_string()
        };

        fs::write(path, content)
            .map_err(|e| DocError::IoError(format!("Failed to write {}: {}", path.display(), e)))?;
        
        self.generated_files.push(path.to_path_buf());
        Ok(())
    }

    /// Minify HTML content (basic implementation)
    fn minify_html(&self, html: &str) -> String {
        html.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Escape HTML content
    fn escape_html(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }

    /// Format description text
    fn format_description(&self, description: &str) -> DocResult<String> {
        // Simple markdown-like formatting
        let mut formatted = description.to_string();
        
        // Convert **bold** to <strong>
        formatted = formatted.replace("**", "<strong>");
        
        // Convert *italic* to <em>
        formatted = formatted.replace("*", "<em>");
        
        // Convert `code` to <code>
        formatted = formatted.replace("`", "<code>");
        
        // Convert line breaks
        formatted = formatted.replace('\n', "<br>");
        
        Ok(formatted)
    }

    /// Extract search keywords from item
    fn extract_search_keywords(&self, item: &DocumentationItem) -> Vec<String> {
        let mut keywords = Vec::new();
        
        keywords.push(item.name.clone());
        keywords.push(item.item_type.to_string());
        
        if let Some(description) = item.description() {
            keywords.extend(
                description.split_whitespace()
                    .filter(|word| word.len() > 3)
                    .map(|word| word.to_lowercase())
                    .collect::<Vec<_>>()
            );
        }

        for param in &item.parameters {
            keywords.push(param.name.clone());
            keywords.push(param.param_type.clone());
        }

        keywords.sort();
        keywords.dedup();
        keywords
    }

    /// Categorize item for organization
    fn categorize_item(&self, item: &DocumentationItem) -> String {
        match item.item_type {
            ItemType::Function => "Functions".to_string(),
            ItemType::Squad => "Structs".to_string(),
            ItemType::Collab => "Interfaces".to_string(),
            _ => "Other".to_string(),
        }
    }

    /// Calculate generation statistics
    fn calculate_stats(&self, items: &[DocumentationItem]) -> DocResult<GenerationStats> {
        let function_count = items.iter().filter(|i| i.item_type == ItemType::Function).count();
        let type_count = items.iter().filter(|i| matches!(i.item_type, ItemType::Squad | ItemType::Collab)).count();
        
        let total_size = self.generated_files.iter()
            .map(|path| fs::metadata(path).map(|m| m.len() as usize).unwrap_or(0))
            .sum();

        Ok(GenerationStats {
            function_count,
            type_count,
            package_count: 1, // Simplified for now
            total_size,
        })
    }

    /// Sanitize filename for web use
    fn sanitize_filename(&self, name: &str) -> String {
        name.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_html_generator_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = HtmlGeneratorConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        
        let generator = HtmlGenerator::new(config);
        assert!(!generator.templates.is_empty());
    }

    #[test]
    fn test_sanitize_filename() {
        let temp_dir = TempDir::new().unwrap();
        let config = HtmlGeneratorConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        let generator = HtmlGenerator::new(config);
        
        assert_eq!(generator.sanitize_filename("test-function"), "test-function");
        assert_eq!(generator.sanitize_filename("test.function"), "test_function");
        assert_eq!(generator.sanitize_filename("test function"), "test_function");
    }

    #[test]
    fn test_escape_html() {
        let temp_dir = TempDir::new().unwrap();
        let config = HtmlGeneratorConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        let generator = HtmlGenerator::new(config);
        
        assert_eq!(generator.escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(generator.escape_html("a & b"), "a &amp; b");
    }
}

// Template Constants
const DEFAULT_INDEX_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <link rel="stylesheet" href="assets/docs.css">
    <link rel="stylesheet" href="assets/highlight.css">
    <link rel="icon" href="favicon.ico" type="image/x-icon">
    <meta name="description" content="{{project_name}} documentation - comprehensive guide and API reference">
</head>
<body class="index-page">
    {{navigation}}
    
    <main class="main-content">
        {{content}}
        
        <footer class="footer">
            <div class="footer-content">
                <p>&copy; 2024 {{project_name}} Documentation. Generated with CURSED documentation system.</p>
                <div class="footer-links">
                    <a href="/">Home</a>
                    <a href="/search.html">Search</a>
                    <a href="/packages/">Packages</a>
                </div>
            </div>
        </footer>
    </main>
    
    <script src="assets/docs.js"></script>
    <script src="assets/highlight.js"></script>
    <script src="assets/navigation.js"></script>
</body>
</html>"#;

const DEFAULT_PACKAGE_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <link rel="stylesheet" href="../assets/docs.css">
    <link rel="stylesheet" href="../assets/highlight.css">
    <link rel="icon" href="../favicon.ico" type="image/x-icon">
    <meta name="description" content="{{package_name}} package documentation">
</head>
<body class="package-page">
    {{navigation}}
    
    <main class="main-content">
        {{breadcrumbs}}
        
        <article class="package-article">
            {{content}}
        </article>
        
        <footer class="footer">
            <div class="footer-content">
                <p>&copy; 2024 {{project_name}} Documentation.</p>
            </div>
        </footer>
    </main>
    
    <script src="../assets/docs.js"></script>
    <script src="../assets/highlight.js"></script>
    <script src="../assets/navigation.js"></script>
</body>
</html>"#;

const DEFAULT_FUNCTION_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <link rel="stylesheet" href="../assets/docs.css">
    <link rel="stylesheet" href="../assets/highlight.css">
    <link rel="icon" href="../favicon.ico" type="image/x-icon">
    <meta name="description" content="{{item_name}} function documentation">
</head>
<body class="function-page">
    {{navigation}}
    
    <main class="main-content">
        {{breadcrumbs}}
        
        <article class="function-article">
            {{content}}
        </article>
        
        <footer class="footer">
            <div class="footer-content">
                <p>&copy; 2024 {{project_name}} Documentation.</p>
            </div>
        </footer>
    </main>
    
    <script src="../assets/docs.js"></script>
    <script src="../assets/highlight.js"></script>
    <script src="../assets/navigation.js"></script>
</body>
</html>"#;

const DEFAULT_TYPE_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <link rel="stylesheet" href="../assets/docs.css">
    <link rel="stylesheet" href="../assets/highlight.css">
    <link rel="icon" href="../favicon.ico" type="image/x-icon">
    <meta name="description" content="{{item_name}} {{item_type}} documentation">
</head>
<body class="type-page">
    {{navigation}}
    
    <main class="main-content">
        {{breadcrumbs}}
        
        <article class="type-article">
            {{content}}
        </article>
        
        <footer class="footer">
            <div class="footer-content">
                <p>&copy; 2024 {{project_name}} Documentation.</p>
            </div>
        </footer>
    </main>
    
    <script src="../assets/docs.js"></script>
    <script src="../assets/highlight.js"></script>
    <script src="../assets/navigation.js"></script>
</body>
</html>"#;

const DEFAULT_SEARCH_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Search - {{project_name}} Documentation</title>
    <link rel="stylesheet" href="assets/docs.css">
    <link rel="stylesheet" href="assets/highlight.css">
    <link rel="icon" href="favicon.ico" type="image/x-icon">
    <meta name="description" content="Search {{project_name}} documentation">
</head>
<body class="search-page">
    {{navigation}}
    
    <main class="main-content">
        <div class="search-header">
            <h1>Search Documentation</h1>
            <p class="search-subtitle">Find functions, types, and other documentation</p>
        </div>
        
        <div class="search-container">
            <div class="search-input-container">
                <input type="text" id="search-input" placeholder="Search functions, types, examples..." autocomplete="off">
                <button id="search-button" aria-label="Search">🔍</button>
                <div class="search-suggestions" id="search-suggestions"></div>
            </div>
            
            <div class="search-filters">
                <label><input type="checkbox" id="filter-functions" checked> Functions</label>
                <label><input type="checkbox" id="filter-types" checked> Types</label>
                <label><input type="checkbox" id="filter-packages" checked> Packages</label>
            </div>
        </div>
        
        <div class="search-results-container">
            <div class="search-stats" id="search-stats"></div>
            <div class="search-results" id="search-results">
                <div class="search-placeholder">
                    <div class="placeholder-icon">🔍</div>
                    <h3>Start typing to search</h3>
                    <p>Search across all documentation including functions, types, and examples</p>
                </div>
            </div>
        </div>
        
        <footer class="footer">
            <div class="footer-content">
                <p>&copy; 2024 {{project_name}} Documentation.</p>
            </div>
        </footer>
    </main>
    
    <script src="assets/docs.js"></script>
    <script src="assets/search-index.js"></script>
</body>
</html>"#;

const DEFAULT_ITEM_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{title}}</title>
    <link rel="stylesheet" href="assets/docs.css">
    <link rel="stylesheet" href="assets/highlight.css">
    <link rel="icon" href="favicon.ico" type="image/x-icon">
    <meta name="description" content="{{item_name}} documentation">
</head>
<body class="item-page">
    {{navigation}}
    
    <main class="main-content">
        {{breadcrumbs}}
        
        <article class="item-article">
            {{content}}
        </article>
        
        <footer class="footer">
            <div class="footer-content">
                <p>&copy; 2024 {{project_name}} Documentation.</p>
            </div>
        </footer>
    </main>
    
    <script src="assets/docs.js"></script>
    <script src="assets/highlight.js"></script>
    <script src="assets/navigation.js"></script>
</body>
</html>"#;

// CSS and JavaScript constants would be very long, so we'll include simplified versions
const DEFAULT_CSS: &str = r#"/* CURSED Documentation Styles - Core */
:root {
    --primary-color: #2c3e50;
    --secondary-color: #3498db;
    --background-color: #f8f9fa;
    --surface-color: #ffffff;
    --text-primary: #2c3e50;
    --text-secondary: #7f8c8d;
    --border-color: #dee2e6;
    --sidebar-width: 280px;
}

* { margin: 0; padding: 0; box-sizing: border-box; }

body {
    font-family: 'Segoe UI', sans-serif;
    line-height: 1.6;
    color: var(--text-primary);
    background: var(--background-color);
    display: flex;
    min-height: 100vh;
}

.sidebar {
    width: var(--sidebar-width);
    background: var(--primary-color);
    color: white;
    padding: 1.5rem;
    overflow-y: auto;
    position: fixed;
    height: 100vh;
    top: 0;
    left: 0;
}

.main-content {
    margin-left: var(--sidebar-width);
    padding: 2rem;
    flex: 1;
    max-width: calc(100% - var(--sidebar-width));
}

.nav-section h3 {
    color: #bdc3c7;
    font-size: 0.875rem;
    text-transform: uppercase;
    margin-bottom: 0.5rem;
}

.nav-section ul { list-style: none; }
.nav-section li { margin-bottom: 0.25rem; }
.nav-section a {
    color: #95a5a6;
    display: block;
    padding: 0.375rem 0;
    text-decoration: none;
    transition: color 0.3s;
}

.nav-section a:hover { color: var(--secondary-color); }

.hero {
    text-align: center;
    margin-bottom: 3rem;
    padding: 3rem 0;
}

.hero h1 {
    font-size: 3rem;
    margin-bottom: 1rem;
    color: var(--primary-color);
}

.stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
    margin-bottom: 3rem;
}

.stat-card {
    background: var(--surface-color);
    padding: 2rem;
    border-radius: 0.75rem;
    text-align: center;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
    transition: transform 0.3s;
}

.stat-card:hover { transform: translateY(-2px); }

.stat-number {
    font-size: 2.5rem;
    font-weight: bold;
    color: var(--secondary-color);
    display: block;
}

.item-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
}

.item-preview {
    background: var(--surface-color);
    padding: 1.5rem;
    border-radius: 0.75rem;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
    transition: transform 0.3s;
}

.item-preview:hover { transform: translateY(-2px); }

pre {
    background: var(--primary-color);
    color: #ecf0f1;
    padding: 1.5rem;
    border-radius: 0.5rem;
    overflow-x: auto;
    position: relative;
}

code {
    font-family: 'SF Mono', Monaco, monospace;
    font-size: 0.875rem;
}

.copy-button {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    background: rgba(0,0,0,0.7);
    color: white;
    border: none;
    padding: 0.375rem 0.75rem;
    border-radius: 0.25rem;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.3s;
}

.code-block:hover .copy-button { opacity: 1; }

.search-input-container { position: relative; margin-bottom: 1.5rem; }

#search-input {
    width: 100%;
    padding: 1rem;
    border: 2px solid var(--border-color);
    border-radius: 0.75rem;
    font-size: 1.125rem;
}

#search-button {
    position: absolute;
    right: 0.5rem;
    top: 50%;
    transform: translateY(-50%);
    background: var(--secondary-color);
    color: white;
    border: none;
    padding: 0.5rem;
    border-radius: 0.5rem;
    cursor: pointer;
}

.footer {
    margin-top: auto;
    padding: 2rem 0;
    border-top: 1px solid var(--border-color);
    background: var(--surface-color);
}

@media (max-width: 768px) {
    body { flex-direction: column; }
    .sidebar { position: relative; width: 100%; height: auto; }
    .main-content { margin-left: 0; max-width: 100%; }
}
"#;

const DEFAULT_JS: &str = r#"// CURSED Documentation JavaScript - Core
document.addEventListener('DOMContentLoaded', function() {
    setupSearch();
    setupCodeBlocks();
    setupNavigation();
});

function setupSearch() {
    const searchInput = document.getElementById('search-input');
    const searchButton = document.getElementById('search-button');
    
    if (searchInput && searchButton) {
        searchButton.addEventListener('click', () => performSearch(searchInput.value));
        searchInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') performSearch(searchInput.value);
        });
    }
}

function performSearch(query) {
    const results = document.getElementById('search-results');
    if (results) {
        results.innerHTML = `<p>Searching for: ${query}</p>`;
    }
}

function setupCodeBlocks() {
    document.querySelectorAll('pre code').forEach(block => {
        const container = block.parentElement;
        container.style.position = 'relative';
        
        const copyButton = document.createElement('button');
        copyButton.className = 'copy-button';
        copyButton.textContent = 'Copy';
        copyButton.onclick = () => copyCode(block);
        
        container.appendChild(copyButton);
    });
}

function copyCode(codeBlock) {
    navigator.clipboard.writeText(codeBlock.textContent).then(() => {
        console.log('Code copied!');
    });
}

function setupNavigation() {
    document.querySelectorAll('a[href^="#"]').forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const target = document.querySelector(link.getAttribute('href'));
            if (target) target.scrollIntoView({ behavior: 'smooth' });
        });
    });
}
"#;

const DEFAULT_HIGHLIGHT_CSS: &str = r#"/* CURSED Syntax Highlighting */
.keyword-function { color: #e74c3c; font-weight: bold; }
.keyword-variable { color: #f39c12; font-weight: bold; }
.keyword-conditional { color: #9b59b6; font-weight: bold; }
.keyword-control { color: #e67e22; font-weight: bold; }
.keyword-type { color: #27ae60; font-weight: bold; }
.type { color: #3498db; font-weight: 600; }
.string { color: #2ecc71; }
.comment { color: #95a5a6; font-style: italic; }
.number { color: #f1c40f; }
"#;

const DEFAULT_HIGHLIGHT_JS: &str = r#"// CURSED Syntax Highlighting
document.addEventListener('DOMContentLoaded', function() {
    document.querySelectorAll('code.language-cursed').forEach(highlightCursedCode);
});

function highlightCursedCode(block) {
    let content = block.innerHTML;
    
    // Keywords
    content = content.replace(/\b(slay|yolo)\b/g, '<span class="keyword-function">$1</span>');
    content = content.replace(/\b(sus|facts)\b/g, '<span class="keyword-variable">$1</span>');
    content = content.replace(/\b(lowkey|highkey)\b/g, '<span class="keyword-conditional">$1</span>');
    content = content.replace(/\b(periodt|bestie|flex)\b/g, '<span class="keyword-control">$1</span>');
    content = content.replace(/\b(squad|collab)\b/g, '<span class="keyword-type">$1</span>');
    
    // Types
    content = content.replace(/\b(normie|str|bool|void)\b/g, '<span class="type">$1</span>');
    
    // Strings
    content = content.replace(/"([^"\\]|\\.)*"/g, '<span class="string">"$1"</span>');
    
    // Comments
    content = content.replace(/\/\/.*$/gm, '<span class="comment">$&</span>');
    
    // Numbers
    content = content.replace(/\b\d+(\.\d+)?\b/g, '<span class="number">$&</span>');
    
    block.innerHTML = content;
}
"#;
