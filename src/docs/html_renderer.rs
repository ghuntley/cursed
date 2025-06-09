//! HTML rendering system for CURSED documentation
//!
//! Handles the actual rendering of documentation items to HTML,
//! including cross-references, syntax highlighting, and responsive layout.

use crate::docs::{DocError, DocResult, DocumentationItem, ItemType, PackageDocumentation, TemplateEngine};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument};

/// HTML documentation renderer
pub struct HtmlRenderer {
    /// Template engine
    template_engine: TemplateEngine,
    /// Output directory
    output_dir: PathBuf,
    /// Cross-reference map
    cross_references: HashMap<String, Vec<String>>,
    /// Generated file paths
    generated_files: Vec<PathBuf>,
}

impl HtmlRenderer {
    /// Create a new HTML renderer
    pub fn new<P: AsRef<Path>>(output_dir: P) -> Self {
        Self {
            template_engine: TemplateEngine::new(),
            output_dir: output_dir.as_ref().to_path_buf(),
            cross_references: HashMap::new(),
            generated_files: Vec::new(),
        }
    }

    /// Set cross-references
    pub fn set_cross_references(&mut self, cross_references: HashMap<String, Vec<String>>) {
        self.cross_references = cross_references;
    }

    /// Render complete package documentation
    #[instrument(skip(self, package))]
    pub fn render_package(&mut self, package: &PackageDocumentation) -> DocResult<()> {
        info!("Rendering package documentation for: {}", package.name);

        // Create output directory
        self.create_output_directory()?;

        // Collect all items
        let all_items = package.root_module.all_items();
        let items: Vec<DocumentationItem> = all_items.into_iter().cloned().collect();

        // Render main index page
        self.render_index_page(&package.name, &items)?;

        // Render individual item pages
        for item in &items {
            self.render_item_page(item, &items)?;
        }

        // Render search page
        self.render_search_page(&items)?;

        // Copy static assets
        self.copy_static_assets()?;

        // Generate search data
        self.generate_search_data(&items)?;

        info!("Generated {} documentation files", self.generated_files.len());
        Ok(())
    }

    /// Create output directory
    fn create_output_directory(&self) -> DocResult<()> {
        fs::create_dir_all(&self.output_dir)
            .map_err(|e| DocError::IoError(format!("Failed to create output directory: {}", e)))?;
        Ok(())
    }

    /// Render index page
    fn render_index_page(&mut self, package_name: &str, items: &[DocumentationItem]) -> DocResult<()> {
        debug!("Rendering index page");

        let html = self.template_engine.render_main_page(package_name, items)?;
        let file_path = self.output_dir.join("index.html");
        
        fs::write(&file_path, html)
            .map_err(|e| DocError::IoError(format!("Failed to write index.html: {}", e)))?;
        
        self.generated_files.push(file_path);
        Ok(())
    }

    /// Render individual item page
    fn render_item_page(&mut self, item: &DocumentationItem, all_items: &[DocumentationItem]) -> DocResult<()> {
        debug!("Rendering page for: {}", item.name);

        // Enhance item with cross-references
        let enhanced_item = self.enhance_item_with_links(item)?;
        
        let html = self.template_engine.render_item_page(&enhanced_item, all_items)?;
        let filename = format!("{}.html", self.sanitize_filename(&item.name));
        let file_path = self.output_dir.join(filename);
        
        fs::write(&file_path, html)
            .map_err(|e| DocError::IoError(format!("Failed to write {}: {}", file_path.display(), e)))?;
        
        self.generated_files.push(file_path);
        Ok(())
    }

    /// Render search page
    fn render_search_page(&mut self, items: &[DocumentationItem]) -> DocResult<()> {
        debug!("Rendering search page");

        let html = self.template_engine.render_index_page(items)?;
        let file_path = self.output_dir.join("search.html");
        
        fs::write(&file_path, html)
            .map_err(|e| DocError::IoError(format!("Failed to write search.html: {}", e)))?;
        
        self.generated_files.push(file_path);
        Ok(())
    }

    /// Enhance item with cross-reference links
    fn enhance_item_with_links(&self, item: &DocumentationItem) -> DocResult<DocumentationItem> {
        let mut enhanced_item = item.clone();
        
        // Add cross-reference links to description
        if let Some(description) = enhanced_item.description() {
            let enhanced_description = self.add_cross_reference_links(description);
            if let Some(ref mut doc_comment) = enhanced_item.doc_comment {
                doc_comment.description = enhanced_description;
            }
        }
        
        // Enhance parameter descriptions
        for param in &mut enhanced_item.parameters {
            if let Some(ref description) = param.description {
                param.description = Some(self.add_cross_reference_links(description));
            }
        }
        
        // Enhance return description
        if let Some(return_desc) = enhanced_item.return_description() {
            let enhanced_return = self.add_cross_reference_links(&return_desc);
            if let Some(ref mut doc_comment) = enhanced_item.doc_comment {
                // Update return tag description
                for tag in &mut doc_comment.tags {
                    if let crate::docs::DocTag::Return { ref mut description } = tag {
                        *description = enhanced_return.clone();
                        break;
                    }
                }
            }
        }
        
        Ok(enhanced_item)
    }

