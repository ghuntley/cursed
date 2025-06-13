//! JSON Documentation Generator
//! 
//! Generates JSON documentation for API consumption and tooling integration.

use super::*;
use std::fs;
use std::path::Path;
use serde_json;

/// JSON documentation generator
pub struct JsonGenerator<'a> {
    config: &'a DocGeneratorConfig,
}

impl<'a> JsonGenerator<'a> {
    pub fn new(config: &'a DocGeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate comprehensive JSON documentation
    pub fn generate_documentation(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> Result<(), Error> {
        let json_doc = self.build_json_documentation(docs)?;
        let json_path = output_dir.join("documentation.json");
        
        let json_string = serde_json::to_string_pretty(&json_doc)
            .map_err(|e| Error::Parse(e.to_string()))?;
        
        fs::write(json_path, json_string).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate search index as JSON
    pub fn generate_search_index(&self, search_index: &[SearchIndexEntry], output_dir: &Path) -> Result<(), Error> {
        let index_path = output_dir.join("search-index.json");
        
        let json_string = serde_json::to_string_pretty(search_index)
            .map_err(|e| Error::Parse(e.to_string()))?;
        
        fs::write(index_path, json_string).map_err(Error::Io)?;
        Ok(())
    }

    /// Build comprehensive JSON documentation structure
    fn build_json_documentation(&self, docs: &[ExtractedDocumentation]) -> Result<JsonDocumentation, Error> {
        let metadata = JsonMetadata {
            title: self.config.title.clone(),
            description: self.config.description.clone(),
            version: self.config.version.clone(),
            authors: self.config.authors.clone(),
            generated_at: chrono::Utc::now(),
            generator_version: env!("CARGO_PKG_VERSION").to_string(),
            total_modules: docs.len(),
            total_items: docs.iter().map(|d| d.items.len()).sum(),
        };

        let modules: Vec<JsonModule> = docs.iter()
            .map(|doc| self.build_json_module(doc))
            .collect::<Result<Vec<_>, _>>()?;

        let statistics = self.build_statistics(docs);

        Ok(JsonDocumentation {
            metadata,
            modules,
            statistics,
        })
    }

    /// Build JSON representation of a module
    fn build_json_module(&self, doc: &ExtractedDocumentation) -> Result<JsonModule, Error> {
        let items: Vec<JsonItem> = doc.items.iter()
            .map(|item| self.build_json_item(item))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(JsonModule {
            name: doc.module_name.clone(),
            package_name: doc.package_name.clone(),
            file_path: doc.file_path.to_string_lossy().to_string(),
            imports: doc.imports.clone(),
            items,
            source_info: JsonSourceInfo {
                file_size: doc.source_info.file_size,
                line_count: doc.source_info.line_count,
                last_modified: doc.source_info.last_modified,
                encoding: doc.source_info.encoding.clone(),
            },
        })
    }

    /// Build JSON representation of a documentation item
    fn build_json_item(&self, item: &DocumentationItem) -> Result<JsonItem, Error> {
        Ok(JsonItem {
            name: item.name.clone(),
            kind: item.kind.clone(),
            visibility: item.visibility.clone(),
            module: item.module.clone(),
            summary: item.summary.clone(),
            description: item.description.clone(),
            signature: item.signature.clone(),
            parameters: item.parameters.iter()
                .map(|p| JsonParameter {
                    name: p.name.clone(),
                    type_name: p.type_name.clone(),
                    description: p.description.clone(),
                    default_value: p.default_value.clone(),
                })
                .collect(),
            return_type: item.return_type.clone(),
            examples: item.examples.iter()
                .map(|e| JsonExample {
                    title: e.title.clone(),
                    description: e.description.clone(),
                    code: e.code.clone(),
                    language: e.language.clone(),
                    output: e.output.clone(),
                })
                .collect(),
            tags: item.tags.clone(),
            location: JsonSourceLocation {
                line: item.location.line,
                column: item.location.column,
                file: item.location.file.clone(),
            },
            source_code: item.source_code.clone(),
        })
    }

    /// Build documentation statistics
    fn build_statistics(&self, docs: &[ExtractedDocumentation]) -> JsonStatistics {
        let mut stats = JsonStatistics {
            total_modules: docs.len(),
            total_items: 0,
            items_by_kind: std::collections::HashMap::new(),
            items_by_visibility: std::collections::HashMap::new(),
            average_items_per_module: 0.0,
            largest_module: None,
            smallest_module: None,
            total_source_lines: 0,
            total_source_bytes: 0,
        };

        let mut module_sizes = Vec::new();

        for doc in docs {
            let item_count = doc.items.len();
            stats.total_items += item_count;
            stats.total_source_lines += doc.source_info.line_count;
            stats.total_source_bytes += doc.source_info.file_size;

            module_sizes.push((doc.module_name.clone(), item_count));

            // Count by kind
            for item in &doc.items {
                *stats.items_by_kind.entry(item.kind.to_string()).or_insert(0) += 1;
                
                let visibility = match item.visibility {
                    Visibility::Public => "public",
                    Visibility::Private => "private",
                };
                *stats.items_by_visibility.entry(visibility.to_string()).or_insert(0) += 1;
            }
        }

        // Calculate averages and extremes
        if !docs.is_empty() {
            stats.average_items_per_module = stats.total_items as f64 / docs.len() as f64;
            
            module_sizes.sort_by_key(|(_, size)| *size);
            
            if let Some((name, _)) = module_sizes.last() {
                stats.largest_module = Some(name.clone());
            }
            
            if let Some((name, _)) = module_sizes.first() {
                stats.smallest_module = Some(name.clone());
            }
        }

        stats
    }
}

/// JSON documentation root structure
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonDocumentation {
    pub metadata: JsonMetadata,
    pub modules: Vec<JsonModule>,
    pub statistics: JsonStatistics,
}

/// Documentation metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonMetadata {
    pub title: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub authors: Vec<String>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub generator_version: String,
    pub total_modules: usize,
    pub total_items: usize,
}

/// JSON module representation
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonModule {
    pub name: String,
    pub package_name: Option<String>,
    pub file_path: String,
    pub imports: Vec<String>,
    pub items: Vec<JsonItem>,
    pub source_info: JsonSourceInfo,
}

/// JSON item representation
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonItem {
    pub name: String,
    pub kind: ItemKind,
    pub visibility: Visibility,
    pub module: String,
    pub summary: String,
    pub description: String,
    pub signature: Option<String>,
    pub parameters: Vec<JsonParameter>,
    pub return_type: Option<String>,
    pub examples: Vec<JsonExample>,
    pub tags: HashMap<String, Vec<String>>,
    pub location: JsonSourceLocation,
    pub source_code: Option<String>,
}

/// JSON parameter representation
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonParameter {
    pub name: String,
    pub type_name: Option<String>,
    pub description: String,
    pub default_value: Option<String>,
}

/// JSON example representation
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonExample {
    pub title: Option<String>,
    pub description: Option<String>,
    pub code: String,
    pub language: String,
    pub output: Option<String>,
}

/// JSON source location
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonSourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

/// JSON source info
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonSourceInfo {
    pub file_size: u64,
    pub line_count: usize,
    pub last_modified: Option<std::time::SystemTime>,
    pub encoding: String,
}

/// Documentation statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonStatistics {
    pub total_modules: usize,
    pub total_items: usize,
    pub items_by_kind: std::collections::HashMap<String, usize>,
    pub items_by_visibility: std::collections::HashMap<String, usize>,
    pub average_items_per_module: f64,
    pub largest_module: Option<String>,
    pub smallest_module: Option<String>,
    pub total_source_lines: usize,
    pub total_source_bytes: u64,
}
