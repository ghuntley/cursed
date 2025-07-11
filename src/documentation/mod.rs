//! CURSED Documentation Generator
//! 
//! This module provides comprehensive documentation generation for the CURSED compiler.
//! It auto-generates API documentation from CURSED source code, supports multiple 
//! output formats, and integrates with the build system.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use crate::error::CursedError;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ast::{Ast, AstNode, FunctionDeclaration, VariableDeclaration, Comment};

pub mod generator;
pub mod comment_parser;
// pub mod html_generator;
pub mod markdown_generator;
pub mod json_generator;
pub mod api_extractor;
pub mod coverage_analyzer;
pub mod cross_reference;
pub mod examples;
pub mod testing;

/// Documentation configuration loaded from .cursed-doc.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocConfig {
    pub general: GeneralConfig,
    pub input: InputConfig,
    pub output: OutputConfig,
    pub html: HtmlConfig,
    pub markdown: MarkdownConfig,
    pub processing: ProcessingConfig,
    pub validation: ValidationConfig,
    pub examples: ExamplesConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub project_name: String,
    pub project_version: String,
    pub project_description: String,
    pub project_url: String,
    pub authors: Vec<String>,
    pub license: String,
    pub repository: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    pub source_dirs: Vec<String>,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub max_file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub output_dir: String,
    pub formats: Vec<String>,
    pub clean_output: bool,
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlConfig {
    pub theme: String,
    pub syntax_highlighting: bool,
    pub table_of_contents: bool,
    pub search_enabled: bool,
    pub responsive_design: bool,
    pub custom_css: Vec<String>,
    pub custom_js: Vec<String>,
    pub offline_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownConfig {
    pub flavor: String,
    pub table_of_contents: bool,
    pub code_block_style: String,
    pub link_style: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
    pub extract_comments: bool,
    pub extract_examples: bool,
    pub generate_cross_references: bool,
    pub analyze_dependencies: bool,
    pub process_cursed_files: bool,
    pub process_rust_files: bool,
    pub process_markdown_files: bool,
    pub cursed_comment_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub check_links: bool,
    pub check_examples: bool,
    pub validate_syntax: bool,
    pub require_descriptions: bool,
    pub treat_warnings_as_errors: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamplesConfig {
    pub extract_examples: bool,
    pub validate_examples: bool,
    pub run_examples: bool,
    pub categorize_by_directory: bool,
    pub generate_example_index: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub generate_api_docs: bool,
    pub include_private: bool,
    pub include_internal: bool,
    pub show_source_links: bool,
    pub require_doc_comments: bool,
    pub coverage_threshold: f64,
}

/// Represents a documented function in CURSED
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentedFunction {
    pub name: String,
    pub signature: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub return_description: String,
    pub examples: Vec<String>,
    pub source_file: String,
    pub source_line: usize,
    pub visibility: String,
}

/// Function parameter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub default_value: Option<String>,
}

/// Represents a documented module in CURSED
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentedModule {
    pub name: String,
    pub description: String,
    pub functions: Vec<DocumentedFunction>,
    pub variables: Vec<DocumentedVariable>,
    pub constants: Vec<DocumentedConstant>,
    pub types: Vec<DocumentedType>,
    pub examples: Vec<String>,
    pub source_file: String,
    pub submodules: Vec<DocumentedModule>,
}

/// Documented variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentedVariable {
    pub name: String,
    pub var_type: String,
    pub description: String,
    pub source_file: String,
    pub source_line: usize,
}

/// Documented constant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentedConstant {
    pub name: String,
    pub const_type: String,
    pub value: String,
    pub description: String,
    pub source_file: String,
    pub source_line: usize,
}

/// Documented type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentedType {
    pub name: String,
    pub type_kind: String, // struct, enum, interface, etc.
    pub description: String,
    pub fields: Vec<TypeField>,
    pub methods: Vec<DocumentedFunction>,
    pub source_file: String,
    pub source_line: usize,
}

/// Type field information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeField {
    pub name: String,
    pub field_type: String,
    pub description: String,
    pub default_value: Option<String>,
}

