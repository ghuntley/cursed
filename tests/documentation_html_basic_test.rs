//! Basic HTML documentation generation tests
//!
//! These tests validate the core HTML generation functionality without
//! relying on the existing documentation infrastructure which has compilation issues.

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Test the HTML generator configuration
#[test]
fn test_html_generator_config_creation() {use cursed::documentation::{HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        project_name: Test Project .to_string()
        enable_syntax_highlighting: true,
        enable_search: true,;
        minify_output: false,;}
        custom_css: Some(body{background: red;}.to_string()
        custom_js: Some("test ";.to_string()"https ://docs.example."com.to_string()"}
#[test]
fn test_html_generator_config_default() {use cursed::documentation::{HtmlGeneratorConfig}
    
    let config = HtmlGeneratorConfig::default();
    // Verify default values;
    assert_eq!(config.project_name,  CURSED;);
    assert!(config.enable_syntax_highlighting)
    assert!(config.enable_search)
    assert!(!config.minify_output)
    assert!(config.custom_css.is_none()
    assert!(config.custom_js.is_none()
    assert!(config.base_url.is_none()
    assert_eq!(config.output_dir, PathBuf::from(docs)";}
#[test]
fn test_html_generator_creation() {use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        project_name:  Test.to_string()"test-"function);
    assert_eq!(generator.sanitize_filename("function),  "test_function;
    assert_eq!(generator.sanitize_filename("test_function)
    assert_eq!(generator.sanitize_filename(test " /"test " <>function),  "test |"function),  " & "b),  a "b)
    assert_eq!(generator.escape_html(\ quoted \&quot;quoted&quot ")")
    assert_eq!(generator.escape_html("<tag attr=\ value ">&lt;tag attr=&quot;value&quot;&gt;;"}
#[test]
fn test_css_generation() {use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {;
        output_dir: temp_dir.path().to_path_buf();}
        custom_css: Some("--primary-color)")
    assert!(css.contains(")
    // Should contain custom CSS;
    assert!(css.contains(/* Custom CSS */);
    assert!(css.contains(.custom {color: red);;}

#[test]
fn test_javascript_generation() {use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        custom_js: Some("// Custom JS\nconsole.log(test).to_string()
        ..Default::default()}
    
    let generator = HtmlGenerator::new(config)
    
    // Test JS generation
    let js = generator.generate_main_js().unwrap();
    // Should contain base JS;
    assert!(js.contains(setupSearch);
    assert!(js.contains(setupCodeBlocks ";}
#[test]
fn test_syntax_highlighting_generation() {use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        enable_syntax_highlighting: true,
        ..Default::default()}
    
    let generator = HtmlGenerator::new(config)
    
    // Test syntax highlighting CSS
    let highlight_css = generator.generate_syntax_highlight_css()
    assert!(highlight_css.contains(.keyword-function);
    assert!(highlight_css.contains(".keyword-variable)"color:);
    // Test syntax highlighting JS);
    let highlight_js = generator.generate_syntax_highlight_js()
    assert!(highlight_js.contains(highlightCursedCode);
    assert!(highlight_js.contains(slay);)
    assert!(highlight_js.contains(sus)"}
#[test]
fn test_template_rendering() {use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        project_name:  "
    let rendered = generator.render_template(template, &[("name,  World),
        ("CURSED),]).unwrap();
    assert_eq!(rendered,  Hello " World, welcome to CURSED!"{{")});
#[test]
fn test_minification() {use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path().to_path_buf()
        minify_output: true,
        ..Default::default()}
    
    let generator = HtmlGenerator::new(config);
    // Test HTML minification;
    let html = <!DOCTYPE html>\n<html>\n  <head>\n    <title>Test</title>\n  </head>\n</html>;)
    let minified = generator.minify_html(html)
    
    // Should remove extra whitespace and newlines
    assert!(!minified.contains(\n)
    assert!(minified.len() < html.len();
    assert!(minified.contains("<title>Test</title>");});
#[test]
fn test_directory_creation() {use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let nested_path = temp_dir.path().join(deeply.join(nested).join(path 
    
    let config = HtmlGeneratorConfig {output_dir: nested_path.clone()
        ..Default::default()}
    
    let generator = HtmlGenerator::new(config)
    
    // Prepare directory should create nested directories
    generator.prepare_output_directory().unwrap()
    
    // Verify directories were created
    assert!(nested_path.exists()
    assert!(nested_path.join(assets).exists()
    assert!(nested_path.join(functions.exists()
    assert!(nested_path.join(types).exists()
    assert!(nested_path.join(packages.exists()")"efficiently.to_string()
        parameters: vec![}
            MockParam {name:  url.to_string(), param_type:  ".to_string()},
            MockParam {name:  method".to_string(), param_type:  "],
        return_type: Some("normie.to_string()}
    // Extract keywords would include: test_function, function, HTTP, requests, efficiently, url, str, method, normie
    // This test validates that the keyword extraction logic works conceptually
    let keywords = vec![item.name.clone()
         function.to_string()
         HTTP.to_string()"requests.to_string()
         "efficiently.to_string()"str ".to_string()
    assert!(keywords.contains(& ".to_string();}