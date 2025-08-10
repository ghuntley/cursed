//! Markdown Documentation Generator
//! 
//! Generates comprehensive Markdown documentation compatible with GitHub,
//! GitLab, and other Markdown processors.

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::error::CursedError;
use crate::documentation::{DocConfig, Documentation, DocumentedModule, DocumentedFunction};

/// Markdown documentation generator
pub struct MarkdownGenerator<'a> {
    config: &'a DocConfig,
}

impl<'a> MarkdownGenerator<'a> {
    /// Create new Markdown generator
    pub fn new(config: &'a DocConfig) -> Self {
        Self { config }
    }

    /// Generate Markdown documentation
    pub fn generate(&self, documentation: &Documentation) -> Result<(), CursedError> {
        let output_dir = Path::new(&self.config.output.output_dir);
        let markdown_dir = output_dir.join("markdown");
        
        fs::create_dir_all(&markdown_dir)
            .map_err(|e| CursedError::IoError(format!("Failed to create markdown directory: {}", e)))?;

        // Generate main README
        self.generate_readme(&markdown_dir, documentation)?;

        // Generate API reference
        self.generate_api_reference(&markdown_dir, documentation)?;

        // Generate module documentation
        for module in &documentation.modules {
            self.generate_module_docs(&markdown_dir, module)?;
        }

        // Generate examples documentation
        self.generate_examples_docs(&markdown_dir, documentation)?;

        // Generate table of contents
        self.generate_toc(&markdown_dir, documentation)?;

        Ok(())
    }

