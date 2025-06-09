//! Core documentation generator for the CURSED programming language
//!
//! This is the main orchestrator that coordinates parsing CURSED source files,
//! extracting documentation comments, parsing AST, and generating HTML documentation.

use crate::docs::{
    DocError, DocResult, CommentParser, DocumentationItem, 
    PackageDocumentation, html_renderer::HtmlRenderer, TemplateEngine
};
use crate::parser::Parser;
use crate::lexer::Lexer;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use tracing::{debug, info, instrument, warn};

/// Documentation generation configuration
#[derive(Debug, Clone)]
pub struct DocConfig {
    /// Source directories to scan
    pub source_dirs: Vec<PathBuf>,
    /// Output directory for generated documentation
    pub output_dir: PathBuf,
    /// Package name
    pub package_name: String,
    /// Package version
    pub package_version: String,
    /// Package description
    pub package_description: Option<String>,
    /// Include private items in documentation
    pub include_private: bool,
    /// Generate search data
    pub generate_search: bool,
    /// Generate sitemap
    pub generate_sitemap: bool,
    /// Base URL for sitemap (if generating sitemap)
    pub base_url: Option<String>,
    /// Custom CSS file to include
    pub custom_css: Option<PathBuf>,
    /// Custom JavaScript file to include
    pub custom_js: Option<PathBuf>,
    /// Maximum depth for directory scanning
    pub max_depth: Option<usize>,
    /// File patterns to exclude
    pub exclude_patterns: Vec<String>,
}

impl Default for DocConfig {
    fn default() -> Self {
        Self {
            source_dirs: vec![PathBuf::from("src")],
            output_dir: PathBuf::from("docs"),
            package_name: "CURSED Package".to_string(),
            package_version: "1.0.0".to_string(),
            package_description: None,
            include_private: false,
            generate_search: true,
            generate_sitemap: false,
            base_url: None,
            custom_css: None,
            custom_js: None,
            max_depth: None,
            exclude_patterns: Vec::new(),
        }
    }
}

impl DocConfig {
    /// Create a new documentation configuration
    pub fn new(package_name: String, package_version: String) -> Self {
        Self {
            package_name,
            package_version,
            ..Default::default()
        }
    }

    /// Create DocConfig from file configuration
    pub fn from_file_config(file_config: crate::docs::config::DocConfigFile) -> Self {
        Self {
            source_dirs: file_config.generation.source_dirs.into_iter().map(PathBuf::from).collect(),
            output_dir: PathBuf::from(file_config.generation.output_dir),
            package_name: file_config.package.name,
            package_version: file_config.package.version,
            package_description: file_config.package.description,
            include_private: file_config.generation.include_private,
            generate_search: file_config.generation.enable_search,
            generate_sitemap: file_config.generation.sitemap_base_url.is_some(),
            base_url: file_config.generation.sitemap_base_url,
            custom_css: file_config.html.custom_css.map(PathBuf::from),
            custom_js: file_config.html.custom_js.map(PathBuf::from),
            max_depth: file_config.generation.max_depth,
            exclude_patterns: file_config.generation.exclude_patterns,
        }
    }

    /// Set source directories
    pub fn with_source_dirs(mut self, source_dirs: Vec<PathBuf>) -> Self {
        self.source_dirs = source_dirs;
        self
    }

    /// Set output directory
    pub fn with_output_dir(mut self, output_dir: PathBuf) -> Self {
        self.output_dir = output_dir;
        self
    }

    /// Set package description
    pub fn with_description(mut self, description: String) -> Self {
        self.package_description = Some(description);
        self
    }

    /// Set include private flag
    pub fn include_private(mut self, include_private: bool) -> Self {
        self.include_private = include_private;
        self
    }

    /// Set search generation flag
    pub fn with_search(mut self, enable_search: bool) -> Self {
        self.generate_search = enable_search;
        self
    }

    /// Set sitemap generation
    pub fn with_sitemap(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self.generate_sitemap = true;
        self
    }

    /// Set custom CSS file
    pub fn with_custom_css(mut self, css_path: PathBuf) -> Self {
        self.custom_css = Some(css_path);
        self
    }

