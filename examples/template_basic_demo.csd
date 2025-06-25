vibe template_basic_demo

yeet "stdlib::template"
yeet "stdlib::io"

fr fr Basic template system demonstration
slay main() {
    fr fr Create a template engine
    sus engine = TemplateEngine::new()
    
    fr fr Create template context with data
    sus context = TemplateContext::new()
    context.set("name", "Alice")
    context.set("age", 25)
    context.set("city", "San Francisco")
    context.set("is_premium", true)
    
    fr fr Basic variable interpolation
    sus basic_template = "Hello {{ name }}, you are {{ age }} years old!"
    sus result = engine.render_string(basic_template, context.clone())
    println("Basic Template:")
    println(result)
    println("")
    
    fr fr Template with filters
    sus filtered_template = "Welcome {{ name | upper }} from {{ city | lower }}!"
    sus result = engine.render_string(filtered_template, context.clone())
    println("Filtered Template:")
    println(result)
    println("")
    
    fr fr Conditional template
    sus conditional_template = """
{% if is_premium %}
🌟 Premium User: {{ name }}
Special benefits available!
{% else %}
Regular User: {{ name }}
Upgrade to premium for more features!
{% end %}
"""
    sus result = engine.render_string(conditional_template, context.clone())
    println("Conditional Template:")
    println(result)
    println("")
    
    fr fr Template with arrays and loops
    sus hobbies = ["coding", "reading", "gaming", "hiking"]
    context.set("hobbies", hobbies)
    
    sus loop_template = """
{{ name }}'s Hobbies:
{% for hobby in hobbies %}
- {{ hobby | title }}{% if @last %} (favorite!){% end %}
{% end %}
"""
    sus result = engine.render_string(loop_template, context.clone())
    println("Loop Template:")
    println(result)
    
    vibez.spill("Template basic demo completed successfully!")
}