    /// Generate main README.md
    fn generate_readme(&self, markdown_dir: &Path, documentation: &Documentation) -> Result<(), CursedError> {
        let readme_file = markdown_dir.join("README.md");
        let mut file = File::create(&readme_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create README.md: {}", e)))?;

        let content = self.generate_readme_content(documentation)?;
        file.write_all(content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write README.md: {}", e)))?;

        Ok(())
    }

    /// Generate README content
    fn generate_readme_content(&self, documentation: &Documentation) -> Result<String, CursedError> {
        let mut content = String::new();

        // Project header
        content.push_str(&format!("# {}\n\n", documentation.project_info.project_name));
        content.push_str(&format!("{}\n\n", documentation.project_info.project_description));

        // Badges
        content.push_str("[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)\n");
        content.push_str("[![Version](https://img.shields.io/badge/version-");
        content.push_str(&documentation.project_info.project_version);
        content.push_str("-green.svg)](#)\n");
        content.push_str(&format!("[![Coverage](https://img.shields.io/badge/docs-{:.1}%25-brightgreen.svg)](#)\n\n", 
                                  documentation.coverage_stats.coverage_percentage));

        // Table of contents
        content.push_str("## Table of Contents\n\n");
        content.push_str("- [Overview](#overview)\n");
        content.push_str("- [Installation](#installation)\n");
        content.push_str("- [Quick Start](#quick-start)\n");
        content.push_str("- [Modules](#modules)\n");
        content.push_str("- [API Reference](#api-reference)\n");
        content.push_str("- [Examples](#examples)\n");
        content.push_str("- [Contributing](#contributing)\n");
        content.push_str("- [License](#license)\n\n");

        // Overview
        content.push_str("## Overview\n\n");
        content.push_str(&format!("{} is a comprehensive programming language with the following features:\n\n", 
                                  documentation.project_info.project_name));
        content.push_str("- **Modern Syntax**: Gen Z slang meets Go-like grammar\n");
        content.push_str("- **Self-Hosting**: Compiler written in CURSED itself\n");
        content.push_str("- **Performance**: Native compilation with LLVM backend\n");
        content.push_str("- **Safety**: Memory safety with garbage collection\n");
        content.push_str("- **Concurrency**: Built-in goroutines and channels\n\n");

        // Statistics
        content.push_str("### Project Statistics\n\n");
        content.push_str(&format!("- **Modules**: {}\n", documentation.modules.len()));
        content.push_str(&format!("- **Functions**: {}\n", documentation.coverage_stats.total_functions));
        content.push_str(&format!("- **Documentation Coverage**: {:.1}%\n", 
                                  documentation.coverage_stats.coverage_percentage));
        content.push_str(&format!("- **Examples**: {}\n\n", documentation.examples.len()));

        // Quick Start
        content.push_str("## Quick Start\n\n");
        content.push_str("### Installation\n\n");
        content.push_str("```bash\n");
        content.push_str("# Clone the repository\n");
        content.push_str(&format!("git clone {}\n", documentation.project_info.repository));
        content.push_str("cd cursed\n\n");
        content.push_str("# Build the compiler\n");
        content.push_str("cargo build --release\n");
        content.push_str("```\n\n");

        content.push_str("### Hello World\n\n");
        content.push_str("```cursed\n");
        content.push_str("fr fr Hello World example\n");
        content.push_str("vibez.spill(\"Hello, world!\")\n");
        content.push_str("```\n\n");

        content.push_str("Run with:\n");
        content.push_str("```bash\n");
        content.push_str("cargo run --bin cursed hello.csd\n");
        content.push_str("```\n\n");

        // Modules section
        content.push_str("## Modules\n\n");
        content.push_str("| Module | Description | Functions |\n");
        content.push_str("|--------|-------------|----------|\n");

        for module in &documentation.modules {
            content.push_str(&format!("| [{}](modules/{}.md) | {} | {} |\n",
                                      module.name,
                                      module.name,
                                      module.description,
                                      module.functions.len()));
        }
        content.push_str("\n");

        // API Reference
        content.push_str("## API Reference\n\n");
        content.push_str("Complete API documentation is available:\n\n");
        content.push_str("- [HTML Documentation](../html/index.html)\n");
        content.push_str("- [JSON API](../api.json)\n");
        content.push_str("- [Module Index](modules/README.md)\n\n");

        // Examples
        content.push_str("## Examples\n\n");
        content.push_str("See the [examples directory](examples/) for comprehensive examples:\n\n");
        
        for example in &documentation.examples {
            content.push_str(&format!("- [{}](examples/{}.md) - {}\n",
                                      example.title,
                                      example.title.replace(" ", "_").to_lowercase(),
                                      example.description));
        }
        content.push_str("\n");

        // Contributing
        content.push_str("## Contributing\n\n");
        content.push_str("Contributions are welcome! Please see our [contributing guidelines](CONTRIBUTING.md).\n\n");

        // License
        content.push_str("## License\n\n");
        content.push_str(&format!("This project is licensed under the {} License - see the [LICENSE](LICENSE) file for details.\n\n",
                                  documentation.project_info.license));

        // Authors
        content.push_str("## Authors\n\n");
        for author in &documentation.project_info.authors {
            content.push_str(&format!("- {}\n", author));
        }
        content.push_str("\n");

        // Links
        content.push_str("## Links\n\n");
        content.push_str(&format!("- [Repository]({})\n", documentation.project_info.repository));
        content.push_str(&format!("- [Project Website]({})\n", documentation.project_info.project_url));
        content.push_str("- [Documentation](docs/)\n");
        content.push_str("- [Examples](examples/)\n");

        Ok(content)
    }

    /// Generate API reference
    fn generate_api_reference(&self, markdown_dir: &Path, documentation: &Documentation) -> Result<(), CursedError> {
        let api_file = markdown_dir.join("API.md");
        let mut file = File::create(&api_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create API.md: {}", e)))?;

        let content = self.generate_api_content(documentation)?;
        file.write_all(content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write API.md: {}", e)))?;

        Ok(())
    }

    /// Generate API reference content
    fn generate_api_content(&self, documentation: &Documentation) -> Result<String, CursedError> {
        let mut content = String::new();

        content.push_str("# API Reference\n\n");
        content.push_str("Complete API reference for all modules and functions.\n\n");

        // Table of contents
        content.push_str("## Table of Contents\n\n");
        for module in &documentation.modules {
            content.push_str(&format!("- [{}](#{}-module)\n", module.name, module.name.to_lowercase()));
        }
        content.push_str("\n");

        // Module documentation
        for module in &documentation.modules {
            content.push_str(&self.generate_module_api_content(module)?);
        }

        Ok(content)
    }

    /// Generate module API content
    fn generate_module_api_content(&self, module: &DocumentedModule) -> Result<String, CursedError> {
        let mut content = String::new();

        content.push_str(&format!("## {} Module\n\n", module.name));
        content.push_str(&format!("{}\n\n", module.description));

        if !module.functions.is_empty() {
            content.push_str("### Functions\n\n");
            
            for function in &module.functions {
                content.push_str(&self.generate_function_markdown(function)?);
            }
        }

        if !module.variables.is_empty() {
            content.push_str("### Variables\n\n");
            
            for variable in &module.variables {
                content.push_str(&format!("#### `{}`\n\n", variable.name));
                content.push_str(&format!("**Type**: `{}`\n\n", variable.var_type));
                content.push_str(&format!("{}\n\n", variable.description));
            }
        }

        if !module.constants.is_empty() {
            content.push_str("### Constants\n\n");
            
            for constant in &module.constants {
                content.push_str(&format!("#### `{}`\n\n", constant.name));
                content.push_str(&format!("**Type**: `{}`\n\n", constant.const_type));
                content.push_str(&format!("**Value**: `{}`\n\n", constant.value));
                content.push_str(&format!("{}\n\n", constant.description));
            }
        }

        content.push_str("---\n\n");

        Ok(content)
    }

    /// Generate function markdown
    fn generate_function_markdown(&self, function: &DocumentedFunction) -> Result<String, CursedError> {
        let mut content = String::new();

        content.push_str(&format!("#### `{}`\n\n", function.name));
        content.push_str(&format!("```cursed\n{}\n```\n\n", function.signature));
        content.push_str(&format!("{}\n\n", function.description));

        if !function.parameters.is_empty() {
            content.push_str("**Parameters:**\n\n");
            for param in &function.parameters {
                content.push_str(&format!("- `{}` (`{}`): {}\n", 
                                          param.name, 
                                          param.param_type, 
                                          param.description));
            }
            content.push_str("\n");
        }

        if !function.return_type.is_empty() {
            content.push_str(&format!("**Returns:** `{}` - {}\n\n", 
                                      function.return_type, 
                                      function.return_description));
        }

        if !function.examples.is_empty() {
            content.push_str("**Examples:**\n\n");
            for example in &function.examples {
                content.push_str(&format!("```cursed\n{}\n```\n\n", example));
            }
        }

        content.push_str(&format!("*Source: [{}:{}]({}#L{})*\n\n", 
                                  function.source_file,
                                  function.source_line,
                                  function.source_file,
                                  function.source_line));

        Ok(content)
    }

    /// Generate module documentation
    fn generate_module_docs(&self, markdown_dir: &Path, module: &DocumentedModule) -> Result<(), CursedError> {
        let modules_dir = markdown_dir.join("modules");
        fs::create_dir_all(&modules_dir)
            .map_err(|e| CursedError::IoError(format!("Failed to create modules directory: {}", e)))?;

        let module_file = modules_dir.join(format!("{}.md", module.name));
        let mut file = File::create(&module_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create module file: {}", e)))?;

        let content = self.generate_module_content(module)?;
        file.write_all(content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write module file: {}", e)))?;

        Ok(())
    }

    /// Generate module content
    fn generate_module_content(&self, module: &DocumentedModule) -> Result<String, CursedError> {
        let mut content = String::new();

        content.push_str(&format!("# {} Module\n\n", module.name));
        content.push_str(&format!("{}\n\n", module.description));

        // Module information
        content.push_str("## Module Information\n\n");
        content.push_str(&format!("- **Source File**: `{}`\n", module.source_file));
        content.push_str(&format!("- **Functions**: {}\n", module.functions.len()));
        content.push_str(&format!("- **Variables**: {}\n", module.variables.len()));
        content.push_str(&format!("- **Constants**: {}\n", module.constants.len()));
        content.push_str(&format!("- **Types**: {}\n\n", module.types.len()));

        // Table of contents
        content.push_str("## Table of Contents\n\n");
        if !module.functions.is_empty() {
            content.push_str("- [Functions](#functions)\n");
        }
        if !module.variables.is_empty() {
            content.push_str("- [Variables](#variables)\n");
        }
        if !module.constants.is_empty() {
            content.push_str("- [Constants](#constants)\n");
        }
        if !module.types.is_empty() {
            content.push_str("- [Types](#types)\n");
        }
        content.push_str("\n");

        // Functions
        if !module.functions.is_empty() {
            content.push_str("## Functions\n\n");
            for function in &module.functions {
                content.push_str(&self.generate_function_markdown(function)?);
            }
        }

        // Variables
        if !module.variables.is_empty() {
            content.push_str("## Variables\n\n");
            for variable in &module.variables {
                content.push_str(&format!("### `{}`\n\n", variable.name));
                content.push_str(&format!("**Type**: `{}`\n\n", variable.var_type));
                content.push_str(&format!("{}\n\n", variable.description));
            }
        }

        // Constants
        if !module.constants.is_empty() {
            content.push_str("## Constants\n\n");
            for constant in &module.constants {
                content.push_str(&format!("### `{}`\n\n", constant.name));
                content.push_str(&format!("**Type**: `{}`\n\n", constant.const_type));
                content.push_str(&format!("**Value**: `{}`\n\n", constant.value));
                content.push_str(&format!("{}\n\n", constant.description));
            }
        }

        // Types
        if !module.types.is_empty() {
            content.push_str("## Types\n\n");
            for doc_type in &module.types {
                content.push_str(&format!("### `{}`\n\n", doc_type.name));
                content.push_str(&format!("**Kind**: {}\n\n", doc_type.type_kind));
                content.push_str(&format!("{}\n\n", doc_type.description));
                
                if !doc_type.fields.is_empty() {
                    content.push_str("**Fields:**\n\n");
                    for field in &doc_type.fields {
                        content.push_str(&format!("- `{}` (`{}`): {}\n", 
                                                  field.name, 
                                                  field.field_type, 
                                                  field.description));
                    }
                    content.push_str("\n");
                }
            }
        }

        Ok(content)
    }

    /// Generate examples documentation
    fn generate_examples_docs(&self, markdown_dir: &Path, documentation: &Documentation) -> Result<(), CursedError> {
        let examples_dir = markdown_dir.join("examples");
        fs::create_dir_all(&examples_dir)
            .map_err(|e| CursedError::IoError(format!("Failed to create examples directory: {}", e)))?;

        // Generate examples index
        let examples_index = examples_dir.join("README.md");
        let mut file = File::create(&examples_index)
            .map_err(|e| CursedError::IoError(format!("Failed to create examples README: {}", e)))?;

        let content = self.generate_examples_index_content(documentation)?;
        file.write_all(content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write examples README: {}", e)))?;

        // Generate individual example files
        for example in &documentation.examples {
            let example_file = examples_dir.join(format!("{}.md", 
                                                          example.title.replace(" ", "_").to_lowercase()));
            let mut file = File::create(&example_file)
                .map_err(|e| CursedError::IoError(format!("Failed to create example file: {}", e)))?;

            let content = self.generate_example_content(example)?;
            file.write_all(content.as_bytes())
                .map_err(|e| CursedError::IoError(format!("Failed to write example file: {}", e)))?;
        }

        Ok(())
    }

    /// Generate examples index content
    fn generate_examples_index_content(&self, documentation: &Documentation) -> Result<String, CursedError> {
        let mut content = String::new();

        content.push_str("# Examples\n\n");
        content.push_str("Comprehensive examples demonstrating CURSED language features.\n\n");

        // Group examples by category
        let mut categories = std::collections::HashMap::new();
        for example in &documentation.examples {
            categories.entry(example.category.clone()).or_insert(Vec::new()).push(example);
        }

        for (category, examples) in categories {
            content.push_str(&format!("## {}\n\n", category));
            
            for example in examples {
                content.push_str(&format!("- [{}]({}.md) - {}\n", 
                                          example.title,
                                          example.title.replace(" ", "_").to_lowercase(),
                                          example.description));
            }
            content.push_str("\n");
        }

        Ok(content)
    }

    /// Generate example content
    fn generate_example_content(&self, example: &crate::documentation::DocumentedExample) -> Result<String, CursedError> {
        let mut content = String::new();

        content.push_str(&format!("# {}\n\n", example.title));
        content.push_str(&format!("{}\n\n", example.description));

        content.push_str("## Code\n\n");
        content.push_str(&format!("```cursed\n{}\n```\n\n", example.code));

        if let Some(output) = &example.output {
            content.push_str("## Output\n\n");
            content.push_str(&format!("```\n{}\n```\n\n", output));
        }

        content.push_str(&format!("**Category**: {}\n\n", example.category));
        content.push_str(&format!("**Source**: [{}]({})\n", example.source_file, example.source_file));

        Ok(content)
    }

    /// Generate table of contents
    fn generate_toc(&self, markdown_dir: &Path, documentation: &Documentation) -> Result<(), CursedError> {
        let toc_file = markdown_dir.join("TOC.md");
        let mut file = File::create(&toc_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create TOC.md: {}", e)))?;

        let content = self.generate_toc_content(documentation)?;
        file.write_all(content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write TOC.md: {}", e)))?;

        Ok(())
    }

    /// Generate table of contents content
    fn generate_toc_content(&self, documentation: &Documentation) -> Result<String, CursedError> {
        let mut content = String::new();

        content.push_str("# Table of Contents\n\n");
        content.push_str("Complete documentation index for the CURSED programming language.\n\n");

        content.push_str("## Documentation Structure\n\n");
        content.push_str("- [README](README.md) - Project overview and quick start\n");
        content.push_str("- [API Reference](API.md) - Complete API documentation\n");
        content.push_str("- [Modules](modules/) - Individual module documentation\n");
        content.push_str("- [Examples](examples/) - Code examples and tutorials\n\n");

        content.push_str("## Modules\n\n");
        for module in &documentation.modules {
            content.push_str(&format!("- [{}](modules/{}.md) - {}\n", 
                                      module.name, 
                                      module.name, 
                                      module.description));
        }
        content.push_str("\n");

        content.push_str("## Examples\n\n");
        for example in &documentation.examples {
            content.push_str(&format!("- [{}](examples/{}.md) - {}\n", 
                                      example.title,
                                      example.title.replace(" ", "_").to_lowercase(),
                                      example.description));
        }

        Ok(content)
    }
}