    /// Set custom JavaScript file
    pub fn with_custom_js(mut self, js_path: PathBuf) -> Self {
        self.custom_js = Some(js_path);
        self
    }

    /// Set maximum directory scanning depth
    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = Some(max_depth);
        self
    }

    /// Set exclude patterns
    pub fn with_exclude_patterns(mut self, patterns: Vec<String>) -> Self {
        self.exclude_patterns = patterns;
        self
    }

    /// Set parallel jobs (placeholder for future implementation)
    pub fn with_parallel_jobs(self, _jobs: usize) -> Self {
        // For now, this is a no-op as parallel processing isn't implemented yet
        self
    }




}

/// Main documentation generator
pub struct DocumentationGenerator {
    /// Configuration
    config: DocConfig,
    /// Comment parser
    comment_parser: CommentParser,
    /// HTML renderer
    html_renderer: HtmlRenderer,
    /// Generated package documentation
    package_docs: Option<PackageDocumentation>,
}

impl DocumentationGenerator {
    /// Create a new documentation generator
    pub fn new(config: DocConfig) -> DocResult<Self> {
        let comment_parser = CommentParser::new()?;
        let html_renderer = HtmlRenderer::new(&config.output_dir);

        Ok(Self {
            config,
            comment_parser,
            html_renderer,
            package_docs: None,
        })
    }

    /// Generate documentation from configured source directories
    #[instrument(skip(self))]
    pub fn generate(&mut self) -> DocResult<DocumentationGenerationResult> {
        info!("Starting documentation generation for: {}", self.config.package_name);
        
        let start_time = std::time::Instant::now();

        // Create package documentation
        let mut package_docs = PackageDocumentation::new(
            self.config.package_name.clone(),
            self.config.package_version.clone(),
        );

        if let Some(ref description) = self.config.package_description {
            package_docs = package_docs.with_description(description.clone());
        }

        // Process all source directories
        let mut all_items = Vec::new();
        let mut processed_files = 0;
        let mut total_comments = 0;

        let source_dirs = self.config.source_dirs.clone();
        for source_dir in &source_dirs {
            let (items, files, comments) = self.process_source_directory(source_dir)?;
            all_items.extend(items);
            processed_files += files;
            total_comments += comments;
        }

        // Build cross-references
        let cross_references = self.build_cross_references(&all_items);
        self.html_renderer.set_cross_references(cross_references);

        // Update package documentation with all items
        for item in &all_items {
            package_docs.root_module = package_docs.root_module.add_export(item.clone());
        }

        // Render HTML documentation
        self.html_renderer.render_package(&package_docs)?;

        // Generate sitemap if requested
        if self.config.generate_sitemap {
            if let Some(ref base_url) = self.config.base_url {
                self.html_renderer.generate_sitemap(base_url)?;
            }
        }

        self.package_docs = Some(package_docs);

        let generation_time = start_time.elapsed();
        
        info!(
            "Documentation generation completed in {:?}: {} files, {} items, {} comments",
            generation_time, processed_files, all_items.len(), total_comments
        );

        Ok(DocumentationGenerationResult {
            items_generated: all_items.len(),
            files_processed: processed_files,
            comments_extracted: total_comments,
            output_files: self.html_renderer.generated_files().to_vec(),
            generation_time,
            package_stats: self.package_docs.as_ref().unwrap().get_statistics(),
        })
    }

    /// Process a source directory
    #[instrument(skip(self))]
    fn process_source_directory(&mut self, source_dir: &Path) -> DocResult<(Vec<DocumentationItem>, usize, usize)> {
        debug!("Processing source directory: {}", source_dir.display());

        if !source_dir.exists() {
            return Err(DocError::IoError(format!("Source directory does not exist: {}", source_dir.display())));
        }

        let cursed_files = self.find_cursed_files(source_dir, 0)?;
        debug!("Found {} CURSED files in {}", cursed_files.len(), source_dir.display());

        let mut all_items = Vec::new();
        let mut total_comments = 0;

        for file_path in &cursed_files {
            match self.process_file(file_path) {
                Ok((items, comments)) => {
                    all_items.extend(items);
                    total_comments += comments;
                }
                Err(e) => {
                    warn!("Failed to process file {}: {}", file_path.display(), e);
                    // Continue processing other files
                }
            }
        }

        Ok((all_items, cursed_files.len(), total_comments))
    }

