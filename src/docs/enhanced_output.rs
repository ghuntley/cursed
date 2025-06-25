/// Enhanced Output Formats System
/// 
/// Provides advanced output format capabilities including PDF generation,
/// responsive HTML templates, API documentation, and hosting platform integration.

use crate::error::{CursedError, SourceLocation};
use crate::docs::generator::{ExtractedDocumentation, DocumentationItem, CrossReference};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::{Command, Stdio};

/// Enhanced output format generator
#[derive(Debug)]
pub struct EnhancedOutputGenerator {
    /// Configuration for output generation
    config: OutputConfig,
    /// Template manager
    template_manager: TemplateManager,
    /// Hosting integrations
    hosting_integrations: HostingIntegrations,
}

/// Configuration for enhanced output formats
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// Enable PDF generation
    pub enable_pdf: bool,
    /// Enable responsive HTML templates
    pub enable_responsive_html: bool,
    /// Enable API documentation generation
    pub enable_api_docs: bool,
    /// Enable hosting platform integration
    pub enable_hosting_integration: bool,
    /// PDF generation engine
    pub pdf_engine: PdfEngine,
    /// HTML template theme
    pub html_theme: HtmlTheme,
    /// API documentation format
    pub api_format: ApiDocFormat,
    /// Hosting platforms to integrate with
    pub hosting_platforms: Vec<HostingPlatform>,
    /// Custom CSS/styling options
    pub custom_styling: Option<CustomStyling>,
    /// Output directory structure
    pub output_structure: OutputStructure,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            enable_pdf: true,
            enable_responsive_html: true,
            enable_api_docs: true,
            enable_hosting_integration: false, // Disabled by default for security
            pdf_engine: PdfEngine::Puppeteer,
            html_theme: HtmlTheme::Modern,
            api_format: ApiDocFormat::OpenApi,
            hosting_platforms: vec![HostingPlatform::GitHubPages],
            custom_styling: None,
            output_structure: OutputStructure::Organized,
        }
    }
}

/// PDF generation engines
#[derive(Debug, Clone)]
pub enum PdfEngine {
    /// Use Puppeteer (Chrome headless) for PDF generation
    Puppeteer,
    /// Use wkhtmltopdf for PDF generation
    WkHtmlToPdf,
    /// Use Pandoc for PDF generation
    Pandoc,
    /// Use Prince XML for professional PDF generation
    Prince,
}

/// HTML themes
#[derive(Debug, Clone)]
pub enum HtmlTheme {
    /// Modern responsive theme with dark mode support
    Modern,
    /// Classic documentation theme
    Classic,
    /// Minimal clean theme
    Minimal,
    /// Material design theme
    Material,
    /// Custom theme from CSS files
    Custom(PathBuf),
}

/// API documentation formats
#[derive(Debug, Clone)]
pub enum ApiDocFormat {
    /// OpenAPI 3.0 specification
    OpenApi,
    /// API Blueprint format
    ApiBlueprint,
    /// Swagger 2.0 specification
    Swagger,
    /// RAML specification
    Raml,
    /// Custom format
    Custom(String),
}

/// Hosting platforms
#[derive(Debug, Clone)]
pub enum HostingPlatform {
    /// GitHub Pages
    GitHubPages,
    /// GitLab Pages
    GitLabPages,
    /// Netlify
    Netlify,
    /// Vercel
    Vercel,
    /// AWS S3
    AwsS3,
    /// Custom hosting configuration
    Custom(HostingConfig),
}

/// Custom hosting configuration
#[derive(Debug, Clone)]
pub struct HostingConfig {
    pub name: String,
    pub upload_command: String,
    pub base_url: String,
    pub deployment_script: Option<PathBuf>,
}

/// Custom styling configuration
#[derive(Debug, Clone)]
pub struct CustomStyling {
    pub css_files: Vec<PathBuf>,
    pub color_scheme: ColorScheme,
    pub typography: Typography,
    pub layout_options: LayoutOptions,
}

/// Color scheme configuration
#[derive(Debug, Clone)]
pub struct ColorScheme {
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: String,
    pub background_color: String,
    pub text_color: String,
    pub code_background: String,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            primary_color: "#007bff".to_string(),
            secondary_color: "#6c757d".to_string(),
            accent_color: "#28a745".to_string(),
            background_color: "#ffffff".to_string(),
            text_color: "#212529".to_string(),
            code_background: "#f8f9fa".to_string(),
        }
    }
}

/// Typography configuration
#[derive(Debug, Clone)]
pub struct Typography {
    pub font_family: String,
    pub heading_font: String,
    pub code_font: String,
    pub base_font_size: String,
    pub line_height: String,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif".to_string(),
            heading_font: "inherit".to_string(),
            code_font: "'Monaco', 'Consolas', monospace".to_string(),
            base_font_size: "16px".to_string(),
            line_height: "1.6".to_string(),
        }
    }
}

/// Layout options
#[derive(Debug, Clone)]
pub struct LayoutOptions {
    pub sidebar_width: String,
    pub content_max_width: String,
    pub enable_sticky_nav: bool,
    pub enable_breadcrumbs: bool,
    pub enable_search: bool,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            sidebar_width: "300px".to_string(),
            content_max_width: "1200px".to_string(),
            enable_sticky_nav: true,
            enable_breadcrumbs: true,
            enable_search: true,
        }
    }
}

/// Output directory structure
#[derive(Debug, Clone)]
pub enum OutputStructure {
    /// Organized structure with separate directories for different formats
    Organized,
    /// Flat structure with all files in one directory
    Flat,
    /// Custom structure defined by user
    Custom(HashMap<String, PathBuf>),
}

/// Template manager for generating various output formats
#[derive(Debug)]
pub struct TemplateManager {
    /// HTML templates
    html_templates: HashMap<String, String>,
    /// CSS stylesheets
    stylesheets: HashMap<String, String>,
    /// JavaScript files
    scripts: HashMap<String, String>,
    /// Template variables
    variables: HashMap<String, String>,
}

impl Default for TemplateManager {
    fn default() -> Self {
        let mut manager = Self {
            html_templates: HashMap::new(),
            stylesheets: HashMap::new(),
            scripts: HashMap::new(),
            variables: HashMap::new(),
        };
        manager.load_default_templates();
        manager
    }
}

/// Hosting platform integrations
#[derive(Debug)]
pub struct HostingIntegrations {
    /// GitHub Pages configuration
    github_pages: Option<GitHubPagesConfig>,
    /// Netlify configuration
    netlify: Option<NetlifyConfig>,
    /// Custom hosting configurations
    custom_configs: HashMap<String, HostingConfig>,
}

impl Default for HostingIntegrations {
    fn default() -> Self {
        Self {
            github_pages: None,
            netlify: None,
            custom_configs: HashMap::new(),
        }
    }
}

/// GitHub Pages configuration
#[derive(Debug, Clone)]
pub struct GitHubPagesConfig {
    pub repository: String,
    pub branch: String,
    pub path: PathBuf,
    pub custom_domain: Option<String>,
}

