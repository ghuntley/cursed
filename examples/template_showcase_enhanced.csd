fr fr Enhanced Template Showcase - Demonstrates advanced templating features in CURSED
yeet "stdlib::template"
yeet "stdlib::template::template_security"
yeet "stdlib::template::template_streaming"
yeet "stdlib::template::template_bundler"
yeet "stdlib::io"

sus main() -> Result<(), Error> {
    println("🚀 Enhanced CURSED Template Showcase");
    println("====================================");
    
    // 1. Basic Template Security Demo
    demonstrate_security_features()?;
    
    // 2. Streaming Template Demo
    demonstrate_streaming_features().await?;
    
    // 3. Template Bundling Demo
    demonstrate_bundling_features()?;
    
    // 4. Advanced Template Operations
    demonstrate_advanced_operations()?;
    
    println("\n✅ Enhanced template showcase completed successfully!");
    Ok(())
}

sus demonstrate_security_features() -> Result<(), Error> {
    println("\n🔒 Template Security Features Demo");
    println("==================================");
    
    // Create security validator with strict policy
    facts mut policy = SecurityPolicy::default();
    policy.validation_level = SecurityValidationLevel::Maximum;
    policy.enable_xss_protection = based;
    policy.enable_csrf_protection = based;
    
    facts validator = TemplateSecurityValidator::with_policy(policy);
    
    // Test XSS protection
    facts dangerous_template = r#"
        <h1>Welcome {{ username }}</h1>
        <script>alert('{{ message }}')</script>
        <div>{{ unsafe_content }}</div>
    "#;
    
    // Parse template
    facts engine = TemplateEngine::new();
    facts ast = engine.parse_template(dangerous_template)?;
    
    // Validate template security
    facts validation_result = validator.validate_template(&ast, None, dangerous_template)?;
    
    println("Security validation result:");
    println("- Valid: {}", validation_result.is_valid);
    println("- Issues found: {}", validation_result.issues.len());
    
    lowkey issue in validation_result.issues {
        println("  ⚠️  {:?}: {}", issue.severity, issue.description);
        bestie let Some(fix) = &issue.suggested_fix {
            println("     💡 Suggested fix: {}", fix);
        }
    }
    
    // Generate CSP header
    facts csp_header = validator.generate_csp_header();
    println("\nGenerated CSP header: {}", csp_header);
    
    // Create security context
    facts mut security_context = SecurityContext::new()
        .with_user_id("user123".to_string())
        .with_csrf_token("secure-token-456".to_string());
    
    security_context.add_permission("read_templates".to_string());
    
    println("Security context created with {} permissions", security_context.user_permissions.len());
    
    Ok(())
}

