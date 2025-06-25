//! HTML Documentation Output Format Tests
//! 
//! Tests the HTML output generation for documentation, including structure,
//! styling, cross-references, and search functionality. This is critical
//! for ensuring generated HTML docs are well-formed and functional.

use cursed::documentation::generator::{DocumentationGenerator, OutputFormat};
use cursed::documentation::{DocumentationConfig, DocumentationSystem};
use cursed::documentation::generator::{FunctionDoc, TypeDoc, ModuleDoc, DocumentationItem, ItemKind, ExampleDoc};
use cursed::error::SourceLocation;
use std::path::PathBuf;
use std::collections::HashMap;
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

    fn create_test_documentation_system() -> DocumentationSystem {
        let config = DocumentationConfig {
            output_formats: vec![OutputFormat::Html],
            output_dir: PathBuf::from("test_output"),
            ..Default::default()
        };
        DocumentationSystem::new(config).unwrap()
    }

    fn create_sample_function_doc() -> FunctionDoc {
        FunctionDoc {
            name: "calculate_fibonacci".to_string(),
            description: Some("Calculate the fibonacci number using recursion".to_string()),
            parameters: vec![
                cursed::documentation::generator::ParameterDoc {
                    name: "n".to_string(),
                    param_type: "i32".to_string(),
                    description: Some("The number to calculate fibonacci for".to_string()),
                },
            ],
            return_type: Some("i32".to_string()),
            return_description: Some("The fibonacci result".to_string()),
            examples: vec![
                ExampleDoc {
                    title: Some("Basic usage".to_string()),
                    code: "let result = calculate_fibonacci(10);".to_string(),
                    description: Some("Calculate fibonacci of 10".to_string()),
                },
            ],
            location: SourceLocation { 
                file: PathBuf::from("fibonacci.csd"),
                line: 15,
                column: 0,
            },
            visibility: "public".to_string(),
            source_code: Some("slay calculate_fibonacci(n: i32) -> i32 {\n    lowkey (n <= 1) {\n        return n;\n    }\n    return calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2);\n}".to_string()),
            tags: HashMap::new(),
        }
    }

    fn create_sample_type_doc() -> TypeDoc {
        TypeDoc {
            name: "Point".to_string(),
            description: Some("Represents a point in 2D space".to_string()),
            type_kind: "struct".to_string(),
            fields: vec![
                cursed::documentation::generator::FieldDoc {
                    name: "x".to_string(),
                    field_type: "f64".to_string(),
                    description: Some("X coordinate".to_string()),
                    visibility: "public".to_string(),
                },
                cursed::documentation::generator::FieldDoc {
                    name: "y".to_string(), 
                    field_type: "f64".to_string(),
                    description: Some("Y coordinate".to_string()),
                    visibility: "public".to_string(),
                },
            ],
            methods: vec![
                cursed::documentation::generator::MethodDoc {
                    name: "distance_from_origin".to_string(),
                    description: Some("Calculate distance from origin".to_string()),
                    parameters: vec![],
                    return_type: Some("f64".to_string()),
                    return_description: Some("The distance".to_string()),
                    visibility: "public".to_string(),
                },
            ],
            type_parameters: vec![],
            location: SourceLocation {
                file: PathBuf::from("geometry.csd"),
                line: 10,
                column: 0,
            },
            visibility: "public".to_string(),
            source_code: Some("squad Point {\n    x: f64,\n    y: f64,\n}".to_string()),
            tags: HashMap::new(),
        }
    }

    #[test]
    fn test_html_generator_initialization() {
        init_tracing!();
        info!("Testing HTML generator initialization");
        
        let doc_system = create_test_documentation_system();
        assert_eq!(doc_system.config().output_formats.len(), 1);
        assert!(matches!(doc_system.config().output_formats[0], OutputFormat::Html));
        
        debug!("HTML generator initialized successfully");
    }

    #[test]
    fn test_function_html_generation() {
        init_tracing!();
        info!("Testing function HTML generation");
        
        let generator = DocumentationGenerator::new(DocumentationConfig::default()).unwrap();
        let func_doc = create_sample_function_doc();
        
        let html_result = tokio_test::block_on(
            generator.generate_function_html(&func_doc)
        );
        assert!(html_result.is_ok());
        
        let html = html_result.unwrap();
        
        // Check basic structure
        assert!(html.contains("<div class=\"function-doc\">"));
        assert!(html.contains("<h3>calculate_fibonacci</h3>"));
        assert!(html.contains("Calculate the fibonacci number"));
        
        // Check parameters section
        assert!(html.contains("<div class=\"parameters\">"));
        assert!(html.contains("<code>n: i32</code>"));
        assert!(html.contains("The number to calculate fibonacci for"));
        
        // Check return type
        assert!(html.contains("<div class=\"return-type\">"));
        assert!(html.contains("<code>i32</code>"));
        assert!(html.contains("The fibonacci result"));
        
        // Check examples
        assert!(html.contains("<div class=\"examples\">"));
        assert!(html.contains("let result = calculate_fibonacci(10);"));
        
        // Check source code
        assert!(html.contains("<div class=\"source-code\">"));
        assert!(html.contains("slay calculate_fibonacci"));
        
        debug!("Function HTML generated successfully: {} characters", html.len());
    }

    #[test]
    fn test_type_html_generation() {
        init_tracing!();
        info!("Testing type HTML generation");
        
        let generator = DocumentationGenerator::new(DocumentationConfig::default()).unwrap();
        let type_doc = create_sample_type_doc();
        
        let html_result = tokio_test::block_on(
            generator.generate_type_html(&type_doc)
        );
        assert!(html_result.is_ok());
        
        let html = html_result.unwrap();
        
        // Check basic structure
        assert!(html.contains("<div class=\"type-doc\">"));
        assert!(html.contains("<h3>Point</h3>"));
        assert!(html.contains("Represents a point in 2D space"));
        
        // Check fields section
        assert!(html.contains("<div class=\"fields\">"));
        assert!(html.contains("<code>x: f64</code>"));
        assert!(html.contains("<code>y: f64</code>"));
        assert!(html.contains("X coordinate"));
        assert!(html.contains("Y coordinate"));
        
        // Check methods section
        assert!(html.contains("<div class=\"methods\">"));
        assert!(html.contains("distance_from_origin"));
        assert!(html.contains("Calculate distance from origin"));
        
        debug!("Type HTML generated successfully: {} characters", html.len());
    }

    #[test]
    fn test_module_page_generation() {
        init_tracing!();
        info!("Testing module page HTML generation");
        
        let generator = DocumentationGenerator::new(DocumentationConfig::default()).unwrap();
        let module_doc = ModuleDoc {
            name: "math_utils".to_string(),
            description: Some("Mathematical utility functions".to_string()),
            path: PathBuf::from("src/math_utils.csd"),
            functions: vec![create_sample_function_doc()],
            types: vec![create_sample_type_doc()],
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
        
        let html_result = tokio_test::block_on(
            generator.generate_module_page(&module_doc)
        );
        assert!(html_result.is_ok());
        
        let html = html_result.unwrap();
        
        // Check page structure
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<html"));
        assert!(html.contains("<head>"));
        assert!(html.contains("<body>"));
        
        // Check module title
        assert!(html.contains("<h1>math_utils</h1>"));
        assert!(html.contains("Mathematical utility functions"));
        
        // Check table of contents
        assert!(html.contains("<div class=\"toc\">"));
        assert!(html.contains("Table of Contents"));
        
        // Check function and type sections
        assert!(html.contains("calculate_fibonacci"));
        assert!(html.contains("Point"));
        
        // Check navigation
        assert!(html.contains("<nav"));
        
        // Check CSS and JavaScript includes
        assert!(html.contains("<link rel=\"stylesheet\""));
        assert!(html.contains("<script"));
        
        debug!("Module page HTML generated successfully: {} characters", html.len());
    }

    #[test]
    fn test_index_page_generation() {
        init_tracing!();
        info!("Testing index page HTML generation");
        
        let generator = DocumentationGenerator::new(DocumentationConfig::default()).unwrap();
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
            ModuleDoc {
                name: "geometry".to_string(),
                description: Some("Geometric calculations".to_string()),
                path: PathBuf::from("geometry.csd"),
                functions: vec![],
                types: vec![create_sample_type_doc()],
                constants: vec![],
                variables: vec![],
                submodules: vec![],
                location: SourceLocation {
                    file: PathBuf::from("geometry.csd"),
                    line: 1,
                    column: 0,
                },
                visibility: "public".to_string(),
                tags: HashMap::new(),
            },
        ];
        
        let html_result = tokio_test::block_on(
            generator.generate_index_page(&modules)
        );
        assert!(html_result.is_ok());
        
        let html = html_result.unwrap();
        
        // Check page structure
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<title>CURSED Project Documentation</title>"));
        
        // Check project information
        assert!(html.contains("<h1>CURSED Project Documentation</h1>"));
        
        // Check module listing
        assert!(html.contains("<div class=\"module-list\">"));
        assert!(html.contains("math_utils"));
        assert!(html.contains("geometry"));
        assert!(html.contains("Mathematical utilities"));
        assert!(html.contains("Geometric calculations"));
        
        // Check links
        assert!(html.contains("href=\"math_utils.html\""));
        assert!(html.contains("href=\"geometry.html\""));
        
        debug!("Index page HTML generated successfully: {} characters", html.len());
    }

    #[test]
    fn test_search_functionality_generation() {
        init_tracing!();
        info!("Testing search functionality generation");
        
        let generator = DocumentationGenerator::new(DocumentationConfig::default()).unwrap();
        let search_index = vec![
            cursed::documentation::SearchIndexEntry {
                id: "math_utils::calculate_fibonacci".to_string(),
                title: "calculate_fibonacci".to_string(),
                item_type: "function".to_string(),
                description: "Calculate the fibonacci number".to_string(),
                keywords: vec!["fibonacci".to_string(), "calculate".to_string(), "recursion".to_string()],
                url: "math_utils.html#calculate_fibonacci".to_string(),
                module_path: "math_utils".to_string(),
            },
            cursed::documentation::SearchIndexEntry {
                id: "geometry::Point".to_string(),
                title: "Point".to_string(),
                item_type: "struct".to_string(),
                description: "Represents a point in 2D space".to_string(),
                keywords: vec!["point".to_string(), "2d".to_string(), "coordinates".to_string()],
                url: "geometry.html#Point".to_string(),
                module_path: "geometry".to_string(),
            },
        ];
        
        let search_js_result = tokio_test::block_on(
            generator.generate_search_index_js(&search_index)
        );
        assert!(search_js_result.is_ok());
        
        let search_js = search_js_result.unwrap();
        
        // Check JavaScript structure
        assert!(search_js.contains("const searchIndex = ["));
        assert!(search_js.contains("\"calculate_fibonacci\""));
        assert!(search_js.contains("\"Point\""));
        assert!(search_js.contains("\"fibonacci\""));
        assert!(search_js.contains("\"function\""));
        assert!(search_js.contains("\"struct\""));
        
        // Check search function
        assert!(search_js.contains("function performSearch("));
        assert!(search_js.contains("function filterResults("));
        
        debug!("Search JavaScript generated successfully: {} characters", search_js.len());
    }

    #[test]
    fn test_css_styling_generation() {
        init_tracing!();
        info!("Testing CSS styling generation");
        
        let generator = DocumentationGenerator::new(DocumentationConfig::default()).unwrap();
        
        let css_result = tokio_test::block_on(
            generator.generate_css_styles()
        );
        assert!(css_result.is_ok());
        
        let css = css_result.unwrap();
        
        // Check basic styling
        assert!(css.contains("body {"));
        assert!(css.contains(".function-doc {"));
        assert!(css.contains(".type-doc {"));
        assert!(css.contains(".module-doc {"));
        
        // Check responsive design
        assert!(css.contains("@media"));
        
        // Check syntax highlighting
        assert!(css.contains(".highlight"));
        assert!(css.contains(".keyword"));
        assert!(css.contains(".string"));
        assert!(css.contains(".comment"));
        
        // Check navigation styling
        assert!(css.contains("nav"));
        assert!(css.contains(".toc"));
        
        // Check search styling
        assert!(css.contains(".search-box"));
        assert!(css.contains(".search-results"));
        
        debug!("CSS styling generated successfully: {} characters", css.len());
    }

    #[test]
    fn test_cross_reference_linking() {
        init_tracing!();
        info!("Testing cross-reference linking in HTML");
        
        let generator = DocumentationGenerator::new(DocumentationConfig::default()).unwrap();
        let cross_refs = HashMap::from([
            (
                "math_utils.csd".to_string(),
                vec![
                    cursed::documentation::CrossReference {
                        target: "Point".to_string(),
                        reference_type: "type_usage".to_string(),
                        location: SourceLocation {
                            file: PathBuf::from("math_utils.csd"),
                            line: 20,
                            column: 10,
                        },
                        context: Some("Function parameter".to_string()),
                    },
                ],
            ),
        ]);
        
        let html_result = tokio_test::block_on(
            generator.generate_cross_reference_links(&cross_refs)
        );
        assert!(html_result.is_ok());
        
        let html = html_result.unwrap();
        
        // Check cross-reference structure
        assert!(html.contains("<div class=\"cross-references\">"));
        assert!(html.contains("Cross References"));
        assert!(html.contains("Point"));
        assert!(html.contains("type_usage"));
        assert!(html.contains("Function parameter"));
        
        // Check links
        assert!(html.contains("<a href="));
        
        debug!("Cross-reference linking generated successfully");
    }

    #[test]
    fn test_code_syntax_highlighting() {
        init_tracing!();
        info!("Testing code syntax highlighting");
        
        let generator = DocumentationGenerator::new(DocumentationConfig::default()).unwrap();
        let code = r#"slay calculate_fibonacci(n: i32) -> i32 {
    lowkey (n <= 1) {
        return n;
    }
    return calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2);
}"#;
        
        let highlighted_result = tokio_test::block_on(
            generator.highlight_code(code, "cursed")
        );
        assert!(highlighted_result.is_ok());
        
        let highlighted = highlighted_result.unwrap();
        
        // Check syntax highlighting classes
        assert!(highlighted.contains("<span class=\"keyword\">slay</span>"));
        assert!(highlighted.contains("<span class=\"keyword\">lowkey</span>"));
        assert!(highlighted.contains("<span class=\"keyword\">return</span>"));
        assert!(highlighted.contains("<span class=\"type\">i32</span>"));
        assert!(highlighted.contains("<span class=\"function\">calculate_fibonacci</span>"));
        
        debug!("Code syntax highlighting generated successfully");
    }

    #[test]
    fn test_responsive_design_elements() {
        init_tracing!();
        info!("Testing responsive design elements");
        
        let generator = DocumentationGenerator::new(DocumentationConfig::default()).unwrap();
        
        let responsive_css_result = tokio_test::block_on(
            generator.generate_responsive_css()
        );
        assert!(responsive_css_result.is_ok());
        
        let responsive_css = responsive_css_result.unwrap();
        
        // Check media queries
        assert!(responsive_css.contains("@media (max-width: 768px)"));
        assert!(responsive_css.contains("@media (max-width: 480px)"));
        
        // Check mobile-friendly elements
        assert!(responsive_css.contains("viewport"));
        assert!(responsive_css.contains("flex"));
        assert!(responsive_css.contains("grid"));
        
        // Check mobile navigation
        assert!(responsive_css.contains(".mobile-nav"));
        assert!(responsive_css.contains(".hamburger"));
        
        debug!("Responsive design elements generated successfully");
    }

    #[test]
    fn test_accessibility_features() {
        init_tracing!();
        info!("Testing accessibility features");
        
        let generator = DocumentationGenerator::new(DocumentationConfig::default()).unwrap();
        let func_doc = create_sample_function_doc();
        
        let html_result = tokio_test::block_on(
            generator.generate_accessible_html(&func_doc)
        );
        assert!(html_result.is_ok());
        
        let html = html_result.unwrap();
        
        // Check ARIA attributes
        assert!(html.contains("role=\""));
        assert!(html.contains("aria-label"));
        assert!(html.contains("aria-labelledby"));
        
        // Check semantic HTML
        assert!(html.contains("<main"));
        assert!(html.contains("<section"));
        assert!(html.contains("<article"));
        
        // Check heading hierarchy
        assert!(html.contains("<h1"));
        assert!(html.contains("<h2"));
        assert!(html.contains("<h3"));
        
        // Check skip links
        assert!(html.contains("skip-to-content"));
        
        debug!("Accessibility features generated successfully");
    }

    #[test]
    fn test_html_validation() {
        init_tracing!();
        info!("Testing HTML validation");
        
        let generator = DocumentationGenerator::new(DocumentationConfig::default()).unwrap();
        let module_doc = ModuleDoc {
            name: "test_module".to_string(),
            description: Some("Test module for validation".to_string()),
            path: PathBuf::from("test.csd"),
            functions: vec![create_sample_function_doc()],
            types: vec![create_sample_type_doc()],
            constants: vec![],
            variables: vec![],
            submodules: vec![],
            location: SourceLocation {
                file: PathBuf::from("test.csd"),
                line: 1,
                column: 0,
            },
            visibility: "public".to_string(),
            tags: HashMap::new(),
        };
        
        let html_result = tokio_test::block_on(
            generator.generate_module_page(&module_doc)
        );
        assert!(html_result.is_ok());
        
        let html = html_result.unwrap();
        
        // Basic HTML structure validation
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("<html"));
        assert!(html.contains("</html>"));
        assert!(html.contains("<head>"));
        assert!(html.contains("</head>"));
        assert!(html.contains("<body>"));
        assert!(html.contains("</body>"));
        
        // Check that tags are properly closed
        let open_tags = html.matches("<div").count();
        let close_tags = html.matches("</div>").count();
        assert_eq!(open_tags, close_tags, "All div tags should be properly closed");
        
        // Check for valid HTML5 elements
        assert!(html.contains("charset=\"utf-8\""));
        assert!(html.contains("viewport"));
        
        debug!("HTML validation passed successfully");
    }
}
