//! JSON Documentation Generator
//! 
//! Generates machine-readable JSON documentation for API integration,
//! tooling, and automated processing.

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use serde_json;
use crate::error::CursedError;
use crate::documentation::{DocConfig, Documentation};

/// JSON documentation generator
pub struct JsonGenerator<'a> {
    config: &'a DocConfig,
}

impl<'a> JsonGenerator<'a> {
    /// Create new JSON generator
    pub fn new(config: &'a DocConfig) -> Self {
        Self { config }
    }

    /// Generate JSON documentation
    pub fn generate(&self, documentation: &Documentation) -> Result<(), CursedError> {
        let output_dir = Path::new(&self.config.output.output_dir);
        let json_dir = output_dir.join("json");
        
        fs::create_dir_all(&json_dir)
            .map_err(|e| CursedError::IoError(format!("Failed to create JSON directory: {}", e)))?;

        // Generate complete API documentation
        self.generate_complete_api(&json_dir, documentation)?;

        // Generate module-specific JSON files
        for module in &documentation.modules {
            self.generate_module_json(&json_dir, module)?;
        }

        // Generate search index
        self.generate_search_index(&json_dir, documentation)?;

        // Generate schema definition
        self.generate_schema(&json_dir)?;

        // Generate OpenAPI specification
        self.generate_openapi_spec(&json_dir, documentation)?;

        Ok(())
    }

