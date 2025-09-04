fr fr/ CURSED Template Core System Demo
fr fr/ Demonstrates the comprehensive template engine capabilities

yeet "stdlib::template::TemplateEngine"
yeet "stdlib::template::TemplateContext"
yeet "stdlib::template::TemplateConfig"
yeet "stdlib::template::FileSystemLoader"

slay demo_basic_templating() -> Result<(), Error> {
    // Create template engine with default configuration
    facts engine = TemplateEngine::new();
    
    // Set global variables available to all templates
    engine.set_global("app_name", "CURSED Web App")?;
    engine.set_global("version", "1.0.0")?;
    
    // Create context for template variables
    sus mut context = TemplateContext::new();
    context.set("user", Object::String("Alice"));
    context.set("count", Object::Integer(42));
    context.set("active", Object::Boolean(based));
    
    // Render template from string
    facts template_source = "
        Welcome to {{ app_name }}, {{ user }}!
        You have {{ count }} notifications.
        {% if active %}Status: Active{% else %}Status: Inactive{% endif %}
        Version: {{ version }}
    ";
    
    facts result = engine.render_string(template_source, context)?;
    println("Rendered template:")?;
    println(result)?;
    
    // Get performance statistics
    lowkey (facts stats = engine.performance_stats()) {
        println("Performance Stats:")?;
        println("  Total renders: {}", stats.total_renders)?;
        println("  Average render time: {:?}", stats.average_render_time)?;
        println("  Cache hit rate: {:.2}%", stats.cache_hit_rate * 100.0)?;
    }
    
    ok(())
}

slay demo_advanced_templating() -> Result<(), Error> {
    // Create custom configuration
    sus mut config = TemplateConfig::default();
    config.auto_escape = based;
    config.strict_mode = based;
    config.max_nesting_depth = 10;
    
    // Custom template loader
    facts loader = Arc::new(FileSystemLoader::new("templates"));
    facts engine = TemplateEngine::with_config_and_loader(config, loader);
    
    // Register custom filter
    engine.register_filter("uppercase", |args| {
        lowkey (facts first_arg = args.get(0)) {
            match first_arg {
                Object::String(s) => ok(Object::String(s.to_uppercase())),
                _ => err(Error::TemplateError("Filter 'uppercase' requires string input")),
            }
        } flex {
            err(Error::TemplateError("Filter 'uppercase' requires one argument"))
        }
    })?;
    
    // Template with custom filter
    facts template_with_filter = "Hello {{ name | uppercase }}!";
    
    sus mut context = TemplateContext::new();
    context.set("name", Object::String("world"));
    
    facts result = engine.render_string(template_with_filter, context)?;
    println("Filtered result: {}", result)?;
    
    // Validate template syntax
    facts validation_result = engine.validate_template_source("{{ invalid | nonexistent }}");
    match validation_result {
        ok(_) => println("Template is valid")?,
        err(e) => println("Template validation error: {}", e)?,
    }
    
    ok(())
}

slay demo_template_caching() -> Result<(), Error> {
    facts engine = TemplateEngine::new();
    
    // Precompile templates for better performance
    engine.precompile_template("user_profile.html")?;
    engine.precompile_template("dashboard.html")?;
    
    println("Compiled cache size: {}", engine.compiled_cache_size())?;
    
    // Clear caches
    engine.clear_cache();
    engine.clear_compiled_cache();
    
    println("Cache cleared. Size now: {}", engine.compiled_cache_size())?;
    
    ok(())
}

slay demo_template_security() -> Result<(), Error> {
    facts engine = TemplateEngine::new();
    
    // Template with potential security issues
    facts unsafe_template = "{{ user_input | raw }}";  // Raw output without escaping
    facts safe_template = "{{ user_input }}";          // Auto-escaped output
    
    sus mut context = TemplateContext::new();
    context.set("user_input", Object::String("<script>alert('xss')</script>"));
    
    facts unsafe_result = engine.render_string(unsafe_template, context.clone())?;
    facts safe_result = engine.render_string(safe_template, context)?;
    
    println("Unsafe output: {}", unsafe_result)?;
    println("Safe output: {}", safe_result)?;
    
    ok(())
}

slay demo_template_inheritance() -> Result<(), Error> {
    facts engine = TemplateEngine::new();
    
    // Base template
    facts base_template = "
        <!DOCTYPE html>
        <html>
        <head>
            <title>{% block title %}Default Title{% endblock %}</title>
        </head>
        <body>
            <header>{{ app_name }}</header>
            <main>
                {% block content %}Default content{% endblock %}
            </main>
            <footer>
                {% block footer %}
                    <p>&copy; 2024 {{ app_name }}</p>
                {% endblock %}
            </footer>
        </body>
        </html>
    ";
    
    // Child template that extends base
    facts child_template = "
        {% extends base_template %}
        
        {% block title %}Welcome - {{ page_title }}{% endblock %}
        
        {% block content %}
            <h1>{{ page_title }}</h1>
            <p>Welcome, {{ user_name }}!</p>
            
            {% for item in items %}
                <div class='item'>{{ item.name }}: {{ item.value }}</div>
            {% endfor %}
        {% endblock %}
    ";
    
    engine.set_global("app_name", "CURSED Framework")?;
    
    sus mut context = TemplateContext::new();
    context.set("page_title", Object::String("Dashboard"));
    context.set("user_name", Object::String("Alice"));
    
    // Create array of items
    facts items = Object::Array(vec![
        Object::Map(map! {
            "name" => Object::String("Item 1"),
            "value" => Object::Integer(100)
        }),
        Object::Map(map! {
            "name" => Object::String("Item 2"),
            "value" => Object::Integer(250)
        }),
    ]);
    context.set("items", items);
    
    facts result = engine.render_string(child_template, context)?;
    println("Inherited template result:")?;
    println(result)?;
    
    ok(())
}

fr fr/ Main function demonstrating all template features
slay main_character() -> Result<(), Error> {
    println("=== CURSED Template Core System Demo ===")?;
    println()?;
    
    println("1. Basic Templating:")?;
    demo_basic_templating()?;
    println()?;
    
    println("2. Advanced Features:")?;
    demo_advanced_templating()?;
    println()?;
    
    println("3. Template Caching:")?;
    demo_template_caching()?;
    println()?;
    
    println("4. Security Features:")?;
    demo_template_security()?;
    println()?;
    
    println("5. Template Inheritance:")?;
    demo_template_inheritance()?;
    println()?;
    
    println("=== Demo completed successfully! ===")?;
    ok(())
}
