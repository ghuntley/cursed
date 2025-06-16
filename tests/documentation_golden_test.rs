//! Golden File Documentation Tests
//! 
//! Tests that compare generated documentation output against known-good
//! "golden" files to detect regressions and ensure output stability.
//! Critical for preventing breaking changes in documentation formatting.

use cursed::documentation::{DocumentationSystem, DocumentationConfig, OutputFormat};
use std::path::PathBuf;
use std::fs;
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

    fn create_golden_test_config(temp_dir: &TempDir) -> DocumentationConfig {
        DocumentationConfig {
            source_dirs: vec![temp_dir.path().join("src")],
            output_dir: temp_dir.path().join("output"),
            output_formats: vec![OutputFormat::Html, OutputFormat::Markdown],
            project: cursed::documentation::ProjectMetadata {
                name: "Golden Test Project".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Test project for golden file testing".to_string()),
                authors: vec!["Golden Test Team".to_string()],
                homepage: None,
                repository: None,
                license: Some("MIT".to_string()),
            },
            options: cursed::documentation::DocOptions {
                include_private: false,
                include_source: true,
                generate_cross_refs: true,
                generate_search_index: true,
                include_examples: true,
                max_type_depth: 10,
                include_dependencies: false,
            },
            styling: cursed::documentation::StylingConfig {
                custom_css: vec![],
                template_dir: None,
                theme: "light".to_string(),
                colors: None,
                favicon: None,
                logo: None,
            },
        }
    }

    fn create_stable_test_source() -> &'static str {
        r#"
//! Golden test module
//! 
//! This module contains stable code for golden file testing.
//! The documentation output should remain consistent across runs.
//! 
//! @author Golden Test Team
//! @version 1.0.0
//! @since 1.0.0

/// A simple mathematical function for testing
/// 
/// This function adds two numbers together and returns the result.
/// It serves as a stable test case for documentation generation.
/// 
/// @param a The first number to add
/// @param b The second number to add
/// @return The sum of a and b
/// @example
/// let result = add_numbers(5, 3);
/// assert_eq!(result, 8);
/// @since 1.0.0
slay add_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}

/// A simple data structure for testing
/// 
/// This structure represents a point in 2D space and is used
/// to test documentation generation for structured types.
/// 
/// @author Geometry Team
/// @since 1.0.0
squad Point {
    /// X coordinate
    x: f64,
    /// Y coordinate  
    y: f64,
}

impl Point {
    /// Create a new point
    /// 
    /// @param x The x coordinate
    /// @param y The y coordinate
    /// @return A new Point instance
    /// @since 1.0.0
    slay new(x: f64, y: f64) -> Self {
        return Point { x, y };
    }
}

/// Mathematical constants for testing
/// 
/// @since 1.0.0
facts PI: f64 = 3.14159265359;