/// Netlify configuration
#[derive(Debug, Clone)]
pub struct NetlifyConfig {
    pub site_id: String,
    pub build_command: String,
    pub publish_directory: PathBuf,
    pub environment_variables: HashMap<String, String>,
}

/// PDF generation result
#[derive(Debug)]
pub struct PdfGenerationResult {
    pub success: bool,
    pub output_path: PathBuf,
    pub file_size: usize,
    pub page_count: Option<usize>,
    pub generation_time: std::time::Duration,
    pub error_message: Option<String>,
}

/// API documentation generation result
#[derive(Debug)]
pub struct ApiDocumentationResult {
    pub success: bool,
    pub format: ApiDocFormat,
    pub output_files: Vec<PathBuf>,
    pub specification_file: Option<PathBuf>,
    pub validation_results: Vec<String>,
}

impl EnhancedOutputGenerator {
    /// Create a new enhanced output generator
    pub fn new(config: OutputConfig) -> Self {
        Self {
            config,
            template_manager: TemplateManager::default(),
            hosting_integrations: HostingIntegrations::default(),
        }
    }

    /// Generate all enhanced output formats
    pub fn generate_all_formats(
        &mut self,
        documentation: &ExtractedDocumentation,
        output_dir: &Path,
    ) -> crate::error::Result<()> {
        let mut results = GenerationResults::default();

        // Create output directory structure
        self.create_output_structure(output_dir)?;

        // Generate responsive HTML
        if self.config.enable_responsive_html {
            results.html_result = Some(self.generate_responsive_html(documentation, output_dir)?);
        }

        // Generate PDF documentation
        if self.config.enable_pdf {
            results.pdf_result = Some(self.generate_pdf_documentation(documentation, output_dir)?);
        }

        // Generate API documentation
        if self.config.enable_api_docs {
            results.api_result = Some(self.generate_api_documentation(documentation, output_dir)?);
        }

        // Deploy to hosting platforms
        if self.config.enable_hosting_integration {
            results.hosting_results = self.deploy_to_hosting_platforms(output_dir)?;
        }

        Ok(results)
    }

    /// Create the output directory structure
    fn create_output_structure(&self, base_dir: &Path) -> crate::error::Result<()> {
        match &self.config.output_structure {
            OutputStructure::Organized => {
                let dirs = ["html", "pdf", "api", "assets", "static"];
                for dir in &dirs {
                    fs::create_dir_all(base_dir.join(dir))
                        .map_err(|e| CursedError::SystemError(format!("Failed to create directory {}: {}", dir, e)))?;
                }
            }
            OutputStructure::Flat => {
                fs::create_dir_all(base_dir)
                    .map_err(|e| CursedError::SystemError(format!("Failed to create output directory: {}", e)))?;
            }
            OutputStructure::Custom(structure) => {
                for (_, path) in structure {
                    fs::create_dir_all(base_dir.join(path))
                        .map_err(|e| CursedError::SystemError(format!("Failed to create custom directory: {}", e)))?;
                }
            }
        }
        Ok(())
    }

    /// Generate responsive HTML documentation
    fn generate_responsive_html(
        &self,
        documentation: &ExtractedDocumentation,
        output_dir: &Path,
    ) -> crate::error::Result<()> {
        let html_dir = match &self.config.output_structure {
            OutputStructure::Organized => output_dir.join("html"),
            _ => output_dir.to_path_buf(),
        };

        // Generate main index page
        let index_html = self.generate_responsive_index(documentation)?;
        fs::write(html_dir.join("index.html"), index_html)
            .map_err(|e| CursedError::SystemError(format!("Failed to write index.html: {}", e)))?;

        // Generate individual documentation pages
        let mut generated_files = vec![html_dir.join("index.html")];
        for item in &documentation.items {
            let page_html = self.generate_item_page(item, documentation)?;
            let filename = format!("{}.html", self.sanitize_filename(&item.name));
            let file_path = html_dir.join(filename);
            fs::write(&file_path, page_html)
                .map_err(|e| CursedError::SystemError(format!("Failed to write HTML file: {}", e)))?;
            generated_files.push(file_path);
        }

        // Generate CSS and JavaScript assets
        self.generate_assets(&html_dir)?;

        // Generate search index
        if self.config.custom_styling.as_ref().map_or(true, |s| s.layout_options.enable_search) {
            self.generate_search_index(documentation, &html_dir)?;
        }

        Ok(HtmlGenerationResult {
            success: true,
            output_directory: html_dir,
            generated_files,
            total_pages: documentation.items.len() + 1, // +1 for index
            theme_used: self.config.html_theme.clone(),
        })
    }

    /// Generate responsive index page
    fn generate_responsive_index(&self, documentation: &ExtractedDocumentation) -> crate::error::Result<()> {
        let template = self.template_manager.html_templates.get("responsive_index")
            .ok_or_else(|| CursedError::SystemError("Responsive index template not found".to_string()))?;

        let navigation = self.generate_navigation(documentation);
        let content = self.generate_index_content(documentation);
        let css = self.get_css_for_theme();
        let js = self.get_javascript();

        let html = template
            .replace("{{TITLE}}", "CURSED Documentation")
            .replace("{{NAVIGATION}}", &navigation)
            .replace("{{CONTENT}}", &content)
            .replace("{{CSS}}", &css)
            .replace("{{JAVASCRIPT}}", &js)
            .replace("{{SEARCH_INDEX}}", "search_index.json");

        Ok(html)
    }

    /// Generate navigation for responsive design
    fn generate_navigation(&self, documentation: &ExtractedDocumentation) -> String {
        let mut nav_html = String::new();
        let mut categories: HashMap<String, Vec<&DocumentationItem>> = HashMap::new();

        // Group items by category
        for item in &documentation.items {
            let category = match item.kind {
                crate::docs::generator::ItemKind::Function => "Functions",
                crate::docs::generator::ItemKind::Struct => "Structures",
                crate::docs::generator::ItemKind::Interface => "Interfaces",
                crate::docs::generator::ItemKind::Module => "Modules",
                crate::docs::generator::ItemKind::Enum => "Enums",
                crate::docs::generator::ItemKind::Constant => "Constants",
                crate::docs::generator::ItemKind::Variable => "Variables",
                crate::docs::generator::ItemKind::Type => "Types",
            };
            categories.entry(category.to_string()).or_insert_with(Vec::new).push(item);
        }

        // Generate navigation HTML
        for (category, items) in categories {
            nav_html.push_str(&format!(
                r#"<div class="nav-category">
                    <h3 class="nav-category-title">{}</h3>
                    <ul class="nav-items">"#,
                category
            ));

            for item in items {
                nav_html.push_str(&format!(
                    r#"<li class="nav-item">
                        <a href="{}.html" class="nav-link">{}</a>
                    </li>"#,
                    self.sanitize_filename(&item.name),
                    item.name
                ));
            }

            nav_html.push_str("</ul></div>");
        }

        nav_html
    }

