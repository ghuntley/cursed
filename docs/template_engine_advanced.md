# CURSED Advanced Template Engine

The CURSED Advanced Template Engine provides modern web template functionality with inheritance, caching, security, and performance optimization features.

## Features

### 🚀 Core Features
- **Template Inheritance**: Extend base templates with block overrides
- **Component System**: Reusable HTML components with props
- **Template Compilation**: Pre-compiled templates for performance
- **Intelligent Caching**: Automatic template caching with invalidation
- **Expression Evaluation**: Complex expressions with security validation
- **Form Generation**: Automatic HTML form generation with validation

### 🔒 Security Features
- **XSS Protection**: Automatic HTML escaping and sanitization
- **Safe Evaluation**: Sandboxed expression evaluation
- **Input Validation**: Template security validation
- **CSRF Protection**: Built-in CSRF token generation
- **Content Security**: Allowed tags and attributes filtering

### ⚡ Performance Features
- **Template Compilation**: Pre-compiled instructions for fast execution
- **Smart Caching**: Template-level caching with hash-based invalidation  
- **Memory Efficiency**: Arena allocators and memory pooling
- **Optimization Pipeline**: Whitespace compression and asset inlining

## Quick Start

```cursed
yeet "template_engine/advanced"

slay main() {
    // Create advanced template engine
    sus engine AdvancedTemplateEngine = create_advanced_template_engine()
    
    // Set template variables
    engine = set_variable_scoped(engine, "title", "My App")
    engine = set_variable_scoped(engine, "user", "Alice")
    
    // Process template
    sus template tea = "Welcome to {{$title}}, {{$user}}!"
    sus result TemplateResult = process_compiled_template(engine, template)
    
    print(result.output)  // "Welcome to My App, Alice!"
}
```

## Template Syntax

### Variables
```html
<!-- Basic variable -->
{{$name}}

<!-- Nested properties -->
{{$user.name}}

<!-- With default values -->
{{default($optional_var, "fallback")}}
```

### Control Flow
```html
<!-- Conditionals -->
{{if $logged_in}}
    Welcome back, {{$username}}!
{{elif $guest_mode}}
    Welcome, guest!
{{else}}
    Please log in.
{{/if}}

<!-- Loops -->
{{for item in $items}}
    <li>{{$item}}</li>
{{/for}}

<!-- Complex conditions -->
{{if $user_role == "admin" && $logged_in}}
    <a href="/admin">Admin Panel</a>
{{/if}}
```

### Functions
```html
<!-- String functions -->
{{upper($text)}}
{{lower($text)}}
{{trim($text)}}
{{truncate($text, 50)}}

<!-- Date functions -->
{{now()}}
{{format_date($date, "YYYY-MM-DD")}}

<!-- Security functions -->
{{sanitize_html($user_content)}}
{{csrf_token()}}

<!-- URL functions -->
{{url_for("home")}}
{{asset_url("style.css")}}
```

## Template Inheritance

### Base Template (layout.html)
```html
<!DOCTYPE html>
<html>
<head>
    <title>{{block:title}}Default Title{{/block:title}}</title>
    {{block:head}}{{/block:head}}
</head>
<body>
    <header>{{block:header}}Default Header{{/block:header}}</header>
    <main>{{block:content}}Default Content{{/block:content}}</main>
    <footer>{{block:footer}}© 2024{{/block:footer}}</footer>
</body>
</html>
```

### Child Template
```html
{{extends "layout.html"}}

{{block:title}}My Blog{{/block:title}}

{{block:head}}
<link rel="stylesheet" href="/css/blog.css">
{{/block:head}}

{{block:header}}
<h1>My Personal Blog</h1>
<nav>
    <a href="/">Home</a>
    <a href="/about">About</a>
</nav>
{{/block:header}}

{{block:content}}
<article>
    <h2>{{$post.title}}</h2>
    <p>{{$post.content}}</p>
</article>
{{/block:content}}
```

## Web Components

### Creating Components
```cursed
// Button component
sus button HTMLComponent = create_button_component(
    "Submit",           // text
    "submit",          // type
    "submitForm()"     // onclick
)

// Card component
sus card HTMLComponent = create_card_component(
    "Product Name",                    // title
    "<p>Product description</p>",      // content
    "/images/product.jpg"              // image URL
)

// Navigation component
sus nav_items [tea] = ["home", "about", "contact"]
sus nav HTMLComponent = create_navigation_component(nav_items, "home")
```

