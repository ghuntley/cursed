/// Comprehensive test suite for the enhanced documentation system
/// 
/// Tests all major components: coverage analysis, advanced examples,
/// enhanced output formats, cross-referencing, and quality analysis.

use cursed::docs::*;
use cursed::error::{Error, SourceLocation};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use tempfile::TempDir;

#[cfg(test)]
mod enhanced_docs_tests {
    use super::*;

    /// Test coverage analyzer functionality
    #[test]
    fn test_coverage_analyzer() {
        let config = CoverageConfig::default();
        let mut analyzer = CoverageAnalyzer::new(config);

        // Create test files
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.csd");
        
        let test_content = r#"
/// This is a documented function
/// @param x The input value
/// @return The doubled value
slay double(x: i32) -> i32 {
    facts result = x * 2;
    result
}

// This function has no documentation
slay undocumented() {
    // Empty function
}

/// Documented struct
/// This struct represents a person
squad Person {
    facts name: String,
    sus age: i32,
}
"#;
        
        fs::write(&test_file, test_content).unwrap();
        
        // Analyze coverage
        let result = analyzer.analyze_files(&[test_file]).unwrap();
        
        // Verify results
        assert!(result.statistics.total_items > 0);
        assert!(result.statistics.coverage_percentage > 0.0);
        assert!(result.statistics.coverage_percentage <= 100.0);
        assert!(!result.suggestions.is_empty());
        
        // Test HTML report generation
        let html_output = temp_dir.path().join("coverage.html");
        analyzer.generate_html_report(&result, &html_output).unwrap();
        assert!(html_output.exists());
        
        // Test markdown report generation
        let md_output = temp_dir.path().join("coverage.md");
        analyzer.generate_markdown_report(&result, &md_output).unwrap();
        assert!(md_output.exists());
    }

    /// Test advanced example generator
    #[test]
    fn test_advanced_example_generator() {
        let config = ExampleConfig::default();
        let mut generator = AdvancedExampleGenerator::new(config);

        // Create test source files
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("examples.csd");
        
        let source_content = r#"
/// Example function with documentation
/// 
/// @example
/// ```cursed
/// facts result = calculate(5);
/// println(result);
/// ```
slay calculate(input: i32) -> i32 {
    input * 2
}

// Example: Usage demonstration
slay demo() {
    facts x = calculate(10);
    println(x);
}
"#;

        fs::write(&source_file, source_content).unwrap();

        // Extract examples
        let result = generator.extract_examples(&[source_file]).unwrap();
        
        // Verify extraction
        assert!(result.total_extracted > 0);
        assert!(result.extraction_errors.is_empty());
        
        let examples_db = generator.get_examples_database();
        assert!(!examples_db.examples.is_empty());
        
        // Generate interactive examples
        let output_dir = temp_dir.path().join("examples_output");
        generator.generate_interactive_examples(&output_dir).unwrap();
        
        // Verify generated files
        assert!(output_dir.join("html").exists());
        assert!(output_dir.join("markdown").exists());
        
        // Test search functionality
        let search_results = generator.search_examples("calculate", None, None);
        assert!(!search_results.is_empty());
    }

    /// Test enhanced output generator
    #[test]
    fn test_enhanced_output_generator() {
        // Create test documentation
        let documentation = create_test_documentation();
        let config = OutputConfig::default();
        let mut generator = EnhancedOutputGenerator::new(config);

        let temp_dir = TempDir::new().unwrap();
        
        // Generate all formats
        let results = generator.generate_all_formats(&documentation, temp_dir.path()).unwrap();
        
        // Verify HTML generation
        if let Some(html_result) = results.html_result {
            assert!(html_result.success);
            assert!(html_result.output_directory.exists());
            assert!(!html_result.generated_files.is_empty());
        }

        // Verify PDF generation (may fail if tools not installed)
        if let Some(pdf_result) = results.pdf_result {
            if pdf_result.success {
                assert!(pdf_result.output_path.exists());
            }
        }

        // Verify API documentation generation
        if let Some(api_result) = results.api_result {
            assert!(api_result.success);
            assert!(!api_result.output_files.is_empty());
        }
    }

