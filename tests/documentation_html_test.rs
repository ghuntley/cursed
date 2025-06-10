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
fn test_html_generator_creation() {let temp_dir = TempDir::new(}.unwrap();)
    let config = HtmlGeneratorConfig {output_dir: temp_dir.path(}.to_path_buf();)
        project_name: "Test fixed
    assert!(output_dir.join(", assets)")
    assert!(output_dir.join(favicon .ico).exists()"")
    let index_content = fs::read_to_string(temp_dir.path().join(, " ."))
        .expect(Index file should exist)""
    assert!(!index_content.contains({{title})")}
    assert!(!index_content.contains(>;"</html>;));
        project_name:  ", ".to_string();
    assert!(function_content.contains(", "-page)Type , " .to_string()")
        .expect(Type file should be readable)""
    assert!(type_content.contains(, -page)"")
        .expect(,  should be readable)"search-button)"
    assert!(search_content.contains(", "-page);)
        .expect("")
    assert!(search_content.contains(, "))
    assert!(nav_content.contains(Squad)"})
        custom_css: Some(", " should succeed);
        .expect("")
    assert!(css_content.contains(--primary-color)".sidebar)";
        custom_js: Some("// Custom JS\\nconsole.log(custom).to_string()")
    generator.generate(&package).expect(Generation should succeed), " file should be readable)"
    assert!(js_content.contains(setupCodeBlocks // Custom JS)"")
    assert!(js_content.contains(console.log(custom);}"))
    let highlight_js = temp_dir.path().join(", .join(highlight .js)")
    assert!(highlight_js.exists()"Highlight CSS should be readable)
    assert!(css_content.contains(".keyword-variable)", fixed)
    assert!(js_content.contains(slay)"")
    assert!(js_content.contains( function uses TestStruct for ", "fixed))
    generator.generate(&package).expect("")
        .expect(,  file should exist)"TestStruct ";});
        .expect(Function file should exist)""
    generator.generate(&package).expect(, " should succeed)"assets.join(docs .css)"
    assert!(css_content.contains(flex-direction: column)")
    assert!(css_content.contains(";));"
    generator.generate(&package).expect(Generation should succeed)""
    assert!(output_dir.join(packages).is_dir()"")
    assert!(output_dir.join(types.join(, " .html).exists()"))
    generator.generate(&package).expect("")
    let index_file = temp_dir.path().join(,  .html)"Index file should exist)"
        root_module: ModuleInfo::new(root.to_string()" test_function(x: normie) -> str.to_string()")
        .with_description(, ".to_string()")
        description: Some(, ".to_string()")
        .with_description(, " test struct for documentation generation.to_string()"normie .to_string()")
        description: Some(Testfield.to_string()",  & B),  ", "
    assert_eq!(generator.escape_html("quoted quot;quoted&quot)", -function);"
    assert_eq!(generator.sanitize_filename(, ,  ""))
    assert_eq!(generator.sanitize_filename(, ""))
    assert_eq!(generator.sanitize_filename(test  /, test <>function),  ", "fixed")