    /// Add cross-reference links to text
    fn add_cross_reference_links(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // Look for references in cross-reference map
        for (item_name, _) in &self.cross_references {
            if result.contains(item_name) {
                let link = format!(
                    r#"<a href="{}.html" class="cross-ref">{}</a>"#,
                    self.sanitize_filename(item_name),
                    item_name
                );
                result = result.replace(item_name, &link);
            }
        }
        
        result
    }

    /// Copy static assets (CSS, JS, images)
    fn copy_static_assets(&mut self) -> DocResult<()> {
        debug!("Copying static assets");

        // Create assets directory
        let assets_dir = self.output_dir.join("assets");
        fs::create_dir_all(&assets_dir)
            .map_err(|e| DocError::IoError(format!("Failed to create assets directory: {}", e)))?;

        // Copy favicon
        let favicon_content = include_bytes!("../../docs/favicon.ico");
        let favicon_path = self.output_dir.join("favicon.ico");
        fs::write(&favicon_path, favicon_content)
            .map_err(|e| DocError::IoError(format!("Failed to write favicon: {}", e)))?;

        self.generated_files.push(favicon_path);

        // Generate CSS file
        let css_path = assets_dir.join("docs.css");
        fs::write(&css_path, ADDITIONAL_CSS)
            .map_err(|e| DocError::IoError(format!("Failed to write CSS: {}", e)))?;

        self.generated_files.push(css_path);

        // Generate JavaScript file
        let js_path = assets_dir.join("docs.js");
        fs::write(&js_path, ADDITIONAL_JS)
            .map_err(|e| DocError::IoError(format!("Failed to write JavaScript: {}", e)))?;

        self.generated_files.push(js_path);

        Ok(())
    }

    /// Generate search data for client-side search
    fn generate_search_data(&mut self, items: &[DocumentationItem]) -> DocResult<()> {
        debug!("Generating search data");

        let mut search_data = Vec::new();
        
        for item in items {
            let search_item = serde_json::json!({
                "name": item.name,
                "type": item.item_type.to_string(),
                "description": item.description().unwrap_or(""),
                "url": format!("{}.html", self.sanitize_filename(&item.name)),
                "keywords": self.extract_keywords(item)
            });
            search_data.push(search_item);
        }
        
        let search_js = format!(
            "window.CURSED_SEARCH_DATA = {};",
            serde_json::to_string_pretty(&search_data)
                .map_err(|e| DocError::IoError(format!("Failed to serialize search data: {}", e)))?
        );
        
        let search_data_path = self.output_dir.join("assets").join("search-data.js");
        fs::write(&search_data_path, search_js)
            .map_err(|e| DocError::IoError(format!("Failed to write search data: {}", e)))?;
        
        self.generated_files.push(search_data_path);
        Ok(())
    }

    /// Extract keywords from documentation item
    fn extract_keywords(&self, item: &DocumentationItem) -> Vec<String> {
        let mut keywords = Vec::new();
        
        // Add item name
        keywords.push(item.name.clone());
        
        // Add item type
        keywords.push(item.item_type.to_string());
        
        // Extract words from description
        if let Some(description) = item.description() {
            let words: Vec<String> = description
                .split_whitespace()
                .filter(|word| word.len() > 3) // Only include words longer than 3 characters
                .map(|word| word.to_lowercase())
                .collect();
            keywords.extend(words);
        }
        
        // Add parameter names
        for param in &item.parameters {
            keywords.push(param.name.clone());
            keywords.push(param.param_type.clone());
        }
        
        // Add return type
        if let Some(return_type) = &item.return_type {
            keywords.push(return_type.clone());
        }
        
        // Remove duplicates and return
        keywords.sort();
        keywords.dedup();
        keywords
    }

    /// Sanitize filename for web use
    fn sanitize_filename(&self, name: &str) -> String {
        name.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect()
    }

    /// Get list of generated files
    pub fn generated_files(&self) -> &[PathBuf] {
        &self.generated_files
    }

    /// Clean output directory
    pub fn clean_output_directory(&self) -> DocResult<()> {
        if self.output_dir.exists() {
            fs::remove_dir_all(&self.output_dir)
                .map_err(|e| DocError::IoError(format!("Failed to clean output directory: {}", e)))?;
        }
        Ok(())
    }

