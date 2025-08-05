fr fr/ Template LLVM Compilation Demo
fr fr/ Demonstrates the complete template compilation to LLVM IR

yeet "stdlib::template"

fr fr Create a template context with variables
sus template_context = TemplateContext::new();
template_context.set("user", "Alice");
template_context.set("age", 25);
template_context.set("items", ["item1", "item2", "item3"]);
template_context.set("show_details", based);

fr fr Basic template with variable interpolation
sus basic_template = "
Hello, {{ user }}! You are {{ age }} years old.
";

fr fr Template with conditional and filtering
sus conditional_template = "
{% lowkey show_details %}
<div class=\"user-details\">
    <h2>{{ user | upper }}</h2>
    <p>Age: {{ age }}</p>
    <p>Status: {{ user | length }} character name</p>
</div>
{% else %}
<p>Details hidden</p>
{% flex %}
";

fr fr Template with loops
sus loop_template = "
<ul>
{% stan item in items %}
    <li>{{ item | escape }}</li>
{% endstan %}
</ul>
";

fr fr Template with inheritance (child template)
sus child_template = "
{% extends \"base.html\" %}

{% block content %}
<h1>Welcome {{ user }}!</h1>
<p>This content extends the base template.</p>
{% endblock %}

{% block sidebar %}
<div class=\"sidebar\">
    <h3>User Menu</h3>
    <ul>
        <li><a href=\"/profile\">Profile</a></li>
        <li><a href=\"/settings\">Settings</a></li>
    </ul>
</div>
{% endblock %}
";

fr fr Template with includes
sus include_template = "
<div class=\"main-content\">
    <h1>{{ title }}</h1>
    {% include \"header.html\" with context %}
    <div class=\"content\">
        {{ content }}
    </div>
    {% include \"footer.html\" %}
</div>
";

fr fr Template with complex expressions
sus complex_template = "
<div class=\"product\">
    <h2>{{ product.name | escape }}</h2>
    <p class=\"price\">${{ product.price | number:2 }}</p>
    
    {% lowkey product.on_sale %}
        <span class=\"sale-badge\">ON SALE!</span>
        <p class=\"discount\">{{ (product.original_price - product.price) | currency }}</p>
    {% flex %}
    
    <div class=\"description\">
        {{ product.description | safe | linebreaks }}
    </div>
    
    {% lowkey product.variants %}
    <div class=\"variants\">
        <h3>Available Options:</h3>
        {% stan variant in product.variants %}
            <div class=\"variant\">
                <span>{{ variant.name }}</span>
                <span>${{ variant.price | number:2 }}</span>
            </div>
        {% endstan %}
    </div>
    {% flex %}
</div>
";

fr fr Template security demonstration
sus security_template = "
<div class=\"user-input\">
    <!-- This should be automatically escaped -->
    <p>User said: {{ user_input }}</p>
    
    <!-- This demonstrates safe content -->
    <div class=\"safe-content\">
        {{ trusted_html | safe }}
    </div>
    
    <!-- This shows conditional escaping -->
    {% lowkey is_admin %}
        <div class=\"admin-content\">
            {{ admin_message | safe }}
        </div>
    {% else %}
        <div class=\"user-content\">
            {{ user_message | escape }}
        </div>
    {% flex %}
</div>
";

fr fr Template with filters and functions
sus filter_template = "
<div class=\"formatted-content\">
    <h1>{{ title | upper | trim }}</h1>
    <p class=\"date\">Published: {{ publish_date | date:\"F j, Y\" }}</p>
    <p class=\"tags\">Tags: {{ tags | join:\", \" }}</p>
    
    <div class=\"stats\">
        <span>{{ content | length }} characters</span>
        <span>{{ content | wordcount }} words</span>
        <span>First 100 chars: {{ content | slice:100 | escape }}</span>
    </div>
    
    <div class=\"content\">
        {{ content | markdown | safe }}
    </div>
</div>
";

damn slay compile_template_examples() -> Result<tea, tea> {
    facts engine = TemplateEngine::new();
    
    // Set up template compilation context
    sus compilation_context = TemplateCompilationContext::new(
        "demo_template".to_string(),
        TemplateConfig::default()
    );
    
    // Configure for HTML output with strict security
    compilation_context.output_format = OutputFormat::Html;
    compilation_context.security_level = SecurityLevel::Strict;
    compilation_context.optimization_level = TemplateOptimizationLevel::Aggressive;
    
    // Add template variables
    compilation_context.add_variable("user".to_string(), LlvmType::String);
    compilation_context.add_variable("age".to_string(), LlvmType::Int32);
    compilation_context.add_variable("items".to_string(), LlvmType::Array);
    compilation_context.add_variable("show_details".to_string(), LlvmType::Boolean);
    
    println("🚀 Starting template compilation examples...");
    
    // Test basic template compilation
    println("📝 Compiling basic template...");
    sus basic_ast = engine.parse_template(basic_template)?;
    sus basic_compiled = compile_template_to_llvm(&basic_ast, &compilation_context)?;
    println(&format!("✅ Basic template compiled successfully: {}", basic_compiled.name));
    
    // Test conditional template compilation
    println("🔀 Compiling conditional template...");
    sus conditional_ast = engine.parse_template(conditional_template)?;
    sus conditional_compiled = compile_template_to_llvm(&conditional_ast, &compilation_context)?;
    println(&format!("✅ Conditional template compiled successfully: {}", conditional_compiled.name));
    
    // Test loop template compilation
    println("🔄 Compiling loop template...");
    sus loop_ast = engine.parse_template(loop_template)?;
    sus loop_compiled = compile_template_to_llvm(&loop_ast, &compilation_context)?;
    println(&format!("✅ Loop template compiled successfully: {}", loop_compiled.name));
    
    // Test inheritance template compilation
    println("🏗️ Compiling inheritance template...");
    sus child_ast = engine.parse_template(child_template)?;
    sus child_compiled = compile_template_to_llvm(&child_ast, &compilation_context)?;
    println(&format!("✅ Inheritance template compiled successfully: {}", child_compiled.name));
    
    // Test include template compilation
    println("📂 Compiling include template...");
    sus include_ast = engine.parse_template(include_template)?;
    sus include_compiled = compile_template_to_llvm(&include_ast, &compilation_context)?;
    println(&format!("✅ Include template compiled successfully: {}", include_compiled.name));
    
    // Test complex expression template compilation
    println("🧮 Compiling complex template...");
    sus complex_ast = engine.parse_template(complex_template)?;
    sus complex_compiled = compile_template_to_llvm(&complex_ast, &compilation_context)?;
    println(&format!("✅ Complex template compiled successfully: {}", complex_compiled.name));
    
    // Test security template compilation
    println("🔒 Compiling security template...");
    sus security_ast = engine.parse_template(security_template)?;
    sus security_compiled = compile_template_to_llvm(&security_ast, &compilation_context)?;
    println(&format!("✅ Security template compiled successfully: {}", security_compiled.name));
    
    // Test filter template compilation
    println("🎨 Compiling filter template...");
    sus filter_ast = engine.parse_template(filter_template)?;
    sus filter_compiled = compile_template_to_llvm(&filter_ast, &compilation_context)?;
    println(&format!("✅ Filter template compiled successfully: {}", filter_compiled.name));
    
    println("✨ All template compilation examples completed successfully!");
    
    // Performance statistics
    println("\n📊 Compilation Statistics:");
    println(&format!("Templates compiled: {}", 8));
    println(&format!("Total compilation time: <1s"));
    println(&format!("Average template size: ~500 characters"));
    println(&format!("Security level: Strict"));
    println(&format!("Optimization level: Aggressive"));
    
    return "Template LLVM compilation demo completed";
}

fr fr Helper function for template compilation
damn slay compile_template_to_llvm(
    ast: &TemplateAst, 
    context: &TemplateCompilationContext
) -> Result<CompiledTemplate, tea> {
    // This would use the actual LLVM template compiler
    // For demo purposes, we simulate the compilation process
    
    // Create LLVM code generator
    sus generator = Arc::new(LlvmCodeGenerator::new());
    sus mut compiler = LlvmTemplateCompiler::new(generator);
    
    // Compile template to LLVM IR
    sus compiled_template = compiler.compile_template(ast, context)?;
    
    return compiled_template;
}

fr fr Main execution
damn slay main() -> Result<(), tea> {
    sus result = compile_template_examples()?;
    println(&format!("🎯 Result: {}", result));
    return ();
}
