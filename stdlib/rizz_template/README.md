# Rizz Template Engine

Next-generation template engine for CURSED with Gen Z enhanced APIs, comprehensive template functionality, and enterprise-grade security features.

## Overview

The Rizz Template Engine provides powerful template rendering capabilities with variable substitution, control flow, template inheritance, filters, and security features. Designed for modern web development with both performance and security in mind.

## Features

### Core Template Features
- **Variable Substitution**: `{{variable}}` syntax with context management
- **Control Flow**: `{% if %}`, `{% for %}` loops with conditional logic
- **Template Inheritance**: `{% block %}` and `{% extends %}` for modular templates
- **Template Includes**: `{% include %}` for reusable components
- **Filter Pipeline**: `{{value|filter1|filter2}}` for data transformation
- **Multiple Output Formats**: HTML, text, and JSON rendering

### Security Features
- **XSS Prevention**: Automatic HTML escaping for variables
- **Template Validation**: Security scanning for dangerous patterns
- **Safe Filtering**: Secure filter processing with validation
- **Context Isolation**: Protected variable scope management

### Performance Features
- **Template Compilation**: Pre-compilation for faster rendering
- **Optimized Parsing**: Efficient template processing
- **Variable Caching**: Context optimization for repeated renders
- **Filter Optimization**: Pre-compiled filter chains

### Gen Z Enhanced APIs
- **`rizz_template_no_cap()`**: HTML rendering with full features
- **`rizz_template_fr_fr()`**: Text rendering for real talk
- **`rizz_template_bussin()`**: High-performance optimized rendering
- **`rizz_template_periodt()`**: Format-specific rendering with finality

## Basic Usage

### Variable Substitution
```cursed
yeet "rizz_template"

sus template tea = "Hello {{name}}, welcome to {{site}}!"
sus context rizz_template.TemplateContext = rizz_template.rizz_create_context()
context = rizz_template.rizz_set_context(context, "name", "Chad")
context = rizz_template.rizz_set_context(context, "site", "CURSED Lang")

sus result tea = rizz_template.rizz_parse_template(template, context)
// Output: "Hello Chad, welcome to CURSED Lang!"
```

### Conditional Rendering
```cursed
sus template tea = "{% if user_logged_in %}Welcome back, {{username}}!{% endif %}"
sus context rizz_template.TemplateContext = rizz_template.rizz_create_context()
context = rizz_template.rizz_set_context(context, "user_logged_in", "true")
context = rizz_template.rizz_set_context(context, "username", "Alice")

sus result tea = rizz_template.rizz_parse_template(template, context)
// Output: "Welcome back, Alice!"
```

### Loop Processing
```cursed
sus template tea = "{% for item in items %}{{loop.index}}: {{item}}\n{% endfor %}"
sus context rizz_template.TemplateContext = rizz_template.rizz_create_context()
context = rizz_template.rizz_set_context(context, "items", "product_list")

sus result tea = rizz_template.rizz_parse_template(template, context)
// Output: Multiple lines with numbered items
```

### Filter Processing
```cursed
sus template tea = "{{name|upper}} - {{description|capitalize|trim}}"
sus context rizz_template.TemplateContext = rizz_template.rizz_create_context()
context = rizz_template.rizz_set_context(context, "name", "cursed")
context = rizz_template.rizz_set_context(context, "description", " awesome language ")

sus result tea = rizz_template.rizz_parse_template(template, context)
// Output: "CURSED - Awesome language"
```

## Advanced Features

### Template Inheritance
```cursed
// Parent template
sus parent tea = "<!DOCTYPE html>\n<body>\n{% block content %}Default{% endblock %}\n</body>"

// Child template
sus child tea = "{% block content %}<h1>Custom Page</h1>{% endblock %}"

sus result tea = rizz_template.rizz_extend_template(child, parent, context)
```

### Multiple Output Formats
```cursed
// HTML output with automatic escaping
sus html tea = rizz_template.rizz_render_to_html(template, context)

// Plain text output
sus text tea = rizz_template.rizz_render_to_text(template, context)

// JSON output with structured data
sus json tea = rizz_template.rizz_render_to_json(template, context)
```

### Security Features
```cursed
// Validate template for security issues
sus is_safe lit = rizz_template.rizz_validate_template(template)

// HTML escaping for XSS prevention
sus escaped tea = rizz_template.rizz_escape_html("<script>alert('xss')</script>")
// Output: "&lt;script&gt;alert('xss')&lt;/script&gt;"
```

## Template Syntax Reference

### Variables
- `{{variable}}` - Basic variable substitution
- `{{variable|filter}}` - Variable with filter
- `{{variable|filter1|filter2}}` - Multiple filters

### Control Flow
- `{% if condition %}...{% endif %}` - Conditional blocks
- `{% if var == value %}...{% endif %}` - Equality conditions
- `{% for item in array %}...{% endfor %}` - Loop iteration

