//! Markdown documentation generator for the CURSED programming language
//!
//! This module provides comprehensive Markdown generation capabilities that convert
//! extracted documentation into GitHub-flavored Markdown with features like:
//! - Table of contents generation
//! - Cross-reference linking
//! - CURSED syntax highlighting
//! - Multiple output formats

use crate::docs::{DocumentationItem, ItemType, PackageDocumentation, DocResult, DocError, ModuleInfo};
use std::collections::{HashMap, BTreeMap};
use std::fmt::Write;
use std::path::{Path, PathBuf};

/// Markdown output format configuration
#[derive(Debug, Clone)]
pub enum MarkdownFormat {
    /// Single comprehensive file containing all documentation
    SingleFile {
        /// Include table of contents
        include_toc: bool,
        /// Maximum heading depth for TOC
        toc_depth: usize,
    },
    /// Multiple files with index
    MultiFile {
        /// Generate index file
        generate_index: bool,
        /// Organize by module
        organize_by_module: bool,
    },
    /// Package README generation
    ReadmeFile {
        /// Include quick start section
        include_quickstart: bool,
        /// Include examples
        include_examples: bool,
    },
    /// API reference documentation
    ApiReference {
        /// Group by item type
        group_by_type: bool,
        /// Include private items
        include_private: bool,
    },
}

/// Markdown generation configuration
#[derive(Debug, Clone)]
pub struct MarkdownConfig {
    /// Output format
    pub format: MarkdownFormat,
    /// Base URL for cross-references
    pub base_url: Option<String>,
    /// Include source links
    pub include_source_links: bool,
    /// Enable CURSED syntax highlighting
    pub enable_syntax_highlighting: bool,
    /// Custom CSS classes for styling
    pub css_classes: HashMap<String, String>,
    /// Include deprecated items
    pub include_deprecated: bool,
    /// Custom templates directory
    pub templates_dir: Option<PathBuf>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            format: MarkdownFormat::SingleFile {
                include_toc: true,
                toc_depth: 3,
            },
            base_url: None,
            include_source_links: true,
            enable_syntax_highlighting: true,
            css_classes: HashMap::new(),
            include_deprecated: true,
            templates_dir: None,
            metadata: HashMap::new(),
        }
    }
}

/// Generated markdown content with metadata
#[derive(Debug, Clone)]
pub struct MarkdownOutput {
    /// Generated markdown content
    pub content: String,
    /// Table of contents (if generated)
    pub table_of_contents: Option<String>,
    /// Cross-reference links
    pub cross_references: HashMap<String, String>,
    /// Generated file paths (for multi-file output)
    pub file_paths: Vec<PathBuf>,
    /// Metadata information
    pub metadata: HashMap<String, String>,
}

/// Markdown generator for CURSED documentation
pub struct MarkdownGenerator {
    config: MarkdownConfig,
    cross_references: HashMap<String, String>,
    heading_counter: BTreeMap<String, usize>,
}

impl MarkdownGenerator {
    /// Create a new markdown generator
    pub fn new(config: MarkdownConfig) -> Self {
        Self {
            config,
            cross_references: HashMap::new(),
            heading_counter: BTreeMap::new(),
        }
    }

    /// Generate markdown documentation from package documentation
    pub fn generate(&mut self, package_docs: &PackageDocumentation) -> DocResult<MarkdownOutput> {
        // Build cross-reference index
        self.build_cross_reference_index(package_docs);

        match &self.config.format {
            MarkdownFormat::SingleFile { include_toc, toc_depth } => {
                self.generate_single_file(package_docs, *include_toc, *toc_depth)
            }
            MarkdownFormat::MultiFile { generate_index, organize_by_module } => {
                self.generate_multi_file(package_docs, *generate_index, *organize_by_module)
            }
            MarkdownFormat::ReadmeFile { include_quickstart, include_examples } => {
                self.generate_readme(package_docs, *include_quickstart, *include_examples)
            }
            MarkdownFormat::ApiReference { group_by_type, include_private } => {
                self.generate_api_reference(package_docs, *group_by_type, *include_private)
            }
        }
    }

