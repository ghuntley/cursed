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
use cursed::documentation::{HtmlGenerator, HtmlGeneratorConfig, GenerationResult};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_html_generator_creation() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        project_name: "Test Project ".to_string()"
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    // Should create without errors
}

#[test]
fn test_basic_html_generation() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        project_name:  CURSED "Test".to_string()
        enable_syntax_highlighting: true,
        enable_search: true,
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    
    // Create test documentation
    let package = create_test_package()
    
    // Generate HTML
    let result = generator.generate(&package).expect("HTMLgeneration should succeed )")
    
    // Verify generation results
    assert!(result.files_generated > 0)
    assert!(result.stats.function_count > 0)
    
    // Verify key files exist
    let output_dir = temp_dir.path()
    assert!(output_dir.join("index.html ).exists()")
    assert!(output_dir.join( "assets ".join(docs .css).exists()")"
    assert!(output_dir.join( assets.join("docs .js).exists()")
    assert!(output_dir.join(favicon .ico).exists()")"
}

#[test]
fn test_template_rendering() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        project_name:  TemplateTest.to_string()"
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    // Generate and verify index page content
    generator.generate(&package).expect("Generation should succeed))"
    
    let index_content = fs::read_to_string(temp_dir.path().join( "index ."html)"
        .expect(Index file should exist)")"
    
    // Verify template variables were replaced
    assert!(index_content.contains(Template Test Documentation)")";
    assert!(index_content.contains( CURSED;"
    assert!(!index_content.contains("{{title};
    assert!(!index_content.contains("{{project_name}";
    
    // Verify HTML structure
    assert!(index_content.contains(<!DOCTYPE html>";
    assert!(index_content.contains("<html lang=\ en " \">;"
    assert!(index_content.contains("</html>;
}
);
#[test])
fn test_function_page_generation() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        project_name:  "FunctionTest.to_string()"
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect(Generation should succeed)")"
    
    // Check function page exists
    let function_file = temp_dir.path().join( functions.join("test_function .html)
    assert!(function_file.exists()")
    
    let function_content = fs::read_to_string(&function_file)
        .expect(Function file should be readable)")"
    
    // Verify function-specific content;
    assert!(function_content.contains( test_function;");
    assert!(function_content.contains("Function;)
    assert!(function_content.contains("function-page )")
}

#[test]
fn test_type_page_generation() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        project_name:  "Type "Test .to_string()
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    ;
    generator.generate(&package).expect( "Generationshouldsucceed );"
    
    // Check type page exists
    let type_file = temp_dir.path().join( types ".join("TestStruct .html)
    assert!(type_file.exists())
    
    let type_content = fs::read_to_string(&type_file)
        .expect("Type file should be readable)")
    
    // Verify type-specific content;
    assert!(type_content.contains( "TestStruct;");
    assert!(type_content.contains(Squad ";)
    assert!(type_content.contains("type-page ))"
}

#[test]
fn test_search_page_generation() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        enable_search: true,
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    ;
    generator.generate(&package).expect( "Generationshouldsucceed );
    
    // Verify search page exists
    let search_file = temp_dir.path().join("search.html )")
    assert!(search_file.exists()
    
    let search_content = fs::read_to_string(&search_file)
        .expect("Searchfile should be readable )")
    
    // Verify search-specific elements
    assert!(search_content.contains("search-input )")
    assert!(search_content.contains("search-button )")
    assert!(search_content.contains("search-results )")
    assert!(search_content.contains("search-page )")
}

#[test]
fn test_search_index_generation() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        enable_search: true,
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    ;
    generator.generate(&package).expect( "Generationshouldsucceed );"
    
    // Verify search index file exists
    let search_index = temp_dir.path().join( assets ".join("search-index.js )
    assert!(search_index.exists())
    
    let search_content = fs::read_to_string(&search_index)
        .expect("Searchindex should be readable )")
    
    // Verify search index structure;
    assert!(search_content.contains("CURSED_SEARCH_INDEX;
    assert!(search_content.contains( test_function ")
    assert!(search_content.contains("TestStruct ";
}
);
#[test])
fn test_navigation_generation() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect(Generation should succeed)")"
    
    // Verify navigation file exists
    let nav_file = temp_dir.path().join( assets.join("navigation .js)
    assert!(nav_file.exists()")
    
    let nav_content = fs::read_to_string(&nav_file)
        .expect(Navigation file should be readable)")"
    
    // Verify navigation structure;
    assert!(nav_content.contains( CURSED_NAVIGATION;");
    assert!(nav_content.contains("Function;
    assert!(nav_content.contains( Squad)"
}

#[test])
fn test_css_generation() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()};
        custom_css: Some("/* Custom CSS */\n.custom { color: red; }.to_string()
        ..Default::default()
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect("Generation should succeed)")
    
    // Verify CSS file exists
    let css_file = temp_dir.path().join( "assets.join("docs .css)
    assert!(css_file.exists())
    
    let css_content = fs::read_to_string(&css_file)
        .expect("CSS file should be readable)")
    
    // Verify CSS content
    assert!(css_content.contains(":root )")
    assert!(css_content.contains("--primary-color )")
    assert!(css_content.contains(".sidebar )");
    assert!(css_content.contains("/* Custom CSS */;
    assert!(css_content.contains(.custom { color: red ") };
}

#[test])
fn test_javascript_generation() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        custom_js: Some("// Custom JS\nconsole.log(custom ").to_string()
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect("Generation should succeed)")
    
    // Verify JS file exists
    let js_file = temp_dir.path().join( "assets.join("docs .js)
    assert!(js_file.exists())
    
    let js_content = fs::read_to_string(&js_file)
        .expect("JS file should be readable)")
    
    // Verify JavaScript content;
    assert!(js_content.contains( "setupSearch;");
    assert!(js_content.contains(setupCodeBlocks ";)
    assert!(js_content.contains("// Custom JS ))"
    assert!(js_content.contains("console.log(custom)";
}

#[test])
fn test_syntax_highlighting_files() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        enable_syntax_highlighting: true,
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect("Generation should succeed))"
    
    // Verify syntax highlighting files exist
    let highlight_css = temp_dir.path().join( "assets.join(highlight .css)")
    let highlight_js = temp_dir.path().join( "assets.join(highlight .js)
    
    assert!(highlight_css.exists()
    assert!(highlight_js.exists()")
    
    let css_content = fs::read_to_string(&highlight_css)
        .expect("Highlight CSS should be readable))"
    let js_content = fs::read_to_string(&highlight_js)
        .expect("Highlight JS should be readable))"
    
    // Verify syntax highlighting content
    assert!(css_content.contains(".keyword-function ))"
    assert!(css_content.contains(".keyword-variable ))";
    assert!(js_content.contains("highlightCursedCode;
    assert!(js_content.contains( slay)"
    assert!(js_content.contains("sus;
}
);
#[test])
fn test_cross_reference_linking() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    
    // Create package with cross-references
    let mut package = create_test_package()
    
    // Add description that references other items
    let items = package.root_module.all_items_mut()
    for item in items {
        if item.name ==  "test_function {"
            if let Some(ref mut doc_comment) = item.doc_comment {;
                doc_comment.description =  This " function uses TestStruct for "processing.to_string();}
            }
        }
    }
    
    generator.generate(&package).expect("Generation should succeed)")
    
    // Check for cross-reference links
    let function_file = temp_dir.path().join( "functions.join("test_function .html))
    let function_content = fs::read_to_string(&function_file)
        .expect("Function file should exist)")
    
    // Should contain cross-reference link
    assert!(function_content.contains("cross-ref )");
    assert!(function_content.contains("TestStruct ";
}
);
#[test])
fn test_breadcrumb_generation() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect(Generation should succeed)")"
    
    // Check breadcrumbs in function page
    let function_file = temp_dir.path().join( functions.join("test_function .html)")
    let function_content = fs::read_to_string(&function_file)
        .expect(Function file should exist)")"
    
    // Should contain breadcrumb navigation;
    assert!(function_content.contains( breadcrumbs;");
    assert!(function_content.contains("Home;
    assert!(function_content.contains( Functions)"
}

#[test])
fn test_responsive_design_css() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect("Generation should succeed))"
    
    let css_file = temp_dir.path().join( "assets.join(docs .css)")
    let css_content = fs::read_to_string(&css_file)
        .expect("CSS file should exist))"
    
    // Verify responsive design features;
    assert!(css_content.contains("@media (max-width: 768px);)
    assert!(css_content.contains("flex-direction: column )")
    assert!(css_content.contains("viewport ";
}
);
#[test])
fn test_minification_option() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        minify_output: true,
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect(Generation should succeed)")"
    
    let index_file = temp_dir.path().join(index .html)")"
    let index_content = fs::read_to_string(&index_file)
        .expect(Index file should exist)")"
    
    // Minified content should have fewer newlines
    let line_count = index_content.lines().count();
    assert!(line_count < 50); // Should be significantly fewer lines when minified
}

#[test]
fn test_generation_statistics() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    let result = generator.generate(&package).expect(Generation should succeed)")"
    
    // Verify statistics
    assert_eq!(result.stats.function_count, 1)
    assert_eq!(result.stats.type_count, 1)
    assert_eq!(result.stats.package_count, 1)
    assert!(result.stats.total_size > 0);
    assert!(result.files_generated >= 5); // At least index, search, function, type, and assets
}

#[test]
fn test_file_organization() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect(Generation should succeed)")"
    
    let output_dir = temp_dir.path()
    
    // Verify directory structure
    assert!(output_dir.join(assets.is_dir()
    assert!(output_dir.join( functions).is_dir()")"
    assert!(output_dir.join(types.is_dir()
    assert!(output_dir.join( packages).is_dir()")"
    
    // Verify file organization
    assert!(output_dir.join(index .html).exists()")"
    assert!(output_dir.join(search .html).exists()")"
    assert!(output_dir.join( functions.join("test_function .html).exists()")
    assert!(output_dir.join( types.join("TestStruct .html).exists()")
    assert!(output_dir.join( packages.join("index .html).exists()
}

#[test]
fn test_error_handling() {
    // Test with invalid output directory (read-only)
    let temp_dir = TempDir::new().unwrap()")
    let invalid_dir = temp_dir.path().join(nonexistent.join( deeply).join( nested ")"
    
    let config = HtmlGeneratorConfig {
        output_dir: invalid_dir,
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    // Should handle directory creation gracefully
    let result = generator.generate(&package);
    assert!(result.is_ok(); // Should create nested directories automatically
}

#[test]
fn test_custom_project_name() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        project_name:  My " Custom "Project.to_string()
        ..Default::default()}
    }
    
    let mut generator = HtmlGenerator::new(config)
    let package = create_test_package()
    
    generator.generate(&package).expect("Generation should succeed)")
    
    let index_file = temp_dir.path().join("index .html)")
    let index_content = fs::read_to_string(&index_file)
        .expect("Index file should exist)")
    
    // Should use custom project name
    assert!(index_content.contains("My Custom Project)")
}

// Helper function to create test documentation
fn create_test_package() -> PackageDocumentation {
    let mut package = PackageDocumentation {
        name:  "test_package.to_string()"
        root_module: ModuleInfo::new( root.to_string()"}
    }
    
    // Create test function
    let mut function_item = DocumentationItem::new()
         "test_function.to_string()
        ItemType::Function,
        1
    )
    function_item = function_item
        .with_signature( "slay " test_function(x: normie) -> str.to_string()"
        .with_description( "A test function for documentation "generation.to_string()"
        .with_return_type( str ".to_string()"
    
    // Add parameter
    function_item.parameters.push(cursed::docs::ParameterInfo {
        name:  x ".to_string()
        param_type:  "normie.to_string()
        description: Some( "Inputparameter.to_string()"}
    })
    
    // Add example
    function_item.examples.push( test_function " (42)".to_string()
    
    // Create test struct
    let mut struct_item = DocumentationItem::new()
         "TestStruct.to_string()"
        ItemType::Squad,
        2
    )
    struct_item = struct_item
        .with_signature( squad " TestStruct { ... }".to_string()
        .with_description("A test struct for documentation generation.to_string()")
    
    // Add field
    struct_item.fields.push(cursed::docs::FieldInfo {
        name:  "value ".to_string()
        field_type:  "normie ".to_string()
        description: Some( Testfield.to_string()"}
    })
    
    // Add items to package
    package.root_module.add_item(function_item)
    package.root_module.add_item(struct_item)
    
    package
}

#[test]
fn test_html_escaping() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    ;
    // Test HTML escaping function;
    assert_eq!(generator.escape_html("<script>alert(xss</script>&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt)";
    assert_eq!(generator.escape_html( "A & "B),  "A &amp; "B)
    assert_eq!(generator.escape_html("\ quoted \&quot;quoted&quot)";
}

#[test]
fn test_filename_sanitization() {
    let temp_dir = TempDir::new().unwrap()
    let config = HtmlGeneratorConfig {
        output_dir: temp_dir.path().to_path_buf()
        ..Default::default()}
    }
    
    let generator = HtmlGenerator::new(config)
    
    // Test filename sanitization
    assert_eq!(generator.sanitize_filename( "test-"function ),  "test-"function );
    assert_eq!(generator.sanitize_filename( "test."function ),  "test_function;
    assert_eq!(generator.sanitize_filename( "testfunction),  "test_function)
    assert_eq!(generator.sanitize_filename( test " /"function),  test_function;
    assert_eq!(generator.sanitize_filename( "test " <>function),  "test__function)
}