### Rendering Components
```cursed
sus engine AdvancedTemplateEngine = create_web_template_engine()
sus html tea = render_component(button, engine)
print(html)
```

## Form Generation

### Creating Forms
```cursed
// Create form
sus contact_form WebForm = create_web_form("contact", "/submit", "POST")

// Add fields
sus name_field FormField = create_text_field("name", "Full Name", based)
sus email_field FormField = create_email_field("email", "Email", based)

sus options [tea] = ["General", "Support", "Sales"]
sus subject_field FormField = create_select_field("subject", "Subject", options, based)

// Add to form
contact_form = add_form_field(contact_form, name_field)
contact_form = add_form_field(contact_form, email_field)
contact_form = add_form_field(contact_form, subject_field)

// Render form
sus html tea = render_web_form(contact_form, engine)
```

### Generated HTML
```html
<form method="POST" action="/submit" class="web-form">
    <input type="hidden" name="csrf_token" value="csrf_1234567890">
    
    <div class="form-group">
        <label for="name">Full Name <span class="required">*</span></label>
        <input type="text" name="name" id="name" class="form-control" required>
    </div>
    
    <div class="form-group">
        <label for="email">Email <span class="required">*</span></label>
        <input type="email" name="email" id="email" placeholder="Enter your email address" class="form-control" autocomplete="email" required>
    </div>
    
    <div class="form-group">
        <label for="subject">Subject <span class="required">*</span></label>
        <select name="subject" id="subject" class="form-control" required>
            <option value="">Select an option</option>
            <option value="General">General</option>
            <option value="Support">Support</option>
            <option value="Sales">Sales</option>
        </select>
    </div>
    
    <div class="form-actions">
        <button type="submit" class="btn btn-primary">Submit</button>
    </div>
</form>
```

## Security Features

### XSS Protection
```cursed
// Automatic HTML escaping
engine.escape_html = based

engine = set_variable_scoped(engine, "user_input", "<script>alert('xss')</script>")
sus template tea = "Input: {{$user_input}}"
sus result TemplateResult = process_compiled_template(engine, template)
// Output: "Input: &lt;script&gt;alert('xss')&lt;/script&gt;"
```

### Template Validation
```cursed
sus security_context SecurityContext = SecurityContext{
    xss_protection: based,
    csrf_protection: based,
    allowed_tags: {"p": based, "div": based},
    allowed_attributes: {"class": based, "id": based},
    max_output_size: 10000
}

sus dangerous_template tea = "<script>evil()</script>"
sus is_safe lit = validate_template_security(dangerous_template, security_context)
// Returns: false
```

### Safe Expression Evaluation
```cursed
// Enable sandbox mode
engine.sandbox_mode = based
engine.max_iterations = 1000
engine.max_depth = 10

// Expressions are validated before execution
sus safe_expr tea = "$user.name + ' - ' + format_date(now())"
sus result tea = evaluate_expression_with_security(engine, safe_expr, context)
```

## Template Compilation & Caching

### Compilation Process
```cursed
// Templates are automatically compiled to instructions
sus compiled CompiledTemplate = compile_template(engine, template)

// Compilation creates optimized instruction set
print("Instructions: " + string(len(compiled.instructions)))
print("Variables: " + string(len(compiled.variables)))
print("Functions: " + string(len(compiled.functions)))
```

### Caching System
```cursed
// Templates are automatically cached by hash
sus template tea = "Hello {{$name}}!"

// First render - cache miss
sus result1 TemplateResult = process_compiled_template(engine, template)
print("Cache misses: " + string(engine.cache.misses))  // 1

// Second render - cache hit
sus result2 TemplateResult = process_compiled_template(engine, template)
print("Cache hits: " + string(engine.cache.hits))      // 1

// Cache statistics
sus hit_ratio normie = (engine.cache.hits * 100) / (engine.cache.hits + engine.cache.misses)
print("Hit ratio: " + string(hit_ratio) + "%")
```

## Performance Optimization

### Template Optimization
```cursed
// Optimize template for production
sus optimized tea = optimize_template_for_web(template)

// Features:
// - Whitespace compression
// - Critical CSS inlining  
// - Script deferring
// - Asset optimization
```

### Memory Efficiency
```cursed
// Templates use arena allocators for memory efficiency
// Automatic memory cleanup after template processing
// Compiled templates are cached and reused
// Variable scoping prevents memory leaks
```

### Concurrent Processing
```cursed
// Template engine supports concurrent processing
bestie i := 0; i < 10; i++ {
    go {
        sus result TemplateResult = process_compiled_template(engine, template)
        // Process result
    }
}
```

