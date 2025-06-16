//! JSON Documentation Generator
//! 
//! Generates structured JSON documentation for API integration,
//! tooling support, and programmatic documentation consumption.

use crate::docs::generator::{DocGeneratorConfig, ExtractedDocumentation, SearchIndexEntry};
use crate::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use serde_json::{json, Value};

pub struct JsonGenerator<'a> {
    config: &'a DocGeneratorConfig,
}

impl<'a> JsonGenerator<'a> {
    pub fn new(config: &'a DocGeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate comprehensive JSON documentation
    pub fn generate_documentation(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> Result<(), Error> {
        let doc_path = output_dir.join("documentation.json");
        
        let json_doc = json!({
            "project": {
                "name": self.config.title,
                "description": self.config.description,
                "version": self.config.version,
                "authors": self.config.authors,
                "base_url": self.config.base_url,
                "generated_at": chrono::Utc::now().to_rfc3339(),
                "generator": "CURSED Documentation Generator",
                "format_version": "1.0.0"
            },
            "configuration": {
                "include_private": self.config.include_private,
                "include_examples": self.config.include_examples,
                "generate_cross_refs": self.config.generate_cross_refs,
                "output_format": "json"
            },
            "statistics": self.generate_statistics(docs),
            "modules": self.generate_modules_json(docs),
            "api_reference": self.generate_api_reference(docs),
            "type_definitions": self.generate_type_definitions(docs),
            "examples": self.generate_examples_json(docs),
            "keywords_guide": self.generate_keywords_guide(),
            "metadata": {
                "total_files": docs.len(),
                "total_items": docs.iter().map(|d| d.items.len()).sum::<usize>(),
                "language": "CURSED",
                "slang_style": "Gen Z"
            }
        });
        
        let formatted_json = serde_json::to_string_pretty(&json_doc)
            .map_err(|e| Error::General(format!("JSON serialization failed: {}", e)))?;
        
        fs::write(doc_path, formatted_json).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate search index JSON
    pub fn generate_search_index(&self, index: &[SearchIndexEntry], output_dir: &Path) -> Result<(), Error> {
        let index_path = output_dir.join("search_index.json");
        
        let search_json = json!({
            "search_index": {
                "version": "1.0.0",
                "generated_at": chrono::Utc::now().to_rfc3339(),
                "total_entries": index.len(),
                "entries": index.iter().map(|entry| {
                    json!({
                        "name": entry.name,
                        "kind": entry.kind.to_string(),
                        "description": entry.description,
                        "module": entry.module,
                        "url": entry.url,
                        "keywords": entry.keywords,
                        "search_weight": self.calculate_search_weight(entry)
                    })
                }).collect::<Vec<_>>()
            },
            "search_config": {
                "fuzzy_matching": true,
                "case_sensitive": false,
                "max_results": 50,
                "search_fields": ["name", "description", "keywords", "module"]
            }
        });
        
        let formatted_json = serde_json::to_string_pretty(&search_json)
            .map_err(|e| Error::General(format!("Search index JSON serialization failed: {}", e)))?;
        
        fs::write(index_path, formatted_json).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate module-specific JSON files
    pub fn generate_module_json(&self, doc: &ExtractedDocumentation, output_dir: &Path) -> Result<(), Error> {
        let module_file = format!("{}.json", doc.module_name.replace("::", "_"));
        let module_path = output_dir.join(module_file);
        
        let module_json = json!({
            "module": {
                "name": doc.module_name,
                "package": doc.package_name,
                "file_path": doc.file_path.to_string_lossy(),
                "source_info": {
                    "file_size": doc.source_info.file_size,
                    "line_count": doc.source_info.line_count,
                    "encoding": doc.source_info.encoding,
                    "last_modified": doc.source_info.last_modified.map(|t| {
                        chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339()
                    })
                },
                "imports": doc.imports,
                "statistics": {
                    "total_items": doc.items.len(),
                    "functions": doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function)).count(),
                    "structs": doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Struct)).count(),
                    "interfaces": doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Interface)).count(),
                    "variables": doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Variable)).count(),
                    "constants": doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Constant)).count()
                },
                "items": doc.items.iter().map(|item| self.item_to_json(item)).collect::<Vec<_>>()
            }
        });
        
        let formatted_json = serde_json::to_string_pretty(&module_json)
            .map_err(|e| Error::General(format!("Module JSON serialization failed: {}", e)))?;
        
        fs::write(module_path, formatted_json).map_err(Error::Io)?;
        Ok(())
    }

    /// Convert documentation item to JSON
    fn item_to_json(&self, item: &crate::docs::generator::DocumentationItem) -> Value {
        json!({
            "name": item.name,
            "kind": item.kind.to_string(),
            "visibility": format!("{:?}", item.visibility).to_lowercase(),
            "module": item.module,
            "summary": item.summary,
            "description": item.description,
            "signature": item.signature,
            "parameters": item.parameters.iter().map(|param| {
                json!({
                    "name": param.name,
                    "type": param.type_name,
                    "description": param.description,
                    "default_value": param.default_value,
                    "is_optional": param.default_value.is_some()
                })
            }).collect::<Vec<_>>(),
            "return_type": item.return_type,
            "examples": item.examples.iter().map(|example| {
                json!({
                    "title": example.title,
                    "description": example.description,
                    "code": example.code,
                    "language": example.language,
                    "output": example.output
                })
            }).collect::<Vec<_>>(),
            "tags": item.tags,
            "location": {
                "line": item.location.line,
                "column": item.location.column,
                "file": item.location.file
            },
            "source_code": if self.config.include_examples { item.source_code.clone() } else { None },
            "cursed_features": self.extract_cursed_features(item)
        })
    }

    /// Generate project statistics
    fn generate_statistics(&self, docs: &[ExtractedDocumentation]) -> Value {
        let total_items = docs.iter().map(|d| d.items.len()).sum::<usize>();
        let total_functions = docs.iter().map(|d| {
            d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function)).count()
        }).sum::<usize>();
        let total_structs = docs.iter().map(|d| {
            d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Struct)).count()
        }).sum::<usize>();
        let total_interfaces = docs.iter().map(|d| {
            d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Interface)).count()
        }).sum::<usize>();
        let total_lines = docs.iter().map(|d| d.source_info.line_count).sum::<usize>();
        
        json!({
            "modules": docs.len(),
            "total_items": total_items,
            "functions": total_functions,
            "structs": total_structs,
            "interfaces": total_interfaces,
            "lines_of_code": total_lines,
            "average_items_per_module": if docs.is_empty() { 0 } else { total_items / docs.len() },
            "largest_module": docs.iter().max_by_key(|d| d.items.len())
                .map(|d| json!({
                    "name": d.module_name,
                    "items": d.items.len()
                })),
            "complexity_metrics": {
                "average_parameters_per_function": self.calculate_avg_parameters(docs),
                "functions_with_generics": self.count_generic_functions(docs),
                "interfaces_count": total_interfaces
            }
        })
    }

    /// Generate modules JSON structure
    fn generate_modules_json(&self, docs: &[ExtractedDocumentation]) -> Value {
        json!(docs.iter().map(|doc| {
            json!({
                "name": doc.module_name,
                "package": doc.package_name,
                "file_path": doc.file_path.to_string_lossy(),
                "items_count": doc.items.len(),
                "imports": doc.imports,
                "exported_items": doc.items.iter()
                    .filter(|item| matches!(item.visibility, crate::docs::generator::Visibility::Public))
                    .map(|item| item.name.clone())
                    .collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>())
    }

    /// Generate API reference
    fn generate_api_reference(&self, docs: &[ExtractedDocumentation]) -> Value {
        let mut api_ref = json!({
            "functions": [],
            "structs": [],
            "interfaces": [],
            "constants": [],
            "types": []
        });
        
        for doc in docs {
            for item in &doc.items {
                let item_json = self.item_to_json(item);
                
                match item.kind {
                    crate::docs::generator::ItemKind::Function => {
                        api_ref["functions"].as_array_mut().unwrap().push(item_json);
                    }
                    crate::docs::generator::ItemKind::Struct => {
                        api_ref["structs"].as_array_mut().unwrap().push(item_json);
                    }
                    crate::docs::generator::ItemKind::Interface => {
                        api_ref["interfaces"].as_array_mut().unwrap().push(item_json);
                    }
                    crate::docs::generator::ItemKind::Constant => {
                        api_ref["constants"].as_array_mut().unwrap().push(item_json);
                    }
                    crate::docs::generator::ItemKind::Type => {
                        api_ref["types"].as_array_mut().unwrap().push(item_json);
                    }
                    _ => {}
                }
            }
        }
        
        api_ref
    }

    /// Generate type definitions
    fn generate_type_definitions(&self, docs: &[ExtractedDocumentation]) -> Value {
        let mut types = json!({});
        
        for doc in docs {
            for item in &doc.items {
                if matches!(item.kind, crate::docs::generator::ItemKind::Struct | crate::docs::generator::ItemKind::Interface) {
                    types[&item.name] = json!({
                        "name": item.name,
                        "kind": item.kind.to_string(),
                        "module": item.module,
                        "fields": item.parameters.iter().map(|param| {
                            json!({
                                "name": param.name,
                                "type": param.type_name,
                                "optional": param.default_value.is_some()
                            })
                        }).collect::<Vec<_>>(),
                        "signature": item.signature
                    });
                }
            }
        }
        
        types
    }

    /// Generate examples JSON
    fn generate_examples_json(&self, docs: &[ExtractedDocumentation]) -> Value {
        let mut examples = Vec::new();
        
        // Getting started example
        examples.push(json!({
            "title": "Hello World",
            "description": "Basic CURSED program that prints a greeting",
            "code": "slay main() {\n    println(\"Hello, world! This is lowkey fire! 🔥\")\n}",
            "language": "cursed",
            "category": "basics",
            "tags": ["hello-world", "getting-started"]
        }));
        
        // Variables example
        examples.push(json!({
            "title": "Variables and Constants",
            "description": "Demonstrating CURSED variable declaration with Gen Z slang",
            "code": "sus name = \"bestie\"        // mutable variable\nfacts pi = 3.14159         // constant",
            "language": "cursed",
            "category": "variables",
            "tags": ["sus", "facts", "variables", "constants"]
        }));
        
        // Function example
        examples.push(json!({
            "title": "Function Definition",
            "description": "Creating functions with the 'slay' keyword",
            "code": "slay greet(name: string) {\n    println(\"Hey \" + name + \"! You're serving looks! ✨\")\n}",
            "language": "cursed",
            "category": "functions",
            "tags": ["slay", "functions", "parameters"]
        }));
        
        // Control flow example
        examples.push(json!({
            "title": "Control Flow",
            "description": "Using lowkey/highkey for conditional statements",
            "code": "lowkey (age >= 18) {\n    println(\"You're an adult, bestie!\")\n} highkey {\n    println(\"Still a baby, no cap\")\n}",
            "language": "cursed",
            "category": "control-flow",
            "tags": ["lowkey", "highkey", "conditionals"]
        }));
        
        json!(examples)
    }

    /// Generate Gen Z keywords guide
    fn generate_keywords_guide(&self) -> Value {
        json!({
            "description": "CURSED uses Gen Z slang for keywords because traditional programming is cheugy",
            "keywords": [
                {
                    "cursed": "slay",
                    "traditional": ["fn", "function"],
                    "description": "Declares a function that absolutely slays",
                    "example": "slay greet() { println(\"Hey bestie!\") }"
                },
                {
                    "cursed": "sus",
                    "traditional": ["let mut", "var"],
                    "description": "Declares a mutable variable (kinda sus if you ask me)",
                    "example": "sus count = 0"
                },
                {
                    "cursed": "facts",
                    "traditional": ["let", "const"],
                    "description": "Declares a constant/immutable value (straight facts)",
                    "example": "facts pi = 3.14159"
                },
                {
                    "cursed": "lowkey",
                    "traditional": ["if"],
                    "description": "Conditional statement (lowkey checking this condition)",
                    "example": "lowkey (x > 0) { println(\"positive vibes\") }"
                },
                {
                    "cursed": "highkey",
                    "traditional": ["else"],
                    "description": "Else clause (highkey the alternative)",
                    "example": "highkey { println(\"different energy\") }"
                },
                {
                    "cursed": "periodt",
                    "traditional": ["while"],
                    "description": "While loop (keeps going, periodt)",
                    "example": "periodt (condition) { /* do work */ }"
                },
                {
                    "cursed": "bestie",
                    "traditional": ["for"],
                    "description": "For loop (going through this together, bestie)",
                    "example": "bestie (item in list) { println(item) }"
                },
                {
                    "cursed": "flex",
                    "traditional": ["break"],
                    "description": "Break statement (flexing out of this loop)",
                    "example": "flex"
                },
                {
                    "cursed": "squad",
                    "traditional": ["struct", "class"],
                    "description": "Struct definition (organizing the squad)",
                    "example": "squad Person { name: string, age: i32 }"
                },
                {
                    "cursed": "collab",
                    "traditional": ["interface", "trait"],
                    "description": "Interface definition (collaborative vibes)",
                    "example": "collab Drawable { slay draw(self) }"
                },
                {
                    "cursed": "stan",
                    "traditional": ["async", "spawn"],
                    "description": "Spawn async operation/goroutine (we stan this concurrency)",
                    "example": "stan async_task()"
                },
                {
                    "cursed": "yolo",
                    "traditional": ["yield", "await"],
                    "description": "Yield/await operation (yolo, just sending it)",
                    "example": "yolo some_future"
                }
            ],
            "style_guide": {
                "philosophy": "CURSED embraces Gen Z culture and slang to make programming more expressive and fun",
                "naming_conventions": {
                    "functions": "snake_case with descriptive Gen Z terms",
                    "variables": "snake_case with vibes-based names",
                    "constants": "SCREAMING_SNAKE_CASE for that main character energy",
                    "types": "PascalCase because types deserve respect"
                },
                "cultural_notes": {
                    "no_cap": "When something is true/honest",
                    "lowkey": "Sort of/kind of/secretly",
                    "highkey": "Definitely/obviously/openly",
                    "periodt": "End of discussion, no arguments",
                    "bestie": "Close friend, used for iteration/collaboration",
                    "slay": "To do something exceptionally well",
                    "sus": "Suspicious/questionable (perfect for mutable variables)",
                    "facts": "Absolutely true (perfect for constants)"
                }
            }
        })
    }

    /// Extract CURSED-specific features from an item
    fn extract_cursed_features(&self, item: &crate::docs::generator::DocumentationItem) -> Value {
        let mut features = Vec::new();
        
        if let Some(signature) = &item.signature {
            if signature.contains("slay") {
                features.push("uses_slay_keyword");
            }
            if signature.contains("sus") {
                features.push("uses_sus_keyword");
            }
            if signature.contains("facts") {
                features.push("uses_facts_keyword");
            }
            if signature.contains("squad") {
                features.push("uses_squad_keyword");
            }
            if signature.contains("collab") {
                features.push("uses_collab_keyword");
            }
        }
        
        // Check for Gen Z naming patterns
        let name_lower = item.name.to_lowercase();
        if name_lower.contains("vibes") || name_lower.contains("energy") || name_lower.contains("mood") {
            features.push("gen_z_naming");
        }
        
        // Check for emoji usage
        if item.description.chars().any(|c| c as u32 > 127) {
            features.push("uses_emojis");
        }
        
        json!({
            "features": features,
            "slang_level": self.calculate_slang_level(item),
            "gen_z_score": features.len()
        })
    }

    /// Calculate search weight for an entry
    fn calculate_search_weight(&self, entry: &SearchIndexEntry) -> f64 {
        let mut weight = 1.0;
        
        // Function items get higher weight
        if matches!(entry.kind, crate::docs::generator::ItemKind::Function) {
            weight += 0.5;
        }
        
        // Public items get higher weight
        weight += 0.3;
        
        // Items with more keywords get higher weight
        weight += entry.keywords.len() as f64 * 0.1;
        
        // Short names are often more important
        if entry.name.len() < 10 {
            weight += 0.2;
        }
        
        weight
    }

    /// Calculate average parameters per function
    fn calculate_avg_parameters(&self, docs: &[ExtractedDocumentation]) -> f64 {
        let functions: Vec<_> = docs.iter()
            .flat_map(|d| &d.items)
            .filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function))
            .collect();
        
        if functions.is_empty() {
            return 0.0;
        }
        
        let total_params: usize = functions.iter().map(|f| f.parameters.len()).sum();
        total_params as f64 / functions.len() as f64
    }

    /// Count functions with generic parameters
    fn count_generic_functions(&self, docs: &[ExtractedDocumentation]) -> usize {
        docs.iter()
            .flat_map(|d| &d.items)
            .filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function))
            .filter(|i| {
                i.signature.as_ref().map_or(false, |s| s.contains('<') && s.contains('>'))
            })
            .count()
    }

    /// Calculate slang level for an item
    fn calculate_slang_level(&self, item: &crate::docs::generator::DocumentationItem) -> String {
        let slang_keywords = ["slay", "sus", "facts", "lowkey", "highkey", "periodt", "bestie", "flex", "squad", "collab", "stan", "yolo"];
        let mut slang_count = 0;
        
        if let Some(signature) = &item.signature {
            for keyword in &slang_keywords {
                if signature.contains(keyword) {
                    slang_count += 1;
                }
            }
        }
        
        // Check description for slang
        for keyword in &slang_keywords {
            if item.description.to_lowercase().contains(keyword) {
                slang_count += 1;
            }
        }
        
        match slang_count {
            0 => "basic".to_string(),
            1..=2 => "lowkey".to_string(),
            3..=4 => "highkey".to_string(),
            _ => "absolutely_iconic".to_string(),
        }
    }
}
