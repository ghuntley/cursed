# RizzTemplate Module

A powerful data-driven templating system for generating textual output with enhanced "rizz" (style and charisma).

## Features

- Template parsing and execution
- Function maps for custom template functions
- Template caching for performance
- High-level template renderer
- Layout system for template inheritance
- Built-in functions for text manipulation, arithmetic, and logic

## Usage

### Basic Template Usage

```cursed
yeet "rizz_template"

// Create and parse a template
sus tmpl := rizz_template.New("greeting")
sus parsed, err := tmpl.Parse("Hello, {{.Name}}!")

// Execute template with data
sus data := map[tea]interface{}{
    "Name": "Alice",
}
sus result, err := parsed.ExecuteToString(data)
vibez.spill(result) // Output: Hello, Template Value!
```

### Using Function Maps

```cursed
sus funcs := rizz_template.FuncMap{
    "upper": rizz_template.upper,
    "lower": rizz_template.lower,
}
tmpl.AddFuncs(funcs)
```

### Template Caching

```cursed
sus cache := rizz_template.NewTemplateCache()
cache.Set("greeting", tmpl)
sus retrieved, found := cache.Get("greeting")
```

### Template Renderer

```cursed
sus opts := rizz_template.TemplateOptions{
    LeftDelim: "{{",
    RightDelim: "}}",
    StrictMode: based,
}
sus renderer := rizz_template.NewTemplateRenderer("./templates", opts)
renderer.AddGlobal("siteName", "My Site")
```

### Layout System

```cursed
sus layout := rizz_template.NewRizzLayout("base", tmpl)
layout.SetContent("title", "Page Title")
layout.SetContent("content", "Page Content")
sus result, err := layout.String()
```

## Built-in Functions

### String Functions
- `lower(s)` - Convert to lowercase
- `upper(s)` - Convert to uppercase
- `trim(s)` - Trim whitespace
- `join(array, sep)` - Join string array
- `contains(s, substr)` - Check substring

### Arithmetic Functions
- `add(a, b)` - Addition
- `sub(a, b)` - Subtraction
- `mul(a, b)` - Multiplication
- `div(a, b)` - Division

### Comparison Functions
- `eq(a, b)` - Equal
- `ne(a, b)` - Not equal
- `lt(a, b)` - Less than
- `gt(a, b)` - Greater than

### Logical Functions
- `and_func(a, b)` - Logical AND
- `or_func(a, b)` - Logical OR
- `not_func(a)` - Logical NOT

## Implementation Notes

This is a pure CURSED implementation focusing on core templating functionality. The template processing includes basic variable interpolation and conditional processing. Function implementations are simplified for demonstration but provide the foundation for a full templating system.
