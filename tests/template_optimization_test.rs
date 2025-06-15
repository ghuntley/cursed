/// Template Optimization Tests - Comprehensive test suite for template bundler optimizations
use std::collections::HashMap;
use std::sync::Arc;

use cursed::stdlib::template::template_bundler::*;
use cursed::stdlib::template::template_core::{FileSystemLoader, TemplateLoader};
use cursed::stdlib::template::template_syntax::{TemplateAst, TemplateNode, TemplateExpression, FilterCall};

#[cfg(test)]
mod template_optimization_tests {
    use super::*;

    /// Mock template loader for testing
    struct MockTemplateLoader {
        templates: HashMap<String, String>,
    }

    impl MockTemplateLoader {
        fn new() -> Self {
            let mut templates = HashMap::new();
            
            // Add test templates
            templates.insert("main.html".to_string(), 
                "<!DOCTYPE html><html>{% include \"header.html\" %}{{ title }}{% include \"footer.html\" %}</html>".to_string());
                
            templates.insert("header.html".to_string(), 
                "<head><title>{{ site_title }}</title></head><body>".to_string());
                
            templates.insert("footer.html".to_string(), 
                "</body></html>".to_string());
                
            templates.insert("header_snippet.html".to_string(), 
                "<!-- Small header snippet -->".to_string());
                
            templates.insert("inline_nav.html".to_string(), 
                "<nav>Home | About</nav>".to_string());
                
            templates.insert("large_template.html".to_string(), 
                format!("<!-- Large template content: {} -->", "X".repeat(2000)));

            Self { templates }
        }
    }

    impl TemplateLoader for MockTemplateLoader {
        fn load(&self, name: &str) -> Result<String, cursed::error::Error> {
            self.templates.get(name)
                .map(|content| content.clone())
                .ok_or_else(|| cursed::error::Error::TemplateError {
                    message: format!("Template not found: {}", name),
                    source_location: None,
                })
        }
    }

    #[test]
    fn test_compression_optimization() {
        let config = BundleConfig {
            enable_compression: true,
            enable_minification: false,
            enable_dead_code_elimination: false,
            enable_dependency_optimization: false,
            ..Default::default()
        };
        
        let loader = Arc::new(MockTemplateLoader::new());
        let mut bundler = TemplateBundler::new(config, loader);
        
        let templates = vec!["main.html".to_string(), "header.html".to_string()];
        let result = futures::executor::block_on(bundler.create_bundle(&templates, "test_compression"));
        
        assert!(result.is_ok());
        let bundle = result.unwrap();
        
        // Compressed size should be smaller than minified size for large content
        assert!(bundle.metadata.size_info.compressed_size <= bundle.metadata.size_info.minified_size);
        
        // Bundle should contain both templates
        assert_eq!(bundle.entries.len(), 2);
        assert!(bundle.entries.contains_key("main.html"));
        assert!(bundle.entries.contains_key("header.html"));
        
        // Metadata should be properly set
        assert_eq!(bundle.metadata.bundle_id, "test_compression");
        assert!(bundle.metadata.size_info.reduction_ratio >= 0.0);
    }

    #[test]
    fn test_minification_optimizer() {
        let optimizer = MinificationOptimizer::new();
        
        let mut content = "  \n\n  Hello   World  \n\n  {# This is a comment #}\n  {{ variable }}  \n\n  ".to_string();
        let mut ast = TemplateAst { nodes: Vec::new() };
        
        let result = optimizer.optimize(&mut content, &mut ast);
        
        assert!(result.is_ok());
        let optimization_result = result.unwrap();
        
        // Should have saved bytes by removing whitespace and comments
        assert!(optimization_result.bytes_saved > 0);
        assert!(optimization_result.optimizations_applied > 0);
        
        // Content should be shorter and cleaner
        assert!(content.len() < 50); // Original was much longer
        assert!(!content.contains("{# This is a comment #}")); // Comment should be removed
        assert!(!content.contains("\n\n")); // Empty lines should be removed
    }

    #[test]
    fn test_dead_code_elimination_optimizer() {
        let optimizer = DeadCodeEliminationOptimizer::new();
        
        // Create AST with unused variable
        let ast_nodes = vec![
            TemplateNode::Set {
                name: "unused_var".to_string(),
                value: TemplateExpression::String("never used".to_string()),
                location: None,
            },
            TemplateNode::Set {
                name: "used_var".to_string(),
                value: TemplateExpression::String("this is used".to_string()),
                location: None,
            },
            TemplateNode::Variable {
                expression: TemplateExpression::Variable("used_var".to_string()),
                filters: Vec::new(),
                location: None,
            },
        ];
        
        let mut content = "{% set unused_var = \"never used\" %}{% set used_var = \"this is used\" %}{{ used_var }}".to_string();
        let mut ast = TemplateAst { nodes: ast_nodes };
        
        let result = optimizer.optimize(&mut content, &mut ast);
        
        assert!(result.is_ok());
        let optimization_result = result.unwrap();
        
        // Should have found and removed unused variable
        assert!(optimization_result.optimizations_applied > 0);
        
        // Warnings should mention unused variables
        assert!(!optimization_result.warnings.is_empty());
        assert!(optimization_result.warnings[0].contains("unused variables"));
    }

