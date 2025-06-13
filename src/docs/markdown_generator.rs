//! Markdown Documentation Generator
//! 
//! Generates Markdown documentation suitable for GitHub, GitLab, and other platforms.

use super::*;
use std::fs;
use std::path::Path;

/// Markdown documentation generator
pub struct MarkdownGenerator<'a> {
    config: &'a DocGeneratorConfig,
}

impl<'a> MarkdownGenerator<'a> {
    pub fn new(config: &'a DocGeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate main README file
    pub fn generate_readme(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> Result<(), Error> {
        let markdown = self.build_readme_markdown(docs)?;
        let readme_path = output_dir.join("README.md");
        fs::write(readme_path, markdown).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate documentation for a single module
    pub fn generate_module_doc(&self, doc: &ExtractedDocumentation, output_dir: &Path) -> Result<(), Error> {
        let markdown = self.build_module_markdown(doc)?;
        let module_path = output_dir.join(format!("{}.md", doc.module_name));
        fs::write(module_path, markdown).map_err(Error::Io)?;
        Ok(())
    }

    /// Build README markdown content
    fn build_readme_markdown(&self, docs: &[ExtractedDocumentation]) -> Result<String, Error> {
        let mut md = String::new();
        
        // Title and description
        md.push_str(&format!("# {}\n\n", self.config.title));
        
        if let Some(ref description) = self.config.description {
            md.push_str(&format!("{}\n\n", description));
        }

        // Badges (if version is available)
        if let Some(ref version) = self.config.version {
            md.push_str(&format!("![Version](https://img.shields.io/badge/version-{}-blue)\n", version));
        }
        md.push_str("![Language](https://img.shields.io/badge/language-CURSED-purple)\n");
        md.push_str("![Documentation](https://img.shields.io/badge/docs-generated-green)\n\n");

        // Table of Contents
        md.push_str("## Table of Contents\n\n");
        md.push_str("- [Overview](#overview)\n");
        md.push_str("- [Modules](#modules)\n");
        md.push_str("- [Quick Start](#quick-start)\n");
        md.push_str("- [API Reference](#api-reference)\n");
        
        if !self.config.authors.is_empty() {
            md.push_str("- [Authors](#authors)\n");
        }
        
        md.push_str("\n");

        // Overview
        md.push_str("## Overview\n\n");
        md.push_str("This documentation covers the CURSED programming language API.\n\n");

        let total_items: usize = docs.iter().map(|d| d.items.len()).sum();
        let total_modules = docs.len();
        
        md.push_str(&format!("- 📦 **{}** modules\n", total_modules));
        md.push_str(&format!("- 📄 **{}** documented items\n", total_items));
        md.push_str(&format!("- 🔗 Cross-references and examples included\n\n"));

        // Modules section
        md.push_str("## Modules\n\n");
        md.push_str("| Module | Items | Description |\n");
        md.push_str("|--------|-------|--------------|\n");
        
        for doc in docs {
            let item_count = doc.items.len();
            let description = self.extract_module_description(doc);
            md.push_str(&format!(
                "| [{}]({}.md) | {} | {} |\n",
                doc.module_name, doc.module_name, item_count, description
            ));
        }
        
        md.push_str("\n");

        // Quick Start
        md.push_str("## Quick Start\n\n");
        md.push_str("```cursed\n");
        md.push_str("// Example CURSED code\n");
        md.push_str("yeet \"stdlib::io\"\n\n");
        md.push_str("slay main() {\n");
        md.push_str("    println(\"Hello, World! This is absolutely fire! 🔥\")\n");
        md.push_str("}\n");
        md.push_str("```\n\n");

        // API Reference
        md.push_str("## API Reference\n\n");
        
        for doc in docs {
            md.push_str(&format!("### [{}]({}.md)\n\n", doc.module_name, doc.module_name));
            
            // Group items by kind
            let mut functions = Vec::new();
            let mut structs = Vec::new();
            let mut interfaces = Vec::new();
            let mut constants = Vec::new();
            let mut variables = Vec::new();
            
            for item in &doc.items {
                match item.kind {
                    ItemKind::Function => functions.push(item),
                    ItemKind::Struct => structs.push(item),
                    ItemKind::Interface => interfaces.push(item),
                    ItemKind::Constant => constants.push(item),
                    ItemKind::Variable => variables.push(item),
                    _ => {}
                }
            }
            
            if !functions.is_empty() {
                md.push_str("**Functions:**\n");
                for func in functions {
                    md.push_str(&format!("- [`{}`]({}.md#{}) - {}\n", 
                        func.name, doc.module_name, func.name.to_lowercase(), func.summary));
                }
                md.push_str("\n");
            }
            
            if !structs.is_empty() {
                md.push_str("**Structs:**\n");
                for struct_item in structs {
                    md.push_str(&format!("- [`{}`]({}.md#{}) - {}\n", 
                        struct_item.name, doc.module_name, struct_item.name.to_lowercase(), struct_item.summary));
                }
                md.push_str("\n");
            }
            
            if !interfaces.is_empty() {
                md.push_str("**Interfaces:**\n");
                for interface in interfaces {
                    md.push_str(&format!("- [`{}`]({}.md#{}) - {}\n", 
                        interface.name, doc.module_name, interface.name.to_lowercase(), interface.summary));
                }
                md.push_str("\n");
            }
        }

        // Authors
        if !self.config.authors.is_empty() {
            md.push_str("## Authors\n\n");
            for author in &self.config.authors {
                md.push_str(&format!("- {}\n", author));
            }
            md.push_str("\n");
        }

        // Footer
        md.push_str("---\n\n");
        md.push_str(&format!("*Generated by CURSED Documentation Generator on {}*\n", 
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        Ok(md)
    }

    /// Build markdown for a single module
    fn build_module_markdown(&self, doc: &ExtractedDocumentation) -> Result<String, Error> {
        let mut md = String::new();
        
        // Module header
        md.push_str(&format!("# Module: {}\n\n", doc.module_name));
        
        if let Some(ref package) = doc.package_name {
            md.push_str(&format!("**Package:** `{}`\n\n", package));
        }
        
        md.push_str(&format!("**File:** `{}`\n\n", doc.file_path.display()));

        // Module description
        let description = self.extract_module_description(doc);
        if !description.is_empty() {
            md.push_str(&format!("{}\n\n", description));
        }

        // Statistics
        md.push_str("## Module Information\n\n");
        md.push_str(&format!("- **Total items:** {}\n", doc.items.len()));
        md.push_str(&format!("- **Source lines:** {}\n", doc.source_info.line_count));
        md.push_str(&format!("- **File size:** {} bytes\n", doc.source_info.file_size));
        md.push_str("\n");

        // Imports
        if !doc.imports.is_empty() {
            md.push_str("## Imports\n\n");
            for import in &doc.imports {
                md.push_str(&format!("- `{}`\n", import));
            }
            md.push_str("\n");
        }

        // Table of Contents
        md.push_str("## Table of Contents\n\n");
        
        // Group by item kind
        let mut by_kind: std::collections::BTreeMap<String, Vec<&DocumentationItem>> = std::collections::BTreeMap::new();
        for item in &doc.items {
            by_kind.entry(item.kind.to_string()).or_insert_with(Vec::new).push(item);
        }
        
        for (kind, items) in &by_kind {
            md.push_str(&format!("- [{}](#{})\n", kind.to_uppercase(), kind.to_lowercase()));
            for item in items {
                md.push_str(&format!("  - [`{}`](#{}-{})\n", item.name, kind.to_lowercase(), item.name.to_lowercase()));
            }
        }
        md.push_str("\n");

        // Documentation for each kind
        for (kind, items) in by_kind {
            md.push_str(&format!("## {}\n\n", kind.to_uppercase()));
            
            for item in items {
                md.push_str(&self.build_item_markdown(item)?);
                md.push_str("\n");
            }
        }

        Ok(md)
    }

    /// Build markdown for a documentation item
    fn build_item_markdown(&self, item: &DocumentationItem) -> Result<String, Error> {
        let mut md = String::new();
        
        // Item header
        md.push_str(&format!("### {}-{}\n\n", item.kind.to_string().to_lowercase(), item.name.to_lowercase()));
        md.push_str(&format!("#### `{}`\n\n", item.name));

        // Visibility badge
        if matches!(item.visibility, Visibility::Public) {
            md.push_str("![Public](https://img.shields.io/badge/visibility-public-green) ");
        } else {
            md.push_str("![Private](https://img.shields.io/badge/visibility-private-orange) ");
        }
        
        md.push_str(&format!("![{}](https://img.shields.io/badge/kind-{}-blue)\n\n", 
            item.kind.to_string(), item.kind.to_string()));

        // Signature
        if let Some(ref signature) = item.signature {
            md.push_str("**Signature:**\n");
            md.push_str("```cursed\n");
            md.push_str(signature);
            md.push_str("\n```\n\n");
        }

        // Summary
        if !item.summary.is_empty() {
            md.push_str(&format!("**Summary:** {}\n\n", item.summary));
        }

        // Description
        if !item.description.is_empty() {
            md.push_str("**Description:**\n\n");
            md.push_str(&item.description);
            md.push_str("\n\n");
        }

        // Parameters
        if !item.parameters.is_empty() {
            md.push_str("**Parameters:**\n\n");
            md.push_str("| Name | Type | Description |\n");
            md.push_str("|------|------|-------------|\n");
            
            for param in &item.parameters {
                let type_name = param.type_name.as_deref().unwrap_or("any");
                md.push_str(&format!("| `{}` | `{}` | {} |\n", 
                    param.name, type_name, param.description));
            }
            md.push_str("\n");
        }

        // Return type
        if let Some(ref return_type) = item.return_type {
            md.push_str(&format!("**Returns:** `{}`\n\n", return_type));
        }

        // Examples
        if !item.examples.is_empty() {
            md.push_str("**Examples:**\n\n");
            
            for (i, example) in item.examples.iter().enumerate() {
                if let Some(ref title) = example.title {
                    md.push_str(&format!("*{}*\n\n", title));
                }
                
                if let Some(ref description) = example.description {
                    md.push_str(&format!("{}\n\n", description));
                }
                
                md.push_str(&format!("```{}\n", example.language));
                md.push_str(&example.code);
                md.push_str("\n```\n");
                
                if let Some(ref output) = example.output {
                    md.push_str("\n**Output:**\n");
                    md.push_str("```\n");
                    md.push_str(output);
                    md.push_str("\n```\n");
                }
                
                if i < item.examples.len() - 1 {
                    md.push_str("\n");
                }
            }
            md.push_str("\n");
        }

        // Tags
        if !item.tags.is_empty() {
            md.push_str("**Tags:**\n\n");
            for (tag_name, values) in &item.tags {
                md.push_str(&format!("- **{}:** {}\n", tag_name, values.join(", ")));
            }
            md.push_str("\n");
        }

        // Source location
        md.push_str(&format!("**Source:** Line {} Column {}\n\n", 
            item.location.line, item.location.column));

        md.push_str("---\n\n");

        Ok(md)
    }

    /// Extract module description from documentation
    fn extract_module_description(&self, doc: &ExtractedDocumentation) -> String {
        // Try to find a module-level description
        // For now, return a generic description
        format!("CURSED module with {} items", doc.items.len())
    }
}
