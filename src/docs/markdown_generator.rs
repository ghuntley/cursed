//! Markdown Documentation Generator
//! 
//! Generates comprehensive Markdown documentation for GitHub and other markdown platforms.

use crate::docs::generator::{DocGeneratorConfig, ExtractedDocumentation, DocumentationItem};
use crate::error::Error;
use std::fs;
use std::path::Path;

/// Markdown documentation generator
pub struct MarkdownGenerator {
    config: DocGeneratorConfig,
}

impl MarkdownGenerator {
    pub fn new(config: &DocGeneratorConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Generate README.md file
    pub fn generate_readme(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> Result<(), Error> {
        let readme_path = output_dir.join("README.md");
        
        let mut content = String::new();
        
        // Title and description
        content.push_str(&format!("# {}\n\n", self.config.title));
        
        if let Some(description) = &self.config.description {
            content.push_str(&format!("{}\n\n", description));
        }
        
        // Project information
        if let Some(version) = &self.config.version {
            content.push_str(&format!("**Version:** {}\n\n", version));
        }
        
        if !self.config.authors.is_empty() {
            content.push_str(&format!("**Authors:** {}\n\n", self.config.authors.join(", ")));
        }
        
        // Table of contents
        content.push_str("## Table of Contents\n\n");
        for doc in docs {
            content.push_str(&format!("- [{}]({})\n", doc.module_name, self.get_module_filename(&doc.module_name)));
        }
        content.push_str("\n");
        
        // Module overview
        content.push_str("## Modules\n\n");
        for doc in docs {
            content.push_str(&format!("### {}\n\n", doc.module_name));
            
            if let Some(package) = &doc.package_name {
                content.push_str(&format!("**Package:** {}\n\n", package));
            }
            
            content.push_str(&format!("**File:** `{}`\n\n", doc.file_path.display()));
            content.push_str(&format!("**Items:** {}\n\n", doc.items.len()));
            
            // Brief item summary
            let functions_count = doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function)).count();
            let structs_count = doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Struct)).count();
            let interfaces_count = doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Interface)).count();
            
            if functions_count > 0 {
                content.push_str(&format!("- Functions: {}\n", functions_count));
            }
            if structs_count > 0 {
                content.push_str(&format!("- Structs: {}\n", structs_count));
            }
            if interfaces_count > 0 {
                content.push_str(&format!("- Interfaces: {}\n", interfaces_count));
            }
            content.push_str("\n");
        }
        
        // Statistics
        let total_items: usize = docs.iter().map(|d| d.items.len()).sum();
        let total_functions = docs.iter().map(|d| d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function)).count()).sum::<usize>();
        let total_structs = docs.iter().map(|d| d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Struct)).count()).sum::<usize>();
        
        content.push_str("## Statistics\n\n");
        content.push_str(&format!("- **Modules:** {}\n", docs.len()));
        content.push_str(&format!("- **Total Items:** {}\n", total_items));
        content.push_str(&format!("- **Functions:** {}\n", total_functions));
        content.push_str(&format!("- **Structs:** {}\n", total_structs));
        content.push_str("\n");
        
        // Usage example
        content.push_str("## Usage\n\n");
        content.push_str("```cursed\n");
        content.push_str("// Example CURSED program\n");
        content.push_str("slay main() {\n");
        content.push_str("    println(\"Hello, CURSED!\")?;\n");
        content.push_str("}\n");
        content.push_str("```\n\n");
        
        // License and footer
        content.push_str("## Documentation\n\n");
        content.push_str("This documentation was generated automatically by the CURSED documentation system.\n\n");
        
        fs::write(readme_path, content).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate documentation for a single module
    pub fn generate_module_doc(&self, doc: &ExtractedDocumentation, output_dir: &Path) -> Result<(), Error> {
        let module_path = output_dir.join(self.get_module_filename(&doc.module_name));
        
        let mut content = String::new();
        
        // Module header
        content.push_str(&format!("# Module: {}\n\n", doc.module_name));
        
        if let Some(package) = &doc.package_name {
            content.push_str(&format!("**Package:** {}\n\n", package));
        }
        
        // Module information
        content.push_str("## Module Information\n\n");
        content.push_str(&format!("- **File:** `{}`\n", doc.file_path.display()));
        content.push_str(&format!("- **Lines:** {}\n", doc.source_info.line_count));
        content.push_str(&format!("- **Size:** {} bytes\n", doc.source_info.file_size));
        content.push_str(&format!("- **Encoding:** {}\n", doc.source_info.encoding));
        content.push_str("\n");
        
        // Imports
        if !doc.imports.is_empty() {
            content.push_str("## Imports\n\n");
            for import in &doc.imports {
                content.push_str(&format!("- `{}`\n", import));
            }
            content.push_str("\n");
        }
        
        // Table of contents
        content.push_str("## Table of Contents\n\n");
        
        // Group items by type for TOC
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut interfaces = Vec::new();
        let mut variables = Vec::new();
        let mut constants = Vec::new();
        
        for item in &doc.items {
            match item.kind {
                crate::docs::generator::ItemKind::Function => functions.push(item),
                crate::docs::generator::ItemKind::Struct => structs.push(item),
                crate::docs::generator::ItemKind::Interface => interfaces.push(item),
                crate::docs::generator::ItemKind::Variable => variables.push(item),
                crate::docs::generator::ItemKind::Constant => constants.push(item),
                _ => {}
            }
        }
        
        if !functions.is_empty() {
            content.push_str("### Functions\n\n");
            for func in &functions {
                content.push_str(&format!("- [{0}](#{1})\n", func.name, self.generate_anchor(&func.name)));
            }
            content.push_str("\n");
        }
        
        if !structs.is_empty() {
            content.push_str("### Structs\n\n");
            for struct_item in &structs {
                content.push_str(&format!("- [{0}](#{1})\n", struct_item.name, self.generate_anchor(&struct_item.name)));
            }
            content.push_str("\n");
        }
        
        if !interfaces.is_empty() {
            content.push_str("### Interfaces\n\n");
            for interface in &interfaces {
                content.push_str(&format!("- [{0}](#{1})\n", interface.name, self.generate_anchor(&interface.name)));
            }
            content.push_str("\n");
        }
        
        if !constants.is_empty() {
            content.push_str("### Constants\n\n");
            for constant in &constants {
                content.push_str(&format!("- [{0}](#{1})\n", constant.name, self.generate_anchor(&constant.name)));
            }
            content.push_str("\n");
        }
        
        if !variables.is_empty() {
            content.push_str("### Variables\n\n");
            for variable in &variables {
                content.push_str(&format!("- [{0}](#{1})\n", variable.name, self.generate_anchor(&variable.name)));
            }
            content.push_str("\n");
        }
        
        // Generate sections for each type
        if !functions.is_empty() {
            content.push_str("## Functions\n\n");
            for func in functions {
                content.push_str(&self.generate_item_documentation(func)?);
            }
        }
        
        if !structs.is_empty() {
            content.push_str("## Structs\n\n");
            for struct_item in structs {
                content.push_str(&self.generate_item_documentation(struct_item)?);
            }
        }
        
        if !interfaces.is_empty() {
            content.push_str("## Interfaces\n\n");
            for interface in interfaces {
                content.push_str(&self.generate_item_documentation(interface)?);
            }
        }
        
        if !constants.is_empty() {
            content.push_str("## Constants\n\n");
            for constant in constants {
                content.push_str(&self.generate_item_documentation(constant)?);
            }
        }
        
        if !variables.is_empty() {
            content.push_str("## Variables\n\n");
            for variable in variables {
                content.push_str(&self.generate_item_documentation(variable)?);
            }
        }
        
        fs::write(module_path, content).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate documentation for a single item
    fn generate_item_documentation(&self, item: &DocumentationItem) -> Result<String, Error> {
        let mut content = String::new();
        
        // Item header
        content.push_str(&format!("### {}\n\n", item.name));
        
        // Signature
        if let Some(signature) = &item.signature {
            content.push_str("```cursed\n");
            content.push_str(signature);
            content.push_str("\n```\n\n");
        }
        
        // Summary and description
        content.push_str(&format!("{}\n\n", item.summary));
        
        if !item.description.is_empty() && item.description != item.summary {
            content.push_str(&format!("{}\n\n", item.description));
        }
        
        // Parameters
        if !item.parameters.is_empty() {
            content.push_str("#### Parameters\n\n");
            content.push_str("| Name | Type | Description | Default |\n");
            content.push_str("|------|------|-------------|----------|\n");
            
            for param in &item.parameters {
                content.push_str(&format!(
                    "| `{}` | `{}` | {} | {} |\n",
                    param.name,
                    param.type_name.as_deref().unwrap_or("unknown"),
                    param.description,
                    param.default_value.as_deref().unwrap_or("None")
                ));
            }
            content.push_str("\n");
        }
        
        // Return type
        if let Some(return_type) = &item.return_type {
            content.push_str("#### Returns\n\n");
            content.push_str(&format!("`{}`\n\n", return_type));
        }
        
        // Examples
        if !item.examples.is_empty() {
            content.push_str("#### Examples\n\n");
            for example in &item.examples {
                if let Some(title) = &example.title {
                    content.push_str(&format!("**{}**\n\n", title));
                }
                if let Some(description) = &example.description {
                    content.push_str(&format!("{}\n\n", description));
                }
                content.push_str(&format!("```{}\n", example.language));
                content.push_str(&example.code);
                content.push_str("\n```\n\n");
                
                if let Some(output) = &example.output {
                    content.push_str("Output:\n\n");
                    content.push_str("```\n");
                    content.push_str(output);
                    content.push_str("\n```\n\n");
                }
            }
        }
        
        // Source code
        if self.config.include_examples && item.source_code.is_some() {
            content.push_str("<details>\n");
            content.push_str("<summary>Source Code</summary>\n\n");
            content.push_str("```cursed\n");
            content.push_str(item.source_code.as_ref().unwrap());
            content.push_str("\n```\n\n");
            content.push_str("</details>\n\n");
        }
        
        // Add separator
        content.push_str("---\n\n");
        
        Ok(content)
    }

    /// Get filename for module documentation
    fn get_module_filename(&self, module_name: &str) -> String {
        format!("{}.md", module_name.replace("::", "_").to_lowercase())
    }

    /// Generate anchor link for markdown
    fn generate_anchor(&self, name: &str) -> String {
        name.to_lowercase()
            .replace(" ", "-")
            .replace("_", "-")
            .replace("::", "-")
    }
}