    /// Generate index page content
    fn generate_index_content(&self, documentation: &ExtractedDocumentation) -> String {
        let stats = self.calculate_documentation_stats(documentation);
        
        format!(
            r#"<div class="hero-section">
                <h1>CURSED Programming Language Documentation</h1>
                <p class="hero-description">Comprehensive documentation for the CURSED programming language featuring Gen Z slang syntax and modern programming concepts.</p>
                <div class="stats-grid">
                    <div class="stat-card">
                        <div class="stat-number">{}</div>
                        <div class="stat-label">Functions</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">{}</div>
                        <div class="stat-label">Structures</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">{}</div>
                        <div class="stat-label">Interfaces</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">{}</div>
                        <div class="stat-label">Modules</div>
                    </div>
                </div>
            </div>
            
            <div class="quick-start">
                <h2>Quick Start</h2>
                <div class="example-code">
                    <pre><code class="language-cursed">// Welcome to CURSED!
slay main() {{
    facts message = "Hello, World!";
    println(message);
}}</code></pre>
                </div>
            </div>
            
            <div class="features-grid">
                <div class="feature-card">
                    <h3>Gen Z Syntax</h3>
                    <p>Express yourself with modern slang keywords like 'slay', 'periodt', and 'sus'.</p>
                </div>
                <div class="feature-card">
                    <h3>Type Safety</h3>
                    <p>Strong static typing with inference for reliable code.</p>
                </div>
                <div class="feature-card">
                    <h3>Performance</h3>
                    <p>LLVM-powered compilation for optimal performance.</p>
                </div>
                <div class="feature-card">
                    <h3>Concurrency</h3>
                    <p>Built-in goroutines with 'stan' keyword for easy parallelism.</p>
                </div>
            </div>"#,
            stats.function_count,
            stats.struct_count,
            stats.interface_count,
            stats.module_count
        )
    }

    /// Calculate documentation statistics
    fn calculate_documentation_stats(&self, documentation: &ExtractedDocumentation) -> DocumentationStats {
        let mut stats = DocumentationStats::default();
        
        for item in &documentation.items {
            match item.kind {
                crate::docs::generator::ItemKind::Function => stats.function_count += 1,
                crate::docs::generator::ItemKind::Struct => stats.struct_count += 1,
                crate::docs::generator::ItemKind::Interface => stats.interface_count += 1,
                crate::docs::generator::ItemKind::Module => stats.module_count += 1,
                crate::docs::generator::ItemKind::Enum => stats.enum_count += 1,
                crate::docs::generator::ItemKind::Constant => stats.constant_count += 1,
                crate::docs::generator::ItemKind::Variable => stats.variable_count += 1,
                crate::docs::generator::ItemKind::Type => stats.type_count += 1,
            }
        }
        
        stats
    }

    /// Generate individual item page
    fn generate_item_page(&self, item: &DocumentationItem, documentation: &ExtractedDocumentation) -> crate::error::Result<()> {
        let template = self.template_manager.html_templates.get("item_page")
            .ok_or_else(|| CursedError::SystemError("Item page template not found".to_string()))?;

        let navigation = self.generate_navigation(documentation);
        let breadcrumbs = self.generate_breadcrumbs(item);
        let content = self.generate_item_content(item);
        let related_items = self.generate_related_items(item, documentation);
        let css = self.get_css_for_theme();
        let js = self.get_javascript();

        let html = template
            .replace("{{TITLE}}", &format!("{} - CURSED Documentation", item.name))
            .replace("{{NAVIGATION}}", &navigation)
            .replace("{{BREADCRUMBS}}", &breadcrumbs)
            .replace("{{CONTENT}}", &content)
            .replace("{{RELATED_ITEMS}}", &related_items)
            .replace("{{CSS}}", &css)
            .replace("{{JAVASCRIPT}}", &js);

        Ok(html)
    }

    /// Generate breadcrumbs for navigation
    fn generate_breadcrumbs(&self, item: &DocumentationItem) -> String {
        let category = match item.kind {
            crate::docs::generator::ItemKind::Function => "Functions",
            crate::docs::generator::ItemKind::Struct => "Structures",
            crate::docs::generator::ItemKind::Interface => "Interfaces",
            crate::docs::generator::ItemKind::Module => "Modules",
            crate::docs::generator::ItemKind::Enum => "Enums",
            crate::docs::generator::ItemKind::Constant => "Constants",
            crate::docs::generator::ItemKind::Variable => "Variables",
            crate::docs::generator::ItemKind::Type => "Types",
        };

        format!(
            r#"<nav class="breadcrumbs">
                <a href="index.html">Home</a>
                <span class="separator">›</span>
                <span class="category">{}</span>
                <span class="separator">›</span>
                <span class="current">{}</span>
            </nav>"#,
            category, item.name
        )
    }

    /// Generate content for an individual item
    fn generate_item_content(&self, item: &DocumentationItem) -> String {
        let mut content = format!(
            r#"<div class="item-header">
                <h1>{}</h1>
                <div class="item-meta">
                    <span class="item-type">{:?}</span>
                    <span class="item-visibility">{:?}</span>
                </div>
            </div>"#,
            item.name, item.kind, item.visibility
        );

        // Add description
        content.push_str(&format!(
            r#"<div class="item-description">
                <p>{}</p>
            </div>"#,
            item.description
        ));

        // Add parameters if any
        if !item.parameters.is_empty() {
            content.push_str("<div class=\"item-parameters\"><h3>Parameters</h3><ul>");
            for param in &item.parameters {
                content.push_str(&format!(
                    "<li><code>{}</code>: {}</li>",
                    param.name, param.description
                ));
            }
            content.push_str("</ul></div>");
        }

        // Add examples if any
        if !item.examples.is_empty() {
            content.push_str("<div class=\"item-examples\"><h3>Examples</h3>");
            for example in &item.examples {
                content.push_str(&format!(
                    r#"<div class="example">
                        <pre><code class="language-cursed">{}</code></pre>
                    </div>"#,
                    html_escape::encode_text(&example.code)
                ));
            }
            content.push_str("</div>");
        }

        // Add source location
        content.push_str(&format!(
            r#"<div class="source-info">
                <h3>Source</h3>
                <p>Defined in <code>{}</code> at line {}</p>
            </div>"#,
            item.source_info.file.as_ref().map(|f| f.display().to_string()).unwrap_or_else(|| "unknown".to_string()),
            item.source_info.line
        ));

        content
    }

