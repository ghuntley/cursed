//! JSON Documentation Generator
//! 
//! Generates machine-readable JSON documentation for API tools and IDEs.

use super::{DocGeneratorConfig, ExtractedDocumentation, DocumentationItem, SearchIndexEntry};
use crate::error::Error;
use std::path::Path;
use std::fs;
use serde_json::{json, Value};

/// JSON documentation generator
pub struct JsonGenerator<'a> {
    config: &'a DocGeneratorConfig,
}

impl<'a> JsonGenerator<'a> {
    /// Create a new JSON generator
    pub fn new(config: &'a DocGeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate comprehensive JSON documentation
    pub fn generate_documentation(
        &self,
        docs: &[ExtractedDocumentation],
        output_dir: &Path,
    ) -> Result<(), Error> {
        // Generate main documentation file
        let main_doc = self.build_main_documentation(docs)?;
        let main_json = serde_json::to_string_pretty(&main_doc)
            .map_err(|e| Error::GenerationError(format!("Failed to serialize main documentation: {}", e)))?;
        
        let main_file = output_dir.join("documentation.json");
        fs::write(&main_file, main_json).map_err(Error::Io)?;

        // Generate individual module files
        for doc in docs {
            let module_doc = self.build_module_documentation(doc)?;
            let module_json = serde_json::to_string_pretty(&module_doc)
                .map_err(|e| Error::GenerationError(format!("Failed to serialize module documentation: {}", e)))?;
            
            let module_filename = format!("{}.json", doc.module_name.replace("::", "_"));
            let module_file = output_dir.join(module_filename);
            fs::write(&module_file, module_json).map_err(Error::Io)?;
        }

        // Generate API index for quick access
        let api_index = self.build_api_index(docs)?;
        let api_json = serde_json::to_string_pretty(&api_index)
            .map_err(|e| Error::GenerationError(format!("Failed to serialize API index: {}", e)))?;
        
        let api_file = output_dir.join("api-index.json");
        fs::write(&api_file, api_json).map_err(Error::Io)?;

        Ok(())
    }

    /// Generate search index JSON file
    pub fn generate_search_index(
        &self,
        search_index: &[SearchIndexEntry],
        output_dir: &Path,
    ) -> Result<(), Error> {
        let search_data = json!({
            "version": "1.0",
            "generated_at": chrono::Utc::now().to_rfc3339(),
            "generator": "CURSED Documentation Generator",
            "index": search_index
        });

        let search_json = serde_json::to_string_pretty(&search_data)
            .map_err(|e| Error::GenerationError(format!("Failed to serialize search index: {}", e)))?;

        let search_file = output_dir.join("search-index.json");
        fs::write(&search_file, search_json).map_err(Error::Io)?;

        Ok(())
    }

    /// Build main documentation structure
    fn build_main_documentation(&self, docs: &[ExtractedDocumentation]) -> Result<Value, Error> {
        let mut statistics = serde_json::Map::new();
        let mut total_items = 0;
        let mut item_counts = serde_json::Map::new();

        // Calculate statistics
        for doc in docs {
            total_items += doc.items.len();
            for item in &doc.items {
                let key = format!("{}_count", item.kind.to_string());
                let current = item_counts.get(&key)
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                item_counts.insert(key, json!(current + 1));
            }
        }

        statistics.insert("total_modules".to_string(), json!(docs.len()));
        statistics.insert("total_items".to_string(), json!(total_items));
        statistics.insert("item_counts".to_string(), json!(item_counts));

        // Build module index
        let modules: Vec<Value> = docs.iter().map(|doc| {
            json!({
                "name": doc.module_name,
                "file_path": doc.file_path.display().to_string(),
                "package_name": doc.package_name,
                "item_count": doc.items.len(),
                "imports": doc.imports,
                "source_info": {
                    "file_size": doc.source_info.file_size,
                    "line_count": doc.source_info.line_count,
                    "last_modified": doc.source_info.last_modified
                        .map(|t| t.duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default().as_secs()),
                    "encoding": doc.source_info.encoding
                }
            })
        }).collect();

        Ok(json!({
            "cursed_documentation": {
                "version": "1.0",
                "generated_at": chrono::Utc::now().to_rfc3339(),
                "generator": "CURSED Documentation Generator",
                "project": {
                    "title": self.config.title,
                    "description": self.config.description,
                    "version": self.config.version,
                    "authors": self.config.authors,
                    "base_url": self.config.base_url
                },
                "configuration": {
                    "include_private": self.config.include_private,
                    "include_examples": self.config.include_examples,
                    "generate_cross_refs": self.config.generate_cross_refs,
                    "format": self.config.format.to_string()
                },
                "statistics": statistics,
                "modules": modules
            }
        }))
    }

    /// Build documentation for a single module
    fn build_module_documentation(&self, doc: &ExtractedDocumentation) -> Result<Value, Error> {
        let items: Vec<Value> = doc.items.iter()
            .map(|item| self.item_to_json(item))
            .collect();

        Ok(json!({
            "module": {
                "name": doc.module_name,
                "file_path": doc.file_path.display().to_string(),
                "package_name": doc.package_name,
                "imports": doc.imports,
                "source_info": {
                    "file_size": doc.source_info.file_size,
                    "line_count": doc.source_info.line_count,
                    "last_modified": doc.source_info.last_modified
                        .map(|t| t.duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default().as_secs()),
                    "encoding": doc.source_info.encoding
                },
                "items": items,
                "generated_at": chrono::Utc::now().to_rfc3339()
            }
        }))
    }

    /// Build API index for quick access
    fn build_api_index(&self, docs: &[ExtractedDocumentation]) -> Result<Value, Error> {
        let mut by_kind = serde_json::Map::new();
        let mut by_module = serde_json::Map::new();

        for doc in docs {
            let mut module_items = Vec::new();
            
            for item in &doc.items {
                let item_ref = json!({
                    "name": item.name,
                    "kind": item.kind.to_string(),
                    "module": item.module,
                    "summary": item.summary,
                    "signature": item.signature,
                    "visibility": format!("{:?}", item.visibility).to_lowercase()
                });

                // Group by kind
                let kind_key = item.kind.to_string();
                let kind_items = by_kind.entry(kind_key.clone())
                    .or_insert_with(|| json!([]));
                if let Some(items_array) = kind_items.as_array_mut() {
                    items_array.push(item_ref.clone());
                }

                module_items.push(item_ref);
            }

            by_module.insert(doc.module_name.clone(), json!(module_items));
        }

        Ok(json!({
            "api_index": {
                "version": "1.0",
                "generated_at": chrono::Utc::now().to_rfc3339(),
                "by_kind": by_kind,
                "by_module": by_module
            }
        }))
    }

    /// Convert documentation item to JSON
    fn item_to_json(&self, item: &DocumentationItem) -> Value {
        let mut json_item = serde_json::Map::new();
        
        json_item.insert("name".to_string(), json!(item.name));
        json_item.insert("kind".to_string(), json!(item.kind.to_string()));
        json_item.insert("visibility".to_string(), json!(format!("{:?}", item.visibility).to_lowercase()));
        json_item.insert("module".to_string(), json!(item.module));
        json_item.insert("summary".to_string(), json!(item.summary));
        json_item.insert("description".to_string(), json!(item.description));
        
        if let Some(signature) = &item.signature {
            json_item.insert("signature".to_string(), json!(signature));
        }
        
        if let Some(return_type) = &item.return_type {
            json_item.insert("return_type".to_string(), json!(return_type));
        }

        // Parameters
        let parameters: Vec<Value> = item.parameters.iter().map(|p| {
            let mut param = serde_json::Map::new();
            param.insert("name".to_string(), json!(p.name));
            param.insert("description".to_string(), json!(p.description));
            if let Some(type_name) = &p.type_name {
                param.insert("type".to_string(), json!(type_name));
            }
            if let Some(default) = &p.default_value {
                param.insert("default".to_string(), json!(default));
            }
            json!(param)
        }).collect();
        json_item.insert("parameters".to_string(), json!(parameters));

        // Examples
        let examples: Vec<Value> = item.examples.iter().map(|ex| {
            let mut example = serde_json::Map::new();
            if let Some(title) = &ex.title {
                example.insert("title".to_string(), json!(title));
            }
            if let Some(description) = &ex.description {
                example.insert("description".to_string(), json!(description));
            }
            example.insert("code".to_string(), json!(ex.code));
            example.insert("language".to_string(), json!(ex.language));
            if let Some(output) = &ex.output {
                example.insert("output".to_string(), json!(output));
            }
            json!(example)
        }).collect();
        json_item.insert("examples".to_string(), json!(examples));

        // Tags
        json_item.insert("tags".to_string(), json!(item.tags));

        // Location
        json_item.insert("location".to_string(), json!({
            "line": item.location.line,
            "column": item.location.column,
            "file": item.location.file
        }));

        // Source code (if included)
        if let Some(source_code) = &item.source_code {
            if self.config.include_examples {
                json_item.insert("source_code".to_string(), json!(source_code));
            }
        }

        json!(json_item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SourceLocation;
    use std::collections::HashMap;

    fn create_test_config() -> DocGeneratorConfig {
        DocGeneratorConfig {
            output_dir: std::path::PathBuf::from("test_docs"),
            format: super::super::DocFormat::Json,
            include_examples: true,
            include_private: false,
            generate_cross_refs: true,
            custom_css: None,
            template_dir: None,
            title: "Test Documentation".to_string(),
            description: Some("Test project documentation".to_string()),
            version: Some("1.0.0".to_string()),
            authors: vec!["Test Author".to_string()],
            base_url: None,
        }
    }

    fn create_test_item() -> DocumentationItem {
        DocumentationItem {
            name: "test_function".to_string(),
            kind: super::super::ItemKind::Function,
            visibility: super::super::Visibility::Public,
            module: "test_module".to_string(),
            summary: "A test function".to_string(),
            description: "This is a test function for unit testing".to_string(),
            signature: Some("slay test_function(param: string) -> i32".to_string()),
            parameters: vec![super::super::Parameter {
                name: "param".to_string(),
                type_name: Some("string".to_string()),
                description: "A test parameter".to_string(),
                default_value: None,
            }],
            return_type: Some("i32".to_string()),
            examples: vec![super::super::Example {
                title: Some("Basic Usage".to_string()),
                description: Some("Shows how to use the function".to_string()),
                code: "let result = test_function(\"hello\");".to_string(),
                language: "cursed".to_string(),
                output: Some("42".to_string()),
            }],
            tags: HashMap::new(),
            location: SourceLocation { line: 1, column: 1, file: None },
            source_code: Some("slay test_function(param: string) -> i32 {\n    return 42;\n}".to_string()),
        }
    }

    #[test]
    fn test_item_to_json() {
        let config = create_test_config();
        let generator = JsonGenerator::new(&config);
        let item = create_test_item();
        
        let json_item = generator.item_to_json(&item);
        
        assert_eq!(json_item["name"], "test_function");
        assert_eq!(json_item["kind"], "function");
        assert_eq!(json_item["visibility"], "public");
        assert_eq!(json_item["module"], "test_module");
        assert_eq!(json_item["summary"], "A test function");
        assert_eq!(json_item["signature"], "slay test_function(param: string) -> i32");
        assert_eq!(json_item["return_type"], "i32");
        
        let parameters = json_item["parameters"].as_array().unwrap();
        assert_eq!(parameters.len(), 1);
        assert_eq!(parameters[0]["name"], "param");
        assert_eq!(parameters[0]["type"], "string");
        
        let examples = json_item["examples"].as_array().unwrap();
        assert_eq!(examples.len(), 1);
        assert_eq!(examples[0]["title"], "Basic Usage");
        assert_eq!(examples[0]["language"], "cursed");
        
        assert!(json_item["source_code"].as_str().unwrap().contains("slay test_function"));
    }

    #[test]
    fn test_build_api_index() {
        let config = create_test_config();
        let generator = JsonGenerator::new(&config);
        
        let doc = ExtractedDocumentation {
            file_path: std::path::PathBuf::from("test.csd"),
            module_name: "test_module".to_string(),
            package_name: None,
            imports: vec![],
            items: vec![create_test_item()],
            source_info: super::super::SourceInfo {
                file_size: 100,
                line_count: 5,
                last_modified: None,
                encoding: "UTF-8".to_string(),
            },
        };
        
        let api_index = generator.build_api_index(&[doc]).unwrap();
        
        let by_kind = &api_index["api_index"]["by_kind"];
        assert!(by_kind["function"].is_array());
        
        let by_module = &api_index["api_index"]["by_module"];
        assert!(by_module["test_module"].is_array());
    }
}
