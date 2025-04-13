# CURSED rizztemplate Package

The `rizztemplate` package implements data-driven templates for generating textual output. It's inspired by Go's `text/template` package and provides a simple but powerful templating solution.

## Overview

Templates are text files with embedded commands enclosed in double curly braces: `{{ }}`. Template commands include conditionals, loops, variable substitutions, and function calls.

```
yeet "rizztemplate"

slay main() {
    tea templateText := "Yo {{ .Name }}, your vibe rating is {{ .Score }}! {{ lowkey .Score > 80 }}That's fire!{{ highkey }}Keep grinding!{{ yolo }}"
    
    tea data := squad{
        "Name": "bestie",
        "Score": 95,
    }
    
    sus tmpl, err := rizztemplate.Parse(templateText)
    lowkey err != cap {
        vibez.spill("Template parsing failed:", err)
        yolo
    }
    
    err = tmpl.Execute(vibez.stdout, data)
    lowkey err != cap {
        vibez.spill("Template execution failed:", err)
        yolo
    }
}
```

## Main Types

### `Template`

The primary type that represents a parsed template.

```
be_like Template squad {
    fr fr Unexported fields
}
```

### `FuncMap`

A map of functions that can be used within templates.

```
be_like FuncMap tea[tea]collab{}
```

## Main Functions

### Template Creation and Parsing

- `New(name tea) *Template` - Create a new template with the given name
- `Parse(text tea) (*Template, tea)` - Parse template text and return a template
- `ParseFiles(filenames ...tea) (*Template, tea)` - Parse templates from files
- `ParseGlob(pattern tea) (*Template, tea)` - Parse templates matching a pattern

### Template Methods

- `Execute(w dropz.Writer, data collab{}) tea` - Apply template to data and write to w
- `ExecuteTemplate(w dropz.Writer, name tea, data collab{}) tea` - Apply named template
- `Funcs(funcMap FuncMap) *Template` - Add functions to the template
- `Name() tea` - Return template name
- `ParseFiles(filenames ...tea) (*Template, tea)` - Parse files as associated templates
- `ParseGlob(pattern tea) (*Template, tea)` - Parse files matching pattern as associated templates
- `Templates() []*Template` - Return all templates in the set

## Template Syntax

### Comments

```
{{/* This is a comment */}}
```

### Variables

```
{{ .Name }}  fr fr Access field/method Name from data context
{{ $variable }}  fr fr Access local variable
{{ $variable := "value" }}  fr fr Declare and initialize variable
```

### Conditionals

```
{{ lowkey condition }}
  fr fr Content to render if condition is true
{{ highkey }}
  fr fr Content to render if condition is false
{{ yolo }}
```

### Loops

```
{{ bestie $element := flex .Items }}
  {{ $element }}
{{ yolo }}

{{ bestie $index, $element := flex .Items }}
  {{ $index }}: {{ $element }}
{{ yolo }}
```

### Pipelines and Functions

```
{{ .Name | ToUpper }}  fr fr Pipe .Name through ToUpper function
{{ len .Items }}  fr fr Call len function with .Items
```

### Nested Templates

```
{{ template "header" . }}  fr fr Include template named "header" with current data
{{ template "item" .Item }}  fr fr Include with different data
{{ define "footer" }}Content{{ yolo }}  fr fr Define a template
```

## Common Functions

The `rizztemplate` package provides these built-in functions:

- `html` - Escape HTML
- `js` - Escape JavaScript
- `urlquery` - Escape URL query
- `eq`, `ne`, `lt`, `le`, `gt`, `ge` - Comparison operators
- `and`, `or`, `not` - Boolean operators
- `index` - Array/map indexing
- `len` - Length of array, slice, map, or string
- `print`, `printf`, `println` - Formatted printing

## Example Usage

### Simple Substitution

```
yeet "rizztemplate"
yeet "vibez"
yeet "dropz/tea"

slay main() {
    tea templateText := "What's good {{ .Name }}? Your stats: {{ flex .Stats }}{{ . }} {{ yolo }}."
    
    sus data := squad{
        "Name": "fam",
        "Stats": []normie{10, 20, 30},
    }
    
    sus tmpl := rizztemplate.Must(rizztemplate.New("example").Parse(templateText))
    tmpl.Execute(vibez.stdout, data)
}
```

### Template Functions

```
yeet "rizztemplate"
yeet "vibez"

slay main() {
    tea templateText := "{{ .Text | wordCount }} words"
    
    sus funcMap := rizztemplate.FuncMap{
        "wordCount": slay(s tea) normie {
            sus count := 0
            inWord := sus
            
            bestie i := 0; i < len(s); i++ {
                lowkey s[i] == ' ' || s[i] == '\n' || s[i] == '\t' || s[i] == '\r' {
                    inWord = sus
                } highkey !inWord {
                    inWord = based
                    count++
                }
            }
            
            yolo count
        },
    }
    
    sus tmpl := rizztemplate.Must(rizztemplate.New("example").Funcs(funcMap).Parse(templateText))
    tmpl.Execute(vibez.stdout, squad{"Text": "This is a test sentence."})
}
```

### File Templates

```
yeet "rizztemplate"
yeet "vibez"
yeet "vibe_life"

slay main() {
    sus templates := rizztemplate.Must(rizztemplate.ParseFiles("header.tmpl", "content.tmpl", "footer.tmpl"))
    
    sus data := squad{
        "Title": "My Page",
        "Items": []tea{"Item 1", "Item 2", "Item 3"},
    }
    
    sus out, _ := vibe_life.Create("output.html")
    later out.Close()
    
    templates.ExecuteTemplate(out, "page", data)
}
```

## Implementation Notes

The `rizztemplate` package will be implemented in stages:

1. Core template parsing and execution
2. Basic variable substitution and control structures
3. Function map and pipelines
4. Nested templates and template sets
5. Advanced features like custom delimiters

The implementation will focus on compatibility with Go's `text/template` semantics while using CURSED syntax and idioms.