    /// Generate complete API documentation
    fn generate_complete_api(&self, json_dir: &Path, documentation: &Documentation) -> Result<(), CursedError> {
        let api_file = json_dir.join("api.json");
        let mut file = File::create(&api_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create api.json: {}", e)))?;

        let json_content = serde_json::to_string_pretty(documentation)
            .map_err(|e| CursedError::SerializationError(format!("Failed to serialize documentation: {}", e)))?;

        file.write_all(json_content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write api.json: {}", e)))?;

        Ok(())
    }

    /// Generate module-specific JSON
    fn generate_module_json(&self, json_dir: &Path, module: &crate::documentation::DocumentedModule) -> Result<(), CursedError> {
        let modules_dir = json_dir.join("modules");
        fs::create_dir_all(&modules_dir)
            .map_err(|e| CursedError::IoError(format!("Failed to create modules directory: {}", e)))?;

        let module_file = modules_dir.join(format!("{}.json", module.name));
        let mut file = File::create(&module_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create module JSON: {}", e)))?;

        let json_content = serde_json::to_string_pretty(module)
            .map_err(|e| CursedError::SerializationError(format!("Failed to serialize module: {}", e)))?;

        file.write_all(json_content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write module JSON: {}", e)))?;

        Ok(())
    }

    /// Generate search index
    fn generate_search_index(&self, json_dir: &Path, documentation: &Documentation) -> Result<(), CursedError> {
        let search_file = json_dir.join("search.json");
        let mut file = File::create(&search_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create search.json: {}", e)))?;

        let search_data = self.build_search_data(documentation)?;
        let json_content = serde_json::to_string_pretty(&search_data)
            .map_err(|e| CursedError::SerializationError(format!("Failed to serialize search data: {}", e)))?;

        file.write_all(json_content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write search.json: {}", e)))?;

        Ok(())
    }

    /// Build search data structure
    fn build_search_data(&self, documentation: &Documentation) -> Result<serde_json::Value, CursedError> {
        let mut search_items = Vec::new();

        // Add modules to search
        for module in &documentation.modules {
            search_items.push(serde_json::json!({
                "id": format!("module-{}", module.name),
                "title": format!("{} Module", module.name),
                "type": "module",
                "content": module.description,
                "url": format!("modules/{}.html", module.name),
                "tags": ["module"],
                "source": module.source_file,
                "functions": module.functions.len(),
                "variables": module.variables.len()
            }));

            // Add functions to search
            for function in &module.functions {
                search_items.push(serde_json::json!({
                    "id": format!("function-{}-{}", module.name, function.name),
                    "title": function.name,
                    "type": "function",
                    "content": function.description,
                    "url": format!("modules/{}.html#{}", module.name, function.name),
                    "tags": ["function", &module.name],
                    "signature": function.signature,
                    "parameters": function.parameters.len(),
                    "return_type": function.return_type,
                    "source": function.source_file,
                    "line": function.source_line
                }));
            }

            // Add variables to search
            for variable in &module.variables {
                search_items.push(serde_json::json!({
                    "id": format!("variable-{}-{}", module.name, variable.name),
                    "title": variable.name,
                    "type": "variable",
                    "content": variable.description,
                    "url": format!("modules/{}.html#{}", module.name, variable.name),
                    "tags": ["variable", &module.name],
                    "var_type": variable.var_type,
                    "source": variable.source_file,
                    "line": variable.source_line
                }));
            }

            // Add constants to search
            for constant in &module.constants {
                search_items.push(serde_json::json!({
                    "id": format!("constant-{}-{}", module.name, constant.name),
                    "title": constant.name,
                    "type": "constant",
                    "content": constant.description,
                    "url": format!("modules/{}.html#{}", module.name, constant.name),
                    "tags": ["constant", &module.name],
                    "const_type": constant.const_type,
                    "value": constant.value,
                    "source": constant.source_file,
                    "line": constant.source_line
                }));
            }
        }

        // Add examples to search
        for example in &documentation.examples {
            search_items.push(serde_json::json!({
                "id": format!("example-{}", example.title.replace(" ", "_").to_lowercase()),
                "title": example.title,
                "type": "example",
                "content": example.description,
                "url": format!("examples/{}.html", example.title.replace(" ", "_").to_lowercase()),
                "tags": ["example", &example.category],
                "category": example.category,
                "source": example.source_file
            }));
        }

        Ok(serde_json::json!({
            "items": search_items,
            "meta": {
                "total_items": search_items.len(),
                "generated_at": chrono::Utc::now().to_rfc3339(),
                "version": "1.0.0"
            }
        }))
    }

    /// Generate JSON schema definition
    fn generate_schema(&self, json_dir: &Path) -> Result<(), CursedError> {
        let schema_file = json_dir.join("schema.json");
        let mut file = File::create(&schema_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create schema.json: {}", e)))?;

        let schema = self.build_json_schema()?;
        let json_content = serde_json::to_string_pretty(&schema)
            .map_err(|e| CursedError::SerializationError(format!("Failed to serialize schema: {}", e)))?;

        file.write_all(json_content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write schema.json: {}", e)))?;

        Ok(())
    }

    /// Build JSON schema
    fn build_json_schema(&self) -> Result<serde_json::Value, CursedError> {
        Ok(serde_json::json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": "CURSED Documentation Schema",
            "description": "JSON schema for CURSED documentation format",
            "type": "object",
            "properties": {
                "project_info": {
                    "type": "object",
                    "properties": {
                        "project_name": {"type": "string"},
                        "project_version": {"type": "string"},
                        "project_description": {"type": "string"},
                        "project_url": {"type": "string"},
                        "authors": {"type": "array", "items": {"type": "string"}},
                        "license": {"type": "string"},
                        "repository": {"type": "string"}
                    },
                    "required": ["project_name", "project_version"]
                },
                "modules": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"},
                            "description": {"type": "string"},
                            "source_file": {"type": "string"},
                            "functions": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string"},
                                        "signature": {"type": "string"},
                                        "description": {"type": "string"},
                                        "parameters": {
                                            "type": "array",
                                            "items": {
                                                "type": "object",
                                                "properties": {
                                                    "name": {"type": "string"},
                                                    "param_type": {"type": "string"},
                                                    "description": {"type": "string"}
                                                }
                                            }
                                        },
                                        "return_type": {"type": "string"},
                                        "return_description": {"type": "string"},
                                        "examples": {"type": "array", "items": {"type": "string"}},
                                        "source_file": {"type": "string"},
                                        "source_line": {"type": "number"}
                                    },
                                    "required": ["name", "signature"]
                                }
                            },
                            "variables": {"type": "array"},
                            "constants": {"type": "array"},
                            "types": {"type": "array"}
                        },
                        "required": ["name", "source_file"]
                    }
                },
                "examples": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "title": {"type": "string"},
                            "description": {"type": "string"},
                            "code": {"type": "string"},
                            "output": {"type": "string"},
                            "category": {"type": "string"},
                            "source_file": {"type": "string"}
                        },
                        "required": ["title", "code", "category"]
                    }
                },
                "coverage_stats": {
                    "type": "object",
                    "properties": {
                        "total_functions": {"type": "number"},
                        "documented_functions": {"type": "number"},
                        "coverage_percentage": {"type": "number"},
                        "missing_docs": {"type": "array", "items": {"type": "string"}}
                    }
                }
            },
            "required": ["project_info", "modules"]
        }))
    }

    /// Generate OpenAPI specification
    fn generate_openapi_spec(&self, json_dir: &Path, documentation: &Documentation) -> Result<(), CursedError> {
        let openapi_file = json_dir.join("openapi.json");
        let mut file = File::create(&openapi_file)
            .map_err(|e| CursedError::IoError(format!("Failed to create openapi.json: {}", e)))?;

        let openapi_spec = self.build_openapi_spec(documentation)?;
        let json_content = serde_json::to_string_pretty(&openapi_spec)
            .map_err(|e| CursedError::SerializationError(format!("Failed to serialize OpenAPI spec: {}", e)))?;

        file.write_all(json_content.as_bytes())
            .map_err(|e| CursedError::IoError(format!("Failed to write openapi.json: {}", e)))?;

        Ok(())
    }

    /// Build OpenAPI specification
    fn build_openapi_spec(&self, documentation: &Documentation) -> Result<serde_json::Value, CursedError> {
        let mut paths = serde_json::Map::new();
        let mut components = serde_json::Map::new();
        let mut schemas = serde_json::Map::new();

        // Add documentation API endpoints
        paths.insert("/api/documentation".to_string(), serde_json::json!({
            "get": {
                "summary": "Get complete documentation",
                "description": "Returns the complete API documentation",
                "responses": {
                    "200": {
                        "description": "Complete documentation object",
                        "content": {
                            "application/json": {
                                "schema": {"$ref": "#/components/schemas/Documentation"}
                            }
                        }
                    }
                }
            }
        }));

        paths.insert("/api/modules".to_string(), serde_json::json!({
            "get": {
                "summary": "Get all modules",
                "description": "Returns information about all modules",
                "responses": {
                    "200": {
                        "description": "List of modules",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "array",
                                    "items": {"$ref": "#/components/schemas/Module"}
                                }
                            }
                        }
                    }
                }
            }
        }));

        paths.insert("/api/modules/{moduleName}".to_string(), serde_json::json!({
            "get": {
                "summary": "Get module by name",
                "description": "Returns detailed information about a specific module",
                "parameters": [{
                    "name": "moduleName",
                    "in": "path",
                    "required": true,
                    "schema": {"type": "string"}
                }],
                "responses": {
                    "200": {
                        "description": "Module information",
                        "content": {
                            "application/json": {
                                "schema": {"$ref": "#/components/schemas/Module"}
                            }
                        }
                    },
                    "404": {
                        "description": "Module not found"
                    }
                }
            }
        }));

        paths.insert("/api/search".to_string(), serde_json::json!({
            "get": {
                "summary": "Search documentation",
                "description": "Search through documentation content",
                "parameters": [{
                    "name": "q",
                    "in": "query",
                    "required": true,
                    "schema": {"type": "string"},
                    "description": "Search query"
                }],
                "responses": {
                    "200": {
                        "description": "Search results",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "array",
                                    "items": {"$ref": "#/components/schemas/SearchResult"}
                                }
                            }
                        }
                    }
                }
            }
        }));

        // Define schemas
        schemas.insert("Documentation".to_string(), serde_json::json!({
            "type": "object",
            "properties": {
                "project_info": {"$ref": "#/components/schemas/ProjectInfo"},
                "modules": {
                    "type": "array",
                    "items": {"$ref": "#/components/schemas/Module"}
                },
                "examples": {
                    "type": "array",
                    "items": {"$ref": "#/components/schemas/Example"}
                },
                "coverage_stats": {"$ref": "#/components/schemas/CoverageStats"}
            }
        }));

        schemas.insert("ProjectInfo".to_string(), serde_json::json!({
            "type": "object",
            "properties": {
                "project_name": {"type": "string"},
                "project_version": {"type": "string"},
                "project_description": {"type": "string"},
                "project_url": {"type": "string"},
                "authors": {"type": "array", "items": {"type": "string"}},
                "license": {"type": "string"},
                "repository": {"type": "string"}
            }
        }));

        schemas.insert("Module".to_string(), serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "description": {"type": "string"},
                "source_file": {"type": "string"},
                "functions": {
                    "type": "array",
                    "items": {"$ref": "#/components/schemas/Function"}
                },
                "variables": {
                    "type": "array",
                    "items": {"$ref": "#/components/schemas/Variable"}
                },
                "constants": {
                    "type": "array",
                    "items": {"$ref": "#/components/schemas/Constant"}
                }
            }
        }));

        schemas.insert("Function".to_string(), serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "signature": {"type": "string"},
                "description": {"type": "string"},
                "parameters": {
                    "type": "array",
                    "items": {"$ref": "#/components/schemas/Parameter"}
                },
                "return_type": {"type": "string"},
                "return_description": {"type": "string"},
                "examples": {"type": "array", "items": {"type": "string"}},
                "source_file": {"type": "string"},
                "source_line": {"type": "number"}
            }
        }));

        schemas.insert("Parameter".to_string(), serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "param_type": {"type": "string"},
                "description": {"type": "string"},
                "default_value": {"type": "string"}
            }
        }));

        schemas.insert("Variable".to_string(), serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "var_type": {"type": "string"},
                "description": {"type": "string"},
                "source_file": {"type": "string"},
                "source_line": {"type": "number"}
            }
        }));

        schemas.insert("Constant".to_string(), serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "const_type": {"type": "string"},
                "value": {"type": "string"},
                "description": {"type": "string"},
                "source_file": {"type": "string"},
                "source_line": {"type": "number"}
            }
        }));

        schemas.insert("Example".to_string(), serde_json::json!({
            "type": "object",
            "properties": {
                "title": {"type": "string"},
                "description": {"type": "string"},
                "code": {"type": "string"},
                "output": {"type": "string"},
                "category": {"type": "string"},
                "source_file": {"type": "string"}
            }
        }));

        schemas.insert("CoverageStats".to_string(), serde_json::json!({
            "type": "object",
            "properties": {
                "total_functions": {"type": "number"},
                "documented_functions": {"type": "number"},
                "coverage_percentage": {"type": "number"},
                "missing_docs": {"type": "array", "items": {"type": "string"}}
            }
        }));

        schemas.insert("SearchResult".to_string(), serde_json::json!({
            "type": "object",
            "properties": {
                "id": {"type": "string"},
                "title": {"type": "string"},
                "type": {"type": "string"},
                "content": {"type": "string"},
                "url": {"type": "string"},
                "tags": {"type": "array", "items": {"type": "string"}}
            }
        }));

        components.insert("schemas".to_string(), serde_json::Value::Object(schemas));

        Ok(serde_json::json!({
            "openapi": "3.0.0",
            "info": {
                "title": format!("{} Documentation API", documentation.project_info.project_name),
                "version": documentation.project_info.project_version,
                "description": format!("API for {} documentation", documentation.project_info.project_name),
                "contact": {
                    "name": documentation.project_info.authors.join(", "),
                    "url": documentation.project_info.project_url
                },
                "license": {
                    "name": documentation.project_info.license,
                    "url": documentation.project_info.repository
                }
            },
            "servers": [{
                "url": "/api",
                "description": "Documentation API server"
            }],
            "paths": paths,
            "components": components
        }))
    }
}
