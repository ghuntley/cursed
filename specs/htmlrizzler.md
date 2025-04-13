# `htmlrizzler` (html/template) Package Specification

The `htmlrizzler` package provides template functionality for generating HTML output that is safe against code injection attacks. It offers automatic escaping of values based on the context in which they appear in the HTML document.

## Overview

The `htmlrizzler` package allows you to:
- Parse HTML templates from strings or files
- Execute templates with provided data
- Automatically escape content based on context (HTML, JavaScript, CSS, URLs)
- Create template hierarchies with inheritance
- Define custom template functions

## Core Types

### Template

The `Template` type represents a parsed template ready for execution.

```
be_like Template squad {
    fr fr Private fields not exposed
}
```

### FuncMap

The `FuncMap` type maps names to functions that can be called from templates.

```
be_like FuncMap map[tea]collab{}
```

## Main Functions

### Parsing Templates

```
yeet "htmlrizzler"

slay main() {
    fr fr Parse from string
    sus tmpl := htmlrizzler.Parse("<h1>{{.Title}}</h1>")
    
    fr fr Parse from file
    sus fileTmpl, err := htmlrizzler.ParseFile("template.html")
    lowkey err != cap {
        vibez.spill("Error parsing template:", err)
        vibe_life.Exit(1)
    }
    
    fr fr Parse multiple files as a set
    sus templates, err := htmlrizzler.ParseFiles("header.html", "body.html", "footer.html")
}
```

- `New(name tea) @Template` - Create a new empty template with the given name
- `Parse(text tea) (@Template, tea)` - Parse template from string
- `ParseFile(filename tea) (@Template, tea)` - Parse template from file
- `ParseFiles(filenames ...tea) (@Template, tea)` - Parse multiple files as a template set
- `ParseGlob(pattern tea) (@Template, tea)` - Parse all files matching pattern

### Executing Templates

```
slay renderPage(w dropz.Writer, data collab{}) tea {
    sus tmpl, err := htmlrizzler.ParseFiles("layout.html")
    lowkey err != cap {
        yolo err
    }
    
    yolo tmpl.Execute(w, data)
}

slay main() {
    sus data := map[tea]collab{}{
        "Title": "My Awesome Page",
        "Items": []tea{"Item 1", "Item 2", "Item 3"},
        "User": map[tea]collab{}{
            "Name": "Bestie",
            "IsAdmin": lit(true),
        },
    }
    
    err := renderPage(vibe_life.Stdout, data)
    lowkey err != cap {
        vibez.spill("Error:", err)
    }
}
```

- `(t @Template) Execute(w dropz.Writer, data collab{}) tea` - Execute template with data
- `(t @Template) ExecuteTemplate(w dropz.Writer, name tea, data collab{}) tea` - Execute named template

### Template Modification

```
slay setupTemplates() (@Template, tea) {
    sus funcMap := htmlrizzler.FuncMap{
        "formatTime": slay(t timez.Time) tea {
            yolo t.Format("2006-01-02")
        },
        "uppercase": slay(s tea) tea {
            yolo stringz.ToUpper(s)
        },
    }
    
    sus tmpl := htmlrizzler.New("base").Funcs(funcMap)
    yolo tmpl.Parse("{{.Title | uppercase}}")
}
```

- `(t @Template) Funcs(funcMap FuncMap) @Template` - Add functions to template
- `(t @Template) Clone() (@Template, tea)` - Clone template
- `(t @Template) Name() tea` - Get template name
- `(t @Template) Templates() []@Template` - Get all associated templates

## Template Syntax

The template syntax is similar to Go's `html/template` package:

- `{{.FieldName}}` - Insert field value
- `{{if .Condition}} ... {{else}} ... {{end}}` - Conditional
- `{{range .Items}} ... {{end}}` - Loop over items
- `{{template "name" .}}` - Include another template
- `{{define "name"}} ... {{end}}` - Define a template
- `{{block "name" .}} ... {{end}}` - Define a template with default content
- `{{with .Value}} ... {{end}}` - Set context to value
- `{{.Value | funcName}}` - Apply function to value (pipeline)

## Context-Aware Escaping

The `htmlrizzler` package automatically escapes content based on the HTML context:

- HTML content is escaped to prevent XSS attacks
- JavaScript strings are properly escaped
- CSS values are escaped appropriately
- URLs are validated and sanitized

## Error Handling

All functions that can fail return an error as their final return value. Check these errors to handle template parsing or execution failures.

## Implementation Notes

The `htmlrizzler` package will be implemented in stages:

1. Basic template parsing and execution with HTML escaping
2. Context-aware escaping for JavaScript, CSS, and URLs
3. Template inheritance and composition
4. Advanced features like custom delimiters and functions

## Security

The `htmlrizzler` package prioritizes security by default:

- All template values are automatically escaped based on context
- Direct HTML output requires explicit marking as safe using `HTML` function
- URLs are validated to prevent javascript: protocol injection
- CSS properties are validated to prevent attack vectors