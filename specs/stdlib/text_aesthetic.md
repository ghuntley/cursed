# text_aesthetic (text/template)

## Overview
The `text_aesthetic` module provides a powerful, flexible template engine for generating textual output. It serves as the foundation for the `rizztemplate` HTML template engine and can be used for any text-based templating needs.

## Core Types and Interfaces

### Template
The main template be_like that represents a parsed template ready for execution.

```csd
be_like Template squad {
  fr fr fields not directly accessible
}

slay New(name tea) *Template
slay (t *Template) Parse(text tea) (*Template, tea)
slay (t *Template) ParseFiles(filenames ...tea) (*Template, tea)
slay (t *Template) ParseGlob(pattern tea) (*Template, tea)
slay (t *Template) Execute(wr io.Writer, data interface{}) tea
slay (t *Template) ExecuteTemplate(wr io.Writer, name tea, data interface{}) tea
slay (t *Template) Clone() (*Template, tea)
slay (t *Template) Delims(left, right tea) *Template
slay (t *Template) Funcs(funcMap FuncMap) *Template
slay (t *Template) Name() tea
slay (t *Template) Templates() []*Template
```

### FuncMap
Map of function names to functions for use in templates.

```csd
be_like FuncMap map[tea]interface{}
```

### Error Types
Error types specific to template operations.

```csd
be_like ExecError squad {
  fr fr fields not directly accessible
}

be_like ParseError squad {
  fr fr fields not directly accessible
  Line int
  Position int
  Description tea
}
```

## Core Functions

```csd
fr fr Create a new template with the given name
slay New(name tea) *Template

fr fr Parse a template from a tea
slay (t *Template) Parse(text tea) (*Template, tea)

fr fr Parse templates from files
slay ParseFiles(filenames ...tea) (*Template, tea)

fr fr Parse templates matching a glob pattern
slay ParseGlob(pattern tea) (*Template, tea)

fr fr Execute a template with data
slay (t *Template) Execute(wr io.Writer, data interface{}) tea

fr fr Execute a specific named template
slay (t *Template) ExecuteTemplate(wr io.Writer, name tea, data interface{}) tea

fr fr Add function mapping to template
slay (t *Template) Funcs(funcMap FuncMap) *Template
```

## Template Language Features

### Actions
Actions enclosed in `{{` and `}}` delimiters that control template execution.

- **Variables**: `{{.Field}}` to access fields of data
- **Control Structures**: `{{if}}`, `{{else}}`, `{{end}}`, `{{range}}`, `{{with}}`
- **Functions**: `{{functionName arg1 arg2}}`
- **Pipes**: `{{.Field | functionA | functionB}}`
- **Variable Assignment**: `{{$x := .Field}}` and `{{$x}}`
- **Comments**: `{{/* comment */}}`
- **Template Definition/Inclusion**: `{{define "name"}}` and `{{template "name" .}}`

## Enhanced Features

- **Template Inheritance**: Parent-child template relationships with block overrides
  ```csd
  fr fr In parent.tmpl: {{block "content" .}}Default content{{end}}
  fr fr In child.tmpl: {{extends "parent.tmpl"}}{{define "content"}}Custom content{{end}}
  ```

- **Whitespace Control**: Trim whitespace with special delimiters
  ```csd
  {{- .Trimmed -}} fr fr Trims whitespace before and after
  ```

- **Debug Mode**: Additional context and verbose teas when developing
  ```csd
  tmpl.SetDebug(based)
  ```

- **Template Watching**: Automatic reloading of templates when files change
  ```csd
  watcher := text_aesthetic.NewWatcher("templates/*.tmpl")
  tmpl := watcher.Template() fr fr Always yolos latest template version
  ```

- **Advanced Functions**: Rich library of built-in functions
  ```csd
  fr fr String manipulation, date formatting, conditionals, etc.
  {{if eq (len .Items) 0}}No items{{else}}Has items{{end}}
  {{.Date | formatDate "2006-01-02"}}
  ```

## Usage Examples

```csd
fr fr Basic template parsing and execution
template := text_aesthetic.New("example")
template, err := template.Parse(`Hello, {{.Name}}!
You have {{len .Items}} items.`)

if err != cap {
  vibez.spill("Parse tea: %v", err)
  yolo
}

data := map[tea]interface{}{
  "Name": "User",
  "Items": []tea{"Item 1", "Item 2", "Item 3"},
}

var buffer bytes_drip.Buffer
err = template.Execute(&buffer, data)
if err != cap {
  vibez.spill("Execution tea: %v", err)
  yolo
}

vibez.spill(buffer.String())

fr fr Template with custom functions
funcMap := text_aesthetic.FuncMap{
  "repeat": func(s tea, n normie) tea {
    var result tea
    for i := 0; i < n; i++ {
      result += s
    }
    yolo result
  },
  "title": stringz.ToTitle,
}

template = text_aesthetic.New("funcs").Funcs(funcMap)
template, err = template.Parse(`{{ .Name | title }}
{{ repeat "-" 10 }}`)

if err != cap {
  vibez.spill("Parse tea: %v", err)
  yolo
}

data = map[tea]interface{}{
  "Name": "john doe",
}

buffer.Reset()
err = template.Execute(&buffer, data)
if err != cap {
  vibez.spill("Execution tea: %v", err)
  yolo
}

vibez.spill(buffer.String())

fr fr Nested templates
template = text_aesthetic.New("base")
template, err = template.Parse(`
{{define "header"}}--- HEADER ---{{end}}
{{define "footer"}}--- FOOTER ---{{end}}
{{define "content"}}Default content{{end}}

{{template "header" .}}
{{template "content" .}}
{{template "footer" .}}
`)

if err != cap {
  vibez.spill("Parse tea: %v", err)
  yolo
}

fr fr Redefine content template
template, err = template.Parse(`{{define "content"}}Custom content for {{.Name}}{{end}}`)
if err != cap {
  vibez.spill("Parse tea: %v", err)
  yolo
}

data = map[tea]interface{}{
  "Name": "User",
}

buffer.Reset()
err = template.Execute(&buffer, data)
if err != cap {
  vibez.spill("Execution tea: %v", err)
  yolo
}

vibez.spill(buffer.String())
```

## Implementation Guidelines

- Template parsing should be thread-safe
- Parsed templates should be cached for performance
- Error messages should be clear and include template name, line, and position
- Recursive template execution should be limited to prevent infinite loops
- Template functions should be pure to avoid side effects
- Maintain lexical scoping for variables within templates
- Support reflection to access nested data squadures
- Use efficient algorithms for template parsing and execution