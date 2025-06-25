//! Comprehensive Tests for LaTeX Documentation Generator
//! 
//! This test suite validates the LaTeX generator functionality including:
//! - Document class support (Article, Report, Book, Beamer)
//! - Syntax highlighting with both listings and minted
//! - Cross-references and bibliography generation
//! - Special character escaping
//! - Mathematical notation support
//! - Index generation
//! - Configuration options

use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

// Import the LaTeX generator and related types
use cursed::documentation::generators::{
    LaTeXGenerator, LaTeXConfig, DocumentClass, SyntaxHighlighting, ColorScheme, PackageConfig
};
use cursed::documentation::{ExtractedDocumentation, FunctionDoc, TypeDoc, ParameterDoc, FieldDoc, ExampleDoc, DocumentationItem, ItemKind};
use cursed::error::SourceLocation;

/// Helper function to create test documentation
fn create_test_documentation() -> Vec<ExtractedDocumentation> {
    let source_location = SourceLocation {
        file: "test.csd".to_string(),
        line: 1,
        column: 1,
    };

    // Create test function documentation
    let test_function = FunctionDoc {
        name: "test_function".to_string(),
        description: Some("A test function that demonstrates CURSED syntax with special characters: $, &, %, #, ^, _, ~, {, }, \\".to_string()),
        parameters: vec![
            ParameterDoc {
                name: "param_1".to_string(),
                param_type: "i32".to_string(),
                description: Some("First parameter with underscores_and_special$chars".to_string()),
                is_optional: false,
                default_value: None,
            },
            ParameterDoc {
                name: "param_2".to_string(),
                param_type: "String".to_string(),
                description: Some("String parameter".to_string()),
                is_optional: true,
                default_value: Some("\"default\"".to_string()),
            },
        ],
        return_type: Some(cursed::documentation::TypeDoc {
            name: "Result<i32, Error>".to_string(),
            description: Some("Result type with generic parameters".to_string()),
            type_def: "result".to_string(),
            fields: vec![],
            methods: vec![],
            location: source_location.clone(),
            source_code: None,
            visibility: "public".to_string(),
            generic_params: vec!["T".to_string(), "E".to_string()],
        }),
        examples: vec![
            ExampleDoc {
                title: Some("Basic Usage".to_string()),
                code: "slay example() {\n    sus result = test_function(42, \"hello\")?\n    bet result\n}".to_string(),
                description: Some("Example showing basic function usage".to_string()),
                is_runnable: true,
                expected_output: Some("42".to_string()),
            },
        ],
        location: source_location.clone(),
        source_code: Some("slay test_function(param_1: i32, param_2: String = \"default\") -> Result<i32, Error> {\n    // Function implementation\n    bet Ok(param_1)\n}".to_string()),
        visibility: "public".to_string(),
        is_async: false,
        generic_params: vec![],
    };

    // Create test type documentation
    let test_type = TypeDoc {
        name: "TestStruct".to_string(),
        description: Some("A test structure with various field types and mathematical notation: x² + y² = z²".to_string()),
        type_def: "struct".to_string(),
        fields: vec![
            FieldDoc {
                name: "field_1".to_string(),
                field_type: "i32".to_string(),
                description: Some("Integer field".to_string()),
                visibility: "public".to_string(),
                is_optional: false,
            },
            FieldDoc {
                name: "field_2".to_string(),
                field_type: "Option<String>".to_string(),
                description: Some("Optional string field with generic type".to_string()),
                visibility: "private".to_string(),
                is_optional: true,
            },
        ],
        methods: vec![test_function.clone()],
        location: source_location.clone(),
        source_code: Some("squad TestStruct {\n    field_1: i32,\n    field_2: Option<String>,\n}".to_string()),
        visibility: "public".to_string(),
        generic_params: vec!["T".to_string()],
    };

    // Create test constant
    let test_constant = DocumentationItem {
        name: "TEST_CONSTANT".to_string(),
        kind: ItemKind::Constant,
        description: Some("A test constant with special characters in documentation".to_string()),
        location: source_location.clone(),
        source_code: Some("facts TEST_CONSTANT: i32 = 42;".to_string()),
        visibility: "public".to_string(),
        metadata: {
            let mut map = HashMap::new();
            map.insert("type".to_string(), "i32".to_string());
            map
        },
    };

    vec![ExtractedDocumentation {
        source_file: PathBuf::from("test_module.csd"),
        module_doc: Some(cursed::documentation::ModuleDoc {
            name: "test_module".to_string(),
            description: Some("Test module for LaTeX generator with mathematical formulas: E = mc², ∑(x²), and special symbols: α, β, γ".to_string()),
            path: PathBuf::from("test_module.csd"),
            exports: vec!["test_function".to_string(), "TestStruct".to_string()],
            submodules: vec![],
            location: source_location.clone(),
        }),
        functions: vec![test_function],
        types: vec![test_type],
        constants: vec![test_constant],
        variables: vec![],
        submodules: vec![],
        source_code: Some("//! Test module documentation\n\nslay test_function() {}\nsquad TestStruct {}".to_string()),
        metadata: cursed::documentation::ExtractionMetadata {
            extracted_at: chrono::Utc::now(),
            generator_version: "test".to_string(),
            item_count: 3,
            warnings: vec![],
            processing_time_ms: 0,
        },
    }]
}