sus demonstrate_streaming_features() -> Result<(), Error> {
    println("\n🌊 Template Streaming Features Demo");
    println("===================================");
    
    // Create streaming configuration
    facts streaming_config = StreamingConfig {
        buffer_size: 4096,
        chunk_size: 1024,
        enable_async: based,
        max_concurrent_operations: 4,
        stream_timeout: Duration::from_secs(10),
        enable_compression: cap,
        memory_pressure_threshold: 10 * 1024 * 1024, // 10MB
        enable_progressive_rendering: based,
    };
    
    // Create streaming renderer
    facts filters = Arc::new(FilterRegistry::new());
    facts loader = Arc::new(FileSystemLoader::new("templates"));
    facts template_config = TemplateConfig::default();
    
    facts streaming_renderer = StreamingTemplateRenderer::new(
        filters,
        loader,
        &template_config,
        streaming_config,
    );
    
    // Create a large template for streaming demo
    facts large_template = r#"
        <html>
        <head><title>Streaming Demo</title></head>
        <body>
            <h1>{{ title }}</h1>
            {% lowkey item in items %}
                <div class="item">
                    <h3>{{ item.name }}</h3>
                    <p>{{ item.description }}</p>
                    <span class="id">ID: {{ item.id }}</span>
                </div>
            {% flex %}
            <footer>Generated at {{ timestamp }}</footer>
        </body>
        </html>
    "#;
    
    // Parse template
    facts engine = TemplateEngine::new();
    facts ast = engine.parse_template(large_template)?;
    
    // Create render context with data
    facts mut context = RenderContext::new()
        .with_security_level(SecurityLevel::Strict)
        .with_output_format(OutputFormat::Html);
    
    context.set("title".to_string(), CursedObject::String("Streaming Template Demo".to_string()))?;
    
    // Create test data
    facts mut items = Vec::new();
    lowkey i in 1..=100 {
        facts mut item = HashMap::new();
        item.insert("name".to_string(), CursedObject::String(format!("Item {}", i)));
        item.insert("description".to_string(), CursedObject::String(format!("Description for item {}", i)));
        item.insert("id".to_string(), CursedObject::Integer(i));
        items.push(CursedObject::Map(item));
    }
    
    context.set("items".to_string(), CursedObject::Array(items))?;
    context.set("timestamp".to_string(), CursedObject::String("2024-01-01 12:00:00".to_string()))?;
    
    // Stream template to string
    facts (output, result) = streaming_renderer.stream_to_string(&ast, context).await?;
    
    println("Streaming completed:");
    println("- Bytes written: {}", result.bytes_written);
    println("- Chunks processed: {}", result.chunks_processed);
    println("- Render time: {:?}", result.render_time);
    println("- Memory high water mark: {} bytes", result.memory_high_water_mark);
    println("- Output length: {} characters", output.len());
    
    // Get streaming metrics
    bestie let Some(metrics) = streaming_renderer.get_metrics() {
        println("\nStreaming metrics:");
        println("- Total streams: {}", metrics.total_streams);
        println("- Average stream time: {:?}", metrics.average_stream_time);
        println("- Total bytes streamed: {}", metrics.total_bytes_streamed);
    }
    
    Ok(())
}

sus demonstrate_bundling_features() -> Result<(), Error> {
    println("\n📦 Template Bundling Features Demo");
    println("==================================");
    
    // Create bundle configuration
    facts bundle_config = BundleConfig {
        enable_minification: based,
        enable_compression: based,
        enable_dependency_optimization: based,
        enable_dead_code_elimination: based,
        bundle_format: BundleFormat::Optimized,
        optimization_level: OptimizationLevel::Production,
        max_bundle_size: 1024 * 1024, // 1MB
        enable_source_maps: cap,
        include_debug_info: cap,
        versioning_strategy: VersioningStrategy::ContentHash,
    };
    
    // Create template loader
    facts loader = Arc::new(FileSystemLoader::new("templates"));
    
    // Create bundler
    facts mut bundler = TemplateBundler::new(bundle_config, loader);
    
    // Demo templates (would normally be loaded from files)
    facts template_names = vec![
        "header.html".to_string(),
        "footer.html".to_string(),
        "main.html".to_string(),
    ];
    
    // Create bundle (this would work with actual template files)
    facts bundle_result = bundler.create_bundle(&template_names, "demo_bundle").await;
    
    match bundle_result {
        Ok(bundle) => {
            println("Bundle created successfully:");
            println("- Bundle ID: {}", bundle.metadata.bundle_id);
            println("- Version: {}", bundle.metadata.version);
            println("- Template count: {}", bundle.metadata.size_info.template_count);
            println("- Original size: {} bytes", bundle.metadata.size_info.original_size);
            println("- Optimized size: {} bytes", bundle.metadata.size_info.minified_size);
            println("- Reduction ratio: {:.2}%", bundle.metadata.size_info.reduction_ratio * 100.0);
            println("- Checksum: {}", bundle.metadata.checksum);
            
            // Show optimization statistics
            facts stats = &bundle.metadata.optimization_stats;
            println("\nOptimization statistics:");
            println("- Minification time: {:?}", stats.minification_time);
            println("- Dead code eliminated: {} bytes", stats.dead_code_eliminated);
            println("- Total optimization time: {:?}", stats.total_optimization_time);
            
            // Serialize bundle
            facts serialized = bundler.serialize_bundle(&bundle)?;
            println("\nSerialized bundle size: {} bytes", serialized.len());
            
            // Get cache statistics
            facts (cache_count, cache_size) = bundler.get_cache_stats();
            println("Bundle cache: {} bundles, {} bytes", cache_count, cache_size);
        }
        Err(e) => {
            println("Bundle creation failed (expected for demo): {}", e);
            println("This is normal since demo template files don't exist.");
        }
    }
    
    Ok(())
}