    /// Build cross-reference index for linking
    fn build_cross_reference_index(&mut self, package_docs: &PackageDocumentation) {
        self.build_cross_reference_index_module(&package_docs.root_module);
    }

    /// Build cross-reference index for a module recursively
    fn build_cross_reference_index_module(&mut self, module: &ModuleInfo) {
        for item in &module.exports {
            let anchor = self.generate_anchor(&item.name, &item.item_type);
            let link = format!("#{}", anchor);
            self.cross_references.insert(item.name.clone(), link);
            
            // Add methods to cross-reference index
            for method in &item.methods {
                let method_anchor = self.generate_anchor(&format!("{}.{}", item.name, method.name), &method.item_type);
                let method_link = format!("#{}", method_anchor);
                self.cross_references.insert(format!("{}.{}", item.name, method.name), method_link);
            }
        }

        // Process submodules
        for submodule in &module.submodules {
            self.build_cross_reference_index_module(submodule);
        }
    }

    /// Generate single comprehensive file
    fn generate_single_file(&mut self, package_docs: &PackageDocumentation, include_toc: bool, toc_depth: usize) -> DocResult<MarkdownOutput> {
        let mut content = String::new();
        let mut toc = String::new();

        // Header
        writeln!(content, "# {} Documentation", package_docs.name).unwrap();
        writeln!(content).unwrap();

        if let Some(description) = &package_docs.description {
            writeln!(content, "{}", description).unwrap();
            writeln!(content).unwrap();
        }

        // Table of contents
        if include_toc {
            toc = self.generate_table_of_contents(package_docs, toc_depth);
            writeln!(content, "{}", toc).unwrap();
        }

        // Package overview
        self.write_package_overview(&mut content, package_docs)?;

        // Modules
        self.write_modules_recursively(&mut content, &package_docs.root_module)?;

        // Index section
        self.write_index_section(&mut content, package_docs)?;

        Ok(MarkdownOutput {
            content,
            table_of_contents: if include_toc { Some(toc) } else { None },
            cross_references: self.cross_references.clone(),
            file_paths: vec![PathBuf::from("documentation.md")],
            metadata: self.config.metadata.clone(),
        })
    }

    /// Generate multiple files with index
    fn generate_multi_file(&mut self, package_docs: &PackageDocumentation, generate_index: bool, organize_by_module: bool) -> DocResult<MarkdownOutput> {
        let mut content = String::new();
        let mut file_paths = Vec::new();

        if generate_index {
            // Generate index file
            writeln!(content, "# {} Documentation Index", package_docs.name).unwrap();
            writeln!(content).unwrap();

            if let Some(description) = &package_docs.description {
                writeln!(content, "{}", description).unwrap();
                writeln!(content).unwrap();
            }

            writeln!(content, "## Modules").unwrap();
            writeln!(content).unwrap();

            self.write_module_index(&mut content, &package_docs.root_module, organize_by_module)?;

            file_paths.push(PathBuf::from("index.md"));
        }

        // Generate module files
        if organize_by_module {
            self.generate_module_files(&package_docs.root_module, &mut file_paths)?;
        } else {
            let mut api_content = String::new();
            writeln!(api_content, "# API Reference").unwrap();
            writeln!(api_content).unwrap();

            self.write_modules_recursively(&mut api_content, &package_docs.root_module)?;
            
            file_paths.push(PathBuf::from("api.md"));
        }

        Ok(MarkdownOutput {
            content,
            table_of_contents: None,
            cross_references: self.cross_references.clone(),
            file_paths,
            metadata: self.config.metadata.clone(),
        })
    }