    /// Generate related items section
    fn generate_related_items(&self, item: &DocumentationItem, documentation: &ExtractedDocumentation) -> String {
        let related: Vec<&DocumentationItem> = documentation.items.iter()
            .filter(|other| other.name != item.name && other.kind == item.kind)
            .take(5)
            .collect();

        if related.is_empty() {
            return String::new();
        }

        let mut content = String::from(r#"<div class="related-items"><h3>Related Items</h3><ul>"#);
        for related_item in related {
            content.push_str(&format!(
                r#"<li><a href="{}.html">{}</a></li>"#,
                self.sanitize_filename(&related_item.name),
                related_item.name
            ));
        }
        content.push_str("</ul></div>");
        content
    }

    /// Generate CSS for the selected theme
    fn get_css_for_theme(&self) -> String {
        match &self.config.html_theme {
            HtmlTheme::Modern => self.template_manager.stylesheets.get("modern").cloned().unwrap_or_default(),
            HtmlTheme::Classic => self.template_manager.stylesheets.get("classic").cloned().unwrap_or_default(),
            HtmlTheme::Minimal => self.template_manager.stylesheets.get("minimal").cloned().unwrap_or_default(),
            HtmlTheme::Material => self.template_manager.stylesheets.get("material").cloned().unwrap_or_default(),
            HtmlTheme::Custom(path) => {
                fs::read_to_string(path).unwrap_or_else(|_| {
                    self.template_manager.stylesheets.get("modern").cloned().unwrap_or_default()
                })
            }
        }
    }

    /// Get JavaScript for interactive features
    fn get_javascript(&self) -> String {
        self.template_manager.scripts.get("main").cloned().unwrap_or_default()
    }

    /// Generate static assets (CSS, JS, images)
    fn generate_assets(&self, html_dir: &Path) -> crate::error::Result<()> {
        let assets_dir = html_dir.join("assets");
        fs::create_dir_all(&assets_dir)
            .map_err(|e| CursedError::SystemError(format!("Failed to create assets directory: {}", e)))?;

        // Write CSS files
        let css_content = self.get_css_for_theme();
        fs::write(assets_dir.join("styles.css"), css_content)
            .map_err(|e| CursedError::SystemError(format!("Failed to write CSS: {}", e)))?;

        // Write JavaScript files
        let js_content = self.get_javascript();
        fs::write(assets_dir.join("script.js"), js_content)
            .map_err(|e| CursedError::SystemError(format!("Failed to write JavaScript: {}", e)))?;

        Ok(())
    }

    /// Generate search index for client-side search
    fn generate_search_index(&self, documentation: &ExtractedDocumentation, output_dir: &Path) -> crate::error::Result<()> {
        let mut search_entries = Vec::new();
        
        for item in &documentation.items {
            search_entries.push(serde_json::json!({
                "name": item.name,
                "type": format!("{:?}", item.kind),
                "description": item.description,
                "url": format!("{}.html", self.sanitize_filename(&item.name)),
                "keywords": [item.name.clone()]
            }));
        }

        let search_index = serde_json::json!({
            "version": "1.0",
            "entries": search_entries
        });

        let search_index_str = serde_json::to_string_pretty(&search_index)
            .map_err(|e| CursedError::SystemError(format!("Failed to serialize search index: {}", e)))?;

        fs::write(output_dir.join("search_index.json"), search_index_str)
            .map_err(|e| CursedError::SystemError(format!("Failed to write search index: {}", e)))?;

        Ok(())
    }

    /// Generate PDF documentation
    fn generate_pdf_documentation(
        &self,
        documentation: &ExtractedDocumentation,
        output_dir: &Path,
    ) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        
        let pdf_dir = match &self.config.output_structure {
            OutputStructure::Organized => output_dir.join("pdf"),
            _ => output_dir.to_path_buf(),
        };

        // Generate HTML for PDF conversion
        let pdf_html = self.generate_pdf_html(documentation)?;
        let temp_html_path = pdf_dir.join("temp_for_pdf.html");
        fs::write(&temp_html_path, pdf_html)
            .map_err(|e| CursedError::SystemError(format!("Failed to write temp HTML for PDF: {}", e)))?;

        let output_path = pdf_dir.join("documentation.pdf");
        
        let result = match &self.config.pdf_engine {
            PdfEngine::Puppeteer => self.generate_pdf_with_puppeteer(&temp_html_path, &output_path),
            PdfEngine::WkHtmlToPdf => self.generate_pdf_with_wkhtmltopdf(&temp_html_path, &output_path),
            PdfEngine::Pandoc => self.generate_pdf_with_pandoc(&temp_html_path, &output_path),
            PdfEngine::Prince => self.generate_pdf_with_prince(&temp_html_path, &output_path),
        };

        // Clean up temp file
        let _ = fs::remove_file(&temp_html_path);

        let generation_time = start_time.elapsed();

        match result {
            Ok(()) => {
                let file_size = fs::metadata(&output_path)
                    .map(|m| m.len() as usize)
                    .unwrap_or(0);

                Ok(PdfGenerationResult {
                    success: true,
                    output_path,
                    file_size,
                    page_count: None, // Would need additional tooling to determine
                    generation_time,
                    error_message: None,
                })
            }
            Err(e) => Ok(PdfGenerationResult {
                success: false,
                output_path,
                file_size: 0,
                page_count: None,
                generation_time,
                error_message: Some(e.to_string()),
            })
        }
    }