    #[test]
    fn test_dependency_optimizer() {
        let optimizer = DependencyOptimizer::new();
        
        // Create AST with inlinable includes
        let ast_nodes = vec![
            TemplateNode::Include {
                template_name: "header_snippet.html".to_string(),
                context: None,
                location: None,
            },
            TemplateNode::Include {
                template_name: "inline_nav.html".to_string(),
                context: None,
                location: None,
            },
            TemplateNode::Include {
                template_name: "header_snippet.html".to_string(), // Duplicate
                context: None,
                location: None,
            },
        ];
        
        let mut content = "{% include \"header_snippet.html\" %}{% include \"inline_nav.html\" %}{% include \"header_snippet.html\" %}".to_string();
        let mut ast = TemplateAst { nodes: ast_nodes };
        
        let result = optimizer.optimize(&mut content, &mut ast);
        
        assert!(result.is_ok());
        let optimization_result = result.unwrap();
        
        // Should have optimized templates
        assert!(optimization_result.optimizations_applied > 0);
        
        // Should have found inlinable templates and duplicates
        if !optimization_result.warnings.is_empty() {
            // Some warnings are expected for templates that couldn't be inlined
            println!("Optimization warnings: {:?}", optimization_result.warnings);
        }
    }

    #[test]
    fn test_bundle_creation_with_all_optimizations() {
        let config = BundleConfig {
            enable_compression: true,
            enable_minification: true,
            enable_dead_code_elimination: true,
            enable_dependency_optimization: true,
            optimization_level: OptimizationLevel::Production,
            ..Default::default()
        };
        
        let loader = Arc::new(MockTemplateLoader::new());
        let mut bundler = TemplateBundler::new(config, loader);
        
        let templates = vec!["main.html".to_string()];
        let result = futures::executor::block_on(bundler.create_bundle(&templates, "test_all_optimizations"));
        
        assert!(result.is_ok());
        let bundle = result.unwrap();
        
        // Bundle should include dependencies
        assert!(bundle.entries.len() >= 1);
        
        // Should have optimization statistics
        let stats = &bundle.metadata.optimization_stats;
        assert!(stats.total_optimization_time.as_millis() > 0);
        
        // Should have compression time recorded
        assert!(stats.compression_time.as_millis() >= 0);
        
        // Size info should be reasonable
        let size_info = &bundle.metadata.size_info;
        assert!(size_info.compressed_size <= size_info.minified_size);
        assert!(size_info.minified_size <= size_info.original_size);
        assert!(size_info.template_count > 0);
    }

    #[test]
    fn test_bundle_serialization_and_deserialization() {
        let config = BundleConfig::default();
        let loader = Arc::new(MockTemplateLoader::new());
        let mut bundler = TemplateBundler::new(config, loader);
        
        let templates = vec!["header.html".to_string()];
        let result = futures::executor::block_on(bundler.create_bundle(&templates, "test_serialization"));
        
        assert!(result.is_ok());
        let original_bundle = result.unwrap();
        
        // Serialize bundle
        let serialized_result = bundler.serialize_bundle(&original_bundle);
        assert!(serialized_result.is_ok());
        let serialized_data = serialized_result.unwrap();
        
        // Deserialize bundle
        let deserialized_result = bundler.deserialize_bundle(&serialized_data);
        assert!(deserialized_result.is_ok());
        let deserialized_bundle = deserialized_result.unwrap();
        
        // Bundles should be equivalent
        assert_eq!(original_bundle.metadata.bundle_id, deserialized_bundle.metadata.bundle_id);
        assert_eq!(original_bundle.entries.len(), deserialized_bundle.entries.len());
    }

    #[test]
    fn test_dependency_analysis() {
        let loader = Arc::new(MockTemplateLoader::new());
        let mut analyzer = DependencyAnalyzer::new(loader);
        
        // Create AST with dependencies
        let ast_nodes = vec![
            TemplateNode::Include {
                template_name: "header.html".to_string(),
                context: None,
                location: None,
            },
            TemplateNode::Include {
                template_name: "footer.html".to_string(),
                context: None,
                location: None,
            },
        ];
        
        let ast = TemplateAst { nodes: ast_nodes };
        let result = analyzer.analyze_dependencies("main.html", &ast);
        
        assert!(result.is_ok());
        let deps = result.unwrap();
        
        // Should have found both dependencies
        assert!(deps.contains("header.html"));
        assert!(deps.contains("footer.html"));
        assert_eq!(deps.len(), 2);
        
        // Get all dependencies (recursive)
        let all_deps = analyzer.get_all_dependencies("main.html");
        assert!(all_deps.contains("header.html"));
        assert!(all_deps.contains("footer.html"));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let loader = Arc::new(MockTemplateLoader::new());
        let mut analyzer = DependencyAnalyzer::new(loader);
        
        // Create circular dependency: A -> B -> A
        let ast_a = TemplateAst {
            nodes: vec![
                TemplateNode::Include {
                    template_name: "template_b.html".to_string(),
                    context: None,
                    location: None,
                },
            ],
        };
        
        let ast_b = TemplateAst {
            nodes: vec![
                TemplateNode::Include {
                    template_name: "template_a.html".to_string(),
                    context: None,
                    location: None,
                },
            ],
        };
        
        // Analyze both templates
        let _ = analyzer.analyze_dependencies("template_a.html", &ast_a);
        let _ = analyzer.analyze_dependencies("template_b.html", &ast_b);
        
        // Detect circular dependencies
        let circular_result = analyzer.detect_circular_dependencies();
        assert!(circular_result.is_ok());
        
        let cycles = circular_result.unwrap();
        // Should detect the circular dependency
        assert!(!cycles.is_empty());
    }