    /// Generate README file
    fn generate_readme(&mut self, package_docs: &PackageDocumentation, include_quickstart: bool, include_examples: bool) -> DocResult<MarkdownOutput> {
        let mut content = String::new();

        // Header with badge
        writeln!(content, "# {} ✨", package_docs.name).unwrap();
        writeln!(content).unwrap();

        if let Some(description) = &package_docs.description {
            writeln!(content, "{}", description).unwrap();
            writeln!(content).unwrap();
        }

        // Quick start section
        if include_quickstart {
            writeln!(content, "## Quick Start 🚀").unwrap();
            writeln!(content).unwrap();
            writeln!(content, "```bash").unwrap();
            writeln!(content, "# Build the project").unwrap();
            writeln!(content, "make build").unwrap();
            writeln!(content).unwrap();
            writeln!(content, "# Run tests").unwrap();
            writeln!(content, "make test").unwrap();
            writeln!(content, "```").unwrap();
            writeln!(content).unwrap();
        }

        // Examples section
        if include_examples {
            writeln!(content, "## Examples 💡").unwrap();
            writeln!(content).unwrap();

            self.write_examples_from_module(&mut content, &package_docs.root_module)?;
        }

        // API overview
        writeln!(content, "## API Overview 📚").unwrap();
        writeln!(content).unwrap();

        self.write_api_overview_from_module(&mut content, &package_docs.root_module)?;

        Ok(MarkdownOutput {
            content,
            table_of_contents: None,
            cross_references: self.cross_references.clone(),
            file_paths: vec![PathBuf::from("README.md")],
            metadata: self.config.metadata.clone(),
        })
    }

    /// Generate API reference documentation
    fn generate_api_reference(&mut self, package_docs: &PackageDocumentation, group_by_type: bool, include_private: bool) -> DocResult<MarkdownOutput> {
        let mut content = String::new();

        writeln!(content, "# {} API Reference", package_docs.name).unwrap();
        writeln!(content).unwrap();

        if let Some(description) = &package_docs.description {
            writeln!(content, "{}", description).unwrap();
            writeln!(content).unwrap();
        }

        if group_by_type {
            self.generate_api_reference_by_type(&mut content, &package_docs.root_module, include_private)?;
        } else {
            self.generate_api_reference_by_module(&mut content, &package_docs.root_module, include_private)?;
        }

        Ok(MarkdownOutput {
            content,
            table_of_contents: None,
            cross_references: self.cross_references.clone(),
            file_paths: vec![PathBuf::from("api.md")],
            metadata: self.config.metadata.clone(),
        })
    }

    /// Generate table of contents
    fn generate_table_of_contents(&self, package_docs: &PackageDocumentation, max_depth: usize) -> String {
        let mut toc = String::new();
        
        writeln!(toc, "## Table of Contents").unwrap();
        writeln!(toc).unwrap();

        // Package overview
        if max_depth >= 1 {
            writeln!(toc, "- [Package Overview](#package-overview)").unwrap();
        }

        // Modules
        if max_depth >= 1 {
            writeln!(toc, "- [Modules](#modules)").unwrap();
            
            if max_depth >= 2 {
                // Start with root module and traverse
                self.write_toc_module(&mut toc, &package_docs.root_module, max_depth, 2);
            }
        }

        // Index
        if max_depth >= 1 {
            writeln!(toc, "- [Index](#index)").unwrap();
        }

        writeln!(toc).unwrap();
        toc
    }

    /// Write table of contents module entry
    fn write_toc_module(&self, toc: &mut String, module: &ModuleInfo, max_depth: usize, current_depth: usize) {
        if current_depth > max_depth {
            return;
        }

        let indent = "  ".repeat(current_depth - 1);
        let module_anchor = self.generate_anchor(&module.name, &ItemType::Module);
        writeln!(toc, "{}- [{}](#{}) {}", indent, module.name, module_anchor, 
            module.documentation.as_deref().unwrap_or("")).unwrap();
        
        if current_depth < max_depth {
            for item in &module.exports {
                let item_anchor = self.generate_anchor(&item.name, &item.item_type);
                writeln!(toc, "{}  - [{}](#{}) ({})", indent, item.name, item_anchor, item.item_type).unwrap();
            }
        }

        // Recursively write submodules
        for submodule in &module.submodules {
            self.write_toc_module(toc, submodule, max_depth, current_depth + 1);
        }
    }



