fr fr Template Context Variable Management Demo
fr fr This example demonstrates the enhanced context variable management system

yeet "stdlib::template"

fr fr Demonstrate basic context operations
facts demo_basic_context() {
    sus engine = template::engine::new();
    sus context = template::context::new();
    
    // Set some variables in the context
    context.set("user_name", "Alice");
    context.set("user_age", 25);
    context.set("is_admin", facts);
    
    // Render a simple template
    sus template_source = "Hello {{user_name}}! You are {{user_age}} years old. Admin: {{is_admin}}";
    sus result = engine.render_string(template_source, context);
    
    println("Basic Context Demo:");
    println(result);
}

fr fr Demonstrate loop context scoping
facts demo_loop_scoping() {
    sus engine = template::engine::new();
    sus context = template::context::new();
    
    // Set up data for loop
    sus items = ["Apple", "Banana", "Cherry"];
    context.set("fruits", items);
    
    sus template_source = "
    <ul>
    {% stan fruit in fruits %}
        <li>{{loop.index1}}. {{fruit}} (index: {{loop.index}})</li>
    {% /stan %}
    </ul>
    ";
    
    sus result = engine.render_string(template_source, context);
    
    println("Loop Scoping Demo:");
    println(result);
}

fr fr Demonstrate include context merging
facts demo_include_context() {
    sus engine = template::engine::new();
    sus context = template::context::new();
    
    // Set base context variables
    context.set("site_name", "My Website");
    context.set("base_url", "https://example.com");
    
    // Template with include (simulated)
    sus template_source = "
    <h1>Welcome to {{site_name}}</h1>
    <p>Base URL: {{base_url}}</p>
    
    <!-- Include with additional context -->
    {% include 'user_info.html' with username='Bob', role='admin' %}
    ";
    
    // In real implementation, this would load and render the included template
    // with merged context
    println("Include Context Demo:");
    println("Template would include user_info.html with merged context:");
    println("  - site_name: My Website (from base)");
    println("  - base_url: https://example.com (from base)");
    println("  - username: Bob (from include)");
    println("  - role: admin (from include)");
}

fr fr Demonstrate variable shadowing
facts demo_variable_shadowing() {
    sus engine = template::engine::new();
    sus parent_context = template::context::new();
    
    // Set variables in parent context
    parent_context.set("theme", "dark");
    parent_context.set("layout", "grid");
    parent_context.set("global_var", "parent_value");
    
    // Create child context that shadows some variables
    sus child_context = template::context::with_parent(parent_context);
    child_context.set("theme", "light");  // Shadow parent variable
    child_context.set("local_var", "child_value");  // Child-only variable
    
    sus template_source = "
    Theme: {{theme}}
    Layout: {{layout}}
    Global: {{global_var}}
    Local: {{local_var}}
    ";
    
    sus result = engine.render_string(template_source, child_context);
    
    println("Variable Shadowing Demo:");
    println(result);
}

fr fr Demonstrate context isolation levels
facts demo_isolation_levels() {
    println("Context Isolation Levels Demo:");
    
    // Strict isolation - variables are read-only from parent
    sus strict_context = template::context::new_with_isolation("strict");
    strict_context.set("strict_var", "value");
    
    // Local isolation - variables can be updated in current context only
    sus local_context = template::context::new_with_isolation("local");
    local_context.set("local_var", "value");
    
    // None isolation - variables can be updated across contexts
    sus none_context = template::context::new_with_isolation("none");
    none_context.set("shared_var", "value");
    
    println("  - Strict: Variables isolated to current context");
    println("  - Local: Variables can shadow parent context");
    println("  - None: Variables can be updated across context chain");
}

fr fr Demonstrate thread-safe context operations
facts demo_thread_safety() {
    println("Thread Safety Demo:");
    
    sus context = template::context::new();
    context.set("shared_counter", 0);
    
    // In a real implementation, multiple goroutines could safely
    // access and modify the context simultaneously
    println("  - Context operations are thread-safe using RwLock");
    println("  - Multiple templates can render concurrently");
    println("  - Variable updates are atomic and consistent");
}

fr fr Main demo function
facts main() {
    println("=== CURSED Template Context Variable Management Demo ===");
    println();
    
    demo_basic_context();
    println();
    
    demo_loop_scoping();
    println();
    
    demo_include_context();
    println();
    
    demo_variable_shadowing();
    println();
    
    demo_isolation_levels();
    println();
    
    demo_thread_safety();
    println();
    
    println("=== Demo Complete ===");
}