    #[test]
    fn test_bundle_cache() {
        let config = BundleConfig::default();
        let loader = Arc::new(MockTemplateLoader::new());
        let mut bundler = TemplateBundler::new(config, loader);
        
        let templates = vec!["header.html".to_string()];
        
        // Create bundle first time
        let result1 = futures::executor::block_on(bundler.create_bundle(&templates, "test_cache"));
        assert!(result1.is_ok());
        
        // Create same bundle again - should come from cache
        let result2 = futures::executor::block_on(bundler.create_bundle(&templates, "test_cache"));
        assert!(result2.is_ok());
        
        let bundle1 = result1.unwrap();
        let bundle2 = result2.unwrap();
        
        // Bundles should be identical (from cache)
        assert_eq!(bundle1.metadata.bundle_id, bundle2.metadata.bundle_id);
        assert_eq!(bundle1.metadata.checksum, bundle2.metadata.checksum);
        
        // Check cache statistics
        let (bundle_count, total_size) = bundler.get_cache_stats();
        assert!(bundle_count > 0);
        assert!(total_size > 0);
        
        // Clear cache
        bundler.clear_cache();
        let (bundle_count_after_clear, _) = bundler.get_cache_stats();
        assert_eq!(bundle_count_after_clear, 0);
    }

    #[test]
    fn test_optimization_levels() {
        let loader = Arc::new(MockTemplateLoader::new());
        
        // Test different optimization levels
        let levels = [
            OptimizationLevel::None,
            OptimizationLevel::Basic,
            OptimizationLevel::Standard,
            OptimizationLevel::Production,
            OptimizationLevel::Aggressive,
        ];
        
        for level in &levels {
            let config = BundleConfig {
                optimization_level: level.clone(),
                ..Default::default()
            };
            
            let mut bundler = TemplateBundler::new(config, Arc::clone(&loader));
            let templates = vec!["header.html".to_string()];
            
            let result = futures::executor::block_on(bundler.create_bundle(&templates, &format!("test_{:?}", level)));
            assert!(result.is_ok(), "Failed to create bundle with optimization level: {:?}", level);
            
            let bundle = result.unwrap();
            assert_eq!(bundle.config.optimization_level, *level);
        }
    }

    #[test]
    fn test_bundle_formats() {
        let loader = Arc::new(MockTemplateLoader::new());
        
        // Test different bundle formats
        let formats = [
            BundleFormat::Raw,
            BundleFormat::Minified,
            BundleFormat::Optimized,
            BundleFormat::Compressed,
            BundleFormat::Precompiled,
        ];
        
        for format in &formats {
            let config = BundleConfig {
                bundle_format: format.clone(),
                ..Default::default()
            };
            
            let mut bundler = TemplateBundler::new(config, Arc::clone(&loader));
            let templates = vec!["header.html".to_string()];
            
            let result = futures::executor::block_on(bundler.create_bundle(&templates, &format!("test_{:?}", format)));
            assert!(result.is_ok(), "Failed to create bundle with format: {:?}", format);
            
            let bundle = result.unwrap();
            assert_eq!(bundle.config.bundle_format, *format);
        }
    }

    #[test]
    fn test_versioning_strategies() {
        let loader = Arc::new(MockTemplateLoader::new());
        
        // Test content hash versioning
        let config = BundleConfig {
            versioning_strategy: VersioningStrategy::ContentHash,
            ..Default::default()
        };
        
        let mut bundler = TemplateBundler::new(config, Arc::clone(&loader));
        let templates = vec!["header.html".to_string()];
        
        let result = futures::executor::block_on(bundler.create_bundle(&templates, "test_versioning"));
        assert!(result.is_ok());
        
        let bundle = result.unwrap();
        assert!(bundle.metadata.version.starts_with("h")); // Content hash starts with 'h'
        
        // Test timestamp versioning
        let config = BundleConfig {
            versioning_strategy: VersioningStrategy::Timestamp,
            ..Default::default()
        };
        
        let mut bundler = TemplateBundler::new(config, Arc::clone(&loader));
        let result = futures::executor::block_on(bundler.create_bundle(&templates, "test_timestamp"));
        assert!(result.is_ok());
        
        let bundle = result.unwrap();
        assert!(bundle.metadata.version.starts_with("t")); // Timestamp starts with 't'
    }
}