    /// Write module section
    fn write_module_section(&mut self, content: &mut String, module: &ModuleInfo) -> DocResult<()> {
        let module_anchor = self.generate_anchor(&module.name, &ItemType::Module);
        writeln!(content, "## Module: {} {{#{}}}", module.name, module_anchor).unwrap();
        writeln!(content).unwrap();

        if let Some(description) = &module.documentation {
            writeln!(content, "{}", description).unwrap();
            writeln!(content).unwrap();
        }

        if self.config.include_source_links {
            if let Some(base_url) = &self.config.base_url {
                writeln!(content, "**Source**: [{}]({}/{})", module.path.display(), base_url, module.path.display()).unwrap();
                writeln!(content).unwrap();
            }
        }

        // Group items by type for better organization
        let mut items_by_type: HashMap<ItemType, Vec<&DocumentationItem>> = HashMap::new();
        
        for item in &module.exports {
            if !self.config.include_deprecated && item.is_deprecated() {
                continue;
            }
            
            items_by_type.entry(item.item_type.clone())
                .or_insert_with(Vec::new)
                .push(item);
        }

        // Write each type section
        for (item_type, items) in items_by_type {
            if items.is_empty() {
                continue;
            }

            writeln!(content, "### {}s", self.capitalize(&item_type.to_string())).unwrap();
            writeln!(content).unwrap();

            for item in items {
                self.write_item_documentation(content, item, 4)?;
            }
        }

        Ok(())
    }

