# text_aesthetic (text/template)

## Overview
The `text_aesthetic` module provides a powerful, flexible template engine for generating textual output. It serves as the foundation for the `rizztemplate` HTML template engine and can be used for any text-based templating needs.

## Core Types and Interfaces

### Template
The main template type that represents a parsed template ready for execution.

```csd
type Template struct {
  // fields not directly accessible
}

func New(name string) *Template
func (t *Template) Parse(text string) (*Template, error)
func (t *Template) ParseFiles(filenames ...string) (*Template, error)
func (t *Template) ParseGlob(pattern string) (*Template, error)
func (t *Template) Execute(wr io.Writer, data interface{}) error
func (t *Template) ExecuteTemplate(wr io.Writer, name string, data interface{}) error
func (t *Template) Clone() (*Template, error)
func (t *Template) Delims(left, right string) *Template
func (t *Template) Funcs(funcMap FuncMap) *Template
func (t *Template) Name() string
func (t *Template) Templates() []*Template
```

### FuncMap
Map of function names to functions for use in templates.

```csd
type FuncMap map[string]interface{}
```

### Error Types
Error types specific to template operations.

```csd
type ExecError struct {
  // fields not directly accessible
}

type ParseError struct {
  // fields not directly accessible
  Line int
  Position int
  Description string
}
```

## Core Functions

```csd
// Create a new template with the given name
func New(name string) *Template

// Parse a template from a string
func (t *Template) Parse(text string) (*Template, error)

// Parse templates from files
func ParseFiles(filenames ...string) (*Template, error)

// Parse templates matching a glob pattern
func ParseGlob(pattern string) (*Template, error)

// Execute a template with data
func (t *Template) Execute(wr io.Writer, data interface{}) error

// Execute a specific named template
func (t *Template) ExecuteTemplate(wr io.Writer, name string, data interface{}) error

// Add function mapping to template
func (t *Template) Funcs(funcMap FuncMap) *Template
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
  // In parent.tmpl: {{block "content" .}}Default content{{end}}
  // In child.tmpl: {{extends "parent.tmpl"}}{{define "content"}}Custom content{{end}}
  ```

- **Whitespace Control**: Trim whitespace with special delimiters
  ```csd
  {{- .Trimmed -}} // Trims whitespace before and after
  ```

- **Debug Mode**: Additional context and verbose errors when developing
  ```csd
  tmpl.SetDebug(true)
  ```

- **Template Watching**: Automatic reloading of templates when files change
  ```csd
  watcher := text_aesthetic.NewWatcher("templates/*.tmpl")
  tmpl := watcher.Template() // Always returns latest template version
  ```

- **Advanced Functions**: Rich library of built-in functions
  ```csd
  // String manipulation, date formatting, conditionals, etc.
  {{if eq (len .Items) 0}}No items{{else}}Has items{{end}}
  {{.Date | formatDate "2006-01-02"}}
  ```

## Usage Examples

```csd
// Basic template parsing and execution
template := text_aesthetic.New("example")
template, err := template.Parse(`Hello, {{.Name}}!
You have {{len .Items}} items.`)

if err != nil {
  vibez.spill("Parse error: %v", err)
  return
}

data := map[string]interface{}{
  "Name": "User",
  "Items": []string{"Item 1", "Item 2", "Item 3"},
}

var buffer bytes_drip.Buffer
err = template.Execute(&buffer, data)
if err != nil {
  vibez.spill("Execution error: %v", err)
  return
}

vibez.spill(buffer.String())

// Template with custom functions
funcMap := text_aesthetic.FuncMap{
  "repeat": func(s string, n int) string {
    var result string
    for i := 0; i < n; i++ {
      result += s
    }
    return result
  },
  "title": stringz.ToTitle,
}

template = text_aesthetic.New("funcs").Funcs(funcMap)
template, err = template.Parse(`{{ .Name | title }}
{{ repeat "-" 10 }}`)

if err != nil {
  vibez.spill("Parse error: %v", err)
  return
}

data = map[string]interface{}{
  "Name": "john doe",
}

buffer.Reset()
err = template.Execute(&buffer, data)
if err != nil {
  vibez.spill("Execution error: %v", err)
  return
}

vibez.spill(buffer.String())

// Nested templates
template = text_aesthetic.New("base")
template, err = template.Parse(`
{{define "header"}}--- HEADER ---{{end}}
{{define "footer"}}--- FOOTER ---{{end}}
{{define "content"}}Default content{{end}}

{{template "header" .}}
{{template "content" .}}
{{template "footer" .}}
`)

if err != nil {
  vibez.spill("Parse error: %v", err)
  return
}

// Redefine content template
template, err = template.Parse(`{{define "content"}}Custom content for {{.Name}}{{end}}`)
if err != nil {
  vibez.spill("Parse error: %v", err)
  return
}

data = map[string]interface{}{
  "Name": "User",
}

buffer.Reset()
err = template.Execute(&buffer, data)
if err != nil {
  vibez.spill("Execution error: %v", err)
  return
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
- Support reflection to access nested data structures
- Use efficient algorithms for template parsing and execution