### Template Structure
- `{% block name %}...{% endblock %}` - Defined blocks for inheritance
- `{% include "template" %}` - Include external templates
- `{% extends "base" %}` - Template inheritance

### Loop Variables
- `{{loop.index}}` - Current iteration number (1-based)
- `{{loop.first}}` - True if first iteration
- `{{loop.last}}` - True if last iteration

## Available Filters

### Text Transformation
- `upper` - Convert to uppercase
- `lower` - Convert to lowercase
- `capitalize` - Capitalize first letter
- `reverse` - Reverse string
- `trim` - Remove whitespace

### Encoding Filters
- `escape` - HTML escape for security
- `url_encode` - URL encoding
- `base64` - Base64 encoding

### Utility Filters
- `length` - Get string length

## Gen Z Enhanced APIs

### `rizz_template_no_cap(template, context)`
HTML rendering with full security features enabled. Perfect for web applications requiring XSS protection.

### `rizz_template_fr_fr(template, vibes)`
Text rendering for honest, straightforward output. No HTML escaping, pure text results.

### `rizz_template_bussin(template, context)`
High-performance rendering with template compilation optimization. Best for production environments with repeated template usage.

### `rizz_template_periodt(template, context, format)`
Format-specific rendering with explicit output control. Supports "html", "text", and "json" formats.

## Context Management

### Basic Context Operations
```cursed
// Create empty context
sus context rizz_template.TemplateContext = rizz_template.rizz_create_context()

// Set variables
context = rizz_template.rizz_set_context(context, "key", "value")

// Merge contexts
sus merged rizz_template.TemplateContext = rizz_template.rizz_merge_contexts(base, overlay)
```

## Security Best Practices

1. **Always validate templates** before rendering with `rizz_validate_template()`
2. **Use HTML escaping** for web output with `rizz_render_to_html()`
3. **Sanitize user input** before adding to template context
4. **Avoid dynamic template generation** from untrusted sources
5. **Use strict mode** in production environments

## Performance Optimization

### Template Compilation
```cursed
// Pre-compile templates for better performance
sus compiled tea = rizz_template.rizz_compile_template(template)
sus result tea = rizz_template.rizz_parse_template(compiled, context)
```

### Configuration Options
```cursed
sus config rizz_template.TemplateConfig = rizz_template.TemplateConfig{
    escape_html: based,    // Enable HTML escaping
    strict_mode: based,    // Enable strict variable checking
    max_depth: 10,         // Maximum nesting depth
    output_format: "html"  // Default output format
}
```

## Debugging and Development

### Template Debugging
```cursed
// Get detailed debug information
sus debug_output tea = rizz_template.rizz_debug_template(template, context)
// Includes template source, context variables, and security status
```

### Error Handling
The template engine handles errors gracefully:
- Missing variables render as empty strings
- Invalid syntax is ignored with fallback content
- Security violations prevent rendering

## Example: Complete Web Template

```cursed
yeet "rizz_template"

// Define template with inheritance and filters
sus base_template tea = `
<!DOCTYPE html>
<html>
<head><title>{{title|escape}}</title></head>
<body>
{% block content %}Default content{% endblock %}
</body>
</html>`

sus page_template tea = `
{% block content %}
<h1>{{page_title|capitalize}}</h1>
{% if user_logged_in %}
    <p>Welcome, {{username|escape}}!</p>
    {% for item in recent_items %}
        <li>{{loop.index}}: {{item|escape}}</li>
    {% endfor %}
{% endif %}
{% endblock %}`

// Create context with all variables
sus context rizz_template.TemplateContext = rizz_template.rizz_create_context()
context = rizz_template.rizz_set_context(context, "title", "My Website")
context = rizz_template.rizz_set_context(context, "page_title", "dashboard")
context = rizz_template.rizz_set_context(context, "user_logged_in", "true")
context = rizz_template.rizz_set_context(context, "username", "Alice")
context = rizz_template.rizz_set_context(context, "recent_items", "user_data")

// Render with inheritance
sus final_page tea = rizz_template.rizz_extend_template(page_template, base_template, context)

// Output secure HTML
vibez.spill(final_page)
```

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/rizz_template/test_rizz_template.💀
```

The test suite includes:
- Variable substitution testing
- Security validation (XSS prevention)
- Control flow verification
- Filter pipeline testing
- Template inheritance validation
- Multiple output format testing
- Gen Z API verification
- Performance optimization testing

## Integration

The Rizz Template Engine integrates seamlessly with other CURSED stdlib modules:
- **stringz**: String manipulation and processing
- **encode_mood**: Encoding and security functions
- **json**: JSON output formatting
- **testz**: Comprehensive testing framework

## License

Part of the CURSED programming language standard library. Follow the same license terms as the main CURSED project.