    /// Write individual item documentation
    fn write_item_documentation(&mut self, content: &mut String, item: &DocumentationItem, heading_level: usize) -> DocResult<()> {
        let heading_prefix = "#".repeat(heading_level);
        let item_anchor = self.generate_anchor(&item.name, &item.item_type);
        
        // Item header
        writeln!(content, "{} {} `{}` {{#{}}}", heading_prefix, item.item_type, item.name, item_anchor).unwrap();
        writeln!(content).unwrap();

        // Deprecated warning
        if item.is_deprecated() {
            writeln!(content, "> ⚠️ **Deprecated**: This item is deprecated and may be removed in future versions.").unwrap();
            writeln!(content).unwrap();
        }

        // Description
        if let Some(description) = item.description() {
            writeln!(content, "{}", description).unwrap();
            writeln!(content).unwrap();
        }

        // Signature (for functions)
        if let Some(signature) = &item.signature {
            writeln!(content, "**Signature:**").unwrap();
            writeln!(content).unwrap();
            if self.config.enable_syntax_highlighting {
                writeln!(content, "```cursed").unwrap();
            } else {
                writeln!(content, "```").unwrap();
            }
            writeln!(content, "{}", signature).unwrap();
            writeln!(content, "```").unwrap();
            writeln!(content).unwrap();
        }

        // Parameters (for functions)
        if !item.parameters.is_empty() {
            writeln!(content, "**Parameters:**").unwrap();
            writeln!(content).unwrap();

            let param_descriptions = item.parameter_descriptions();
            
            for param in &item.parameters {
                let description = param_descriptions.get(&param.name)
                    .or(param.description.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("No description");
                
                writeln!(content, "- `{}` ({}): {}", param.name, param.param_type, description).unwrap();
            }
            writeln!(content).unwrap();
        }

        // Return type (for functions)
        if let Some(return_type) = &item.return_type {
            writeln!(content, "**Returns:** `{}`", return_type).unwrap();
            
            if let Some(return_desc) = item.return_description() {
                writeln!(content, " - {}", return_desc).unwrap();
            }
            writeln!(content).unwrap();
        }

        // Fields (for structs/interfaces)
        if !item.fields.is_empty() {
            writeln!(content, "**Fields:**").unwrap();
            writeln!(content).unwrap();

            writeln!(content, "| Field | Type | Visibility | Description |").unwrap();
            writeln!(content, "|-------|------|------------|-------------|").unwrap();

            for field in &item.fields {
                let description = field.description.as_deref().unwrap_or("No description");
                writeln!(content, "| `{}` | `{}` | {} | {} |", 
                    field.name, field.field_type, field.visibility, description).unwrap();
            }
            writeln!(content).unwrap();
        }

        // Methods (for structs/interfaces)
        if !item.methods.is_empty() {
            writeln!(content, "**Methods:**").unwrap();
            writeln!(content).unwrap();

            for method in &item.methods {
                self.write_item_documentation(content, method, heading_level + 1)?;
            }
        }

        // Examples
        if !item.examples.is_empty() {
            writeln!(content, "**Examples:**").unwrap();
            writeln!(content).unwrap();

            for (i, example) in item.examples.iter().enumerate() {
                if item.examples.len() > 1 {
                    writeln!(content, "Example {}:", i + 1).unwrap();
                    writeln!(content).unwrap();
                }

                if self.config.enable_syntax_highlighting {
                    writeln!(content, "```cursed").unwrap();
                } else {
                    writeln!(content, "```").unwrap();
                }
                writeln!(content, "{}", example).unwrap();
                writeln!(content, "```").unwrap();
                writeln!(content).unwrap();
            }
        }

        // Source link
        if self.config.include_source_links {
            if let Some(base_url) = &self.config.base_url {
                writeln!(content, "*Source: Line {} in [source]({}#L{})*", 
                    item.line, base_url, item.line).unwrap();
                writeln!(content).unwrap();
            }
        }

        writeln!(content, "---").unwrap();
        writeln!(content).unwrap();

        Ok(())
    }



    /// Generate anchor for cross-references
    fn generate_anchor(&self, name: &str, item_type: &ItemType) -> String {
        let normalized = name.to_lowercase()
            .replace(' ', "-")
            .replace('.', "")
            .replace('_', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        
        format!("{}-{}", item_type.to_string().to_lowercase(), normalized)
    }

    /// Capitalize first letter
    fn capitalize(&self, s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    /// Write modules recursively
    fn write_modules_recursively(&mut self, content: &mut String, module: &ModuleInfo) -> DocResult<()> {
        if !module.exports.is_empty() {
            self.write_module_section(content, module)?;
        }

        for submodule in &module.submodules {
            self.write_modules_recursively(content, submodule)?;
        }
        
        Ok(())
    }

    /// Write module index for multi-file generation
    fn write_module_index(&mut self, content: &mut String, module: &ModuleInfo, organize_by_module: bool) -> DocResult<()> {
        if !module.exports.is_empty() {
            let module_file = if organize_by_module {
                format!("{}.md", module.name)
            } else {
                "api.md".to_string()
            };
            
            writeln!(content, "- [{}]({}) - {}", 
                module.name, 
                module_file,
                module.documentation.as_deref().unwrap_or("No description")
            ).unwrap();
        }

        for submodule in &module.submodules {
            self.write_module_index(content, submodule, organize_by_module)?;
        }
        
        Ok(())
    }

    /// Generate module files recursively
    fn generate_module_files(&mut self, module: &ModuleInfo, file_paths: &mut Vec<PathBuf>) -> DocResult<()> {
        if !module.exports.is_empty() {
            let mut module_content = String::new();
            self.write_module_section(&mut module_content, module)?;
            file_paths.push(PathBuf::from(format!("{}.md", module.name)));
        }

        for submodule in &module.submodules {
            self.generate_module_files(submodule, file_paths)?;
        }
        
        Ok(())
    }

    /// Write examples from module recursively
    fn write_examples_from_module(&mut self, content: &mut String, module: &ModuleInfo) -> DocResult<()> {
        // Find example functions/items in this module
        for item in &module.exports {
            if !item.examples.is_empty() {
                writeln!(content, "### {}", item.name).unwrap();
                writeln!(content).unwrap();
                
                if let Some(description) = item.description() {
                    writeln!(content, "{}", description).unwrap();
                    writeln!(content).unwrap();
                }

                for example in &item.examples {
                    if self.config.enable_syntax_highlighting {
                        writeln!(content, "```cursed").unwrap();
                    } else {
                        writeln!(content, "```").unwrap();
                    }
                    writeln!(content, "{}", example).unwrap();
                    writeln!(content, "```").unwrap();
                    writeln!(content).unwrap();
                }
            }
        }

        for submodule in &module.submodules {
            self.write_examples_from_module(content, submodule)?;
        }
        
        Ok(())
    }

    /// Write API overview from module recursively
    fn write_api_overview_from_module(&mut self, content: &mut String, module: &ModuleInfo) -> DocResult<()> {
        if !module.exports.is_empty() {
            writeln!(content, "### {}", module.name).unwrap();
            writeln!(content).unwrap();

            if let Some(description) = &module.documentation {
                writeln!(content, "{}", description).unwrap();
                writeln!(content).unwrap();
            }

            let public_items: Vec<_> = module.exports.iter()
                .filter(|item| item.visibility == "public")
                .collect();

            if !public_items.is_empty() {
                writeln!(content, "| Item | Type | Description |").unwrap();
                writeln!(content, "|------|------|-------------|").unwrap();

                for item in public_items {
                    let description = item.description().unwrap_or("No description");
                    writeln!(content, "| `{}` | {} | {} |", 
                        item.name, 
                        item.item_type, 
                        description.split('\n').next().unwrap_or("")
                    ).unwrap();
                }
                writeln!(content).unwrap();
            }
        }

        for submodule in &module.submodules {
            self.write_api_overview_from_module(content, submodule)?;
        }
        
        Ok(())
    }

    /// Generate API reference grouped by type
    fn generate_api_reference_by_type(&mut self, content: &mut String, module: &ModuleInfo, include_private: bool) -> DocResult<()> {
        // Group items by type
        let mut items_by_type: HashMap<ItemType, Vec<&DocumentationItem>> = HashMap::new();
        self.collect_items_by_type(module, &mut items_by_type, include_private);

        // Write each type section
        for (item_type, items) in items_by_type {
            writeln!(content, "## {}s", self.capitalize(&item_type.to_string())).unwrap();
            writeln!(content).unwrap();

            for item in items {
                self.write_item_documentation(content, item, 3)?;
            }
        }
        
        Ok(())
    }

    /// Generate API reference grouped by module
    fn generate_api_reference_by_module(&mut self, content: &mut String, module: &ModuleInfo, include_private: bool) -> DocResult<()> {
        if !module.exports.is_empty() {
            writeln!(content, "## Module: {}", module.name).unwrap();
            writeln!(content).unwrap();

            for item in &module.exports {
                if !include_private && item.visibility != "public" {
                    continue;
                }
                
                self.write_item_documentation(content, item, 3)?;
            }
        }

        for submodule in &module.submodules {
            self.generate_api_reference_by_module(content, submodule, include_private)?;
        }
        
        Ok(())
    }

    /// Collect items by type recursively
    fn collect_items_by_type<'a>(&self, module: &'a ModuleInfo, items_by_type: &mut HashMap<ItemType, Vec<&'a DocumentationItem>>, include_private: bool) {
        for item in &module.exports {
            if !include_private && item.visibility != "public" {
                continue;
            }
            
            items_by_type.entry(item.item_type.clone())
                .or_insert_with(Vec::new)
                .push(item);
        }

        for submodule in &module.submodules {
            self.collect_items_by_type(submodule, items_by_type, include_private);
        }
    }

    /// Update write_package_overview to work with new structure
    fn write_package_overview(&mut self, content: &mut String, package_docs: &PackageDocumentation) -> DocResult<()> {
        writeln!(content, "## Package Overview").unwrap();
        writeln!(content).unwrap();

        if let Some(description) = &package_docs.description {
            writeln!(content, "{}", description).unwrap();
            writeln!(content).unwrap();
        }

        // Statistics
        let total_modules = self.count_modules(&package_docs.root_module);
        let total_items = package_docs.root_module.item_count();

        writeln!(content, "### Statistics").unwrap();
        writeln!(content).unwrap();
        writeln!(content, "- **Modules**: {}", total_modules).unwrap();
        writeln!(content, "- **Total Items**: {}", total_items).unwrap();
        
        // Count by type
        let mut type_counts: HashMap<ItemType, usize> = HashMap::new();
        self.count_types_recursively(&package_docs.root_module, &mut type_counts);

        for (item_type, count) in type_counts {
            writeln!(content, "- **{}s**: {}", self.capitalize(&item_type.to_string()), count).unwrap();
        }

        writeln!(content).unwrap();
        Ok(())
    }

    /// Count modules recursively
    fn count_modules(&self, module: &ModuleInfo) -> usize {
        1 + module.submodules.iter().map(|m| self.count_modules(m)).sum::<usize>()
    }

    /// Count types recursively
    fn count_types_recursively(&self, module: &ModuleInfo, type_counts: &mut HashMap<ItemType, usize>) {
        for item in &module.exports {
            *type_counts.entry(item.item_type.clone()).or_insert(0) += 1;
        }

        for submodule in &module.submodules {
            self.count_types_recursively(submodule, type_counts);
        }
    }

    /// Update write_index_section to work with new structure
    fn write_index_section(&mut self, content: &mut String, package_docs: &PackageDocumentation) -> DocResult<()> {
        writeln!(content, "## Index").unwrap();
        writeln!(content).unwrap();

        // Collect all items
        let mut all_items: Vec<(&str, &DocumentationItem)> = Vec::new();
        self.collect_all_items_with_module(&package_docs.root_module, &mut all_items);

        // Sort alphabetically
        all_items.sort_by(|a, b| a.1.name.cmp(&b.1.name));

        writeln!(content, "| Item | Type | Module | Description |").unwrap();
        writeln!(content, "|------|------|--------|-------------|").unwrap();

        for (module_name, item) in all_items {
            if !self.config.include_deprecated && item.is_deprecated() {
                continue;
            }

            let anchor = self.generate_anchor(&item.name, &item.item_type);
            let description = item.description()
                .unwrap_or("No description")
                .split('\n')
                .next()
                .unwrap_or("");

            writeln!(content, "| [`{}`](#{}) | {} | {} | {} |", 
                item.name, anchor, item.item_type, module_name, description).unwrap();
        }

        writeln!(content).unwrap();
        Ok(())
    }

    /// Collect all items with their module names
    fn collect_all_items_with_module<'a>(&self, module: &'a ModuleInfo, all_items: &mut Vec<(&'a str, &'a DocumentationItem)>) {
        for item in &module.exports {
            all_items.push((&module.name, item));
        }

        for submodule in &module.submodules {
            self.collect_all_items_with_module(submodule, all_items);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::docs::{DocumentationItem, ItemType, PackageDocumentation, package_docs::ModuleInfo};

    fn create_test_package() -> PackageDocumentation {
        let item = DocumentationItem::new("test_function".to_string(), ItemType::Function, 10)
            .with_signature("slay test_function()".to_string());

        let mut root_module = ModuleInfo::new("test_package".to_string(), PathBuf::from("src/"));
        root_module.documentation = Some("Test package description".to_string());
        root_module.exports.push(item);

        // Add a submodule
        let mut submodule = ModuleInfo::new("test_module".to_string(), PathBuf::from("src/test_module.rs"));
        submodule.documentation = Some("Test module description".to_string());
        
        let sub_item = DocumentationItem::new("helper_function".to_string(), ItemType::Function, 20)
            .with_signature("slay helper_function()".to_string());
        submodule.exports.push(sub_item);
        
        root_module.submodules.push(submodule);

        PackageDocumentation {
            name: "test_package".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test package description".to_string()),
            root_module,
            cross_references: HashMap::new(),
            external_dependencies: Vec::new(),
        }
    }

    #[test]
    fn test_single_file_generation() {
        let package = create_test_package();
        let config = MarkdownConfig::default();
        let mut generator = MarkdownGenerator::new(config);

        let result = generator.generate(&package).unwrap();
        
        assert!(result.content.contains("# test_package Documentation"));
        assert!(result.content.contains("## Table of Contents"));
        assert!(result.content.contains("## Module: test_package"));
        assert!(result.table_of_contents.is_some());
    }

    #[test]
    fn test_readme_generation() {
        let package = create_test_package();
        let config = MarkdownConfig {
            format: MarkdownFormat::ReadmeFile {
                include_quickstart: true,
                include_examples: true,
            },
            ..Default::default()
        };
        let mut generator = MarkdownGenerator::new(config);

        let result = generator.generate(&package).unwrap();
        
        assert!(result.content.contains("# test_package ✨"));
        assert!(result.content.contains("## Quick Start 🚀"));
        assert!(result.content.contains("## API Overview 📚"));
        assert_eq!(result.file_paths, vec![PathBuf::from("README.md")]);
    }

    #[test]
    fn test_cross_reference_generation() {
        let package = create_test_package();
        let config = MarkdownConfig::default();
        let mut generator = MarkdownGenerator::new(config);

        let result = generator.generate(&package).unwrap();
        
        assert!(result.cross_references.contains_key("test_function"));
        assert!(result.cross_references["test_function"].starts_with("#"));
    }
}