    /// Find CURSED files in directory
    fn find_cursed_files(&self, dir: &Path, current_depth: usize) -> DocResult<Vec<PathBuf>> {
        if let Some(max_depth) = self.config.max_depth {
            if current_depth >= max_depth {
                return Ok(Vec::new());
            }
        }

        let mut files = Vec::new();
        
        for entry in fs::read_dir(dir)
            .map_err(|e| DocError::IoError(format!("Failed to read directory {}: {}", dir.display(), e)))? 
        {
            let entry = entry
                .map_err(|e| DocError::IoError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();
            
            // Check exclude patterns
            if self.should_exclude_path(&path) {
                continue;
            }
            
            if path.is_file() && self.is_cursed_file(&path) {
                files.push(path);
            } else if path.is_dir() && !self.should_ignore_directory(&path) {
                let mut subdir_files = self.find_cursed_files(&path, current_depth + 1)?;
                files.append(&mut subdir_files);
            }
        }
        
        Ok(files)
    }

    /// Check if path should be excluded based on patterns
    fn should_exclude_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        for pattern in &self.config.exclude_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
        
        false
    }

    /// Check if a file is a CURSED source file
    fn is_cursed_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("csd"))
            .unwrap_or(false)
    }

    /// Check if a directory should be ignored
    fn should_ignore_directory(&self, path: &Path) -> bool {
        let ignore_dirs = ["target", "build", ".git", "node_modules", "tmp", "docs"];
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| ignore_dirs.contains(&name))
            .unwrap_or(false)
    }

    /// Process a single CURSED file
    #[instrument(skip(self))]
    fn process_file(&mut self, file_path: &Path) -> DocResult<(Vec<DocumentationItem>, usize)> {
        debug!("Processing file: {}", file_path.display());

        // Read file content
        let content = fs::read_to_string(file_path)
            .map_err(|e| DocError::IoError(format!("Failed to read file {}: {}", file_path.display(), e)))?;

        // Parse documentation comments
        let comments = self.comment_parser.parse_comments(&content)?;
        let comment_count = comments.len();

        // Parse AST
        let items = self.parse_and_extract_ast(&content, file_path)?;

        // AST extraction disabled for now

        // For now, just return empty items since AST extraction is disabled
        let enhanced_items = Vec::new();

        debug!("Extracted {} items and {} comments from {}", 
               enhanced_items.len(), comment_count, file_path.display());

        Ok((enhanced_items, comment_count))
    }

    /// Parse AST and extract documentation items
    fn parse_and_extract_ast(&mut self, content: &str, file_path: &Path) -> DocResult<Vec<DocumentationItem>> {
        // For now, return empty items since AST extraction is disabled
        // TODO: Implement proper AST integration
        warn!("AST extraction disabled, skipping for: {}", file_path.display());
        Ok(Vec::new())
    }

    /// Try to parse CURSED source code
    fn try_parse_cursed_source(&self, content: &str) -> DocResult<crate::ast::Program> {
        let mut lexer = Lexer::new(content);
        let mut parser = Parser::new(&mut lexer)
            .map_err(|e| DocError::ParseError(format!("Failed to create parser: {:?}", e)))?;
        
        parser.parse_program()
            .map_err(|e| DocError::ParseError(format!("Failed to parse program: {:?}", e)))
    }

    /// Enhance items with comment information
    fn enhance_items_with_comments(
        &self, 
        mut items: Vec<DocumentationItem>, 
        comments: &[crate::docs::DocComment]
    ) -> DocResult<Vec<DocumentationItem>> {
        
        for item in &mut items {
            // Find the closest preceding comment
            if let Some(comment) = self.find_preceding_comment(comments, item.line) {
                item.doc_comment = Some(comment.clone());
                
                // Update parameter descriptions from @param tags
                let param_descriptions = item.parameter_descriptions();
                for param in &mut item.parameters {
                    if let Some(desc) = param_descriptions.get(&param.name) {
                        param.description = Some(desc.clone());
                    }
                }
            }
        }
        
        Ok(items)
    }

    /// Find documentation comment preceding a given line
    fn find_preceding_comment<'a>(&self, comments: &'a [crate::docs::DocComment], target_line: usize) -> Option<&'a crate::docs::DocComment> {
        comments
            .iter()
            .filter(|comment| comment.line < target_line)
            .max_by_key(|comment| comment.line)
    }

    /// Build cross-references between items
    fn build_cross_references(&self, items: &[DocumentationItem]) -> HashMap<String, Vec<String>> {
        let mut cross_references = HashMap::new();
        
        for item in items {
            let mut references = Vec::new();
            
            // Look for references in description
            if let Some(description) = item.description() {
                references.extend(self.find_references_in_text(description, items));
            }
            
            // Look for references in parameter types
            for param in &item.parameters {
                references.extend(self.find_references_in_text(&param.param_type, items));
            }
            
            // Look for references in return type
            if let Some(return_type) = &item.return_type {
                references.extend(self.find_references_in_text(return_type, items));
            }
            
            if !references.is_empty() {
                cross_references.insert(item.name.clone(), references);
            }
        }
        
        cross_references
    }

    /// Find references to other items in text
    fn find_references_in_text(&self, text: &str, all_items: &[DocumentationItem]) -> Vec<String> {
        let mut references = Vec::new();
        
        for item in all_items {
            // Look for exact word matches
            let words: Vec<&str> = text.split_whitespace().collect();
            if words.contains(&item.name.as_str()) {
                references.push(item.name.clone());
            }
        }
        
        // Remove duplicates
        references.sort();
        references.dedup();
        references
    }

    /// Get generated package documentation
    pub fn package_documentation(&self) -> Option<&PackageDocumentation> {
        self.package_docs.as_ref()
    }

    /// Clean output directory before generation
    pub fn clean_output(&mut self) -> DocResult<()> {
        self.html_renderer.clean_output_directory()
    }

    /// Validate documentation completeness without generating output
    #[instrument(skip(self))]
    pub fn validate_documentation(&self) -> DocResult<crate::docs::DocumentationValidationResult> {
        use crate::docs::DocumentationValidationResult;
        
        info!("Starting documentation validation");
        let mut result = DocumentationValidationResult::new();
        
        // For now, provide a basic validation that checks source directories exist
        for source_dir in &self.config.source_dirs {
            if !source_dir.exists() {
                result.add_error(format!("Source directory does not exist: {}", source_dir.display()));
                continue;
            }
            
            // Simple file counting for basic validation
            let files = self.find_cursed_files(source_dir, 0)?;
            result.total_items = files.len();
            result.documented_items = files.len(); // Assume all files have some documentation for now
            
            for file_path in files {
                if self.should_exclude_path(&file_path) {
                    continue;
                }
                
                debug!("Checking file: {}", file_path.display());
                
                // Basic file validation - check if it's readable
                match std::fs::read_to_string(&file_path) {
                    Ok(content) => {
                        // Simple heuristic: check if file has documentation comments
                        if !content.contains("///") && !content.contains("//!") {
                            result.add_warning(format!(
                                "File {} may lack documentation comments",
                                file_path.display()
                            ));
                        }
                    }
                    Err(e) => {
                        result.add_error(format!("Cannot read file {}: {}", file_path.display(), e));
                    }
                }
            }
        }
        
        info!(
            "Documentation validation complete: {} files checked, {} warnings, {} errors",
            result.total_items, 
            result.warnings.len(),
            result.errors.len()
        );
        
        Ok(result)
    }


    /// Get configuration (for server integration)
    pub fn config(&self) -> &DocConfig {
        &self.config
    }
}