    /// Test cross-reference analyzer
    #[test]
    fn test_cross_reference_analyzer() {
        let config = CrossReferenceConfig::default();
        let mut analyzer = CrossReferenceAnalyzer::new(config);

        let documentation = create_test_documentation();
        let temp_dir = TempDir::new().unwrap();
        
        // Create test source files
        let source_file = temp_dir.path().join("source.csd");
        let source_content = r#"
import "stdlib::math";

slay main() {
    facts x = calculate(5);
    println(x);
}

slay calculate(input: i32) -> i32 {
    input * 2
}

squad Person {
    facts name: String,
    sus age: i32,
}

collab Drawable {
    slay draw() -> String;
}
"#;
        
        fs::write(&source_file, source_content).unwrap();
        
        // Analyze cross-references
        let result = analyzer.analyze_cross_references(&documentation, &[source_file]).unwrap();
        
        // Verify analysis results
        assert!(result.total_references > 0);
        assert!(!result.references_by_type.is_empty());
        assert!(result.dependency_summary.total_nodes > 0);
        
        // Generate cross-reference outputs
        let output_dir = temp_dir.path().join("crossref_output");
        analyzer.generate_cross_reference_outputs(&result, &output_dir).unwrap();
        
        // Verify generated files
        assert!(output_dir.join("cross_references.html").exists());
        assert!(output_dir.join("cross_references.json").exists());
    }

    /// Test quality analyzer
    #[test]
    fn test_quality_analyzer() {
        let config = QualityConfig::default();
        let mut analyzer = DocumentationQualityAnalyzer::new(config);

        let documentation = create_test_documentation_with_issues();
        
        // Analyze quality
        let result = analyzer.analyze_quality(&documentation).unwrap();
        
        // Verify analysis results
        assert!(result.overall_score >= 0.0 && result.overall_score <= 1.0);
        assert!(!matches!(result.quality_rating, QualityRating::Critical));
        assert!(result.metrics.total_issues >= 0);
        
        // Verify category scores
        let scores = &result.category_scores;
        assert!(scores.grammar_score >= 0.0 && scores.grammar_score <= 1.0);
        assert!(scores.spelling_score >= 0.0 && scores.spelling_score <= 1.0);
        assert!(scores.style_score >= 0.0 && scores.style_score <= 1.0);
        assert!(scores.completeness_score >= 0.0 && scores.completeness_score <= 1.0);
        
        // Test quality report generation
        let temp_dir = TempDir::new().unwrap();
        let report_path = temp_dir.path().join("quality_report.html");
        analyzer.generate_quality_report(&result, &report_path).unwrap();
        assert!(report_path.exists());
        
        // Verify suggestions are generated
        assert!(!result.suggestions.is_empty());
    }

    /// Test integration of all enhanced systems
    #[test]
    fn test_enhanced_system_integration() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create comprehensive test project
        let project_dir = temp_dir.path().join("test_project");
        fs::create_dir_all(&project_dir).unwrap();
        
        // Create multiple source files
        let main_file = project_dir.join("main.csd");
        let utils_file = project_dir.join("utils.csd");
        