## Web Layout System

### Creating Layouts
```cursed
// Create web layout
sus layout WebLayout = create_web_layout("app_layout")

// Configure SEO
layout.seo_data.title = "My Web App"
layout.seo_data.description = "Built with CURSED"
layout.seo_data.keywords = ["cursed", "web", "app"]

// Add assets
layout.stylesheets = ["/css/app.css", "/css/bootstrap.css"]
layout.scripts = ["/js/app.js"]

// Render with content
sus content tea = "<h1>Welcome</h1><p>This is my app content.</p>"
sus html tea = render_web_layout(layout, content, engine)
```

### Generated Layout
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="Built with CURSED">
    <meta name="keywords" content="cursed,web,app">
    <meta property="og:title" content="My Web App">
    <meta property="og:description" content="Built with CURSED">
    <meta property="og:type" content="website">
    <title>My Web App</title>
    <link rel="stylesheet" href="/css/app.css">
    <link rel="stylesheet" href="/css/bootstrap.css">
</head>
<body>
    <h1>Welcome</h1>
    <p>This is my app content.</p>
    <script src="/js/app.js"></script>
</body>
</html>
```

## Advanced Features

### Expression Parser
```cursed
// Complex expressions with proper precedence
{{$a + $b * $c}}               // Multiplication first
{{($a + $b) * $c}}             // Parentheses override
{{$user_role == "admin" && $logged_in}}  // Boolean logic
```

### Custom Functions
```cursed
// Register custom template functions
engine = set_function(engine, "custom_format", "my_format_implementation")

// Use in templates
{{custom_format($data, "special")}}
```

### Asset Management
```cursed
// Create asset manager
sus assets AssetManager = create_asset_manager("https://cdn.example.com")
assets = add_stylesheet(assets, "/css/app.css")
assets = add_script(assets, "/js/app.js")

// Assets are automatically versioned and CDN-enabled
```

### Progressive Web App Support
```cursed
// Generate PWA manifest
sus manifest tea = create_pwa_manifest(
    "My App",
    "CURSED-powered web app", 
    "/icon-192.png"
)

// Add to layout
layout = add_pwa_support(layout, "/manifest.json")
```

## Error Handling

### Template Errors
```cursed
sus result TemplateResult = process_compiled_template(engine, template)

vibes !result.success {
    print("Error: " + result.error_message)
    print("Processed: " + string(result.processed_tokens))
}
```

### Security Violations
```cursed
// Security violations are logged and blocked
sus result tea = evaluate_expression_with_security(engine, dangerous_expr, context)

vibes result == "SECURITY_VIOLATION" {
    print("Expression blocked for security")
}
```

## Best Practices

### Performance
1. **Use template compilation** for frequently-used templates
2. **Enable caching** in production environments
3. **Minimize template size** by avoiding unnecessary whitespace
4. **Use components** for reusable template parts
5. **Optimize assets** with CDN and versioning

### Security  
1. **Always enable HTML escaping** for user content
2. **Use CSRF protection** for forms
3. **Validate template content** before rendering
4. **Enable sandbox mode** for untrusted templates
5. **Sanitize user inputs** before template processing

### Development
1. **Use template inheritance** for consistent layouts
2. **Create reusable components** for common UI elements
3. **Organize templates** in logical directory structures
4. **Use meaningful variable names** in templates
5. **Test templates** with various data scenarios

## API Reference

### Core Types
- `AdvancedTemplateEngine`: Main template engine
- `TemplateResult`: Template processing result
- `CompiledTemplate`: Pre-compiled template
- `TemplateInstruction`: Compiled instruction
- `HTMLComponent`: Reusable component
- `WebLayout`: Page layout system
- `WebForm`: Form generation system

### Main Functions
- `create_advanced_template_engine()`: Create engine
- `process_compiled_template()`: Process template
- `compile_template()`: Compile template
- `set_variable_scoped()`: Set template variable
- `render_component()`: Render HTML component
- `render_web_layout()`: Render complete page

### Security Functions
- `escape_html_content()`: Escape HTML
- `validate_template_security()`: Validate template
- `evaluate_expression_with_security()`: Safe evaluation

### Web Functions
- `create_web_template_engine()`: Web-optimized engine
- `create_html_component()`: Create component
- `create_web_form()`: Create form
- `render_web_form()`: Render form HTML

The CURSED Advanced Template Engine provides a complete solution for modern web template needs with enterprise-grade security, performance, and functionality.
