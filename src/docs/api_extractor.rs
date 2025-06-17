//! API Documentation Extractor
//! 
//! Extracts comprehensive API documentation from the CURSED standard library and Rust source code.

use crate::docs::generator::{ExtractedDocumentation, DocumentationItem, ItemKind, Visibility, Parameter, Example, SourceInfo};
use crate::error::{Error, SourceLocation};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde_json;

/// API documentation extractor for stdlib modules
pub struct ApiExtractor {
    include_internal: bool,
    extract_examples: bool,
    base_path: PathBuf,
}

impl ApiExtractor {
    /// Create new API extractor
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            include_internal: false,
            extract_examples: true,
            base_path,
        }
    }

    /// Configure extraction options
    pub fn with_options(mut self, include_internal: bool, extract_examples: bool) -> Self {
        self.include_internal = include_internal;
        self.extract_examples = extract_examples;
        self
    }

    /// Extract all standard library documentation
    pub fn extract_stdlib_documentation(&self) -> Result<Vec<ExtractedDocumentation>, Error> {
        let stdlib_path = self.base_path.join("src/stdlib");
        if !stdlib_path.exists() {
            return Err(Error::General("Standard library path not found".to_string()));
        }

        let mut all_docs = Vec::new();

        // Extract from major stdlib modules
        let stdlib_modules = vec![
            ("math", "Mathematical functions and constants"),
            ("io", "Input/output operations"),
            ("crypto", "Cryptographic functions"),
            ("database", "Database connectivity and ORM"),
            ("web", "Web framework and HTTP utilities"),
            ("collections", "Data structures and collections"),
            ("string", "String manipulation utilities"),
            ("time", "Date and time operations"),
            ("fs", "File system operations"),
            ("process", "Process management and IPC"),
            ("concurrency", "Concurrency primitives"),
            ("memory", "Memory management utilities"),
        ];

        for (module_name, description) in stdlib_modules {
            let module_path = stdlib_path.join(module_name);
            if module_path.exists() {
                match self.extract_module_documentation(&module_path, module_name, description) {
                    Ok(mut docs) => all_docs.append(&mut docs),
                    Err(e) => eprintln!("Warning: Failed to extract {} module: {}", module_name, e),
                }
            }
        }

        // Extract from root stdlib files
        if let Ok(entries) = fs::read_dir(&stdlib_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                        if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                            if file_name != "mod" && file_name != "lib" {
                                match self.extract_rust_file_documentation(&path) {
                                    Ok(doc) => all_docs.push(doc),
                                    Err(e) => eprintln!("Warning: Failed to extract {}: {}", path.display(), e),
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(all_docs)
    }

    /// Extract documentation from a specific module directory
    fn extract_module_documentation(&self, module_path: &Path, module_name: &str, description: &str) -> Result<Vec<ExtractedDocumentation>, Error> {
        let mut docs = Vec::new();

        // Main module file
        let mod_file = module_path.join("mod.rs");
        if mod_file.exists() {
            let mut doc = self.extract_rust_file_documentation(&mod_file)?;
            doc.module_name = format!("stdlib::{}", module_name);
            docs.push(doc);
        }

        // Submodule files
        if let Ok(entries) = fs::read_dir(module_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                        if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                            if file_name != "mod" {
                                match self.extract_rust_file_documentation(&path) {
                                    Ok(mut doc) => {
                                        doc.module_name = format!("stdlib::{}::{}", module_name, file_name);
                                        docs.push(doc);
                                    }
                                    Err(e) => eprintln!("Warning: Failed to extract {}: {}", path.display(), e),
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(docs)
    }

    /// Extract documentation from a Rust source file
    fn extract_rust_file_documentation(&self, file_path: &Path) -> Result<ExtractedDocumentation, Error> {
        let source = fs::read_to_string(file_path).map_err(Error::Io)?;
        let module_name = self.derive_module_name(file_path);
        
        let mut items = Vec::new();
        
        // Parse Rust source for documentation
        let parsed_items = self.parse_rust_source(&source)?;
        items.extend(parsed_items);
        
        // Extract source file information
        let source_info = self.gather_source_info(&source, file_path)?;
        
        Ok(ExtractedDocumentation {
            file_path: file_path.to_path_buf(),
            module_name,
            package_name: Some("stdlib".to_string()),
            imports: self.extract_imports(&source),
            items,
            source_info,
        })
    }

    /// Parse Rust source code for documentable items
    fn parse_rust_source(&self, source: &str) -> Result<Vec<DocumentationItem>, Error> {
        let mut items = Vec::new();
        let lines: Vec<&str> = source.split("\n").collect();
        
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Look for doc comments
            if line.starts_with("///") || line.starts_with("//!") {
                // Extract doc comment block
                let (doc_content, doc_end) = self.extract_doc_block(&lines, i)?;
                
                // Look for the item being documented
                if let Some(item) = self.find_documented_item(&lines, doc_end, &doc_content)? {
                    items.push(item);
                }
                
                i = doc_end + 1;
            } else {
                // Look for items without doc comments
                if let Some(item) = self.parse_rust_item(line, i + 1)? {
                    items.push(item);
                }
                i += 1;
            }
        }
        
        Ok(items)
    }

    /// Extract documentation comment block
    fn extract_doc_block(&self, lines: &[&str], start: usize) -> Result<(String, usize), Error> {
        let mut doc_lines = Vec::new();
        let mut end = start;
        
        for i in start..lines.len() {
            let line = lines[i].trim();
            if line.starts_with("///") {
                doc_lines.push(line.strip_prefix("///").unwrap_or("").trim());
                end = i;
            } else if line.starts_with("//!") {
                doc_lines.push(line.strip_prefix("//!").unwrap_or("").trim());
                end = i;
            } else if line.is_empty() && i == start + doc_lines.len() {
                // Allow empty lines within doc blocks
                continue;
            } else {
                break;
            }
        }
        
        Ok((doc_lines.join("\n"), end))
    }

    /// Find the item being documented after doc comments
    fn find_documented_item(&self, lines: &[&str], doc_end: usize, doc_content: &str) -> Result<Option<DocumentationItem>, Error> {
        // Skip empty lines after doc comments
        let mut item_start = doc_end + 1;
        while item_start < lines.len() && lines[item_start].trim().is_empty() {
            item_start += 1;
        }
        
        if item_start >= lines.len() {
            return Ok(None);
        }
        
        let item_line = lines[item_start].trim();
        
        // Parse the item with documentation
        if let Some(mut item) = self.parse_rust_item(item_line, item_start + 1)? {
            let (summary, description, examples) = self.parse_doc_content(doc_content)?;
            item.summary = summary;
            item.description = description;
            item.examples = examples;
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    /// Parse a Rust item (function, struct, etc.)
    fn parse_rust_item(&self, line: &str, line_num: usize) -> Result<Option<DocumentationItem>, Error> {
        let location = SourceLocation {
            line: line_num as u32,
            column: 1,
            file: None,
        };

        // Function declarations
        if line.starts_with("pub fn ") || line.starts_with("fn ") {
            return Ok(Some(self.parse_function_item(line, &location)?));
        }
        
        // Struct declarations
        if line.starts_with("pub struct ") || line.starts_with("struct ") {
            return Ok(Some(self.parse_struct_item(line, &location)?));
        }
        
        // Enum declarations
        if line.starts_with("pub enum ") || line.starts_with("enum ") {
            return Ok(Some(self.parse_enum_item(line, &location)?));
        }
        
        // Trait declarations
        if line.starts_with("pub trait ") || line.starts_with("trait ") {
            return Ok(Some(self.parse_trait_item(line, &location)?));
        }
        
        // Type aliases
        if line.starts_with("pub type ") || line.starts_with("type ") {
            return Ok(Some(self.parse_type_item(line, &location)?));
        }
        
        // Constants
        if line.starts_with("pub const ") || line.starts_with("const ") {
            return Ok(Some(self.parse_const_item(line, &location)?));
        }
        
        // Static variables
        if line.starts_with("pub static ") || line.starts_with("static ") {
            return Ok(Some(self.parse_static_item(line, &location)?));
        }
        
        Ok(None)
    }

    /// Parse function item
    fn parse_function_item(&self, line: &str, location: &SourceLocation) -> Result<DocumentationItem, Error> {
        let is_public = line.starts_with("pub ");
        let fn_part = if is_public { &line[4..] } else { line };
        
        // Extract function name
        let name = if let Some(paren_pos) = fn_part.find('(') {
            let name_part = &fn_part[3..paren_pos]; // Skip "fn "
            name_part.trim().to_string()
        } else {
            "unknown_function".to_string()
        };
        
        // Extract parameters (simplified)
        let parameters = if let Some(start) = fn_part.find('(') {
            if let Some(end) = fn_part.find(')') {
                let params_str = &fn_part[start + 1..end];
                self.parse_function_parameters(params_str)?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        // Extract return type
        let return_type = if let Some(arrow_pos) = fn_part.find(" -> ") {
            let after_arrow = &fn_part[arrow_pos + 4..];
            if let Some(brace_pos) = after_arrow.find(" {") {
                Some(after_arrow[..brace_pos].trim().to_string())
            } else {
                Some(after_arrow.trim().to_string())
            }
        } else {
            None
        };
        
        Ok(DocumentationItem {
            name: name.clone(),
            kind: ItemKind::Function,
            visibility: if is_public { Visibility::Public } else { Visibility::Private },
            module: "stdlib".to_string(),
            summary: format!("Function {}", name),
            description: String::new(),
            signature: Some(line.to_string()),
            parameters,
            return_type,
            examples: Vec::new(),
            tags: HashMap::new(),
            location: location.clone(),
            source_code: if self.extract_examples { Some(line.to_string()) } else { None },
        })
    }

    /// Parse struct item
    fn parse_struct_item(&self, line: &str, location: &SourceLocation) -> Result<DocumentationItem, Error> {
        let is_public = line.starts_with("pub ");
        let struct_part = if is_public { &line[4..] } else { line };
        
        // Extract struct name
        let name = if let Some(space_pos) = struct_part[7..].find(' ') { // Skip "struct "
            struct_part[7..7 + space_pos].trim().to_string()
        } else if let Some(brace_pos) = struct_part.find(" {") {
            struct_part[7..brace_pos].trim().to_string()
        } else {
            struct_part[7..].trim().to_string()
        };
        
        Ok(DocumentationItem {
            name: name.clone(),
            kind: ItemKind::Struct,
            visibility: if is_public { Visibility::Public } else { Visibility::Private },
            module: "stdlib".to_string(),
            summary: format!("Struct {}", name),
            description: String::new(),
            signature: Some(line.to_string()),
            parameters: Vec::new(),
            return_type: None,
            examples: Vec::new(),
            tags: HashMap::new(),
            location: location.clone(),
            source_code: if self.extract_examples { Some(line.to_string()) } else { None },
        })
    }

    /// Parse enum item
    fn parse_enum_item(&self, line: &str, location: &SourceLocation) -> Result<DocumentationItem, Error> {
        let is_public = line.starts_with("pub ");
        let enum_part = if is_public { &line[4..] } else { line };
        
        // Extract enum name
        let name = if let Some(space_pos) = enum_part[5..].find(' ') { // Skip "enum "
            enum_part[5..5 + space_pos].trim().to_string()
        } else if let Some(brace_pos) = enum_part.find(" {") {
            enum_part[5..brace_pos].trim().to_string()
        } else {
            enum_part[5..].trim().to_string()
        };
        
        Ok(DocumentationItem {
            name: name.clone(),
            kind: ItemKind::Type, // Treat enums as types
            visibility: if is_public { Visibility::Public } else { Visibility::Private },
            module: "stdlib".to_string(),
            summary: format!("Enum {}", name),
            description: String::new(),
            signature: Some(line.to_string()),
            parameters: Vec::new(),
            return_type: None,
            examples: Vec::new(),
            tags: HashMap::new(),
            location: location.clone(),
            source_code: if self.extract_examples { Some(line.to_string()) } else { None },
        })
    }

    /// Parse trait item
    fn parse_trait_item(&self, line: &str, location: &SourceLocation) -> Result<DocumentationItem, Error> {
        let is_public = line.starts_with("pub ");
        let trait_part = if is_public { &line[4..] } else { line };
        
        // Extract trait name
        let name = if let Some(space_pos) = trait_part[6..].find(' ') { // Skip "trait "
            trait_part[6..6 + space_pos].trim().to_string()
        } else if let Some(brace_pos) = trait_part.find(" {") {
            trait_part[6..brace_pos].trim().to_string()
        } else {
            trait_part[6..].trim().to_string()
        };
        
        Ok(DocumentationItem {
            name: name.clone(),
            kind: ItemKind::Interface, // Treat traits as interfaces
            visibility: if is_public { Visibility::Public } else { Visibility::Private },
            module: "stdlib".to_string(),
            summary: format!("Trait {}", name),
            description: String::new(),
            signature: Some(line.to_string()),
            parameters: Vec::new(),
            return_type: None,
            examples: Vec::new(),
            tags: HashMap::new(),
            location: location.clone(),
            source_code: if self.extract_examples { Some(line.to_string()) } else { None },
        })
    }

    /// Parse type alias item
    fn parse_type_item(&self, line: &str, location: &SourceLocation) -> Result<DocumentationItem, Error> {
        let is_public = line.starts_with("pub ");
        let type_part = if is_public { &line[4..] } else { line };
        
        // Extract type name
        let name = if let Some(eq_pos) = type_part.find(" = ") {
            type_part[5..eq_pos].trim().to_string() // Skip "type "
        } else {
            type_part[5..].trim().to_string()
        };
        
        Ok(DocumentationItem {
            name: name.clone(),
            kind: ItemKind::Type,
            visibility: if is_public { Visibility::Public } else { Visibility::Private },
            module: "stdlib".to_string(),
            summary: format!("Type alias {}", name),
            description: String::new(),
            signature: Some(line.to_string()),
            parameters: Vec::new(),
            return_type: None,
            examples: Vec::new(),
            tags: HashMap::new(),
            location: location.clone(),
            source_code: if self.extract_examples { Some(line.to_string()) } else { None },
        })
    }

    /// Parse constant item
    fn parse_const_item(&self, line: &str, location: &SourceLocation) -> Result<DocumentationItem, Error> {
        let is_public = line.starts_with("pub ");
        let const_part = if is_public { &line[4..] } else { line };
        
        // Extract constant name
        let name = if let Some(colon_pos) = const_part.find(':') {
            const_part[6..colon_pos].trim().to_string() // Skip "const "
        } else {
            const_part[6..].trim().to_string()
        };
        
        Ok(DocumentationItem {
            name: name.clone(),
            kind: ItemKind::Constant,
            visibility: if is_public { Visibility::Public } else { Visibility::Private },
            module: "stdlib".to_string(),
            summary: format!("Constant {}", name),
            description: String::new(),
            signature: Some(line.to_string()),
            parameters: Vec::new(),
            return_type: None,
            examples: Vec::new(),
            tags: HashMap::new(),
            location: location.clone(),
            source_code: if self.extract_examples { Some(line.to_string()) } else { None },
        })
    }

    /// Parse static variable item
    fn parse_static_item(&self, line: &str, location: &SourceLocation) -> Result<DocumentationItem, Error> {
        let is_public = line.starts_with("pub ");
        let static_part = if is_public { &line[4..] } else { line };
        
        // Extract static name
        let name = if let Some(colon_pos) = static_part.find(':') {
            static_part[7..colon_pos].trim().to_string() // Skip "static "
        } else {
            static_part[7..].trim().to_string()
        };
        
        Ok(DocumentationItem {
            name: name.clone(),
            kind: ItemKind::Variable,
            visibility: if is_public { Visibility::Public } else { Visibility::Private },
            module: "stdlib".to_string(),
            summary: format!("Static variable {}", name),
            description: String::new(),
            signature: Some(line.to_string()),
            parameters: Vec::new(),
            return_type: None,
            examples: Vec::new(),
            tags: HashMap::new(),
            location: location.clone(),
            source_code: if self.extract_examples { Some(line.to_string()) } else { None },
        })
    }

    /// Parse function parameters
    fn parse_function_parameters(&self, params_str: &str) -> Result<Vec<Parameter>, Error> {
        let mut parameters = Vec::new();
        
        if params_str.trim().is_empty() {
            return Ok(parameters);
        }
        
        // Simple parameter parsing (not handling complex types)
        for param in params_str.split(',') {
            let param = param.trim();
            if let Some(colon_pos) = param.find(':') {
                let name = param[..colon_pos].trim();
                let type_str = param[colon_pos + 1..].trim();
                
                // Skip 'self' parameters
                if name == "self" || name == "&self" || name == "&mut self" {
                    continue;
                }
                
                parameters.push(Parameter {
                    name: name.to_string(),
                    type_name: Some(type_str.to_string()),
                    description: format!("Parameter {}", name),
                    default_value: None,
                });
            }
        }
        
        Ok(parameters)
    }

    /// Parse documentation content
    fn parse_doc_content(&self, content: &str) -> Result<(String, String, Vec<Example>), Error> {
        let lines: Vec<&str> = content.split("\n").collect();
        let mut summary = String::new();
        let mut description_lines = Vec::new();
        let mut examples = Vec::new();
        
        let mut in_example = false;
        let mut example_lines = Vec::new();
        let mut example_lang = "rust".to_string();
        
        for (i, line) in lines.iter().enumerate() {
            let line = line.trim();
            
            if line.starts_with("```") {
                if in_example {
                    // End example
                    examples.push(Example {
                        title: None,
                        description: None,
                        code: example_lines.join("\n"),
                        language: example_lang.clone(),
                        output: None,
                    });
                    example_lines.clear();
                    in_example = false;
                } else {
                    // Start example
                    example_lang = line.strip_prefix("```").unwrap_or("rust").to_string();
                    if example_lang.is_empty() {
                        example_lang = "rust".to_string();
                    }
                    in_example = true;
                }
            } else if in_example {
                example_lines.push(line.to_string());
            } else if i == 0 && summary.is_empty() {
                summary = line.to_string();
            } else if !line.is_empty() {
                description_lines.push(line.to_string());
            }
        }
        
        let description = description_lines.join(" ");
        
        Ok((summary, description, examples))
    }

    /// Extract imports from source code
    fn extract_imports(&self, source: &str) -> Vec<String> {
        let mut imports = Vec::new();
        
        for line in source.split("\n") {
            let line = line.trim();
            if line.starts_with("use ") && line.ends_with(';') {
                let import = line[4..line.len() - 1].trim(); // Remove "use " and ";"
                imports.push(import.to_string());
            }
        }
        
        imports
    }

    /// Derive module name from file path
    fn derive_module_name(&self, file_path: &Path) -> String {
        let relative_path = file_path.strip_prefix(&self.base_path)
            .unwrap_or(file_path);
        
        let mut name_parts = Vec::new();
        
        for component in relative_path.components() {
            if let Some(part) = component.as_os_str().to_str() {
                if part != "src" && part != "mod.rs" {
                    if part.ends_with(".rs") {
                        name_parts.push(&part[..part.len() - 3]);
                    } else {
                        name_parts.push(part);
                    }
                }
            }
        }
        
        name_parts.join("::")
    }

    /// Gather source file information
    fn gather_source_info(&self, source: &str, file_path: &Path) -> Result<SourceInfo, Error> {
        let file_size = source.len() as u64;
        let line_count = source.split("\n").count();
        
        let last_modified = fs::metadata(file_path)
            .ok()
            .and_then(|meta| meta.modified().ok());

        Ok(SourceInfo {
            file_size,
            line_count,
            last_modified,
            encoding: "UTF-8".to_string(),
        })
    }

    /// Generate comprehensive stdlib API reference
    pub fn generate_api_reference(&self, output_path: &Path) -> Result<(), Error> {
        let docs = self.extract_stdlib_documentation()?;
        
        // Create comprehensive API reference structure
        let api_ref = serde_json::json!({
            "api_version": "1.0.0",
            "generated_at": chrono::Utc::now().to_rfc3339(),
            "language": "cursed",
            "stdlib_version": env!("CARGO_PKG_VERSION"),
            "modules": docs.len(),
            "total_items": docs.iter().map(|d| d.items.len()).sum::<usize>(),
            "documentation": docs
        });
        
        let api_content = serde_json::to_string_pretty(&api_ref)
            .map_err(|e| Error::General(format!("Failed to serialize API reference: {}", e)))?;
        
        fs::write(output_path, api_content).map_err(Error::Io)?;
        Ok(())
    }
}