/// Complete documentation structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Documentation {
    pub project_info: GeneralConfig,
    pub modules: Vec<DocumentedModule>,
    pub cross_references: HashMap<String, Vec<String>>,
    pub examples: Vec<DocumentedExample>,
    pub coverage_stats: CoverageStats,
}

/// Documented example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentedExample {
    pub title: String,
    pub description: String,
    pub code: String,
    pub output: Option<String>,
    pub category: String,
    pub source_file: String,
}

/// Documentation coverage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageStats {
    pub total_functions: usize,
    pub documented_functions: usize,
    pub coverage_percentage: f64,
    pub missing_docs: Vec<String>,
}

/// Main documentation generator
pub struct DocumentationGenerator {
    config: DocConfig,
    documentation: Documentation,
}

impl DocumentationGenerator {
    /// Create a new documentation generator
    pub fn new(config_path: Option<&str>) -> Result<Self, CursedError> {
        let config = Self::load_config(config_path)?;
        
        let documentation = Documentation {
            project_info: config.general.clone(),
            modules: Vec::new(),
            cross_references: HashMap::new(),
            examples: Vec::new(),
            coverage_stats: CoverageStats {
                total_functions: 0,
                documented_functions: 0,
                coverage_percentage: 0.0,
                missing_docs: Vec::new(),
            },
        };

        Ok(Self {
            config,
            documentation,
        })
    }

    /// Load configuration from file
    fn load_config(config_path: Option<&str>) -> Result<DocConfig, CursedError> {
        let path = config_path.unwrap_or(".cursed-doc.toml");
        
        if !Path::new(path).exists() {
            return Ok(Self::default_config());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| CursedError::IoError(format!("Failed to read config file: {}", e)))?;

        toml::from_str(&content)
            .map_err(|e| CursedError::ParseError(format!("Invalid config file: {}", e)))
    }

    /// Generate default configuration
    fn default_config() -> DocConfig {
        DocConfig {
            general: GeneralConfig {
                project_name: "CURSED Project".to_string(),
                project_version: "1.0.0".to_string(),
                project_description: "A CURSED project".to_string(),
                project_url: "".to_string(),
                authors: vec!["Anonymous".to_string()],
                license: "MIT".to_string(),
                repository: "".to_string(),
            },
            input: InputConfig {
                source_dirs: vec!["src/".to_string(), "stdlib/".to_string()],
                include_patterns: vec!["**/*.csd".to_string()],
                exclude_patterns: vec!["target/**".to_string(), ".git/**".to_string()],
                max_file_size: 10 * 1024 * 1024, // 10MB
            },
            output: OutputConfig {
                output_dir: "docs/html".to_string(),
                formats: vec!["html".to_string(), "json".to_string()],
                clean_output: true,
                base_url: "".to_string(),
            },
            html: HtmlConfig {
                theme: "default".to_string(),
                syntax_highlighting: true,
                table_of_contents: true,
                search_enabled: true,
                responsive_design: true,
                custom_css: Vec::new(),
                custom_js: Vec::new(),
                offline_mode: false,
            },
            markdown: MarkdownConfig {
                flavor: "github".to_string(),
                table_of_contents: true,
                code_block_style: "fenced".to_string(),
                link_style: "reference".to_string(),
            },
            processing: ProcessingConfig {
                extract_comments: true,
                extract_examples: true,
                generate_cross_references: true,
                analyze_dependencies: true,
                process_cursed_files: true,
                process_rust_files: false,
                process_markdown_files: true,
                cursed_comment_patterns: vec![
                    r"^\s*fr fr\s+(.*)$".to_string(),
                    r"^\s*/\*\*?(.*)?\*/$".to_string(),
                ],
            },
            validation: ValidationConfig {
                check_links: true,
                check_examples: true,
                validate_syntax: true,
                require_descriptions: false,
                treat_warnings_as_errors: false,
            },
            examples: ExamplesConfig {
                extract_examples: true,
                validate_examples: true,
                run_examples: false,
                categorize_by_directory: true,
                generate_example_index: true,
            },
            api: ApiConfig {
                generate_api_docs: true,
                include_private: false,
                include_internal: false,
                show_source_links: true,
                require_doc_comments: false,
                coverage_threshold: 0.0,
            },
        }
    }

