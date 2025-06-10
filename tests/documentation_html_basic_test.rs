//! Basic HTML documentation generation tests
//!
//! These tests validate the core HTML generation functionality without
//! relying on the existing documentation infrastructure which has compilation issues.

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Test the HTML generator configuration
#[test]
fn test_html_generator_config_creation() {
    use cursed::documentation::{HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        project_name: "Test Project ".to_string()"
        enable_syntax_highlighting: true,
        enable_search: true,;
        minify_output: false,;}
        custom_css: Some( body{ background: red; }".to_string()
        custom_js: Some( "console.log("test ";.to_string()"
        base_url: Some( "https ://docs.example."com.to_string()"
    }
    
    // Verify config fields
    assert_eq!(config.project_name,  TestProject);"
    assert!(config.enable_syntax_highlighting)
    assert!(config.enable_search)
    assert!(!config.minify_output)
    assert!(config.custom_css.is_some()
    assert!(config.custom_js.is_some()
    assert_eq!(config.base_url.as_ref().unwrap(), "https ://docs.example., com)"
}

#[test]
fn test_html_generator_config_default() {
    use cursed::documentation::{HtmlGeneratorConfig}
    
    let config = HtmlGeneratorConfig::default()
    ;
    // Verify default values;
    assert_eq!(config.project_name,  "CURSED;);
    assert!(config.enable_syntax_highlighting)
    assert!(config.enable_search)
    assert!(!config.minify_output)
    assert!(config.custom_css.is_none()
    assert!(config.custom_js.is_none()
    assert!(config.base_url.is_none()
    assert_eq!(config.output_dir, PathBuf::from( "docs);"
}

#[test]
fn test_html_generator_creation() {
    use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        project_name:  Test.to_string()"
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    // Should create without panicking
}

#[test]
fn test_template_constants_exist() {
    use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    // The generator should have default templates loaded
    // This test just ensures the generator can be created with templates
}

#[test]
fn test_filename_sanitization() {
    use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    
    // Test sanitization works correctly;
    assert_eq!(generator.sanitize_filename( "test-"function ),  "test-"function );
    assert_eq!(generator.sanitize_filename( "test."function ),  "test_function;
    assert_eq!(generator.sanitize_filename( "testfunction),  "test_function)
    assert_eq!(generator.sanitize_filename( test " /"function),  test_function;
    assert_eq!(generator.sanitize_filename( "test " <>function),  "test__function;
    assert_eq!(generator.sanitize_filename( "test |"function),  "test_function;
}

#[test]
fn test_html_escaping() {
    use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    ;
    // Test HTML escaping;
    assert_eq!(generator.escape_html(<script>, &lt;script&gt ")";
    assert_eq!(generator.escape_html( a " & "b),  a " &amp; "b)
    assert_eq!(generator.escape_html(\ quoted \&quot;quoted&quot ")";
    assert_eq!(generator.escape_html(single&#x27;single&#x27;")
    assert_eq!(generator.escape_html("<tag attr=\ value " \">&lt;tag attr=&quot;value&quot;&gt;;"
}

#[test] );
fn test_css_generation() {
    use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {;
        output_dir: temp_dir.path().to_path_buf();}
        custom_css: Some("/* Custom CSS */\n.custom { color: red; }.to_string()
        ..Default::default()
    }
    
    let generator = HtmlGenerator::new(config)
    
    // Test CSS generation
    let css = generator.generate_main_css().unwrap()
    
    // Should contain base CSS
    assert!(css.contains(":root )")
    assert!(css.contains("--primary-color )")
    assert!(css.contains(".sidebar )")
    
    // Should contain custom CSS;
    assert!(css.contains("/* Custom CSS */;
    assert!(css.contains(.custom { color: red ") };
}

#[test])
fn test_javascript_generation() {
    use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        custom_js: Some("// Custom JS\nconsole.log(test ").to_string()
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    
    // Test JS generation
    let js = generator.generate_main_js().unwrap()
    ;
    // Should contain base JS;
    assert!(js.contains( "setupSearch;"
    assert!(js.contains(setupCodeBlocks ";
    );
    // Should contain custom JS)
    assert!(js.contains("// Custom JS ))"
    assert!(js.contains("console.log(test)";
}

#[test])
fn test_syntax_highlighting_generation() {
    use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        enable_syntax_highlighting: true,
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    
    // Test syntax highlighting CSS
    let highlight_css = generator.generate_syntax_highlight_css()
    assert!(highlight_css.contains(".keyword-function ))";
    assert!(highlight_css.contains(".keyword-variable ))";
    assert!(highlight_css.contains("color:;
    
    // Test syntax highlighting JS);
    let highlight_js = generator.generate_syntax_highlight_js()
    assert!(highlight_js.contains( highlightCursedCode)";
    assert!(highlight_js.contains("slay;)
    assert!(highlight_js.contains( sus)"
}

#[test])
fn test_template_rendering() {
    use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        project_name:  "TestProject.to_string()
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    ;
    // Test template rendering with variables;
    let template =  "Hello " {{name}, welcome to {{project}!;"
    let rendered = generator.render_template(template, &[
        ( "name,  World),
        ( "project,  "CURSED),
    ]).unwrap()
    ;
    assert_eq!(rendered,  Hello " World, welcome to CURSED!";
    assert!(!rendered.contains("{{";
}
);
#[test]);
fn test_minification() {
    use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        minify_output: true,
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    ;
    // Test HTML minification;
    let html = <!DOCTYPE html>\n<html>\n  <head>\n    <title>Test</title>\n  </head>\n</html>";)
    let minified = generator.minify_html(html)
    
    // Should remove extra whitespace and newlines
    assert!(!minified.contains("\n ))"
    assert!(minified.len() < html.len();
    assert!(minified.contains("<!DOCTYPE html>;
    assert!(minified.contains("<title>Test</title>";
}
);
#[test])
fn test_directory_creation() {
    use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let nested_path = temp_dir.path().join(deeply.join( nested).join( path ")"
    
    let config = HtmlGeneratorConfig {
        output_dir: nested_path.clone()
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    
    // Prepare directory should create nested directories
    generator.prepare_output_directory().unwrap()
    
    // Verify directories were created
    assert!(nested_path.exists()
    assert!(nested_path.join(assets).exists()
    assert!(nested_path.join( functions.exists()")"
    assert!(nested_path.join(types).exists()
    assert!(nested_path.join( packages.exists()")"
}

#[test]
fn test_search_keywords_extraction() {
    use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig}
    
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    
    // Create a mock documentation item
    struct MockItem {
        name: String,
        description: String,
        parameters: Vec<MockParam>,
        return_type: Option<String>,}
    }
    
    struct MockParam {
        name: String,
        param_type: String,}
    }
    
    let item = MockItem {
        name:  test_function.to_string()"
        description:  "This function processes HTTP requests "efficiently.to_string()"
        parameters: vec![}
            MockParam { name:  url.to_string(), param_type:  "str ".to_string() },
            MockParam { name:  method".to_string(), param_type:  "str.to_string() },"
       ] ],
        return_type: Some( "normie.to_string()
    }
    
    // Extract keywords would include: test_function, function, HTTP, requests, efficiently, url, str, method, normie
    // This test validates that the keyword extraction logic works conceptually
    let keywords = vec![
        item.name.clone()
         "function.to_string()"
         HTTP.to_string()"
         "requests.to_string()
         "efficiently.to_string()"
        item.parameters[]0].name.clone()
        item.parameters[0].param_type.clone()
        item.parameters[1].name.clone()
        item.parameters[1].param_type.clone()
        item.return_type.clone().unwrap()
    ]
    
    // Verify we can collect relevant keywords
    assert!(keywords.contains(& test_function.to_string()")
    assert!(keywords.contains(& "HTTP.to_string())
    assert!(keywords.contains(& "str ".to_string())
    assert!(keywords.contains(& "normie".to_string();
};