    /// Generate sitemap.xml
    pub fn generate_sitemap(&self, base_url: &str) -> DocResult<()> {
        let mut sitemap = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        sitemap.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
        
        for file_path in &self.generated_files {
            if let Some(filename) = file_path.file_name().and_then(|n| n.to_str()) {
                if filename.ends_with(".html") {
                    sitemap.push_str(&format!(
                        r#"<url><loc>{}/{}</loc><changefreq>weekly</changefreq><priority>0.8</priority></url>"#,
                        base_url.trim_end_matches('/'),
                        filename
                    ));
                }
            }
        }
        
        sitemap.push_str(r#"</urlset>"#);
        
        let sitemap_path = self.output_dir.join("sitemap.xml");
        fs::write(&sitemap_path, sitemap)
            .map_err(|e| DocError::IoError(format!("Failed to write sitemap: {}", e)))?;
        
        Ok(())
    }
}

// Additional CSS for enhanced styling
const ADDITIONAL_CSS: &str = r#"
/* Additional CURSED Documentation Styles */

.cross-ref {
    color: #3498db;
    text-decoration: none;
    border-bottom: 1px dotted #3498db;
    transition: all 0.3s ease;
}

.cross-ref:hover {
    color: #2980b9;
    border-bottom-color: #2980b9;
    background-color: rgba(52, 152, 219, 0.1);
}

.deprecated-warning {
    background: linear-gradient(135deg, #e74c3c, #c0392b);
    animation: pulse 2s infinite;
}

@keyframes pulse {
    0% { opacity: 1; }
    50% { opacity: 0.8; }
    100% { opacity: 1; }
}

.code-block {
    position: relative;
}

.copy-code {
    position: absolute;
    top: 10px;
    right: 10px;
    background: #34495e;
    color: white;
    border: none;
    padding: 5px 10px;
    border-radius: 3px;
    cursor: pointer;
    font-size: 12px;
    opacity: 0;
    transition: opacity 0.3s ease;
}

.code-block:hover .copy-code {
    opacity: 1;
}

.copy-code:hover {
    background: #2c3e50;
}

.search-highlight {
    background-color: #f39c12;
    color: white;
    padding: 2px 4px;
    border-radius: 2px;
}

.item-card:hover {
    transform: translateY(-2px);
    transition: transform 0.3s ease;
}

.breadcrumb {
    margin-bottom: 20px;
    padding: 10px 0;
    border-bottom: 1px solid #ecf0f1;
}

.breadcrumb a {
    color: #3498db;
    text-decoration: none;
}

.breadcrumb a:hover {
    text-decoration: underline;
}

.breadcrumb-separator {
    margin: 0 10px;
    color: #7f8c8d;
}

.toc {
    background: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 5px;
    padding: 20px;
    margin-bottom: 30px;
}

.toc h3 {
    margin-bottom: 15px;
    color: #495057;
}

.toc ul {
    list-style: none;
    margin: 0;
    padding: 0;
}

.toc li {
    margin-bottom: 5px;
}

.toc a {
    color: #6c757d;
    text-decoration: none;
}

.toc a:hover {
    color: #495057;
    text-decoration: underline;
}

.loading {
    display: inline-block;
    width: 20px;
    height: 20px;
    border: 3px solid #f3f3f3;
    border-top: 3px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}
"#;

// Additional JavaScript for enhanced functionality
const ADDITIONAL_JS: &str = r##"
// Additional CURSED Documentation JavaScript

document.addEventListener('DOMContentLoaded', function() {
    // Add copy functionality to code blocks
    addCopyFunctionality();
    
    // Add table of contents generation
    generateTableOfContents();
    
    // Add smooth scrolling
    addSmoothScrolling();
    
    // Add keyboard navigation
    addKeyboardNavigation();
    
    // Initialize enhanced search
    initializeEnhancedSearch();
});

function addCopyFunctionality() {
    const codeBlocks = document.querySelectorAll('pre code');
    codeBlocks.forEach(block => {
        const container = block.parentElement;
        container.style.position = 'relative';
        container.classList.add('code-block');
        
        const copyButton = document.createElement('button');
        copyButton.className = 'copy-code';
        copyButton.textContent = 'Copy';
        copyButton.onclick = () => copyToClipboard(block.textContent, copyButton);
        
        container.appendChild(copyButton);
    });
}

function copyToClipboard(text, button) {
    navigator.clipboard.writeText(text).then(() => {
        const originalText = button.textContent;
        button.textContent = 'Copied!';
        setTimeout(() => {
            button.textContent = originalText;
        }, 2000);
    });
}

function generateTableOfContents() {
    const content = document.querySelector('.item-content, .index-content, .overview');
    if (!content) return;
    
    const headings = content.querySelectorAll('h2, h3');
    if (headings.length < 2) return;
    
    const toc = document.createElement('div');
    toc.className = 'toc';
    toc.innerHTML = '<h3>Table of Contents</h3><ul></ul>';
    
    const ul = toc.querySelector('ul');
    
    headings.forEach((heading, index) => {
        const id = 'toc-' + index;
        heading.id = id;
        
        const li = document.createElement('li');
        const link = document.createElement('a');
        link.href = '#' + id;
        link.textContent = heading.textContent;
        li.appendChild(link);
        ul.appendChild(li);
    });
    
    // Insert TOC after the first heading
    const firstHeading = content.querySelector('h1');
    if (firstHeading && firstHeading.nextElementSibling) {
        firstHeading.parentNode.insertBefore(toc, firstHeading.nextElementSibling);
    }
}

function addSmoothScrolling() {
    const links = document.querySelectorAll('a[href^="#"]');
    links.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({ behavior: 'smooth' });
            }
        });
    });
}

