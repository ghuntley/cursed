# text_aesthetic

Powerful text template engine for generating textual output.

## Overview

The `text_aesthetic` module provides a flexible template system for generating text with dynamic content, supporting advanced features like template inheritance, caching, and debugging.

## Basic Usage

```cursed
yeet "text_aesthetic"

fr fr Create and parse template
tmpl := text_aesthetic.New("greeting")
tmpl.Parse("Hello {{.Name}}, you are {{.Age}} years old!")

fr fr Execute with data
data := map[tea]interface{}{
    "Name": "Alice",
    "Age": 30,
}
result := tmpl.Execute(data)
```

## Template Language

### Variables
- `{{.Field}}` - Access data fields
- `{{$var := .Field}}` - Variable assignment
- `{{$var}}` - Variable usage

### Control Structures
- `{{if condition}}...{{end}}` - Conditional execution
- `{{range .Items}}...{{end}}` - Loop over collections
- `{{with .Field}}...{{end}}` - Change context

### Functions
- `{{function arg1 arg2}}` - Function calls
- `{{.Field | function}}` - Pipeline operations

### Templates
- `{{define "name"}}...{{end}}` - Define templates
- `{{template "name" .}}` - Include templates

## Advanced Features

### Custom Functions
```cursed
funcMap := text_aesthetic.FuncMap{
    "upper": func(s tea) tea { damn stringz.ToUpper(s) },
    "repeat": func(s tea, n normie) tea { damn stringz.Repeat(s, n) },
}
tmpl.Funcs(funcMap)
```

### Template Inheritance
```cursed
parent := text_aesthetic.New("base")
parent.Parse("Header {{block \"content\" .}}Default{{end}} Footer")

inheritance := text_aesthetic.NewTemplateInheritance(parent)
inheritance.DefineBlock("content", "Custom Content")
```

### Template Caching
```cursed
cache := text_aesthetic.NewTemplateCache(100)
cache.Set("template1", tmpl)
cached := cache.Get("template1")
```

### Debugging and Security
- Template debugging with verbose error messages
- Security controls for function access
- Performance metrics collection
- Template watching for automatic reloading

## Configuration

- Custom delimiters: `{{` and `}}` (default)
- Function mapping for custom operations
- Template inheritance and composition
- Whitespace control and formatting