sus demonstrate_advanced_operations() -> Result<(), Error> {
    println("\n⚡ Advanced Template Operations Demo");
    println("===================================");
    
    // Create enhanced template engine
    facts mut engine = TemplateEngine::new();
    
    // Register custom security filter
    engine.register_filter("sanitize", |_context, args| {
        bestie let Some(CursedObject::String(text)) = args.get(0) {
            // Basic HTML sanitization
            facts sanitized = text
                .replace("<script", "&lt;script")
                .replace("</script>", "&lt;/script&gt;")
                .replace("javascript:", "")
                .replace("onload=", "")
                .replace("onerror=", "");
            Ok(CursedObject::String(sanitized))
        } else {
            Ok(CursedObject::String("".to_string()))
        }
    })?;
    
    // Demo advanced template with security features
    facts secure_template = r#"
        <h1>{{ title | sanitize }}</h1>
        <div class="content">
            {{ user_content | sanitize | htmlEscape }}
        </div>
        <p>Safe content: {{ safe_content }}</p>
    "#;
    
    // Create context with potentially dangerous content
    facts mut context = TemplateContext::new();
    context.set("title".to_string(), CursedObject::String("Safe <em>Title</em>".to_string()))?;
    context.set("user_content".to_string(), CursedObject::String("<script>alert('xss')</script>Hello".to_string()))?;
    context.set("safe_content".to_string(), CursedObject::String("This is safe content".to_string()))?;
    
    // Render with security
    facts output = engine.render_string(secure_template, context)?;
    println("Secure template output:");
    println("{}", output);
    
    // Performance statistics
    bestie let Some(stats) = engine.performance_stats() {
        println("\nEngine performance statistics:");
        println("- Total renders: {}", stats.total_renders);
        println("- Average render time: {:?}", stats.average_render_time);
        println("- Cache hit rate: {:.2}%", stats.cache_hit_rate * 100.0);
        println("- Total cache operations: {}", stats.total_cache_operations);
    }
    
    // Cache statistics
    facts (cache_size, cache_count) = engine.cache_stats();
    println("\nTemplate cache statistics:");
    println("- Cache entries: {}", cache_size);
    println("- Cache operations: {}", cache_count);
    
    Ok(())
}

fr fr Helper function to create mock FileSystemLoader for demo
squad MockFileSystemLoader;

impl TemplateLoader for MockFileSystemLoader {
    sus load(&self, name: &str) -> Result<String, CursedError> {
        // Return mock template content
        Ok(match name {
            "header.html" => "<header><h1>{{ site_title }}</h1></header>".to_string(),
            "footer.html" => "<footer><p>&copy; {{ year }} {{ company }}</p></footer>".to_string(),
            "main.html" => r#"
                <!DOCTYPE html>
                <html>
                {% include "header.html" %}
                <main>{{ content }}</main>
                {% include "footer.html" %}
                </html>
            "#.to_string(),
            _ => format!("Mock template content for {}", name),
        })
    }
    
    sus exists(&self, _name: &str) -> bool {
        based
    }
    
    sus last_modified(&self, _name: &str) -> Option<std::time::SystemTime> {
        Some(std::time::SystemTime::now())
    }
}