#[test]
fn test_latex_generator_creation() {
    let config = LaTeXConfig::default();
    let generator = LaTeXGenerator::new(config);
    // Generator should be created successfully
    assert!(true);
}

#[test]
fn test_document_class_article() {
    let mut config = LaTeXConfig::default();
    config.document_class = DocumentClass::Article;
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    let files = result.unwrap();
    
    // Should generate main document, bibliography, makefile, and script
    assert!(!files.is_empty());
    assert!(files.iter().any(|f| f.file_name().unwrap() == "documentation.tex"));
    assert!(files.iter().any(|f| f.file_name().unwrap() == "references.bib"));
    assert!(files.iter().any(|f| f.file_name().unwrap() == "Makefile"));
    assert!(files.iter().any(|f| f.file_name().unwrap() == "compile.sh"));
}

#[test]
fn test_document_class_report() {
    let mut config = LaTeXConfig::default();
    config.document_class = DocumentClass::Report;
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    let files = result.unwrap();
    
    // Report class should generate additional module files
    assert!(!files.is_empty());
    assert!(files.iter().any(|f| f.file_name().unwrap() == "documentation.tex"));
    assert!(files.iter().any(|f| f.file_name().unwrap().to_string_lossy().contains("test_module")));
}

#[test]
fn test_document_class_book() {
    let mut config = LaTeXConfig::default();
    config.document_class = DocumentClass::Book;
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    let files = result.unwrap();
    
    // Book class should generate additional module files
    assert!(!files.is_empty());
    assert!(files.iter().any(|f| f.file_name().unwrap() == "documentation.tex"));
}

#[test]
fn test_document_class_beamer() {
    let mut config = LaTeXConfig::default();
    config.document_class = DocumentClass::Beamer;
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    let files = result.unwrap();
    
    // Beamer class should generate presentation slides
    assert!(!files.is_empty());
    assert!(files.iter().any(|f| f.file_name().unwrap() == "documentation.tex"));
}

#[test]
fn test_syntax_highlighting_listings() {
    let mut config = LaTeXConfig::default();
    config.syntax_highlighting.use_minted = false;
    config.include_code_listings = true;
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    let files = result.unwrap();
    
    // Check that main document was generated
    let main_file = temp_dir.path().join("documentation.tex");
    assert!(main_file.exists());
    
    // Read and verify content contains listings setup
    let content = std::fs::read_to_string(&main_file).expect("Failed to read main file");
    assert!(content.contains("\\usepackage{listings}"));
    assert!(content.contains("\\lstdefinelanguage{CURSED}"));
    assert!(content.contains("\\lstdefinestyle{cursedstyle}"));
    assert!(!content.contains("\\usepackage{minted}"));
}

