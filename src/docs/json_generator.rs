//! JSON Documentation Generator
//! 
//! Generates comprehensive JSON documentation for API consumption and tooling integration.

use crate::docs::generator::{DocGeneratorConfig, ExtractedDocumentation, SearchIndexEntry};
use crate::error::Error;
use serde_json;
use std::fs;
use std::path::Path;

/// JSON documentation generator
pub struct JsonGenerator {
    config: DocGeneratorConfig,
}

impl JsonGenerator {
    pub fn new(config: &DocGeneratorConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Generate comprehensive JSON documentation
    pub fn generate_documentation(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> Result<(), Error> {
        let json_path = output_dir.join("documentation.json");
        
        // Create comprehensive documentation structure
        let doc_data = serde_json::json!({
            "metadata": {
                "title": self.config.title,
                "description": self.config.description,
                "version": self.config.version,
                "authors": self.config.authors,
                "generated_at": chrono::Utc::now().to_rfc3339(),
                "generator": "CURSED Documentation System",
                "format_version": "1.0.0"
            },
            "configuration": {
                "include_examples": self.config.include_examples,
                "include_private": self.config.include_private,
                "generate_cross_refs": self.config.generate_cross_refs,
                "base_url": self.config.base_url
            },
            "statistics": {
                "total_modules": docs.len(),
                "total_items": docs.iter().map(|d| d.items.len()).sum::<usize>(),
                "total_functions": docs.iter().map(|d| 
                    d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function)).count()
                ).sum::<usize>(),
                "total_structs": docs.iter().map(|d| 
                    d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Struct)).count()
                ).sum::<usize>(),
                "total_interfaces": docs.iter().map(|d| 
                    d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Interface)).count()
                ).sum::<usize>(),
                "total_constants": docs.iter().map(|d| 
                    d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Constant)).count()
                ).sum::<usize>(),
                "total_variables": docs.iter().map(|d| 
                    d.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Variable)).count()
                ).sum::<usize>()
            },
            "modules": docs
        });
        
        let json_content = serde_json::to_string_pretty(&doc_data)
            .map_err(|e| Error::General(format!("Failed to serialize documentation: {}", e)))?;
        
        fs::write(json_path, json_content).map_err(Error::Io)?;
        
        // Generate individual module files
        for doc in docs {
            let module_path = output_dir.join(format!("{}.json", self.sanitize_module_name(&doc.module_name)));
            let module_json = serde_json::to_string_pretty(doc)
                .map_err(|e| Error::General(format!("Failed to serialize module {}: {}", doc.module_name, e)))?;
            fs::write(module_path, module_json).map_err(Error::Io)?;
        }
        
        Ok(())
    }

    /// Generate search index JSON
    pub fn generate_search_index(&self, search_index: &[SearchIndexEntry], output_dir: &Path) -> Result<(), Error> {
        let search_path = output_dir.join("search_index.json");
        
        let search_data = serde_json::json!({
            "metadata": {
                "generated_at": chrono::Utc::now().to_rfc3339(),
                "total_entries": search_index.len(),
                "version": "1.0.0"
            },
            "index": search_index
        });
        
        let json_content = serde_json::to_string_pretty(&search_data)
            .map_err(|e| Error::General(format!("Failed to serialize search index: {}", e)))?;
        
        fs::write(search_path, json_content).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate API schema for tooling integration
    pub fn generate_api_schema(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> Result<(), Error> {
        let schema_path = output_dir.join("api_schema.json");
        
        // Extract API-like items (functions, interfaces)
        let mut api_endpoints = Vec::new();
        let mut type_definitions = Vec::new();
        
        for doc in docs {
            for item in &doc.items {
                match item.kind {
                    crate::docs::generator::ItemKind::Function => {
                        let endpoint = serde_json::json!({
                            "name": item.name,
                            "module": item.module,
                            "signature": item.signature,
                            "parameters": item.parameters,
                            "return_type": item.return_type,
                            "description": item.description,
                            "examples": item.examples
                        });
                        api_endpoints.push(endpoint);
                    }
                    crate::docs::generator::ItemKind::Struct | 
                    crate::docs::generator::ItemKind::Interface => {
                        let type_def = serde_json::json!({
                            "name": item.name,
                            "kind": item.kind,
                            "module": item.module,
                            "signature": item.signature,
                            "fields": item.parameters,
                            "description": item.description
                        });
                        type_definitions.push(type_def);
                    }
                    _ => {}
                }
            }
        }
        
        let schema = serde_json::json!({
            "api_version": "1.0.0",
            "generated_at": chrono::Utc::now().to_rfc3339(),
            "language": "cursed",
            "endpoints": api_endpoints,
            "types": type_definitions,
            "metadata": {
                "title": self.config.title,
                "description": self.config.description,
                "version": self.config.version
            }
        });
        
        let schema_content = serde_json::to_string_pretty(&schema)
            .map_err(|e| Error::General(format!("Failed to serialize API schema: {}", e)))?;
        
        fs::write(schema_path, schema_content).map_err(Error::Io)?;
        Ok(())
    }

    /// Generate metrics and analytics data
    pub fn generate_metrics(&self, docs: &[ExtractedDocumentation], output_dir: &Path) -> Result<(), Error> {
        let metrics_path = output_dir.join("metrics.json");
        
        let mut module_metrics = Vec::new();
        let mut complexity_scores = Vec::new();
        
        for doc in docs {
            let functions_count = doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Function)).count();
            let structs_count = doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Struct)).count();
            let interfaces_count = doc.items.iter().filter(|i| matches!(i.kind, crate::docs::generator::ItemKind::Interface)).count();
            
            // Calculate complexity score (simple heuristic)
            let complexity = self.calculate_module_complexity(doc);
            
            let module_metric = serde_json::json!({
                "module": doc.module_name,
                "package": doc.package_name,
                "file_path": doc.file_path,
                "metrics": {
                    "total_items": doc.items.len(),
                    "functions": functions_count,
                    "structs": structs_count,
                    "interfaces": interfaces_count,
                    "lines_of_code": doc.source_info.line_count,
                    "file_size_bytes": doc.source_info.file_size,
                    "complexity_score": complexity,
                    "documentation_coverage": self.calculate_documentation_coverage(doc)
                }
            });
            
            module_metrics.push(module_metric);
            complexity_scores.push(complexity);
        }
        
        let total_complexity: f64 = complexity_scores.iter().sum();
        let avg_complexity = if !complexity_scores.is_empty() {
            total_complexity / complexity_scores.len() as f64
        } else {
            0.0
        };
        
        let metrics = serde_json::json!({
            "generated_at": chrono::Utc::now().to_rfc3339(),
            "summary": {
                "total_modules": docs.len(),
                "total_lines": docs.iter().map(|d| d.source_info.line_count).sum::<usize>(),
                "total_size_bytes": docs.iter().map(|d| d.source_info.file_size).sum::<u64>(),
                "average_complexity": avg_complexity,
                "max_complexity": complexity_scores.iter().cloned().fold(0.0, f64::max),
                "min_complexity": complexity_scores.iter().cloned().fold(f64::INFINITY, f64::min)
            },
            "modules": module_metrics,
            "recommendations": self.generate_recommendations(docs)
        });
        
        let metrics_content = serde_json::to_string_pretty(&metrics)
            .map_err(|e| Error::General(format!("Failed to serialize metrics: {}", e)))?;
        
        fs::write(metrics_path, metrics_content).map_err(Error::Io)?;
        Ok(())
    }

    /// Calculate module complexity score
    fn calculate_module_complexity(&self, doc: &ExtractedDocumentation) -> f64 {
        let mut score = 0.0;
        
        // Base complexity from number of items
        score += doc.items.len() as f64 * 0.5;
        
        // Complexity from function parameters
        for item in &doc.items {
            if matches!(item.kind, crate::docs::generator::ItemKind::Function) {
                score += item.parameters.len() as f64 * 0.3;
                
                // Add complexity for return types
                if item.return_type.is_some() {
                    score += 0.2;
                }
                
                // Add complexity for examples (indicates complex usage)
                score += item.examples.len() as f64 * 0.1;
            }
        }
        
        // Normalize by file size
        if doc.source_info.line_count > 0 {
            score = score / (doc.source_info.line_count as f64 / 100.0).max(1.0);
        }
        
        score
    }

    /// Calculate documentation coverage percentage
    fn calculate_documentation_coverage(&self, doc: &ExtractedDocumentation) -> f64 {
        if doc.items.is_empty() {
            return 0.0;
        }
        
        let documented_items = doc.items.iter()
            .filter(|item| !item.summary.is_empty() || !item.description.is_empty())
            .count();
        
        (documented_items as f64 / doc.items.len() as f64) * 100.0
    }

    /// Generate recommendations based on analysis
    fn generate_recommendations(&self, docs: &[ExtractedDocumentation]) -> Vec<serde_json::Value> {
        let mut recommendations = Vec::new();
        
        for doc in docs {
            let coverage = self.calculate_documentation_coverage(doc);
            
            if coverage < 50.0 {
                recommendations.push(serde_json::json!({
                    "type": "documentation",
                    "severity": "warning",
                    "module": doc.module_name,
                    "message": format!("Low documentation coverage: {:.1}%", coverage),
                    "suggestion": "Add documentation comments to functions and types"
                }));
            }
            
            let complexity = self.calculate_module_complexity(doc);
            if complexity > 20.0 {
                recommendations.push(serde_json::json!({
                    "type": "complexity",
                    "severity": "info",
                    "module": doc.module_name,
                    "message": format!("High complexity score: {:.1}", complexity),
                    "suggestion": "Consider breaking down large functions or reducing parameter counts"
                }));
            }
            
            if doc.items.len() > 50 {
                recommendations.push(serde_json::json!({
                    "type": "organization",
                    "severity": "info",
                    "module": doc.module_name,
                    "message": format!("Large module with {} items", doc.items.len()),
                    "suggestion": "Consider splitting into smaller, more focused modules"
                }));
            }
        }
        
        recommendations
    }

    /// Sanitize module name for filename
    fn sanitize_module_name(&self, name: &str) -> String {
        name.replace("::", "_")
            .replace(" ", "_")
            .replace("/", "_")
            .to_lowercase()
    }
}