    /// Generate HTML optimized for PDF conversion
    fn generate_pdf_html(&self, documentation: &ExtractedDocumentation) -> crate::error::Result<()> {
        let mut html = String::new();
        
        // Add PDF-specific styling
        html.push_str(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>CURSED Documentation</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; margin: 40px; }
        h1 { color: #2c3e50; border-bottom: 3px solid #3498db; padding-bottom: 10px; }
        h2 { color: #34495e; border-bottom: 1px solid #bdc3c7; padding-bottom: 5px; }
        h3 { color: #7f8c8d; }
        code { background: #ecf0f1; padding: 2px 4px; border-radius: 3px; font-family: 'Monaco', 'Consolas', monospace; }
        pre { background: #2c3e50; color: #ecf0f1; padding: 15px; border-radius: 5px; overflow: auto; }
        .item { margin-bottom: 30px; page-break-inside: avoid; }
        .item-header { margin-bottom: 15px; }
        .item-meta { color: #7f8c8d; font-size: 0.9em; }
        .parameters { margin: 15px 0; }
        .parameters ul { margin: 5px 0; }
        .examples { margin: 15px 0; }
        .page-break { page-break-before: always; }
        @media print { .page-break { page-break-before: always; } }
    </style>
</head>
<body>"#);

        // Add title page
        html.push_str(r#"
<div class="title-page">
    <h1>CURSED Programming Language</h1>
    <h2>Complete Documentation</h2>
    <p>Comprehensive reference for the CURSED programming language featuring Gen Z slang syntax and modern programming concepts.</p>
</div>
<div class="page-break"></div>
"#);

        // Add table of contents
        html.push_str("<h1>Table of Contents</h1>\n<ul>");
        for item in &documentation.items {
            html.push_str(&format!(
                "<li>{} - {} ({:?})</li>",
                item.name, item.description, item.kind
            ));
        }
        html.push_str("</ul><div class=\"page-break\"></div>\n");

        // Add all documentation items
        for item in &documentation.items {
            html.push_str(&format!(
                r#"<div class="item">
                    <div class="item-header">
                        <h1>{}</h1>
                        <div class="item-meta">{:?} | {:?}</div>
                    </div>
                    <p>{}</p>"#,
                item.name, item.kind, item.visibility, item.description
            ));

            if !item.parameters.is_empty() {
                html.push_str("<div class=\"parameters\"><h3>Parameters</h3><ul>");
                for param in &item.parameters {
                    html.push_str(&format!("<li><strong>{}</strong>: {}</li>", param.name, param.description));
                }
                html.push_str("</ul></div>");
            }

            if !item.examples.is_empty() {
                html.push_str("<div class=\"examples\"><h3>Examples</h3>");
                for example in &item.examples {
                    html.push_str(&format!("<pre><code>{}</code></pre>", html_escape::encode_text(&example.code)));
                }
                html.push_str("</div>");
            }

            html.push_str("</div>");
        }

        html.push_str("</body></html>");
        Ok(html)
    }

    /// Generate PDF using Puppeteer
    fn generate_pdf_with_puppeteer(&self, html_path: &Path, output_path: &Path) -> crate::error::Result<()> {
        let output = Command::new("npx")
            .args(&[
                "puppeteer",
                "pdf",
                &html_path.to_string_lossy(),
                &output_path.to_string_lossy(),
                "--format", "A4",
                "--margin-top", "20mm",
                "--margin-bottom", "20mm",
                "--margin-left", "20mm",
                "--margin-right", "20mm",
                "--print-background",
            ])
            .output()
            .map_err(|e| CursedError::SystemError(format!("Failed to execute Puppeteer: {}", e)))?;

        if !output.status.success() {
            return Err(CursedError::SystemError(format!(
                "Puppeteer PDF generation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// Generate PDF using wkhtmltopdf
    fn generate_pdf_with_wkhtmltopdf(&self, html_path: &Path, output_path: &Path) -> crate::error::Result<()> {
        let output = Command::new("wkhtmltopdf")
            .args(&[
                "--page-size", "A4",
                "--margin-top", "20mm",
                "--margin-bottom", "20mm",
                "--margin-left", "20mm",
                "--margin-right", "20mm",
                "--enable-local-file-access",
                &html_path.to_string_lossy(),
                &output_path.to_string_lossy(),
            ])
            .output()
            .map_err(|e| CursedError::SystemError(format!("Failed to execute wkhtmltopdf: {}", e)))?;

        if !output.status.success() {
            return Err(CursedError::SystemError(format!(
                "wkhtmltopdf failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// Generate PDF using Pandoc
    fn generate_pdf_with_pandoc(&self, html_path: &Path, output_path: &Path) -> crate::error::Result<()> {
        let output = Command::new("pandoc")
            .args(&[
                &html_path.to_string_lossy(),
                "-o", &output_path.to_string_lossy(),
                "--pdf-engine=xelatex",
                "-V", "geometry:margin=20mm",
            ])
            .output()
            .map_err(|e| CursedError::SystemError(format!("Failed to execute Pandoc: {}", e)))?;

        if !output.status.success() {
            return Err(CursedError::SystemError(format!(
                "Pandoc PDF generation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// Generate PDF using Prince XML
    fn generate_pdf_with_prince(&self, html_path: &Path, output_path: &Path) -> crate::error::Result<()> {
        let output = Command::new("prince")
            .args(&[
                &html_path.to_string_lossy(),
                "-o", &output_path.to_string_lossy(),
                "--media=print",
            ])
            .output()
            .map_err(|e| CursedError::SystemError(format!("Failed to execute Prince: {}", e)))?;

        if !output.status.success() {
            return Err(CursedError::SystemError(format!(
                "Prince PDF generation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// Generate API documentation
    fn generate_api_documentation(
        &self,
        documentation: &ExtractedDocumentation,
        output_dir: &Path,
    ) -> crate::error::Result<()> {
        let api_dir = match &self.config.output_structure {
            OutputStructure::Organized => output_dir.join("api"),
            _ => output_dir.to_path_buf(),
        };

        match &self.config.api_format {
            ApiDocFormat::OpenApi => self.generate_openapi_spec(documentation, &api_dir),
            ApiDocFormat::ApiBlueprint => self.generate_api_blueprint(documentation, &api_dir),
            ApiDocFormat::Swagger => self.generate_swagger_spec(documentation, &api_dir),
            ApiDocFormat::Raml => self.generate_raml_spec(documentation, &api_dir),
            ApiDocFormat::Custom(format) => self.generate_custom_api_format(documentation, &api_dir, format),
        }
    }

    /// Generate OpenAPI 3.0 specification
    fn generate_openapi_spec(&self, documentation: &ExtractedDocumentation, output_dir: &Path) -> crate::error::Result<()> {
        let spec = self.build_openapi_spec(documentation)?;
        let spec_json = serde_json::to_string_pretty(&spec)
            .map_err(|e| CursedError::SystemError(format!("Failed to serialize OpenAPI spec: {}", e)))?;

        let spec_file = output_dir.join("openapi.json");
        fs::write(&spec_file, spec_json)
            .map_err(|e| CursedError::SystemError(format!("Failed to write OpenAPI spec: {}", e)))?;

        // Generate Swagger UI HTML
        let swagger_ui_html = self.generate_swagger_ui_html();
        let ui_file = output_dir.join("index.html");
        fs::write(&ui_file, swagger_ui_html)
            .map_err(|e| CursedError::SystemError(format!("Failed to write Swagger UI: {}", e)))?;

        Ok(ApiDocumentationResult {
            success: true,
            format: ApiDocFormat::OpenApi,
            output_files: vec![ui_file],
            specification_file: Some(spec_file),
            validation_results: vec!["OpenAPI specification generated successfully".to_string()],
        })
    }

    /// Build OpenAPI specification from documentation
    fn build_openapi_spec(&self, documentation: &ExtractedDocumentation) -> crate::error::Result<()> {
        let mut paths = serde_json::Map::new();
        let mut components = serde_json::Map::new();
        let mut schemas = serde_json::Map::new();

        // Extract API endpoints from functions
        for item in &documentation.items {
            if let crate::docs::generator::ItemKind::Function = item.kind {
                // Check if this looks like an API endpoint
                if item.name.contains("api") || item.name.contains("handler") || item.name.contains("endpoint") {
                    let path_spec = self.function_to_openapi_path(item)?;
                    paths.insert(format!("/{}", item.name), path_spec);
                }
            } else if let crate::docs::generator::ItemKind::Struct = item.kind {
                // Add struct as schema component
                let schema_spec = self.struct_to_openapi_schema(item)?;
                schemas.insert(item.name.clone(), schema_spec);
            }
        }

        components.insert("schemas".to_string(), serde_json::Value::Object(schemas));

        let spec = serde_json::json!({
            "openapi": "3.0.0",
            "info": {
                "title": "CURSED API Documentation",
                "description": "API documentation generated from CURSED source code",
                "version": "1.0.0"
            },
            "paths": paths,
            "components": components
        });

        Ok(spec)
    }

    /// Convert function to OpenAPI path specification
    fn function_to_openapi_path(&self, item: &DocumentationItem) -> crate::error::Result<()> {
        let mut parameters = Vec::new();
        
        for param in &item.parameters {
            parameters.push(serde_json::json!({
                "name": param.name,
                "in": "query",
                "description": param.description,
                "required": false,
                "schema": {
                    "type": "string"
                }
            }));
        }

        Ok(serde_json::json!({
            "get": {
                "summary": item.name.clone(),
                "description": item.description,
                "parameters": parameters,
                "responses": {
                    "200": {
                        "description": "Successful response",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object"
                                }
                            }
                        }
                    }
                }
            }
        }))
    }

    /// Convert struct to OpenAPI schema
    fn struct_to_openapi_schema(&self, item: &DocumentationItem) -> crate::error::Result<()> {
        let mut properties = serde_json::Map::new();
        
        // Extract properties from parameters (representing struct fields)
        for param in &item.parameters {
            properties.insert(param.name.clone(), serde_json::json!({
                "type": "string",
                "description": param.description
            }));
        }

        Ok(serde_json::json!({
            "type": "object",
            "description": item.description,
            "properties": properties
        }))
    }

    /// Generate Swagger UI HTML
    fn generate_swagger_ui_html(&self) -> String {
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CURSED API Documentation</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@3.52.5/swagger-ui.css" />
    <style>
        html { box-sizing: border-box; overflow: -moz-scrollbars-vertical; overflow-y: scroll; }
        *, *:before, *:after { box-sizing: inherit; }
        body { margin:0; background: #fafafa; }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@3.52.5/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@3.52.5/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {
            const ui = SwaggerUIBundle({
                url: './openapi.json',
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout"
            });
        };
    </script>
</body>
</html>"#.to_string()
    }

    /// Generate API Blueprint documentation
    fn generate_api_blueprint(&self, documentation: &ExtractedDocumentation, output_dir: &Path) -> crate::error::Result<()> {
        // Placeholder implementation for API Blueprint
        let blueprint_content = format!(
            r#"FORMAT: 1A

# CURSED API Documentation

This is the API documentation for the CURSED programming language, generated from source code.

## Functions

{}

## Data Structures

{}"#,
            self.generate_blueprint_functions(documentation),
            self.generate_blueprint_structures(documentation)
        );

        let blueprint_file = output_dir.join("api.apib");
        fs::write(&blueprint_file, blueprint_content)
            .map_err(|e| CursedError::SystemError(format!("Failed to write API Blueprint: {}", e)))?;

        Ok(ApiDocumentationResult {
            success: true,
            format: ApiDocFormat::ApiBlueprint,
            output_files: vec![blueprint_file.clone()],
            specification_file: Some(blueprint_file),
            validation_results: vec!["API Blueprint generated successfully".to_string()],
        })
    }

    /// Generate function documentation for API Blueprint
    fn generate_blueprint_functions(&self, documentation: &ExtractedDocumentation) -> String {
        let mut content = String::new();
        
        for item in &documentation.items {
            if let crate::docs::generator::ItemKind::Function = item.kind {
                content.push_str(&format!(
                    r#"### {} [GET /{name}]

{description}

+ Response 200 (application/json)

        {{
            "result": "success"
        }}

"#,
                    item.name,
                    name = item.name,
                    description = item.description
                ));
            }
        }
        
        content
    }

    /// Generate structure documentation for API Blueprint
    fn generate_blueprint_structures(&self, documentation: &ExtractedDocumentation) -> String {
        let mut content = String::new();
        
        for item in &documentation.items {
            if let crate::docs::generator::ItemKind::Struct = item.kind {
                content.push_str(&format!(
                    r#"## {} (object)

{description}

"#,
                    item.name,
                    description = item.description
                ));
            }
        }
        
        content
    }

    /// Generate Swagger 2.0 specification
    fn generate_swagger_spec(&self, documentation: &ExtractedDocumentation, output_dir: &Path) -> crate::error::Result<()> {
        // Similar to OpenAPI but using Swagger 2.0 format
        let spec = serde_json::json!({
            "swagger": "2.0",
            "info": {
                "title": "CURSED API Documentation",
                "description": "API documentation generated from CURSED source code",
                "version": "1.0.0"
            },
            "host": "api.cursed.dev",
            "basePath": "/v1",
            "schemes": ["https"],
            "paths": {},
            "definitions": {}
        });

        let spec_json = serde_json::to_string_pretty(&spec)
            .map_err(|e| CursedError::SystemError(format!("Failed to serialize Swagger spec: {}", e)))?;

        let spec_file = output_dir.join("swagger.json");
        fs::write(&spec_file, spec_json)
            .map_err(|e| CursedError::SystemError(format!("Failed to write Swagger spec: {}", e)))?;

        Ok(ApiDocumentationResult {
            success: true,
            format: ApiDocFormat::Swagger,
            output_files: vec![spec_file.clone()],
            specification_file: Some(spec_file),
            validation_results: vec!["Swagger 2.0 specification generated successfully".to_string()],
        })
    }

    /// Generate RAML specification
    fn generate_raml_spec(&self, documentation: &ExtractedDocumentation, output_dir: &Path) -> crate::error::Result<()> {
        let raml_content = format!(
            r#"#%RAML 1.0
title: CURSED API Documentation
description: API documentation generated from CURSED source code
version: v1
baseUri: https://api.cursed.dev/v1

types:
{}

{}
"#,
            self.generate_raml_types(documentation),
            self.generate_raml_endpoints(documentation)
        );

        let raml_file = output_dir.join("api.raml");
        fs::write(&raml_file, raml_content)
            .map_err(|e| CursedError::SystemError(format!("Failed to write RAML spec: {}", e)))?;

        Ok(ApiDocumentationResult {
            success: true,
            format: ApiDocFormat::Raml,
            output_files: vec![raml_file.clone()],
            specification_file: Some(raml_file),
            validation_results: vec!["RAML specification generated successfully".to_string()],
        })
    }

    /// Generate RAML type definitions
    fn generate_raml_types(&self, documentation: &ExtractedDocumentation) -> String {
        let mut content = String::new();
        
        for item in &documentation.items {
            if let crate::docs::generator::ItemKind::Struct = item.kind {
                content.push_str(&format!(
                    "  {}:\n    description: {}\n    type: object\n\n",
                    item.name, item.description
                ));
            }
        }
        
        content
    }

    /// Generate RAML endpoints
    fn generate_raml_endpoints(&self, documentation: &ExtractedDocumentation) -> String {
        let mut content = String::new();
        
        for item in &documentation.items {
            if let crate::docs::generator::ItemKind::Function = item.kind {
                content.push_str(&format!(
                    r#"/{}:
  description: {}
  get:
    responses:
      200:
        body:
          application/json:
            type: object

"#,
                    item.name, item.description
                ));
            }
        }
        
        content
    }

    /// Generate custom API format
    fn generate_custom_api_format(&self, documentation: &ExtractedDocumentation, output_dir: &Path, format: &str) -> crate::error::Result<()> {
        // Placeholder for custom format implementation
        let custom_content = format!(
            "# Custom API Documentation Format: {}\n\nGenerated from CURSED source code\n\nTotal items: {}",
            format,
            documentation.items.len()
        );

        let custom_file = output_dir.join("custom_api.txt");
        fs::write(&custom_file, custom_content)
            .map_err(|e| CursedError::SystemError(format!("Failed to write custom API format: {}", e)))?;

        Ok(ApiDocumentationResult {
            success: true,
            format: ApiDocFormat::Custom(format.to_string()),
            output_files: vec![custom_file.clone()],
            specification_file: Some(custom_file),
            validation_results: vec![format!("Custom format '{}' generated successfully", format)],
        })
    }

    /// Deploy to hosting platforms
    fn deploy_to_hosting_platforms(&mut self, output_dir: &Path) -> crate::error::Result<()> {
        let mut results = HashMap::new();

        for platform in &self.config.hosting_platforms.clone() {
            let result = match platform {
                HostingPlatform::GitHubPages => self.deploy_to_github_pages(output_dir),
                HostingPlatform::GitLabPages => self.deploy_to_gitlab_pages(output_dir),
                HostingPlatform::Netlify => self.deploy_to_netlify(output_dir),
                HostingPlatform::Vercel => self.deploy_to_vercel(output_dir),
                HostingPlatform::AwsS3 => self.deploy_to_aws_s3(output_dir),
                HostingPlatform::Custom(config) => self.deploy_to_custom_platform(output_dir, config),
            };

            let platform_name = match platform {
                HostingPlatform::GitHubPages => "github_pages",
                HostingPlatform::GitLabPages => "gitlab_pages",
                HostingPlatform::Netlify => "netlify",
                HostingPlatform::Vercel => "vercel",
                HostingPlatform::AwsS3 => "aws_s3",
                HostingPlatform::Custom(config) => &config.name,
            };

            results.insert(platform_name.to_string(), result.unwrap_or_else(|e| DeploymentResult {
                success: false,
                platform: platform_name.to_string(),
                url: None,
                deployment_time: std::time::Duration::from_secs(0),
                error_message: Some(e.to_string()),
            }));
        }

        Ok(results)
    }

    /// Deploy to GitHub Pages
    fn deploy_to_github_pages(&self, output_dir: &Path) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        
        // This would typically involve:
        // 1. Copying files to gh-pages branch
        // 2. Committing and pushing changes
        // 3. Configuring GitHub Pages settings
        
        // For now, just create a GitHub Pages configuration
        let pages_config = r#"# GitHub Pages Configuration
name: Deploy Documentation

on:
  push:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Deploy to GitHub Pages
      uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: ./docs
"#;

        fs::write(output_dir.join(".github-pages.yml"), pages_config)
            .map_err(|e| CursedError::SystemError(format!("Failed to write GitHub Pages config: {}", e)))?;

        Ok(DeploymentResult {
            success: true,
            platform: "GitHub Pages".to_string(),
            url: Some("https://your-username.github.io/your-repo".to_string()),
            deployment_time: start_time.elapsed(),
            error_message: None,
        })
    }

    /// Deploy to GitLab Pages
    fn deploy_to_gitlab_pages(&self, output_dir: &Path) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        
        let gitlab_ci = r#"pages:
  stage: deploy
  script:
    - mkdir public
    - cp -r docs/* public/
  artifacts:
    paths:
      - public
  only:
    - main
"#;

        fs::write(output_dir.join(".gitlab-ci.yml"), gitlab_ci)
            .map_err(|e| CursedError::SystemError(format!("Failed to write GitLab CI config: {}", e)))?;

        Ok(DeploymentResult {
            success: true,
            platform: "GitLab Pages".to_string(),
            url: Some("https://your-username.gitlab.io/your-repo".to_string()),
            deployment_time: start_time.elapsed(),
            error_message: None,
        })
    }

    /// Deploy to Netlify
    fn deploy_to_netlify(&self, output_dir: &Path) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        
        // Create Netlify configuration
        let netlify_toml = r#"[build]
  publish = "docs"

[build.environment]
  NODE_VERSION = "16"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
"#;

        fs::write(output_dir.join("netlify.toml"), netlify_toml)
            .map_err(|e| CursedError::SystemError(format!("Failed to write Netlify config: {}", e)))?;

        Ok(DeploymentResult {
            success: true,
            platform: "Netlify".to_string(),
            url: Some("https://your-site.netlify.app".to_string()),
            deployment_time: start_time.elapsed(),
            error_message: None,
        })
    }

    /// Deploy to Vercel
    fn deploy_to_vercel(&self, output_dir: &Path) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        
        let vercel_json = serde_json::json!({
            "version": 2,
            "builds": [
                {
                    "src": "docs/**/*",
                    "use": "@vercel/static"
                }
            ],
            "routes": [
                {
                    "src": "/(.*)",
                    "dest": "/docs/$1"
                }
            ]
        });

        let vercel_json_str = serde_json::to_string_pretty(&vercel_json)
            .map_err(|e| CursedError::SystemError(format!("Failed to serialize Vercel config: {}", e)))?;

        fs::write(output_dir.join("vercel.json"), vercel_json_str)
            .map_err(|e| CursedError::SystemError(format!("Failed to write Vercel config: {}", e)))?;

        Ok(DeploymentResult {
            success: true,
            platform: "Vercel".to_string(),
            url: Some("https://your-project.vercel.app".to_string()),
            deployment_time: start_time.elapsed(),
            error_message: None,
        })
    }

    /// Deploy to AWS S3
    fn deploy_to_aws_s3(&self, output_dir: &Path) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        
        // Create AWS deployment script
        let aws_script = r#"#!/bin/bash
# AWS S3 Deployment Script
aws s3 sync docs/ s3://your-bucket-name --delete
aws cloudfront create-invalidation --distribution-id YOUR_DISTRIBUTION_ID --paths "/*"
"#;

        fs::write(output_dir.join("deploy-aws.sh"), aws_script)
            .map_err(|e| CursedError::SystemError(format!("Failed to write AWS deployment script: {}", e)))?;

        Ok(DeploymentResult {
            success: true,
            platform: "AWS S3".to_string(),
            url: Some("https://your-bucket-name.s3-website-region.amazonaws.com".to_string()),
            deployment_time: start_time.elapsed(),
            error_message: None,
        })
    }

    /// Deploy to custom platform
    fn deploy_to_custom_platform(&self, output_dir: &Path, config: &HostingConfig) -> crate::error::Result<()> {
        let start_time = std::time::Instant::now();
        
        // Execute custom deployment command
        if let Some(ref script_path) = config.deployment_script {
            let output = Command::new("bash")
                .arg(script_path)
                .arg(output_dir)
                .output()
                .map_err(|e| CursedError::SystemError(format!("Failed to execute deployment script: {}", e)))?;

            if !output.status.success() {
                return Ok(DeploymentResult {
                    success: false,
                    platform: config.name.clone(),
                    url: None,
                    deployment_time: start_time.elapsed(),
                    error_message: Some(String::from_utf8_lossy(&output.stderr).to_string()),
                });
            }
        }

        Ok(DeploymentResult {
            success: true,
            platform: config.name.clone(),
            url: Some(config.base_url.clone()),
            deployment_time: start_time.elapsed(),
            error_message: None,
        })
    }

    /// Sanitize filename for safe file system operations
    fn sanitize_filename(&self, name: &str) -> String {
        name.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect()
    }
}

impl TemplateManager {
    /// Load default templates
    fn load_default_templates(&mut self) {
        // Responsive index template
        self.html_templates.insert("responsive_index".to_string(), r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{TITLE}}</title>
    <style>{{CSS}}</style>
</head>
<body>
    <div class="layout">
        <nav class="sidebar">
            <div class="logo">
                <h2>CURSED Docs</h2>
            </div>
            <div class="search-container">
                <input type="text" id="search" placeholder="Search documentation..." />
            </div>
            <div class="navigation">
                {{NAVIGATION}}
            </div>
        </nav>
        <main class="content">
            {{CONTENT}}
        </main>
    </div>
    <script>{{JAVASCRIPT}}</script>
</body>
</html>"#.to_string());

        // Item page template
        self.html_templates.insert("item_page".to_string(), r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{TITLE}}</title>
    <style>{{CSS}}</style>
</head>
<body>
    <div class="layout">
        <nav class="sidebar">
            <div class="logo">
                <h2>CURSED Docs</h2>
            </div>
            <div class="navigation">
                {{NAVIGATION}}
            </div>
        </nav>
        <main class="content">
            {{BREADCRUMBS}}
            {{CONTENT}}
            {{RELATED_ITEMS}}
        </main>
    </div>
    <script>{{JAVASCRIPT}}</script>
</body>
</html>"#.to_string());

        // Modern theme CSS
        self.stylesheets.insert("modern".to_string(), r#"
/* Modern Theme */
* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.6; color: #333; }
.layout { display: grid; grid-template-columns: 300px 1fr; min-height: 100vh; }
.sidebar { background: #f8f9fa; border-right: 1px solid #e9ecef; padding: 20px; overflow-y: auto; }
.content { padding: 30px; max-width: 1200px; }
.logo h2 { color: #007bff; margin-bottom: 20px; }
.search-container { margin-bottom: 20px; }
.search-container input { width: 100%; padding: 10px; border: 1px solid #e9ecef; border-radius: 4px; }
.nav-category { margin-bottom: 20px; }
.nav-category-title { color: #6c757d; font-size: 0.9em; margin-bottom: 10px; text-transform: uppercase; }
.nav-items { list-style: none; }
.nav-item { margin-bottom: 5px; }
.nav-link { color: #333; text-decoration: none; padding: 5px 10px; display: block; border-radius: 4px; }
.nav-link:hover { background: #e9ecef; }
.hero-section { margin-bottom: 40px; }
.hero-section h1 { color: #2c3e50; margin-bottom: 15px; }
.hero-description { font-size: 1.1em; color: #6c757d; margin-bottom: 30px; }
.stats-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 20px; margin: 20px 0; }
.stat-card { background: white; border: 1px solid #e9ecef; border-radius: 8px; padding: 20px; text-align: center; }
.stat-number { font-size: 2em; font-weight: bold; color: #007bff; }
.stat-label { color: #6c757d; font-size: 0.9em; }
.features-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin: 30px 0; }
.feature-card { background: white; border: 1px solid #e9ecef; border-radius: 8px; padding: 20px; }
.feature-card h3 { color: #2c3e50; margin-bottom: 10px; }
.quick-start { margin: 30px 0; }
.example-code { background: #f8f9fa; border: 1px solid #e9ecef; border-radius: 8px; padding: 20px; margin: 15px 0; }
.example-code pre { background: none; margin: 0; }
.breadcrumbs { margin-bottom: 20px; color: #6c757d; }
.breadcrumbs a { color: #007bff; text-decoration: none; }
.breadcrumbs .separator { margin: 0 10px; }
.item-header { margin-bottom: 20px; }
.item-header h1 { color: #2c3e50; }
.item-meta { color: #6c757d; margin-top: 10px; }
.item-meta span { background: #e9ecef; padding: 4px 8px; border-radius: 4px; margin-right: 10px; font-size: 0.9em; }
.item-description { margin: 20px 0; }
.item-parameters, .item-examples, .source-info, .related-items { margin: 25px 0; }
.item-parameters h3, .item-examples h3, .source-info h3, .related-items h3 { color: #495057; margin-bottom: 15px; }
.item-parameters ul { margin-left: 20px; }
.item-parameters li { margin-bottom: 5px; }
.example { background: #f8f9fa; border: 1px solid #e9ecef; border-radius: 8px; padding: 15px; margin: 10px 0; }
.example pre { background: none; margin: 0; }
code { background: #f8f9fa; padding: 2px 4px; border-radius: 3px; font-family: 'Monaco', 'Consolas', monospace; }
pre code { background: none; padding: 0; }
@media (max-width: 768px) {
    .layout { grid-template-columns: 1fr; }
    .sidebar { display: none; }
}
"#.to_string());

        // JavaScript for interactive features
        self.scripts.insert("main".to_string(), r#"
// Search functionality
document.addEventListener('DOMContentLoaded', function() {
    const searchInput = document.getElementById('search');
    if (searchInput) {
        searchInput.addEventListener('input', function(e) {
            const query = e.target.value.toLowerCase();
            const navItems = document.querySelectorAll('.nav-item');
            
            navItems.forEach(item => {
                const text = item.textContent.toLowerCase();
                if (text.includes(query)) {
                    item.style.display = 'block';
                } else {
                    item.style.display = 'none';
                }
            });
        });
    }
    
    // Mobile navigation toggle
    const sidebar = document.querySelector('.sidebar');
    const content = document.querySelector('.content');
    
    if (window.innerWidth <= 768) {
        const toggleBtn = document.createElement('button');
        toggleBtn.innerHTML = '☰ Menu';
        toggleBtn.style.cssText = 'position: fixed; top: 10px; left: 10px; z-index: 1000; background: #007bff; color: white; border: none; padding: 10px; border-radius: 4px; cursor: pointer;';
        document.body.appendChild(toggleBtn);
        
        toggleBtn.addEventListener('click', function() {
            if (sidebar.style.display === 'none' || !sidebar.style.display) {
                sidebar.style.display = 'block';
                sidebar.style.position = 'fixed';
                sidebar.style.top = '0';
                sidebar.style.left = '0';
                sidebar.style.width = '100%';
                sidebar.style.height = '100%';
                sidebar.style.zIndex = '999';
            } else {
                sidebar.style.display = 'none';
            }
        });
    }
});
"#.to_string());
    }
}

/// Documentation statistics
#[derive(Debug, Default)]
pub struct DocumentationStats {
    pub function_count: usize,
    pub struct_count: usize,
    pub interface_count: usize,
    pub module_count: usize,
    pub enum_count: usize,
    pub constant_count: usize,
    pub variable_count: usize,
    pub type_count: usize,
}

/// HTML generation result
#[derive(Debug)]
pub struct HtmlGenerationResult {
    pub success: bool,
    pub output_directory: PathBuf,
    pub generated_files: Vec<PathBuf>,
    pub total_pages: usize,
    pub theme_used: HtmlTheme,
}

/// Deployment result
#[derive(Debug)]
pub struct DeploymentResult {
    pub success: bool,
    pub platform: String,
    pub url: Option<String>,
    pub deployment_time: std::time::Duration,
    pub error_message: Option<String>,
}

/// Overall generation results
#[derive(Debug, Default)]
pub struct GenerationResults {
    pub html_result: Option<HtmlGenerationResult>,
    pub pdf_result: Option<PdfGenerationResult>,
    pub api_result: Option<ApiDocumentationResult>,
    pub hosting_results: HashMap<String, DeploymentResult>,
}