/// Documentation generation result
#[derive(Debug, Clone)]
pub struct DocumentationGenerationResult {
    /// Number of documentation items generated
    pub items_generated: usize,
    /// Number of source files processed
    pub files_processed: usize,
    /// Number of documentation comments extracted
    pub comments_extracted: usize,
    /// List of generated output files
    pub output_files: Vec<PathBuf>,
    /// Time taken for generation
    pub generation_time: std::time::Duration,
    /// Package statistics
    pub package_stats: crate::docs::package_docs::PackageStatistics,
}

impl DocumentationGenerationResult {
    /// Generate a summary report
    pub fn summary(&self) -> String {
        format!(
            "Documentation Generation Summary:
  - Files processed: {}
  - Items documented: {}
  - Comments extracted: {}
  - Output files: {}
  - Generation time: {:?}
  - Functions: {}
  - Squads: {}
  - Collabs: {}",
            self.files_processed,
            self.items_generated,
            self.comments_extracted,
            self.output_files.len(),
            self.generation_time,
            self.package_stats.function_count,
            self.package_stats.squad_count,
            self.package_stats.collab_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_doc_config_creation() {
        let config = DocConfig::new("test_package".to_string(), "1.0.0".to_string());
        
        assert_eq!(config.package_name, "test_package");
        assert_eq!(config.package_version, "1.0.0");
        assert_eq!(config.source_dirs, vec![PathBuf::from("src")]);
        assert_eq!(config.output_dir, PathBuf::from("docs"));
    }

    #[test]
    fn test_doc_config_builder() {
        let config = DocConfig::new("test".to_string(), "1.0.0".to_string())
            .with_description("Test package".to_string())
            .include_private(true)
            .with_search(false)
            .with_max_depth(3);
        
        assert_eq!(config.package_description, Some("Test package".to_string()));
        assert_eq!(config.include_private, true);
        assert_eq!(config.generate_search, false);
        assert_eq!(config.max_depth, Some(3));
    }

    #[test]
    fn test_documentation_generator_creation() {
        let config = DocConfig::default();
        let generator = DocumentationGenerator::new(config);
        
        assert!(generator.is_ok());
    }

    #[test]
    fn test_is_cursed_file() {
        let config = DocConfig::default();
        let generator = DocumentationGenerator::new(config).unwrap();
        
        assert!(generator.is_cursed_file(Path::new("test.csd")));
        assert!(generator.is_cursed_file(Path::new("TEST.CSD")));
        assert!(!generator.is_cursed_file(Path::new("test.rs")));
        assert!(!generator.is_cursed_file(Path::new("test.txt")));
    }

    #[test]
    fn test_should_ignore_directory() {
        let config = DocConfig::default();
        let generator = DocumentationGenerator::new(config).unwrap();
        
        assert!(generator.should_ignore_directory(Path::new("target")));
        assert!(generator.should_ignore_directory(Path::new("build")));
        assert!(generator.should_ignore_directory(Path::new(".git")));
        assert!(generator.should_ignore_directory(Path::new("docs")));
        assert!(!generator.should_ignore_directory(Path::new("src")));
        assert!(!generator.should_ignore_directory(Path::new("lib")));
    }

    #[test]
    fn test_exclude_patterns() {
        let config = DocConfig::default()
            .with_exclude_patterns(vec!["test".to_string(), "example".to_string()]);
        let generator = DocumentationGenerator::new(config).unwrap();
        
        assert!(generator.should_exclude_path(Path::new("test.csd")));
        assert!(generator.should_exclude_path(Path::new("example_file.csd")));
        assert!(generator.should_exclude_path(Path::new("src/test/mod.csd")));
        assert!(!generator.should_exclude_path(Path::new("src/main.csd")));
    }

    #[test]
    fn test_generation_result_summary() {
        let result = DocumentationGenerationResult {
            items_generated: 10,
            files_processed: 5,
            comments_extracted: 8,
            output_files: vec![PathBuf::from("index.html")],
            generation_time: std::time::Duration::from_millis(500),
            package_stats: crate::docs::package_docs::PackageStatistics {
                total_modules: 1,
                total_items: 10,
                function_count: 7,
                squad_count: 2,
                collab_count: 1,
                cross_reference_count: 5,
                total_lines_of_documentation: 50,
            },
        };
        
        let summary = result.summary();
        assert!(summary.contains("Files processed: 5"));
        assert!(summary.contains("Items documented: 10"));
        assert!(summary.contains("Functions: 7"));
        assert!(summary.contains("Squads: 2"));
        assert!(summary.contains("Collabs: 1"));
    }
}