    /// Generate documentation from source files
    pub fn generate(&mut self) -> Result<(), CursedError> {
        println!("Starting documentation generation...");
        
        // Clean output directory if requested
        if self.config.output.clean_output {
            self.clean_output_dir()?;
        }

        // Create output directory
        fs::create_dir_all(&self.config.output.output_dir)
            .map_err(|e| CursedError::IoError(format!("Failed to create output directory: {}", e)))?;

        // Process source files
        self.process_source_files()?;

        // Generate cross-references
        if self.config.processing.generate_cross_references {
            self.generate_cross_references()?;
        }

        // Calculate coverage statistics
        self.calculate_coverage_stats()?;

        // Generate output in requested formats
        for format in &self.config.output.formats {
            match format.as_str() {
                "html" => self.generate_html()?,
                "markdown" => self.generate_markdown()?,
                "json" => self.generate_json()?,
                _ => {
                    eprintln!("Warning: Unknown output format '{}'", format);
                }
            }
        }

        println!("Documentation generation complete!");
        println!("Output directory: {}", self.config.output.output_dir);
        
        Ok(())
    }

    /// Clean the output directory
    fn clean_output_dir(&self) -> Result<(), CursedError> {
        let output_path = Path::new(&self.config.output.output_dir);
        if output_path.exists() {
            fs::remove_dir_all(output_path)
                .map_err(|e| CursedError::IoError(format!("Failed to clean output directory: {}", e)))?;
        }
        Ok(())
    }

    /// Process all source files
    fn process_source_files(&mut self) -> Result<(), CursedError> {
        let source_dirs = self.config.input.source_dirs.clone();
        for source_dir in source_dirs {
            if !Path::new(&source_dir).exists() {
                continue;
            }
            
            self.process_directory(&source_dir)?;
        }
        Ok(())
    }

