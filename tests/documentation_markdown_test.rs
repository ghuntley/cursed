//! Markdown Documentation Output Format Tests
//! 
//! Tests the Markdown output generation for documentation, including structure,
//! formatting, cross-references, and GitHub-compatible features. This is critical
//! for ensuring generated Markdown docs are well-formatted and readable.

use cursed::documentation::generator::{DocumentationGenerator, OutputFormat};
use cursed::documentation::{DocumentationConfig, load_config, save_config, create_default_config};
use cursed::documentation::generator::{FunctionDoc, TypeDoc, ModuleDoc, ExampleDoc};
use cursed::error::SourceLocation;
use std::path::PathBuf;
use std::collections::HashMap;
use tempfile::TempDir;
use tracing::{debug, info};

#[path = "common.rs"]
mod common;

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_markdown_config() -> DocumentationConfig {
        DocumentationConfig {
            output_formats: vec![OutputFormat::Markdown],
            output_dir: PathBuf::from("docs"),
            ..Default::default()
        }
    }

    fn create_sample_function_doc() -> FunctionDoc {
        FunctionDoc {
            name: "calculate_sum".to_string(),
            description: Some("Calculate the sum of two numbers\n\nThis function adds two integers and returns the result.".to_string()),
            parameters: vec![
                cursed::documentation::generator::ParameterDoc {
                    name: "a".to_string(),
                    param_type: "i32".to_string(),
                    description: Some("First number to add".to_string()),
                },
                cursed::documentation::generator::ParameterDoc {
                    name: "b".to_string(),
                    param_type: "i32".to_string(),
                    description: Some("Second number to add".to_string()),
                },
            ],
            return_type: Some("i32".to_string()),
            return_description: Some("The sum of a and b".to_string()),
            examples: vec![
                ExampleDoc {
                    title: Some("Basic addition".to_string()),
                    code: "let result = calculate_sum(5, 3);\nassert_eq!(result, 8);".to_string(),
                    description: Some("Add two positive numbers".to_string()),
                },
            ],
            location: SourceLocation {
                file: PathBuf::from("math.csd"),
                line: 10,
                column: 0,
            },
            visibility: "public".to_string(),
            source_code: Some("slay calculate_sum(a: i32, b: i32) -> i32 {\n    return a + b;\n}".to_string()),
            tags: HashMap::new(),
        }
    }

    #[test]
    fn test_markdown_generator_initialization() {
        init_tracing!();
        info!("Testing Markdown generator initialization");
        
        let config = create_markdown_config();
        let generator = DocumentationGenerator::new(config).unwrap();
        assert_eq!(generator.config().output_formats.len(), 1);
        assert!(matches!(generator.config().output_formats[0], OutputFormat::Markdown));
        
        debug!("Markdown generator initialized successfully");
    }

    #[test]
    fn test_function_markdown_generation() {
        init_tracing!();
        info!("Testing function Markdown generation");
        
        let generator = DocumentationGenerator::new(create_markdown_config()).unwrap();
        let func_doc = create_sample_function_doc();
        
        let markdown_result = tokio_test::block_on(
            generator.generate_function_markdown(&func_doc)
        );
        assert!(markdown_result.is_ok());
        
        let markdown = markdown_result.unwrap();
        
        // Check function header
        assert!(markdown.contains("## calculate_sum"));
        assert!(markdown.contains("Calculate the sum of two numbers"));
        
        // Check parameters section
        assert!(markdown.contains("### Parameters"));
        assert!(markdown.contains("- `a: i32` - First number to add"));
        assert!(markdown.contains("- `b: i32` - Second number to add"));
        
        // Check return section
        assert!(markdown.contains("### Returns"));
        assert!(markdown.contains("`i32` - The sum of a and b"));
        
        // Check examples section
        assert!(markdown.contains("### Examples"));
        assert!(markdown.contains("```cursed"));
        assert!(markdown.contains("let result = calculate_sum(5, 3);"));
        assert!(markdown.contains("assert_eq!(result, 8);"));
        assert!(markdown.contains("```"));
        
        // Check source code section
        assert!(markdown.contains("### Source"));
        assert!(markdown.contains("slay calculate_sum"));
        
        debug!("Function Markdown generated successfully: {} characters", markdown.len());
    }

    #[test]
    fn test_type_markdown_generation() {
        init_tracing!();
        info!("Testing type Markdown generation");
        
        let generator = DocumentationGenerator::new(create_markdown_config()).unwrap();
        let type_doc = TypeDoc {
            name: "Rectangle".to_string(),
            description: Some("Represents a rectangle with width and height".to_string()),
            type_kind: "struct".to_string(),
            fields: vec![
                cursed::documentation::generator::FieldDoc {
                    name: "width".to_string(),
                    field_type: "f64".to_string(),
                    description: Some("Width of the rectangle".to_string()),
                    visibility: "public".to_string(),
                },
                cursed::documentation::generator::FieldDoc {
                    name: "height".to_string(),
                    field_type: "f64".to_string(),
                    description: Some("Height of the rectangle".to_string()),
                    visibility: "public".to_string(),
                },
            ],
            methods: vec![
                cursed::documentation::generator::MethodDoc {
                    name: "area".to_string(),
                    description: Some("Calculate the area of the rectangle".to_string()),
                    parameters: vec![],
                    return_type: Some("f64".to_string()),
                    return_description: Some("The area".to_string()),
                    visibility: "public".to_string(),
                },
            ],
            type_parameters: vec![],
            location: SourceLocation {
                file: PathBuf::from("shapes.csd"),
                line: 5,
                column: 0,
            },
            visibility: "public".to_string(),
            source_code: Some("squad Rectangle {\n    width: f64,\n    height: f64,\n}".to_string()),
            tags: HashMap::new(),
        };
        
        let markdown_result = tokio_test::block_on(
            generator.generate_type_markdown(&type_doc)
        );
        assert!(markdown_result.is_ok());
        
        let markdown = markdown_result.unwrap();
        
        // Check type header
        assert!(markdown.contains("## Rectangle"));
        assert!(markdown.contains("Represents a rectangle"));
        
        // Check fields section
        assert!(markdown.contains("### Fields"));
        assert!(markdown.contains("- `width: f64` - Width of the rectangle"));
        assert!(markdown.contains("- `height: f64` - Height of the rectangle"));
        
        // Check methods section
        assert!(markdown.contains("### Methods"));
        assert!(markdown.contains("#### area"));
        assert!(markdown.contains("Calculate the area"));
        
        debug!("Type Markdown generated successfully: {} characters", markdown.len());
    }

    #[test]
    fn test_module_markdown_generation() {
        init_tracing!();
        info!("Testing module Markdown generation");
        
        let generator = DocumentationGenerator::new(create_markdown_config()).unwrap();
        let module_doc = ModuleDoc {
            name: "math_utils".to_string(),
            description: Some("Mathematical utility functions and types".to_string()),
            path: PathBuf::from("src/math_utils.csd"),
            functions: vec![create_sample_function_doc()],
            types: vec![],
            constants: vec![],
            variables: vec![],
            submodules: vec![],
            location: SourceLocation {
                file: PathBuf::from("math_utils.csd"),
                line: 1,
                column: 0,
            },
            visibility: "public".to_string(),
            tags: HashMap::new(),
        };
        
        let markdown_result = tokio_test::block_on(
            generator.generate_module_markdown(&module_doc)
        );
        assert!(markdown_result.is_ok());
        
        let markdown = markdown_result.unwrap();
        
        // Check module header
        assert!(markdown.contains("# math_utils"));
        assert!(markdown.contains("Mathematical utility functions"));
        
        // Check table of contents
        assert!(markdown.contains("## Table of Contents"));
        assert!(markdown.contains("- [Functions](#functions)"));
        
        // Check functions section
        assert!(markdown.contains("## Functions"));
        assert!(markdown.contains("calculate_sum"));
        
        debug!("Module Markdown generated successfully: {} characters", markdown.len());
    }

    #[test]
    fn test_readme_generation() {
        init_tracing!();
        info!("Testing README generation");
        
        let generator = DocumentationGenerator::new(create_markdown_config()).unwrap();
        let modules = vec![
            ModuleDoc {
                name: "math_utils".to_string(),
                description: Some("Mathematical utilities".to_string()),
                path: PathBuf::from("math_utils.csd"),
                functions: vec![create_sample_function_doc()],
                types: vec![],
                constants: vec![],
                variables: vec![],
                submodules: vec![],
                location: SourceLocation {
                    file: PathBuf::from("math_utils.csd"),
                    line: 1,
                    column: 0,
                },
                visibility: "public".to_string(),
                tags: HashMap::new(),
            },
        ];
        
        let readme_result = tokio_test::block_on(
            generator.generate_readme(&modules)
        );
        assert!(readme_result.is_ok());
        
        let readme = readme_result.unwrap();
        
        // Check README structure
        assert!(readme.contains("# CURSED Project Documentation"));
        assert!(readme.contains("## Overview"));
        assert!(readme.contains("## Modules"));
        assert!(readme.contains("## Getting Started"));
        
        // Check module listing
        assert!(readme.contains("- [math_utils](math_utils.md)"));
        assert!(readme.contains("Mathematical utilities"));
        
        // Check code examples
        assert!(readme.contains("```cursed"));
        
        debug!("README generated successfully: {} characters", readme.len());
    }

    #[test]
    fn test_cross_reference_links() {
        init_tracing!();
        info!("Testing cross-reference links in Markdown");
        
        let generator = DocumentationGenerator::new(create_markdown_config()).unwrap();
        let cross_refs = HashMap::from([
            (
                "math_utils.csd".to_string(),
                vec![
                    cursed::documentation::CrossReference {
                        target: "Rectangle".to_string(),
                        reference_type: "type_usage".to_string(),
                        location: SourceLocation {
                            file: PathBuf::from("math_utils.csd"),
                            line: 15,
                            column: 20,
                        },
                        context: Some("Function return type".to_string()),
                    },
                ],
            ),
        ]);
        
        let markdown_result = tokio_test::block_on(
            generator.generate_cross_reference_markdown(&cross_refs)
        );
        assert!(markdown_result.is_ok());
        
        let markdown = markdown_result.unwrap();
        
        // Check cross-reference structure
        assert!(markdown.contains("## Cross References"));
        assert!(markdown.contains("[Rectangle]"));
        assert!(markdown.contains("type_usage"));
        assert!(markdown.contains("Function return type"));
        
        debug!("Cross-reference Markdown generated successfully");
    }

    #[test]
    fn test_configuration_parsing() {
        init_tracing!();
        info!("Testing configuration parsing");
        
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("cursed-doc.toml");
        
        // Create a default config file
        let default_config_result = create_default_config(&config_path);
        assert!(default_config_result.is_ok());
        
        // Load the config file
        let loaded_config_result = load_config(&config_path);
        assert!(loaded_config_result.is_ok());
        
        let loaded_config = loaded_config_result.unwrap();
        assert_eq!(loaded_config.project.name, "CURSED Project");
        assert_eq!(loaded_config.project.version, "0.1.0");
        assert!(loaded_config.source_dirs.contains(&PathBuf::from("src")));
        assert!(loaded_config.source_dirs.contains(&PathBuf::from("lib")));
        
        debug!("Configuration parsed successfully");
    }

    #[test]
    fn test_configuration_validation() {
        init_tracing!();
        info!("Testing configuration validation");
        
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid-config.toml");
        
        // Create an invalid configuration
        let invalid_toml = r#"
[project]
name = "Test Project"
# Missing required version field

[options]
include_private = "not a boolean"  # Invalid type
"#;
        
        std::fs::write(&config_path, invalid_toml).unwrap();
        
        // Try to load invalid config
        let result = load_config(&config_path);
        assert!(result.is_err(), "Invalid config should fail to load");
        
        debug!("Configuration validation working correctly");
    }

    #[test]
    fn test_json_configuration_support() {
        init_tracing!();
        info!("Testing JSON configuration support");
        
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("cursed-doc.json");
        
        let json_config = r#"{
    "source_dirs": ["src", "examples"],
    "output_dir": "documentation",
    "output_formats": ["Markdown", "Html"],
    "project": {
        "name": "JSON Test Project",
        "version": "1.0.0",
        "description": "Test project with JSON config",
        "authors": ["Test Author"],
        "homepage": "https://example.com",
        "repository": "https://github.com/test/project",
        "license": "MIT"
    },
    "options": {
        "include_private": false,
        "include_source": true,
        "generate_cross_refs": true,
        "generate_search_index": true,
        "include_examples": true,
        "max_type_depth": 10,
        "include_dependencies": false
    },
    "styling": {
        "custom_css": [],
        "template_dir": null,
        "theme": "dark",
        "colors": null,
        "favicon": null,
        "logo": null
    }
}"#;
        
        std::fs::write(&config_path, json_config).unwrap();
        
        let loaded_config_result = load_config(&config_path);
        assert!(loaded_config_result.is_ok());
        
        let loaded_config = loaded_config_result.unwrap();
        assert_eq!(loaded_config.project.name, "JSON Test Project");
        assert_eq!(loaded_config.project.version, "1.0.0");
        assert_eq!(loaded_config.styling.theme, "dark");
        assert!(loaded_config.source_dirs.contains(&PathBuf::from("examples")));
        
        debug!("JSON configuration loaded successfully");
    }

    #[test]
    fn test_configuration_save_and_load_roundtrip() {
        init_tracing!();
        info!("Testing configuration save and load roundtrip");
        
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("roundtrip-config.toml");
        
        // Create a custom configuration
        let mut custom_config = DocumentationConfig::default();
        custom_config.project.name = "Roundtrip Test".to_string();
        custom_config.project.version = "2.0.0".to_string();
        custom_config.project.description = Some("Test roundtrip config".to_string());
        custom_config.output_dir = PathBuf::from("custom_docs");
        custom_config.options.include_private = true;
        custom_config.options.max_type_depth = 20;
        
        // Save the configuration
        let save_result = save_config(&custom_config, &config_path);
        assert!(save_result.is_ok());
        
        // Load the configuration back
        let loaded_config_result = load_config(&config_path);
        assert!(loaded_config_result.is_ok());
        
        let loaded_config = loaded_config_result.unwrap();
        assert_eq!(loaded_config.project.name, "Roundtrip Test");
        assert_eq!(loaded_config.project.version, "2.0.0");
        assert_eq!(loaded_config.project.description, Some("Test roundtrip config".to_string()));
        assert_eq!(loaded_config.output_dir, PathBuf::from("custom_docs"));
        assert_eq!(loaded_config.options.include_private, true);
        assert_eq!(loaded_config.options.max_type_depth, 20);
        
        debug!("Configuration roundtrip successful");
    }

    #[test]
    fn test_markdown_code_block_formatting() {
        init_tracing!();
        info!("Testing Markdown code block formatting");
        
        let generator = DocumentationGenerator::new(create_markdown_config()).unwrap();
        
        let cursed_code = r#"/// Example function with complex syntax
slay complex_function<T: Clone>(items: Vec<T>, predicate: fn(&T) -> bool) -> Vec<T> {
    sus mut results = Vec::new();
    
    bestie item in items {
        lowkey (predicate(&item)) {
            results.push(item.clone());
        }
    }
    
    return results;
}"#;
        
        let formatted_result = tokio_test::block_on(
            generator.format_code_block(cursed_code, "cursed")
        );
        assert!(formatted_result.is_ok());
        
        let formatted = formatted_result.unwrap();
        
        // Check code block structure
        assert!(formatted.starts_with("```cursed"));
        assert!(formatted.ends_with("```"));
        assert!(formatted.contains("slay complex_function"));
        assert!(formatted.contains("lowkey (predicate(&item))"));
        assert!(formatted.contains("bestie item in items"));
        
        debug!("Markdown code block formatted successfully");
    }

    #[test]
    fn test_table_generation() {
        init_tracing!();
        info!("Testing table generation for parameters and fields");
        
        let generator = DocumentationGenerator::new(create_markdown_config()).unwrap();
        let parameters = vec![
            ("name", "String", "The name of the item"),
            ("count", "i32", "Number of items"),
            ("active", "bool", "Whether the item is active"),
        ];
        
        let table_result = tokio_test::block_on(
            generator.generate_parameter_table(&parameters)
        );
        assert!(table_result.is_ok());
        
        let table = table_result.unwrap();
        
        // Check table structure
        assert!(table.contains("| Parameter | Type | Description |"));
        assert!(table.contains("|-----------|------|-------------|"));
        assert!(table.contains("| `name` | `String` | The name of the item |"));
        assert!(table.contains("| `count` | `i32` | Number of items |"));
        assert!(table.contains("| `active` | `bool` | Whether the item is active |"));
        
        debug!("Parameter table generated successfully");
    }

    #[test]
    fn test_github_compatible_features() {
        init_tracing!();
        info!("Testing GitHub-compatible Markdown features");
        
        let generator = DocumentationGenerator::new(create_markdown_config()).unwrap();
        let func_doc = create_sample_function_doc();
        
        let markdown_result = tokio_test::block_on(
            generator.generate_github_compatible_markdown(&func_doc)
        );
        assert!(markdown_result.is_ok());
        
        let markdown = markdown_result.unwrap();
        
        // Check GitHub-specific features
        assert!(markdown.contains("<!-- TOC -->"));
        assert!(markdown.contains("<!-- /TOC -->"));
        
        // Check collapsible sections
        assert!(markdown.contains("<details>"));
        assert!(markdown.contains("<summary>"));
        assert!(markdown.contains("</details>"));
        
        // Check anchor links
        assert!(markdown.contains("#calculate_sum"));
        
        // Check badges or shields (if applicable)
        // assert!(markdown.contains("![](https://img.shields.io/"));
        
        debug!("GitHub-compatible Markdown generated successfully");
    }

    #[test]
    fn test_markdown_link_validation() {
        init_tracing!();
        info!("Testing Markdown link validation");
        
        let generator = DocumentationGenerator::new(create_markdown_config()).unwrap();
        let modules = vec![
            ModuleDoc {
                name: "module_a".to_string(),
                description: Some("Module A".to_string()),
                path: PathBuf::from("module_a.csd"),
                functions: vec![],
                types: vec![],
                constants: vec![],
                variables: vec![],
                submodules: vec![],
                location: SourceLocation {
                    file: PathBuf::from("module_a.csd"),
                    line: 1,
                    column: 0,
                },
                visibility: "public".to_string(),
                tags: HashMap::new(),
            },
        ];
        
        let links_result = tokio_test::block_on(
            generator.validate_markdown_links(&modules)
        );
        assert!(links_result.is_ok());
        
        let validation_result = links_result.unwrap();
        assert!(validation_result.is_valid);
        
        debug!("Markdown link validation completed successfully");
    }
}
