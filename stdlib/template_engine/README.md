# Template Engine Module

A powerful template processing system for CURSED that enables dynamic content generation with variable substitution, control flow, and function calls.

## Features

- **Variable Substitution**: Replace placeholders with dynamic values
- **Control Flow**: Conditional rendering and loops
- **Function Calls**: Built-in and custom template functions
- **Multiple Formats**: HTML, Markdown, Email, and custom templates
- **Security**: HTML escaping and safe template execution
- **Performance**: Efficient tokenization and processing
- **Extensibility**: Custom delimiters and functions

## Core Components

### Template Engine
The main template processing engine that handles tokenization, parsing, and rendering.

### Template Context
Contains variables, functions, and state for template processing.

### Template Tokens
Parsed template elements including text, expressions, and control structures.

## Basic Usage

### Simple Variable Substitution
```cursed
// Template: "Hello {{$name}}, welcome to {{$site}}"
// Variables: name = "User", site = "CURSED"
// Result: "Hello User, welcome to CURSED"
```

### Template Functions
```cursed
// Template: "{{upper($message)}}"
// Variables: message = "hello world"
// Result: "HELLO WORLD"
```

### Control Flow
```cursed
// Template: "{{if user == 'admin'}}Admin Panel{{/if}}"
// Variables: user = "admin"
// Result: "Admin Panel"
```

## Template Syntax

### Variable Substitution
- `{{$variable}}` - Simple variable substitution
- `{{$object.field}}` - Object field access
- `{{$array[index]}}` - Array element access

### Functions
- `{{upper($text)}}` - Convert to uppercase
- `{{lower($text)}}` - Convert to lowercase  
- `{{len($text)}}` - Get string length
- `{{default($value, "fallback")}}` - Default value if empty
- `{{join($array, ", ")}}` - Join array elements

### Control Structures
- `{{if condition}}...{{/if}}` - Conditional rendering
- `{{for item in collection}}...{{/for}}` - Loop over collection
- `{{include "template"}}` - Include another template

### Comments
- `{{/* This is a comment */}}` - Template comments (not rendered)

## Configuration

### Custom Delimiters
```cursed
// Change default {{}} to custom delimiters
// Default: {{ and }}
// Custom: [[ and ]]
```

### HTML Escaping
```cursed
// Enable/disable HTML escaping for security
// Default: enabled for HTML templates
// Disabled for plain text templates
```

### Template Modes
```cursed
// HTML Mode: Automatic HTML escaping
// Markdown Mode: Preserve markdown formatting
// Email Mode: Email-specific formatting
// Custom Mode: User-defined behavior
```

## Template Functions

### String Functions
- `upper($text)` - Convert to uppercase
- `lower($text)` - Convert to lowercase
- `trim($text)` - Remove whitespace
- `len($text)` - Get string length
- `substr($text, start, length)` - Get substring

### Utility Functions
- `default($value, fallback)` - Provide default value
- `join($array, separator)` - Join array elements
- `split($text, separator)` - Split string into array
- `replace($text, old, new)` - Replace text

### Conditional Functions
- `if($condition, true_val, false_val)` - Conditional value
- `eq($a, $b)` - Test equality
- `ne($a, $b)` - Test inequality
- `gt($a, $b)` - Test greater than
- `lt($a, $b)` - Test less than

## Advanced Features

### Template Inheritance
```cursed
// Base template with blocks
{{define "base"}}
<html>
<head><title>{{$title}}</title></head>
<body>{{block "content"}}{{/block}}</body>
</html>
{{/define}}

// Child template extends base
{{extends "base"}}
{{block "content"}}
<h1>{{$heading}}</h1>
<p>{{$content}}</p>
{{/block}}
```

### Custom Functions
```cursed
// Register custom function
engine = set_function(engine, "currency", format_currency)

// Use in template
{{currency($amount)}}
```

### Loop Variables
```cursed
// Access loop variables
{{for item in items}}
  {{$index}}: {{$item}} ({{$first}} {{$last}})
{{/for}}
```

### Template Caching
```cursed
// Cache compiled templates for performance
// Automatic cache invalidation on changes
```

## Security Features

### HTML Escaping
- Automatic escaping of HTML entities
- Prevention of XSS attacks
- Safe rendering of user content

### Template Sandboxing
- Restricted function access
- Safe template execution
- Prevention of code injection

### Input Validation
- Template syntax validation
- Variable type checking
- Function argument validation

## Performance Optimization

### Template Compilation
- Pre-compile templates for better performance
- Cache compiled templates
- Lazy loading of templates

### Efficient Processing
- Minimal string allocations
- Optimized tokenization
- Fast variable lookup

## Error Handling

### Syntax Errors
- Clear error messages for invalid syntax
- Line number and position reporting
- Helpful suggestions for fixes

### Runtime Errors
- Graceful handling of missing variables
- Function call error handling
- Fallback values for errors

## Integration Examples

### Web Applications
```cursed
// Render HTML pages
sus engine TemplateEngine = create_html_engine()
engine = set_variable(engine, "title", "My App")
engine = set_variable(engine, "content", "Welcome!")
sus result TemplateResult = process_template(engine, page_template)
```

### Email Templates
```cursed
// Generate email content
sus engine TemplateEngine = create_email_engine()
engine = set_variable(engine, "user", "John Doe")
engine = set_variable(engine, "message", "Hello!")
sus email_content tea = process_template(engine, email_template)
```

### Configuration Files
```cursed
// Generate config files
sus engine TemplateEngine = create_template_engine()
engine = set_variable(engine, "database_url", "localhost:5432")
engine = set_variable(engine, "debug_mode", "true")
sus config_content tea = process_template(engine, config_template)
```

## Best Practices

1. **Variable Naming**: Use descriptive variable names
2. **Template Organization**: Keep templates modular and reusable
3. **Error Handling**: Always check template processing results
4. **Security**: Enable HTML escaping for web templates
5. **Performance**: Cache frequently used templates
6. **Testing**: Test templates with various data combinations

## Testing

Test the template engine:
```bash
cargo run --bin cursed stdlib/template_engine/test_template_engine.csd
```

## Template Examples

### Basic HTML Template
```html
<!DOCTYPE html>
<html>
<head>
    <title>{{$title}}</title>
</head>
<body>
    <h1>{{$heading}}</h1>
    <p>Welcome {{$user}}, today is {{$date}}</p>
    {{if $show_menu}}
    <nav>{{include "menu"}}</nav>
    {{/if}}
</body>
</html>
```

### Email Template
```
Subject: {{$subject}}
To: {{$recipient}}

Dear {{$name}},

{{$message}}

{{if $include_footer}}
Best regards,
{{$sender}}
{{/if}}
```

### Configuration Template
```
[database]
host = {{$db_host}}
port = {{$db_port}}
user = {{$db_user}}

[server]
listen = {{$server_host}}:{{$server_port}}
debug = {{$debug_mode}}
```

This template engine provides a powerful and flexible system for dynamic content generation while maintaining security and performance in CURSED applications.
