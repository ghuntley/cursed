# RizzTemplate - CURSED Template Engine

A powerful, pure CURSED template engine with variable interpolation, conditionals, loops, and advanced features.

## Features

- **Variable Interpolation**: `{{variable}}` syntax for dynamic content
- **Conditional Rendering**: `{{if condition}}...{{endif}}` blocks
- **Loop Processing**: `{{for item in items}}...{{endfor}}` iteration
- **Layout Support**: Master layout templates with content injection
- **Include System**: Template composition and reusability
- **Validation**: Template syntax validation and error reporting
- **Performance**: Optimized rendering with compilation support

## Quick Start

```cursed
yeet "rizz_template"

# Create a new template
sus template RizzTemplate = rizz_template_new("Hello {{name}}!")

# Set variables
rizz_template_set_var(&template, "name", "CURSED")

# Render template
sus result tea = rizz_template_render(&template)
vibez.spill(result)  # Output: "Hello CURSED!"
```

## Variable Interpolation

Variables are enclosed in double curly braces and automatically escaped:

```cursed
sus template RizzTemplate = rizz_template_new("Welcome {{name}} to {{place}}!")
rizz_template_set_var(&template, "name", "Developer")
rizz_template_set_var(&template, "place", "CURSED")
sus result tea = rizz_template_render(&template)
# Output: "Welcome Developer to CURSED!"
```

## Conditional Rendering

Use `{{if condition}}...{{endif}}` blocks for conditional content:

```cursed
sus template RizzTemplate = rizz_template_new(`
{{if is_logged_in}}
Welcome back, {{username}}!
{{endif}}
`)

rizz_template_set_var(&template, "is_logged_in", "true")
rizz_template_set_var(&template, "username", "John")
```

### Conditional Operators

- **Variable existence**: `{{if variable}}` - true if variable exists and is not empty
- **Equality**: `{{if variable == "value"}}` - true if variable equals value
- **Nested conditions**: Conditions can be nested for complex logic

## Loop Processing

Use `{{for item in items}}...{{endfor}}` for iteration:

```cursed
sus template RizzTemplate = rizz_template_new(`
Your items:
{{for item in shopping_list}}
- {{item}}
{{endfor}}
`)

rizz_template_set_var(&template, "shopping_list", "apples,bananas,oranges")
```

## Layout Templates

Create master layouts with content injection:

```cursed
sus content_template RizzTemplate = rizz_template_new("This is the main content.")
rizz_template_set_var(&content_template, "title", "My Page")

sus layout_content tea = `
<!DOCTYPE html>
<html>
<head><title>{{title}}</title></head>
<body>{{content}}</body>
</html>
`

sus result tea = rizz_template_render_with_layout(&content_template, layout_content)
```

## Include System

Include other templates for composition:

```cursed
sus main_template RizzTemplate = rizz_template_new("Header content")
rizz_template_set_var(&main_template, "site_name", "My Site")

sus header_content tea = "Welcome to {{site_name}}!"
sus header_rendered tea = rizz_template_include(&main_template, header_content)
```

## Template Validation

Validate templates before rendering:

```cursed
sus is_valid lit
sus error_msg tea
(is_valid, error_msg) = rizz_template_validate("Hello {{name}}!")

if !is_valid {
    vibez.spill("Template error: " + error_msg)
}
```

## Advanced Features

### Template Compilation

Pre-compile templates for better performance:

```cursed
sus template RizzTemplate = rizz_template_new("Complex template content")
rizz_template_compile(&template)  # Optimize for repeated rendering
```

### Error Handling

The template engine provides comprehensive error detection:

- Unmatched braces: `{{variable` without closing `}}`
- Unmatched conditionals: `{{if}}` without `{{endif}}`
- Unmatched loops: `{{for}}` without `{{endfor}}`
- Invalid syntax: Malformed expressions

### Performance Optimization

- **Lazy evaluation**: Variables are only interpolated when needed
- **Caching**: Compiled templates cache parsing results
- **Streaming**: Large templates can be processed in chunks
- **Memory efficient**: Minimal memory allocation during rendering

## Template Syntax Reference

### Variables
- `{{variable}}` - Simple variable interpolation
- `{{  variable  }}` - Whitespace around variables is ignored

### Conditionals
- `{{if condition}}...{{endif}}` - Basic conditional
- `{{if var == "value"}}...{{endif}}` - Equality comparison
- Nested conditions are supported

### Loops
- `{{for item in items}}...{{endfor}}` - Loop over comma-separated items
- `{{item}}` - Access current loop item
- All template variables are available inside loops

### Comments
- `{{# This is a comment}}` - Comments are ignored during rendering

## Best Practices

1. **Validate templates** before production use
2. **Compile templates** that will be rendered multiple times
3. **Use layouts** for consistent page structure
4. **Separate concerns** with includes for reusable components
5. **Handle errors** gracefully in production code

## Example: Complete Web Page

```cursed
sus page_template RizzTemplate = rizz_template_new(`
{{if user_logged_in}}
<div class="user-info">
  <h2>Welcome, {{username}}!</h2>
  {{if is_admin}}
  <p>Admin Panel: <a href="/admin">Manage Site</a></p>
  {{endif}}
</div>
{{endif}}

<div class="content">
  <h1>{{page_title}}</h1>
  <p>{{page_content}}</p>
  
  {{if show_items}}
  <ul>
  {{for item in items}}
    <li>{{item}}</li>
  {{endfor}}
  </ul>
  {{endif}}
</div>
`)

# Set all variables
rizz_template_set_var(&page_template, "user_logged_in", "true")
rizz_template_set_var(&page_template, "username", "John Doe")
rizz_template_set_var(&page_template, "is_admin", "true")
rizz_template_set_var(&page_template, "page_title", "My Dashboard")
rizz_template_set_var(&page_template, "page_content", "Welcome to your dashboard!")
rizz_template_set_var(&page_template, "show_items", "true")
rizz_template_set_var(&page_template, "items", "Task 1,Task 2,Task 3")

sus final_page tea = rizz_template_render(&page_template)
vibez.spill(final_page)
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/rizz_template/test_rizz_template.csd
```

The test suite covers:
- ✅ Basic template creation and rendering
- ✅ Variable interpolation and management
- ✅ Conditional rendering (true/false cases)
- ✅ Loop processing with multiple items
- ✅ Complex templates with all features
- ✅ Template validation and error handling
- ✅ Layout rendering and includes
- ✅ Helper functions and utilities
- ✅ Edge cases and error conditions
- ✅ Performance testing

## License

Part of the CURSED standard library - Pure CURSED implementation without external dependencies.