function addKeyboardNavigation() {
    document.addEventListener('keydown', function(e) {
        if (e.ctrlKey && e.key === 'k') {
            e.preventDefault();
            const searchInput = document.getElementById('search-input');
            if (searchInput) {
                searchInput.focus();
            }
        }
        
        if (e.key === 'Escape') {
            const searchResults = document.getElementById('search-results');
            if (searchResults) {
                searchResults.innerHTML = '';
            }
        }
    });
}

function initializeEnhancedSearch() {
    const searchInput = document.getElementById('search-input');
    const searchResults = document.getElementById('search-results');
    
    if (!searchInput || !searchResults) return;
    
    // Load search data
    const script = document.createElement('script');
    script.src = 'assets/search-data.js';
    script.onload = () => {
        searchInput.addEventListener('input', function() {
            const query = this.value.trim().toLowerCase();
            if (query.length < 2) {
                searchResults.innerHTML = '';
                return;
            }
            
            performEnhancedSearch(query, searchResults);
        });
    };
    document.head.appendChild(script);
}

function performEnhancedSearch(query, resultsContainer) {
    if (!window.CURSED_SEARCH_DATA) {
        resultsContainer.innerHTML = '<p>Search data not loaded.</p>';
        return;
    }
    
    const results = window.CURSED_SEARCH_DATA.filter(item => {
        return item.name.toLowerCase().includes(query) ||
               item.description.toLowerCase().includes(query) ||
               item.keywords.some(keyword => keyword.includes(query));
    });
    
    if (results.length === 0) {
        resultsContainer.innerHTML = '<p>No results found.</p>';
        return;
    }
    
    let html = '';
    results.forEach(item => {
        html += '<div class="search-result">';
        html += '<h3><a href="' + item.url + '">' + highlightMatch(item.name, query) + '</a> ';
        html += '<span class="item-type">' + item.type + '</span></h3>';
        html += '<p>' + highlightMatch(item.description, query) + '</p>';
        html += '</div>';
    });
    
    resultsContainer.innerHTML = html;
}

function highlightMatch(text, query) {
    const regex = new RegExp('(' + query + ')', 'gi');
    return text.replace(regex, '<span class="search-highlight">$1</span>');
}
"##;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_html_renderer_creation() {
        let temp_dir = TempDir::new().unwrap();
        let renderer = HtmlRenderer::new(temp_dir.path());
        
        assert_eq!(renderer.output_dir, temp_dir.path());
        assert_eq!(renderer.generated_files.len(), 0);
    }

    #[test]
    fn test_sanitize_filename() {
        let temp_dir = TempDir::new().unwrap();
        let renderer = HtmlRenderer::new(temp_dir.path());
        
        assert_eq!(renderer.sanitize_filename("test-function"), "test_function");
        assert_eq!(renderer.sanitize_filename("test.function"), "test_function");
        assert_eq!(renderer.sanitize_filename("test function"), "test_function");
        assert_eq!(renderer.sanitize_filename("test_function"), "test_function");
    }

    #[test]
    fn test_extract_keywords() {
        let temp_dir = TempDir::new().unwrap();
        let renderer = HtmlRenderer::new(temp_dir.path());
        
        let mut item = DocumentationItem::new("test_function".to_string(), ItemType::Function, 1);
        item = item.with_return_type("normie".to_string());
        
        let keywords = renderer.extract_keywords(&item);
        
        assert!(keywords.contains(&"test_function".to_string()));
        assert!(keywords.contains(&"function".to_string()));
        assert!(keywords.contains(&"normie".to_string()));
    }

    #[test]
    fn test_create_output_directory() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("docs");
        let renderer = HtmlRenderer::new(&output_path);
        
        assert!(renderer.create_output_directory().is_ok());
        assert!(output_path.exists());
    }
}