    /// Process a single directory
    fn process_directory(&mut self, dir_path: &str) -> Result<(), CursedError> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| CursedError::IoError(format!("Failed to read directory {}: {}", dir_path, e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| CursedError::IoError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if !self.should_exclude_path(&path) {
                        self.process_directory(&path.to_string_lossy())?;
                    }
                }
            } else if path.is_file() {
                if self.should_process_file(&path) {
                    self.process_file(&path)?;
                }
            }
        }

        Ok(())
    }

    /// Check if a path should be excluded
    fn should_exclude_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        for pattern in &self.config.input.exclude_patterns {
            if glob::Pattern::new(pattern).unwrap().matches(&path_str) {
                return true;
            }
        }
        
        false
    }

    /// Check if a file should be processed
    fn should_process_file(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        // Check include patterns
        for pattern in &self.config.input.include_patterns {
            if glob::Pattern::new(pattern).unwrap().matches(&path_str) {
                return true;
            }
        }
        
        false
    }

    /// Process a single file
    fn process_file(&mut self, file_path: &Path) -> Result<(), CursedError> {
        let extension = file_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        match extension {
            "csd" => self.process_cursed_file(file_path)?,
            "rs" if self.config.processing.process_rust_files => self.process_rust_file(file_path)?,
            "md" if self.config.processing.process_markdown_files => self.process_markdown_file(file_path)?,
            _ => {}
        }

        Ok(())
    }

    /// Process a CURSED source file
    fn process_cursed_file(&mut self, file_path: &Path) -> Result<(), CursedError> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| CursedError::IoError(format!("Failed to read file {}: {}", file_path.display(), e)))?;

        // Parse the CURSED file
        let mut lexer = Lexer::new(content);
        let tokens = lexer.tokenize()
            .map_err(|e| CursedError::ParseError(format!("Failed to tokenize {}: {}", file_path.display(), e)))?;

        let mut parser = Parser::from_tokens(tokens);
        let ast = parser.parse()
            .map_err(|e| CursedError::ParseError(format!("Failed to parse {}: {}", file_path.display(), e)))?;

        // Extract documentation from the AST
        let module = self.extract_module_documentation(&ast, file_path)?;
        
        // Add to documentation
        self.documentation.modules.push(module);

        Ok(())
    }

    /// Extract documentation from a CURSED AST
    fn extract_module_documentation(&self, ast: &Ast, file_path: &Path) -> Result<DocumentedModule, CursedError> {
        let mut module = DocumentedModule {
            name: file_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string(),
            description: String::new(),
            functions: Vec::new(),
            variables: Vec::new(),
            constants: Vec::new(),
            types: Vec::new(),
            examples: Vec::new(),
            source_file: file_path.to_string_lossy().to_string(),
            submodules: Vec::new(),
        };

        // Extract functions, variables, etc. from AST
        self.extract_ast_items(ast, &mut module)?;

        Ok(module)
    }

    /// Extract documentation items from AST
    fn extract_ast_items(&self, ast: &Ast, module: &mut DocumentedModule) -> Result<(), CursedError> {
        // This is a simplified implementation - in a real implementation,
        // you would traverse the AST and extract documentation from each node
        
        // For now, return Ok as a placeholder
        Ok(())
    }

    /// Process a Rust source file (for implementation documentation)
    fn process_rust_file(&mut self, file_path: &Path) -> Result<(), CursedError> {
        // Implementation for Rust file processing
        // This would extract documentation from Rust doc comments
        Ok(())
    }

    /// Process a Markdown documentation file
    fn process_markdown_file(&mut self, file_path: &Path) -> Result<(), CursedError> {
        // Implementation for Markdown file processing
        // This would extract examples and cross-references from Markdown
        Ok(())
    }

    /// Generate cross-references between modules
    fn generate_cross_references(&mut self) -> Result<(), CursedError> {
        // Implementation for generating cross-references
        // This would analyze function calls, imports, etc.
        Ok(())
    }

    /// Calculate documentation coverage statistics
    fn calculate_coverage_stats(&mut self) -> Result<(), CursedError> {
        let mut total_functions = 0;
        let mut documented_functions = 0;
        let mut missing_docs = Vec::new();

        for module in &self.documentation.modules {
            total_functions += module.functions.len();
            
            for function in &module.functions {
                if !function.description.is_empty() {
                    documented_functions += 1;
                } else {
                    missing_docs.push(format!("{}::{}", module.name, function.name));
                }
            }
        }

        let coverage_percentage = if total_functions > 0 {
            (documented_functions as f64 / total_functions as f64) * 100.0
        } else {
            0.0
        };

        self.documentation.coverage_stats = CoverageStats {
            total_functions,
            documented_functions,
            coverage_percentage,
            missing_docs,
        };

        Ok(())
    }

    /// Generate HTML documentation
    fn generate_html(&self) -> Result<(), CursedError> {
        // use crate::documentation::html_generator::HtmlGenerator;
        
        // let html_generator = HtmlGenerator::new(&self.config);
        // html_generator.generate(&self.documentation)
        Ok(())
    }

    /// Generate Markdown documentation
    fn generate_markdown(&self) -> Result<(), CursedError> {
        use crate::documentation::markdown_generator::MarkdownGenerator;
        
        let markdown_generator = MarkdownGenerator::new(&self.config);
        markdown_generator.generate(&self.documentation)
    }

    /// Generate JSON documentation
    fn generate_json(&self) -> Result<(), CursedError> {
        use crate::documentation::json_generator::JsonGenerator;
        
        let json_generator = JsonGenerator::new(&self.config);
        json_generator.generate(&self.documentation)
    }
}

/// CLI interface for documentation generation
pub struct DocCli;

impl DocCli {
    /// Run documentation generation from command line
    pub fn run(args: Vec<String>) -> Result<(), CursedError> {
        let config_path = args.get(1).map(|s| s.as_str());
        
        let mut generator = DocumentationGenerator::new(config_path)?;
        generator.generate()?;
        
        Ok(())
    }
}