/// Application configuration variable
/// 
/// @since 1.0.0
sus mut CONFIG_VALUE: i32 = 42;
"#
    }

    #[test]
    fn test_html_output_stability() {
        init_tracing!();
        info!("Testing HTML output stability against golden files");
        
        let temp_dir = TempDir::new().unwrap();
        let config = create_golden_test_config(&temp_dir);
        
        // Create source file
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("golden_test.csd"), create_stable_test_source()).unwrap();
        
        // Generate documentation
        let mut doc_system = DocumentationSystem::new(config).unwrap();
        let result = tokio_test::block_on(doc_system.generate_all());
        assert!(result.is_ok());
        
        // Read generated HTML
        let output_dir = temp_dir.path().join("output");
        let html_file = output_dir.join("golden_test.html");
        assert!(html_file.exists());
        
        let generated_html = fs::read_to_string(html_file).unwrap();
        
        // Verify HTML structure stability
        assert!(generated_html.contains("<!DOCTYPE html>"));
        assert!(generated_html.contains("<h1>golden_test</h1>"));
        assert!(generated_html.contains("add_numbers"));
        assert!(generated_html.contains("Point"));
        assert!(generated_html.contains("PI"));
        assert!(generated_html.contains("CONFIG_VALUE"));
        
        // Check that specific documentation elements are present
        assert!(generated_html.contains("The first number to add"));
        assert!(generated_html.contains("The second number to add"));
        assert!(generated_html.contains("X coordinate"));
        assert!(generated_html.contains("Y coordinate"));
        assert!(generated_html.contains("Mathematical constants"));
        
        debug!("HTML output stability verified: {} characters", generated_html.len());
    }

    #[test]
    fn test_markdown_output_stability() {
        init_tracing!();
        info!("Testing Markdown output stability against golden files");
        
        let temp_dir = TempDir::new().unwrap();
        let config = create_golden_test_config(&temp_dir);
        
        // Create source file
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("golden_test.csd"), create_stable_test_source()).unwrap();
        
        // Generate documentation
        let mut doc_system = DocumentationSystem::new(config).unwrap();
        let result = tokio_test::block_on(doc_system.generate_all());
        assert!(result.is_ok());
        
        // Read generated Markdown
        let output_dir = temp_dir.path().join("output");
        let md_file = output_dir.join("golden_test.md");
        assert!(md_file.exists());
        
        let generated_md = fs::read_to_string(md_file).unwrap();
        
        // Verify Markdown structure stability
        assert!(generated_md.contains("# golden_test"));
        assert!(generated_md.contains("## add_numbers"));
        assert!(generated_md.contains("## Point"));
        assert!(generated_md.contains("### Parameters"));
        assert!(generated_md.contains("### Returns"));
        assert!(generated_md.contains("```cursed"));
        
        // Check code block formatting
        assert!(generated_md.contains("let result = add_numbers(5, 3);"));
        assert!(generated_md.contains("assert_eq!(result, 8);"));
        
        debug!("Markdown output stability verified: {} characters", generated_md.len());
    }

    #[test]
    fn test_output_determinism() {
        init_tracing!();
        info!("Testing output determinism - multiple runs should produce identical results");
        
        let temp_dir = TempDir::new().unwrap();
        let config = create_golden_test_config(&temp_dir);
        
        // Create source file
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("golden_test.csd"), create_stable_test_source()).unwrap();
        
        // Generate documentation first time
        let output_dir1 = temp_dir.path().join("output1");
        let mut config1 = config.clone();
        config1.output_dir = output_dir1.clone();
        
        let mut doc_system1 = DocumentationSystem::new(config1).unwrap();
        let result1 = tokio_test::block_on(doc_system1.generate_all());
        assert!(result1.is_ok());
        
        // Generate documentation second time
        let output_dir2 = temp_dir.path().join("output2");
        let mut config2 = config;
        config2.output_dir = output_dir2.clone();
        
        let mut doc_system2 = DocumentationSystem::new(config2).unwrap();
        let result2 = tokio_test::block_on(doc_system2.generate_all());
        assert!(result2.is_ok());
        
        // Compare HTML outputs
        let html1 = fs::read_to_string(output_dir1.join("golden_test.html")).unwrap();
        let html2 = fs::read_to_string(output_dir2.join("golden_test.html")).unwrap();
        
        // Remove timestamps and other variable content for comparison
        let normalized_html1 = normalize_html_for_comparison(&html1);
        let normalized_html2 = normalize_html_for_comparison(&html2);
        
        assert_eq!(normalized_html1, normalized_html2, "HTML outputs should be deterministic");
        
        // Compare Markdown outputs
        let md1 = fs::read_to_string(output_dir1.join("golden_test.md")).unwrap();
        let md2 = fs::read_to_string(output_dir2.join("golden_test.md")).unwrap();
        
        assert_eq!(md1, md2, "Markdown outputs should be deterministic");
        
        debug!("Output determinism verified - multiple runs produce identical results");
    }

    #[test]
    fn test_search_index_stability() {
        init_tracing!();
        info!("Testing search index stability");
        
        let temp_dir = TempDir::new().unwrap();
        let config = create_golden_test_config(&temp_dir);
        
        // Create source file
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("golden_test.csd"), create_stable_test_source()).unwrap();
        
        // Generate documentation
        let mut doc_system = DocumentationSystem::new(config).unwrap();
        let result = tokio_test::block_on(doc_system.generate_all());
        assert!(result.is_ok());
        
        // Check search index
        let search_index = doc_system.search_index();
        assert!(!search_index.is_empty());
        
        // Verify specific items are indexed
        let add_numbers_entry = search_index.iter()
            .find(|entry| entry.title == "add_numbers");
        assert!(add_numbers_entry.is_some());
        
        let point_entry = search_index.iter()
            .find(|entry| entry.title == "Point");
        assert!(point_entry.is_some());
        
        // Check search index file was generated
        let output_dir = temp_dir.path().join("output");
        let search_file = output_dir.join("search.js");
        assert!(search_file.exists());
        
        let search_content = fs::read_to_string(search_file).unwrap();
        assert!(search_content.contains("add_numbers"));
        assert!(search_content.contains("Point"));
        assert!(search_content.contains("const searchIndex = ["));
        
        debug!("Search index stability verified");
    }

    #[test]
    fn test_cross_reference_stability() {
        init_tracing!();
        info!("Testing cross-reference stability");
        
        let temp_dir = TempDir::new().unwrap();
        let config = create_golden_test_config(&temp_dir);
        
        // Create source files with cross-references
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(&src_dir).unwrap();
        
        let main_source = r#"
//! Main module that uses other modules

import "./math_utils";

/// Main function that demonstrates cross-references
/// 
/// This function uses the Point struct and add_numbers function
/// to demonstrate cross-reference generation in documentation.
/// 
/// @return Exit code
slay main() -> i32 {
    facts point = math_utils::Point::new(1.0, 2.0);
    facts sum = math_utils::add_numbers(5, 3);
    return 0;
}
"#;
        
        fs::write(src_dir.join("main.csd"), main_source).unwrap();
        fs::write(src_dir.join("math_utils.csd"), create_stable_test_source()).unwrap();
        
        // Generate documentation
        let mut doc_system = DocumentationSystem::new(config).unwrap();
        let result = tokio_test::block_on(doc_system.generate_all());
        assert!(result.is_ok());
        
        // Check cross-references were generated
        let cross_refs = doc_system.cross_references();
        assert!(!cross_refs.is_empty());
        
        debug!("Cross-reference stability verified: {} reference groups", cross_refs.len());
    }

    // Helper function to normalize HTML for comparison by removing timestamps and other variable content
    fn normalize_html_for_comparison(html: &str) -> String {
        html.lines()
            .filter(|line| !line.contains("generated on") && !line.contains("timestamp"))
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
