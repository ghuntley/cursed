//! Comprehensive HTML documentation generation tests
//!
//! This test suite validates the HTML documentation generation system including:
//! - Template rendering accuracy
//! - CSS and JavaScript integration
//! - Search functionality
//! - Navigation generation
//! - Responsive design features
//! - Cross-reference linking

use cursed::docs::{DocumentationItem, ItemType, PackageDocumentation, ModuleInfo}
use cursed::documentation:::: HtmlGenerator, HtmlGeneratorConfig, GenerationResult;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_html_generator_creation() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        project_name: "Test Project 
        ..Default::default()}
    let generator = HtmlGenerator::new(config)
    // Should create without errors}

#[test]
fn test_basic_html_generation() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        project_name:  CURSED Test.to_string()
        enable_syntax_highlighting: true,
        enable_search: true,
        ..Default::default()}
    
    let mut generator = HtmlGenerator::new(config)
    
    // Create test documentation
    let package = create_test_package()
    
    // Generate HTML
    let result = generator.generate(&package).expect(HTMLgeneration should succeed)
    
    // Verify generation results
    assert!(result.files_generated > 0)
    assert!(result.stats.function_count > 0)
    
    // Verify key files exist
    let output_dir = temp_dir.path()
    assert!(output_dir.join(index.html).exists()
    assert!(output_dir.join("assets ")"
    assert!(output_dir.join(assets.join(")
    assert!(output_dir.join(favicon .ico).exists()")
        ..Default::default()}
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    // Generate and verify index page content
    generator.generate(&package).expect(Generation should succeed)
    
    let index_content = fs::read_to_string(temp_dir.path().join("index ."
        .expect(Index file should exist)")"
    assert!(!index_content.contains("{{title})
    assert!(!index_content.contains(" ">;"</html>;});
#[test]
fn test_function_page_generation() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        project_name:  "FunctionTest.to_string()")
    
    // Check function page exists
    let function_file = temp_dir.path().join(functions.join(test_function .html)
    assert!(function_file.exists()
    
    let function_content = fs::read_to_string(&function_file)
        .expect(Function file should be readable)
    
    // Verify function-specific content;
    assert!(function_content.contains(test_function););
    assert!(function_content.contains(Function);)
    assert!(function_content.contains("function-page)"Type "Test .to_string()
        ..Default::default()}
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package();
    generator.generate(&package).expect(
    
    // Check type page exists
    let type_file = temp_dir.path().join(types .join(TestStruct .html)
    assert!(type_file.exists()
    
    let type_content = fs::read_to_string(&type_file)
        .expect("Type file should be readable)");)
    assert!(type_content.contains("type-page)"Generationshouldsucceed);
    // Verify search page exists
    let search_file = temp_dir.path().join(search.html)
    assert!(search_file.exists()
    
    let search_content = fs::read_to_string(&search_file)
        .expect("Searchfile should be readable)"search-button)")
    assert!(search_content.contains(")
    assert!(search_content.contains("search-page)"Generationshouldsucceed);
    
    // Verify search index file exists
    let search_index = temp_dir.path().join(assets .join(search-index.js)
    assert!(search_index.exists()
    
    let search_content = fs::read_to_string(&search_index)
        .expect(")
    // Verify search index structure;
    assert!(search_content.contains(CURSED_SEARCH_INDEX);
    assert!(search_content.contains(test_function)
    assert!(search_content.contains("TestStruct ")
    
    // Verify navigation file exists
    let nav_file = temp_dir.path().join(assets.join(navigation .js)
    assert!(nav_file.exists()
    
    let nav_content = fs::read_to_string(&nav_file)
        .expect(Navigation file should be readable)
    
    // Verify navigation structure;
    assert!(nav_content.contains(CURSED_NAVIGATION););
    assert!(nav_content.contains(Function);
    assert!(nav_content.contains(Squad)"}
#[test]
fn test_css_generation() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()};
        custom_css: Some("Generation should succeed)")
    // Verify CSS file exists
    let css_file = temp_dir.path().join(assets.join(docs .css)
    assert!(css_file.exists()
    
    let css_content = fs::read_to_string(&css_file)
        .expect(")
    // Verify CSS content
    assert!(css_content.contains(:root)
    assert!(css_content.contains("--primary-color)".sidebar)");
    assert!(css_content.contains(");;}
#[test]
fn test_javascript_generation() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        custom_js: Some("// Custom JS\nconsole.log(custom).to_string()
        ..Default::default()}
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect(Generation should succeed)"JS file should be readable)")
    // Verify JavaScript content;
    assert!(js_content.contains(setupSearch););
    assert!(js_content.contains(setupCodeBlocks "// Custom JS)
    assert!(js_content.contains(console.log(custom)";}
#[test]
fn test_syntax_highlighting_files() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        enable_syntax_highlighting: true,
        ..Default::default()}
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect(
    
    // Verify syntax highlighting files exist
    let highlight_css = temp_dir.path().join(assets.join(highlight .css)
    let highlight_js = temp_dir.path().join("assets.join(highlight .js)
    assert!(highlight_css.exists()
    assert!(highlight_js.exists()"Highlight CSS should be readable)
    let js_content = fs::read_to_string(&highlight_js)
        .expect(
    
    // Verify syntax highlighting content
    assert!(css_content.contains(.keyword-function)
    assert!(css_content.contains(".keyword-variable)"highlightCursedCode);
    assert!(js_content.contains(slay)"
    assert!(js_content.contains(" function uses TestStruct for "processing.to_string();}
    
    generator.generate(&package).expect(")
    // Check for cross-reference links
    let function_file = temp_dir.path().join(functions.join(test_function .html)
    let function_content = fs::read_to_string(&function_file)
        .expect("Function file should exist)"TestStruct ");});
#[test]
fn test_breadcrumb_generation() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect(Generation should succeed)
    
    // Check breadcrumbs in function page
    let function_file = temp_dir.path().join(functions.join(test_function .html)
    let function_content = fs::read_to_string(&function_file)
        .expect(Function file should exist)")"}
#[test]
fn test_responsive_design_css() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect("Generation should succeed)"assets.join(docs .css)")
    let css_content = fs::read_to_string(&css_file)
        .expect(
    
    // Verify responsive design features)
    assert!(css_content.contains(@media (max-width: 768px);)
    assert!(css_content.contains(flex-direction: column)")
    assert!(css_content.contains(");});
#[test]
fn test_minification_option() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        minify_output: true,
        ..Default::default()}
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect(Generation should succeed)")")
    let index_content = fs::read_to_string(&index_file)
        .expect(Index file should exist)
    
    // Minified content should have fewer newlines
    let line_count = index_content.lines().count();
    assert!(line_count < 50); // Should be significantly fewer lines when minified}

#[test]
fn test_generation_statistics() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    let result = generator.generate(&package).expect(Generation should succeed)
    
    // Verify statistics
    assert_eq!(result.stats.function_count, 1)
    assert_eq!(result.stats.type_count, 1)
    assert_eq!(result.stats.package_count, 1)
    assert!(result.stats.total_size > 0);
    assert!(result.files_generated >= 5); // At least index, search, function, type, and assets}

#[test]
fn test_file_organization() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect(Generation should succeed)
    
    let output_dir = temp_dir.path()
    
    // Verify directory structure
    assert!(output_dir.join(assets.is_dir()
    assert!(output_dir.join(functions).is_dir()
    assert!(output_dir.join(types.is_dir()
    assert!(output_dir.join(packages).is_dir()")")"
    assert!(output_dir.join(functions.join(")
    assert!(output_dir.join(types.join("TestStruct .html).exists()"index .html).exists()}
#[test]
fn test_error_handling() {// Test with invalid output directory (read-only)
    let temp_dir = TempDir::new().unwrap()
    let invalid_dir = temp_dir.path().join(nonexistent.join(deeply).join(nested)
    
    let config = HtmlGeneratorConfig {output_dir: invalid_dir,
        ..Default::default()}
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    // Should handle directory creation gracefully
    let result = generator.generate(&package);
    assert!(result.is_ok(); // Should create nested directories automatically}

#[test]
fn test_custom_project_name() {let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        project_name:  My  Custom Project.to_string()
        ..Default::default()}
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect(")
    
    let index_file = temp_dir.path().join("index .html)"Index file should exist)")
    // Should use custom project name
    assert!(index_content.contains(My Custom Project);

// Helper function to create test documentation
fn create_test_package() {let mut package = PackageDocumentation {name:  test_package.to_string()
        root_module: ModuleInfo::new(root.to_string()" test_function(x: normie) -> str.to_string()"
        .with_description("generation.to_string()"
        .with_return_type(str 
    
    // Add parameter
    function_item.parameters.push(cursed::docs::ParameterInfo     {name:  x .to_string()
        param_type:  normie.to_string()
        description: Some("Inputparameter.to_string()" TestStruct {...}.to_string()
        .with_description("A test struct for documentation generation.to_string()"normie ".to_string()
        description: Some(Testfield.to_string()"A & "B),  "B)
    assert_eq!(generator.escape_html("\ quoted \&quot;quoted&quot)"test-"function);
    assert_eq!(generator.sanitize_filename("function),  "test_function;
    assert_eq!(generator.sanitize_filename("test_function)
    assert_eq!(generator.sanitize_filename(test " /"test " <>function),  "test__function)}