        fs::write(&main_file, r#"
/// Main application entry point
/// This function starts the application
/// @example
/// ```cursed
/// main();
/// ```
slay main() {
    facts result = add_numbers(5, 3);
    println("Result: " + result.to_string());
}
"#).unwrap();

        fs::write(&utils_file, r#"
/// Utility function to add two numbers
/// @param a First number
/// @param b Second number  
/// @return Sum of a and b
slay add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// Missing documentation here
slay subtract_numbers(a: i32, b: i32) -> i32 {
    a - b
}
"#).unwrap();

        let source_files = vec![main_file, utils_file];
        
        // Test complete workflow
        let output_dir = temp_dir.path().join("enhanced_docs");
        fs::create_dir_all(&output_dir).unwrap();
        
        // 1. Coverage analysis
        let coverage_config = CoverageConfig::default();
        let mut coverage_analyzer = CoverageAnalyzer::new(coverage_config);
        let coverage_result = coverage_analyzer.analyze_files(&source_files).unwrap();
        
        assert!(coverage_result.statistics.total_items > 0);
        assert!(coverage_result.statistics.coverage_percentage < 100.0); // Should detect missing docs
        
        // 2. Example extraction
        let example_config = ExampleConfig::default();
        let mut example_generator = AdvancedExampleGenerator::new(example_config);
        let example_result = example_generator.extract_examples(&source_files).unwrap();
        
        assert!(example_result.total_extracted > 0);
        
        // 3. Cross-reference analysis
        let crossref_config = CrossReferenceConfig::default();
        let mut crossref_analyzer = CrossReferenceAnalyzer::new(crossref_config);
        let documentation = create_test_documentation();
        let crossref_result = crossref_analyzer.analyze_cross_references(&documentation, &source_files).unwrap();
        
        assert!(crossref_result.total_references > 0);
        
        // 4. Quality analysis
        let quality_config = QualityConfig::default();
        let mut quality_analyzer = DocumentationQualityAnalyzer::new(quality_config);
        let quality_result = quality_analyzer.analyze_quality(&documentation).unwrap();
        
        assert!(quality_result.overall_score > 0.0);
        
        // 5. Enhanced output generation
        let output_config = OutputConfig::default();
        let mut output_generator = EnhancedOutputGenerator::new(output_config);
        let output_results = output_generator.generate_all_formats(&documentation, &output_dir).unwrap();
        
        // Verify all systems worked together
        assert!(output_dir.exists());
        
        // Generate comprehensive reports
        coverage_analyzer.generate_html_report(&coverage_result, &output_dir.join("coverage.html")).unwrap();
        example_generator.generate_interactive_examples(&output_dir.join("examples")).unwrap();
        crossref_analyzer.generate_cross_reference_outputs(&crossref_result, &output_dir.join("references")).unwrap();
        quality_analyzer.generate_quality_report(&quality_result, &output_dir.join("quality.html")).unwrap();
        
        // Verify all reports were generated
        assert!(output_dir.join("coverage.html").exists());
        assert!(output_dir.join("examples").exists());
        assert!(output_dir.join("references").exists());
        assert!(output_dir.join("quality.html").exists());
    }

    /// Test error handling in enhanced systems
    #[test]
    fn test_enhanced_system_error_handling() {
        // Test with invalid/missing files
        let nonexistent_files = vec![PathBuf::from("nonexistent.csd")];
        
        let coverage_config = CoverageConfig::default();
        let mut coverage_analyzer = CoverageAnalyzer::new(coverage_config);
        
        // Should handle missing files gracefully
        let result = coverage_analyzer.analyze_files(&nonexistent_files);
        assert!(result.is_err());
        
        // Test with empty documentation
        let empty_documentation = ExtractedDocumentation {
            items: Vec::new(),
            cross_references: Vec::new(),
            search_index: Vec::new(),
            generation_time: std::time::SystemTime::now(),
            source_files: Vec::new(),
        };
        
        let quality_config = QualityConfig::default();
        let mut quality_analyzer = DocumentationQualityAnalyzer::new(quality_config);
        let quality_result = quality_analyzer.analyze_quality(&empty_documentation).unwrap();
        
        // Should handle empty documentation
        assert_eq!(quality_result.metrics.total_issues, 0);
    }

    /// Test configuration customization
    #[test]
    fn test_configuration_customization() {
        // Test custom coverage configuration
        let mut coverage_config = CoverageConfig::default();
        coverage_config.min_quality_score = 0.9;
        coverage_config.require_public_docs = true;
        
        let coverage_analyzer = CoverageAnalyzer::new(coverage_config);
        // Analyzer should use custom configuration
        
        // Test custom quality configuration
        let mut quality_config = QualityConfig::default();
        quality_config.enable_grammar_check = false;
        quality_config.enable_spelling_check = true;
        quality_config.min_quality_score = 0.8;
        
        let quality_analyzer = DocumentationQualityAnalyzer::new(quality_config);
        // Analyzer should respect custom settings
        
        // Test custom output configuration
        let mut output_config = OutputConfig::default();
        output_config.enable_pdf = false;
        output_config.enable_api_docs = true;
        output_config.html_theme = HtmlTheme::Modern;
        
        let output_generator = EnhancedOutputGenerator::new(output_config);
        // Generator should use custom configuration
    }

    // Helper functions

    fn create_test_documentation() -> ExtractedDocumentation {
        use cursed::docs::generator::{DocumentationItem, ItemKind, Visibility, Parameter, Example, SourceInfo};
        
        let items = vec![
            DocumentationItem {
                name: "main".to_string(),
                kind: ItemKind::Function,
                description: "Main application entry point".to_string(),
                visibility: Visibility::Public,
                parameters: Vec::new(),
                examples: vec![Example {
                    title: Some("Basic usage".to_string()),
                    description: Some("How to call main".to_string()),
                    code: "main();".to_string(),
                    language: "cursed".to_string(),
                    output: None,
                }],
                source_info: SourceInfo {
                    line: 1,
                    column: 1,
                    file: Some(PathBuf::from("main.csd")),
                },
            },
            DocumentationItem {
                name: "calculate".to_string(),
                kind: ItemKind::Function,
                description: "Calculates a value".to_string(),
                visibility: Visibility::Public,
                parameters: vec![Parameter {
                    name: "input".to_string(),
                    description: "Input value".to_string(),
                }],
                examples: Vec::new(),
                source_info: SourceInfo {
                    line: 10,
                    column: 1,
                    file: Some(PathBuf::from("utils.csd")),
                },
            },
        ];

        ExtractedDocumentation {
            items,
            cross_references: Vec::new(),
            search_index: Vec::new(),
            generation_time: std::time::SystemTime::now(),
            source_files: vec![PathBuf::from("main.csd"), PathBuf::from("utils.csd")],
        }
    }

    fn create_test_documentation_with_issues() -> ExtractedDocumentation {
        use cursed::docs::generator::{DocumentationItem, ItemKind, Visibility, SourceInfo};
        
        let items = vec![
            DocumentationItem {
                name: "poorly_documented".to_string(),
                kind: ItemKind::Function,
                description: "short".to_string(), // Too short
                visibility: Visibility::Public,
                parameters: Vec::new(),
                examples: Vec::new(),
                source_info: SourceInfo {
                    line: 1,
                    column: 1,
                    file: Some(PathBuf::from("test.csd")),
                },
            },
            DocumentationItem {
                name: "undocumented".to_string(),
                kind: ItemKind::Function,
                description: "".to_string(), // Empty description
                visibility: Visibility::Public,
                parameters: Vec::new(),
                examples: Vec::new(),
                source_info: SourceInfo {
                    line: 5,
                    column: 1,
                    file: Some(PathBuf::from("test.csd")),
                },
            },
        ];

        ExtractedDocumentation {
            items,
            cross_references: Vec::new(),
            search_index: Vec::new(),
            generation_time: std::time::SystemTime::now(),
            source_files: vec![PathBuf::from("test.csd")],
        }
    }
}

/// Integration test for the complete enhanced documentation workflow
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_documentation_workflow() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join("cursed_project");
        fs::create_dir_all(&project_dir).unwrap();