#[test]
fn test_syntax_highlighting_minted() {
    let mut config = LaTeXConfig::default();
    config.syntax_highlighting.use_minted = true;
    config.include_code_listings = true;
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    let files = result.unwrap();
    
    // Check that main document was generated
    let main_file = temp_dir.path().join("documentation.tex");
    assert!(main_file.exists());
    
    // Read and verify content contains minted setup
    let content = std::fs::read_to_string(&main_file).expect("Failed to read main file");
    assert!(content.contains("\\usepackage{minted}"));
    assert!(!content.contains("\\usepackage{listings}"));
    
    // Check that Makefile includes shell-escape flag
    let makefile = temp_dir.path().join("Makefile");
    let makefile_content = std::fs::read_to_string(&makefile).expect("Failed to read Makefile");
    assert!(makefile_content.contains("-shell-escape"));
}

#[test]
fn test_special_character_escaping() {
    let config = LaTeXConfig::default();
    let generator = LaTeXGenerator::new(config);
    
    let test_cases = vec![
        ("\\", r#"\textbackslash{}"#),
        ("{", r#"\{"#),
        ("}", r#"\}"#),
        ("$", r#"\$"#),
        ("&", r#"\&"#),
        ("%", r#"\%"#),
        ("#", r#"\#"#),
        ("^", r#"\textasciicircum{}"#),
        ("_", r#"\_"#),
        ("~", r#"\textasciitilde{}"#),
    ];
    
    for (input, expected) in test_cases {
        let escaped = generator.escape_latex(input);
        assert_eq!(escaped, expected, "Failed to escape '{}'", input);
    }
    
    // Test complex string with multiple special characters
    let complex_input = "Function with $ and % and _ and {braces}";
    let escaped = generator.escape_latex(complex_input);
    assert!(escaped.contains(r#"\$"#));
    assert!(escaped.contains(r#"\%"#));
    assert!(escaped.contains(r#"\_"#));
    assert!(escaped.contains(r#"\{"#));
    assert!(escaped.contains(r#"\}"#));
}

#[test]
fn test_cross_references_generation() {
    let mut config = LaTeXConfig::default();
    config.generate_cross_refs = true;
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    
    // Check that main document contains cross-reference packages and labels
    let main_file = temp_dir.path().join("documentation.tex");
    let content = std::fs::read_to_string(&main_file).expect("Failed to read main file");
    
    assert!(content.contains("\\usepackage{hyperref}"));
    assert!(content.contains("\\label{"));
    assert!(content.contains("colorlinks=true"));
}

#[test]
fn test_bibliography_generation() {
    let mut config = LaTeXConfig::default();
    config.generate_bibliography = true;
    config.bibliography_style = "alpha".to_string();
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    
    // Check bibliography file exists and contains entries
    let bib_file = temp_dir.path().join("references.bib");
    assert!(bib_file.exists());
    
    let bib_content = std::fs::read_to_string(&bib_file).expect("Failed to read bibliography file");
    assert!(bib_content.contains("@misc{cursed_lang"));
    assert!(bib_content.contains("@misc{cursed_docs"));
    
    // Check main document references bibliography
    let main_file = temp_dir.path().join("documentation.tex");
    let content = std::fs::read_to_string(&main_file).expect("Failed to read main file");
    assert!(content.contains("\\bibliographystyle{alpha}"));
    assert!(content.contains("\\bibliography{references}"));
}

#[test]
fn test_index_generation() {
    let mut config = LaTeXConfig::default();
    config.generate_index = true;
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    
    // Check that main document contains index setup and entries
    let main_file = temp_dir.path().join("documentation.tex");
    let content = std::fs::read_to_string(&main_file).expect("Failed to read main file");
    
    assert!(content.contains("\\usepackage{makeidx}"));
    assert!(content.contains("\\makeindex"));
    assert!(content.contains("\\printindex"));
    assert!(content.contains("\\index{"));
}

#[test]
fn test_table_of_contents_generation() {
    let mut config = LaTeXConfig::default();
    config.generate_toc = true;
    config.toc_depth = 2;
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    
    // Check that main document contains TOC
    let main_file = temp_dir.path().join("documentation.tex");
    let content = std::fs::read_to_string(&main_file).expect("Failed to read main file");
    
    assert!(content.contains("\\setcounter{tocdepth}{2}"));
    assert!(content.contains("\\tableofcontents"));
}

#[test]
fn test_mathematical_notation_support() {
    let mut config = LaTeXConfig::default();
    config.math_support = true;
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    
    // Check that main document contains math packages
    let main_file = temp_dir.path().join("documentation.tex");
    let content = std::fs::read_to_string(&main_file).expect("Failed to read main file");
    
    assert!(content.contains("\\usepackage{amsmath}"));
    assert!(content.contains("\\usepackage{amsfonts}"));
    assert!(content.contains("\\usepackage{amssymb}"));
    assert!(content.contains("\\usepackage{mathtools}"));
    
    // Check that function signatures use math mode
    assert!(content.contains("\\begin{equation*}"));
    assert!(content.contains("\\rightarrow"));
}

#[test]
fn test_custom_color_scheme() {
    let mut config = LaTeXConfig::default();
    config.syntax_highlighting.color_scheme = ColorScheme {
        background: "lightgray".to_string(),
        comment: "darkgreen".to_string(),
        keyword: "blue".to_string(),
        string: "red".to_string(),
        number: "purple".to_string(),
        function: "cyan".to_string(),
        type_name: "orange".to_string(),
    };
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    
    // Check that custom colors are used in the style definition
    let main_file = temp_dir.path().join("documentation.tex");
    let content = std::fs::read_to_string(&main_file).expect("Failed to read main file");
    
    assert!(content.contains("backgroundcolor=\\color{lightgray}"));
    assert!(content.contains("commentstyle=\\color{darkgreen}"));
    assert!(content.contains("keywordstyle=\\color{blue}"));
}

#[test]
fn test_makefile_generation() {
    let config = LaTeXConfig::default();
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    
    // Check Makefile exists and contains correct targets
    let makefile = temp_dir.path().join("Makefile");
    assert!(makefile.exists());
    
    let makefile_content = std::fs::read_to_string(&makefile).expect("Failed to read Makefile");
    assert!(makefile_content.contains("MAIN = documentation"));
    assert!(makefile_content.contains("LATEX = pdflatex"));
    assert!(makefile_content.contains("BIBTEX = bibtex"));
    assert!(makefile_content.contains("MAKEINDEX = makeindex"));
    assert!(makefile_content.contains(".PHONY: all clean cleanall view help"));
    assert!(makefile_content.contains("all: $(MAIN).pdf"));
}

#[test]
fn test_compile_script_generation() {
    let config = LaTeXConfig::default();
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    
    // Check compile script exists
    let script = temp_dir.path().join("compile.sh");
    assert!(script.exists());
    
    let script_content = std::fs::read_to_string(&script).expect("Failed to read compile script");
    assert!(script_content.contains("#!/bin/bash"));
    assert!(script_content.contains("pdflatex"));
    assert!(script_content.contains("command -v"));
    assert!(script_content.starts_with("#!/bin/bash"));
    
    // Check script is executable (on Unix systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&script).expect("Failed to get script metadata");
        let permissions = metadata.permissions();
        assert!(permissions.mode() & 0o111 != 0, "Script should be executable");
    }
}

#[test]
fn test_large_documentation_set() {
    let config = LaTeXConfig::default();
    let mut generator = LaTeXGenerator::new(config);
    
    // Create a larger set of documentation (multiple modules)
    let mut docs = create_test_documentation();
    for i in 2..=5 {
        let mut doc = docs[0].clone();
        doc.source_file = PathBuf::from(format!("module_{}.csd", i));
        if let Some(ref mut module_doc) = doc.module_doc {
            module_doc.name = format!("module_{}", i);
            module_doc.description = Some(format!("Module {} with comprehensive documentation", i));
        }
        docs.push(doc);
    }
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    let files = result.unwrap();
    
    // Should handle multiple modules correctly
    assert!(files.len() >= 4); // At least main, bib, makefile, script
    
    // Check that main document contains all modules
    let main_file = temp_dir.path().join("documentation.tex");
    let content = std::fs::read_to_string(&main_file).expect("Failed to read main file");
    
    for i in 1..=5 {
        assert!(content.contains(&format!("module_{}", i)));
    }
}

#[test]
fn test_configuration_validation() {
    // Test various configuration combinations
    let mut config = LaTeXConfig::default();
    
    // Test with all features enabled
    config.generate_toc = true;
    config.generate_lof = true;
    config.generate_lot = true;
    config.generate_index = true;
    config.generate_bibliography = true;
    config.include_code_listings = true;
    config.generate_cross_refs = true;
    config.math_support = true;
    config.unicode_support = true;
    config.custom_headers = true;
    
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    
    // Check that all features are included in the output
    let main_file = temp_dir.path().join("documentation.tex");
    let content = std::fs::read_to_string(&main_file).expect("Failed to read main file");
    
    assert!(content.contains("\\tableofcontents"));
    assert!(content.contains("\\listoffigures"));
    assert!(content.contains("\\listoftables"));
    assert!(content.contains("\\printindex"));
    assert!(content.contains("\\bibliography{references}"));
    assert!(content.contains("\\usepackage{hyperref}"));
    assert!(content.contains("\\usepackage{amsmath}"));
    assert!(content.contains("\\usepackage[utf8]{inputenc}"));
    assert!(content.contains("\\usepackage{fancyhdr}"));
}

#[test]
fn test_error_handling() {
    let config = LaTeXConfig::default();
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    // Test with invalid output directory (read-only)
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let readonly_dir = temp_dir.path().join("readonly");
    std::fs::create_dir(&readonly_dir).expect("Failed to create readonly dir");
    
    // Make directory read-only on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&readonly_dir).expect("Failed to get metadata").permissions();
        perms.set_mode(0o444);
        std::fs::set_permissions(&readonly_dir, perms).expect("Failed to set permissions");
    }
    
    // This should handle the error gracefully
    let result = generator.generate_documentation(&docs, &readonly_dir);
    
    // On Unix systems, this should fail due to permissions
    #[cfg(unix)]
    assert!(result.is_err());
    
    // On other systems, it might succeed, which is also fine
    #[cfg(not(unix))]
    let _ = result;
}

#[test]
fn test_content_accuracy() {
    let config = LaTeXConfig::default();
    let mut generator = LaTeXGenerator::new(config);
    let docs = create_test_documentation();
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = generator.generate_documentation(&docs, temp_dir.path());
    
    assert!(result.is_ok());
    
    // Check that generated content accurately reflects the input documentation
    let main_file = temp_dir.path().join("documentation.tex");
    let content = std::fs::read_to_string(&main_file).expect("Failed to read main file");
    
    // Check that function name appears in the document
    assert!(content.contains("test\\_function"));
    
    // Check that type name appears
    assert!(content.contains("TestStruct"));
    
    // Check that constant appears
    assert!(content.contains("TEST\\_CONSTANT"));
    
    // Check that parameter information is included
    assert!(content.contains("param\\_1"));
    assert!(content.contains("param\\_2"));
    
    // Check that descriptions are included (with proper escaping)
    assert!(content.contains("A test function"));
    assert!(content.contains("special"));
    
    // Check that code examples are included
    assert!(content.contains("slay example"));
    assert!(content.contains("Basic Usage"));
    
    // Check that return types are documented
    assert!(content.contains("Result"));
    
    // Check that module statistics are included
    assert!(content.contains("Functions & 1"));
    assert!(content.contains("Types & 1"));
    assert!(content.contains("Constants & 1"));
}
