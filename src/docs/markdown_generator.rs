//! Markdown Documentation Generator
//! 
//! Generates GitHub-compatible markdown documentation with proper formatting,
//! cross-references, and table of contents.

use crate::docs::generator::{DocGeneratorConfig, ExtractedDocumentation, DocumentationItem, ItemKind};
use crate::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

pub struct MarkdownGenerator<'a> {
    config: &'a DocGeneratorConfig,
}

impl<'a> MarkdownGenerator<'a> {
    pub fn new(config: &'a DocGeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate main README
    pub fn generate_readme(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> Result<(), Error> {
        let readme_path = output_dir.join("README.md");
        
        let mut md = String::new();
        
        // Project header
        md.push_str(&format!("# {}\n\n", self.config.title));
        
        if let Some(desc) = &self.config.description {
            md.push_str(&format!("{}\n\n", desc));
        }
        
        // Badges
        md.push_str("[![CURSED](https://img.shields.io/badge/language-CURSED-ff6b9d)](https://github.com/your-org/cursed)\n");
        md.push_str("[![Gen%20Z](https://img.shields.io/badge/slang-Gen%20Z-4ecdc4)](https://github.com/your-org/cursed)\n");
        md.push_str("[![No%20Cap](https://img.shields.io/badge/vibes-no%20cap-ffe66d)](https://github.com/your-org/cursed)\n\n");

        // Project info
        md.push_str("## 📋 Project Information\n\n");
        if let Some(version) = &self.config.version {
            md.push_str(&format!("- **Version**: {}\n", version));
        }
        if !self.config.authors.is_empty() {
            md.push_str(&format!("- **Authors**: {}\n", self.config.authors.join(", ")));
        }
        if let Some(base_url) = &self.config.base_url {
            md.push_str(&format!("- **Homepage**: {}\n", base_url));
        }
        md.push_str("\n");

        // Getting started
        md.push_str("## 🚀 Getting Started with CURSED\n\n");
        md.push_str("CURSED is a programming language that speaks Gen Z. Here's how to get started:\n\n");
        
        md.push_str("### Hello World Example\n\n");
        md.push_str("```cursed\n");
        md.push_str("// Hello world in CURSED - because we're iconic like that\n");
        md.push_str("slay main() {\n");
        md.push_str("    println(\"Hello, world! This is lowkey fire! 🔥\")\n");
        md.push_str("}\n");
        md.push_str("```\n\n");

        // Gen Z Keywords Guide
        md.push_str("### 📚 Gen Z Keywords Guide\n\n");
        md.push_str("CURSED uses Gen Z slang for keywords because traditional programming languages are cheugy:\n\n");
        md.push_str("| Traditional | CURSED | What it does |\n");
        md.push_str("|-------------|--------|-------------|\n");
        md.push_str("| `fn` / `function` | `slay` | Declares a function |\n");
        md.push_str("| `let mut` | `sus` | Declares a mutable variable |\n");
        md.push_str("| `let` / `const` | `facts` | Declares a constant/immutable |\n");
        md.push_str("| `if` | `lowkey` | Conditional statement |\n");
        md.push_str("| `else` | `highkey` | Else clause |\n");
        md.push_str("| `while` | `periodt` | While loop |\n");
        md.push_str("| `for` | `bestie` | For loop |\n");
        md.push_str("| `break` | `flex` | Break statement |\n");
        md.push_str("| `struct` | `squad` | Struct definition |\n");
        md.push_str("| `interface` | `collab` | Interface definition |\n");
        md.push_str("| `async` | `stan` | Async/goroutine spawn |\n");
        md.push_str("| `yield` | `yolo` | Yield/await operation |\n\n");

        // Quick examples
        md.push_str("### 💫 Quick Examples\n\n");
        
        md.push_str("#### Variables and Constants\n");
        md.push_str("```cursed\n");
        md.push_str("sus name = \"bestie\"        // mutable variable\n");
        md.push_str("facts pi = 3.14159         // constant\n");
        md.push_str("```\n\n");
        
        md.push_str("#### Functions\n");
        md.push_str("```cursed\n");
        md.push_str("slay greet(name: string) {\n");
        md.push_str("    println(\"Hey \" + name + \"! You're serving looks! ✨\")\n");
        md.push_str("}\n");
        md.push_str("```\n\n");
        
        md.push_str("#### Control Flow\n");
        md.push_str("```cursed\n");
        md.push_str("lowkey (age >= 18) {\n");
        md.push_str("    println(\"You're an adult, bestie!\")\n");
        md.push_str("} highkey {\n");
        md.push_str("    println(\"Still a baby, no cap\")\n");
        md.push_str("}\n");
        md.push_str("```\n\n");

        // Modules documentation
        md.push_str("## 📁 Modules\n\n");
        md.push_str("| Module | Items | Description |\n");
        md.push_str("|--------|-------|-------------|\n");
        
        for doc in docs {
            let module_file = format!("{}.md", doc.module_name.replace("::", "_"));
            md.push_str(&format!(
                "| [{}]({}) | {} | {} |\n",
                doc.module_name,
                module_file,
                doc.items.len(),
                self.get_module_description(doc)
            ));
        }
        md.push_str("\n");

        // API Overview
        md.push_str("## 🔥 API Overview\n\n");
        let total_functions = docs.iter().map(|d| d.items.iter().filter(|i| matches!(i.kind, ItemKind::Function)).count()).sum::<usize>();
        let total_structs = docs.iter().map(|d| d.items.iter().filter(|i| matches!(i.kind, ItemKind::Struct)).count()).sum::<usize>();
        let total_interfaces = docs.iter().map(|d| d.items.iter().filter(|i| matches!(i.kind, ItemKind::Interface)).count()).sum::<usize>();
        
        md.push_str(&format!("- 🎯 **{}** functions that absolutely slay\n", total_functions));
        md.push_str(&format!("- 👥 **{}** squads (structs) for organizing your data\n", total_structs));
        md.push_str(&format!("- 🤝 **{}** collabs (interfaces) for that polymorphic energy\n", total_interfaces));
        md.push_str(&format!("- 📝 **{}** modules with comprehensive documentation\n", docs.len()));
        md.push_str("\n");

        // Installation (placeholder)
        md.push_str("## 📦 Installation\n\n");
        md.push_str("```bash\n");
        md.push_str("# Clone the repo (it's giving main character energy)\n");
        md.push_str("git clone https://github.com/your-org/cursed.git\n\n");
        md.push_str("# Build with cargo (no cap, it just works)\n");
        md.push_str("cd cursed\n");
        md.push_str("cargo build --release\n\n");
        md.push_str("# Run your first CURSED program\n");
        md.push_str("./target/release/cursed examples/hello_world.csd\n");
        md.push_str("```\n\n");

        // Contributing
        md.push_str("## 🤝 Contributing\n\n");
        md.push_str("Want to contribute? That's very demure, very mindful of you! 💅\n\n");
        md.push_str("1. Fork the repo (giving fork energy)\n");
        md.push_str("2. Create your feature branch (`git checkout -b feature/absolute-unit`)\n");
        md.push_str("3. Commit your changes (`git commit -m 'Add some fire features'`)\n");
        md.push_str("4. Push to the branch (`git push origin feature/absolute-unit`)\n");
        md.push_str("5. Open a Pull Request (and let's make this code slap)\n\n");

        // Footer
        md.push_str("---\n\n");
        md.push_str("*Generated with 💖 by the CURSED Documentation Generator*\n\n");
        md.push_str("*This project is serving looks and taking names. No cap! 🧢❌*\n");
        
        fs::write(readme_path, md).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate module documentation
    pub fn generate_module_doc(&self, doc: &ExtractedDocumentation, output_dir: &Path) -> Result<(), Error> {
        let module_file = format!("{}.md", doc.module_name.replace("::", "_"));
        let module_path = output_dir.join(module_file);
        
        let mut md = String::new();
        
        // Module header
        md.push_str(&format!("# {} Module\n\n", doc.module_name));
        
        if let Some(package) = &doc.package_name {
            md.push_str(&format!("**Package**: `{}`  \n", package));
        }
        md.push_str(&format!("**File**: `{}`  \n", doc.file_path.display()));
        md.push_str(&format!("**Items**: {}  \n", doc.items.len()));
        md.push_str(&format!("**Lines of Code**: {}  \n\n", doc.source_info.line_count));

        // Navigation
        md.push_str("[⬅️ Back to Index](README.md)\n\n");

        // Module description
        md.push_str("## 📖 Description\n\n");
        md.push_str(&format!("{}\n\n", self.get_module_description(doc)));

        // Quick stats
        md.push_str("## 📊 Module Statistics\n\n");
        let functions = doc.items.iter().filter(|i| matches!(i.kind, ItemKind::Function)).count();
        let structs = doc.items.iter().filter(|i| matches!(i.kind, ItemKind::Struct)).count();
        let interfaces = doc.items.iter().filter(|i| matches!(i.kind, ItemKind::Interface)).count();
        let variables = doc.items.iter().filter(|i| matches!(i.kind, ItemKind::Variable)).count();
        let constants = doc.items.iter().filter(|i| matches!(i.kind, ItemKind::Constant)).count();
        
        md.push_str("| Type | Count | Description |\n");
        md.push_str("|------|-------|-------------|\n");
        md.push_str(&format!("| 🎯 Functions | {} | Functions that absolutely slay |\n", functions));
        md.push_str(&format!("| 👥 Structs | {} | Squads for organizing data |\n", structs));
        md.push_str(&format!("| 🤝 Interfaces | {} | Collabs for polymorphic vibes |\n", interfaces));
        md.push_str(&format!("| 📝 Variables | {} | Sus (mutable) variables |\n", variables));
        md.push_str(&format!("| 📌 Constants | {} | Facts (immutable) values |\n", constants));
        md.push_str("\n");

        // Table of contents
        md.push_str("## 📑 Table of Contents\n\n");
        
        // Group items by type
        let mut functions_list = Vec::new();
        let mut structs_list = Vec::new();
        let mut interfaces_list = Vec::new();
        let mut variables_list = Vec::new();
        let mut constants_list = Vec::new();
        
        for item in &doc.items {
            match item.kind {
                ItemKind::Function => functions_list.push(item),
                ItemKind::Struct => structs_list.push(item),
                ItemKind::Interface => interfaces_list.push(item),
                ItemKind::Variable => variables_list.push(item),
                ItemKind::Constant => constants_list.push(item),
                _ => {}
            }
        }
        
        if !functions_list.is_empty() {
            md.push_str("### 🎯 Functions\n");
            for item in &functions_list {
                md.push_str(&format!("- [{}](#{}) - {}\n", item.name, self.anchor_name(&item.name), item.summary));
            }
            md.push_str("\n");
        }
        
        if !structs_list.is_empty() {
            md.push_str("### 👥 Structs (Squads)\n");
            for item in &structs_list {
                md.push_str(&format!("- [{}](#{}) - {}\n", item.name, self.anchor_name(&item.name), item.summary));
            }
            md.push_str("\n");
        }
        
        if !interfaces_list.is_empty() {
            md.push_str("### 🤝 Interfaces (Collabs)\n");
            for item in &interfaces_list {
                md.push_str(&format!("- [{}](#{}) - {}\n", item.name, self.anchor_name(&item.name), item.summary));
            }
            md.push_str("\n");
        }
        
        if !variables_list.is_empty() {
            md.push_str("### 📝 Variables\n");
            for item in &variables_list {
                md.push_str(&format!("- [{}](#{}) - {}\n", item.name, self.anchor_name(&item.name), item.summary));
            }
            md.push_str("\n");
        }
        
        if !constants_list.is_empty() {
            md.push_str("### 📌 Constants\n");
            for item in &constants_list {
                md.push_str(&format!("- [{}](#{}) - {}\n", item.name, self.anchor_name(&item.name), item.summary));
            }
            md.push_str("\n");
        }

        // Detailed documentation
        md.push_str("---\n\n");
        md.push_str("## 📖 Detailed Documentation\n\n");
        
        // Generate docs for each item type
        if !functions_list.is_empty() {
            md.push_str("### 🎯 Functions\n\n");
            for item in &functions_list {
                md.push_str(&self.generate_item_markdown(item));
            }
        }
        
        if !structs_list.is_empty() {
            md.push_str("### 👥 Structs (Squads)\n\n");
            for item in &structs_list {
                md.push_str(&self.generate_item_markdown(item));
            }
        }
        
        if !interfaces_list.is_empty() {
            md.push_str("### 🤝 Interfaces (Collabs)\n\n");
            for item in &interfaces_list {
                md.push_str(&self.generate_item_markdown(item));
            }
        }
        
        if !variables_list.is_empty() {
            md.push_str("### 📝 Variables\n\n");
            for item in &variables_list {
                md.push_str(&self.generate_item_markdown(item));
            }
        }
        
        if !constants_list.is_empty() {
            md.push_str("### 📌 Constants\n\n");
            for item in &constants_list {
                md.push_str(&self.generate_item_markdown(item));
            }
        }

        // Footer
        md.push_str("---\n\n");
        md.push_str("*Generated with 💖 by the CURSED Documentation Generator*\n");
        
        fs::write(module_path, md).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate markdown for a documentation item
    fn generate_item_markdown(&self, item: &DocumentationItem) -> String {
        let mut md = String::new();
        
        // Item header
        let emoji = match item.kind {
            ItemKind::Function => "🎯",
            ItemKind::Struct => "👥",
            ItemKind::Interface => "🤝",
            ItemKind::Variable => "📝",
            ItemKind::Constant => "📌",
            _ => "📄",
        };
        
        md.push_str(&format!("#### {} {} {}\n\n", emoji, item.name, self.get_kind_badge(&item.kind)));
        
        // Signature
        if let Some(signature) = &item.signature {
            md.push_str("**Signature:**\n");
            md.push_str("```cursed\n");
            md.push_str(signature);
            md.push_str("\n```\n\n");
        }
        
        // Description
        md.push_str("**Description:**  \n");
        md.push_str(&format!("{}\n\n", item.description));
        
        // Parameters
        if !item.parameters.is_empty() {
            md.push_str("**Parameters:**\n\n");
            md.push_str("| Name | Type | Description | Default |\n");
            md.push_str("|------|------|-------------|----------|\n");
            
            for param in &item.parameters {
                let type_str = param.type_name.as_ref().map(|t| format!("`{}`", t)).unwrap_or_else(|| "—".to_string());
                let default_str = param.default_value.as_ref().map(|d| format!("`{}`", d)).unwrap_or_else(|| "—".to_string());
                md.push_str(&format!(
                    "| `{}` | {} | {} | {} |\n",
                    param.name, type_str, param.description, default_str
                ));
            }
            md.push_str("\n");
        }
        
        // Return type
        if let Some(return_type) = &item.return_type {
            md.push_str(&format!("**Returns:** `{}`\n\n", return_type));
        }
        
        // Source code example
        if self.config.include_examples {
            if let Some(source) = &item.source_code {
                md.push_str("**Source:**\n");
                md.push_str("```cursed\n");
                md.push_str(source);
                md.push_str("\n```\n\n");
            }
        }
        
        // Usage example
        md.push_str(&self.generate_usage_example(item));
        
        md.push_str("---\n\n");
        md
    }

    /// Generate usage example for an item
    fn generate_usage_example(&self, item: &DocumentationItem) -> String {
        let mut example = String::new();
        
        match item.kind {
            ItemKind::Function => {
                example.push_str("**Example:**\n");
                example.push_str("```cursed\n");
                
                // Generate realistic function call
                if item.parameters.is_empty() {
                    example.push_str(&format!("{}()", item.name));
                } else {
                    let params: Vec<String> = item.parameters.iter().map(|p| {
                        match p.type_name.as_deref() {
                            Some("string") => "\"example\"".to_string(),
                            Some("i32") | Some("int") => "42".to_string(),
                            Some("f64") | Some("float") => "3.14".to_string(),
                            Some("bool") => "facts".to_string(),
                            _ => "value".to_string(),
                        }
                    }).collect();
                    example.push_str(&format!("{}({})", item.name, params.join(", ")));
                }
                
                if item.return_type.is_some() {
                    example.insert_str(0, "sus result = ");
                }
                
                example.push_str("\n```\n\n");
            }
            ItemKind::Struct => {
                example.push_str("**Example:**\n");
                example.push_str("```cursed\n");
                example.push_str(&format!("sus instance = {}::new()\n", item.name));
                example.push_str("```\n\n");
            }
            ItemKind::Variable => {
                example.push_str("**Example:**\n");
                example.push_str("```cursed\n");
                example.push_str(&format!("sus value = {}\n", item.name));
                example.push_str("```\n\n");
            }
            ItemKind::Constant => {
                example.push_str("**Example:**\n");
                example.push_str("```cursed\n");
                example.push_str(&format!("sus result = {} * 2\n", item.name));
                example.push_str("```\n\n");
            }
            _ => {}
        }
        
        example
    }

    /// Get module description
    fn get_module_description(&self, doc: &ExtractedDocumentation) -> String {
        if doc.items.is_empty() {
            return "Empty module".to_string();
        }
        
        let functions = doc.items.iter().filter(|i| matches!(i.kind, ItemKind::Function)).count();
        let structs = doc.items.iter().filter(|i| matches!(i.kind, ItemKind::Struct)).count();
        
        format!(
            "Module containing {} function{} and {} struct{} with comprehensive CURSED language features",
            functions,
            if functions == 1 { "" } else { "s" },
            structs,
            if structs == 1 { "" } else { "s" }
        )
    }

    /// Get kind badge
    fn get_kind_badge(&self, kind: &ItemKind) -> String {
        match kind {
            ItemKind::Function => "![Function](https://img.shields.io/badge/-Function-ff6b9d)".to_string(),
            ItemKind::Struct => "![Struct](https://img.shields.io/badge/-Squad-4ecdc4)".to_string(),
            ItemKind::Interface => "![Interface](https://img.shields.io/badge/-Collab-ffe66d)".to_string(),
            ItemKind::Variable => "![Variable](https://img.shields.io/badge/-Sus-orange)".to_string(),
            ItemKind::Constant => "![Constant](https://img.shields.io/badge/-Facts-green)".to_string(),
            _ => "![Other](https://img.shields.io/badge/-Other-gray)".to_string(),
        }
    }

    /// Generate anchor name for linking
    fn anchor_name(&self, name: &str) -> String {
        name.to_lowercase().replace(' ', "-")
    }
}