        // Create a realistic CURSED project structure
        create_sample_project(&project_dir);

        // Run complete documentation generation workflow
        let output_dir = temp_dir.path().join("documentation");
        
        // This would be the main entry point for documentation generation
        let result = generate_enhanced_documentation(&project_dir, &output_dir);
        
        assert!(result.is_ok());
        assert!(output_dir.exists());
        
        // Verify all expected outputs exist
        assert!(output_dir.join("index.html").exists());
        assert!(output_dir.join("coverage").exists());
        assert!(output_dir.join("examples").exists());
        assert!(output_dir.join("api").exists());
        assert!(output_dir.join("quality").exists());
    }

    fn create_sample_project(project_dir: &Path) {
        // Create main source file
        let main_content = r#"
/// CURSED Sample Application
/// This application demonstrates the CURSED programming language features
/// 
/// @example
/// ```cursed
/// slay main() {
///     println("Hello, CURSED!");
/// }
/// ```
slay main() {
    facts greeting = "Hello, CURSED!";
    println(greeting);
    
    facts result = math_utils::add(5, 3);
    println("5 + 3 = " + result.to_string());
}
"#;

        let utils_content = r#"
/// Mathematical utility functions
/// This module provides basic mathematical operations

/// Adds two integers
/// @param a First operand
/// @param b Second operand
/// @return Sum of a and b
/// 
/// @example
/// ```cursed
/// facts sum = add(5, 3);
/// assert_eq!(sum, 8);
/// ```
slay add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two integers
/// @param a First operand
/// @param b Second operand
/// @return Product of a and b
slay multiply(a: i32, b: i32) -> i32 {
    a * b
}

// This function lacks documentation - quality issue
slay subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Person data structure
/// Represents a person with basic information
squad Person {
    facts name: String,
    sus age: i32,
    facts email: String,
}

/// Drawable interface
/// Defines the contract for drawable objects
collab Drawable {
    /// Renders the object to a string
    /// @return String representation
    slay render() -> String;
}
"#;

        fs::write(project_dir.join("main.csd"), main_content).unwrap();
        fs::write(project_dir.join("math_utils.csd"), utils_content).unwrap();
    }

    /// Main function to generate enhanced documentation
    fn generate_enhanced_documentation(source_dir: &Path, output_dir: &Path) -> Result<(), Error> {
        fs::create_dir_all(output_dir)?;

        // Collect source files
        let source_files = collect_source_files(source_dir)?;

        // 1. Generate base documentation
        let documentation = extract_documentation(&source_files)?;

        // 2. Run coverage analysis
        let coverage_config = CoverageConfig::default();
        let mut coverage_analyzer = CoverageAnalyzer::new(coverage_config);
        let coverage_result = coverage_analyzer.analyze_files(&source_files)?;
        coverage_analyzer.generate_html_report(&coverage_result, &output_dir.join("coverage.html"))?;

        // 3. Extract and generate examples
        let example_config = ExampleConfig::default();
        let mut example_generator = AdvancedExampleGenerator::new(example_config);
        let _example_result = example_generator.extract_examples(&source_files)?;
        example_generator.generate_interactive_examples(&output_dir.join("examples"))?;

        // 4. Analyze cross-references
        let crossref_config = CrossReferenceConfig::default();
        let mut crossref_analyzer = CrossReferenceAnalyzer::new(crossref_config);
        let crossref_result = crossref_analyzer.analyze_cross_references(&documentation, &source_files)?;
        crossref_analyzer.generate_cross_reference_outputs(&crossref_result, &output_dir.join("references"))?;

        // 5. Analyze quality
        let quality_config = QualityConfig::default();
        let mut quality_analyzer = DocumentationQualityAnalyzer::new(quality_config);
        let quality_result = quality_analyzer.analyze_quality(&documentation)?;
        quality_analyzer.generate_quality_report(&quality_result, &output_dir.join("quality.html"))?;

        // 6. Generate enhanced output formats
        let output_config = OutputConfig::default();
        let mut output_generator = EnhancedOutputGenerator::new(output_config);
        let _output_results = output_generator.generate_all_formats(&documentation, output_dir)?;

        Ok(())
    }

    fn collect_source_files(dir: &Path) -> Result<Vec<PathBuf>, Error> {
        let mut source_files = Vec::new();
        
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("csd") {
                        source_files.push(path);
                    }
                }
            }
        }
        
        Ok(source_files)
    }

    fn extract_documentation(source_files: &[PathBuf]) -> Result<ExtractedDocumentation, Error> {
        // This would use the main documentation generator
        // For now, create a mock documentation structure
        use cursed::docs::generator::{DocumentationItem, ItemKind, Visibility, SourceInfo};
        
        let items = vec![
            DocumentationItem {
                name: "main".to_string(),
                kind: ItemKind::Function,
                description: "CURSED Sample Application entry point".to_string(),
                visibility: Visibility::Public,
                parameters: Vec::new(),
                examples: Vec::new(),
                source_info: SourceInfo {
                    line: 8,
                    column: 1,
                    file: source_files.get(0).cloned(),
                },
            },
            DocumentationItem {
                name: "add".to_string(),
                kind: ItemKind::Function,
                description: "Adds two integers".to_string(),
                visibility: Visibility::Public,
                parameters: Vec::new(),
                examples: Vec::new(),
                source_info: SourceInfo {
                    line: 12,
                    column: 1,
                    file: source_files.get(1).cloned(),
                },
            },
        ];

        Ok(ExtractedDocumentation {
            items,
            cross_references: Vec::new(),
            search_index: Vec::new(),
            generation_time: std::time::SystemTime::now(),
            source_files: source_files.to_vec(),
        })
    }
}
